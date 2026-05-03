mod mesh;
mod shader_types;
mod host_to_gpu;

use std::env;
use core::f32::consts::{PI, TAU};

use leimu::{
    EventError, EventResult,
    core::{TryExtend, collections::{AHashMap, EntryExt}},
    default,
    gpu::{self, MemoryBinder, ext},
    mem::{
        vec::ArrayVec
    },
    sync::{Arc, RwLock, atomic::{self, AtomicU64}},
    win,
};

use mesh::Mesh;

fn main() -> EventResult<()> {
    env_logger::init();
    let entry = leimu::Entry::new()?;
    let mut layers = Vec::new();
    if env::args().any(|arg| arg == "with_validation") {
        layers.push(gpu::layer_khronos_validation(false));
    }
    let instance = entry.create_instance(
        c"mesh shader",
        gpu::make_api_version(0, 0, 1, 0),
        layers,
    )?;
    let devices = instance.enumerate_suitable_physical_devices(
        gpu::default_device_attributes()
            .with_device_extension(ext::mesh_shader::Extension {
                required_features: ext::mesh_shader::Features::default()
                    .mesh_shader(true)
            }).with_device_extension(ext::push_descriptor::Extension)
    )?;
    let mut selected = 0;
    for (idx, device) in devices.enumerate() {
        if device.device_type() == gpu::PhysicalDeviceType::DISCRETE_GPU {
            selected = idx;
        }
    }
    let mut queue = gpu::DeviceQueue::uninit();
    let device = devices.create_device(
        selected,
        [gpu::DeviceQueueCreateInfo::new(
            &mut queue,
            "queue",
            devices[selected]
                .enumerate_queue_families()
                .iter()
                .find_map(|(idx, prop)| {
                    prop.queue_flags
                        .contains(gpu::QueueFlags::GRAPHICS)
                        .then_some(idx)
                }).ok_or_else(|| EventError::just_context("failed to find queue"))?,
            0,
        )]
    )?;
    let queue = queue.get()?;
    let globals = leimu::create_globals();
    #[repr(C)]
    #[repr(align(16))]
    #[derive(Clone, Copy)]
    struct Meshlet {
        debug_color: [f32; 3],
        vertex_count: u32,
        triangle_count: u32,
        vertices: [u32; 6],
        triangles: [[u32; 3]; 2],
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Mvp {
        m: glam::Mat4,
        v: glam::Mat4,
        p: glam::Mat4,
        imv: glam::Mat4
    }
    let shader_set = globals.add(|event_loop| {
        let gpu = event_loop.gpu();
        let mesh = gpu::default_shader_attributes()
            .with_stage(gpu::ShaderStage::MeshEXT)
            .with_name("mesh shader")
            .with_glsl("
                #version 450

                #extension GL_EXT_mesh_shader : require

                layout(local_size_x = 32) in;
                layout(triangles, max_vertices = 64, max_primitives = 128) out;

                struct Meshlet {
                    vec3 debug_color;
                    uint vertex_count;
                    uint triangle_count;
                    uint vertices[6];
                    uint triangles[2][3];
                };

                struct Vertex {
                    vec3 position;
                    vec3 normal;
                    vec2 uv;
                };

                layout(set = 0, binding = 0) buffer Meshlets {
                    uint len;
                    Meshlet data[];
                } meshlets;

                layout(set = 0, binding = 1) buffer Vertices {
                    Vertex data[];
                } vertices;

                layout(set = 0, binding = 2) uniform Mvp {
                    mat4 m;
                    mat4 v;
                    mat4 p;
                    mat4 imv;
                } mvp;
                
                layout(location = 0) perprimitiveEXT out vec3 out_color[];

                void main() {
                    uint meshlet_id = gl_WorkGroupID.x;
                    if (meshlet_id >= meshlets.len) {
                        return;
                    }
                    Meshlet m = meshlets.data[meshlet_id];
                    SetMeshOutputsEXT(m.vertex_count, m.triangle_count);
                    mat4 transform = mvp.p * mvp.m;
                    uint tid = gl_LocalInvocationIndex;
                    if (tid < m.vertex_count) {
                        Vertex vertex = vertices.data[m.vertices[tid]];
                        gl_MeshVerticesEXT[tid].gl_Position =
                            transform * vec4(vertex.position, 1.0f);
                    }
                    if (tid < m.triangle_count) {
                        gl_PrimitiveTriangleIndicesEXT[tid] =
                            uvec3(
                                m.triangles[tid][0],
                                m.triangles[tid][1],
                                m.triangles[tid][2]
                            );
                        out_color[tid] = m.debug_color;
                    }
                }
            ").build(gpu)?;
        let fragment = gpu::default_shader_attributes()
            .with_stage(gpu::ShaderStage::Fragment)
            .with_name("fragment shader")
            .with_glsl("
                #version 450

                #extension GL_EXT_mesh_shader : require

                layout(location = 0) perprimitiveEXT in vec3 in_color;
                layout(location = 0) out vec4 out_color;

                void main() {
                    out_color = vec4(in_color, 1.0f);
                }
            ").build(gpu)?;
        let shader_set = gpu.create_shader_set(
            [mesh, fragment],
            gpu::default_shader_set_attributes()
                .with_descriptor_set_layout_flags(
                    0, gpu::DescriptorSetLayoutFlags::PUSH_DESCRIPTOR
                )
        )?;
        Ok(shader_set)
    });
    let mesh = Mesh::from_obj(include_str!("../low-poly-sphere.obj"))?;
    #[derive(Clone, Copy)]
    struct Buffers {
        meshlet: gpu::BufferId,
        vertex: gpu::BufferId,
        mvp: gpu::BufferId,
    }
    let mut buffers: Option<Buffers> = None;
    let mut graphics_pipelines = AHashMap::new();
    #[derive(Default)]
    struct SwapchainDependent {
        current_pipeline: gpu::GraphicsPipelineId,
        depth_images: ArrayVec<gpu::ImageViewId, 3>,
        resolution: (u32, u32),
    }
    let sc_dependent = Arc::new(RwLock::new(SwapchainDependent::default()));
    let sc_binder = gpu::LinearBinder::new(
        device.clone(),
        1920 * 1080 * 2,
        gpu::MemoryProperties::DEVICE_LOCAL,
        gpu::MemoryProperties::DEVICE_LOCAL
    )?;
    let frame_semaphore = globals.add(|event_loop| {
        let gpu = event_loop.gpu();
        let mut id = default();
        gpu.create_timeline_semaphores([gpu::semaphore_create_info(
            &mut id,
            3
        )])?;
        Ok(id)
    });
    let window = globals.add(|event_loop| {
        Ok(event_loop.create_window(win::default_attributes()
            .with_title("mesh shader")
            .with_resizable(true)
        )?)
    });
    let frame = Arc::new(AtomicU64::new(3));
    let host_to_gpu = globals.add(|event_loop| {
        let gpu = event_loop.gpu();
        host_to_gpu::Scheduler::new(gpu.clone(), 3)
    });
    let device_local_binder = gpu::GlobalBinder::new(
        device.clone(),
        gpu::MemoryProperties::DEVICE_LOCAL,
        gpu::MemoryProperties::DEVICE_LOCAL
    );
    let mut rot = 0.0;
    leimu::Leimu::new(
        entry,
        device,
        leimu::default_attributes(),
        &globals,
        |event_loop, event| {
            if let leimu::Event::CleanUp { ended_on_error } = &event  && 
                *ended_on_error
            {
                return Ok(())
            }
            let surface_id = window.surface_id();
            let Some(mut window) = event_loop.get_window(*window) else {
                event_loop.exit();
                return Ok(())
            };
            if window.key_state(win::KeyCode::Escape).pressed() {
                window.close();
                return Ok(())
            }
            let gpu = event_loop.gpu();
            let current_frame = frame.load(atomic::Ordering::Relaxed);
            host_to_gpu.set_frame(current_frame);
            rot = (rot + window.delta_time_secs_f32()) % TAU;
            match event {
                leimu::Event::Initialized => {
                    let _ = leimu::block_on(gpu.get_shader_set(*shader_set))?;
                    Ok(())
                },
                leimu::Event::Update => {
                    let buffers = if let Some(buffers) = buffers {
                        buffers
                    } else {
                        let [mut meshlet, mut vertex, mut mvp] = default();
                        gpu.create_resources(
                            [
                                gpu::BufferCreateInfo::new(
                                    &mut meshlet,
                                    &device_local_binder,
                                    (align_of::<Meshlet>() + size_of::<Meshlet>() * mesh.meshlets.len()) as u64,
                                    gpu::BufferUsages::TRANSFER_DST | gpu::BufferUsages::STORAGE_BUFFER
                                ).unwrap(),
                                gpu::BufferCreateInfo::new(
                                    &mut vertex,
                                    &device_local_binder,
                                    size_of_val(mesh.vertices.as_slice()) as u64,
                                    gpu::BufferUsages::TRANSFER_DST | gpu::BufferUsages::STORAGE_BUFFER
                                ).unwrap(),
                                gpu::BufferCreateInfo::new(
                                    &mut mvp,
                                    &device_local_binder,
                                    size_of::<Mvp>() as u64,
                                    gpu::BufferUsages::TRANSFER_DST | gpu::BufferUsages::UNIFORM_BUFFER
                                ).unwrap()
                            ],
                            [],
                        )?;
                        let debug_colors = [
                            glam::vec3(
                                19.0 / 255.0,
                                214.0 / 255.0, 
                                156.0 / 255.0
                            ),
                            glam::vec3(
                                214.0 / 255.0,
                                19.0 / 255.0,
                                154.0 / 255.0
                            ),
                            glam::vec3(
                                202.0 / 255.0,
                                214.0 / 255.0,
                                19.0 / 255.0
                            )
                        ];
                        let meshlets: Vec<Meshlet> = mesh.meshlets
                            .iter().enumerate().map(|(i, meshlet)| {
                                Meshlet {
                                    debug_color: debug_colors[i % 3].into(),
                                    vertex_count: meshlet.local_vertices.len() as u32,
                                    triangle_count: meshlet.local_triangles.len() as u32,
                                    vertices: unsafe {
                                        meshlet.local_vertices.clone().assume_init_array()
                                    },
                                    triangles: unsafe {
                                        meshlet.local_triangles.clone().assume_init_array()
                                    },
                                }
                            }).collect();
                        host_to_gpu.buffer_to_buffer(
                            meshlet,
                            16, &meshlets
                        )?;
                        host_to_gpu.buffer_to_buffer(
                            vertex,
                            0, &mesh.vertices,
                        )?;
                        *buffers.insert(Buffers {
                            meshlet,
                            vertex,
                            mvp,
                        })
                    };
                    let n_meshlets = mesh.meshlets.len() as u32;
                    gpu.schedule_commands(|scheduler| {
                        let cmd0 = if host_to_gpu.is_empty() {
                            None
                        } else {
                            let host_to_gpu = host_to_gpu.clone();
                            Some(scheduler.new_commands::<gpu::NewCopyCommands>(
                                queue.clone(),
                                move |cmd| {
                                    unsafe {
                                        host_to_gpu.flush(current_frame - 2)?;
                                    }
                                    host_to_gpu.record_copies(cmd)
                                }
                            )?.with_wait_semaphore(
                                *frame_semaphore,
                                current_frame - 2, gpu::MemoryDependencyHint::TRANSFER
                            ).id())
                        };
                        let sc_dependent = sc_dependent.clone();
                        let frame = frame.clone();
                        let cmd1 = scheduler.new_commands::<gpu::NewGraphicsCommands>(
                            queue.clone(),
                            move |cmd| {
                                let SwapchainDependent { current_pipeline, depth_images, resolution }
                                    = &*sc_dependent.read();
                                let mut copy_commands = cmd.copy_commands();
                                let mvp = Mvp {
                                    m: glam::Mat4::from_rotation_translation(
                                        glam::Quat::from_axis_angle(
                                            glam::vec3(0.0, 1.0, 0.0),
                                            rot,
                                        ),
                                        glam::vec3(0.0, 0.0, -3.0),
                                    ),
                                    v: glam::Mat4::IDENTITY,
                                    p: glam::Mat4::perspective_rh(
                                        PI / 2.0,
                                        resolution.0 as f32 / resolution.1 as f32,
                                        0.1, 100.0
                                    ),
                                    imv: glam::Mat4::IDENTITY,
                                };
                                copy_commands.update_buffer(
                                    buffers.meshlet,
                                    0, &[n_meshlets],
                                    gpu::CommandOrdering::Lenient
                                )?;
                                copy_commands.update_buffer(
                                    buffers.mvp,
                                    0, &[mvp],
                                    gpu::CommandOrdering::Lenient,
                                )?;
                                let frame = frame.fetch_add(1, atomic::Ordering::Relaxed) + 1;
                                let frame_idx = (frame % 3) as usize;
                                let (swapchain_image, _) = cmd.swapchain_image_view(surface_id)?;
                                cmd.render(
                                    default(),
                                    &[
                                        gpu::PassAttachment::new(swapchain_image)
                                            .with_load_op(gpu::AttachmentLoadOp::Clear)
                                    ],
                                    &gpu::DepthStencilAttachment::Depth(
                                        gpu::PassAttachment::new(depth_images[frame_idx])
                                            .with_load_op(gpu::AttachmentLoadOp::Clear)
                                            .with_clear_value(gpu::ClearValue::DepthStencil {
                                                depth: 1.0, stencil: 0,
                                            })
                                    ),
                                    |cmd| {
                                        cmd.dynamic_draw(|cmd| {
                                            let mut cmd = cmd.bind_pipeline(
                                                *current_pipeline,
                                                &[gpu::Viewport::default()
                                                    .width(resolution.0 as f32)
                                                    .height(resolution.1 as f32)
                                                ],
                                                &[gpu::Scissor::default()
                                                    .width(resolution.0)
                                                    .height(resolution.1)
                                                ]
                                            )?;
                                            cmd.push_descriptor_bindings(
                                                &[
                                                    gpu::PushDescriptorBinding::new(
                                                        c"meshlets",
                                                        0,
                                                        &gpu::descriptor_buffer_info(buffers.meshlet),
                                                        None
                                                    )?,
                                                    gpu::PushDescriptorBinding::new(
                                                        c"vertices",
                                                        0,
                                                        &gpu::descriptor_buffer_info(buffers.vertex),
                                                        None
                                                    )?,
                                                    gpu::PushDescriptorBinding::new(
                                                        c"mvp",
                                                        0,
                                                        &gpu::descriptor_buffer_info(buffers.mvp),
                                                        gpu::CommandBarrierInfo::new(
                                                            gpu::CommandOrdering::Strict,
                                                            gpu::ExplicitAccess::SHADER_READ
                                                        ),
                                                    )?,
                                                ]
                                            )?;
                                            cmd.draw_mesh_tasks_ext(
                                                n_meshlets, 1, 1,
                                            )?;
                                            Ok(())
                                        })?;
                                        Ok(())
                                    }
                                )?;
                                Ok(())
                            }
                        )?.with_signal_semaphore(*frame_semaphore, current_frame + 1);
                        if let Some(cmd0) = cmd0 {
                            cmd1.with_dependencies([gpu::CommandDependency::new(
                                cmd0,
                                gpu::MemoryDependencyHint::MESH_SHADER,
                            )]);
                        } else {
                            cmd1.with_wait_semaphore(
                                *frame_semaphore, current_frame - 2,
                                gpu::MemoryDependencyHint::FRAGMENT_SHADER,
                            );
                        }
                        Ok(())
                    })?;
                    Ok(())
                },
                leimu::Event::GpuEvent(event) => {
                    let gpu::Event::SwapchainCreated {
                        new_format, new_size, image_count, ..
                    } = event;
                    if image_count < 3 {
                        return Err(EventError::just_context(
                            "this example requires image count of at least 3"
                        ))
                    }
                    let mut sc_dependent = sc_dependent.write();
                    sc_dependent.current_pipeline = *graphics_pipelines
                        .entry(new_format)
                        .or_try_insert_with(|| {
                            let mut batch = gpu.create_pipeline_batch(None)?;
                            let mut id = default();
                            batch.with_graphics_pipelines([
                                gpu::GraphicsPipelineCreateInfo::new(
                                    &mut id,
                                    *shader_set
                                ).with_color_output(
                                    new_format,
                                    gpu::ColorComponents::RGBA,
                                    None
                                ).with_depth_output(gpu::Format::D16_Unorm)
                                .with_depth_stencil(gpu::DepthStencilInfo::default()
                                    .depth_bounds(Some(gpu::DepthBounds::default()
                                        .max(1.0)
                                    )).compare_op(gpu::CompareOp::LESS)
                                    .write_enable(true)
                                )
                            ]);
                            let _ = batch.build()?;
                            EventResult::Ok(id)
                        })?;
                    if !sc_dependent.depth_images.is_empty() {
                        let last_frame = frame.load(atomic::Ordering::Relaxed);
                        if gpu.wait_for_semaphores(
                            [gpu::semaphore_wait_info(*frame_semaphore, last_frame)],
                            leimu::duration_secs(1.0)
                        )? {
                            gpu.destroy_resources(
                                [],
                                sc_dependent.depth_images.iter()
                                    .map(|id| id.image_id())
                            )?;
                            unsafe {
                                sc_binder.release_resources();
                            }
                            sc_dependent.depth_images.clear();
                        } else {
                            return Err(EventError::just_context(
                                "timeout while waiting for frame semaphore"
                            ))
                        }
                    }
                    sc_dependent.resolution = new_size;
                    let mut new_images = [default(); 3];
                    gpu.create_resources(
                        [],
                        new_images.iter_mut()
                            .map(|id| {
                                gpu::ImageCreateInfo::new(id, &sc_binder)
                                    .with_dimensions(new_size)
                                    .with_format(gpu::Format::D16_Unorm, false)
                                    .with_usage(gpu::ImageUsages::DEPTH_STENCIL_ATTACHMENT)
                            })
                    )?;
                    sc_dependent.depth_images.try_extend(new_images
                        .into_iter().map(|id| {
                            gpu.create_image_view(
                                id,
                                gpu::ImageRange::whole_range(gpu::ImageAspects::DEPTH)
                            )
                        })
                    )?;
                    Ok(())
                },
                _ => { Ok(()) },
            }
        },
    )?.run()?;
    Ok(())
}

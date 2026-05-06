mod meshlet;
mod obj;
mod shader_types;
mod host_to_gpu;

use std::env;
use std::sync::{Arc, atomic::{self, AtomicU64}};
use core::f32::consts::{PI, TAU, FRAC_PI_3};

use leimu::{
    EventError, EventResult,
    core::*,
    default,
    gpu::{self, MemoryBinder, ext},
    mem::{
        vec::ArrayVec
    },
    sync::{
        SwapLock,
    },
    win,
};

use ahash::AHashMap;
use parking_lot::RwLock;

#[inline]
fn rgb_from_hsv(hue: f32, sat: f32, val: f32) -> glam::Vec3 {
    let map = |n: f32| -> f32 {
        let k = (n + hue / FRAC_PI_3) % 6.0;
        let ch = val - val * sat * k.min(4.0 - k).clamp(0.0, 1.0);
        if ch <= 0.04045 {
            ch / 12.92
        } else {
            ((ch + 0.055) / 1.055).powf(2.4)
        }
    };
    glam::vec3(map(5.0), map(3.0), map(1.0))
}

fn main() -> EventResult<()> {
    env_logger::init();
    let entry = leimu::Entry::new()?;
    let mut layers = Vec::new();
    for arg in env::args().skip(1) {
        if arg == "with_validation" {
            layers.push(gpu::layer_khronos_validation(false));
        } else {
            log::error!("invalid argument {arg}");
        }
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
            .with_required_device_features(gpu::BaseDeviceFeatures::default()
                .fragment_stores_and_atomics(true)
            )
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
    let max_local_vertices = 128;
    let max_local_primitives = 256;
    let mesh_shader_header = format!(
"#version 450

#extension GL_EXT_mesh_shader : require

const uint MAX_VERTICES = {max_local_vertices};
const uint MAX_PRIMITIVES = {max_local_primitives};
"   );
    let shader_set = globals.add(|event_loop| {
        let gpu = event_loop.gpu();
        let mesh = gpu::default_shader_attributes()
            .with_stage(gpu::ShaderStage::MeshEXT)
            .with_name("mesh shader")
            .with_glsl(&(mesh_shader_header + "
                layout(local_size_x = 128) in;
                layout(triangles, max_vertices = MAX_VERTICES, max_primitives = MAX_PRIMITIVES) out;

                struct Meshlet {
                    vec3 debug_color;
                    uint vertex_count;
                    uint primitive_count;
                    uint vertices[MAX_VERTICES];
                    uint primitives[MAX_PRIMITIVES][3];
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
                    mat4 mvp;
                    mat4 imv;
                } mvp;
 
                layout(location = 0) perprimitiveEXT out vec3 out_color[];
                layout(location = 1) perprimitiveEXT flat out uint out_meshlet_id[];
                layout(location = 2) out vec2 out_ndc[];

                void main() {
                    uint meshlet_id = gl_WorkGroupID.x;
                    if (meshlet_id >= meshlets.len) {
                        return;
                    }
                    Meshlet m = meshlets.data[meshlet_id];
                    SetMeshOutputsEXT(m.vertex_count, m.primitive_count);
                    uint tid = gl_LocalInvocationIndex;
                    if (tid < m.vertex_count) {
                        Vertex vertex = vertices.data[m.vertices[tid]];
                        vec4 pos = mvp.mvp * vec4(vertex.position, 1.0f);
                        gl_MeshVerticesEXT[tid].gl_Position = pos;
                        out_ndc[tid] = pos.xy / pos.w;
                    }
                    for (uint i = 0; i < 2; ++i) {
                        uint prim_id = tid + i * 128;
                        if (prim_id < m.primitive_count) {
                            gl_PrimitiveTriangleIndicesEXT[prim_id] =
                                uvec3(
                                    m.primitives[prim_id][0],
                                    m.primitives[prim_id][1],
                                    m.primitives[prim_id][2]
                                );
                            out_color[prim_id] = m.debug_color;
                            out_meshlet_id[prim_id] = meshlet_id;
                        }
                    }
                }
            ")).build(gpu)?;
        let fragment = gpu::default_shader_attributes()
            .with_stage(gpu::ShaderStage::Fragment)
            .with_name("fragment shader")
            .with_glsl("
                #version 450

                #extension GL_EXT_mesh_shader : require

                layout(location = 0) perprimitiveEXT in vec3 in_color;
                layout(location = 1) perprimitiveEXT flat in uint in_meshlet_id;
                layout(location = 2) in vec2 in_ndc;
                layout(location = 0) out vec4 out_color;

                layout(push_constant) uniform Meta {
                    vec2 norm_cursor_pos;
                } meta;

                layout(set = 0, binding = 3) buffer Highlights1 {
                    uint data[];
                } highlights_in;

                layout(set = 0, binding = 4) buffer Highlights2 {
                    uint data[];
                } highlights_out;

                void main() {
                    vec2 norm_coord = (in_ndc + vec2(1.0f)) * 0.5f;
                    norm_coord.y = 1.0 - norm_coord.y; 
                    if (length(norm_coord - meta.norm_cursor_pos) < 0.001f) {
                        highlights_out.data[in_meshlet_id] = 1;
                    }
                    if (highlights_in.data[in_meshlet_id] != 0) {
                        out_color = vec4(1.0f);
                    } else {
                        out_color = vec4(in_color, 1.0f);
                    }
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
    #[derive(Clone, Copy)]
    struct MeshBuffers {
        n_meshlets: u32,
        meshlet: gpu::BufferId,
        vertex: gpu::BufferId,
        mvp: gpu::BufferId,
        highlights: [gpu::BufferId; 3],
    }
    let bunny = obj::parse(include_str!("../stanford-bunny.obj"))?;
    let mut bunny = meshlet::Mesh::new(
        "stanford-bunny.obj", bunny,
        max_local_vertices, max_local_primitives,
    );
    bunny.recenter();
    let sphere = obj::parse(include_str!("../low-poly-sphere.obj"))?;
    let sphere = meshlet::Mesh::new(
        "low-poly-sphere.obj", sphere,
        max_local_vertices, max_local_primitives
    );
    let meshes = [bunny, sphere];
    let mut current_mesh: usize = 0;
    let mesh_buffers: Arc<SwapLock<Vec<MeshBuffers>>> = default();
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
    let mut y_rot = 0.0;
    let mut cam_pos_z = 0.0;
    let mut rot = glam::Quat::IDENTITY;
    let mut cursor_pos = glam::Vec2::default();
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
            if window.key_state(win::KeyCode::ArrowLeft).pressed() {
                current_mesh = current_mesh.wrapping_sub(1).clamp(0, meshes.len() - 1);
                rot = glam::Quat::IDENTITY;
            } else if window.key_state(win::KeyCode::ArrowRight).pressed() {
                current_mesh += 1;
                if current_mesh >= meshes.len() {
                    current_mesh = 0;
                }
                rot = glam::Quat::IDENTITY;
            }
            let delta_lines = window.mouse_scroll_delta_lines().1;
            let delta_pixels = window.mouse_scroll_delta_pixels_f32().1 / 100.0;
            let mut delta_scroll = delta_lines;
            if delta_lines.abs() < delta_pixels.abs() {
                delta_scroll = delta_pixels;
            }
            cam_pos_z = (cam_pos_z + delta_scroll).clamp(0.0, 2.0);
            let new_cursor_pos = window.cursor_position_f32().into();
            if window.mouse_button_state(win::MouseButton::Left).held() {
                let delta_cursor_pos: glam::Vec2 = (new_cursor_pos - cursor_pos) / 500.0;
                if delta_cursor_pos.length_squared() > f32::EPSILON {
                    let right = glam::Vec2::from_angle(PI / 2.0).rotate(glam::vec2(delta_cursor_pos.x, delta_cursor_pos.y)).normalize();
                    rot = glam::Quat::from_axis_angle(glam::vec3(right.x, right.y, 0.0).normalize(), delta_cursor_pos.length()) * rot;
                    rot = rot.normalize();
                }
            }
            cursor_pos = new_cursor_pos;
            let norm_cursor_pos = window.normalized_cursor_position_f32();
            let gpu = event_loop.gpu();
            let current_frame = frame.load(atomic::Ordering::Relaxed);
            host_to_gpu.set_frame(current_frame);
            y_rot = (y_rot + window.delta_time_secs_f32()) % TAU;
            match event {
                leimu::Event::Initialized => {
                    let _ = leimu::block_on(gpu.get_shader_set(*shader_set))?;
                    Ok(())
                },
                leimu::Event::Update => {
                    if mesh_buffers.load().len() != meshes.len() {
                        mesh_buffers.modify(|mesh_buffers| {
                            for mesh in meshes.iter().skip(mesh_buffers.len()) {
                                let [mut meshlet, mut vertex, mut mvp] = default();
                                gpu.create_resources(
                                    [
                                        gpu::BufferCreateInfo::new(
                                            &mut meshlet,
                                            &device_local_binder,
                                            (
                                                align_of::<shader_types::Meshlet>() +
                                                shader_types::Meshlet::array_stride(max_local_vertices, max_local_primitives) *
                                                mesh.meshlets().len()
                                            ) as u64,
                                            gpu::BufferUsages::TRANSFER_DST | gpu::BufferUsages::STORAGE_BUFFER
                                        ).unwrap(),
                                        gpu::BufferCreateInfo::new(
                                            &mut vertex,
                                            &device_local_binder,
                                            size_of_val(mesh.vertices()) as u64,
                                            gpu::BufferUsages::TRANSFER_DST | gpu::BufferUsages::STORAGE_BUFFER
                                        ).unwrap(),
                                        gpu::BufferCreateInfo::new(
                                            &mut mvp,
                                            &device_local_binder,
                                            size_of::<shader_types::Mvp>() as u64,
                                            gpu::BufferUsages::TRANSFER_DST | gpu::BufferUsages::UNIFORM_BUFFER
                                        ).unwrap(),
                                    ],
                                    [],
                                )?;
                                let mut highlights = [default(); 3];
                                gpu.create_resources(
                                    highlights.iter_mut().map(|id| {
                                        gpu::BufferCreateInfo::new(
                                            id,
                                            &device_local_binder,
                                            mesh.meshlets().len() as u64 * 4,
                                            gpu::BufferUsages::TRANSFER_DST | gpu::BufferUsages::STORAGE_BUFFER
                                        ).unwrap()
                                    }),
                                    []
                                )?;
                                let mut hue = 0.0;
                                let hue_step = TAU / mesh.meshlets().len() as f32;
                                let mut buffer: Vec<_> = vec![];
                                let stride = shader_types::Meshlet::array_stride(max_local_vertices, max_local_primitives);
                                for meshlet in mesh.meshlets() {
                                    let start = buffer.len();
                                    buffer.extend_from_slice(shader_types::Meshlet {
                                        debug_color: rgb_from_hsv(hue, 0.6, rand::random_range(0.7..1.0)).into(),
                                        vertex_count: meshlet.local_vertices().len() as u32,
                                        triangle_count: meshlet.local_triangles().len() as u32,
                                    }.as_inline_bytes());
                                    buffer.extend_from_slice(meshlet.local_vertices_full_as_bytes());
                                    buffer.extend_from_slice(meshlet.local_triangles_full_as_bytes());
                                    buffer.extend(((buffer.len() - start)..stride).map(|_| 0));
                                    assert!(buffer.len() - start == stride);
                                    hue += hue_step;
                                }
                                host_to_gpu.buffer_to_buffer(
                                    meshlet,
                                    align_of::<shader_types::Meshlet>() as u64, &buffer
                                )?;
                                host_to_gpu.buffer_to_buffer(
                                    vertex,
                                    0, mesh.vertices(),
                                )?;
                                mesh_buffers.push(MeshBuffers {
                                    n_meshlets: mesh.meshlets().len() as u32,
                                    meshlet,
                                    vertex,
                                    mvp,
                                    highlights,
                                });
                            }
                            EventResult::Ok(())
                        })?;
                    }
                    gpu.schedule_commands(|scheduler| {
                        let cmd0 = if host_to_gpu.is_empty() {
                            None
                        } else {
                            let host_to_gpu = host_to_gpu.clone();
                            Some(scheduler.new_commands::<gpu::NewCopyCommands>(
                                queue.clone(),
                                move |cmd| {
                                    unsafe {
                                        host_to_gpu.flush(current_frame - 3)?;
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
                        let mesh_buffers = mesh_buffers.clone();
                        let cmd1 = scheduler.new_commands::<gpu::NewGraphicsCommands>(
                            queue.clone(),
                            move |cmd| {
                                let SwapchainDependent { current_pipeline, depth_images, resolution }
                                    = &*sc_dependent.read();
                                let mut copy_commands = cmd.copy_commands();
                                let mesh_buffers = mesh_buffers.load();
                                let mesh_buffers = mesh_buffers[current_mesh];
                                let m = glam::Mat4::from_rotation_translation(
                                    //glam::Quat::from_axis_angle(glam::vec3(0.0, 1.0, 0.0), y_rot) * rot,
                                    rot,
                                    glam::vec3(0.0, 0.0, 3.0),
                                );
                                let v = glam::Mat4::look_to_rh(
                                    glam::vec3(0.0, 0.0, cam_pos_z),
                                    glam::vec3(0.0, 0.0, 1.0),
                                    glam::vec3(0.0, 1.0, 0.0)
                                );
                                let p = glam::Mat4::perspective_rh(
                                    PI / 2.0,
                                    resolution.0 as f32 / resolution.1 as f32,
                                    0.1, 100.0
                                );
                                let mvp = shader_types::Mvp {
                                    mvp:  p * v * m,
                                    imv: glam::Mat4::IDENTITY,
                                };
                                copy_commands.update_buffer(
                                    mesh_buffers.meshlet,
                                    0, &[mesh_buffers.n_meshlets],
                                    gpu::CommandOrdering::Lenient
                                )?;
                                copy_commands.update_buffer(
                                    mesh_buffers.mvp,
                                    0, &[mvp],
                                    gpu::CommandOrdering::Lenient,
                                )?;
                                let frame = frame.fetch_add(1, atomic::Ordering::Relaxed) + 1;
                                let frame_idx = (frame % 3) as usize;
                                let hl_out = mesh_buffers.highlights[frame_idx];
                                let hl_in = mesh_buffers.highlights[if frame_idx == 0 { 2 } else { frame_idx - 1 }];
                                copy_commands.fill_buffer(
                                    hl_out,
                                    0, None, 0,
                                    gpu::CommandOrdering::Lenient,
                                )?;
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
                                                    .height(-(resolution.1 as f32))
                                                    .y(resolution.1 as f32)
                                                ],
                                                &[gpu::Scissor::default()
                                                    .width(resolution.0)
                                                    .height(resolution.1)
                                                ]
                                            )?;
                                            cmd.push_constants(
                                                0,
                                                &[norm_cursor_pos]
                                            )?;
                                            cmd.push_descriptor_bindings(
                                                &[
                                                    gpu::PushDescriptorBinding::new(
                                                        c"meshlets",
                                                        0,
                                                        &gpu::descriptor_buffer_info(mesh_buffers.meshlet),
                                                        None
                                                    )?,
                                                    gpu::PushDescriptorBinding::new(
                                                        c"vertices",
                                                        0,
                                                        &gpu::descriptor_buffer_info(mesh_buffers.vertex),
                                                        None
                                                    )?,
                                                    gpu::PushDescriptorBinding::new(
                                                        c"mvp",
                                                        0,
                                                        &gpu::descriptor_buffer_info(mesh_buffers.mvp),
                                                        gpu::CommandBarrierInfo::new(
                                                            gpu::CommandOrdering::Strict,
                                                            gpu::ExplicitAccess::SHADER_READ
                                                        ),
                                                    )?,
                                                    gpu::PushDescriptorBinding::new(
                                                        c"highlights_in",
                                                        0,
                                                        &gpu::descriptor_buffer_info(hl_in),
                                                        gpu::CommandBarrierInfo::new(
                                                            gpu::CommandOrdering::Strict,
                                                            gpu::ExplicitAccess::SHADER_READ_AND_WRITE
                                                        )
                                                    )?,
                                                    gpu::PushDescriptorBinding::new(
                                                        c"highlights_out",
                                                        0,
                                                        &gpu::descriptor_buffer_info(hl_out),
                                                        gpu::CommandBarrierInfo::new(
                                                            gpu::CommandOrdering::Strict,
                                                            gpu::ExplicitAccess::SHADER_READ_AND_WRITE
                                                        )
                                                    )?,
                                                ]
                                            )?;
                                            cmd.draw_mesh_tasks_ext(
                                                mesh_buffers.n_meshlets, 1, 1,
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
                                ).with_front_face(gpu::FrontFace::CLOCK_WISE),
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

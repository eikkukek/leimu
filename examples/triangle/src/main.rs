use core::f32::consts::{PI, TAU, FRAC_PI_3};

use leimu::{
    Entry,
    gpu::{self, ext},
    core::collections::{EntryExt, HashMap},
    sync::{Arc, SwapLock, atomic::{self, AtomicU64}},
    EventError,
};

fn hsva_to_srgb_pack32(hue: f32, sat: f32, val: f32) -> u32 {
    let map = |n: f32| -> f32 {
        let k = (n + hue / FRAC_PI_3) % 6.0;
        let ch = val - val * sat * k.min(4.0 - k).clamp(0.0, 1.0);
        if ch <= 0.04045 {
            ch / 12.92
        } else {
            ((ch + 0.055) / 1.055).powf(2.4)
        }
    };
    let (r, g, b) = (map(5.0), map(3.0), map(1.0));
    u32::from_le_bytes([
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        255u8,
    ])
}

fn main() -> leimu::EventResult<()> {
    let entry = Entry
        ::new()
        .expect("failed to init library");
    let instance = entry.create_instance(
        c"test",
        gpu::make_api_version(0, 1, 0, 0),
        [gpu::layer_khronos_validation(false)]
    )?;
    let device_attributes = gpu
        ::default_device_attributes()
        .with_device_extension(ext::push_descriptor::Extension);
    let devices = instance.enumerate_suitable_physical_devices(
        device_attributes
    )?;
    let mut idx = 0;
    for (i, device) in devices.enumerate() {
        if device.device_type() ==
            gpu::PhysicalDeviceType::DISCRETE_GPU
        {
            idx = i;
        }
    }
    let physical_device = &devices[idx];
    let queue_family_index = physical_device
        .enumerate_queue_families()
        .into_iter()
        .find_map(|(i, properties)|
            (properties.queue_flags.contains(gpu::QueueFlags::GRAPHICS))
            .then_some(i)
        ).ok_or_else(|| EventError::just_context("failed to find graphics queue"))?;
    let mut queue = gpu::DeviceQueue::uninit();
    let queue_create_info = gpu::DeviceQueueCreateInfo::new(
        &mut queue,
        "graphics queue",
        queue_family_index,
        0
    );
    let logical_device = devices
        .create_device(idx, [queue_create_info])
        .unwrap();
    let queue = queue.get()?;
    let min_uniform_buffer_offset_alignment = logical_device
        .physical_device()
        .limits().min_uniform_buffer_offset_alignment();
    let globals = leimu::create_globals();
    let window = globals.add(|event_loop| {
        Ok(event_loop.create_window(
            leimu::win::default_attributes()
                .with_resizable(true)
                .with_title("Hello, Triangle")
        )?)
    });
    let pipeline_cache = globals.add(|event_loop| {
        Ok(gpu::PipelineCache::new(
            event_loop.gpu().device().clone(),
            None
        )?)
    });
    let shader_set = globals.add(|event_loop| {
        let vertex_shader = gpu::Shader::new(
            event_loop.gpu(), gpu
                ::default_shader_attributes()
                .with_name("vertex shader")
                .with_stage(gpu::ShaderStage::Vertex)
                .with_glsl("
                    #version 450

                    const vec2 positions[3] = {
                        vec2(-0.5f, 0.5f),
                        vec2(0.0f, -0.5f),
                        vec2(0.5f, 0.5f)
                    };

                    layout(set = 0, binding = 0) uniform Colors {
                        uvec3 data;
                    } colors;

                    layout(location = 0) out vec4 out_color;

                    vec4 unpack_color(uint color) {
                        return vec4(
                            (color & 0xFF) / 255.0f,
                            ((color >> 8) & 0xFF) / 255.0f,
                            ((color >> 16) & 0xFF) / 255.0f,
                            1.0f
                        );
                    }

                    void main() {
                        uint idx = gl_VertexIndex;
                        out_color = unpack_color(colors.data[idx]);
                        gl_Position = vec4(positions[idx], 0.0, 1.0f);
                    }
                ")
            )?;
            let fragment_shader = gpu::Shader::new(
                event_loop.gpu(),
                gpu::default_shader_attributes().with_name("fragment shader")
                    .with_stage(gpu::ShaderStage::Fragment)
                    .with_glsl("
                        #version 450

                        layout(location = 0) in vec4 in_color;
                        layout(location = 0) out vec4 out_color;

                        void main() {
                            out_color = in_color;
                        }
                    ")
            )?;
            Ok(event_loop
                .gpu()
                .create_shader_set(
                    [vertex_shader, fragment_shader],
                    gpu::default_shader_set_attributes()
                        .with_descriptor_set_layout_flags(
                            0,
                            gpu::DescriptorSetLayoutFlags::PUSH_DESCRIPTOR,
                        )
                )?)
    });
    let buffer = globals.add(|event_loop| {
        let mut id = Default::default();
        let memory_binder = gpu::
            GlobalBinder::new(
                event_loop.gpu().device().clone(),
                gpu::MemoryProperties::DEVICE_LOCAL,
                gpu::MemoryProperties::HOST_VISIBLE
            );
        event_loop
            .gpu()
            .create_resources(
                [gpu::BufferCreateInfo::new(
                    &mut id, 
                    &memory_binder,
                    12.max(min_uniform_buffer_offset_alignment) * 3,
                    gpu::BufferUsages::UNIFORM_BUFFER |
                    gpu::BufferUsages::TRANSFER_DST,
                ).unwrap()],
                [],
            )?;
        Ok(id)
    });
    let timeline_semaphore = globals.add(|event_loop| {
        let mut id = Default::default();
        event_loop
            .gpu()
            .create_timeline_semaphores([gpu::semaphore_create_info(&mut id, 3)])?;
        Ok(id)
    });
    let mut timeline_value = 3;
    #[derive(Default)]
    struct FrameBufferState {
        pipelines: SwapLock<HashMap<gpu::Format, gpu::GraphicsPipelineId>>,
        extent: AtomicU64,
    }
    let fb_state: Arc<FrameBufferState> = Default::default();
    let mut hues = [0.0, PI / 3.0, 2.0 * PI / 3.0];
    let sat: f32 = 94.0 / 100.0;
    let val: f32 = 97.0 / 100.0;
    leimu::Leimu::new(
        entry,
        logical_device,
        leimu::default_attributes(),
        &globals,
        |event_loop, event| {
            match event {
                leimu::Event::Update => {
                    let window = *window;
                    if !event_loop.is_window_valid(window) {
                        event_loop.exit();
                    }
                    let buffer_id = *buffer;
                    let fb_state = fb_state.clone(); 
                    event_loop.gpu().schedule_commands(|schduler| {
                        schduler.new_commands::<gpu::NewGraphicsCommands>(
                            queue.clone(),
                            move |cmd| {
                                let mut copy_cmd = cmd.copy_commands();
                                let buffer_offset = timeline_value % 3 * 12.max(
                                    min_uniform_buffer_offset_alignment
                                );
                                copy_cmd.update_buffer(
                                    buffer_id,
                                    buffer_offset,
                                    &[
                                        hsva_to_srgb_pack32(hues[0], sat, val),
                                        hsva_to_srgb_pack32(hues[1], sat, val),
                                        hsva_to_srgb_pack32(hues[2], sat, val),
                                    ],
                                    gpu::CommandOrdering::Lenient,
                                )?;
                                let (image_view, format) = cmd.swapchain_image_view(
                                    window.surface_id()
                                )?;
                                cmd.render(
                                    gpu::RenderingInfo::default(),
                                    &[
                                        gpu::PassAttachment
                                            ::new(image_view)
                                            .with_load_op(gpu::AttachmentLoadOp::Clear)
                                            .with_clear_value(
                                                gpu::ClearColorValue::Float([0.01, 0.01, 0.01, 0.5])
                                            ),
                                    ],
                                    &gpu::DepthStencilAttachment::None,
                                    |pass| {
                                        let frame_buffer_size = fb_state.extent.load(
                                            atomic::Ordering::Acquire
                                        );
                                        let (width, height) = (
                                            (frame_buffer_size & 0xFFFFFFFF) as u32,
                                            (frame_buffer_size >> 32) as u32,
                                        );
                                        pass.dynamic_draw(|cmd| {
                                            let mut pipeline_cmd = cmd.bind_pipeline(
                                                *fb_state.pipelines.load().get(&format).unwrap(),
                                                &[gpu::Viewport
                                                    ::default()
                                                    .width(width as f32)
                                                    .height(height as f32)
                                                ],
                                                &[gpu::Scissor
                                                    ::default()
                                                    .width(width)
                                                    .height(height)
                                                ],
                                            )?;
                                            pipeline_cmd.push_descriptor_bindings(&[
                                                gpu::PushDescriptorBinding::new(
                                                    c"colors",
                                                    0,
                                                    gpu::DescriptorInfos::buffers(&[
                                                        gpu::descriptor_buffer_info(buffer_id)
                                                            .offset(buffer_offset)
                                                        .range(12)
                                                    ]),
                                                    gpu::CommandBarrierInfo::new(
                                                        gpu::CommandOrdering::Strict,
                                                        gpu::ExplicitAccess::SHADER_READ,
                                                    )
                                                )?,
                                            ])?;
                                            pipeline_cmd.begin_drawing(
                                                gpu::DrawInfo
                                                    ::default()
                                                    .vertex_count(3),
                                                    &[], None,
                                                    |cmd| {
                                                        cmd.draw()?;
                                                        Ok(())
                                                    }
                                                )?;
                                            Ok(())
                                        })?;
                                        Ok(())
                                    }
                                )?;
                                Ok(())
                            },
                        )?.with_wait_semaphore(
                            *timeline_semaphore,
                            timeline_value - 2,
                            gpu::MemoryDependencyHint::TRANSFER,
                        ).with_signal_semaphore(
                            *timeline_semaphore,
                            timeline_value + 1,
                        );
                        Ok(())
                    })?;
                    timeline_value += 1;
                    for hue in &mut hues {
                        *hue = (*hue + event_loop.delta_time_secs_f32()) % TAU;
                    }
                    Ok(())
                },
                leimu::Event::GpuEvent(event) => {
                    match event {
                        gpu::Event::SwapchainCreated {
                            surface_id: _, new_format, new_size, image_count: _
                        } => {
                            let mut extent = new_size.0 as u64;
                            extent |= (new_size.1 as u64) << 32;
                            fb_state.extent.store(extent, atomic::Ordering::Release);
                            if fb_state.pipelines.load().get(&new_format).is_none() {
                                fb_state.pipelines
                                .modify(|pipelines| {
                                    pipelines
                                        .entry(new_format)
                                        .or_try_insert_with_key(|&format| {
                                            let mut id = Default::default();
                                            let mut batch = event_loop
                                                .gpu()
                                                .create_pipeline_batch(pipeline_cache.clone())?;
                                            batch 
                                                .with_graphics_pipelines([
                                                    gpu::GraphicsPipelineCreateInfo
                                                        ::new(&mut id, *shader_set)
                                                        .with_color_output(
                                                            format,
                                                            gpu::ColorComponents::default(),
                                                            None,
                                                        )
                                                ]);
                                            let _ = batch.build()?;
                                            leimu::error::Result::Ok(id)
                                        })?;
                                    leimu::Result::Ok(())
                                })?;
                            } 
                            Ok(())
                        },
                    }
                }
                _ => Ok(())
            }
        }
    )?.run()?;
    Ok(())
}

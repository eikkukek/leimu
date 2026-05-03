use leimu::{
    Error, EventError, Leimu, Entry,
    core::collections::{EntryExt, HashMap},
    default, gpu::{self, ext::push_descriptor},
    sync::{Arc, RwLock},
};

fn main() -> leimu::Result<()> {
    let entry = Entry::new()?;
    let instance = entry.create_instance(
        c"flame",
        gpu::make_api_version(0, 1, 0, 0),
        [gpu::layer_khronos_validation(false)],
    )?;
    let suitable = instance.enumerate_suitable_physical_devices(
        gpu::default_device_attributes()
            .with_device_extension(push_descriptor::Extension)
    )?;
    let mut selected = 0;
    for (idx, device) in suitable.enumerate() {
        if device.device_type() == gpu::PhysicalDeviceType::DISCRETE_GPU {
            selected = idx;
        }
    }
    let device = &suitable[selected];
    let idx = device
        .enumerate_queue_families()
        .into_iter()
        .find_map(|(i, queue)| {
            (queue.queue_flags
                .contains(
                    gpu::QueueFlags::GRAPHICS |
                    gpu::QueueFlags::COMPUTE
                )
            ).then_some(i)
        }).ok_or_else(|| Error::just_context_tracked("failed to find suitable queue"))?;
    let mut queue = gpu::DeviceQueue::uninit();
    let queue_info = gpu::DeviceQueueCreateInfo::new(
        &mut queue,
        "primary queue",
        idx,
        0,
    );
    let device = suitable.create_device(selected, [queue_info])?;
    let queue = queue.get()?;
    let globals = leimu::create_globals();
    #[repr(C)]
    struct Particle {
        pos: (u32, u32),
        velocity: (u32, u32),
        ty: i32,
        lifetime: f32,
        size: f32,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Meta {
        resolution: (f32, f32),
        mouse_pos: (f32, f32),
        time: f32,
        delta_time: f32,
        intenstiy: f32,
    }
    const MAX_PARTICLES: gpu::DeviceSize = 8 * 20;
    const NUM_INVOCATIONS: u32 = 8;
    #[derive(Clone, Copy)]
    struct Buffer {
        id: gpu::BufferId,
        size: gpu::DeviceSize,
    }
    let buffers = globals.add(move |event_loop| {
        let gpu = event_loop.gpu();
        let mut particle = default();
        let mut seed = default();
        let binder = gpu::GlobalBinder::new(
            gpu.device().clone(),
            gpu::MemoryProperties::DEVICE_LOCAL,
            gpu::MemoryProperties::DEVICE_LOCAL
        );
        let particle_size = (MAX_PARTICLES + 1) * size_of::<Particle>() as gpu::DeviceSize;
        let seed_size = MAX_PARTICLES * size_of::<f32>() as gpu::DeviceSize;
        gpu.create_resources(
            [   gpu::BufferCreateInfo::new(
                    &mut particle,
                    &binder,
                    particle_size,
                    gpu::BufferUsages::STORAGE_BUFFER | gpu::BufferUsages::TRANSFER_DST,
                ).unwrap(),
                gpu::BufferCreateInfo::new(
                    &mut seed,
                    &binder,
                    seed_size,
                    gpu::BufferUsages::STORAGE_BUFFER | gpu::BufferUsages::TRANSFER_DST
                ).unwrap(),
            ],
            [],
        )?;
        Ok((
            Buffer {
                id: particle,
                size: particle_size,
            },
            Buffer {
                id: seed,
                size: seed_size,
            },
        ))
    });
    let compute_pipeline = globals.add(move |event_loop| {
        let gpu = event_loop.gpu();
        let shader = gpu::default_shader_attributes()
            .with_name("compute shader")
            .with_stage(gpu::ShaderStage::Compute)
            .with_glsl("
                #version 450

                #extension GL_EXT_nonuniform_qualifier : require

                const uint PARTICLE_TYPE_NONE = 0;
                const uint PARTICLE_TYPE_SMOKE = 1;

                struct Particle {
                    vec2 pos;
                    vec2 velocity;
                    uint type;
                    float lifetime;
                    float size;
                };

                layout (local_size_x = 8) in;

                layout(std140, set = 0, binding = 0) buffer Particles {
                    uint len;
                    Particle data[];
                } particles;

                layout(set = 0, binding = 1) buffer Seeds {
                    float data[];
                } seeds;

                layout(push_constant) uniform Meta {
                    vec2 resolution;
                    vec2 mouse_pos;
                    float time;
                    float delta_time;
                    float intensity;
                } meta;

                const float PI = 3.14159f;

                float random(float x) {
                    return fract(sin(x) * 100000.0f);
                }

                void main() {
                    uint gID = gl_GlobalInvocationID.x;
                    if (gID < particles.len) {
                        Particle particle = particles.data[gID];
                        float seed = seeds.data[gID];
                        particle.pos += particle.velocity * meta.delta_time;
                        particle.lifetime += meta.delta_time;
                        if (particle.type == PARTICLE_TYPE_NONE &&
                            particle.lifetime > 0.1f
                        ) {
                            float pos_off = seed;
                            particle.pos = meta.mouse_pos;
                            float diff = meta.mouse_pos.x - 0.5;
                            particle.pos.x = 0.5 + diff * (meta.resolution.x/meta.resolution.y);
                            seed = random(seed);
                            particle.pos.x += mod(pos_off, 0.01f) - 0.005f;
                            seed = random(seed);
                            float x = seed;
                            x = mod(x * 1000.0f, PI * 2.0f);
                            const float area = 4.0f * PI / 5.0f;
                            x /= 2.0f * PI / area;
                            x += PI * 0.5f - area * 0.5;
                            particle.velocity = vec2(cos(x), -sin(x)) * 0.1f;
                            //particle.velocity.y -= 0.3f;
                            particle.lifetime = 0.0f;
                            particle.type = PARTICLE_TYPE_SMOKE;
                            particle.size = 1.0f;
                        }
                        if (particle.type == PARTICLE_TYPE_SMOKE) {
                            if (particle.lifetime < mod(seed, meta.intensity)
                            )
                            {
                                if (particle.velocity.x > 0.0f) {
                                    particle.velocity.x = clamp(
                                        particle.velocity.x - meta.delta_time * abs(particle.velocity.x),
                                        0.0f, 1000.0f
                                    );
                                } else {
                                    particle.velocity.x = clamp(
                                        particle.velocity.x + meta.delta_time * abs(particle.velocity.x),
                                        -1000.0f, 0.0f
                                    );
                                }
                                particle.size = clamp(particle.size - meta.delta_time,
                                    0.05f, 1.0f
                                );
                                particles.data[gID] = particle;
                            } else {
                                particles.data[gID].type = PARTICLE_TYPE_NONE;
                            }
                        }
                        seeds.data[gID] = seed;
                    }
                }
            ").build(gpu)?;
        let set = gpu.create_shader_set(
            [shader],
            gpu::default_shader_set_attributes()
                .with_descriptor_set_layout_flags(
                    0, gpu::DescriptorSetLayoutFlags::PUSH_DESCRIPTOR)
        )?;
        let gpu = event_loop.gpu();
        let mut compute_id = default();
        let mut batch = gpu.create_pipeline_batch(None)?;
        batch
            .with_compute_pipelines([gpu::ComputePipelineCreateInfo::new(
                &mut compute_id, set,
            )]);
        let _ = batch.build()?;
        Ok(compute_id)
    });
    let graphics_set = globals.add(move |event_loop| {
        let gpu = event_loop.gpu();
        let vertex = gpu::default_shader_attributes()
            .with_name("vertex shader")
            .with_stage(gpu::ShaderStage::Vertex)
            .with_glsl("
                #version 450

                vec2 positions[6] = vec2[](
                    vec2(1.0, 1.0),
                    vec2(0.0, 1.0),
                    vec2(0.0, 0.0),
                    vec2(1.0, 0.0),
                    vec2(1.0, 1.0),
                    vec2(0.0, 0.0)

                );

                vec2 uvs[6] = vec2[](
                    vec2(1.0, 1.0),
                    vec2(0.0, 1.0),
                    vec2(0.0, 0.0),
                    vec2(1.0, 0.0),
                    vec2(1.0, 1.0),
                    vec2(0.0, 0.0)
                );

                layout(location = 0) out vec2 out_uv;

                void main() {
                    uint idx = gl_VertexIndex;
                    out_uv = uvs[idx];
                    gl_Position = vec4((positions[idx] - vec2(0.5, 0.5)) * 2.0f, 0.0f, 1.0f);
                }
            ").build(gpu)?;
        let fragment_shader = gpu::default_shader_attributes()
            .with_name("fragment shader")
            .with_stage(gpu::ShaderStage::Fragment)
            .with_glsl("
                #version 450

                layout(location = 0) in vec2 in_uv;
                layout(location = 0) out vec4 out_color;

                const uint PARTICLE_TYPE_NONE = 0;
                const uint PARTICLE_TYPE_SMOKE = 1;

                struct Particle {
                    vec2 pos;
                    vec2 velocity;
                    uint type;
                    float lifetime;
                    float size;
                };

                layout(std140, set = 0, binding = 0) readonly buffer Particles {
                    uint len;
                    Particle data[];
                } particles;

                layout(push_constant) uniform Meta {
                    vec2 resolution;
                    vec2 mouse_pos;
                    float time;
                    float delta_time;
                    float intensity;
                } meta;

                #define PI 3.14159265359

                float mod_time(float x) {
                    return mod(meta.time, PI * 2.0f * x) * 1.0f / x;
                }

                void main() {
                    vec4 color = vec4(0.0f);
                    float aspect = meta.resolution.x / meta.resolution.y;
                    vec2 frag_pos = in_uv;
                    frag_pos.x *= aspect;
                    {
                        vec2 uv = frag_pos * 0.8f;
                        float deg = mod_time(12.0f);
                        float x = uv.x * cos(deg) - uv.y * sin(deg);
                        float y = uv.x * sin(deg) + uv.y * cos(deg);
                        float r = cos(10000.0 * x / 50.0f) * 5.0f;
                        r += sin(10000.0 * y / 50.0f) * 5.0;
                        vec2 wave = cos(
                            vec2(cos(x * 1.33) * 0.5 + meta.time, sin(y * 0.56) * 0.5 + meta.time * 0.5)
                        ) + vec2(0.5f);
                        vec3 col = cos(
                            (6.0 - cos(mod_time(0.75))) * 0.1 * r *
                            vec3(wave.x, wave.y, wave.x * wave.y) + meta.time
                        );
                        color += vec4(col, 0.1);
                    }
                    for (uint i = 0; i < particles.len; i++) {
                        Particle particle = particles.data[i];
                        vec2 pos = particle.pos;
                        pos.x = pos.x + (aspect - 1.0) / 2.0f;
                        float size = 0.015f * particle.size;
                        vec4 rect = vec4(pos - vec2(size), pos + vec2(size));
                        vec2 hv = step(rect.xy, frag_pos) * step(frag_pos, rect.zw);
                        float on_off = hv.x * hv.y;
                        if (particle.type == PARTICLE_TYPE_SMOKE) {
                            vec4 col1 = vec4(1.0f, 0.0f, 0.0f, 1.0f);
                            vec4 col2 = vec4(1.0f, 0.0f, 142.0f / 255.0f, 0.5f);
                            vec4 new = mix(
                                vec4(0.0f),
                                mix(col2, col1, particle.size),
                                on_off
                            );
                            color = vec4(
                                new.xyz * new.a + color.xyz * (1.0 - new.a),
                                new.a + color.a * (1.0 - new.a)
                            );
                        }
                    }
                    out_color = color;
                }
            ").build(gpu)?;
        let set = gpu.create_shader_set(
            [vertex, fragment_shader],
            gpu::default_shader_set_attributes()
                .with_descriptor_set_layout_flags(
                    0, gpu::DescriptorSetLayoutFlags::PUSH_DESCRIPTOR
                ),
        )?;
        Ok(set)
    });
    #[derive(Default)]
    struct FrameData {
        current_pipeline: gpu::GraphicsPipelineId,
        resolution: (u32, u32),
    }
    struct FrameContext {
        pipelines: HashMap<gpu::Format, gpu::GraphicsPipelineId>,
        data: Arc<RwLock<FrameData>>,
    }
    let mut frame_ctx = FrameContext {
        pipelines: default(),
        data: default(),
    };
    let window = globals.add(|event_loop| {
        Ok(event_loop.create_window(leimu::win::default_attributes()
            .with_size(540, 540)
            .with_resizable(true)
        )?)
    });
    let mut first_frame = true;
    let mut seed = 0.0;
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Config {
        intensity: f32,
    }
    let config = Config {
        intensity: 5.0,
    };
    let start_time = std::time::Instant::now();
    Leimu::new(
        entry,
        device,
        leimu::default_attributes()
            .with_desired_buffered_frames(3.try_into().unwrap()),
        &globals,
        |event_loop, event| {
            let surface_id = window.surface_id();
            let Some(mut window) = event_loop.get_window(*window) else {
                event_loop.exit();
                return Ok(())
            };
            if window
                .key_state(leimu::win::KeyCode::Escape)
                .pressed()
            {
                window.close();
                return Ok(())
            }
            let gpu = event_loop.gpu();

            match event {
                leimu::Event::Update => {
                    gpu.schedule_commands(|scheduler| {
                        let (particle_buffer, seed_buffer) = *buffers;
                        let cmd1 = scheduler.new_commands::<gpu::NewGraphicsCommands>(
                            queue.clone(), 
                            move |cmd| {
                                if first_frame {
                                    let mut cmd = cmd.copy_commands();
                                    cmd.fill_buffer(
                                        particle_buffer.id,
                                        0, None,
                                        0,
                                        gpu::CommandOrdering::Lenient,
                                    )?;
                                    let mut i = 0;
                                    let end = seed_buffer.size;
                                    let mut value = 0.12345f32;
                                    while i < end {
                                        cmd.update_buffer(
                                            seed_buffer.id,
                                            i, &[value],
                                            gpu::CommandOrdering::Strict
                                        )?;
                                        value = value.cos().fract();
                                        i += 4;
                                    }
                                    cmd.update_buffer(
                                        particle_buffer.id,
                                        0, &[MAX_PARTICLES as u32],
                                        gpu::CommandOrdering::Strict
                                    )?;
                                }
                                Ok(())
                            },
                        )?.id();
                        let compute_pipeline = *compute_pipeline;
                        let frame_data = frame_ctx.data.clone();
                        let delta_time = window.delta_time_secs_f32();
                        seed += delta_time;
                        let fres = {
                            let data = frame_data.read();
                            (data.resolution.0 as f32, data.resolution.1 as f32)
                        };
                        let mouse_pos = window.normalized_cursor_position_f32();
                        let time = start_time.elapsed().as_secs_f32();
                        let cmd2 = scheduler.new_commands::<gpu::NewComputeCommands>(
                            queue.clone(),
                            move |cmd| {
                                cmd.bind_pipeline(compute_pipeline, |cmd| {
                                    cmd.push_descriptor_bindings(&[
                                        gpu::PushDescriptorBinding::new(
                                            c"particles",
                                            0,
                                            &gpu::descriptor_buffer_info(particle_buffer.id),
                                            None,
                                        )?,
                                        gpu::PushDescriptorBinding::new(
                                            c"seeds",
                                            0,
                                            &gpu::descriptor_buffer_info(seed_buffer.id),
                                            None,
                                        )?,
                                    ])?;
                                    cmd.push_constants(0, &[Meta {
                                        resolution: fres,
                                        mouse_pos,
                                        time,
                                        delta_time,
                                        intenstiy: config.intensity,
                                    }])?;
                                    Ok(())
                                })?;
                                cmd.dispatch(MAX_PARTICLES as u32 / NUM_INVOCATIONS, 1, 1)?;
                                Ok(())
                            },
                        )?.with_dependencies([gpu::CommandDependency::new(
                            cmd1, gpu::MemoryDependencyHint::COMPUTE_SHADER
                        )]).id();
                        let frame_data = frame_ctx.data.clone();
                        scheduler.new_commands::<gpu::NewGraphicsCommands>(
                            queue.clone(),
                            move |cmd| {
                                let frame_data = frame_data.read();
                                let (swapchain_image, _) = cmd.swapchain_image_view(surface_id)?;
                                cmd.render(
                                    default(),
                                    &[
                                        gpu::PassAttachment::new(swapchain_image)
                                            .with_load_op(gpu::AttachmentLoadOp::Clear)
                                            .with_store_op(gpu::AttachmentStoreOp::Store)
                                            .with_preserve_contents(false),
                                    ],
                                    &default(),
                                    |pass| {
                                        pass.dynamic_draw(|cmd| {
                                            let mut cmd = cmd.bind_pipeline(
                                                frame_data.current_pipeline,
                                                &[gpu::Viewport
                                                    ::default()
                                                    .width(frame_data.resolution.0 as f32)
                                                    .height(frame_data.resolution.1 as f32)
                                                ],
                                                &[gpu::Scissor
                                                    ::default()
                                                    .width(frame_data.resolution.0)
                                                    .height(frame_data.resolution.1)
                                                ]
                                            )?;
                                            cmd.push_descriptor_bindings(&[
                                                gpu::PushDescriptorBinding::new(
                                                    c"particles",
                                                    0,
                                                    &gpu::descriptor_buffer_info(particle_buffer.id),
                                                    None,
                                                )?,
                                            ])?;
                                            cmd.push_constants(0, &[Meta {
                                                resolution: fres,
                                                mouse_pos,
                                                time,
                                                delta_time,
                                                intenstiy: config.intensity,
                                            }])?;
                                            cmd.begin_drawing(
                                                gpu::DrawInfo::default()
                                                    .vertex_count(6),
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
                        })?.with_dependencies([gpu::CommandDependency::new(
                            cmd2, gpu::MemoryDependencyHint::FRAGMENT_SHADER,
                        )]);
                        Ok(())
                    })?;
                    first_frame = false;
                },
                leimu::Event::GpuEvent(event) => {
                    let gpu::Event::SwapchainCreated {
                        new_format, new_size, image_count, ..
                    } = event;
                    if image_count < 3 {
                        return Err(EventError::just_context(
                            "this example requires at least"
                        ))
                    }
                    let current_pipeline = *frame_ctx
                        .pipelines
                        .entry(new_format)
                        .or_try_insert_with(|| {
                            let mut batch = gpu.create_pipeline_batch(None)?;
                            let mut new_id = default();
                            batch.with_graphics_pipelines([gpu::GraphicsPipelineCreateInfo
                                ::new(&mut new_id, *graphics_set)
                                .with_color_output(
                                    new_format,
                                    gpu::ColorComponents::RGBA,
                                    Some(gpu::ColorOutputBlendState {
                                        src_color_blend_factor: gpu::BlendFactor::SRC_ALPHA,
                                        dst_color_blend_factor: gpu::BlendFactor::ONE_MINUS_SRC_ALPHA,
                                        color_blend_op: gpu::BlendOp::ADD,
                                        src_alpha_blend_factor: gpu::BlendFactor::ONE,
                                        dst_alpha_blend_factor: gpu::BlendFactor::ONE_MINUS_SRC_ALPHA,
                                        alpha_blend_op: gpu::BlendOp::ADD,
                                    }),
                                )
                            ]);
                            let _ = batch.build()?;
                            leimu::Result::Ok(new_id)
                        })?;
                    let mut data = frame_ctx.data.write();
                    data.resolution = new_size;
                    data.current_pipeline = current_pipeline;
                },
                _ => {}
            }
            Ok(())
        }
    )?.run()?;
    Ok(())
}

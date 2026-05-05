use tuhka::vk;
use leimu_mem::{
    alloc::LocalAlloc, arena, int::NonZeroOption, vec::{FixedVec32, NonNullVec32}
};

use crate::{
    gpu::prelude::{
        command_cache::*,
        *,
    },
    core::{OptionExt, TryExtend},
    error::*,
};

pub struct PipelineCommands<'a, 'b> {
    pub(super) gpu: Gpu,
    pub(super) command_buffer: vk::CommandBuffer,
    pipeline: PipelineHandle,
    pub(super) cache: &'a mut PipelineCommandCache,
    pub(super) alloc: &'a dyn LocalAlloc<Error = arena::Error>,
    pub(super) buffers: &'a ResourceReadGuard<'b, BufferMeta, BufferId>,
    pub(super) images: &'a ResourceReadGuard<'b, ImageMeta, ImageIndex>,
}

impl<'a, 'b> PipelineCommands<'a, 'b> {

    pub(crate) unsafe fn new(
        gpu: Gpu,
        command_buffer: vk::CommandBuffer,
        pipeline: PipelineHandle,
        cache: &'a mut PipelineCommandCache,
        alloc: &'a dyn LocalAlloc<Error = arena::Error>,
        buffers: &'a ResourceReadGuard<'b, BufferMeta, BufferId>,
        images: &'a ResourceReadGuard<'b, ImageMeta, ImageIndex>,
    ) -> Self {
        Self {
            gpu,
            command_buffer,
            pipeline,
            cache,
            alloc,
            buffers,
            images,
        }
    }

    /// Binds [`descriptor sets`][1] for use in subsequent draw calls.
    ///
    /// Binds descriptor sets from `first_set` to `first_set` + `sets.len()` for the currently
    /// bound [`pipeline`][2].
    ///
    /// [`Barrier information`][3] *can* be specified for stricter memory barrier behaviour when
    /// the resources are consumed.
    ///
    /// # Valid usage
    /// - A [`pipeline`][2] *must* be bound before calling this command.
    /// - Each id in `sets` *must* be a valid [`ShaderResourceId`].
    /// - `first_set` + `sets.len()` *must* be less than or equal to the descriptor set count of 
    ///   the [`shader set`][4] used to [`create`][5] the currently bound pipeline.
    /// - Each set bound *must* have the same [`descriptor type`][6] and descriptor count as the
    ///   descriptor set layout defined at that index in the [`shader set`][4].
    /// - If a descriptor set layout in the [`shader_set`][4] was created with the
    ///   [`push descriptor flag`][7] set, it *must* not be bound by this command and *must* be
    ///   pushed by [`push_descriptor_set`][8] instead.
    /// - When the [`descriptor sets`][1] are consumed by [`commands`][9], they *must* be in
    ///   a valid state.
    /// - Each [`barrier info`][3] in `barrier_infos` *should* point to a valid set index and set
    ///   binding index.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html>
    ///
    /// [1]: DescriptorSetId
    /// [2]: PipelineHandle
    /// [3]: BindingBarrierInfo
    /// [4]: ShaderSet
    /// [5]: PipelineBatch
    /// [6]: DescriptorType
    /// [7]: DescriptorSetFlags::PUSH_DESCRIPTOR
    /// [8]: PipelineCommands::push_descriptor_bindings
    /// [9]: CommandScheduler
    pub fn bind_descriptor_sets(
        &mut self,
        first_set: u32,
        sets: &[DescriptorSetId],
        barrier_infos: &[BindingBarrierInfo],
    ) -> Result<&mut Self>
    {
        {
        if sets.is_empty() {
            return Ok(self)
        }
        let tmp_alloc = self.gpu.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let n_sets = sets.len() as u32;
        let shader_set =    self.pipeline.shader_set();
        let set_layouts = shader_set.descriptor_set_layouts();
        let sets_end = first_set + n_sets;
        if sets_end > set_layouts.len() as u32 {
            return Err(Error::just_context(format!(
                "{}{}",
                format_args!("first set {first_set} + set count {n_sets} was out of range of"),
                format_args!("pipeline shader set descriptor set count {}", set_layouts.len())
            )))
        }
        let mut descriptor_sets = FixedVec32::with_capacity(
            n_sets, &tmp_alloc,
        ).context("alloc failed")?;
        let pools = self.gpu.get_descriptor_pools();
        let mut write_cache = FixedVec32
            ::with_len_with(pools.capacity(), |_| None, &tmp_alloc)
            .context("alloc failed")?;
        let mut stage_flags = vk::ShaderStageFlags::empty();
        for (i, set_layout) in set_layouts[first_set as usize..sets_end as usize].iter().enumerate() {
            let set_id = sets[i];
            let pool_id = set_id.pool_id();
            let pool = OptionExt::get_or_try_insert_with(
                write_cache
                    .get_mut(pool_id.slot_index().index() as usize)
                    .ok_or_else(|| Error::just_context(format!("invalid descriptor pool id {pool_id}")))?,
                || {
                    Ok(pools.get(pool_id.slot_index())
                        .context_with(|| format!(
                            "invalid descriptor pool id {pool_id}",
                        ))?.write())
                })?;
            let set = pool.get_descriptor_set(set_id)?;
            if set_layout.is_push_descriptor() {
                return Err(Error::just_context(format!(
                    "{}{}",
                    "attempting to bind descriptor set with descriptor set layout that was ",
                    "created with the push descriptor flag",
                ) 
                ))
            }
            if set_layout.bindings
                .iter()
                .map(|b| {
                    stage_flags |= b.stage_flags.into();
                    (b.descriptor_type, b.descriptor_count)
                }).ne(set
                    .bindings()
                    .iter()
                    .map(|b| {
                        (b.ty(), b.descriptor_count())
                    })
                )
            {
                return Err(Error::just_context(format!(
                    "{}{}",
                    format_args!("pipeline shader set descriptor set at set index {} doesn't match ", first_set + i as u32),
                    format_args!("with the descriptor set {set_id} bound"),
                )))
            }
            descriptor_sets.push(set.handle());
        }
        let mut barriers = NonNullVec32
            ::with_capacity(barrier_infos.len() as u32, self.alloc)
            .context("alloc failed")?
            .into_static();
        barriers.append(barrier_infos);
        let mut set_ids = NonNullVec32
            ::with_capacity(n_sets, self.alloc)
            .context("alloc failed")?
            .into_static();
        set_ids.fast_append(sets);
        self.cache.descriptor_set_binds.push(DescriptorSetBindCall { sets: set_ids, barriers });
        unsafe {
            let info = vk::BindDescriptorSetsInfo {
                stage_flags,
                layout: shader_set.pipeline_layout(),
                first_set,
                descriptor_set_count: descriptor_sets.len(),
                p_descriptor_sets: descriptor_sets.as_ptr(),
                ..Default::default()
            };
            self.gpu.device().cmd_bind_descriptor_sets2(
                self.command_buffer,
                &info,
            );
        }
        }
        Ok(self)
    }

    /// Pushes individual [`descriptor bindings`][1] for use in subsequent draw calls.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html>
    ///
    /// [1]: DescriptorSetLayoutBinding
    pub fn push_descriptor_bindings(
        &mut self,
        bindings: &[PushDescriptorBinding<'_>],
    ) -> Result<&mut Self> {
        if bindings.is_empty() {
            return Ok(self)
        }
        let tmp_alloc = self.gpu.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let shader_set = self.pipeline.shader_set();
        let command_buffer = self.command_buffer;
        self.cache.push_descriptor_binding_calls.clear();
        let mut infos = FixedVec32::<
            (
                FixedVec32<vk::DescriptorBufferInfo, _>,
                FixedVec32<vk::DescriptorImageInfo, _>,
            ),
            _
        >::with_capacity(bindings.len() as u32, &tmp_alloc)
            .context("alloc failed")?;
        for binding in bindings {
            let (set, layout_binding) = shader_set.push_descriptor_binding(
                binding.binding,
            ).ok_or_else(|| Error::just_context(format!(
                "binding {:?} is not from a push descriptor set", binding.binding,
            )))?;
            let min_uniform_buffer_offset_alignment
                = self.gpu.device_limits().min_uniform_buffer_offset_alignment();
            match layout_binding.descriptor_type {
                crate::buffer_descriptor_types!() => {
                    let Some(buffers) = binding.buffer_infos() else {
                        return Err(Error::just_context(format!(
                            "expected buffer descriptors for binding {:?}",
                            binding.binding
                        )))
                    };
                    let n = buffers.len() as u32;
                    if binding.starting_index + n > layout_binding.descriptor_count {
                        return Err(Error::just_context(format!(
                            "binding {:?} starting index {} + descriptor count {n} was more than layout descriptor count {}",
                            binding.binding, binding.starting_index, layout_binding.descriptor_count,
                        )))
                    }
                    let buffer_usage = layout_binding.descriptor_type
                        .buffer_usage()
                        .unwrap();
                    let mut call = PushDescriptorBindingCall {
                        barrier: binding.barrier_info,
                        stage_flags: layout_binding.stage_flags,
                        buffers: NonNullVec32
                            ::with_capacity(n, self.alloc)
                            .context("alloc failed")?
                            .into_static(),
                        images: Default::default(),
                    };
                    call.buffers.fast_append(buffers);
                    let mut buffer_infos = FixedVec32
                        ::with_capacity(n, &tmp_alloc)
                        .context("alloc failed")?;
                    buffer_infos.try_extend(
                        buffers.iter().enumerate().map(|(i, info)| {
                            let buffer = self.buffers
                                .get(info.buffer_id)
                                .context_with(|| format!(
                                    "failed to get buffer for binding {:?} at descriptor index {}",
                                    binding.binding, binding.starting_index + i as u32,
                                ))?;
                            if let Some(err) = buffer.validate_usage(buffer_usage) {
                                return Err(Error::new(err, format!(
                                    "binding {:?} buffer usage mismatch at descriptor index {}",
                                    binding.binding, binding.starting_index + i as u32,
                                )))
                            }
                            if !info.offset.is_multiple_of(min_uniform_buffer_offset_alignment) {
                                return Err(Error::just_context(format!(
                                    "buffer offset {} is not a multiple of min uniform buffer offset alignment {}",
                                    info.offset, min_uniform_buffer_offset_alignment,
                                )))
                            }
                            Ok(vk::DescriptorBufferInfo {
                                buffer: buffer.handle(),
                                offset: info.offset,
                                range: info.range.unwrap_or_sentinel(vk::WHOLE_SIZE),
                            })
                        })
                    )?;
                    self.cache.push_descriptor_binding_cache.insert_writes(
                        set,
                        layout_binding.stage_flags,
                        &[vk::WriteDescriptorSet {
                            dst_binding: layout_binding.binding,
                            dst_array_element: binding.starting_index,
                            descriptor_count: buffer_infos.len(),
                            p_buffer_info: buffer_infos.as_ptr(),
                            descriptor_type: layout_binding.descriptor_type.into(),
                            ..Default::default()
                        }],
                    );
                    infos.push((buffer_infos, FixedVec32::new(&tmp_alloc)));
                    self.cache.push_descriptor_binding_calls.push(call);
                },
                crate::image_descriptor_types!() => {
                    let Some(images) = binding.image_infos() else {
                        return Err(Error::just_context(format!(
                            "expected image descriptors for binding {:?}",
                            binding.binding,
                        )))
                    };
                    let n = images.len() as u32;
                    if binding.starting_index + n > layout_binding.descriptor_count {
                        return Err(Error::just_context(format!(
                            "binding {:?} starting index {} + descriptor count {n} was more than layout descriptor count {}",
                            binding.binding, binding.starting_index, layout_binding.descriptor_count,
                        )))
                    }
                    let mut call = PushDescriptorBindingCall {
                        barrier: binding.barrier_info,
                        stage_flags: layout_binding.stage_flags,
                        buffers: Default::default(),
                        images: NonNullVec32
                            ::with_capacity(n, self.alloc)
                            .context("alloc failed")?
                            .into_static(),
                    };
                    let image_layout = layout_binding.descriptor_type.shader_image_layout();
                    let image_usage = layout_binding.descriptor_type.image_usage().unwrap();
                    let mut image_infos = FixedVec32
                        ::with_capacity(n, &tmp_alloc)
                        .context("alloc failed")?;
                    call.images.extend(images.iter().map(|image| {
                        (image.clone(), image_layout)
                    }));
                    image_infos.try_extend(
                        images.iter().enumerate().map(|(i, info)| {
                            let mut vk_info = vk::DescriptorImageInfo::default();
                            if let Some(layout) = image_layout
                            {
                                let Some(image_view) = info.image_view_id else {
                                    return Err(Error::just_context(format!(
                                        "expected image view for binding {:?} at descriptor index {}",
                                        binding.binding, binding.starting_index + i as u32,
                                    )))
                                };
                                let image = self.images
                                    .get(image_view.image_id().slot_index())
                                    .context_with(|| format!(
                                        "failed to get image for binding {:?} at descriptor index {}",
                                        binding.binding, binding.starting_index + i as u32,
                                    ))?;
                                if let Some(err) = image.validate_usage(image_usage) {
                                    return Err(Error::new(err, format!(
                                        "binding {:?} image usage mismatch at descriptor index {}",
                                        binding.binding, binding.starting_index + i as u32,
                                    )))
                                }
                                vk_info.image_view = image
                                    .get_view(image_view)
                                    .context_with(|| format!(
                                        "failed to get image view for binding {:?} at descriptor index {}",
                                        binding.binding, binding.starting_index + i as u32,
                                    ))?.handle;
                                vk_info.image_layout = layout.into();
                            }
                            if layout_binding.descriptor_type.requires_sampler() {
                                let Some(sampler) = &info.sampler else {
                                    return Err(Error::just_context(format!(
                                        "expected sampler for binding {:?} at descriptor index {}",
                                        binding.binding, binding.starting_index + i as u32,
                                    )))
                                };
                                vk_info.sampler = sampler.handle().into_inner();
                            }
                            Ok(vk_info)
                        })
                    )?;
                    self.cache.push_descriptor_binding_cache.insert_writes(
                        set,
                        layout_binding.stage_flags,
                        &[vk::WriteDescriptorSet {
                            dst_binding: layout_binding.binding,
                            dst_array_element: binding.starting_index,
                            descriptor_count: image_infos.len(),
                            p_image_info: image_infos.as_ptr(),
                            descriptor_type: layout_binding.descriptor_type.into(),
                            ..Default::default()
                        }]
                    );
                    infos.push((FixedVec32::new(&tmp_alloc), image_infos));
                    self.cache.push_descriptor_binding_calls.push(call);
                },
                _ => return Err(Error::just_context(format!(
                    "descriptor type {} can't be used as a push descriptor",
                    layout_binding.descriptor_type,
                )))
            };
        }
        unsafe {
            self.cache.push_descriptor_binding_cache.push_descriptor_sets(
                self.gpu.device(),
                command_buffer,
                shader_set.pipeline_layout(),
            );
        }
        Ok(self)
    }

    /// Updates the values of [`push constants`][1].
    ///
    /// # Valid usage
    /// - `offset` *must* be a multiple of 4.
    /// - The size of `values` in bytes *must* be a multiple of 4.
    /// - `offset` + the size of `values` in bytes *must* be less than or equal to 
    ///   [`max push constant size`][2].
    ///
    /// [1]: https://docs.vulkan.org/guide/latest/push_constants.html
    /// [2]: PhysicalDeviceLimits::max_push_constants_size
    pub fn push_constants<T>(
        &mut self,
        offset: u32,
        values: &[T],
    ) -> Result<&mut Self>
        where T: Copy
    {
        if !offset.is_multiple_of(4) {
            return Err(Error::just_context(format!(
                "push constant offset {offset} is not a multiple of 4"
            )))
        }
        let size = size_of_val(values) as u32;
        if !size.is_multiple_of(4) {
            return Err(Error::just_context(format!(
                "push constant data size {size} is not a a multiple of 4"
            )))
        }
        if offset + size > self.gpu.device_limits().max_push_constants_size() {
            return Err(Error::just_context(format!(
                "push constant offset {offset} + size {size} is larger than max push constant size {}",
                self.gpu.device_limits().max_push_constants_size()
            )))
        }
        let byte_end = offset + size;
        let shader_set = self.pipeline.shader_set();
        let mut stage_flags = ShaderStageFlags::empty();
        for pc in shader_set.push_constant_ranges() {
            if offset < pc.offset + pc.size &&
                pc.offset < byte_end
            {
                stage_flags |= pc.stage.into();
            }
        }
        let info = vk::PushConstantsInfo {
            layout: shader_set.pipeline_layout(),
            stage_flags: stage_flags.into(),
            offset,
            size,
            p_values: values.as_ptr() as *const core::ffi::c_void,
            ..Default::default()
        };
        unsafe {
            self.gpu.device()
            .cmd_push_constants2(
                self.command_buffer,
                &info
            );
        }
        Ok(self)
    }
}

use core::ffi::CStr;

use ahash::{AHashSet, AHashMap};

use leimu_mem::{
    vec::{Vec32, FixedVec32, NonNullVec32},
    alloc::LocalAlloc,
    arena::{self, ArenaGuard},
    conditional::True,
};
use tuhka::vk;

use crate::{
    core::OptionExt,
    gpu::prelude::*,
    error::*,
};

#[derive(Default)]
struct BufferCommandCache {
    stage_mask: vk::PipelineStageFlags2,
    access_flags: vk::AccessFlags2,
    command_ordering: CommandOrdering,
    ranges: Vec32<(DeviceSize, DeviceSize)>,
}

impl BufferCommandCache {

    pub fn touch(
        &mut self,
        offset: DeviceSize,
        size: DeviceSize,
        stage_mask: vk::PipelineStageFlags2,
        access_flags: vk::AccessFlags2,
        ordering: CommandOrdering,
    ) -> bool {
        let new = self.ranges.is_empty();
        self.ranges.push((offset, size));
        self.stage_mask |= stage_mask;
        self.access_flags |= access_flags;
        if ordering == CommandOrdering::Strict {
            self.command_ordering = CommandOrdering::Strict;
        }
        new
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.stage_mask = vk::PipelineStageFlags2::empty();
        self.access_flags = vk::AccessFlags2::empty();
        self.command_ordering = CommandOrdering::Lenient;
        self.ranges.clear();
    }
}

#[derive(Default)]
pub struct ImageCommandCache {
    layout: Option<ShaderImageLayout>,
    access_mask: Option<ExplicitAccess>,
    stage_mask: vk::PipelineStageFlags2,
    subresource_ranges: AHashSet<BareImageViewId>,
}

impl ImageCommandCache {

    #[inline(always)]
    pub fn touch(
        &mut self,
        layout: ShaderImageLayout,
        stage_mask: vk::PipelineStageFlags2,
        access_mask: Option<ExplicitAccess>,
    ) -> bool
    {
        if let Some(access) = access_mask {
            let current = self.access_mask.get_or_insert(access);
            *current |= access;
        }
        self.stage_mask |= stage_mask;
        match &mut self.layout {
            Some(current) => {
                *current = current.combine(layout);
                false
            },
            None => {
                self.layout = Some(layout);
                true
            },
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.layout = None;
        self.stage_mask = vk::PipelineStageFlags2::empty();
        self.access_mask = None;
        self.subresource_ranges.clear();
    }
}

#[derive(Default)]
pub(crate) struct ShaderResourceCache {
    pub image_memory_barrier_cache: ImageMemoryBarrierCache,
    pub buffer_memory_barrier_cache: BufferMemoryBarrierCache,
    image_cache: AHashMap<ImageIndex, ImageCommandCache>,
    image_id_cache: Vec32<ImageIndex>,
    buffer_cache: AHashMap<BufferId, BufferCommandCache>,
    buffer_id_cache: Vec32<BufferId>,
}

impl ShaderResourceCache {

    #[inline(always)]
    pub fn touch_image(
        &mut self,
        image_view: BareImageViewId,
        layout: ShaderImageLayout,
        stage_mask: vk::PipelineStageFlags2,
        access_mask: Option<ExplicitAccess>,
    )
    {
        let cache = self.image_cache
            .entry(image_view.image_id())
            .or_default();
        if cache.touch(layout, stage_mask, access_mask) {
            self.image_id_cache.push(image_view.image_id());
        }
        cache.subresource_ranges.insert(image_view);
    }

    #[inline(always)]
    pub fn touch_buffer(
        &mut self,
        buffer: BufferId,
        offset: DeviceSize,
        size: DeviceSize,
        stage_mask: vk::PipelineStageFlags2,
        access_mask: ExplicitAccess,
        ordering: CommandOrdering,
    ) {
        let cache = self.buffer_cache
            .entry(buffer)
            .or_default();
        if cache.touch(offset, size, stage_mask, vk::AccessFlags2::from_raw(access_mask.as_raw()), ordering) {
            self.buffer_id_cache.push(buffer);
        }
    }

    pub unsafe fn process(
        &mut self,
        recorder: &mut CommandRecorder<'_, '_>,
        command_buffer: vk::CommandBuffer,
        queue_family_index: u32,
        command_id: CommandId,
        tmp_alloc: &impl LocalAlloc<Error = arena::Error>,
    ) -> Result<()> {
        let mut buffer_memory_barriers = FixedVec32::with_capacity(
            self.buffer_id_cache.len(),
            tmp_alloc
        ).context("alloc failed")?;
        let gpu = recorder.gpu().clone();
        recorder.write_resources(|guard| {
            for &id in &self.buffer_id_cache {
                let buffer = guard.register_buffer(id)?;
                let cache = self.buffer_cache.get_mut(&id).unwrap();
                let state = BufferState {
                    access_mask: cache.access_flags,
                    stage_mask: cache.stage_mask,
                    queue_family_index,
                };
                let ordering = cache.command_ordering;
                let mut barrier_ranges = FixedVec32
                    ::with_capacity(cache.ranges.len(), tmp_alloc)
                    .context("alloc failed")?;
                for &(offset, size) in &cache.ranges {
                    let range = buffer.memory_barrier(
                        offset, size, state,
                        ordering, &mut self.buffer_memory_barrier_cache,
                    ).context_with(|| format!(
                        "memory barrier for buffer with id {id} failed",
                    ))?;
                    barrier_ranges.push(range);
                }
                let memory_barriers = self.buffer_memory_barrier_cache.flush(
                    &barrier_ranges,
                    tmp_alloc,
                )?;
                buffer_memory_barriers.push(memory_barriers);
                cache.reset();
            }
            let buffer_memory_barriers = FixedVec32::flattened(&buffer_memory_barriers, tmp_alloc)
                .context("alloc failed")?;
            let mut image_memory_barriers = FixedVec32::with_capacity(
                self.image_id_cache.len(),
                tmp_alloc,
            ).context("alloc failed")?;
            for &id in &self.image_id_cache {
                let image = guard.register_image(id.slot_index(), command_id.index())?;
                let cache = self.image_cache.get_mut(&id).unwrap();
                let layout = cache.layout.unwrap();
                let state = ImageSubresourceState {
                    stage_mask: cache.stage_mask,
                    access_mask: cache.access_mask
                        .map(|access| access.into())
                        .unwrap_or(layout.access_mask()),
                    layout: layout.into(),
                    queue_family_index,
                };
                let mut barrier_ranges = FixedVec32::with_capacity(
                    cache.subresource_ranges.len() as u32, tmp_alloc
                ).context("alloc failed")?;
                for &view_id in &cache.subresource_ranges {
                    let range = image
                        .view_memory_barrier(
                            state, view_id, true,
                            &mut self.image_memory_barrier_cache,
                        ).context_with(|| format!(
                            "memory barrier for image with id {id} failed"
                        ))?;
                    barrier_ranges.push(range);
                }
                let memory_barriers = self.image_memory_barrier_cache.flush(
                    &barrier_ranges,
                    tmp_alloc
                )?;
                image_memory_barriers.push(memory_barriers);
                cache.reset();
            }
            let image_memory_barriers = FixedVec32::flattened(&image_memory_barriers, tmp_alloc)
                .context("alloc failed")?;
            let dependency_info = vk::DependencyInfo {
                s_type: vk::StructureType::DEPENDENCY_INFO,
                buffer_memory_barrier_count: buffer_memory_barriers.len(),
                p_buffer_memory_barriers: buffer_memory_barriers.as_ptr(),
                image_memory_barrier_count: image_memory_barriers.len(),
                p_image_memory_barriers: image_memory_barriers.as_ptr(),
                ..Default::default()
            };
            unsafe {
                gpu.device().cmd_pipeline_barrier2(command_buffer, &dependency_info);
            }
            self.image_id_cache.clear();
            self.buffer_id_cache.clear();
            Ok(())
        }) 
    }
    
}

#[derive(Clone)]
pub struct PushDescriptorBinding<'a> {
    pub(super) binding: &'a CStr,
    pub(super) starting_index: u32,
    pub(super) infos: DescriptorInfos<'a>,
    pub(super) barrier_info: Option<CommandBarrierInfo>,
}

impl<'a> PushDescriptorBinding<'a> {

    #[inline(always)]
    pub fn new<Infos, Barrier>(
        binding: &'a CStr,
        starting_index: u32,
        infos: Infos,
        barrier_info: Barrier,
    ) -> Result<Self>
        where
            Infos: Into<DescriptorInfos<'a>>,
            Barrier: Into<Option<CommandBarrierInfo>>,
    {
        let infos = infos.into();
        if infos.is_inline_uniform_block() {
            return Err(Error::just_context(
                "push descriptor binding can't be inline uniform block"
            ))
        }
        Ok(Self {
            binding,
            starting_index,
            infos,
            barrier_info: barrier_info.into(),
        })
    }

    #[inline(always)]
    pub fn buffer_infos(&self) -> Option<&[DescriptorBufferInfo]> {
        self.infos.as_buffers()
    }

    #[inline(always)]
    pub fn image_infos(&self) -> Option<&[DescriptorImageInfo]> {
        self.infos.as_images()
    }
}

#[derive(Default)]
struct PushDescriptorWrites {
    stage_flags: ShaderStageFlags,
    writes: Vec32<vk::WriteDescriptorSet<'static>>,
}

#[derive(Default)]
pub struct PushDescriptorBindingsCache {
    bindings: Vec32<PushDescriptorWrites>,
}

impl PushDescriptorBindingsCache {

    /// Inserts [`writes`][1] to this cache.
    ///
    /// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkWriteDescriptorSet.html
    #[inline(always)]
    pub fn insert_writes(
        &mut self,
        set: u32,
        stage_flags: ShaderStageFlags,
        writes: &[vk::WriteDescriptorSet<'static>],
    ) {
        if self.bindings.len() <= set {
            self.bindings.resize_with(set + 1, Default::default);
        }
        let binding = &mut self.bindings[set as usize];
        binding.stage_flags |= stage_flags;
        binding.writes.fast_append(writes);
    }

    #[inline(always)]
    fn clear(&mut self) {
        for binding in &mut self.bindings {
            binding.stage_flags.clear();
            binding.writes.clear();
        }
    }

    /// Pushes the descriptor sets in this cache via [`cmd_push_descriptor_set2`][1]
    ///
    /// # Safety
    /// - `command_buffer` *must* be a valid command buffer handle and it *must* be in a recording
    ///   state.
    /// - `pipeline_layout` *must* be a valid pipeline layout handle and it *must* be compatible
    ///   with this cache.
    /// - All writes in this cache *must* be valid.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html>
    ///
    /// [1]: tuhka::Device::cmd_push_descriptor_set2
    #[inline(always)]
    pub unsafe fn push_descriptor_sets(
        &mut self,
        device: &VkDevice,
        command_buffer: CommandBuffer,
        pipeline_layout: vk::PipelineLayout,
    ) {
        for (i, binding) in self.bindings.iter().enumerate() {
            let set = i as u32;
            if !binding.writes.is_empty() {
                let info = vk::PushDescriptorSetInfo {
                    stage_flags: binding.stage_flags.into(),
                    layout: pipeline_layout,
                    set,
                    descriptor_write_count: binding.writes.len(),
                    p_descriptor_writes: binding.writes.as_ptr(),
                    ..Default::default()
                };
                unsafe {
                    device.cmd_push_descriptor_set2(
                        command_buffer,
                        &info
                    );
                }
            }
        }
        self.clear();
    }
}

pub struct DescriptorSetBindCall {
    pub sets: NonNullVec32<'static, DescriptorSetId>,
    pub barriers: NonNullVec32<'static, BindingBarrierInfo>,
}

pub struct PushDescriptorBindingCall {
    pub barrier: Option<CommandBarrierInfo>,
    pub stage_flags: ShaderStageFlags,
    pub buffers: NonNullVec32<'static, DescriptorBufferInfo>,
    pub images: NonNullVec32<'static, (DescriptorImageInfo, Option<ShaderImageLayout>)>,
}

#[derive(Default)]
pub struct PipelineCommandCache {
    pub descriptor_set_binds: Vec32<DescriptorSetBindCall>,
    pub push_descriptor_binding_calls: Vec32<PushDescriptorBindingCall>,
    pub push_descriptor_binding_cache: PushDescriptorBindingsCache,
}

impl PipelineCommandCache {

    /// Resets the cache.
    ///
    /// # Safety
    /// `alloc` *must* have been the same allocator used to update the cache.
    pub unsafe fn reset<Alloc>(&mut self, alloc: &Alloc)
        where Alloc: LocalAlloc<Error = arena::Error>
    {
        unsafe {
            for bind in &mut self.descriptor_set_binds {
                bind.sets.drop_and_free(alloc);
                bind.barriers.drop_and_free(alloc);
            }
            for call in &mut self.push_descriptor_binding_calls {
                call.buffers.drop_and_free(alloc);
                call.images.drop_and_free(alloc);
            }
            self.descriptor_set_binds.clear();
            self.push_descriptor_binding_calls.clear();
        }
    }

    /// Prepares [`shader resource cache`][1] with the contents of this cache.
    ///
    /// [1]: ShaderResourceCache
    pub fn prepare_shader_resource_cache(
        &self,
        recorder: &mut CommandRecorder<'_, '_>,
        tmp_alloc: &ArenaGuard<'_, True>,
    ) -> Result<ShaderStageFlags>
    {
        let gpu = recorder.gpu().clone();
        if self.descriptor_set_binds.is_empty() &&
            self.push_descriptor_binding_calls.is_empty()
        {
            return Ok(ShaderStageFlags::empty())
        }
        let pools = gpu.get_descriptor_pools();
        let mut write_cache = FixedVec32
            ::with_len_with(pools.capacity(), |_| None, &tmp_alloc)
            .context("alloc failed")?;
        let mut all_shader_stages = ShaderStageFlags::empty();
        let current_frame = recorder.current_frame();
        let shader_resource_cache = &mut unsafe { &mut *recorder.cache().get() }.shader_resource_cache;
        for call in &self.descriptor_set_binds {
            let mut barrier_info = FixedVec32
                ::with_capacity(call.barriers.len(), &tmp_alloc)
                .context("alloc failed")?;
            for (i, &set_id) in call.sets.iter().enumerate() {
                let set = i as u32;
                barrier_info.clear();
                barrier_info.extend(call.barriers.iter().copied().filter(|barrier| barrier.set == set));
                let pool_id = set_id.pool_id();
                let pool = OptionExt::get_or_try_insert_with(
                    write_cache
                        .get_mut(pool_id.slot_index().index() as usize)
                        .ok_or_else(|| Error::just_context(format!("invalid pool id {pool_id}")))?,
                    || {
                        Ok(pools.get(pool_id.slot_index())
                            .context_with(|| format!(
                                "failed to find pool {pool_id}",
                            ))?.write())
                    })?;
                let mut descriptor_set = pool.get_descriptor_set_for_submit(
                    set_id,
                    current_frame,
                )?;
                let stage_flags = descriptor_set.stage_flags();
                let stage_mask = stage_flags.pipeline_stage_mask();
                all_shader_stages |= stage_flags;
                for binding in descriptor_set.binding_iter() {
                    let (ordering, access, explicit_access) = barrier_info
                        .iter()
                        .find_map(|bar| (bar.binding == binding.binding())
                            .then_some((
                                bar.barrier_info.ordering,
                                bar.barrier_info.access,
                                Some(bar.barrier_info.access)
                            ))
                        ).unwrap_or((CommandOrdering::Lenient, ExplicitAccess::SHADER_READ_AND_WRITE, None));
                    for buffer in binding.buffer_descriptors() {
                        if let Some((id, offset, size)) = buffer.buffer {
                            shader_resource_cache.touch_buffer(
                                id, offset, size,
                                stage_mask,
                                access,
                                ordering
                            );
                        }
                    }
                    for image in binding.image_descriptors() {
                        if let Some((id, layout)) = image.image {
                            let id = id.into_bare();
                            shader_resource_cache.touch_image(
                                id,
                                layout,
                                stage_mask,
                                explicit_access,
                            );
                        }
                        if let Some(sampler) = image.sampler.clone() {
                            recorder
                                .get_current_worker()
                                .add_sampler(sampler);
                        }
                    }
                }
            }
        }
        for call in &self.push_descriptor_binding_calls {
            let stage_mask = call.stage_flags.pipeline_stage_mask();
            let (ordering, access, explicit_access) = call.barrier
                .map(|barrier|
                    (barrier.ordering, barrier.access, Some(barrier.access))
                ).unwrap_or((CommandOrdering::Lenient, ExplicitAccess::SHADER_READ_AND_WRITE, None));
            for buffer in &call.buffers {
                shader_resource_cache.touch_buffer(
                    buffer.buffer_id,
                    buffer.offset, buffer.size,
                    stage_mask,
                    access,
                    ordering
                );
            }
            for (image, layout) in &call.images {
                if let Some(image_view) = image.image_view {
                    let id = image_view.into_bare();
                    shader_resource_cache.touch_image(
                        id,
                        layout.unwrap(),
                        stage_mask, explicit_access,
                    );
                }
                if let Some(sampler) = &image.sampler {
                    recorder
                        .get_current_worker()
                        .add_sampler(sampler.clone());
                }
            }
        }
        Ok(all_shader_stages)
    }
}

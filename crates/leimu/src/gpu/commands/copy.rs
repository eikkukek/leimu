use tuhka::vk;
use leimu_mem::{
    vec::{FixedVec32},
    alloc,
    arena,
};

use crate::{
    core::{slice, OptionExt},
    error::*,
    gpu::prelude::*,
};

/// A wrapper around a [`command buffer`][1], which allows transfer commands to be performed.
///
/// Available through [`CommandScheduler::new_copy_commands`] and
/// [`GraphicsCommands::copy_commands`].
///
/// In addition to copy commands, this also exposes [`clear_color_image`][3] and
/// [`clear_depth_stencil_image`][4].
///
/// # Vulkan docs
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_copy_commands2.html>
///
/// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBuffer.html
/// [2]: CommandScheduler::new_copy_commands
/// [3]: CopyCommands::clear_color_image
/// [4]: CopyCommands::clear_depth_stencil_image
pub struct CopyCommands<'a, 'b> {
    gpu: Gpu,
    recorder: CommandRecorder<'a, 'b>,
    queue: DeviceQueue,
    command_buffer: vk::CommandBuffer,
    command_id: CommandId,
    wait_scope: vk::PipelineStageFlags2,
    signal_scope: vk::PipelineStageFlags2,
}

pub struct NewCopyCommands;

impl NewCommands for NewCopyCommands {

    const NAME: &'static str = "copy commands";

    type Target<'a, 'b> = CopyCommands<'a, 'b>;

    fn new<'a, 'b>(
        recorder: CommandRecorder<'a, 'b>,
        command_id: CommandId,
        queue: DeviceQueue,
    ) -> Result<Self::Target<'a, 'b>>
        where Self::Target<'a, 'b>: Commands<'a, 'b>
    {
        CopyCommands::new(recorder, command_id, queue, None)
    }
}

unsafe impl<'a, 'b> Commands<'a, 'b> for CopyCommands<'a, 'b> {

    fn add_signal_semaphore(
        &mut self, 
        semaphore_id: TimelineSemaphoreId,
        value: u64,
    ) {
        self.recorder.add_signal_semaphore(self.command_id, semaphore_id, value);
    }

    fn add_wait_semaphore(
        &mut self,
        semaphore_id: TimelineSemaphoreId,
        value: u64,
        dependency_hint: MemoryDependencyHint,
    ) {
        self.recorder.add_wait_semaphore(
            self.command_id, semaphore_id,
            value, dependency_hint
        );
    }

    fn finish<'c, Alloc>(self, alloc: &'c Alloc) -> Result<CommandResult<'c, Alloc>>
        where Alloc: ?Sized + alloc::LocalAlloc<Error = arena::Error>
    {
        unsafe {
            self.gpu.device()
                .end_command_buffer(self.command_buffer)
                .context("failed to end command buffer")?;
        }
        let mut primary_command_buffers = FixedVec32
            ::with_capacity(1, alloc)
            .context("alloc failed")?;
        primary_command_buffers.push(self.command_buffer);
        Ok(CommandResult {
            primary_command_buffers,
            wait_scope: self.wait_scope,
            signal_scope: self.signal_scope,
            queue: self.queue
        })
    }
}

impl<'a, 'b> CopyCommands<'a, 'b> {

    pub(super) fn new(
        mut recorder: CommandRecorder<'a, 'b>,
        command_id: CommandId,
        queue: DeviceQueue,
        mut command_buffer: Option<vk::CommandBuffer>,
    ) -> Result<Self> {
        if !queue.queue_flags().contains(QueueFlags::GRAPHICS) {
            return Err(Error::just_context(format!(
                "queue {queue} doesn't support graphics operations"
            )))
        }
        let &mut command_buffer = OptionExt::get_or_try_insert_with(
            &mut command_buffer,
            || {
                let command_buffer = recorder
                    .get_current_worker()
                    .allocate_primaries(&queue, 1)?[0];
                let begin_info = vk::CommandBufferBeginInfo {
                    s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
                    flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
                    ..Default::default()
                };
                let gpu = recorder.gpu().clone();
                unsafe {
                    gpu.device()
                        .begin_command_buffer(command_buffer, &begin_info)
                        .context("failed to begin primary command buffer")?;
                };
                Ok(command_buffer)
            },
        )?;
        Ok(Self {
            gpu: recorder.gpu().clone(),
            recorder,
            queue,
            command_buffer,
            command_id,
            wait_scope: vk::PipelineStageFlags2::NONE,
            signal_scope: vk::PipelineStageFlags2::NONE,
        })
    }

    /// Updates a buffer's contents from host memory.
    ///
    /// # Valid usage
    /// - `dst_buffer_id` *must* be a valid [`BufferId`].
    /// - `dst_offset` *must* be a multiple of 4.
    /// - The size of `data` *must* be less than or equal to 65536 bytes and it *must* be a
    ///   multiple of 4.
    /// - `dst_offset` + the size of `data` *must* be less than or equal to the size of the buffer.
    /// - Destination buffer's usage *must* contain the [`transfer destination usage`][1].
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateBuffer.html>
    ///
    /// [1]: BufferUsages::TRANSFER_DST
    pub fn update_buffer<T: Copy>(
        &mut self,
        dst_buffer_id: BufferId,
        dst_offset: DeviceSize,
        data: &[T],
        ordering: CommandOrdering,
    ) -> Result<()> {
        self.wait_scope |= vk::PipelineStageFlags2::COPY;
        self.signal_scope |= vk::PipelineStageFlags2::COPY;
        let data = slice::as_bytes(data);
        let data_size = data.len() as DeviceSize;
        if !dst_offset.is_multiple_of(4) {
            return Err(Error::just_context(format!(
                "destination offset {dst_offset} is not a multiple of 4"
            )))
        }
        if data_size > 65536 {
            return Err(Error::just_context(format!(
                "data size {data_size} is more than 65535"
            )))
        }
        if !data_size.is_multiple_of(4) {
            return Err(Error::just_context(format!(
                "data size {data_size} is not a multiple of 4"
            )))
        }
        let cache = unsafe { &mut *self.recorder.cache().get() };
        self.recorder.write_resources(|guard| {
            let buffer = guard.register_buffer(dst_buffer_id)?;
            let buf_properties = buffer.properties();
            if !buf_properties.usage.contains(BufferUsages::TRANSFER_DST) {
                return Err(Error::just_context(
                    "destination buffer usage doesn't contain transfer destination usage"
                ))
            }
            if dst_offset + data_size > buf_properties.size {
                return Err(Error::just_context(format!(
                    "destination offset {dst_offset} + data size {data_size} is greater than buffer size {}",
                    buf_properties.size,
                )))
            }
            let queue_family_index = self.queue.family_index();
            let dst_state = BufferState::new(
                vk::PipelineStageFlags2::COPY,
                vk::AccessFlags2::TRANSFER_WRITE,
                queue_family_index,
            );
            let range = unsafe {
                buffer.memory_barrier_unchecked(
                    dst_offset, data_size,
                    dst_state,
                    ordering,
                    &mut cache.shader_resource_cache.buffer_memory_barrier_cache,
                )
            };
            let command_buffer = self.command_buffer;
            if !range.is_empty() {
                let tmp_alloc = self.gpu.tmp_alloc();
                let tmp_alloc = tmp_alloc.guard();
                let memory_barriers = cache.shader_resource_cache.buffer_memory_barrier_cache.flush(
                    &[range], &tmp_alloc
                )?;
                let dependency_info = vk::DependencyInfo {
                    buffer_memory_barrier_count: memory_barriers.len(),
                    p_buffer_memory_barriers: memory_barriers.as_ptr(),
                    ..Default::default()
                };
                unsafe {
                    self.gpu.device()
                    .cmd_pipeline_barrier2(command_buffer, &dependency_info);
                }
            }
            unsafe {
                self.gpu.device()
                .cmd_update_buffer(
                    command_buffer,
                    buffer.handle(), dst_offset,
                    data,
                );
            }
            Ok(())
        })
    }

    /// Fill a region of a buffer with a [`u32`] value.
    ///
    /// If `size` is [`None`], it means that the remaining region of the buffer from `dst_offset`
    /// is updated, rounded down to the nearest multiple of 4.
    ///
    /// # Valid usage
    /// - `dst_buffer_id` *must* be a valid [`BufferId`].
    /// - Destination buffer's usage *must* contain the [`transfer destination usage`][1].
    /// - `dst_offset` *must* be less than the buffer's size.
    /// - `dst_offset` *must* be a multiple of 4.
    /// - If `size` is [`Some`], `dst_offset` + `size` *must* be less than or equal to the size
    ///   of the buffer, and it *must* be a multiple of 4.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateBuffer.html>
    ///
    /// [1]: BufferUsages::TRANSFER_DST
    pub fn fill_buffer(
        &mut self,
        dst_buffer_id: BufferId,
        dst_offset: DeviceSize,
        size: Option<NonZeroDeviceSize>,
        data: u32,
        ordering: CommandOrdering,
    ) -> Result<()> {
        self.wait_scope |= vk::PipelineStageFlags2::COPY;
        self.signal_scope |= vk::PipelineStageFlags2::COPY;
        if !dst_offset.is_multiple_of(4) {
            return Err(Error::just_context(format!(
                "destination offset {dst_offset} is not a multiple of 4"
            )))
        }
        let cache = unsafe { &mut *self.recorder.cache().get() };
        self.recorder.write_resources(|guard| {
            let buffer = guard.register_buffer(dst_buffer_id)?;
            let buf_properties = buffer.properties();
            if !buf_properties.usage.contains(BufferUsages::TRANSFER_DST) {
                return Err(Error::just_context(
                    "destination buffer usage doesn't contain transfer destination usage"
                ))
            }
            let (size, vk_size) = {
                if let Some(size) = size {
                    let size = size.get();
                    if !size.is_multiple_of(4) {
                        return Err(Error::just_context(format!(
                            "size {size} is not a multiple of 4"
                        )))
                    }
                    if size == 0 {
                        return Err(Error::just_context(
                            "size is zero"
                        ))
                    }
                    if dst_offset + size > buf_properties.size {
                        return Err(Error::just_context(format!(
                            "destination offset {dst_offset} + size {size} is greater than buffer size {}",
                            buf_properties.size,
                        )))
                    }
                    (size, size)
                } else {
                    if dst_offset >= buf_properties.size {
                        return Err(Error::just_context(format!(
                            "destination offset {dst_offset} is greater than or equal to buffer size {}",
                            buf_properties.size,
                        )))
                    }
                    (buf_properties.size - dst_offset, vk::WHOLE_SIZE)
                }
            };
            let queue_family_index = self.queue.family_index();
            let state = BufferState::new(
               vk::PipelineStageFlags2::COPY,
               vk::AccessFlags2::TRANSFER_WRITE,
               queue_family_index
            );
            let range = unsafe {
                buffer.memory_barrier_unchecked(
                    dst_offset, size,
                    state, ordering,
                    &mut cache.shader_resource_cache.buffer_memory_barrier_cache,
                )
            };
            let command_buffer = self.command_buffer;
            if !range.is_empty() {
                let tmp_alloc = self.gpu.tmp_alloc();
                let tmp_alloc = tmp_alloc.guard();
                let memory_barriers = cache.shader_resource_cache.buffer_memory_barrier_cache.flush(
                    &[range], &tmp_alloc,
                )?;
                let dependency_info = vk::DependencyInfo {
                    buffer_memory_barrier_count: memory_barriers.len(),
                    p_buffer_memory_barriers: memory_barriers.as_ptr(),
                    ..Default::default()
                };
                unsafe {
                    self.gpu.device()
                    .cmd_pipeline_barrier2(command_buffer, &dependency_info);
                }
            }
            unsafe {
                self.gpu.device()
                .cmd_fill_buffer(
                    command_buffer,
                    buffer.handle(),
                    dst_offset, vk_size,
                    data,
                );
            }
            Ok(())
        }) 
    }

    /// Copies regions of a buffer to regions of another buffer.
    ///
    /// # Valid usage
    /// - `src_buffer_id` and `dst_buffer_id` *must* be valid [`BufferId`]s.
    /// - Source buffer *must* have been created with the [`BufferUsages::TRANSFER_SRC`] bit
    ///   set.
    /// - Destination buffer *must* have been created with the [`BufferUsages::TRANSFER_DST`]
    ///   bit set.
    /// - [`BufferCopy::src_offset`] + size and [`BufferCopy::dst_offset`] + size *must* be in the
    ///   range of the source and destination buffers respectively in each [`BufferCopy`] in
    ///   `regions`.
    /// - The regions specified *must* not overlap in device memory. That is, if both of the
    ///   buffers are bound by the same [`vk::DeviceMemory`] object, the offsets and sizes in
    ///   `regions` *must* not cause overlap within that memory.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html>
    #[inline(always)]
    pub fn copy_buffer(
        &mut self,
        src_buffer_id: BufferId, 
        dst_buffer_id: BufferId,
        regions: &[BufferCopy],
        ordering: CommandOrdering,
    ) -> Result<()>
    {
        self.wait_scope |= vk::PipelineStageFlags2::COPY;
        self.signal_scope |= vk::PipelineStageFlags2::COPY;
        let tmp_alloc = self.gpu.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let cache = unsafe { &mut *self.recorder.cache().get() };
        self.recorder.write_resources(|guard| {
            let src_buffer: *mut BufferMeta = guard.register_buffer(
                src_buffer_id
            )?;
            let src_buffer = unsafe { &mut *src_buffer };
            let src_properties = src_buffer.properties();
            let dst_buffer: *mut BufferMeta = guard.register_buffer(
                dst_buffer_id
            )?;
            let dst_buffer = unsafe { &mut *dst_buffer };
            let dst_properties = dst_buffer.properties();
            let src_mem = src_buffer.memory();
            let dst_mem = dst_buffer.memory();
            let overlap = src_mem.overlaps(dst_mem).then(||
                (src_mem.offset(), dst_mem.offset())
            );
            if !src_properties.usage.contains(BufferUsages::TRANSFER_SRC) {
                return Err(Error::just_context(
                    "source buffer usage doesn't contain transfer source usage"
                ))
            }
            if !dst_properties.usage.contains(BufferUsages::TRANSFER_DST) {
                return Err(Error::just_context(
                    "destination buffer usage doesn't transfer destination usage"
                ))
            }
            let queue_family_index = self.queue.family_index();
            let src_state = BufferState::new(
                vk::PipelineStageFlags2::COPY,
                vk::AccessFlags2::TRANSFER_READ,
                queue_family_index,
            );
            let dst_state = BufferState::new(
                vk::PipelineStageFlags2::COPY,
                vk::AccessFlags2::TRANSFER_WRITE,
                queue_family_index,
            );
            let command_buffer = self.command_buffer;
            let mut vk_regions = FixedVec32::<vk::BufferCopy2, _>
                ::with_capacity(regions.len() as u32, &tmp_alloc)
                .context("alloc failed")?;
            let mut memory_barrier_ranges = FixedVec32
                ::with_capacity(regions.len() as u32 * 2, &tmp_alloc)
                .context("alloc failed")?;
            for &region in regions {
                if region.src_offset + region.size > src_properties.size {
                    return Err(Error::just_context(format!(
                        "region source offset {} + size {} was out of the range of the source buffer size {}",
                        region.src_offset, region.size, src_properties.size
                    )))
                }
                if region.dst_offset + region.size > dst_properties.size {
                    return Err(Error::just_context(format!(
                        "region destination offset {} + size {} was out of the range of the destination buffer size {}",
                        region.dst_offset, region.size, dst_properties.size
                    )))
                }
                if let Some((src_mem_off, dst_mem_off)) = overlap {
                    let dst_offset = dst_mem_off + region.dst_offset;
                    let dst_end = dst_offset + region.size;
                    for &region in regions {
                        let src_offset = src_mem_off + region.src_offset;
                        if src_offset < dst_end &&
                            dst_offset < src_offset + region.size
                        {
                            return Err(Error::just_context(format!(
                                "{}{}",
                                format_args!("buffer regions overlap in memory with source memory offset {}, ",
                                    region.size
                                ),
                                format_args!("destination memory offset {} and size {}", src_offset, dst_offset),
                            )))
                        }
                    } 
                }
                let ranges1 = unsafe { src_buffer.memory_barrier_unchecked(
                    region.src_offset, region.size,
                    src_state,
                    ordering,
                    &mut cache.shader_resource_cache.buffer_memory_barrier_cache,
                ) };
                let ranges2 = unsafe { dst_buffer.memory_barrier_unchecked(
                    region.dst_offset, region.size,
                    dst_state,
                    ordering,
                    &mut cache.shader_resource_cache.buffer_memory_barrier_cache,
                ) };
                memory_barrier_ranges.fast_append(&[ranges1, ranges2]);
                vk_regions.push(region.into());
            }
            let buffer_memory_barriers = cache.shader_resource_cache.buffer_memory_barrier_cache.flush(
                &memory_barrier_ranges,
                &tmp_alloc
            )?;
            if !buffer_memory_barriers.is_empty() {
                let dependency_info = vk::DependencyInfo {
                    buffer_memory_barrier_count: buffer_memory_barriers.len(),
                    p_buffer_memory_barriers: buffer_memory_barriers.as_ptr(),
                    ..Default::default()
                };
                unsafe {
                    self.gpu.device()
                    .cmd_pipeline_barrier2(command_buffer, &dependency_info);
                }
            }
            let info = vk::CopyBufferInfo2 {
                src_buffer: src_buffer.handle(),
                dst_buffer: dst_buffer.handle(),
                region_count: vk_regions.len(),
                p_regions: vk_regions.as_ptr(),
                ..Default::default()
            };
            unsafe {
                self.gpu.device().cmd_copy_buffer2(
                    command_buffer,
                    &info,
                );
            }
            Ok(())
        }) 
    }

    /// Copies regions of an image to regions of another image.
    ///
    /// # Valid usage
    /// - `src_image_id` *must* be a valid [`ImageId`].
    /// - `dst_image_id` *must* be a valid [`ImageId`].
    /// - Source image [`format features`][1] *must* support transfer source operations.
    /// - Destination image [`format features][1] *must* support transfer destintion operations.
    /// - Source image *must* have been created with the [`transfer source usage`][2].
    /// - Destination image *must* have been created with the [`transfer destination usage][3]
    /// - If neither source or destination image has a multi-planar format, they *must* be
    ///   [`size compatible`][4].
    /// - If both source and destination images are [`compressed`][5] image formats, they *must*
    ///   have the same [`texel block extent`][6].
    /// - All [`extents`][9] in `regions` *must* be non-zero.
    /// - All source and destination [`subresource layers`][7] in `regions` *must* be compatible
    ///   with source image and destination image respectively.
    /// - For each image with a multi-planar format, each region's aspect mask *must* only contain
    ///   exactly one of either [`plane 0`][15], [`plane 1`][16] or [`plane 2`][17] aspect for that image's
    ///   [`subresource`][7].
    /// - If either source or destination image has a multi-planar format, their formats *must*
    ///   be [`compatible`][8] at each source and destination aspect in `regions`.
    /// - If neither source or destination image has a multi-planar format, each
    ///   [`source aspect and destination aspect`][10] in `regions` *must* be the same.
    /// - For each [`source offset`][11] + [`extent`][9] in `regions` *must* not be greater than
    ///   the source images [`Dimensions`] at that [mip level`][12].
    /// - For each [`destination offset`][13] + [`extent`][9] in `regions` *must* not be greater
    ///   than destination image's [`Dimensions`] at that [`mip level`][12] when
    ///   [`adjusted for size compatibility`][14]
    /// - Each [`source offset`][11] in `regions` *must* be a multiple of the source image's
    ///   [`texel block extent`][6] for each dimension.
    /// - Each [`destination offset`][13] in `regions` *must* be a multiple of the destination 
    ///   image's [`texel block extent`][6] for each dimension.
    /// - For each region in `regions`, if [`source offset`][11] + [`extent`][9] is not equal to
    ///   the source image's [`Dimensions`], [`extent`][9] *must* be a multiple of the source image's
    ///   [`texel block extent`][6] for each dimension.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html>
    ///
    /// [1]: FormatFeatures
    /// [2]: ImageUsages::TRANSFER_SRC
    /// [3]: ImageUsages::TRANSFER_DST
    /// [4]: FormatCompatibilityClass::is_size_compatible
    /// [5]: FormatCompatibilityClass::is_compressed
    /// [6]: FormatCompatibilityClass::texel_block_extent
    /// [7]: ImageSubresourceLayers
    /// [8]: https://docs.vulkan.org/spec/latest/chapters/formats.html#formats-compatible-planes
    /// [9]: ImageCopy::extent
    /// [10]: ImageAspects
    /// [11]: ImageCopy::src_offset
    /// [12]: ImageSubresourceLayers::mip_level
    /// [13]: ImageCopy::dst_offset
    /// [14]: https://docs.vulkan.org/spec/latest/chapters/formats.html#formats-size-compatibility
    /// [15]: ImageAspects::PLANE_0
    /// [16]: ImageAspects::PLANE_1
    /// [17]: ImageAspects::PLANE_2
    pub fn copy_image(
        &mut self,
        src_image_id: ImageId,
        dst_image_id: ImageId,
        regions: &[ImageCopy],
    ) -> Result<()> {
        self.wait_scope |= vk::PipelineStageFlags2::COPY;
        self.signal_scope |= vk::PipelineStageFlags2::COPY;
        let tmp_alloc = self.gpu.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let cache = unsafe { &mut *self.recorder.cache().get() };
        let command_id = self.command_id;
        self.recorder.write_resources(|guard| {
            let src_image: *mut ImageMeta = guard.register_image(
               src_image_id.slot_index(), command_id.index()
            )?;
            let src_image = unsafe { &mut *src_image };
            let dst_image: *mut ImageMeta = guard.register_image(
                dst_image_id.slot_index(), command_id.index()
            )?;
            let src_properties = src_image.properties();
            if !src_properties.format_features.contains(FormatFeatures::TRANSFER_SRC) {
                return Err(Error::just_context(
                    "source image format doesn't support transfer source operations"
                ))
            }
            let src_format_class = src_properties.format.compatibility();
            let src_planes = src_properties.format.plane_formats();
            let src_multi_planar = src_properties.format.plane_count() > 1;
            let src_texel_block_extent: Dimensions = src_format_class.texel_block_extent();
            if let Some(err) = src_image.validate_usage(ImageUsages::TRANSFER_SRC) {
                return Err(Error::new(err, "source image usage mismatch"))
            }
            let dst_image = unsafe { &mut *dst_image };
            let dst_properties = dst_image.properties();
            if !dst_properties.format_features.contains(FormatFeatures::TRANSFER_DST) {
                return Err(Error::just_context(
                    "destination image format doesn't support transfer destination operations"
                ))
            }
            let dst_format_class = dst_properties.format.compatibility();
            let dst_planes = src_properties.format.plane_formats();
            let dst_multi_planar = src_properties.format.plane_count() > 1;
            let dst_texel_block_extent: Dimensions = dst_format_class.texel_block_extent();
            if let Some(err) = dst_image.validate_usage(ImageUsages::TRANSFER_DST) {
                return Err(Error::new(err, "destination image usage mismatch"))
            }
            if !src_multi_planar && !dst_multi_planar &&
                !src_format_class.is_size_compatible(dst_format_class)
            {
                return Err(Error::just_context(format!(
                    "source format {} is not size compatible with destination format {}",
                    src_properties.format, dst_properties.format,
                )))
            }
            if src_texel_block_extent != dst_texel_block_extent &&
                src_properties.format.is_compressed() && dst_properties.format.is_compressed()
            {
                return Err(Error::just_context(format!(
                    "source image format {} doesn't have the same texel block extent as desctination format {}",
                    src_properties.format, dst_properties.format,
                )))
            }
            let queue_family_index = self.queue.family_index();
            let mut src_state = ImageSubresourceState::new(
                vk::PipelineStageFlags2::COPY,
                vk::AccessFlags2::TRANSFER_READ,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                queue_family_index,
            );
            let mut dst_state = ImageSubresourceState::new(
                vk::PipelineStageFlags2::COPY,
                vk::AccessFlags2::TRANSFER_WRITE,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                queue_family_index
            );
            if src_image_id == dst_image_id &&
                regions[0..regions.len() - 1]
                    .iter()
                    .enumerate()
                    .any(|(i, r)| {
                        let src = r.src_subresource;
                        regions[i + 1..]
                            .iter()
                            .any(|r| src.overlaps(r.dst_subresource))
                    })
            {
                src_state.access_mask |= vk::AccessFlags2::TRANSFER_WRITE;
                src_state.layout = vk::ImageLayout::GENERAL;
                dst_state = src_state;
            }
            let n_regions = regions.len() as u32;
            let mut vk_regions = FixedVec32
                ::with_capacity(n_regions, &tmp_alloc)
                .context("alloc failed")?;
            let mut memory_barrier_ranges = FixedVec32
                ::with_capacity(n_regions * 2, &tmp_alloc)
                .context("alloc failed")?;
            let adj_extent = {
                let a = src_texel_block_extent;
                let b = dst_texel_block_extent;
                Dimensions::new(
                    a.width * b.width,
                    a.height * b.height,
                    a.depth * b.depth
                )
            };
            for &region in regions {
                if !src_multi_planar && !dst_multi_planar {
                    if region.src_subresource.aspect_mask != region.dst_subresource.aspect_mask {
                        return Err(Error::just_context(format!(
                            "{}{}",
                            format_args!("source aspect mask {} and destination aspect mask {} aren't the same when ",
                                region.src_subresource.aspect_mask, region.dst_subresource.aspect_mask,
                            ),
                            "neither source or destination image has multi-planar format"
                        )))
                    }
                } else if src_multi_planar {
                    let Some(src_plane) = region.src_subresource.aspect_mask.plane() else {
                        return Err(Error::just_context(format!(
                            "expected a single plane aspect source subresource, found {}",
                            region.src_subresource.aspect_mask,
                        )))
                    };
                    if dst_multi_planar {
                        let Some(dst_plane) = region.dst_subresource.aspect_mask.plane() else {
                            return Err(Error::just_context(format!(
                                "expected a single plane aspect for destination subresource, found {}",
                                region.dst_subresource.aspect_mask,
                            )))
                        };
                        if src_planes[src_plane as usize] != dst_planes[dst_plane as usize] {
                            return Err(Error::just_context(format!(
                                "{}{}",
                                format_args!("source format {} is not compatible with destination format {} ",
                                    src_properties.format, dst_properties.format,
                                ),
                                format_args!("with source aspect mask {} and destination aspect mask {}",
                                    region.src_subresource.aspect_mask, region.dst_subresource.aspect_mask,
                                ),
                            )))
                        }
                    } else if src_planes[src_plane as usize] != dst_properties.format {
                        return Err(Error::just_context(format!(
                            "{}{}",
                            format_args!("source format {} is not compatible with destination format {} ",
                                src_properties.format, dst_properties.format,
                            ),
                            format_args!("with source aspect mask {} and destination aspect mask {}",
                                region.src_subresource.aspect_mask, region.dst_subresource.aspect_mask,
                            ),
                        )))
                    }
                } else {
                    let Some(dst_plane) = region.dst_subresource.aspect_mask.plane() else {
                        return Err(Error::just_context(format!(
                            "expected a single plane aspect destination subresource, found {}",
                            region.src_subresource.aspect_mask,
                        )))
                    };
                    if src_properties.format != dst_planes[dst_plane as usize] {
                        return Err(Error::just_context(format!(
                            "{}{}",
                            format_args!("source format {} is not compatible with destination format {} ",
                                src_properties.format, dst_properties.format,
                            ),
                            format_args!("with source aspect mask {} and destination aspect mask {}",
                                region.src_subresource.aspect_mask, region.dst_subresource.aspect_mask,
                            ),
                        )))
                    }
                }
                if region.extent.is_zero() {
                    return Err(Error::just_context(
                        "region extent was zero"
                    ))
                }
                let src_dim = src_properties.dimensions.lod(region.src_subresource.mip_level);
                let src_end = region.src_offset + region.extent;
                if src_end.x > src_dim.width ||
                    src_end.y > src_dim.height ||
                    src_end.z > src_dim.depth
                {
                    return Err(Error::just_context(format!(
                        "region source offset {} + extent {} is out range of image dimensions {}",
                        region.src_offset, region.extent, src_dim,
                    )))
                }
                let adjusted = Dimensions::new(
                    region.extent.width / adj_extent.width,
                    region.extent.height / adj_extent.height,
                    region.extent.depth / adj_extent.depth,
                );
                let dst_dim = dst_properties.dimensions.lod(region.dst_subresource.mip_level);
                if !region.dst_offset.is_in_range(dst_dim, adjusted) {
                    return Err(Error::just_context(format!(
                        "regions destination offset {} + adjusted extent {} is out of range of image dimensions {}",
                        region.dst_offset, adjusted, dst_dim,
                    )))
                }
                if !region.src_offset.is_multiple_of(src_texel_block_extent) {
                    return Err(Error::just_context(format!(
                        "source image offset {} is not a multiple of source format texel block extent {}",
                        region.src_offset, src_texel_block_extent,
                    )))
                }
                if !region.dst_offset.is_multiple_of(dst_texel_block_extent) {
                    return Err(Error::just_context(format!(
                        "destination image offset {} is not a multiple of destination format texel block extent {}",
                        region.dst_offset, dst_texel_block_extent,
                    )))
                }
                if (src_end.x != src_dim.width ||
                    src_end.y != src_dim.height ||
                    src_end.z != src_dim.depth) &&
                    !region.extent.is_multiple_of(src_texel_block_extent)
                {
                    return Err(Error::just_context(format!(
                        "extent {} is not a multiple of source image texel block extent {}",
                        region.extent, src_texel_block_extent,
                    )))
                }
                let ranges1 = src_image.memory_barrier(
                    src_state,
                    region.src_subresource.into_range(),
                    true,
                    &mut cache.shader_resource_cache.image_memory_barrier_cache,
                ).context("source image memory barrier failed")?;
                let ranges2 = dst_image.memory_barrier(
                    dst_state,
                    region.dst_subresource.into_range(),
                    true,
                    &mut cache.shader_resource_cache.image_memory_barrier_cache
                ).context("destination image memory barrier failed")?;
                memory_barrier_ranges.fast_append(&[ranges1, ranges2]);
                vk_regions.push(region.into());
            }
            let mem_barriers = cache.shader_resource_cache.image_memory_barrier_cache.flush(
                &memory_barrier_ranges,
                &tmp_alloc
            )?;
            let dependency_info = vk::DependencyInfo {
                image_memory_barrier_count: mem_barriers.len(),
                p_image_memory_barriers: mem_barriers.as_ptr(),
                ..Default::default()
            };
            let command_buffer = self.command_buffer;
            unsafe {
                self.gpu.device().cmd_pipeline_barrier2(command_buffer, &dependency_info);
            }
            let info = vk::CopyImageInfo2 {
                src_image: src_image.handle(),
                dst_image: dst_image.handle(),
                src_image_layout: src_state.layout,
                dst_image_layout: dst_state.layout,
                region_count: vk_regions.len(),
                p_regions: vk_regions.as_ptr(),
                ..Default::default()
            };
            unsafe {
                self.gpu.device().cmd_copy_image2(command_buffer, &info);
            }
            Ok(())
        })
    }

    /// Copies regions of a buffer to regions of an image.
    ///
    /// # Valid usage
    /// - 'src_buffer_id' *must* be a valid [`BufferId`].
    /// - 'dst_image_id' *must* be a valid [`ImageId`].
    /// - Source buffer *must* have been created with the [`BufferUsages::TRANSFER_SRC`] bit
    ///   set.
    /// - Destination image *must* have been created with the [`ImageUsages::TRANSFER_DST`] bit
    ///   set.
    /// - The format features of the destination image *must* contain [`FormatFeatures::TRANSFER_DST`]
    ///   bit.
    /// - For each region, [`buffer offset`][1] + [`calculated buffer size`][2] *must* be within
    ///   the range of the buffer.
    /// - For each region, [`image offset`][3] + [`image extent`][4] *must* be within the range of the
    ///   destination image's [`Dimensions`] and [`image subresource`][5] *must* be a valid
    ///   [`image subresource layers structure`][6] for that image.
    /// - For each region, [`image extent`][4] *must* not be zero.
    /// - For each region, [`buffer row length`][7] *must* be either [`None`] or greater than or
    ///   equal to [`image extent`][8] width and a multiple of [`texel block extent`][9] width of the
    ///   [`Format`] of the image.
    /// - For each region, [`buffer image height`][10] *must* be either [`None`] or greater than or
    ///   equal to [`image extent`][8] height and a multiple of [`texel block extent`][9] height of the
    ///   [`Format`] of the image.
    /// - If the destination image has a multi-planar format, the [`aspect mask`][11] of each region
    ///   *must* contain only exactly one of either [`plane 0`][12], [`plane 1`][13] or [`plane 2`][14]
    ///   aspect.
    /// - If the destination image doesn't have a depth/stencil or multi-planar format, each
    ///   region's [`buffer offset`][1] *must* be a multiple of the image format's
    ///   [`texel block size`][15].
    /// - If the destination image has a depth/stencil format, each region's [`buffer offset`][1]
    ///   *must* be a multiple of 4.
    /// - If the destination image has a multi-planar format, each region's [`buffer offset`][1]
    ///   *must* be a multiple of the given [`plane`][16] [`aspect's format`][17] [`texel block size`][15].
    /// - The regions specified *must* not overlap in device memory. That is, if both of the
    ///   objects are bound by the same [`vk::DeviceMemory`] object, the offsets and sizes/extents
    ///   in `regions` *must* not cause overlap within that memory.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html>
    ///
    /// [1]: BufferImageCopy::buffer_offset
    /// [2]: BufferImageCopy::calculate_buffer_size
    /// [3]: BufferImageCopy::image_offset
    /// [4]: BufferImageCopy::image_extent
    /// [5]: BufferImageCopy::image_subresource
    /// [6]: ImageSubresourceLayers
    /// [7]: BufferImageCopy::buffer_row_length
    /// [8]: BufferImageCopy::image_extent
    /// [9]: FormatCompatibilityClass::texel_block_extent
    /// [10]: BufferImageCopy::buffer_image_height
    /// [11]: ImageAspects
    /// [12]: ImageAspects::PLANE_0
    /// [13]: ImageAspects::PLANE_1
    /// [14]: ImageAspects::PLANE_2
    /// [15]: FormatCompatibilityClass::texel_block_size
    /// [16]: Format::plane_formats
    /// [17]: ImageAspects::plane
    #[inline(always)]
    pub fn copy_buffer_to_image(
        &mut self,
        src_buffer_id: BufferId,
        dst_image_id: ImageId,
        regions: &[BufferImageCopy],
        ordering: CommandOrdering,
    ) -> Result<()>
    {
        self.wait_scope |= vk::PipelineStageFlags2::COPY;
        self.signal_scope |= vk::PipelineStageFlags2::COPY;
        let cache = unsafe { &mut *self.recorder.cache().get() };
        self.recorder.write_resources(|guard| {
            let buffer: *mut BufferMeta = guard.register_buffer(src_buffer_id)?;
            let buffer = unsafe { &mut *buffer };
            let buf_properties = buffer.properties();
            if !buf_properties.usage.contains(BufferUsages::TRANSFER_SRC) {
                return Err(Error::just_context(
                    "source buffer usage doesn't contain transfer source usage"
                ))
            }
            let image: *mut ImageMeta = guard.register_image(
                dst_image_id.slot_index(),
                self.command_id.index()
            )?;
            let image = unsafe { &mut *image };
            let img_properties = image.properties();
            if !img_properties.usage.contains(ImageUsages::TRANSFER_DST) {
                return Err(Error::just_context(
                    "destination image usage doesn't contain transfer destination usage"
                ))
            }
            if !img_properties.format_features.contains(FormatFeatures::TRANSFER_DST) {
                return Err(Error::just_context(format!(
                    "image format {} doesn't support transfer destination operations",
                    img_properties.format,
                )))
            }
            let queue_family_index = self.queue.family_index();
            let src_state = BufferState::new(
                vk::PipelineStageFlags2::COPY,
                vk::AccessFlags2::TRANSFER_READ,
                queue_family_index
            );
            let dst_state = ImageSubresourceState::new(
                vk::PipelineStageFlags2::COPY,
                vk::AccessFlags2::TRANSFER_WRITE,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                queue_family_index
            );
            let command_buffer = self.command_buffer;
            let tmp_alloc = self.gpu.tmp_alloc();
            let tmp_alloc = tmp_alloc.guard();
            let format_class = img_properties.format.compatibility();
            let texel_block_size = format_class.texel_block_size();
            let texel_block_extent: Dimensions = format_class.texel_block_extent();
            let planes = img_properties.format
                .plane_formats()
                .map(|format| format.compatibility().texel_block_size());
            let is_multiplanar = img_properties.format.plane_count() > 1;
            let is_depth_stencil = img_properties.format.is_depth_stencil();
            let n_regions = regions.len() as u32;
            let mut buffer_memory_barrier_ranges = FixedVec32::with_capacity(
                n_regions, &tmp_alloc
            ).context("alloc failed")?;
            let mut image_memory_barrier_ranges = FixedVec32::with_capacity(
                n_regions, &tmp_alloc
            ).context("alloc failed")?;
            let mut vk_regions = FixedVec32
                ::with_capacity(n_regions, &tmp_alloc)
                .context("alloc failed")?;
            for &region in regions {
                if let Some(row_length) = region.buffer_row_length 
                {
                    if !row_length.get().is_multiple_of(texel_block_extent.width) {
                        return Err(Error::just_context(format!(
                            "source buffer row length {row_length} is not a multiple of texel block extent width {} of format {}",
                            texel_block_extent.width, img_properties.format,
                        )))
                    } 
                    if row_length.get() < region.image_extent.width {
                        return Err(Error::just_context(format!(
                            "source image extent width {} was out of range of destination buffer row length {row_length}",
                            region.image_extent.width,
                        )))
                    }
                }
                if let Some(image_height) = region.buffer_image_height
                {
                    if !image_height.get().is_multiple_of(texel_block_extent.height) {
                        return Err(Error::just_context(format!(
                            "source buffer image height {image_height} is not a multiple of texel block extent height {} of format {}",
                            texel_block_extent.height, texel_block_extent.height,
                        )))
                    }
                    if image_height.get() < region.image_extent.height {
                        return Err(Error::just_context(format!(
                            "source image extent height {} was out of range of destination buffer image height {image_height}",
                            region.image_extent.height,
                        )))
                    }
                }
                if region.image_extent.is_zero() {
                    return Err(Error::just_context(format!(
                        "region image extent {} was zero",
                        region.image_extent,
                    )))
                }
                let layer_count = region.image_subresource
                    .effective(img_properties.array_layers)
                    .layer_count;
                let buffer_size = region.calculate_buffer_size(
                    format_class,
                    img_properties.format,
                    region.image_subresource.aspect_mask,
                    layer_count,
                );
                if region.buffer_offset + buffer_size > buf_properties.size {
                    return Err(Error::just_context(format!(
                        "source offset {} + calculated size {} was out of range of the source buffer size {}",
                        region.buffer_offset, buffer_size, buf_properties.size,
                    )))
                }
                let dim = img_properties.dimensions.lod(region.image_subresource.mip_level);
                if !region.image_offset.is_multiple_of(texel_block_extent) {
                    return Err(Error::just_context(format!(
                        "region destination image offset {} is not a multiple of format {} texel block extent {}",
                        region.image_offset, img_properties.format, format_class.texel_block_extent::<Dimensions>(),
                    )))
                }
                if !region.image_offset.is_in_range(
                    dim,
                    region.image_extent,
                ) {
                    return Err(Error::just_context(format!(
                        "region destination offset {} + extent {} was out range of the image's dimensions {}",
                        region.image_offset, region.image_extent, img_properties.dimensions,
                    )))
                }
                if is_multiplanar {
                    let Some(plane) = region.image_subresource.aspect_mask.plane() else {
                        return Err(Error::just_context(
                            "region aspect mask doesn't contain a single plane"
                        ))
                    };
                    if !region.buffer_offset.is_multiple_of(planes[plane as usize]) {
                        return Err(Error::just_context(format!(
                            "{}{}",
                            format_args!("region buffer offset is not a multiple of the image's texel block size {} at ",
                                planes[plane as usize]
                            ),
                            format_args!("aspect {}", region.image_subresource.aspect_mask),
                        )))
                    }
                } else if is_depth_stencil {
                    if !region.buffer_offset.is_multiple_of(4) {
                        return Err(Error::just_context(format!(
                            "region buffer offset {} is not a multiple of 4 when the image is a depth/stencil image",
                            region.buffer_offset,
                        )))
                    }
                } else if !region.buffer_offset.is_multiple_of(texel_block_size) {
                    return Err(Error::just_context(format!(
                        "region buffer offset {} is not a multiple of the image's texel block size {}",
                        region.buffer_offset, texel_block_size,
                    )))
                }
                let ranges1 = unsafe { buffer.memory_barrier_unchecked(
                    region.buffer_offset, buffer_size,
                    src_state,
                    ordering,
                    &mut cache.shader_resource_cache.buffer_memory_barrier_cache,
                ) };
                let ranges2 = image.memory_barrier(
                    dst_state,
                    region.image_subresource.into_range(),
                    true,
                    &mut cache.shader_resource_cache.image_memory_barrier_cache,
                ).context("image memory barrier failed")?;
                buffer_memory_barrier_ranges.push(ranges1);
                image_memory_barrier_ranges.push(ranges2);
                vk_regions.push(region.into());
            }
            let buffer_memory_barriers = cache.shader_resource_cache.buffer_memory_barrier_cache.flush(
                &buffer_memory_barrier_ranges, &tmp_alloc
            )?;
            let image_memory_barriers = cache.shader_resource_cache.image_memory_barrier_cache.flush(
                &image_memory_barrier_ranges, &tmp_alloc
            )?;
            let dependency_info = vk::DependencyInfo {
                buffer_memory_barrier_count: buffer_memory_barriers.len(),
                p_buffer_memory_barriers: buffer_memory_barriers.as_ptr(),
                image_memory_barrier_count: image_memory_barriers.len(),
                p_image_memory_barriers: image_memory_barriers.as_ptr(),
                ..Default::default()
            };
            unsafe {
                self.gpu.device()
                .cmd_pipeline_barrier2(command_buffer, &dependency_info);
            }
            let info = vk::CopyBufferToImageInfo2 {
                src_buffer: buffer.handle(),
                dst_image: image.handle(),
                dst_image_layout: dst_state.layout,
                region_count: vk_regions.len(),
                p_regions: vk_regions.as_ptr(),
                ..Default::default()
            };
            unsafe {
                self.gpu.device().cmd_copy_buffer_to_image2(
                    command_buffer, &info
                );
            }
            Ok(())
        }) 
    }

    /// Copies regions of an image to regions of a buffer.
    ///
    /// # Valid usage
    /// - 'src_image_id' *must* be a valid [`ImageId`].
    /// - 'dst_buffer_id' *must* be a valid [`BufferId`].
    /// - Source image *must* have been created with the [`ImageUsages::TRANSFER_SRC`] bit
    ///   set.
    /// - Destination buffer *must* have been created with the [`BufferUsages::TRANSFER_DST`] bit
    ///   set.
    /// - The format features of the source image *must* contain [`FormatFeatures::TRANSFER_SRC`]
    ///   bit.
    /// - For each region, [`buffer offset`][1] + [`calculated buffer size`][2] *must* be within
    ///   the range of the buffer.
    /// - For each region, [`image offset`][3] + [`image extent`][4] *must* be within the range of the
    ///   source image's [`Dimensions`] and [`image subresource`][5] *must* be a valid
    ///   [`image subresource layers structure`][6] for that image.
    /// - For each region, [`image extent`][4] *must* not be zero.
    /// - For each region, [`buffer row length`][7] *must* be either [`None`] or greater than or
    ///   equal to [`image extent`][8] width and a multiple of [`texel block extent`][9] width of the
    ///   [`Format`] of the image.
    /// - For each region, [`buffer image height`][10] *must* be either [`None`] or greater than or
    ///   equal to [`image extent`][8] height and a multiple of [`texel block extent`][9] height of the
    ///   [`Format`] of the image.
    /// - If the source image has a multi-planar format, the [`aspect mask`][11] of each region
    ///   *must* contain only exactly one of either [`plane 0`][12], [`plane 1`][13] or [`plane 2`][14]
    ///   aspect.
    /// - If the source image doesn't have a depth/stencil or multi-planar format, each
    ///   region's [`buffer offset`][1] *must* be a multiple of the image format's
    ///   [`texel block size`][15].
    /// - If the source image has a depth/stencil format, each region's [`buffer offset`][1]
    ///   *must* be a multiple of 4.
    /// - If the source image has a multi-planar format, each region's [`buffer offset`][1]
    ///   *must* be a multiple of the given [`plane aspect's`][16] [`format`][17] [`texel block size`][15].
    /// - The regions specified *must* not overlap in device memory. That is, if both of the
    ///   objects are bound by the same [`vk::DeviceMemory`] object, the offsets and sizes/extents
    ///   in `regions` *must* not cause overlap within that memory.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html>
    ///
    /// [1]: BufferImageCopy::buffer_offset
    /// [2]: BufferImageCopy::calculate_buffer_size
    /// [3]: BufferImageCopy::image_offset
    /// [4]: BufferImageCopy::image_extent
    /// [5]: BufferImageCopy::image_subresource
    /// [6]: ImageSubresourceLayers
    /// [7]: BufferImageCopy::buffer_row_length
    /// [8]: BufferImageCopy::image_extent
    /// [9]: FormatCompatibilityClass::texel_block_extent
    /// [10]: BufferImageCopy::buffer_image_height
    /// [11]: ImageAspects
    /// [12]: ImageAspects::PLANE_0
    /// [13]: ImageAspects::PLANE_1
    /// [14]: ImageAspects::PLANE_2
    /// [15]: FormatCompatibilityClass::texel_block_size
    /// [16]: Format::plane_formats
    /// [17]: ImageAspects::plane
    pub fn copy_image_to_buffer(
        &mut self,
        src_image_id: ImageId,
        dst_buffer_id: BufferId,
        regions: &[BufferImageCopy],
        ordering: CommandOrdering,
    ) -> Result<()> {
        self.wait_scope |= vk::PipelineStageFlags2::COPY;
        self.signal_scope |= vk::PipelineStageFlags2::COPY;
        let cache = unsafe { &mut *self.recorder.cache().get() };
        let command_id = self.command_id;
        self.recorder.write_resources(|guard| {
            let image: *mut ImageMeta = guard.register_image(
                src_image_id.slot_index(),
                command_id.index(),
            )?;
            let image = unsafe {
                &mut *image
            };
            let buffer: *mut BufferMeta = guard.register_buffer(
                dst_buffer_id
            )?;
            let buffer = unsafe {
                &mut *buffer
            };
            let img_properties = image.properties();
            let buf_properties = buffer.properties();
            if !img_properties.usage.contains(ImageUsages::TRANSFER_SRC) {
                return Err(Error::just_context(
                    "source image usage doesn't contain transfer source usage"
                ))
            }
            if !img_properties.format_features.contains(FormatFeatures::TRANSFER_SRC) {
                return Err(Error::just_context(format!(
                    "source image format {} doesn't support transfer source operations",
                    img_properties.format,
                )))
            }
            if img_properties.samples != MsaaSamples::X1 {
                return Err(Error::just_context(format!(
                    "source image sample count {} is not X1",
                    img_properties.samples,
                )))
            }
            if !buf_properties.usage.contains(BufferUsages::TRANSFER_DST) {
                return Err(Error::just_context(
                    "destination buffer doesn't contain transfer destination usage"
                ))
            }
            let queue_family_index = self.queue.family_index();
            let src_state = ImageSubresourceState::new(
                vk::PipelineStageFlags2::COPY,
                vk::AccessFlags2::TRANSFER_READ,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                queue_family_index
            );
            let dst_state = BufferState::new(
                vk::PipelineStageFlags2::COPY,
                vk::AccessFlags2::TRANSFER_WRITE,
                queue_family_index
            );
            let command_buffer = self.command_buffer;
            let tmp_alloc = self.gpu.tmp_alloc();
            let tmp_alloc = tmp_alloc.guard();
            let format_class = img_properties.format.compatibility();
            let texel_block_size = format_class.texel_block_size();
            let texel_block_extent: Dimensions = format_class.texel_block_extent();
            let planes = img_properties.format
                .plane_formats()
                .map(|format| format.compatibility().texel_block_size());
            let is_multiplanar = img_properties.format.plane_count() > 1;
            let is_depth_stencil = img_properties.format.is_depth_stencil();
            let n_regions = regions.len() as u32;
            let mut image_memory_barrier_ranges = FixedVec32::with_capacity(
                n_regions, &tmp_alloc
            ).context("alloc failed")?;
            let mut buffer_memory_barrier_ranges = FixedVec32::with_capacity(
                n_regions, &tmp_alloc
            ).context("alloc failed")?;
            let mut vk_regions = FixedVec32::with_capacity(
                n_regions, &tmp_alloc
            ).context("alloc failed")?;
            for &region in regions {
                let layer_count = region.image_subresource
                    .effective(img_properties.array_layers)
                    .layer_count;
                let buffer_size = region.calculate_buffer_size(
                    format_class,
                    img_properties.format,
                    region.image_subresource.aspect_mask,
                    layer_count,
                );
                if region.buffer_offset + buffer_size > buf_properties.size {
                    return Err(Error::just_context(format!(
                        "destination offset {} + calculated size {} was out of range of the destination buffer size {}",
                        region.buffer_offset, buffer_size, buf_properties.size,
                    )))
                }
                if !region.image_offset.is_multiple_of(texel_block_extent) {
                    return Err(Error::just_context(format!(
                        "region source image offset {} is not a multiple of format {} texel block extent {}",
                        region.image_offset, img_properties.format, texel_block_extent,
                    )))
                }
                if region.image_extent.is_zero() {
                    return Err(Error::just_context(format!(
                        "region image extent {} is zero",
                        region.image_extent,
                    )))
                }
                if !region.image_offset.is_in_range(img_properties.dimensions, region.image_extent) {
                    return Err(Error::just_context(format!(
                        "source image offset {} + extent {} is out of range of image dimensions {}",
                        region.image_offset, region.image_extent, region.image_extent,
                    )))
                }
                if let Some(row_length) = region.buffer_row_length 
                {
                    if !row_length.get().is_multiple_of(texel_block_extent.width) {
                        return Err(Error::just_context(format!(
                            "destination buffer row length {row_length} is not a multiple of texel block extent width {} of format {}",
                            texel_block_extent.width, img_properties.format,
                        )))
                    } 
                    if row_length.get() > region.image_extent.width {
                        return Err(Error::just_context(format!(
                            "destination image extent width {} was out of range of destination buffer row length {row_length}",
                            region.image_extent.width,
                        )))
                    }
                }
                if let Some(image_height) = region.buffer_image_height
                {
                    if !image_height.get().is_multiple_of(texel_block_extent.height) {
                        return Err(Error::just_context(format!(
                            "destination buffer image height {image_height} is not a multiple of texel block extent height {} of format {}",
                            texel_block_extent.height, texel_block_extent.height,
                        )))
                    }
                    if image_height.get() > region.image_extent.height {
                        return Err(Error::just_context(format!(
                            "destination image extent height {} was out of range of destination buffer image height {image_height}",
                            region.image_extent.height,
                        )))
                    }
                }
                if is_multiplanar {
                    let Some(plane) = region.image_subresource.aspect_mask.plane() else {
                        return Err(Error::just_context(
                            "region aspect mask doesn't contain a single plane"
                        ))
                    };
                    if !region.buffer_offset.is_multiple_of(planes[plane as usize]) {
                        return Err(Error::just_context(format!(
                            "{}{}",
                            format_args!("region buffer offset is not a multiple of the image's texel block size {} at ",
                                planes[plane as usize]
                            ),
                            format_args!("aspect {}", region.image_subresource.aspect_mask),
                        )))
                    }
                } else if is_depth_stencil {
                    if !region.buffer_offset.is_multiple_of(4) {
                        return Err(Error::just_context(format!(
                            "region buffer offset {} is not a multiple of 4 when the image is a depth/stencil image",
                            region.buffer_offset,
                        )))
                    }
                } else if !region.buffer_offset.is_multiple_of(texel_block_size) {
                    return Err(Error::just_context(format!(
                        "region buffer offset {} is not a multiple of the image's texel block size {}",
                        region.buffer_offset, texel_block_size,
                    )))
                }
                let ranges1 = image.memory_barrier(
                    src_state,
                    region.image_subresource.into_range(),
                    true,
                    &mut cache.shader_resource_cache.image_memory_barrier_cache,
                ).context("source image memory barrier failed")?;
                let ranges2 = unsafe {
                    buffer.memory_barrier_unchecked(
                        region.buffer_offset,
                        buffer_size,
                        dst_state,
                        ordering,
                        &mut cache.shader_resource_cache.buffer_memory_barrier_cache,
                    )
                };
                image_memory_barrier_ranges.push(ranges1);
                buffer_memory_barrier_ranges.push(ranges2);
                vk_regions.push(region.into());
            }
            let image_memory_barriers = cache.shader_resource_cache.image_memory_barrier_cache.flush(
                &image_memory_barrier_ranges, &tmp_alloc,
            )?;
            let buffer_memory_barriers = cache.shader_resource_cache.buffer_memory_barrier_cache.flush(
                &buffer_memory_barrier_ranges, &tmp_alloc,
            )?;
            let dependency_info = vk::DependencyInfo {
                buffer_memory_barrier_count: buffer_memory_barriers.len(),
                p_buffer_memory_barriers: buffer_memory_barriers.as_ptr(),
                image_memory_barrier_count: image_memory_barriers.len(),
                p_image_memory_barriers: image_memory_barriers.as_ptr(),
                ..Default::default()
            };
            unsafe {
                self.gpu.device()
                .cmd_pipeline_barrier2(command_buffer, &dependency_info);
            }
            let info = vk::CopyImageToBufferInfo2 {
                src_image: image.handle(),
                src_image_layout: vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                dst_buffer: buffer.handle(),
                region_count: vk_regions.len(),
                p_regions: vk_regions.as_ptr(),
                ..Default::default()
            };
            unsafe {
                self.gpu.device()
                .cmd_copy_image_to_buffer2(command_buffer, &info);
            }
            Ok(())
        }) 
    }

    /// Clears an image with the given [`ClearColorValue`].
    ///
    /// To clear all layers and mip levels of the image, set [`subresources`][1] to [`None`].
    ///
    /// Otherwise pass each range you wish to clear.
    ///
    /// [1]: ImageSubresourceRange
    ///
    /// # Valid usage
    /// - `image_id` *must* be a valid [`ImageId`].
    /// - The image *must* have been created with the [`ImageUsages::TRANSFER_DST`] usage set.
    /// - The format of the image *must* support transfer destination operations.
    /// - The format of the image *must* have the [`ImageAspects::COLOR`] aspect.
    /// - Each subresource range *must* be a valid subresource for the image.
    /// - Each subresource range *must* only contain the [`ImageAspects::COLOR`] aspect.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearColorImage.html>
    pub fn clear_color_image(
        &mut self,
        image_id: ImageId,
        clear_value: ClearColorValue,
        subresources: Option<&[ImageSubresourceRange]>,
    ) -> Result<()> {
        self.wait_scope |= vk::PipelineStageFlags2::CLEAR;
        self.signal_scope |= vk::PipelineStageFlags2::CLEAR;
        let tmp_alloc = self.gpu.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let cache = unsafe {
            &mut *self.recorder.cache().get()
        };
        self.recorder.write_resources(|guard| {
            let image = guard.register_image(image_id.slot_index(), self.command_id.index())?;
            if let Some(err) = image.validate_usage(ImageUsages::TRANSFER_DST) {
                return Err(Error::new(err, "image has incompatible usage"))
            }
            if !image.properties().format_features.contains(FormatFeatures::TRANSFER_DST) {
                return Err(Error::just_context("image format doesn't support transfer destination operations"))
            }
            let ranges =
            if let Some(infos) = subresources {
                let mut ranges = FixedVec32::<vk::ImageSubresourceRange, _>
                    ::with_capacity(infos.len() as u32, &tmp_alloc)
                    .context("alloc failed")?;
                for &info in infos.iter() {
                    if let Err(err) = image.properties().validate_subresource_range(&info) {
                        return Err(Error::new(err, "given subresource range is incompatible with image"))
                    }
                    if info.aspect_mask != ImageAspects::COLOR {
                        return Err(Error::just_context(format!(
                            "subresource ranges must only contain color aspects, found aspect mask was {}",
                            info.aspect_mask,
                        )))
                    }
                    ranges.push(info.into());
                }
                ranges
            }
            else {
                let mut ranges = FixedVec32
                    ::with_capacity(1, &tmp_alloc)
                    .context("alloc failed")?;
                let mut range = ImageSubresourceRange
                    ::default()
                    .aspect_mask(image.properties().aspect_mask);
                if !range.aspect_mask.contains(ImageAspects::COLOR) {
                    return Err(Error::just_context(format!(
                        "image aspect mask must contain color aspect, found aspect mask was {}",
                        range.aspect_mask,
                    )))
                }
                range.aspect_mask = ImageAspects::COLOR;
                ranges.push(range.into());
                ranges
            };
            let state = ImageSubresourceState::new(
                vk::PipelineStageFlags2::CLEAR,
                vk::AccessFlags2::TRANSFER_WRITE,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                self.queue.family_index()
            );
            let command_buffer = self.command_buffer;
            let mut memory_barrier_ranges = FixedVec32::with_capacity(
                ranges.len(), &tmp_alloc,
            ).context("alloc failed")?;
            for &range in &ranges {
                unsafe {
                    memory_barrier_ranges.push(image.memory_barrier_unchecked(
                        state,
                        range.into(),
                        false,
                        &mut cache.shader_resource_cache.image_memory_barrier_cache,
                    ))
                };
            }
            let memory_barriers = cache.shader_resource_cache.image_memory_barrier_cache.flush(
                &memory_barrier_ranges,
                &tmp_alloc,
            )?;
            let dependency_info = vk::DependencyInfo {
                image_memory_barrier_count: memory_barriers.len(),
                p_image_memory_barriers: memory_barriers.as_ptr(),
                ..Default::default()
            };
            unsafe {
                self.gpu.device().cmd_pipeline_barrier2(
                    command_buffer,
                    &dependency_info
                );
                self.gpu.device().cmd_clear_color_image(
                    command_buffer,
                    image.handle(),
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &clear_value.into(),
                    &ranges,
                );
            }
            Ok(())
        }) 
    }

    /// Clears an image with the given `depth` and `stencil` values.
    ///
    /// To clear all layers and mip levels of the image, set [`subresources`][1] to [`None`].
    ///
    /// Otherwise pass each range you wish to clear.
    ///
    /// [1]: ImageSubresourceRange
    ///
    /// # Valid usage
    /// - `image_id` *must* be a valid [`ImageId`].
    /// - The image *must* have been created with the [`ImageUsages::TRANSFER_DST`] usage set.
    /// - The format of the image *must* support transfer destination operations.
    /// - The format of the image *must* have [`ImageAspects::DEPTH`] and/or
    ///   [`ImageAspects::STENCIL`] aspects.
    /// - Each subresource range *must* be a valid subresource for the image.
    /// - Each subresource range *must* only contain [`ImageAspects::DEPTH`] and/or
    ///   [`ImageAspects::STENCIL`] aspects.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearDepthStencilImage.html>
    pub fn clear_depth_stencil_image(
        &mut self,
        image_id: ImageId,
        depth: f32,
        stencil: u32,
        subresources: Option<&[ImageSubresourceRange]>,
    ) -> Result<()>
    {
        self.wait_scope |= vk::PipelineStageFlags2::CLEAR;
        self.signal_scope |= vk::PipelineStageFlags2::CLEAR;
        let tmp_alloc = self.gpu.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let cache = unsafe {
            &mut *self.recorder.cache().get()
        };
        self.recorder.write_resources(|guard| {
            let image = guard.register_image(image_id.slot_index(), self.command_id.index())?;
            if let Some(err) = image.validate_usage(ImageUsages::TRANSFER_DST) {
                return Err(Error::new(err, "image has incompatible usage"))
            }
            if !image.properties().format_features.contains(FormatFeatures::TRANSFER_DST) {
                return Err(Error::just_context("image format doesn't support transfer destination operations"))
            }
            let ranges = {
                if let Some(infos) = subresources {
                    let mut ranges = FixedVec32::<vk::ImageSubresourceRange, _>
                        ::with_capacity(infos.len() as u32, &tmp_alloc)
                        .context("alloc failed")?;
                    for &info in infos.iter() {
                        if let Err(err) = image.properties().validate_subresource_range(&info) {
                            return Err(Error::new(err, "given subresource range is incompatible with image"))
                        }
                        if info.aspect_mask & (ImageAspects::DEPTH | ImageAspects::STENCIL) != info.aspect_mask {
                            return Err(Error::just_context(format!(
                                "subresource ranges must only contain depth or stencil aspects, found aspect mask was {}",
                                info.aspect_mask,
                            )))
                        }
                        ranges.push(info.into());
                    }
                    ranges
                }
                else {
                    let mut ranges = FixedVec32
                        ::with_capacity(1, &tmp_alloc)
                        .context("allocfailed")?;
                    let mut range = ImageSubresourceRange
                        ::default()
                        .aspect_mask(image.properties().aspect_mask);
                    let aspect_mask = range.aspect_mask & (ImageAspects::DEPTH | ImageAspects::STENCIL);
                    if aspect_mask.is_empty() {
                        return Err(Error::just_context(format!(
                            "image aspect mask {} doesn't contain depth or stencil aspect",
                            range.aspect_mask,
                        )))
                    }
                    range.aspect_mask = aspect_mask;
                    ranges.push(range.into());
                    ranges
                }
            };
            let state = ImageSubresourceState::new(
                vk::PipelineStageFlags2::CLEAR,
                vk::AccessFlags2::TRANSFER_WRITE,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                self.queue.family_index(),
            );
            let command_buffer = self.command_buffer;
            let mut memory_barrier_ranges = FixedVec32
                ::with_capacity(ranges.len(), &tmp_alloc)
                .context("alloc failed")?;
            for &range in &ranges {
                unsafe {
                    memory_barrier_ranges.push(image.memory_barrier_unchecked(
                        state,
                        range.into(),
                        false,
                        &mut cache.shader_resource_cache.image_memory_barrier_cache,
                    ))
                }
            }
            let memory_barriers = cache.shader_resource_cache.image_memory_barrier_cache.flush(
                &memory_barrier_ranges, &tmp_alloc
            )?;
            let dependency_info = vk::DependencyInfo {
                image_memory_barrier_count: memory_barriers.len(),
                p_image_memory_barriers: memory_barriers.as_ptr(),
                ..Default::default()
            };
            unsafe {
                self.gpu.device().cmd_pipeline_barrier2(
                    command_buffer,
                    &dependency_info
                );
                self.gpu.device().cmd_clear_depth_stencil_image(
                    command_buffer,
                    image.handle(),
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &vk::ClearDepthStencilValue { depth, stencil, },
                    &ranges,
                );
            }
            Ok(())
        })
    }

    /// Performs an image blit.
    ///
    /// For mip map generation, consider using [`gen_mip_map`][1] for efficiency.
    ///
    /// # Valid usage
    /// - `src_image_id` and `dst_image_id` *must* be valid [`ImageId`]s.
    /// - Source image *must* have been created with [`ImageUsages::TRANSFER_SRC`] set.
    /// - Destination image *must* have been created with [`ImageUsages::TRANSFER_DST`] set.
    /// - Both images *must* have msaa sample count of 1.
    /// - Format features of the source image *must* contain [`FormatFeatures::BLIT_SRC`] bit.
    /// - Format features of the destination image *must* contain [`FormatFeatures::BLIT_DST`]
    ///   bit.
    /// - If the source image format is a depth/stencil format, `filter` *must* be
    ///   [`Filter::NEAREST`].
    /// - If `filter` is [`Filter::LINEAR`], the format features of the source image *must*
    ///   contain [`FormatFeatures::SAMPLED_IMAGE_FILTER_LINEAR`].
    /// - Both [`source offsets`][2] and [`destination offsets`][3] *must* not be outside the range
    ///   of the dimensions of the image at the mip level defined in [`source subresources`][4] and
    ///   [`destination subresources`][5] respectively. You can calculate the dimensions of subresources
    ///   at a mip level with [`Dimensions::lod`].
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html>
    ///
    /// [1]: CopyCommands::gen_mip_map
    /// [2]: ImageBlitRegion::src_offsets
    /// [3]: ImageBlitRegion::dst_offsets
    /// [4]: ImageBlitRegion::src_subresource
    /// [5]: ImageBlitRegion::dst_subresource
    pub fn blit_image(
        &mut self,
        src_image_id: ImageId,
        dst_image_id: ImageId,
        regions: &[ImageBlitRegion],
        filter: Filter,
    ) -> Result<()> {
        self.wait_scope |= vk::PipelineStageFlags2::BLIT;
        self.signal_scope |= vk::PipelineStageFlags2::BLIT;
        let mut src_image_layout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;
        let mut src_access_mask = vk::AccessFlags2::TRANSFER_READ;
        let mut dst_image_layout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
        let mut dst_access_mask = vk::AccessFlags2::TRANSFER_WRITE;
        if src_image_id == dst_image_id {
            let n_regions = regions.len();
            if regions[0..n_regions - 1].iter().enumerate().any(|(i, a)| {
                let src = a.src_subresource;
                regions[i + 1..]
                    .iter()
                    .any(|b| src.overlaps(b.dst_subresource))
            }) {
                src_image_layout = vk::ImageLayout::GENERAL;
                src_access_mask = vk::AccessFlags2::TRANSFER_READ | vk::AccessFlags2::TRANSFER_WRITE;
                dst_image_layout = vk::ImageLayout::GENERAL;
                dst_access_mask = vk::AccessFlags2::TRANSFER_READ | vk::AccessFlags2::TRANSFER_WRITE;
            }
        }
        let command_buffer = self.command_buffer;
        let cache = unsafe { &mut *self.recorder.cache().get() };
        self.recorder.write_resources(|guard| {
            let src_image: *mut ImageMeta = guard.register_image(
                src_image_id.slot_index(),
                self.command_id.index(),
            )?;
            let src_image = unsafe { &mut *src_image };
            let dst_image: *mut ImageMeta = guard.register_image(
                dst_image_id.slot_index(),
                self.command_id.index(),
            )?;
            let dst_image = unsafe { &mut *dst_image };
            let src_properties = src_image.properties();
            let dst_properties = dst_image.properties();
            let src_handle = src_image.handle();
            let dst_handle = dst_image.handle();
            if !src_properties.usage.contains(ImageUsages::TRANSFER_SRC) {
                return Err(Error::just_context(
                    "source image usage must contain transfer source usage"
                ))
            }
            if !dst_properties.usage.contains(ImageUsages::TRANSFER_DST) {
                return Err(Error::just_context(
                    "destination image usage must contain transfer destination usage"
                ))
            }
            if src_properties.samples != MsaaSamples::X1 ||
                dst_properties.samples != MsaaSamples::X1
            {
                return Err(Error::just_context("both source and destination images don't both have msaa sample count of 1"))
            }
            if !src_properties.format_features.contains(FormatFeatures::BLIT_SRC) {
                return Err(Error::just_context(
                    "source image format features must contain blit source feature"
                ))
            }
            if !dst_properties.format_features.contains(FormatFeatures::BLIT_DST) {
                return Err(Error::just_context(
                    "source image format features must contain blit destination feature"
                ))
            }
            if filter == Filter::LINEAR {
                if src_properties.aspect_mask.intersects(ImageAspects::DEPTH | ImageAspects::STENCIL) {
                    return Err(Error::just_context(format!(
                        "depth/stencil images only support nearest filtering for blitting, found filter was {}",
                        filter,
                    )))
                }
                if !src_properties.format_features.contains(FormatFeatures::SAMPLED_IMAGE_FILTER_LINEAR) {
                    return Err(Error::just_context(format!(
                        "image doesn't support linear filtering for blitting, image format features must contain {}",
                        FormatFeatures::SAMPLED_IMAGE_FILTER_LINEAR,
                    )))
                }
            }
            let queue_family_index = self.queue.family_index();
            let src_state = ImageSubresourceState::new(
                vk::PipelineStageFlags2::BLIT,
                src_access_mask,
                src_image_layout,
                queue_family_index,
            );
            let dst_state = ImageSubresourceState::new(
                vk::PipelineStageFlags2::BLIT,
                dst_access_mask,
                dst_image_layout,
                queue_family_index,
            );
            let tmp_alloc = self.gpu.tmp_alloc();
            let tmp_alloc = tmp_alloc.guard();
            let n_regions = regions.len() as u32;
            let mut vk_regions = FixedVec32
                ::with_capacity(n_regions, &tmp_alloc)
                .context("alloc failed")?;
            let mut image_memory_barrier_ranges = FixedVec32::with_capacity(
                n_regions * 2, &tmp_alloc
            ).context("alloc failed")?;
            for region in regions {
                let range1 = src_image.memory_barrier(
                    src_state,
                    region.src_subresource.into_range(),
                    true,
                    &mut cache.shader_resource_cache.image_memory_barrier_cache,
                ).context("source image memory barrier failed")?;
                let range2 = src_image.memory_barrier(
                    dst_state,
                    region.dst_subresource.into_range(),
                    true,
                    &mut cache.shader_resource_cache.image_memory_barrier_cache,
                ).context("destination image memory barrier failed")?;
                image_memory_barrier_ranges.fast_append(&[range1, range2]);
                let lod = src_properties.dimensions.lod(region.src_subresource.mip_level);
                for offset in region.src_offsets {
                    if offset.x > lod.width ||
                        offset.y > lod.height ||
                        offset.z > lod.depth
                    {
                        return Err(Error::just_context(format!(
                            "source region offset {offset} was out of range of image subresource dimensions {lod} with mip level {}",
                            region.src_subresource.mip_level,
                        )))
                    }
                }
                let lod = dst_properties.dimensions.lod(region.dst_subresource.mip_level);
                for offset in region.dst_offsets {
                    if offset.x > lod.width ||
                        offset.y > lod.height ||
                        offset.z > lod.depth
                    {
                        return Err(Error::just_context(format!(
                            "destination region offset {offset} was out of range of image subresource dimensions {lod} with mip level {}",
                            region.dst_subresource.mip_level,
                        )))
                    }
                }
                vk_regions.push(vk::ImageBlit2 {
                    src_subresource: region.src_subresource.into(),
                    src_offsets: [region.src_offsets[0].into(), region.src_offsets[1].into()],
                    dst_subresource: region.dst_subresource.into(),
                    dst_offsets: [region.dst_offsets[0].into(), region.dst_offsets[1].into()],
                    ..Default::default()
                });
            }
            let image_memory_barriers = cache.shader_resource_cache.image_memory_barrier_cache.flush(
                &image_memory_barrier_ranges, &tmp_alloc
            )?;
            let dependency_info = vk::DependencyInfo {
                image_memory_barrier_count: image_memory_barriers.len(),
                p_image_memory_barriers: image_memory_barriers.as_ptr(),
                ..Default::default()
            };
            unsafe {
                self.gpu.device().cmd_pipeline_barrier2(
                    command_buffer, &dependency_info
                );
            }
            let info = vk::BlitImageInfo2 {
                src_image: src_handle,
                src_image_layout,
                dst_image: dst_handle,
                dst_image_layout,
                region_count: vk_regions.len(),
                p_regions: vk_regions.as_ptr(),
                filter: filter.into(),
                ..Default::default()
            };
            unsafe {
                self.gpu.device().cmd_blit_image2(
                    command_buffer, &info
                );
            }
            Ok(())
        })
    }

    /// Generates mip maps from the first mip level of an image.
    ///
    /// # Valid usage
    /// - `image_id` *must* be a valid [`ImageId`].
    /// - The specified usage of the image *must* contain [`ImageUsages::TRANSFER_SRC`] and
    ///   [`ImageUsages::TRANSFER_DST`] bits.
    /// - Format features of the image *must* contain [`FormatFeatures::BLIT_SRC`] and
    ///   [`FormatFeatures::BIT_DST`] bits.
    /// - The image *must* have [`msaa sample count`][1] of 1.
    /// - If the image format has a depth/stencil [`aspect`][2], `filter` *must* be [`Filter::NEAREST`].
    /// - If `filter` is [`Filter::LINEAR`], the format features of the image *must* contain
    ///   [`FormatFeatures::SAMPLED_IMAGE_FILTER_LINEAR`].
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html>
    ///
    /// [1]: MsaaSamples
    /// [2]: ImageAspects
    pub fn gen_mip_map(
        &mut self,
        image_id: ImageId,
        filter: Filter,
    ) -> Result<()>
    {
        self.wait_scope |= vk::PipelineStageFlags2::BLIT;
        self.signal_scope |= vk::PipelineStageFlags2::BLIT;
        let tmp_alloc = self.gpu.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let command_id = self.command_id;
        let cache = unsafe {
            &mut *self.recorder.cache().get()
        };
        self.recorder.write_resources(|guard| {
            let image = guard.register_image(
                image_id.slot_index(),
                command_id.index(),
            )?;
            if let Some(err) = image.validate_usage(
                ImageUsages::TRANSFER_SRC |
                ImageUsages::TRANSFER_DST
            ) {
                return Err(Error::new(err, "image has incompatible usage"))
            }
            let handle = image.handle();
            let properties = image.properties();
            if filter == Filter::LINEAR {
                if properties.aspect_mask.intersects(ImageAspects::DEPTH | ImageAspects::STENCIL) {
                    return Err(Error::just_context(format!(
                        "depth/stencil images only support nearest filtering for blitting, found filter was {}",
                        filter,
                    )))
                }
                if !properties.format_features.contains(FormatFeatures::SAMPLED_IMAGE_FILTER_LINEAR) {
                    return Err(Error::just_context(format!(
                        "image doesn't support linear filtering for blitting, image format features must contain {}",
                        FormatFeatures::SAMPLED_IMAGE_FILTER_LINEAR,
                    )))
                }
            }
            if properties.samples != MsaaSamples::X1 {
                return Err(Error::just_context(format!(
                    "image sample count {} must be 1", 
                    properties.samples,
                )))
            }
            if !properties.format_features.contains(FormatFeatures::BLIT_SRC | FormatFeatures::BLIT_DST) {
                return Err(Error::just_context(
                    "image format doesn't support image blitting"
                ))
            }
            let mip_levels = properties.mip_levels;
            let mut mip_dimensions = properties.dimensions;
            let queue_family_index = self.queue.family_index();
            let src_state = ImageSubresourceState::new(
                vk::PipelineStageFlags2::TRANSFER,
                vk::AccessFlags2::TRANSFER_READ,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                queue_family_index,
            );
            let dst_state = ImageSubresourceState::new(
                vk::PipelineStageFlags2::TRANSFER,
                vk::AccessFlags2::TRANSFER_WRITE,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                queue_family_index,
            );
            let command_buffer = self.command_buffer;
            let ranges = image.memory_barrier(
                dst_state,
                ImageSubresourceRange
                    ::default()
                    .aspect_mask(image.properties().aspect_mask),
                true,
                &mut cache.shader_resource_cache.image_memory_barrier_cache,
            ).context("image memory barrier failed")?;
            let device = self.gpu.device();
            let mem_barriers = cache.shader_resource_cache.image_memory_barrier_cache.flush(
                &[ranges], &tmp_alloc
            )?;
            let dependency_info = vk::DependencyInfo {
                image_memory_barrier_count: mem_barriers.len(),
                p_image_memory_barriers: mem_barriers.as_ptr(),
                ..Default::default()
            };
            unsafe {
                device.cmd_pipeline_barrier2(command_buffer, &dependency_info);
            }
            let filter: vk::Filter = filter.into();
            for i in 1..mip_levels {
                let next_mip_dimensions = properties.dimensions.lod(i);
                let subresource = ImageSubresourceRange::default()
                    .aspect_mask(properties.aspect_mask)
                    .base_mip_level(i - 1)
                    .level_count(1)
                    .layer_count(properties.array_layers);
                let ranges = image.memory_barrier(
                    src_state,
                    subresource,
                    true,
                    &mut cache.shader_resource_cache.image_memory_barrier_cache,
                ).context("image memory barrier failed")?;
                let mem_barriers = cache.shader_resource_cache.image_memory_barrier_cache.flush(
                    &[ranges], &tmp_alloc
                )?;
                let dependency_info = vk::DependencyInfo {
                    image_memory_barrier_count: mem_barriers.len(),
                    p_image_memory_barriers: mem_barriers.as_ptr(),
                    ..Default::default()
                };
                unsafe {
                    device.cmd_pipeline_barrier2(command_buffer, &dependency_info);
                }
                let blit = vk::ImageBlit2 {
                    src_offsets: [
                        vk::Offset3D { x: 0, y: 0, z: 0 },
                        vk::Offset3D {
                            x: mip_dimensions.width as i32, 
                            y: mip_dimensions.height as i32,
                            z: 1,
                        },
                    ],
                    src_subresource: ImageSubresourceLayers::default()
                        .aspect_mask(properties.aspect_mask)
                        .mip_level(i - 1)
                        .into(),
                    dst_offsets: [
                        vk::Offset3D { x: 0, y: 0, z: 0 },
                        vk::Offset3D {
                            x: next_mip_dimensions.width as i32,
                            y: next_mip_dimensions.height as i32,
                            z: 1,
                        }
                    ],
                    dst_subresource: ImageSubresourceLayers::default()
                        .aspect_mask(properties.aspect_mask)
                        .mip_level(i)
                        .into(),
                    ..Default::default()
                };
                let blit_info = vk::BlitImageInfo2 {
                    src_image: handle,
                    src_image_layout: vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    dst_image: handle,
                    dst_image_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    region_count: 1,
                    p_regions: &blit,
                    filter,
                    ..Default::default()
                };
                unsafe {
                    device.cmd_blit_image2(
                        command_buffer,
                        &blit_info,
                    );
                }
                mip_dimensions = next_mip_dimensions;
            }
            Ok(())
        })
    }
}

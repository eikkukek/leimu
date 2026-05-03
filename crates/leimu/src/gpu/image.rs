mod state;
mod create_info;
mod properties;
mod error;
mod view;

use core::{
    ptr::NonNull,
    fmt::{self, Display},
    marker::PhantomData,
};

use tuhka::vk;
use leimu_mem::{
    vec::{Vec32, NonNullVec32},
    alloc::{LocalAlloc, StdAlloc, Layout},
    arena::{self, Arena},
    slot_map::SlotIndex,
    int::Integer,
};

use {
    crate::gpu::prelude::{
        subresource_state::*,
        *
    },
    crate::error::*,
};

pub use error::ImageSubresourceOutOfRangeError;
pub use create_info::*;
pub use view::*;
pub use state::*;
pub(crate) use properties::ImageProperties;

enum MemorySource {
    Joint { _device_memory: DeviceMemoryObj, arena_size: usize },
    Swapchain,
}

pub struct ImageMeta {
    device: Device,
    handle: vk::Image,
    image_views: Vec32<ImageView>,
    properties: ImageProperties,
    states: NonNullVec32<'static, NonNullVec32<'static, ImageLayerRange>>,
    memory: MemorySource,
}

impl ResourceMeta for ImageMeta {

    const NAME: &str = "image";
}

unsafe impl Send for ImageMeta {}
unsafe impl Sync for ImageMeta {}

impl ImageMeta {

    fn new(
        device: Device,
        create_info: &ImageCreateInfo<'_>,
        bind_memory_info: &mut vk::BindImageMemoryInfo<'static>,
    ) -> Result<Self>
    {
        let mut image_type = vk::ImageType::TYPE_2D;
        if create_info.dimensions.depth > 1 {
            if create_info.array_layers != 1 {
                return Err(Error::just_context(
                    "image layers must be 1 if depth is greater than 1"
                ))
            }
            image_type = vk::ImageType::TYPE_3D;
        }
        if create_info.format == Format::Undefined {
            return Err(Error::just_context(
                "image format must be defined"
            ))
        }
        let mut flags = Default::default();
        if create_info.mutable_format {
            flags |= vk::ImageCreateFlags::MUTABLE_FORMAT;
        }
        if create_info.cube_map {
            flags |= vk::ImageCreateFlags::CUBE_COMPATIBLE;
            if create_info.dimensions.width != create_info.dimensions.height ||
                create_info.dimensions.depth != 1
            {
                return Err(Error::just_context(format!(
                    "width ({}) and height ({}) of a cube map compatible image must be equal and depth ({}) must be 1",
                    create_info.dimensions.width, create_info.dimensions.height, create_info.dimensions.depth,
                )))
            }
        }
        if create_info.dimensions.is_zero() {
            return Err(Error::just_context(format!(
                "image dimensions {} must not be zero",
                create_info.dimensions,
            )))
        }
        if create_info.array_layers == 0 {
            return Err(Error::just_context(
                "image layers must be greater than 0",
            ))
        }
        if create_info.cube_map && create_info.array_layers < 6 {
            return Err(Error::just_context(format!(
                "layer count {} of a cube map/array image must be at least 6",
                create_info.array_layers,
            )))
        }
        if create_info.mip_levels == 0 {
            return Err(Error::just_context(
                "mip levels must be greater than zero",
            ))
        }
        let mut image_format_properties = vk::ImageFormatProperties2::default();
        unsafe {
            device.instance().get_physical_device_image_format_properties2(
                device.physical_device().handle(),
                &vk::PhysicalDeviceImageFormatInfo2 {
                    format: create_info.format.into(),
                    ty: image_type,
                    tiling: vk::ImageTiling::OPTIMAL,
                    usage: create_info.usage.into(),
                    flags,
                    ..Default::default()
                },
                &mut image_format_properties,
            ).context("failed to get image format properties")?;
        }
        let image_format_properties = image_format_properties.image_format_properties;
        let mut max_dimensions: Dimensions = image_format_properties.max_extent.into();
        if create_info.usage.intersects(
            ImageUsages::COLOR_ATTACHMENT |
            ImageUsages::DEPTH_STENCIL_ATTACHMENT |
            ImageUsages::INPUT_ATTACHMENT
        ) {
            let limits = device.physical_device().limits();
            max_dimensions.width = max_dimensions.width.min(limits.max_framebuffer_width());
            max_dimensions.height = max_dimensions.width.min(limits.max_framebuffer_height());
            max_dimensions.depth = 1;
        }
        if max_dimensions.width < create_info.dimensions.width ||
            max_dimensions.height < create_info.dimensions.height ||
            max_dimensions.depth < create_info.dimensions.depth
        {
            return Err(Error::just_context(format!(
                "given dimensions {} are greater than the maximum supported dimensions {} for the image",
                create_info.dimensions, max_dimensions,
            )))
        }
        if image_format_properties.max_mip_levels < create_info.mip_levels {
            return Err(Error::just_context(format!(
                "given mip levels {} are greater than the maximum supported mip levels {} for the image",
                create_info.mip_levels, image_format_properties.max_mip_levels,
            )))
        }
        if image_format_properties.max_array_layers < create_info.array_layers {
            return Err(Error::just_context(format!(
                "given array layers {} are greater thane the maximum supported array layers {} for the image",
                create_info.array_layers, image_format_properties.max_array_layers,
            )))
        }
        if create_info.samples.count_ones() != 1 {
            return Err(Error::just_context(format!(
                "image sample count {} must only contain one flag set",
                create_info.samples,
            )))
        }
        let supported_samples: MsaaSamples = image_format_properties.sample_counts.into();
        if supported_samples & create_info.samples != create_info.samples {
            return Err(Error::just_context(format!(
                "image doesn't support given sample count {}, supported samples for this image are {supported_samples}",
                create_info.samples,
            )))
        }
        let vk_create_info = vk::ImageCreateInfo {
            s_type: vk::StructureType::IMAGE_CREATE_INFO,
            flags,
            image_type,
            format: create_info.format.into(),
            extent: create_info.dimensions.into(),
            mip_levels: create_info.mip_levels,
            array_layers: create_info.array_layers,
            samples: create_info.samples.into(),
            tiling: vk::ImageTiling::OPTIMAL,
            usage: create_info.usage.into(),
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            ..Default::default()
        };
        let mut dedicated_memory_requirements = vk::MemoryDedicatedRequirements::default();
        let mut mem_requirements = vk::MemoryRequirements2
            ::default().push_next(&mut dedicated_memory_requirements);
        unsafe {
            let device_mem_requirements = vk::DeviceImageMemoryRequirements {
                s_type: vk::StructureType::DEVICE_IMAGE_MEMORY_REQUIREMENTS,
                p_create_info: &vk_create_info,
                ..Default::default()
            };
            device.get_device_image_memory_requirements(
                &device_mem_requirements, &mut mem_requirements,
            );
        };
        let memory = unsafe {
            create_info.memory_binder.alloc(&mem_requirements)
            .context("failed to allocate GPU memory for image")?
        };
        let handle = unsafe {
            device.create_image(&vk_create_info, None)
            .context("failed to create Vulkan image")?
        }.value;
        *bind_memory_info = vk::BindImageMemoryInfo {
            image: handle,
            memory: vk::DeviceMemory::from_raw(memory.handle()),
            memory_offset: memory.offset(),
            ..Default::default()
        };
        let mut format_properties3 = vk::FormatProperties3::default();
        let mut format_properties = vk::FormatProperties2
            ::default()
            .push_next(&mut format_properties3);
        unsafe {
            device.instance().get_physical_device_format_properties2(
                device.physical_device().handle(),
                create_info.format.into(),
                &mut format_properties,
            );
        }
        let properties = ImageProperties {
            dimensions: create_info.dimensions,
            format: create_info.format,
            aspect_mask: create_info.aspects,
            usage: create_info.usage,
            samples: create_info.samples,
            array_layers: create_info.array_layers,
            mip_levels: create_info.mip_levels,
            create_flags: flags,
            format_resolve_modes: create_info.resolve_modes,
            format_features: FormatFeatures::from_raw(
                format_properties3.optimal_tiling_features.as_raw()
            ),
        };
        let layers =  properties.array_layers;
        let num_levels = properties.aspect_mask.count_ones() * properties.mip_levels;
        let arena = Arena::new(
            (num_levels as usize * size_of::<NonNullVec32<ImageLayerRange>>()).next_multiple_of(8) +
            (num_levels * layers) as usize * size_of::<ImageLayerRange>()
        ).context("failed to create arena")?;
        let mut states = NonNullVec32
            ::with_capacity(num_levels, &arena)
            .unwrap()
            .into_static();
        states.resize_with(num_levels, || {
            let mut vec = NonNullVec32::with_capacity(layers, &arena).unwrap();
            vec.push(ImageLayerRange {
                state: ImageSubresourceState {
                    stage_mask: vk::PipelineStageFlags2::NONE,
                    access_mask: vk::AccessFlags2::NONE,
                    layout: vk::ImageLayout::UNDEFINED,
                    queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                },
                base_array_layer: 0,
                layer_count: layers,
            });
            vec.into_static()
        });
        Ok(Self {
            device,
            handle,
            image_views: Vec32::with_capacity(1),
            properties,
            states,
            memory: MemorySource::Joint { _device_memory: memory, arena_size: arena.into_raw_parts().1, },
        })
    }

    pub(crate) unsafe fn from_swapchain_image(
        device: Device,
        handle: vk::Image,
        dimensions: Dimensions,
        format: vk::Format,
        usage: vk::ImageUsageFlags,
        alloc: &impl LocalAlloc<Error = arena::Error>
    ) -> Result<Self> {
        let mut format_properties3 = vk::FormatProperties3::default();
        let mut format_properties = vk::FormatProperties2
            ::default()
            .push_next(&mut format_properties3);
        unsafe {
            device.instance().get_physical_device_format_properties2(
                device.physical_device().handle(),
                format,
                &mut format_properties,
            );
        }
        let format = unsafe {
            ::core::mem::transmute::<
                vk::Format,
                Format
            >(format)
        };
        let properties = ImageProperties {
            dimensions,
            format,
            aspect_mask: ImageAspects::COLOR,
            usage: ImageUsages::from_raw(usage.as_raw()),
            samples: MsaaSamples::X1,
            array_layers: 1,
            mip_levels: 1,
            create_flags: vk::ImageCreateFlags::empty(),
            format_resolve_modes: Default::default(),
            format_features: FormatFeatures::from_raw(
                format_properties3.optimal_tiling_features.as_raw()
            ),
        };
        let layers = properties.array_layers;
        let num_levels = properties.aspect_mask.count_ones() * properties.mip_levels;
        let mut states = NonNullVec32
            ::with_capacity(num_levels, alloc)
            .context("alloc failed")?
            .into_static();
        states.push({
            let mut vec = NonNullVec32
                ::with_capacity(1, alloc)
                .context("alloc failed")?;
            vec.push(ImageLayerRange {
                state: ImageSubresourceState {
                    stage_mask: vk::PipelineStageFlags2::NONE,
                    access_mask: vk::AccessFlags2::NONE,
                    layout: vk::ImageLayout::UNDEFINED,
                    queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                },
                base_array_layer: 0,
                layer_count: layers,
            });
            vec.into_static()
        });
        Ok(Self {
            device,
            handle,
            image_views: Vec32::with_capacity(1),
            properties,
            states,
            memory: MemorySource::Swapchain,
        })
    }

    #[inline]
    pub fn is_swapchain(&self) -> bool {
        matches!(self.memory, MemorySource::Swapchain)
    }

    fn get_states_mut(
        &mut self,
        aspect: ImageAspects,
        level: u32,
    ) -> Option<&mut NonNullVec32<'static, ImageLayerRange>>
    {
        let aspect_mask = self.properties.aspect_mask.as_raw();
        let aspect = aspect.as_raw();
        if aspect.count_ones() != 1 || aspect_mask & aspect != aspect {
            return None
        }
        let mut cindex = 0u32;
        let mut index = 0u32;
        macro_rules! index_op {
            ($($n:expr),* $(,)?) => {
                $(
                    cindex += ((1 << $n) & aspect_mask) >> $n;
                    index |= (((1 << $n) & aspect) >> $n).wrapping_neg() & cindex;
                )*
            };
        }
        index_op!(
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            /* 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, <- not needed */
        );
        index -= 1;
        let index = index * self.properties.mip_levels + level;
        Some(&mut self.states[index as usize])
    }

    fn get_states(
        &self,
        aspect: ImageAspects,
        level: u32,
    ) -> Option<&NonNullVec32<'static, ImageLayerRange>>
    {
        let aspect_mask = self.properties.aspect_mask.as_raw();
        let aspect = aspect.as_raw();
        if aspect.count_ones() != 1 || aspect_mask & aspect != aspect {
            return None
        }
        let mut cindex = 0u32;
        let mut index = 0u32;
        macro_rules! index_op {
            ($($n:expr),* $(,)?) => {
                $(
                    cindex += ((1 << $n) & aspect_mask) >> $n;
                    index |= (((1 << $n) & aspect) >> $n).wrapping_neg() & cindex;
                )*
            };
        }
        index_op!(
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            /* 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, <- not needed */
        );
        index -= 1;
        let index = index * self.properties.mip_levels + level;
        Some(&self.states[index as usize])
    }

    pub fn view_state<AnyImageId>(
        &self,
        id: AnyImageViewId<AnyImageId>,
        aspect: ImageAspects,
    ) -> Result<ImageSubresourceState>
        where AnyImageId: ResourceId<ImageMeta>
    {
        let range = self.image_views
            .get(id.view_id() as usize)
            .ok_or_else(|| Error::just_context(format!(
                "invalid image view id {id}"
            )))?.subresource_range;
        let states = self.get_states(aspect, range.base_array_layer)
            .ok_or_else(|| Error::just_context("invalid aspect mask"))?;
        Ok(states.iter().find_map(|layer_range| {
            (layer_range.base_array_layer <= range.base_array_layer && 
                layer_range.base_array_layer + layer_range.layer_count > range.base_array_layer
            ).then_some(layer_range.state)
        }).unwrap())
    }

    #[inline]
    pub fn handle(&self) -> vk::Image {
        self.handle
    }

    #[inline]
    pub fn properties(&self) -> ImageProperties {
        self.properties
    }

    pub fn validate_usage(
        &self,
        usage: ImageUsages,
    ) -> Option<MissingFlagsError<ImageUsages>> {
        let has = ImageUsages::from_raw(self.properties.usage.as_raw());
        (has & usage != usage)
        .then(|| MissingFlagsError::new(usage, has))
    }

    pub(crate) fn create_view(
        &mut self,
        range: ImageRange,
    ) -> Result<u32> {
        let component_info = 
            if let Some(info) = range.component_info {
                info
            } else {
                ComponentInfo {
                    component_mapping: ComponentMapping::default(),
                    format: self.properties.format,
                }
            };
        let view_type = self.properties.validate_range(&range)?;
        let create_info = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
            image: self.handle,
            view_type,
            format: component_info.format.into(),
            components: component_info.component_mapping.into(),
            subresource_range: range.subresource_range.into(),
            ..Default::default()
        };
        let handle = unsafe {
            self.device.create_image_view(&create_info, None)
            .context("failed to create image view")?
        }.value;
        let id = self.image_views.len();
        self.image_views.push(ImageView {
            handle,
            subresource_range: range.subresource_range,
            component_info,
            is_cube_map: range.is_cube_map,
        });
        Ok(id)
    } 
    
    #[inline]
    pub(crate) fn get_view<AnyImageId>(
        &self,
        id: AnyImageViewId<AnyImageId>,
    ) -> Result<&ImageView>
        where AnyImageId: ResourceId<ImageMeta>
    {
        self.image_views
            .get(id.view_id() as usize)
            .ok_or_else(|| Error::just_context(format!(
                "invalid image view id {id}"
            )))
    }

    #[inline]
    pub(crate) fn view_index_iter(
        &self,
        image_id: ImageId,
    ) -> impl Iterator<Item = ImageViewId> {
        (0..self.image_views.len()).map(move |index| {
            ImageViewId::new(image_id, index)
        })
    }

    pub fn flush_subresources(&mut self) {
        for states in &mut self.states {
            for range in states {
                range.state.stage_mask = vk::PipelineStageFlags2::ALL_COMMANDS;
                range.state.access_mask = vk::AccessFlags2::MEMORY_WRITE;
            }
        }
    }

    /// Registers a memory barrier, which *can* be used to perform [`pipeline barrier`][1] with the
    /// [`cache`][2].
    ///
    /// The returned [`range`][3] *must* be [`flushed`][4] and recorded, if the range is not empty.
    ///
    /// # Safety
    /// This does *not* the validity of the the [`subresource range`][5].
    ///
    /// The range *must* be either be checked manually or the [`checked`][6] or [`image view`][7]
    /// version of this function *must* be used.
    ///
    /// [1]: Device::cmd_pipeline_barrier2
    /// [2]: ImageMemoryBarrierCache
    /// [3]: ImageMemoryBarrierRange
    /// [4]: ImageMemoryBarrierCache::flush
    /// [5]: ImageSubresourceRange
    /// [6]: Self::memory_barrier
    /// [7]: Self::view_memory_barrier
    pub unsafe fn memory_barrier_unchecked(
        &mut self,
        state: ImageSubresourceState,
        subresource_range: ImageSubresourceRange,
        preserve_contents: bool,
        cache: &mut ImageMemoryBarrierCache,
    ) -> ImageMemoryBarrierRange
    {
        let effective_range = subresource_range.effective(
            self.properties.mip_levels,
            self.properties.array_layers
        );
        let mut layer_range = ImageLayerRange {
            state,
            base_array_layer: effective_range.base_array_layer,
            layer_count: effective_range.layer_count,
        };
        let cache_index = cache.barriers.len();
        let level_count = effective_range.level_count;
        let level_start = effective_range.base_mip_level;
        let level_end = effective_range.base_mip_level + level_count;
        for aspect in subresource_range.aspect_mask.as_raw()
            .bit_iter()
            .map(ImageAspects::from_raw)
        {
            for level in level_start..level_end {
                let mut not_inserted = None;
                let ranges = self.get_states_mut(aspect, level).unwrap();
                for i in (0..ranges.len()).rev() {
                    match unsafe { ranges.get_unchecked(i as usize).overwrite(&layer_range) } {
                        StateOverwrite::NoOverlap => continue,
                        StateOverwrite::Combine(new_range) => {
                            ranges.remove(i);
                            layer_range = new_range;
                            not_inserted = Some(i);
                        },
                        StateOverwrite::Consume(mut barrier) => {
                            ranges.remove(i);
                            barrier.subresource_range.aspect_mask = aspect.into();
                            barrier.subresource_range.base_mip_level = level;
                            if !preserve_contents {
                                barrier.old_layout = vk::ImageLayout::UNDEFINED;
                            }
                            cache.insert(aspect, barrier);
                            not_inserted = Some(i);
                        },
                        StateOverwrite::Cut(left, right, mut barrier) => {
                            ranges.remove(i);
                            barrier.subresource_range.aspect_mask = aspect.into();
                            barrier.subresource_range.base_mip_level = level;
                            if !preserve_contents {
                                barrier.old_layout = vk::ImageLayout::UNDEFINED;
                            }
                            cache.insert(aspect, barrier);
                            let mut idx = i;
                            if left.layer_count != 0 {
                                ranges.insert(idx, left);
                                idx += 1;
                            }
                            ranges.insert(idx, layer_range);
                            idx += 1;
                            if right.layer_count != 0 {
                                ranges.insert(idx, right);
                            }
                            not_inserted = None;
                            break
                        },
                        StateOverwrite::Shrink(new_range, mut barrier) => {
                            ranges[i as usize] = new_range;
                            barrier.subresource_range.aspect_mask = aspect.into();
                            barrier.subresource_range.base_mip_level = level;
                            if !preserve_contents {
                                barrier.old_layout = vk::ImageLayout::UNDEFINED;
                            }
                            cache.insert(aspect, barrier);
                            if new_range.base_array_layer < layer_range.base_array_layer {
                                ranges.insert(i + 1, layer_range);
                                not_inserted = None;
                                break
                            }
                        },
                    }
                }
                if let Some(i) = not_inserted {
                    ranges.insert(i, layer_range);
                }
            }
        }
        for key in &cache.touched {
            let barriers = cache.cache.get_mut(key).unwrap();
            for i in (0..barriers.len() as usize - 1).rev() {
                let next_idx = i + 1;
                let mut next = barriers[next_idx];
                let this = &mut barriers[i];
                if this.src_stage_mask == next.src_stage_mask &&
                    this.src_access_mask == next.src_access_mask &&
                    this.old_layout == next.old_layout &&
                    next.subresource_range.base_mip_level ==
                    this.subresource_range.level_count + 1
                {
                    next.subresource_range.base_mip_level -= 1;
                    next.subresource_range.level_count += 1;
                    *this = next;
                    barriers.remove(next_idx as u32);
                }
            }
            cache.barriers.append(barriers);
            barriers.clear();
        }
        cache.touched.clear();
        ImageMemoryBarrierRange {
            handle: self.handle,
            range_start: cache_index,
            range_end: cache.barriers.len()
        }
    }

    /// Registers a memory barrier, which *can* be used to perform [`pipeline barrier`][1] with the
    /// [`cache`][2].
    ///
    /// The returned [`range`][3] *must* be [`flushed`][4] and recorded, if the range is not empty.
    ///
    /// # Valid usage
    /// - `subresource_range` *must* be a valid [`subresource range`][5] for this image.
    ///
    /// [1]: Device::cmd_pipeline_barrier2
    /// [2]: ImageMemoryBarrierCache
    /// [3]: ImageMemoryBarrierRange
    /// [4]: ImageMemoryBarrierCache::flush
    /// [5]: ImageSubresourceRange
    #[inline(always)]
    pub fn memory_barrier(
        &mut self,
        state: ImageSubresourceState,
        subresource_range: ImageSubresourceRange,
        preserve_contents: bool,
        cache: &mut ImageMemoryBarrierCache,
    ) -> Result<ImageMemoryBarrierRange>
    {
        self.properties.validate_subresource_range(&subresource_range)?;
        unsafe {
            Ok(self.memory_barrier_unchecked(
                state,
                subresource_range,
                preserve_contents,
                cache
            ))
        }
    }

    /// Registers a memory barrier, which *can* be used to perform [`pipeline barrier`][1] with the
    /// [`cache`][2].
    ///
    /// The returned [`range`][3] *must* be [`flushed`][4] and recorded, if the range is not empty.
    ///
    /// [1]: Device::cmd_pipeline_barrier2
    /// [2]: ImageMemoryBarrierCache
    /// [3]: ImageMemoryBarrierRange
    /// [4]: ImageMemoryBarrierCache::flush
    #[inline(always)]
    pub fn view_memory_barrier<AnyImageId>(
        &mut self,
        state: ImageSubresourceState,
        view_id: AnyImageViewId<AnyImageId>,
        preserve_contents: bool,
        cache: &mut ImageMemoryBarrierCache,
    ) -> Result<ImageMemoryBarrierRange>
        where AnyImageId: ResourceId<ImageMeta>
    {
        let view = self.image_views
            .get(view_id.view_id() as usize)
            .ok_or_else(|| Error::just_context(format!(
                "invalid image view id {view_id}"
            )))?;
        unsafe {
            Ok(self.memory_barrier_unchecked(
                state,
                view.subresource_range,
                preserve_contents,
                cache
            ))
        }
    }
}

impl Drop for ImageMeta {

    fn drop(&mut self) {
        unsafe {
            for &view in &self.image_views {
                self.device.destroy_image_view(view.handle, None);
            }
            if let MemorySource::Joint { _device_memory, arena_size, } = &self.memory {
                self.device.destroy_image(self.handle(), None);
                StdAlloc.free_raw(
                    NonNull::new_unchecked(self.states.as_mut_ptr()).cast(),
                    Layout::from_size_align_unchecked(
                        *arena_size,
                        arena::max_align()
                    ),
                );
            }
        }
    }
}

pub(crate) type ImageIndex = SlotIndex<ImageMeta>;

mod image_id_base {

    use super::*;

    #[must_use]
    #[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Id<Marker>(SlotIndex<ImageMeta>, PhantomData<Marker>)
        where Marker:  Copy;

    impl<Marker> Id<Marker>
        where Marker: Copy
    {

        pub(crate) fn new(slot_index: SlotIndex<ImageMeta>) -> Self {
            Self(slot_index, PhantomData)
        }
    }

    impl<Marker: Copy> Display for Id<Marker> {

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl<Marker> ResourceId<ImageMeta> for Id<Marker>
        where Marker: Copy
    {

        #[inline]
        fn slot_index(self) -> SlotIndex<ImageMeta> {
            self.0
        }
    }
}

pub type ImageId = image_id_base::Id<()>;
pub type SwapchainImageId<'a> = image_id_base::Id<&'a ()>;

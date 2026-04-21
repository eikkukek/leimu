use core::{
    num::NonZeroU32,
    ops::Add,
    fmt::{self, Display},
};

use tuhka::vk;
use leimu_proc::BuildStructure;
use leimu_mem::int::NonZeroOption;

use super::{
    ext::MissingDeviceFeatureError,
    *
};

pub type DeviceSize = u64;

/// Sets which base device features to enable.
///
/// By default [`sample_rate_shading`][1], [`sampler_anisotropy`][2] and [`alpha_to_one`][3] are enabled.
///
/// [1]: BaseDeviceFeatures::sample_rate_shading
/// [2]: BaseDeviceFeatures::sampler_anisotropy
/// [3]: BaseDeviceFeatures::alpha_to_one
///
/// You can find the descriptions for each feature here:
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFeatures.html>
#[derive(Clone, Copy, BuildStructure)]
pub struct BaseDeviceFeatures {
    pub robust_buffer_access: bool,
    pub full_draw_index_uint32: bool,
    pub image_cube_array: bool,
    pub independent_blend: bool,
    pub geometry_shader: bool,
    pub tessellation_shader: bool,
    #[default(true)]
    pub sample_rate_shading: bool,
    pub dual_src_blend: bool,
    pub logic_op: bool,
    pub multi_draw_indirect: bool,
    pub draw_indirect_first_instance: bool,
    pub depth_clamp: bool,
    pub depth_bias_clamp: bool,
    pub fill_mode_non_solid: bool,
    pub depth_bounds: bool,
    pub wide_lines: bool,
    pub large_points: bool,
    #[default(true)]
    pub alpha_to_one: bool,
    pub multi_viewport: bool,
    #[default(true)]
    pub sampler_anisotropy: bool,
    pub texture_compression_etc2: bool,
    pub texture_compression_astc_ldr: bool,
    pub texture_compression_bc: bool,
    pub occlusion_query_precise: bool,
    pub pipeline_statistics_query: bool,
    pub vertex_pipeline_stores_and_atomics: bool,
    pub fragment_stores_and_atomics: bool,
    pub shader_tessellation_and_geometry_point_size: bool,
    pub shader_image_gather_extended: bool,
    pub shader_storage_image_extended_formats: bool,
    pub shader_storage_image_multisample: bool,
    pub shader_storage_image_read_without_format: bool,
    pub shader_storage_image_write_without_format: bool,
    pub shader_uniform_buffer_array_dynamic_indexing: bool,
    pub shader_sampled_image_array_dynamic_indexing: bool,
    pub shader_storage_buffer_array_dynamic_indexing: bool,
    pub shader_storage_image_array_dynamic_indexing: bool,
    pub shader_clip_distance: bool,
    pub shader_cull_distance: bool,
    pub shader_float64: bool,
    pub shader_int64: bool,
    pub shader_int16: bool,
    pub shader_resource_residency: bool,
    pub shader_resource_min_lod: bool,
    pub sparse_binding: bool,
    pub sparse_residency_buffer: bool,
    pub sparse_residency_image_2d: bool,
    pub sparse_residency_image_3d: bool,
    pub sparse_residency_2_samples: bool,
    pub sparse_residency_4_samples: bool,
    pub sparse_residency_8_samples: bool,
    pub sparse_residency_16_samples: bool,
    pub sparse_residency_aliased: bool,
    pub variable_multisample_rate: bool,
    pub inherited_queries: bool,
}

impl BaseDeviceFeatures {

    pub(crate) fn find_missing_features(
        self,
        available: &vk::PhysicalDeviceFeatures,
    ) -> Option<MissingDeviceFeatureError>
    {
        macro_rules! check {
            ($($field:ident),+ $(,)?) => {
                $(
                    if self.$field && (available.$field == 0) {
                        return Some(MissingDeviceFeatureError::new(stringify!($field)))
                    }
                )+
            };
        }
        check!(
            robust_buffer_access,
            full_draw_index_uint32,
            image_cube_array,
            independent_blend,
            geometry_shader,
            tessellation_shader,
            sample_rate_shading,
            dual_src_blend,
            logic_op,
            multi_draw_indirect,
            draw_indirect_first_instance,
            depth_clamp,
            depth_bias_clamp,
            fill_mode_non_solid,
            depth_bounds,
            wide_lines,
            large_points,
            alpha_to_one,
            multi_viewport,
            sampler_anisotropy,
            texture_compression_etc2,
            texture_compression_astc_ldr,
            texture_compression_bc,
            occlusion_query_precise,
            pipeline_statistics_query,
            vertex_pipeline_stores_and_atomics,
            fragment_stores_and_atomics,
            shader_tessellation_and_geometry_point_size,
            shader_image_gather_extended,
            shader_storage_image_extended_formats,
            shader_storage_image_multisample,
            shader_storage_image_read_without_format,
            shader_storage_image_write_without_format,
            shader_uniform_buffer_array_dynamic_indexing,
            shader_sampled_image_array_dynamic_indexing,
            shader_storage_buffer_array_dynamic_indexing,
            shader_storage_image_array_dynamic_indexing,
            shader_clip_distance,
            shader_cull_distance,
            shader_float64,
            shader_int64,
            shader_int16,
            shader_resource_residency,
            shader_resource_min_lod,
            sparse_binding,
            sparse_residency_buffer,
            sparse_residency_aliased,
            variable_multisample_rate,
            inherited_queries,
        );
        if self.sparse_residency_image_2d && (available.sparse_residency_image2_d == 0) {
            return Some(MissingDeviceFeatureError::new("sparse_residency_image_2d"))
        }
        if self.sparse_residency_image_3d && (available.sparse_residency_image2_d == 0) {
            return Some(MissingDeviceFeatureError::new("sparse_residency_image_3d"))
        }
        if self.sparse_residency_2_samples && (available.sparse_residency2_samples == 0) {
            return Some(MissingDeviceFeatureError::new("sparse_residency_2_samples"))
        }
        if self.sparse_residency_4_samples && (available.sparse_residency4_samples == 0) {
            return Some(MissingDeviceFeatureError::new("sparse_residency_4_samples"))
        }
        if self.sparse_residency_8_samples && (available.sparse_residency8_samples == 0) {
            return Some(MissingDeviceFeatureError::new("sparse_residency_8_samples"))
        }
        if self.sparse_residency_16_samples && (available.sparse_residency16_samples == 0) {
            return Some(MissingDeviceFeatureError::new("sparse_residency_16_samples"))
        }
        None
    }
}

impl From<BaseDeviceFeatures> for vk::PhysicalDeviceFeatures {

    fn from(value: BaseDeviceFeatures) -> Self {
        Self {
            robust_buffer_access: value.robust_buffer_access as u32,
            full_draw_index_uint32: value.full_draw_index_uint32 as u32,
            image_cube_array: value.image_cube_array as u32,
            independent_blend: value.independent_blend as u32,
            geometry_shader: value.geometry_shader as u32,
            tessellation_shader: value.tessellation_shader as u32,
            sample_rate_shading: value.sample_rate_shading as u32,
            dual_src_blend: value.dual_src_blend as u32,
            logic_op: value.logic_op as u32,
            multi_draw_indirect: value.multi_draw_indirect as u32,
            draw_indirect_first_instance: value.draw_indirect_first_instance as u32,
            depth_clamp: value.depth_clamp as u32,
            depth_bias_clamp: value.depth_bias_clamp as u32,
            fill_mode_non_solid: value.fill_mode_non_solid as u32,
            depth_bounds: value.depth_bounds as u32,
            wide_lines: value.wide_lines as u32,
            large_points: value.large_points as u32,
            alpha_to_one: value.alpha_to_one as u32,
            multi_viewport: value.multi_viewport as u32,
            sampler_anisotropy: value.sampler_anisotropy as u32,
            texture_compression_etc2: value.texture_compression_etc2 as u32,
            texture_compression_astc_ldr: value.texture_compression_astc_ldr as u32,
            texture_compression_bc: value.texture_compression_bc as u32,
            occlusion_query_precise: value.occlusion_query_precise as u32,
            pipeline_statistics_query: value.pipeline_statistics_query as u32,
            vertex_pipeline_stores_and_atomics: value.vertex_pipeline_stores_and_atomics as u32,
            fragment_stores_and_atomics: value.fragment_stores_and_atomics as u32,
            shader_tessellation_and_geometry_point_size: value.shader_tessellation_and_geometry_point_size as u32,
            shader_image_gather_extended: value.shader_image_gather_extended as u32,
            shader_storage_image_extended_formats: value.shader_storage_image_extended_formats as u32,
            shader_storage_image_multisample: value.shader_storage_image_multisample as u32,
            shader_storage_image_read_without_format: value.shader_storage_image_read_without_format as u32,
            shader_storage_image_write_without_format: value.shader_storage_image_write_without_format as u32,
            shader_uniform_buffer_array_dynamic_indexing: value.shader_uniform_buffer_array_dynamic_indexing as u32,
            shader_sampled_image_array_dynamic_indexing: value.shader_sampled_image_array_dynamic_indexing as u32,
            shader_storage_buffer_array_dynamic_indexing: value.shader_storage_buffer_array_dynamic_indexing as u32,
            shader_storage_image_array_dynamic_indexing: value.shader_storage_image_array_dynamic_indexing as u32,
            shader_clip_distance: value.shader_clip_distance as u32,
            shader_cull_distance: value.shader_cull_distance as u32,
            shader_float64: value.shader_float64 as u32,
            shader_int64: value.shader_int64 as u32,
            shader_int16: value.shader_int16 as u32,
            sparse_binding: value.sparse_binding as u32,
            shader_resource_residency: value.shader_resource_residency as u32,
            shader_resource_min_lod: value.shader_resource_min_lod as u32,
            sparse_residency_buffer: value.sparse_residency_buffer as u32,
            sparse_residency_image2_d: value.sparse_residency_image_2d as u32,
            sparse_residency_image3_d: value.sparse_residency_image_3d as u32,
            sparse_residency2_samples: value.sparse_residency_2_samples as u32,
            sparse_residency4_samples: value.sparse_residency_4_samples as u32,
            sparse_residency8_samples: value.sparse_residency_8_samples as u32,
            sparse_residency16_samples: value.sparse_residency_16_samples as u32,
            sparse_residency_aliased: value.sparse_residency_aliased as u32,
            variable_multisample_rate: value.variable_multisample_rate as u32,
            inherited_queries: value.inherited_queries as u32,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, BuildStructure)]
pub struct Offset2D {
    pub x: i32,
    pub y: i32
}

impl Offset2D {

    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl From<Offset2D> for vk::Offset2D {

    #[inline]
    fn from(value: Offset2D) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl Display for Offset2D {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Offset3D {

    #[inline]
    pub fn new(x: i32, y: i32, z: i32) -> Self
    {
        Self {x, y, z}
    }
}

impl From<Offset3D> for vk::Offset3D {

    #[inline]
    fn from(value: Offset3D) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl Display for Offset3D {
   
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// Used for image dimensions and extents.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug, BuildStructure)]
pub struct Dimensions {
    /// The width of an image region.
    pub width: u32,
    /// The height of an image region.
    pub height: u32,
    /// The depth of an image region.
    pub depth: u32,
}

impl Dimensions {

    #[inline]
    pub const fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.width == 0 ||
        self.height == 0 ||
        self.depth == 0
    }

    #[inline]
    pub const fn texel_count(&self) -> DeviceSize {
        self.width as DeviceSize *
        self.height as DeviceSize *
        self.depth as DeviceSize
    }

    #[inline]
    pub const fn is_multiple_of(&self, other: Self) -> bool {
        self.width.is_multiple_of(other.width) &&
        self.height.is_multiple_of(other.height) &&
        self.depth.is_multiple_of(other.depth)
    }

    #[inline]
    pub fn map<F>(self, mut f: F) -> Self
        where F: FnMut(u32) -> u32
    {
        Self {
            width: f(self.width),
            height: f(self.height),
            depth: f(self.depth),
        }
    }

    /// Gets the extent of an image with these dimensions at `mip_level`.
    #[must_use]
    #[inline]
    pub fn lod(self, mip_level: u32) -> Self {
        self.map(|x| (x >> mip_level).max(1))
    }

    #[must_use]
    #[inline]
    pub fn into_offset(self) -> ImageCopyOffset {
        ImageCopyOffset {
            x: self.width,
            y: self.height,
            z: self.depth,
        }
    }
}

impl Display for Dimensions {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})",
            self.width,
            self.height,
            self.depth,
        )
    }
}

impl Add<ImageCopyOffset> for Dimensions {

    type Output = ImageCopyOffset;

    #[inline]
    fn add(self, rhs: ImageCopyOffset) -> Self::Output {
        ImageCopyOffset {
            x: self.width + rhs.x,
            y: self.height + rhs.y,
            z: self.depth + rhs.z,
        }
    }
}

impl Add<Dimensions> for ImageCopyOffset {

    type Output = Self;

    #[inline]
    fn add(self, rhs: Dimensions) -> Self::Output {
        Self {
            x: self.x + rhs.width,
            y: self.y + rhs.height,
            z: self.z + rhs.depth,
        }
    }
}

impl From<Dimensions> for vk::Extent3D {

    #[inline]
    fn from(value: Dimensions) -> Self {
        Self {
            width: value.width,
            height: value.height,
            depth: value.depth,
        }
    }
}

impl From<vk::Extent3D> for Dimensions {

    #[inline]
    fn from(value: vk::Extent3D) -> Self {
        Self::new(
            value.width,
            value.height,
            value.depth
        )
    }
}

impl From<vk::Extent2D> for Dimensions {

    #[inline]
    fn from(value: vk::Extent2D) -> Self {
        Self::new(
            value.width,
            value.height,
            1,
        )
    }
}

impl From<(u32, u32)> for Dimensions {
    
    #[inline]
    fn from(value: (u32, u32)) -> Self {
        Self {
            width: value.0,
            height: value.1,
            depth: 1,
        }
    }
}

impl From<[u32; 2]> for Dimensions {
    
    #[inline]
    fn from(value: [u32; 2]) -> Self {
        Self {
            width: value[0],
            height: value[1],
            depth: 1,
        }
    }
}

impl From<(u32, u32, u32)> for Dimensions {

    #[inline]
    fn from(value: (u32, u32, u32)) -> Self {
        Self {
            width: value.0,
            height: value.1,
            depth: value.2,
        }
    }
}

impl From<[u32; 3]> for Dimensions {

    #[inline]
    fn from(value: [u32; 3]) -> Self {
        Self {
            width: value[0],
            height: value[1],
            depth: value[2],
        }
    }
}

/// Specifies how colors are mapped.
#[derive(Default, Clone, Copy, PartialEq, Eq, BuildStructure)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}

impl From<ComponentMapping> for vk::ComponentMapping {
    
    #[inline]
    fn from(value: ComponentMapping) -> Self {
        Self {
            r: value.r.into(),
            g: value.g.into(),
            b: value.b.into(),
            a: value.a.into(),
        }
    }
}

/// # Vulkan docs
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageSubresourceRange.html>
#[derive(Clone, Copy, PartialEq, Eq, BuildStructure)]
pub struct ImageSubresourceRange {
    /// Specifies the [aspect mask][1] of the range.
    ///
    /// [1]: ImageAspects
    pub aspect_mask: ImageAspects,
    /// Specifies the first mipmap level of the range.
    pub base_mip_level: u32,
    /// Specifies the number of mipmap levels in the range.
    ///
    /// Set this to [`None`] to to specify all remaining levels from [`base_mip_level`][1].
    ///
    /// [1]: ImageSubresourceRange::base_mip_level
    #[skip]
    #[default(None)]
    pub level_count: Option<NonZeroU32>,
    /// Specifies the first array layer of the range.
    pub base_array_layer: u32,
    /// Specifies the number of array layers in the range.
    ///
    /// Set this to [`None`] to specify all remaining layers from [`base_array_layer`][1].
    ///
    /// [1]: ImageSubresourceRange::base_array_layer
    #[skip]
    #[default(None)]
    pub layer_count: Option<NonZeroU32>,
}

impl ImageSubresourceRange {

    /// Specifies the number of mipmap levels in the range.
    ///
    /// Set this to zero to to specify all remaining levels from [`base_mip_level`][1].
    ///
    /// [1]: ImageSubresourceRange::base_mip_level
    #[inline]
    pub fn level_count(mut self, level_count: u32) -> Self {
        self.level_count = NonZeroU32::new(level_count);
        self
    }

    /// Specifies the number of array layers in the range.
    ///
    /// Set this to zero to specify all remaining layers from [`base_array_layer`][1].
    ///
    /// [1]: ImageSubresourceRange::base_array_layer
    #[inline]
    pub fn layer_count(mut self, layer_count: u32) -> Self {
        self.layer_count = NonZeroU32::new(layer_count);
        self
    }

    /// Checks whether two subresource ranges overlap.
    #[inline]
    pub fn overlaps(self, other: Self) -> bool {
        if self.aspect_mask.intersects(other.aspect_mask) {
            let level_intersects =
                if self.level_count.is_none() && other.level_count.is_none() {
                    return true
                } else if let Some(a) = self.level_count &&
                    let Some(b) = other.level_count
                {
                    self.base_mip_level < other.base_mip_level + b.get() &&
                    other.base_mip_level < self.base_mip_level + a.get()
                } else if let Some(a) = self.level_count {
                     other.base_mip_level < self.base_mip_level + a.get()
                } else if let Some(b) = other.level_count {
                    self.base_mip_level < other.base_mip_level + b.get()
                } else {
                    false
                };
            if level_intersects {
                return true
            }
            if self.layer_count.is_none() && other.layer_count.is_none() {
                true
            } else if let Some(a) = self.layer_count &&
                let Some(b) = other.layer_count
            {
                self.base_array_layer < other.base_array_layer + b.get() &&
                other.base_array_layer < self.base_array_layer + a.get()
            } else if let Some(a) = self.layer_count {
                 other.base_array_layer < self.base_array_layer + a.get()
            } else if let Some(b) = other.layer_count {
                self.base_array_layer < other.base_array_layer + b.get()
            } else {
                false
            }
        } else {
            false
        }
    }

    #[inline]
    pub(crate) fn effective(
        self,
        image_level_count: u32,
        image_layer_count: u32,
    ) -> vk::ImageSubresourceRange {
        vk::ImageSubresourceRange {
            aspect_mask: self.aspect_mask.into(),
            base_mip_level: self.base_mip_level,
            level_count: self.level_count
                .unwrap_or_sentinel_with(||
                    image_level_count.saturating_sub(self.base_mip_level)
                ),
            base_array_layer: self.base_array_layer,
            layer_count: self.layer_count
                .unwrap_or_sentinel_with(|| {
                    image_layer_count.saturating_sub(self.base_array_layer)
                }),
        }
    }
}

impl From<ImageSubresourceRange> for vk::ImageSubresourceRange {

    #[inline]
    fn from(value: ImageSubresourceRange) -> Self {
        Self {
            aspect_mask: value.aspect_mask.into(),
            base_mip_level: value.base_mip_level,
            level_count: value.level_count.unwrap_or_sentinel(vk::REMAINING_MIP_LEVELS),
            base_array_layer: value.base_array_layer,
            layer_count: value.layer_count.unwrap_or_sentinel(vk::REMAINING_ARRAY_LAYERS),
        }
    }
}

impl From<vk::ImageSubresourceRange> for ImageSubresourceRange {

    #[inline]
    fn from(value: vk::ImageSubresourceRange) -> Self {
        Self {
            aspect_mask: ImageAspects::from_raw(value.aspect_mask.as_raw()),
            base_mip_level: value.base_mip_level,
            level_count: if value.level_count != vk::REMAINING_MIP_LEVELS {
                NonZeroU32::new(value.level_count)
            } else { None },
            base_array_layer: value.base_array_layer,
            layer_count: if value.layer_count != vk::REMAINING_ARRAY_LAYERS {
                NonZeroU32::new(value.layer_count)
            } else { None },
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, BuildStructure)]
pub struct ImageSubresourceLayers {
    /// Specifies the [`aspects`][1] to be copied.
    ///
    /// [1]: ImageAspects
    pub aspect_mask: ImageAspects,
    /// Specifies the mipmap level to copy.
    pub mip_level: u32,
    /// Specifies the starting array layer to copy.
    pub base_array_layer: u32,
    /// Specifies the number of layers to copy.
    ///
    /// Set this to [`None`] to to copy all remaining layers from [`base_array_layer`][1].
    ///
    /// [1]: ImageSubresourceLayers::base_array_layer
    #[skip]
    pub layer_count: Option<NonZeroU32>,
}

impl ImageSubresourceLayers { 

    /// Specifies the number of layers to copy.
    ///
    /// Set this to zero to copy all remaining layers from [`base_array_layer`][1].
    ///
    /// [1]: ImageSubresourceLayers::base_array_layer
    #[inline]
    pub fn layer_count(mut self, layer_count: u32) -> Self {
        self.layer_count = NonZeroU32::new(layer_count);
        self
    }

    #[inline]
    pub fn overlaps(self, other: Self) -> bool {
        if !self.aspect_mask.intersects(other.aspect_mask) || self.mip_level != other.mip_level {
            false
        } else if self.layer_count.is_none() && other.layer_count.is_none() {
            true
        } else if let Some(a) = self.layer_count &&
            let Some(b) = other.layer_count
        {
            self.base_array_layer < other.base_array_layer + b.get() &&
            other.base_array_layer < self.base_array_layer + a.get()
        } else if let Some(a) = self.layer_count {
            other.base_array_layer < self.base_array_layer + a.get()
        } else if let Some(b) = other.layer_count {
            self.base_array_layer < other.base_array_layer + b.get()
        } else {
            false
        }
    }

    #[inline]
    pub fn into_range(self) -> ImageSubresourceRange {
        ImageSubresourceRange {
            aspect_mask: self.aspect_mask,
            base_mip_level: self.mip_level,
            level_count: NonZeroU32::new(1),
            base_array_layer: self.base_array_layer,
            layer_count: self.layer_count,
        }
    }

    #[inline]
    pub fn effective(self, image_layer_count: u32) -> vk::ImageSubresourceLayers {
        vk::ImageSubresourceLayers {
            aspect_mask: self.aspect_mask.into(),
            mip_level: self.mip_level,
            base_array_layer: self.base_array_layer,
            layer_count: self.layer_count.unwrap_or_sentinel_with(|| {
                image_layer_count - self.base_array_layer
            }),
        }
    }
    
}

impl From<ImageSubresourceLayers> for vk::ImageSubresourceLayers {

    fn from(value: ImageSubresourceLayers) -> Self {
        Self {
            aspect_mask: value.aspect_mask.into(),
            mip_level: value.mip_level,
            base_array_layer: value.base_array_layer,
            layer_count: value.layer_count.unwrap_or_sentinel(vk::REMAINING_ARRAY_LAYERS),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, BuildStructure)]
pub struct ComponentInfo {
    pub component_mapping: ComponentMapping,
    pub format: Format,
}

impl ComponentInfo {

    pub fn new(
        component_mapping: ComponentMapping,
        format: Format,
    ) -> Self
    {
        Self {
            component_mapping,
            format,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, BuildStructure)]
pub struct ImageRange {
    pub subresource_range: ImageSubresourceRange,
    pub component_info: Option<ComponentInfo>,
    pub is_cube_map: bool,
}

impl ImageRange {

    #[inline]
    pub fn whole_range(aspect: ImageAspects) -> Self {
        Self {
            subresource_range: ImageSubresourceRange::default().aspect_mask(aspect),
            component_info: None,
            is_cube_map: false,
        }
    }
}

/// Specifies a viewport.
///
/// # Valid usage
/// - [`width`][Self::width] *must* be greater than 0.0
/// - [`min_depth`][1] *must* be inclusively between 0.0 and 1.0, if the VK_EXT_depth_range_unrestricted
///   extension is not enabled.
/// - [`max_depth`][2] *must* be inclusively between 0.0 and 1.0, if the VK_EXT_depth_range_unrestricted
///   extension is not enabled.
/// 
/// # Vulkan docs
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkViewport.html>
///
/// [1]: Self::min_depth
/// [2]: Self::max_depth
#[repr(C)]
#[derive(Clone, Copy, BuildStructure)]
pub struct Viewport {
    /// Specifies the x-coordinate of the viewport's upper left corner.
    pub x: f32,
    /// Specifies the y-coordinate of the viewport's upper left corner.
    pub y: f32,
    /// Specifies the width of the viewport.
    pub width: f32,
    /// Specifies the height of the viewport.
    pub height: f32,
    /// Specifies the minimum of the viewport's depth range.
    ///
    /// The default value is 0.0.
    #[default(0.0)]
    pub min_depth: f32,
    /// Specifies the maximum of the viewport's depth range.
    ///
    /// The default value is 1.0.
    #[default(1.0)]
    pub max_depth: f32,
}

impl From<Viewport> for vk::Viewport {

    #[inline]
    fn from(value: Viewport) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
            min_depth: value.min_depth,
            max_depth: value.max_depth,
        }
    }
}

/// Specifies a scissor.
///
/// This is used instead of `VkRect2D`, to enforce that x >= 0 and y >= 0.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, BuildStructure)]
pub struct Scissor {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct FormatResolveModes {
    pub color: ResolveModes,
    pub depth: ResolveModes,
    pub stencil: ResolveModes,
}

impl FormatResolveModes {

    #[inline]
    pub fn by_aspect(self, aspect: ResolveAspect) -> ResolveModes {
        match aspect {
            ResolveAspect::Color => self.color,
            ResolveAspect::Depth => self.depth,
            ResolveAspect::Stencil => self.stencil,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ImageFormatProperties {
    pub max_dimensions: Dimensions,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: MsaaSamples,
    pub format_features: FormatFeatures,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, BuildStructure)]
pub struct ImageCopyOffset {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Display for ImageCopyOffset {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ImageCopyOffset {

    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self {
            x, y, z,
        }
    }

    #[inline]
    pub fn is_multiple_of(self, extent: Dimensions) -> bool {
        self.x.is_multiple_of(extent.width) &&
        self.y.is_multiple_of(extent.height) &&
        self.z.is_multiple_of(extent.depth)
    }

    #[inline]
    pub fn is_in_range(
        self,
        image_dimensions: Dimensions,
        copy_extent: Dimensions,
    ) -> bool {
        let off = self + copy_extent;
        off.x <= image_dimensions.width &&
        off.y <= image_dimensions.height &&
        off.z <= image_dimensions.depth
    }

    #[inline]
    pub fn is_zero(self) -> bool {
        self.x == 0 ||
        self.y == 0 ||
        self.z == 0
    }
}

impl From<ImageCopyOffset> for vk::Offset3D {

    #[inline]
    fn from(value: ImageCopyOffset) -> Self {
        Self {
            x: value.x as i32,
            y: value.y as i32,
            z: value.z as i32,
        }
    }
}

pub type ImageBlitOffset = ImageCopyOffset;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ImageBlitRegion {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offsets: [ImageBlitOffset; 2],
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offsets: [ImageBlitOffset; 2],
}

/// Controls the robustness of pipelines.
///
/// # Vulkan docs
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessCreateInfo.html>
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PipelineRobustnessInfo {
    pub storage_buffer_behavior: PipelineRobustnessBufferBehavior,
    pub uniform_buffer_behavior: PipelineRobustnessBufferBehavior,
    pub vertex_input_behavior: PipelineRobustnessBufferBehavior,
    pub image_behavior: PipelineRobustnessImageBehavior,
}

impl From<PipelineRobustnessInfo> for vk::PipelineRobustnessCreateInfo<'_> {

    #[inline]
    fn from(value: PipelineRobustnessInfo) -> Self {
        Self {
            storage_buffers: value.storage_buffer_behavior.into(),
            uniform_buffers: value.uniform_buffer_behavior.into(),
            vertex_inputs: value.vertex_input_behavior.into(),
            images: value.image_behavior.into(),
            ..Default::default()
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, BuildStructure)]
pub struct BufferCopy {
    pub src_offset: DeviceSize,
    pub dst_offset: DeviceSize,
    pub size: DeviceSize,
}

impl BufferCopy {
    
    #[inline]
    pub fn new(
        src_offset: DeviceSize,
        dst_offset: DeviceSize,
        size: DeviceSize,
    ) -> Self {
        Self {
            src_offset,
            dst_offset,
            size,
        }
    }
}

impl From<BufferCopy> for vk::BufferCopy2<'_> {

    #[inline]
    fn from(value: BufferCopy) -> Self {
        Self {
            src_offset: value.src_offset,
            dst_offset: value.dst_offset,
            size: value.size,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, BuildStructure)]
pub struct ImageCopy {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: ImageCopyOffset,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: ImageCopyOffset,
    pub extent: Dimensions,
}

impl From<ImageCopy> for vk::ImageCopy2<'_> {

    #[inline]
    fn from(value: ImageCopy) -> Self {
        Self {
            src_subresource: value.src_subresource.into(),
            src_offset: value.src_offset.into(),
            dst_subresource: value.dst_subresource.into(),
            dst_offset: value.dst_offset.into(),
            extent: value.extent.into(),
            ..Default::default()
        }
    }
}

#[derive(Default, Clone, Copy, BuildStructure)]
pub struct BufferImageCopy {
    pub buffer_offset: DeviceSize,
    #[skip]
    pub buffer_row_length: Option<NonZeroU32>,
    #[skip]
    pub buffer_image_height: Option<NonZeroU32>,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: ImageCopyOffset,
    #[skip]
    pub image_extent: Dimensions,
}

impl BufferImageCopy {

    /// This *must* either be zero or greater than or equal to [`image_extent`][1] width.
    ///
    /// [1]: BufferImageCopy::image_extent
    #[must_use]
    #[inline]
    pub fn buffer_row_length(mut self, buffer_row_length: u32) -> Self {
        self.buffer_row_length = NonZeroU32::new(buffer_row_length);
        self
    }

    /// This *must* either be zero or greater than or equal to [`image_extent`][1] height.
    /// 
    /// [1]: BufferImageCopy::image_extent
    #[must_use]
    #[inline]
    pub fn buffer_image_height(mut self, buffer_row_length: u32) -> Self {
        self.buffer_row_length = NonZeroU32::new(buffer_row_length);
        self
    }

    pub fn image_extent<Dim>(mut self, extent: Dim) -> Self
        where Dim: Into<Dimensions>
    {
        self.image_extent = extent.into();
        self
    }

    /// Calculates the minimum size a buffer needs to be for this copy.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/spec/latest/chapters/copies.html#copies-buffers-images-addressing>
    pub fn calculate_buffer_size(
        &self,
        compat: &FormatCompatibility,
        format: Format,
        aspect: ImageAspects,
        layer_count: u32,
    ) -> DeviceSize
    {
        let block_size =
            if let Some(plane) = aspect.plane() {
                format.plane_formats()[plane as usize]
                    .compatibility()
                    .texel_block_size()
            } else {
                compat.texel_block_size()
            };
        let block_extent: Dimensions = compat
            .texel_block_extent();
        let row_extent = self.buffer_row_length
            .unwrap_or_sentinel(self.image_extent.width)
            .div_ceil(block_extent.width) as DeviceSize * block_size;
        let slice_extent = self.buffer_image_height
            .unwrap_or_sentinel(self.image_extent.height)
            .div_ceil(block_extent.height) as DeviceSize * row_extent;
        let layer_extent = self.image_extent.depth
            .div_ceil(block_extent.depth) as DeviceSize * slice_extent;
        let (x, y, z, layer) = (
            self.image_extent.width - 1,
            self.image_extent.height - 1,
            self.image_extent.depth - 1,
            layer_count - 1,
        );
        (x / block_extent.width) as DeviceSize * block_size +
        (y / block_extent.height) as DeviceSize * row_extent +
        (z / block_extent.depth) as DeviceSize * slice_extent +
        layer as DeviceSize * layer_extent +
        block_size
    }
}

impl From<BufferImageCopy> for vk::BufferImageCopy2<'_> {

    #[inline]
    fn from(value: BufferImageCopy) -> Self {
        Self {
            buffer_offset: value.buffer_offset,
            buffer_row_length: value.buffer_row_length
                .unwrap_or_sentinel(0),
            buffer_image_height: value.buffer_image_height
                .unwrap_or_sentinel(0),
            image_subresource: value.image_subresource.into(),
            image_offset: value.image_offset.into(),
            image_extent: value.image_extent.into(),
            ..Default::default()
        }
    }
}

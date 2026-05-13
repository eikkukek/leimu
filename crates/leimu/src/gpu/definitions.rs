use core::{
    num::NonZeroU32,
    ops::Add,
    fmt::{self, Display},
};

use tuhka::vk;
use leimu_proc::BuildStructure;
use leimu_mem::int::NonZeroOption;

use crate::core::*;
use crate::error::*;

use crate::gpu::prelude::*;

/// Represents [`DeviceMemory`][1] size and offset values.
///
/// [1]: super::DeviceMemory
pub type DeviceSize = u64;

/// Non-zero [`DeviceSize`] value.
pub type NonZeroDeviceSize = ::core::num::NonZero<DeviceSize>;

/// Represents a [`device address`][1].
///
/// [1]: super::Gpu::get_buffer_device_address
pub type DeviceAddress = u64;

/// A two-dimensional offset.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, BuildStructure)]
pub struct Offset2D {
    /// The x-offset.
    pub x: i32,
    /// The y-offset.
    pub y: i32
}

impl Offset2D {

    /// Creates the offset from the x and y values.
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

/// Specifies the three-dimensional offset.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, BuildStructure)]
pub struct Offset3D {
    /// The x-offset.
    pub x: i32,
    /// The y-offset.
    pub y: i32,
    /// The z-offset.
    pub z: i32,
}

impl Offset3D {

    /// Creates the offset from the x, y and z values.
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
    
    /// Creates new dimensions from a width. height and depth.
    #[inline]
    pub const fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    /// Returns whether any dimension is zero.
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.width == 0 ||
        self.height == 0 ||
        self.depth == 0
    }

    /// Returns the texel count of the dimensions.
    #[inline]
    pub const fn texel_count(&self) -> DeviceSize {
        self.width as DeviceSize *
        self.height as DeviceSize *
        self.depth as DeviceSize
    }

    /// Returns whether each dimension of self is a multiple of each respective dimension of
    /// `other`.
    #[inline]
    pub const fn is_multiple_of(&self, other: Self) -> bool {
        self.width.is_multiple_of(other.width) &&
        self.height.is_multiple_of(other.height) &&
        self.depth.is_multiple_of(other.depth)
    }

    /// Maps the dimensions with a closure.
    #[must_use]
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

    /// Converts the dimensions into [`ImageCopyOffset`].
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
    /// Specifies the `r` component swizzle.
    pub r: ComponentSwizzle,
    /// Specifies the `g` component swizzle.
    pub g: ComponentSwizzle,
    /// Specifies the `b` component swizzle.
    pub b: ComponentSwizzle,
    /// Specifies the `a` component swizzle.
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

/// Specifies the layers of a given [`mip level`][1].
///
/// [1]: Self::mip_level
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

    /// Returns whether self overlaps with `other`.
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
    
    /// Converts self into [`ImageSubresourceRange`].
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
    pub(crate) fn effective(self, image_layer_count: u32) -> vk::ImageSubresourceLayers {
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

/// Specifies component info of an [`image view`][1].
///
/// [1]: Gpu::create_image_view
#[derive(Clone, Copy, PartialEq, Eq, BuildStructure)]
pub struct ComponentInfo {
    /// Specifies the [`ComponentMapping`] of the view.
    pub component_mapping: ComponentMapping,
    /// Specifies the [`Format`] of the view.
    ///
    /// This **must** be the same as the [`image's`][1] format, if the image was *not* created with
    /// [`mutable format`][2] enabled.
    ///
    /// [1]: ImageId
    /// [2]: ImageCreateInfo::with_format
    pub format: Format,
}

impl ComponentInfo {
    
    /// Creates new [`ComponentInfo`].
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
    /// The x offset of the scissor.
    pub x: u32,
    /// The y offset of the scissor.
    pub y: u32,
    /// The width of the scissor.
    pub width: u32,
    /// The height of the scissor.
    pub height: u32,
}

/// A structure specifying supported [`ResolveModes`].
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct FormatResolveModes {
    /// A bitmask of supported color [`ResolveMode`]
    pub color: ResolveModes,
    /// A bitmask of supported depth [`ResolveMode`]
    pub depth: ResolveModes,
    /// A bitmask of supported stencil [`ResolveMode`]
    pub stencil: ResolveModes,
}

impl FormatResolveModes {

    /// Returns the [`ResolveMode`] bitmask by a specific [`ResolveAspect`].
    #[inline]
    pub fn by_aspect(self, aspect: ResolveAspect) -> ResolveModes {
        match aspect {
            ResolveAspect::Color => self.color,
            ResolveAspect::Depth => self.depth,
            ResolveAspect::Stencil => self.stencil,
        }
    }
}

/// A structure describing image [`Format`] properties.
#[derive(Clone, Copy)]
pub struct ImageFormatProperties {
    /// Maximum supported dimensions of an image with the format.
    pub max_dimensions: Dimensions,
    /// Maximum mip levels of an image with the format.
    pub max_mip_levels: u32,
    /// Maximum array layers of an image with the format.
    pub max_array_layers: u32,
    /// Bitmask of supported [`sample counts`][1] of an image with the format.
    ///
    /// [1]: MsaaSamples
    pub sample_counts: MsaaSamples,
    /// A bitmask of supported [`FormatFeatures`] of an image with the format.
    pub format_features: FormatFeatures,
}

/// Specifies an image offset used with [`copy commands`][1].
///
/// [1]: CopyCommands
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, BuildStructure)]
pub struct ImageCopyOffset {
    /// The x-offset.
    pub x: u32,
    /// The y-offset.
    pub y: u32,
    /// The z-offset.
    pub z: u32,
}

impl Display for ImageCopyOffset {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ImageCopyOffset {

    /// Creates a new offset with the x, y and z values.
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self {
            x, y, z,
        }
    }

    /// Returns whether each offset of self is a multiple of each respective dimension of `extent`.
    #[inline]
    pub fn is_multiple_of(self, extent: Dimensions) -> bool {
        self.x.is_multiple_of(extent.width) &&
        self.y.is_multiple_of(extent.height) &&
        self.z.is_multiple_of(extent.depth)
    }

    #[inline]
    pub(crate) fn is_in_range(
        self,
        image_dimensions: Dimensions,
        copy_extent: Dimensions,
    ) -> bool {
        let off = self + copy_extent;
        off.x <= image_dimensions.width &&
        off.y <= image_dimensions.height &&
        off.z <= image_dimensions.depth
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

/// An offset used in [`blit_image`][1] and [`gen_mip_map`][2].
///
/// [1]: CopyCommands::blit_image
/// [2]: CopyCommands::gen_mip_map
pub type ImageBlitOffset = ImageCopyOffset;

/// Specifies an image [`blit`][1] region.
///
/// [1]: CopyCommands::blit_image.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ImageBlitRegion {
    /// Specifies source [`subresource layers`][1].
    ///
    /// [1]: ImageSubresourceLayers
    pub src_subresource: ImageSubresourceLayers,
    /// Specifies source offsets.
    ///
    /// These define the source rect of the blitting.
    pub src_offsets: [ImageBlitOffset; 2],
    /// Specifies destination [`subresource layers`][1].
    ///
    /// [1]: ImageSubresourceLayers
    pub dst_subresource: ImageSubresourceLayers,
    /// Specifies destination offsets.
    ///
    /// These define the destination rect of the blitting.
    pub dst_offsets: [ImageBlitOffset; 2],
}

/// Specifies a buffer-to-buffer copy range.
#[derive(Default, Clone, Copy, PartialEq, Eq, BuildStructure)]
pub struct BufferCopy {
    /// Specifies the source offset.
    pub src_offset: DeviceSize,
    /// Specifies the destination offset.
    pub dst_offset: DeviceSize,
    /// Specifies the range of the copy.
    pub size: DeviceSize,
}

impl BufferCopy {
   
    /// Creates a new copy range.
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

/// Specifies an image-to-image copy range.
#[derive(Clone, Copy, PartialEq, Eq, BuildStructure)]
pub struct ImageCopy {
    /// Specifies the source [`subresource layers`][1].
    ///
    /// [1]: ImageSubresourceLayers
    pub src_subresource: ImageSubresourceLayers,
    /// Specifies the source [`offset`][1].
    ///
    /// [1]: ImageCopyOffset
    pub src_offset: ImageCopyOffset,
    /// Specifies the destination [`subresource layers`][1].
    ///
    /// [1]: ImageSubresourceLayers
    pub dst_subresource: ImageSubresourceLayers,
    /// Specifies the destination [`offset`][1].
    ///
    /// [1]: ImageCopyOffset
    pub dst_offset: ImageCopyOffset,
    /// Specifies the [`extent`][1] of the copy.
    ///
    /// [1]: Dimensions
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

/// Specifies a buffer-image copy range.
#[derive(Default, Clone, Copy, BuildStructure)]
pub struct BufferImageCopy {
    /// Specifies the offset into the buffer.
    pub buffer_offset: DeviceSize,
    /// Specifies the row length of the buffer.
    #[skip]
    pub buffer_row_length: Option<NonZeroU32>,
    /// Specifies the image height of the buffer.
    #[skip]
    pub buffer_image_height: Option<NonZeroU32>,
    /// Specifies the [`subresource layers'][1] of the image.
    ///
    /// [1]: ImageSubresourceLayers
    pub image_subresource: ImageSubresourceLayers,
    /// Specifies the [`image offset`][1].
    ///
    /// [1]: ImageCopyOffset
    pub image_offset: ImageCopyOffset,
    /// Specifies the image [`extent`][1].
    ///
    /// [1]: Dimensions
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

/// Limits of a [`PhysicalDevice`].
///
/// See the [`Vulkan docs`][1] for a full description.
///
/// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLimits.html
#[derive(Clone, Copy)]
pub struct PhysicalDeviceLimits(pub(crate) vk::PhysicalDeviceLimits);

impl PhysicalDeviceLimits {

    /// Largest width guaranteed to be supported by 1D images.
    #[inline]
    pub fn max_image_dimension_1d(&self) -> u32 {
        self.0.max_image_dimension1_d
    }

    /// Largest width or height guaranteed to be supported 2D images.
    #[inline]
    pub fn max_image_dimension_2d(&self) -> u32 {
        self.0.max_image_dimension2_d
    }

    /// Largest width, height or depth guaranteed to be supported 3D images.
    #[inline]
    pub fn max_image_dimension_3d(&self) -> u32 {
        self.0.max_image_dimension3_d
    }

    /// Largest wdith or height guaranteed to be supported by 2D images that are
    /// [`cube map compatible`][1].
    ///
    /// [1]: ImageCreateInfo::with_cube_map
    #[inline]
    pub fn max_image_dimension_cube(&self) -> u32 {
        self.0.max_image_dimension_cube
    }

    /// The maximum number of supported [`layers`][1] for an image.
    ///
    /// [1]: ImageCreateInfo::with_array_layers
    #[inline]
    pub fn max_image_array_layers(&self) -> u32 {
        self.0.max_image_array_layers
    }

    /// The maximum number of addressable texels for a buffer, which was created with
    /// [`UNIFORM_TEXEL_BUFFER`][1] or [`STORAGE_TEXEL_BUFFER`][2] usages set.
    ///
    /// [1]: BufferUsages::UNIFORM_TEXEL_BUFFER
    /// [2]: BufferUsages::STORAGE_TEXEL_BUFFER
    #[inline]
    pub fn max_texel_buffer_elements(&self) -> u32 {
        self.0.max_texel_buffer_elements
    }

    /// The maximum value that **can** be specified for the [`range`][1] member of
    /// [`DescriptorBufferInfo`] when descriptor type is [`uniform buffer`][2].
    ///
    /// [1]: DescriptorBufferInfo::range
    /// [2]: DescriptorType::UniformBuffer
    #[inline]
    pub fn max_uniform_buffer_range(&self) -> u32 {
        self.0.max_uniform_buffer_range
    }

    /// The maximum value that **can** be specified for the [`range`][1] member of
    /// [`DescriptorBufferInfo`] when descriptor type is [`storage buffer`][2].
    ///
    /// [1]: DescriptorBufferInfo::range
    /// [2]: DescriptorType::StorageBuffer
    #[inline]
    pub fn max_storage_buffer_range(&self) -> u32 {
        self.0.max_storage_buffer_range
    }
   
    /// The maximum size, in bytes, of the pool of push constant memory.
    ///
    /// The offsets + sizes of push constants in shaders **must** be less than or equal to this
    /// limit.
    #[inline]
    pub fn max_push_constants_size(&self) -> u32 {
        self.0.max_push_constants_size
    }
   
    /// This is the maximum number of [`device memory allocations`][1], which **can**
    /// simultaneously exist on a [`Device`].
    ///
    /// [1]: MemoryBinder
    #[inline]
    pub fn max_memory_allocation_count(&self) -> u32 {
        self.0.max_memory_allocation_count
    }
   
    /// This is the maximum number of [`samplers`][1], which **can** simultaneously exist on a
    /// [`Device`].
    ///
    /// [1]: Sampler
    #[inline]
    pub fn max_sampler_allocation_count(&self) -> u32 {
        self.0.max_sampler_allocation_count
    }
   
    /// The granularity, in bytes, at which buffer or linear image resources, linear or optimal
    /// tensor resources, and optimal image resources can be bound to adjacent offsets in the same
    /// [`DeviceMemory`] object without aliasing.
    #[inline]
    pub fn buffer_image_granularity(&self) -> DeviceSize {
        self.0.buffer_image_granularity
    }
   
    /// The total amount of address space available, in bytes, for sparse memory resources.
    #[inline]
    pub fn sparse_address_space_size(&self) -> DeviceSize {
        self.0.sparse_address_space_size
    }
   
    /// The maximum number of descriptor sets that **can** be simultaneously used by a pipeline.
    #[inline]
    pub fn max_bound_descriptor_sets(&self) -> u32 {
        self.0.max_bound_descriptor_sets
    }
   
    /// The maximum number of samplers that **can** be accessible to a single shader stage in
    /// a [`shader set`][1].
    ///
    /// [1]: ShaderSet
    #[inline]
    pub fn max_per_stage_descriptor_samplers(&self) -> u32 {
        self.0.max_per_stage_descriptor_samplers
    }
    
    /// The maximum number of uniform buffers that **can** be accessible to a single shader stage in a
    /// [`shader set`][1].
    ///
    /// [1]: ShaderSet
    #[inline]
    pub fn max_per_stage_descriptor_uniform_buffers(&self) -> u32 {
        self.0.max_per_stage_descriptor_uniform_buffers
    }

    /// The maximum number of storage buffers that **can** be accessible to a single shader stage in a
    /// [`shader set`][1].
    ///
    /// [1]: ShaderSet
    #[inline]
    pub fn max_per_stage_descriptor_storage_buffers(&self) -> u32 {
        self.0.max_per_stage_descriptor_storage_buffers
    }

    /// The maximum number of sampled images that **can** be accessible to a single shader stage in a
    /// [`shader set`][1].
    ///
    /// [1]: ShaderSet
    #[inline]
    pub fn max_per_stage_descriptor_sampled_images(&self) -> u32 {
        self.0.max_per_stage_descriptor_sampled_images
    }

    /// The maximum number of storage images that **can** be accessible to a single shader stage in a
    /// [`shader set`][1].
    ///
    /// [1]: ShaderSet
    #[inline]
    pub fn max_per_stage_descriptor_storage_images(&self) -> u32 {
        self.0.max_per_stage_descriptor_storage_images
    }

    /// The maximum number of input attachments that **can** be accessible to a single shader stage in 
    /// a [`shader set`][1].
    ///
    /// [1]: ShaderSet   
    #[inline]
    pub fn max_per_stage_descriptor_input_attachments(&self) -> u32 {
        self.0.max_per_stage_descriptor_input_attachments
    }
    
    /// The maximum number of resources that **can** be accessible to a single shader stage in a
    /// [`shader set`][1]
    ///
    /// [1]: ShaderSet
    #[inline]
    pub fn max_per_stage_resources(&self) -> u32 {
        self.0.max_per_stage_resources
    }
   
    /// The maximum number of samplers that **can** be included in a [`shader set`][1].
    ///
    /// [1]: ShaderSet
    #[inline]
    pub fn max_descriptor_set_samplers(&self) -> u32 {
        self.0.max_descriptor_set_samplers
    }

    /// The maximum number of uniform buffers that **can** be included in a [`shader set`][1].
    ///
    /// [1]: ShaderSet
    #[inline]
    pub fn max_descriptor_set_uniform_buffers(&self) -> u32 {
        self.0.max_descriptor_set_uniform_buffers
    }

    /// The maximum number of dynamic uniform buffers that **can** be included in a [`shader set`][1].
    ///
    /// [1]: ShaderSet
    #[inline]
    pub fn max_descriptor_set_uniform_buffers_dynamic(&self) -> u32 {
        self.0.max_descriptor_set_uniform_buffers_dynamic
    }

    /// The maximum number of storage buffers that **can** be included in a [`shader set`][1].
    ///
    /// [1]: ShaderSet
    #[inline]
    pub fn max_descriptor_set_storage_buffers(&self) -> u32 {
        self.0.max_descriptor_set_storage_buffers
    }

    /// The maximum number of dynamic storage buffers that **can** be included in a
    /// [`shader set`][1].
    ///
    /// [1]: ShaderSet
    #[inline]
    pub fn max_descriptor_set_storage_buffers_dynamic(&self) -> u32 {
        self.0.max_descriptor_set_storage_buffers_dynamic
    }

    /// The maximum number of sampled images that **can** be included in a [`shader set`][1].
    ///
    /// [1]: ShaderSet
    #[inline]
    pub fn max_descriptor_set_sampled_images(&self) -> u32 {
        self.0.max_descriptor_set_sampled_images
    }

    /// The maximum number of storage images that **can** be included in a [`shader set`][1].
    ///
    /// [1]: ShaderSet
    #[inline]
    pub fn max_descriptor_set_storage_images(&self) -> u32 {
        self.0.max_descriptor_set_storage_images
    }

    /// The maximum number of input attachments that **can** be included in a [`shader set`][1].
    ///
    /// [1]: ShaderSet
    #[inline]
    pub fn max_descriptor_set_input_attachments(&self) -> u32 {
        self.0.max_descriptor_set_input_attachments
    }
   
    /// The maximum number of vertex buffers that can be specified for providing vertex attributes
    /// to a [`graphics pipeline`][1].
    ///
    /// [1]: GraphicsPipelineCreateInfo
    #[inline]
    pub fn max_vertex_input_attributes(&self) -> u32 {
        self.0.max_vertex_input_attributes
    }
   
    /// The maximum number of vertex buffers that can be specified for providing vertex attributes
    /// to a [`graphics pipeline`][1].
    ///
    /// [1]: GraphicsPipelineCreateInfo
    #[inline]
    pub fn max_vertex_input_bindings(&self) -> u32 {
        self.0.max_vertex_input_bindings
    }

    /// The maximum vertex input attribute offset that **can** be added to the vertex input binding
    /// stride.
    ///
    /// The [`offset`][1] of [`VertexInputAttribute`] **must** be less than or equal to this limit.
    ///
    /// [1]: VertexInputAttribute::offset
    #[inline]
    pub fn max_vertex_input_attribute_offset(&self) -> u32 {
        self.0.max_vertex_input_attribute_offset
    }

    /// The maximum vertex input binding stride that **can** be specified in a vertex input binding.
    ///
    /// The [`stride`][1] of a [`VertexInputBinding`] **must** be less than or equal to this limit.
    ///
    /// [1]: VertexInputBinding::stride   
    #[inline]
    pub fn max_vertex_input_binding_stride(&self) -> u32 {
        self.0.max_vertex_input_binding_stride
    }
   
    /// The maximum number of components of output variables which can be output by a vertex
    /// shader.
    #[inline]
    pub fn max_vertex_output_components(&self) -> u32 {
        self.0.max_vertex_output_components
    }
   
    /// The maximum tessellation generation level supported by the fixed-function tessellation
    /// primitive generator.
    #[inline]
    pub fn max_tessellation_generation_level(&self) -> u32 {
        self.0.max_tessellation_generation_level
    }
   
    /// The maximum patch size, in vertices, of patches that can be processed by the tessellation
    /// control shader and tessellation primitive generator.
    #[inline]
    pub fn max_tessellation_patch_size(&self) -> u32 {
        self.0.max_tessellation_patch_size
    }

    /// The maximum number of components of input variables which can be provided as per-vertex
    /// inputs to the tessellation control shader stage.
    #[inline]
    pub fn max_tessellation_control_per_vertex_input_components(&self) -> u32 {
        self.0.max_tessellation_control_per_vertex_input_components
    }
   
    /// The maximum number of components of per-vertex output variables which can be output from
    /// the tessellation control shader stage.
    #[inline]
    pub fn max_tessellation_control_per_vertex_output_components(&self) -> u32 {
        self.0.max_tessellation_control_per_vertex_output_components
    }
    
    /// The maximum number of components of per-patch output variables which can be output from the
    /// tessellation control shader stage.
    #[inline]
    pub fn max_tessellation_control_per_patch_output_components(&self) -> u32 {
        self.0.max_tessellation_control_per_patch_output_components
    }
   
    /// The maximum total number of components of per-vertex and per-patch output variables which
    /// can be output from the tessellation control shader stage.
    #[inline]
    pub fn max_tessellation_control_total_output_components(&self) -> u32 {
        self.0.max_tessellation_control_total_output_components
    }
   
    /// The maximum number of components of input variables which can be provided as per-vertex
    /// inputs to the tessellation evaluation shader stage.
    #[inline]
    pub fn max_tessellation_evaluation_input_components(&self) -> u32 {
        self.0.max_tessellation_evaluation_input_components
    }
   
    /// The maximum number of components of per-vertex output variables which can be output from the
    /// tessellation evaluation shader stage.
    #[inline]
    pub fn max_tessellation_evaluation_output_components(&self) -> u32 {
        self.0.max_tessellation_evaluation_output_components
    }
   
    /// The maximum number of components of input variables which can be provided as inputs to the
    /// fragment shader stage.
    #[inline]
    pub fn max_fragment_input_components(&self) -> u32 {
        self.0.max_fragment_input_components
    }
   
    /// The maximum number of output attachments which can be written to by the fragment shader stage.
    #[inline]
    pub fn max_fragment_output_attachments(&self) -> u32 {
        self.0.max_fragment_output_attachments
    }
   
    /// The maximum number of output attachments which can be written to by the fragment shader
    /// stage when blending is enabled and one of the dual source blend modes is in use.
    #[inline]
    pub fn max_fragment_dual_src_attachments(&self) -> u32 {
        self.0.max_fragment_dual_src_attachments
    }
   
    /// The total number of storage buffers, storage images, and output Location decorated color
    /// attachments which can be used in the fragment shader stage.
    #[inline]
    pub fn max_fragment_combined_output_resources(&self) -> u32 {
        self.0.max_fragment_combined_output_resources
    }
   
    /// The maximum total storage size, in bytes, available for variables declared with the
    /// Workgroup storage class in shader modules (or with the shared storage qualifier in GLSL) in
    /// the compute shader stage.
    #[inline]
    pub fn max_compute_shared_memory_size(&self) -> u32 {
        self.0.max_compute_shared_memory_size
    }
    
    /// The maximum number of local workgroups that can be dispatched by a single dispatching
    /// command.
    #[inline]
    pub fn max_compute_work_group_count(&self) -> [u32; 3usize] {
        self.0.max_compute_work_group_count
    }
    
    /// The maximum total number of compute shader invocations in a single local workgroup.
    #[inline]
    pub fn max_compute_work_group_invocations(&self) -> u32 {
        self.0.max_compute_work_group_invocations
    }
   
    /// The maximum size of a local compute workgroup, per dimension.
    #[inline]
    pub fn max_compute_work_group_size(&self) -> [u32; 3usize] {
        self.0.max_compute_work_group_size
    }
   
    /// The number of bits of subpixel precision in framebuffer coordinates.
    #[inline]
    pub fn sub_pixel_precision_bits(&self) -> u32 {
        self.0.sub_pixel_precision_bits
    }
   
    /// The number of bits of precision in the division along an axis of an image used for
    /// minification and magnification filter.
    #[inline]
    pub fn sub_texel_precision_bits(&self) -> u32 {
        self.0.sub_texel_precision_bits
    }
   
    /// The number of bits of division that the LOD calculation for mipmap fetching get snapped to
    /// when determining the contribution from each mip level to the mip filtered results.
    #[inline]
    pub fn mipmap_precision_bits(&self) -> u32 {
        self.0.mipmap_precision_bits
    }
   
    /// The maximum index value that can be used for indexed draw calls when using 32-bit indices.
    #[inline]
    pub fn max_draw_indexed_index_value(&self) -> u32 {
        self.0.max_draw_indexed_index_value
    }
   
    /// The maximum absolute [`sampler LOD bias`][1].
    ///
    /// [1]: SamplerAttributes::mip_lod_bias
    #[inline]
    pub fn max_sampler_lod_bias(&self) -> f32 {
        self.0.max_sampler_lod_bias
    }
   
    /// The maximum degree of sampler anisotropy. 
    ///
    /// The maximum degree of anisotropic filtering used for an image sampling operation is the
    /// minimum of the [`max anisotropy`][1] member of the [`SamplerAttributes`] and this limit.
    ///
    /// [1]: SamplerAttributes::max_anisotropy
    #[inline]
    pub fn max_sampler_anisotropy(&self) -> f32 {
        self.0.max_sampler_anisotropy
    }
   
    /// The maximum number of active viewports.
    #[inline]
    pub fn max_viewports(&self) -> u32 {
        self.0.max_viewports
    }
   
    /// The maximum viewport dimensions in the width and height dimensions, respectively.
    #[inline]
    pub fn max_viewport_dimensions(&self) -> [u32; 2usize] {
        self.0.max_viewport_dimensions
    }
   
    /// The minimum..maximum range that the corners of a viewport must be contained in.
    #[inline]
    pub fn viewport_bounds_range(&self) -> [f32; 2usize] {
        self.0.viewport_bounds_range
    }
   
    /// The number of bits of subpixel precision for viewport bounds.
    #[inline]
    pub fn viewport_sub_pixel_bits(&self) -> u32 {
        self.0.viewport_sub_pixel_bits
    }
   
    /// The minimum required alignment, in bytes, of host visible memory allocations within the
    /// host address space.
    ///
    /// See [`DeviceMemory::map_memory`].
    #[inline]
    pub fn min_memory_map_alignment(&self) -> usize {
        self.0.min_memory_map_alignment
    }
  
    /*
    TODO: Buffer views
    #[inline]
    pub fn min_texel_buffer_offset_alignment(&self) -> DeviceSize {
        self.0.min_texel_buffer_offset_alignment
    }
    */
    
    /// The minimum required alignment, in bytes, for the [`offset`][1] member of the
    /// [`DescriptorBufferInfo`] for [`uniform buffers`][2].
    ///
    /// [1]: DescriptorBufferInfo::offset
    /// [2]: DescriptorType::UniformBuffer
    #[inline]
    pub fn min_uniform_buffer_offset_alignment(&self) -> DeviceSize {
        self.0.min_uniform_buffer_offset_alignment
    }

    /// The minimum required alignment, in bytes, for the [`offset`][1] member of the
    /// [`DescriptorBufferInfo`] for [`storage buffers`][2].
    ///
    /// [1]: DescriptorBufferInfo::offset   
    /// [2]: DescriptorType::StorageBuffer
    #[inline]
    pub fn min_storage_buffer_offset_alignment(&self) -> DeviceSize {
        self.0.min_storage_buffer_offset_alignment
    }
   
    /// The minimum offset value for the Offset or ConstOffset image operand of any of the
    /// OpImageSample* or OpImageFetch* image instructions.
    #[inline]
    pub fn min_texel_offset(&self) -> i32 {
        self.0.min_texel_offset
    }
   
    /// The maximum offset value for the Offset or ConstOffset image operand of any of the
    /// OpImageSample* or OpImageFetch* image instructions.
    #[inline]
    pub fn max_texel_offset(&self) -> u32 {
        self.0.max_texel_offset
    }
   
    /// The minimum offset value for the Offset, ConstOffset, or ConstOffsets image operands of any
    /// of the OpImage*Gather image instructions.
    #[inline]
    pub fn min_texel_gather_offset(&self) -> i32 {
        self.0.min_texel_gather_offset
    }
   
    /// The maximum offset value for the Offset, ConstOffset, or ConstOffsets image operands of any
    /// of the OpImage*Gather image instructions.
    #[inline]
    pub fn max_texel_gather_offset(&self) -> u32 {
        self.0.max_texel_gather_offset
    }
   
    /// The base minimum (inclusive) negative offset value for the Offset operand of the
    /// InterpolateAtOffset extended instruction.
    #[inline]
    pub fn min_interpolation_offset(&self) -> f32 {
        self.0.min_interpolation_offset
    }
   
    /// The base maximum (inclusive) positive offset value for the Offset operand of the
    /// InterpolateAtOffset extended instruction.
    #[inline]
    pub fn max_interpolation_offset(&self) -> f32 {
        self.0.max_interpolation_offset
    }
   
    /// The number of fractional bits that the x and y offsets to the InterpolateAtOffset extended
    /// instruction may be rounded to as fixed-point values.
    #[inline]
    pub fn sub_pixel_interpolation_offset_bits(&self) -> u32 {
        self.0.sub_pixel_interpolation_offset_bits
    }
   
    /// The maximum width for a framebuffer.
    ///
    /// The [`width`][1] + the x value of [`offset`][2] of [`render_area`][3] in [`RenderingInfo`]
    /// **must** be less than or equal to this value.
    ///
    /// [1]: RenderArea::width
    /// [2]: RenderArea::offset
    /// [3]: RenderArea
    #[inline]
    pub fn max_framebuffer_width(&self) -> u32 {
        self.0.max_framebuffer_width
    }
    
    /// The maximum height for a framebuffer.
    ///
    /// The [`height`][1] + the y value of [`offset`][2] of [`render_area`][3] in [`RenderingInfo`]
    /// **must** be less than or equal to this value.
    ///
    /// [1]: RenderArea::height
    /// [2]: RenderArea::offset
    /// [3]: RenderArea
    #[inline]
    pub fn max_framebuffer_height(&self) -> u32 {
        self.0.max_framebuffer_height
    }
   
    /// The maximum layer count for a layered framebuffer.
    ///
    /// The [`layer count`][1] in [`RenderingInfo`] **must** be less than or equal to this value.
    ///
    /// [1]: RenderingInfo::layer_count
    #[inline]
    pub fn max_framebuffer_layers(&self) -> u32 {
        self.0.max_framebuffer_layers
    }
   
    /// A bitmask of [`MsaaSamples`] indicating the color sample counts that are supported
    /// for all framebuffer color attachments with floating- or fixed-point formats.
    ///
    /// The [`msaa samples`][1] in [`RenderingInfo`] **must** be contained in this value.
    ///
    /// [1]: RenderingInfo::msaa_samples
    #[inline]
    pub fn framebuffer_color_sample_counts(&self) -> MsaaSamples {
        MsaaSamples::from_raw(self.0.framebuffer_color_sample_counts.as_raw())
    }

    /// A bitmask of [`MsaaSamples`] indicating the supported depth sample counts for all
    /// framebuffer depth/stencil attachments, when the format includes a depth component.
    ///
    /// The [`msaa samples`][1] in [`RenderingInfo`] **must** be contained in this value.
    ///
    /// [1]: RenderingInfo::msaa_samples
    #[inline]
    pub fn framebuffer_depth_sample_counts(&self) -> MsaaSamples {
        MsaaSamples::from_raw(self.0.framebuffer_depth_sample_counts.as_raw())
    }
   
    /// A bitmask of [`MsaaSamples`] indicating the supported stencil sample counts for all
    /// framebuffer depth/stencil attachments, when the format includes a stencil component.
    ///
    /// The [`msaa samples`][1] in [`RenderingInfo`] **must** be contained in this value.
    ///
    /// [1]: RenderingInfo::msaa_samples
    #[inline]
    pub fn framebuffer_stencil_sample_counts(&self) -> MsaaSamples {
        MsaaSamples::from_raw(self.0.framebuffer_stencil_sample_counts.as_raw())
    }
   
    /// A bitmask of [`MsaaSamples`] indicating the supported sample counts for a subpass
    /// which uses no attachments.
    #[inline]
    pub fn framebuffer_no_attachments_sample_counts(&self) -> MsaaSamples {
        MsaaSamples::from_raw(self.0.framebuffer_no_attachments_sample_counts.as_raw())
    }
   
    /// The maximum number of color attachments that can be used by a subpass in a render pass.
    ///
    /// The color attachment count when [`rendering`][1] **must** be less than or equal to this
    /// value.
    ///
    /// [1]: GraphicsCommands::render
    #[inline]
    pub fn max_color_attachments(&self) -> u32 {
        self.0.max_color_attachments
    }
   
    /// A bitmask of [`MsaaSamples`] indicating the sample counts supported for all 2D images
    /// created with the [`SAMPLED`][1] usage and a [`non-integer`][2] [`color format`][3].
    ///
    /// [1]: ImageUsages::SAMPLED
    /// [2]: NumericFormat::is_integer
    /// [3]: Format::numeric_format_color
    #[inline]
    pub fn sampled_image_color_sample_counts(&self) -> MsaaSamples {
        MsaaSamples::from_raw(self.0.sampled_image_color_sample_counts.as_raw())
    }
   
    /// a bitmask of [`MsaaSamples`] indicating the sample counts supported for all 2D images
    /// created with the [`SAMPLED`][1] usage and an [`integer`][2] [`color format`][3].
    ///
    /// [1]: ImageUsages::SAMPLED
    /// [2]: NumericFormat::is_integer
    /// [3]: Format::numeric_format_color
    #[inline]
    pub fn sampled_image_integer_sample_counts(&self) -> MsaaSamples {
        MsaaSamples::from_raw(self.0.sampled_image_integer_sample_counts.as_raw())
    }
   
    /// A bitmask of [`MsaaSamples`] indicating the sample counts supported for all 2D images
    /// created with the [`SAMPLED`][1] usage and a [`depth format`][2]
    ///
    /// [1]: ImageUsages::SAMPLED
    /// [2]: Format::numeric_format_depth
    #[inline]
    pub fn sampled_image_depth_sample_counts(&self) -> MsaaSamples {
        MsaaSamples::from_raw(self.0.sampled_image_depth_sample_counts.as_raw())
    }

    /// A bitmask of [`MsaaSamples`] indicating the sample counts supported for all 2D images
    /// created with the [`SAMPLED`][1] usage and a [`stencil format`][2]
    ///
    /// [1]: ImageUsages::SAMPLED
    /// [2]: Format::numeric_format_stencil
    #[inline]
    pub fn sampled_image_stencil_sample_counts(&self) -> MsaaSamples {
        MsaaSamples::from_raw(self.0.sampled_image_stencil_sample_counts.as_raw())
    }

    /// A bitmask of [`MsaaSamples`] indicating the sample counts supported for all 2D images
    /// created with the [`STORAGE`][1] usage.
    ///
    /// [1]: ImageUsages::STORAGE
    #[inline]
    pub fn storage_image_sample_counts(&self) -> MsaaSamples {
        MsaaSamples::from_raw(self.0.storage_image_sample_counts.as_raw())
    }

    /// The maximum number of array elements of a variable decorated with the SampleMask built-in
    /// decoration.
    #[inline]
    pub fn max_sample_mask_words(&self) -> u32 {
        self.0.max_sample_mask_words
    }

    /// Specifies support for timestamps on all graphics and compute queues.
    #[inline]
    pub fn timestamp_compute_and_graphics(&self) -> bool {
        self.0.timestamp_compute_and_graphics != 0
    }

    /// The number of nanoseconds required for a timestamp query to be incremented by 1.
    #[inline]
    pub fn timestamp_period(&self) -> f32 {
        self.0.timestamp_period
    }

    /// The maximum number of clip distances that can be used in a single shader stage.
    #[inline]
    pub fn max_clip_distances(&self) -> u32 {
        self.0.max_clip_distances
    }

    /// The maximum number of cull distances that can be used in a single shader stage.
    #[inline]
    pub fn max_cull_distances(&self) -> u32 {
        self.0.max_cull_distances
    }

    /// The maximum combined number of clip and cull distances that can be used in a single shader
    /// stage.
    #[inline]
    pub fn max_combined_clip_and_cull_distances(&self) -> u32 {
        self.0.max_combined_clip_and_cull_distances
    }

    /*
    TODO: DeviceQueue priorities
    #[inline]
    pub fn discrete_queue_priorities(&self) -> u32 {
        self.0.discrete_queue_priorities
    }*/

    /// The range minimum..maximum of supported sizes for points.
    ///
    /// Values written to variables decorated with the PointSize built-in decoration are clamped to
    /// this range.
    #[inline]
    pub fn point_size_range(&self) -> [f32; 2usize] {
        self.0.point_size_range
    }

    /// The range minimum..maximum of supported widths for lines.
    ///
    /// Values specified by the [`line_width`][1] of [`GraphicsPipelineCreateInfo`] or the
    /// `line_width` parameter to [`set_line_width`][2] are clamped to this range.
    ///
    /// [1]: GraphicsPipelineCreateInfo::with_line_width
    /// [2]: DrawPipelineCommands::set_line_width
    #[inline]
    pub fn line_width_range(&self) -> [f32; 2usize] {
        self.0.line_width_range
    }

    /// The granularity of supported point sizes.
    #[inline]
    pub fn point_size_granularity(&self) -> f32 {
        self.0.point_size_granularity
    }

    /// The granularity of supported line widths.
    #[inline]
    pub fn line_width_granularity(&self) -> f32 {
        self.0.line_width_granularity
    }

    /// Specifies whether lines are rasterized according to the preferred method of rasterization.
    #[inline]
    pub fn strict_lines(&self) -> bool {
        self.0.strict_lines != 0
    }

    /// Specifies whether rasterization uses the standard sample locations.
    #[inline]
    pub fn standard_sample_locations(&self) -> bool {
        self.0.standard_sample_locations != 0
    }

    /// The optimal buffer offset alignment in bytes for [`copy_buffet_to_image`][1] and
    /// [`copy_image_to_buffer`][2].
    ///
    /// [1]: CopyCommands::copy_buffer_to_image
    /// [2]: CopyCommands::copy_image_to_buffer
    #[inline]
    pub fn optimal_buffer_copy_offset_alignment(&self) -> DeviceSize {
        self.0.optimal_buffer_copy_offset_alignment
    }

    /// The optimal buffer row pitch alignment in bytes for [`copy_buffer_to_image`][1] and
    /// [`copy_image_to_buffer`][2].
    ///
    /// [1]: CopyCommands::copy_buffer_to_image
    /// [2]: CopyCommands::copy_image_to_buffer
    #[inline]
    pub fn optimal_buffer_copy_row_pitch_alignment(&self) -> DeviceSize {
        self.0.optimal_buffer_copy_row_pitch_alignment
    }

    /// The size and alignment in bytes that bounds concurrent access to host-mapped device memory.
    #[inline]
    pub fn non_coherent_atom_size(&self) -> DeviceSize {
        self.0.non_coherent_atom_size
    }
}

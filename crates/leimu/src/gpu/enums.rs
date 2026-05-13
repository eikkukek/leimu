use core::{
    fmt::{self, Display},
    hash::Hash,
};

use tuhka::vk;

use crate::{bitflags, macros::vk_enum};

use crate::gpu::prelude::*;

/// 32-bit bit masks.
pub type Flags32 = u32;
/// 64-bit bit masks.
pub type Flags64 = u64;

bitflags! {

    /// Specifies a bitmask of multisample anti-aliasing sample counts
    ///
    /// Default value is [`MSAA::X1`].
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleCountFlagBits.html>
    #[default = Self::X1]
    pub struct MsaaSamples: Flags32 {
        /// Specifies one sample per pixel.
        X1 = vk::SampleCountFlags::TYPE_1.as_raw(),
        /// Specifies 2 samples per pixel.
        X2 = vk::SampleCountFlags::TYPE_2.as_raw(),
        /// Specifies 4 samples per pixel.
        X4 = vk::SampleCountFlags::TYPE_4.as_raw(),
        /// Specifies 8 samples per pixel.
        X8 = vk::SampleCountFlags::TYPE_8.as_raw(),
        /// Specifies 16 samples per pixel.
        X16 = vk::SampleCountFlags::TYPE_16.as_raw(),
        /// Specifies 32 samples per pixel.
        X32 = vk::SampleCountFlags::TYPE_32.as_raw(),
        /// Specifies 64 samples per pixel.
        X64 = vk::SampleCountFlags::TYPE_64.as_raw(),
    }
    /// Describes what a buffer **can** be used for.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits.html>
    #[default = Self::empty()]
    pub struct BufferUsages: Flags32 {
        /// Specifies that the buffer **can** be used as the source of transfer operations.
        TRANSFER_SRC = vk::BufferUsageFlags::TRANSFER_SRC.as_raw(),
        /// Specifies that the buffer **can** be used as the destination of transfer operations.
        TRANSFER_DST = vk::BufferUsageFlags::TRANSFER_DST.as_raw(), 
        /// Specifies that the buffer **can** be used as a uniform texel buffer.
        UNIFORM_TEXEL_BUFFER = vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER.as_raw(),
        /// Specifies that the buffer **can** be used as a storage texel buffer.
        STORAGE_TEXEL_BUFFER = vk::BufferUsageFlags::STORAGE_TEXEL_BUFFER.as_raw(),
        /// Specifies that the buffer **can** be used as a uniform buffer.
        UNIFORM_BUFFER = vk::BufferUsageFlags::UNIFORM_BUFFER.as_raw(),
        /// Specifies that the buffer **can** be used as a storage buffer.
        STORAGE_BUFFER = vk::BufferUsageFlags::STORAGE_BUFFER.as_raw(),
        /// Specifies that the buffer **can** be used as an index buffer.
        INDEX_BUFFER = vk::BufferUsageFlags::INDEX_BUFFER.as_raw(),
        /// Specifies that the buffer **can** be used as a vertex buffer.
        VERTEX_BUFFER = vk::BufferUsageFlags::VERTEX_BUFFER.as_raw(),
        /// Specifies that the buffer **can** be used as in indirect commands.
        INDIRECT_BUFFER = vk::BufferUsageFlags::INDIRECT_BUFFER.as_raw(),
        /// Specifies that the buffer **can** be used as a [`descriptor heap`][1].
        ///
        /// [1]: ext::descriptor_heap
        DESCRIPTOR_HEAP_EXT = vk::BufferUsageFlags::DESCRIPTOR_HEAP_EXT.as_raw(),
    }

    /// Specifies what an [`Image`] **can** be used for.
    ///
    /// Default value is [`ImageUsages::empty()`].
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlagBits.html>
    #[default = Self::empty()]
    pub struct ImageUsages: Flags32 {
        /// Specifies that the image **can** be used as the source of transfer operations.
        TRANSFER_SRC = vk::ImageUsageFlags::TRANSFER_SRC.as_raw(),
        /// Specifies that the image **can** be used as the destination of transfer operations.
        TRANSFER_DST = vk::ImageUsageFlags::TRANSFER_DST.as_raw(),
        /// Specifies that the image **can** be used sampled from in a shader.
        SAMPLED = vk::ImageUsageFlags::SAMPLED.as_raw(),
        /// Specifies that the image **can** be used as a storage image in a shader.
        STORAGE = vk::ImageUsageFlags::STORAGE.as_raw(),
        /// Specifies that the image **can** be used as a color attachment in rendering.
        COLOR_ATTACHMENT = vk::ImageUsageFlags::COLOR_ATTACHMENT.as_raw(),
        /// Specifies that the image **can** be used as a depth/stencil attachment in rendering.
        DEPTH_STENCIL_ATTACHMENT = vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT.as_raw(),
        /// Specifies that the image **can** be used as an input attachment in rendering.
        INPUT_ATTACHMENT = vk::ImageUsageFlags::INPUT_ATTACHMENT.as_raw(),
    }
    
    /// Specifies additional flags used when [`creating`][1] an image.
    ///
    /// [1]: Gpu::create_resources
    #[default = Self::empty()]
    pub struct ImageCreateFlags: Flags32 {
        /// Specifies that the image **can** be used to [`create`][1] an image view with a
        /// different format from the image.
        ///
        /// [1]: Gpu::create_image_view
        MUTABLE_FORMAT = vk::ImageCreateFlags::MUTABLE_FORMAT.as_raw(),
        /// Specifies that the image **can** be used to [`create`][1] an image view of 
        ///
        /// # Valid usage
        /// - The [`image's type`][2] **must** be [`Type2d`][3].
        /// - The [`width and height`][4] of the image **must** be equal and depth **must** 
        ///   implicitly be 1.
        /// - The [`array layers`][5] of the image **must** be at least 6.
        ///
        /// [1]: Gpu::create_image_view
        /// [2]: ImageCreateInfo::with_type
        /// [3]: ImageType::Type2d
        /// [4]: ImageCreateInfo::with_dimensions
        /// [5]: ImageCreateInfo::with_array_layers
        CUBE_COMPATIBLE = vk::ImageCreateFlags::CUBE_COMPATIBLE.as_raw(),
        /// Specifies that image views of type [`Type2dArray`][2] **can** be created with an
        /// image of [`Type3d`][3].
        ///
        /// # Valid usage
        /// - The [`image's type`][4] **must** be [`Type3d`][3].
        ///
        /// [2]: ImageViewType::Type2dArray
        /// [3]: ImageViewType::Type3d
        /// [4]: ImageCreateInfo::with_type
        TYPE_2D_ARRAY_COMPATIBLE = vk::ImageCreateFlags::TYPE_2D_ARRAY_COMPATIBLE.as_raw(),
    }

    /// Specifies which image aspect to use for e.g. [`ImageSubresourceRange`].
    ///
    /// Default value is [`NONE`][1].
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageAspectFlagBits.html#>
    ///
    /// [1]: Self::NONE
    #[default = Self::NONE]
    pub struct ImageAspects: Flags32 {
        /// Specifies no image aspect.
        NONE = vk::ImageAspectFlags::NONE.as_raw(),
        /// Specifies the color aspect.
        COLOR = vk::ImageAspectFlags::COLOR.as_raw(),
        /// Specifies the depth aspect.
        DEPTH = vk::ImageAspectFlags::DEPTH.as_raw(),
        /// Specifies the stencil aspect.
        STENCIL = vk::ImageAspectFlags::STENCIL.as_raw(),
        /// Specifies the plane 0 of a [`multi-planar format`][1].
        ///
        /// [1]: Format::plane_count
        PLANE_0 = vk::ImageAspectFlags::PLANE_0.as_raw(),
        /// Specifies the plane 1 of a [`multi-planar format`][1].
        ///
        /// [1]: Format::plane_count
        PLANE_1 = vk::ImageAspectFlags::PLANE_1.as_raw(),
        /// Specifies the plane 2 of a [`multi-planar format`][1].
        ///
        /// [1]: Format::plane_count
        PLANE_2 = vk::ImageAspectFlags::PLANE_2.as_raw(),
    }

    /// Specifies sets of stencil state for which to update operations.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilFaceFlagBits.html>
    pub struct StencilFaces: Flags32 {
        /// Specifies that only the front set of stencil state is updated.
        FRONT = vk::StencilFaceFlags::FRONT.as_raw(),
        /// Specifies that only the back set of stencil state is updated.
        BACK = vk::StencilFaceFlags::BACK.as_raw(),
        /// Specifies that both the front and the back of stencil state is updated.
        FRONT_AND_BACK = vk::StencilFaceFlags::FRONT_AND_BACK.as_raw(),
    }
   
    /// Bitmask controlling triangle culling.
    ///
    /// Default is [`CullModeFlags::FRONT`].
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCullModeFlagBits.html>
    #[default = Self::FRONT]
    pub struct CullModes: Flags32 {
        /// Specifies that no triangles are discarded.
        NONE = vk::CullModeFlags::NONE.as_raw(),
        /// Specifies that front-facing triangles are discarded.
        ///
        /// Front face is defined by [`FrontFace`].
        FRONT = vk::CullModeFlags::FRONT.as_raw(),
        /// Specifies that back-facing triangles are discarded.
        ///
        /// Front face is defined by [`FrontFace`].
        BACK = vk::CullModeFlags::BACK.as_raw(),
        /// Specifies that all triangles are discarded.
        FRONT_AND_BACK = vk::CullModeFlags::FRONT_AND_BACK.as_raw(),
    }

    /// Bitmask specifying what features are supported by a format.
    pub struct FormatFeatures: Flags64 {
        /// Specifies that an image view **can** be sampled from.
        SAMPLED_IMAGE = vk::FormatFeatureFlags2::SAMPLED_IMAGE.as_raw(),
        /// Specifies that an image view **can** be used as a storage image.
        STORAGE_IMAGE = vk::FormatFeatureFlags2::STORAGE_IMAGE.as_raw(),
        /// Specifies that an image view **can** be used as a storage image that supports atomic
        /// operations.
        STORAGE_IMAGE_ATOMIC = vk::FormatFeatureFlags2::STORAGE_IMAGE_ATOMIC.as_raw(),
        /// Specifies that the format **can** be used as a vertex attribute format.
        VERTEX_BUFFER = vk::FormatFeatureFlags2::VERTEX_BUFFER.as_raw(),
        /// Specifies that an image view **can** be used as a color attachment.
        COLOR_ATTACHMENT = vk::FormatFeatureFlags2::COLOR_ATTACHMENT.as_raw(),
        /// Specifies that an image view **can** be used as a color attachment that supports blending.
        COLOR_ATTACHMENT_BLEND = vk::FormatFeatureFlags2::COLOR_ATTACHMENT_BLEND.as_raw(),
        /// Specifies that an image view **can** be used as a depth/stencil attachment and as
        /// an input attachment.
        DEPTH_STENCIL_ATTACHMENT = vk::FormatFeatureFlags2::DEPTH_STENCIL_ATTACHMENT.as_raw(),
        /// Specifies an image **can** be used as the source of a blitting.
        BLIT_SRC = vk::FormatFeatureFlags2::BLIT_SRC.as_raw(),
        /// Specifies an image **can** be used as the destination of a blitting.
        BLIT_DST = vk::FormatFeatureFlags2::BLIT_DST.as_raw(),
        /// Specifies that an image **can** be sampled from with a linear [`Filter`].
        SAMPLED_IMAGE_FILTER_LINEAR = vk::FormatFeatureFlags2::SAMPLED_IMAGE_FILTER_LINEAR.as_raw(),
        /// Specifies that an image **can** be used as the source image of copy commands.
        TRANSFER_SRC = vk::FormatFeatureFlags2::TRANSFER_SRC.as_raw(),
        /// Specifies that an image **can** be used as the destionation image of copy commands and
        /// clear commands.
        TRANSFER_DST = vk::FormatFeatureFlags2::TRANSFER_DST.as_raw(),
    }

    /// Bitmask specifying image resolve modes.
    #[default = Self::NONE]
    pub struct ResolveModes: Flags32 {
        /// Specifies that no resolve operation is done.
        NONE = vk::ResolveModeFlags::NONE.as_raw(),
        /// Specifies that result of the resolve operation is equal to the value of sample 0.
        SAMPLE_ZERO = vk::ResolveModeFlags::SAMPLE_ZERO.as_raw(),
        /// Specifies that result of the resolve operation is the average of the sample values.
        AVERAGE = vk::ResolveModeFlags::AVERAGE.as_raw(),
        /// Specifies that result of the resolve operation is the minimum of the sample values.
        MIN = vk::ResolveModeFlags::MIN.as_raw(),
        /// Specifies that result of the resolve operation is the maximum of the sample values.
        MAX = vk::ResolveModeFlags::MAX.as_raw(),
    }

    /// Bitmask controlling which components are written to the framebuffer.
    #[default = Self::RGBA]
    pub struct ColorComponents: Flags32 {
        /// Specifies that the R component is written to the framebuffer.
        R = vk::ColorComponentFlags::R.as_raw(),
        /// Specifies that the G component is written to the framebuffer.
        G = vk::ColorComponentFlags::G.as_raw(),
        /// Specifies that the B component is written to the framebuffer.
        B = vk::ColorComponentFlags::B.as_raw(),
        /// Specifies that the A component is written to the framebuffer.
        A = vk::ColorComponentFlags::A.as_raw(),
        /// Specifies that all components are written to the framebuffer.
        RGBA =
            Self::R.as_raw() |
            Self::G.as_raw() |
            Self::B.as_raw() |
            Self::A.as_raw(),
    }

    /// A bitmask specifying capabilities of queues in a [`queue family`][1].
    ///
    /// [1]: QueueFamilyProperties
    pub struct QueueFlags: Flags32 {
        /// Specifies that the queue supports graphics operations.
        ///
        /// # Supports commands
        /// - [`CopyCommands`]
        /// - [`GraphicsCommands`]
        /// - [`DrawCommands`]
        GRAPHICS = vk::QueueFlags::GRAPHICS.as_raw(),
        /// Specifies that the queue supports compute operations.
        ///
        /// # Supports commands
        /// - [`ComputeCommands`]
        COMPUTE = vk::QueueFlags::COMPUTE.as_raw(),
    }
}

impl ImageAspects {

    /// Returns the nth plane of this [`image aspect`][1].
    ///
    /// Returns [`None`] if the mask is not a single plane aspect.
    ///
    /// [1]: ImageAspects
    pub fn plane(self) -> Option<u32> {
        if self == Self::PLANE_0 {
            Some(0)
        } else if self == Self::PLANE_1 {
            Some(1)
        } else if self == Self::PLANE_2 {
            Some(2)
        } else {
            None
        }
    }
}

vk_enum! {
    /// Specifies the type of a gpu.
    pub enum PhysicalDeviceType: i32 {
        /// Specifies that the device doesn't match any available types.
        #[display("other")]
        Other = vk::PhysicalDeviceType::OTHER,
        /// Specifies an integrated gpu.
        #[display("integrated gpu")]
        IntegratedGpu = vk::PhysicalDeviceType::INTEGRATED_GPU,
        /// Specifies a discrete gpu.
        #[display("discrete gpu")]
        DiscreteGpu = vk::PhysicalDeviceType::DISCRETE_GPU,
        /// Specifies a virtual gpu.
        #[display("virtual gpu")]
        VirtualGpu = vk::PhysicalDeviceType::VIRTUAL_GPU,
        /// Specifies a cpu instead of a gpu.
        #[display("cpu")]
        Cpu = vk::PhysicalDeviceType::CPU,
    }

    /// Specifies how a component is swizzled.
    ///
    /// Default value is [`ComponentSwizzle::IDENTITY`].
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkComponentSwizzle.html>
    #[derive(Default)]
    pub enum ComponentSwizzle: i32 {
        /// Specifies that the component is set to the identity swizzle.
        #[display("identity")]
        #[default]
        Identity = vk::ComponentSwizzle::IDENTITY,
        /// Specifies that the component is set to zero.
        #[display("zero")]
        Zero = vk::ComponentSwizzle::ZERO,
        /// Specifies that the component is set to 1 or 1.0, depending on whether the format of the
        /// image view is an integer or floating-point format.
        #[display("one")]
        One = vk::ComponentSwizzle::ONE,
        /// Specifies that the component is set to the value of the R component.
        #[display("r")]
        R = vk::ComponentSwizzle::R,
        /// Specifies that the component is set to the value of the G component.
        #[display("g")]
        G = vk::ComponentSwizzle::G,
        /// Specifies that the component is set to the value of the B component.
        #[display("b")]
        B = vk::ComponentSwizzle::B,
        /// Specifies that the component is set to the value of the A component.
        #[display("a")]
        A = vk::ComponentSwizzle::A,
    }
    /// Specifies filters used for texture lookups.
    ///
    /// Default value is [`Filter::NEAREST`].
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFilter.html>
    #[derive(Default)]
    pub enum Filter: i32 {
        /// Specifies nearest filtering.
        #[display("nearest")]
        #[default]
        Nearest = vk::Filter::NEAREST,
        /// Specifies linear filtering.
        #[display("linear")]
        Linear = vk::Filter::LINEAR,
        /// Províded by [`VK_EXT_filter_cubic`].
        #[display("cubic ext")]
        CubicExt = vk::Filter::CUBIC_EXT,
    }

    /// Specifies mipmap mode used for texture lookups.
    ///
    /// Default value is [`MipmapMode::NEAREST`].
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerMipmapMode.html>
    #[derive(Default)]
    pub enum MipmapMode: i32 {
        /// Specifies nearest filtering.
        #[display("nearest")]
        #[default]
        Nearest = vk::SamplerMipmapMode::NEAREST,
        /// Specifies linear filtering.
        #[display("linear")]
        Linear = vk::SamplerMipmapMode::LINEAR,
    }

    /// Specifies behaviour of sampling with texture coordinates outside an image.
    ///
    /// Default value is [`SamplerAddressMode::REPEAT`].
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerAddressMode.html>
    #[derive(Default)]
    pub enum SamplerAddressMode: i32 {
        /// Specifies repeat wrap mode.
        #[display("repeat")]
        #[default]
        Repeat = vk::SamplerAddressMode::REPEAT,
        /// Specifies mirrored repeat wrap mode.
        #[display("mirrored repeat")]
        MirroredRepeat = vk::SamplerAddressMode::MIRRORED_REPEAT,
        /// Specifies clamp to edge wrap mode.
        #[display("clamp to edge")]
        ClampToEdge = vk::SamplerAddressMode::CLAMP_TO_EDGE,
        /// Specifies clamp to border wrap mode.
        #[display("clamp to border")]
        ClampToBorder = vk::SamplerAddressMode::CLAMP_TO_BORDER,
        /// Specifies mirror clamp to edge wrap mode.
        ///
        /// # Valid usage
        /// - The [`sampler_mirror_clamp_to_edge`][1] device feature **must** be enabled.
        ///
        /// [1]: Vulkan12DeviceFeatures::sampler_mirror_clamp_to_edge
        #[display("mirror clamp to edge")]
        MirrorClampToEdge = vk::SamplerAddressMode::MIRROR_CLAMP_TO_EDGE,
    }

    /// Specifies the border color used for texture lookup.
    ///
    /// Default value is [`FLOAT_TRANSPARENT_BLACK`][1].
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBorderColor.html>
    ///
    /// [1]: Self::FLOAT_TRANSPARENT_BLACK
    #[derive(Default)]
    pub enum BorderColor: i32 {
        /// Specifies a transparent, floating-point format, black color.
        #[display("float transparent black")]
        #[default]
        FloatTransparentBlack = vk::BorderColor::FLOAT_TRANSPARENT_BLACK,
        /// Specifies a transparent, integer format, black color.
        #[display("int transparent black")]
        IntTransparentBlack = vk::BorderColor::INT_TRANSPARENT_BLACK,
        /// Specifies an opaque, floating-point format, black color.
        #[display("float opaque black")]
        FloatOpaqueBlack = vk::BorderColor::FLOAT_OPAQUE_BLACK,
        /// Specifies an opaque, integer format, black color.
        #[display("int opaque black")]
        IntOpaqueBlack = vk::BorderColor::INT_OPAQUE_BLACK,
        /// Specifies an opaque, floating-point format, white color.
        #[display("float opaque white")]
        FloatOpaqueWhite = vk::BorderColor::FLOAT_OPAQUE_WHITE,
        /// Specifies an opaque, integer format, white color.
        #[display("int opaque white")]
        IntOpaqueWhite = vk::BorderColor::INT_OPAQUE_WHITE,
        /// Specifies that a [`vk::SamplerCustomBorderColorCreateInfoEXT`] structure is included in the
        /// [`p_next chain`][1] containing the color data in floating-point format.
        ///
        /// [1]: SamplerAttributes::with_p_next
        #[display("float custom ext")]
        FloatCustomExt = vk::BorderColor::FLOAT_CUSTOM_EXT,
        /// Specifies that a [`vk::SamplerCustomBorderColorCreateInfoEXT`] structure is included in the
        /// [`p_next chain`][1] containing the color data in integer format.
        ///
        /// [1]: SamplerAttributes::with_p_next
        #[display("float custom ext")]
        IntCustomExt = vk::BorderColor::INT_CUSTOM_EXT,
    }

    /// Specifies comparison operator for depth, stencil and sampler operations.
    ///
    /// Default value is [`CompareOp::Never`].
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCompareOp.html>
    #[derive(Default)]
    pub enum CompareOp: i32 {
        /// Specifies that the comparison always evaluates `false`.
        #[display("never")]
        #[default]
        Never = vk::CompareOp::NEVER,
        /// Specifies that the comparison evaluates *reference* &lt; *test*.
        #[display("less")]
        Less = vk::CompareOp::LESS,
        /// Specifies that the comparison evaluates *reference* == *test*
        #[display("equal")]
        Equal = vk::CompareOp::EQUAL,
        /// Specifies that the comparison evaluates *reference* <= *test*
        #[display("less or equal")]
        LessOrEqual = vk::CompareOp::LESS_OR_EQUAL,
        /// Specifies that the comparison evaulates *reference* &gt; *test*
        #[display("greater")]
        Greater = vk::CompareOp::GREATER,
        /// Specifies that the comparison evaluates *reference* != *test*
        #[display("not equal")]
        NotEqual = vk::CompareOp::NOT_EQUAL,
        /// Specifies that the comparison evaluates *reference* >= *test*
        #[display("greater or equal")]
        GreaterOrEqual = vk::CompareOp::GREATER_OR_EQUAL,
        /// Specifies that the comparison always evaluates `true`.
        #[display("always")]
        Always = vk::CompareOp::ALWAYS,
    }

    /// Specifies the robustness of buffer accesses in a pipeline.
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessBufferBehavior.html>
    #[derive(Default)]
    pub enum PipelineRobustnessBufferBehavior: i32 {
        /// Specifies that out of bounds buffer accesses follow the behavior of robust buffer access
        /// features enabled for the device.
        #[display("device default")]
        #[default]
        DeviceDefault = vk::PipelineRobustnessBufferBehavior::DEVICE_DEFAULT,
        /// Specifies that buffer accesses **must** not be out of bounds.
        #[display("disabled")]
        Disabled = vk::PipelineRobustnessBufferBehavior::DISABLED,
        /// Specifies that bounds checks to shader buffers are performed.
        ///
        /// Out of bounds reads will either return zero values or values from the underlying
        /// [`DeviceMemory`] bound to the buffer, including bytes outside the buffer itself.
        ///
        /// Out of bounds writes will either be discarded, or write values to the underlying
        /// [`DeciceMemory`] bound to the buffer including outside the buffer's range.
        ///
        /// Atomic read-modify-write operations will behave the same as writes outside bounds,
        /// but will return *undefined* values.
        #[display("robust buffer access")]
        RobustBufferAccess = vk::PipelineRobustnessBufferBehavior::ROBUST_BUFFER_ACCESS,
        /// Specifies that stricter bounds checks to shader buffers are performed.
        ///
        /// Out of bounds reads will produce zero values (with some caveats described in the docs).
        ///
        /// Out of bounds writes will not modify any memory.
        ///
        /// Atomic read-modify-write operations will behave the same as writes outside bounds, but
        /// will return *undefined* values.
        #[display("robust buffer access 2")]
        RobustBufferAccess2 = vk::PipelineRobustnessBufferBehavior::ROBUST_BUFFER_ACCESS_2,
    }

    /// Specifies the robustness of image accesses in a pipeline.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessImageBehavior.html>
    #[derive(Default)]
    pub enum PipelineRobustnessImageBehavior: i32 {
        /// Specifies that out of bounds image accesses follow the behavior of robust image access features
        /// enabled for the device.
        #[display("device default")]
        #[default]
        DeviceDefault = vk::PipelineRobustnessImageBehavior::DEVICE_DEFAULT,
        /// Specifies that image accesses **must** not be out of bounds.
        #[display("disabled")]
        Disabled = vk::PipelineRobustnessImageBehavior::DISABLED,
        /// Specifies that out of bounds checks to shader images are performed.
        ///
        /// Out of bounds writes and atomic read-modify-write operations will not modify any
        /// memory.
        ///
        /// Reads, atomic read-modify-write operations, or fetches from images outside bounds will
        /// return zero values with (0,0,1) or (0,0,0) values inserted for missing G, B or A
        /// components based on the format.
        #[display("robust image access")]
        RobustImageAccess = vk::PipelineRobustnessImageBehavior::ROBUST_IMAGE_ACCESS,
        /// Specifies that out of bounds checks to shader images are performed.
        ///
        /// Out of bounds writes and atomic read-modify-write operations will not modify any
        /// memory.
        ///
        /// Reads, atomic read-modify-write operations, or fetches from images outside bounds will
        /// return zero values with (0,0,1) values inserted for missing G, B or A components based
        /// on the format.
        #[display("robust image access 2")]
        RobustImageAccess2 = vk::PipelineRobustnessImageBehavior::ROBUST_IMAGE_ACCESS_2,
    }

    /// Describes what parts of a pipeline can (and must) be dynamically changed.
    ///
    /// Doesn't include viewport or scissor since they are always enabled.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDynamicState.html>
    pub enum DynamicState: i32 {
        /// Specifies that the `line_width` **must** be dynamically set with [`set_line_width`][1].
        ///
        /// [1]: DrawPipelineCommands::set_line_width
        #[display("line width")]
        LineWidth = vk::DynamicState::LINE_WIDTH,
        /// Specifies that `depth_bias_constant_factor`, `depth_bias_clamp` and
        /// `depth_bias_slope_factor` **must** be dynamically set with [`set_depth_bias`][1].
        ///
        /// [1]: DrawPipelineCommands::set_depth_bias
        #[display("depth bias")]
        DepthBias = vk::DynamicState::DEPTH_BIAS,
        /// Specifies that `blend_constants` **must** be dynamically set with
        /// [`set_blend_constants`][1].
        ///
        /// [1]: DrawPipelineCommands::set_blend_constants
        #[display("blend constants")]
        BlendConstants = vk::DynamicState::BLEND_CONSTANTS,
        /// Specifies that `min_depth_bounds` and `max_depth_bounds` **must** be dynamically set
        /// with [`set_depth_bounds`][1].
        ///
        /// [1]: DrawPipelineCommands::set_depth_bounds
        #[display("depth bounds")]
        DepthBounds = vk::DynamicState::DEPTH_BOUNDS,
        /// Specifies stencil `compare_mask` **must** be dynamically set with
        /// [`set_stencil_compare_mask`][1] for both front and back [`StencilFaces`].
        ///
        /// [1]: DrawPipelineCommands::set_stencil_compare_mask
        #[display("stencil compare mask")]
        StencilCompareMask = vk::DynamicState::STENCIL_COMPARE_MASK,
        /// Specifies that stencil `write_mask` **must** be dynamically set with
        /// [`set_stencil_write_mask`][1] for both front and back [`StencilFaces`].
        ///
        /// [1]: DrawPipelineCommands::set_stencil_write_mask
        #[display("stencil write mask")]
        StencilWriteMask = vk::DynamicState::STENCIL_WRITE_MASK,
        /// Specifies that stencil `reference` **must** be dynamically set with
        /// [`set_stencil_reference`][1] for both front and back [`StencilFaces`].
        ///
        /// [1]: DrawPipelineCommands::set_stencil_reference
        #[display("stencil reference")]
        StencilReference = vk::DynamicState::STENCIL_REFERENCE,
        /// Specifies that `cull_mode` **must** be dynamically set with [`set_cull_mode`][1].
        ///
        /// [1]: DrawPipelineCommands::set_cull_mode
        #[display("cull mode")]
        CullMode = vk::DynamicState::CULL_MODE,
        /// Specifies that `front_face` **must** be dynamically set with [`set_front_face`][1].
        ///
        /// [1]: DrawPipelineCommands::set_front_face
        #[display("front face")]
        FrontFace = vk::DynamicState::FRONT_FACE,
        /// Specifies that primitive `topology` **must** be dynamically set with
        /// [`set_primitive_topology`][1].
        ///
        /// [1]: DrawPipelineCommands::set_primitive_topology
        #[display("primitive topology")]
        PrimitiveTopology = vk::DynamicState::PRIMITIVE_TOPOLOGY,
        /// Specifies that vertex input stride **must** be specified when 
        /// [`binding vertex buffers`][1].
        ///
        /// [1]: DrawPipelineCommands::begin_drawing
        #[display("vertex input binding stride")]
        VertexInputBindingStride = vk::DynamicState::VERTEX_INPUT_BINDING_STRIDE,
        /// Specifies that `depth_test_enable` **must** be dynamically set with
        /// [`set_depth_test_enable`][1].
        ///
        /// [1]: DrawPipelineCommands::set_depth_test_enable
        #[display("depth test enable")]
        DepthTestEnable =  vk::DynamicState::DEPTH_TEST_ENABLE,
        /// Specifies that `depth_write_enable` **must** be dynamically set with
        /// [`set_depth_write_enable`][1].
        ///
        /// [1]: DrawPipelineCommands::set_depth_write_enable
        #[display("depth write enable")]
        DepthWriteEnable = vk::DynamicState::DEPTH_WRITE_ENABLE,
        /// Specifies that `depth_compare_op` **must** be dynamically set with
        /// [`set_depth_compare_op`][1].
        ///
        /// [1]: DrawPipelineCommands::set_depth_compare_op
        #[display("depth compare op")]
        DepthCompareOp = vk::DynamicState::DEPTH_COMPARE_OP,
        /// Specifies that `depth_bounds_test_enable` **must** be dynamically set with
        /// [`set_depth_bounds_test_enable`][1].
        ///
        /// [1]: DrawPipelineCommands::set_depth_bounds_test_enable
        #[display("depth bounds test enable")]
        DepthBoundsTestEnable = vk::DynamicState::DEPTH_BOUNDS_TEST_ENABLE,
        /// Specifies that `stencil_test_enable` **must** be dynamically set with
        /// [`set_stencil_test_enable`][1].
        ///
        /// [1]: DrawPipelineCommands::set_stencil_test_enable
        #[display("stencil test enable")]
        StencilTestEnable = vk::DynamicState::STENCIL_TEST_ENABLE,
        /// Specifies that `fail_op`, `pass_op`, `depth_fail_op` and `compare_op` **must** be
        /// dynamically set with [`set_stencil_op`][1] for both front and back [`StencilFaces`].
        ///
        /// [1]: DrawPipelineCommands::set_stencil_op
        #[display("stencil op")]
        StencilOp = vk::DynamicState::STENCIL_OP,
        /// Specifies that `rasterizer_discard_enable` **must** be dynamically set with
        /// [`set_rasterizer_discard_enable`][1].
        ///
        /// # Valid usage
        /// - The [`extended_dynamic_state2`][2] extension **must** be enabled with the
        ///   [`extended_dynamic_state2`][3] feature enabled.
        ///
        /// [1]: DrawPipelineCommands::set_rasterizer_discard_enable
        /// [2]: ext::extended_dynamic_state2
        /// [3]: ext::extended_dynamic_state2::Features::extended_dynamic_state2
        #[display("rasterizer discard enable")]
        RasterizerDiscardEnable = vk::DynamicState::RASTERIZER_DISCARD_ENABLE,
        /// Specifies that `depth_bias_enable` **must** be dynamically set with
        /// [`set_depth_bias_enable`][1].
        ///
        /// # Valid usage
        /// - The [`extended_dynamic_state2`][2] extension **must** be enabled with the
        ///   [`extended_dynamic_state2`][3] feature enabled.
        ///
        /// [1]: DrawPipelineCommands::set_depth_bias_enable
        /// [2]: ext::extended_dynamic_state2
        /// [3]: ext::extended_dynamic_state2::Features::extended_dynamic_state2
        #[display("depth bias enable")]
        DepthBiasEnable = vk::DynamicState::DEPTH_BIAS_ENABLE,
        /// Specifies that `primitive_restart_enable` **must** be dynamically set with
        /// [`set_primitive_restart_enable`][1].
        ///
        /// # Valid usage
        /// - The [`extended_dynamic_state2`][2] extension **must** be enabled with the
        ///   [`extended_dynamic_state2`][3] feature enabled.
        ///
        /// [1]: DrawPipelineCommands::set_primitive_restart_enable
        /// [2]: ext::extended_dynamic_state2
        /// [3]: ext::extended_dynamic_state2::Features::extended_dynamic_state2
        #[display("primitive restart enable")]
        PrimitiveRestartEnable = vk::DynamicState::PRIMITIVE_RESTART_ENABLE,
        /// Provided by Vulkan 1.4.
        #[display("line stipple")]
        LineStipple = vk::DynamicState::LINE_STIPPLE,
        /// Provided by VK_NV_clip_space_w_scaling:
        #[display("viewport w scaling nv")]
        ViewportWScalingNv = vk::DynamicState::VIEWPORT_W_SCALING_NV,
        /// Provided by VK_EXT_discard_rectangles.
        #[display("discard rectangle ext")]
        DiscardRectangleExt = vk::DynamicState::DISCARD_RECTANGLE_EXT,
        /// Provided by VK_EXT_discard_rectangles.
        #[display("discard rectangle enable ext")]
        DiscardRectangleEnableExt = vk::DynamicState::DISCARD_RECTANGLE_ENABLE_EXT,
        /// Provided by VK_EXT_discard_rectangles.
        #[display("discard rectangle mode ext")]
        DiscardRectangleModeExt = vk::DynamicState::DISCARD_RECTANGLE_MODE_EXT,
        /// Provided by VK_EXT_sample_locations.
        #[display("sample locations ext")]
        SampleLocationsExt = vk::DynamicState::SAMPLE_LOCATIONS_EXT,
        /// Provided by VK_KHR_ray_tracing_pipeline.
        #[display("ray tracing pipeline stack size khr")]
        RayTracingPipelineStackSizeKhr = vk::DynamicState::RAY_TRACING_PIPELINE_STACK_SIZE_KHR,
        /// Provided by VK_NV_shading_rate_image.
        #[display("viewport shading rate palette nv")]
        ViewportShadingRatePaletteNv = vk::DynamicState::VIEWPORT_SHADING_RATE_PALETTE_NV,
        /// Provided by VK_NV_shading_rate_image.
        #[display("viewport coarse sample order nv")]
        ViewportCoarseSampleOrderNv = vk::DynamicState::VIEWPORT_COARSE_SAMPLE_ORDER_NV,
        /// Provided by VK_NV_scissor_exclusive.
        #[display("exclusive scissor nv")]
        ExclusiveScissorEnableNv = vk::DynamicState::EXCLUSIVE_SCISSOR_ENABLE_NV,
        /// Provided by VK_NV_scissor_exclusive.
        #[display("exclusive scissor nv")]
        ExclusiveScissorNv = vk::DynamicState::EXCLUSIVE_SCISSOR_NV,
        /// Provided by VK_KHR_fragment_shading_rate
        #[display("fragment shading rate khr")]
        FragmentShadingRateKhr = vk::DynamicState::FRAGMENT_SHADING_RATE_KHR,
        /// Provided VK_EXT_vertex_input_dynamic_state
        #[display("vertex input ext")]
        VertexInputExt = vk::DynamicState::VERTEX_INPUT_EXT,
        /// Specifies that `patch_control_points` **must** be dynamically set with
        #[display("patch control points ext")]
        PatchControlPointsExt = vk::DynamicState::PATCH_CONTROL_POINTS_EXT,
        /// Provided by [`VK_EXT_extended_dynamic_state2`][ext::extended_dynamic_state2].
        #[display("logic op ext")]
        LogicOpExt = vk::DynamicState::LOGIC_OP_EXT,
        /// Provided by VK_EXT_color_write_enable.
        #[display("color write enable ext")]
        ColorWriteEnableExt = vk::DynamicState::COLOR_WRITE_ENABLE_EXT,
        /// Specifies that `depth_clamp_enable` **must** be dynamically set with
        /// [`set_depth_clamp_enable_ext`][1].
        ///
        /// [1]: DrawPipelineCommands::set_depth_clamp_enable_ext
        #[display("depth clamp enable ext")]
        DepthClampEnableExt = vk::DynamicState::DEPTH_CLAMP_ENABLE_EXT,
        /// Specifies that `polygon_mode` **must** be dynamically set with
        /// [`set_polygon_mode_ext`][1].
        ///
        /// [1]: DrawPipelineCommands::set_polygon_mode_ext
        #[display("polygon mode ext")]
        PolygonModeExt = vk::DynamicState::POLYGON_MODE_EXT,
        /// Specifies that `alpha_to_coverage_enable` **must** be dynamically set with
        /// [`set_alpha_to_coverage_enable_ext`][1].
        ///
        /// [1]: DrawPipelineCommands::set_alpha_to_coverage_enable_ext
        #[display("alpha to coverage enable")]
        AlphaToCoverageEnableExt = vk::DynamicState::ALPHA_TO_COVERAGE_ENABLE_EXT,
    }

    /// Specifies polygon front-facing orientation.
    ///
    /// The default value is [`counter clockwise`][1].
    /// 
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFrontFace.html>
    ///
    /// [1]: Self::COUNTER_CLOCKWISE
    #[derive(Default)]
    pub enum FrontFace: i32 {
        /// Specifies that triangles with positive area are considered front-facing.
        #[display("counter clockwise")]
        #[default]
        CounterClockwise = vk::FrontFace::COUNTER_CLOCKWISE,
        /// Specifies that triangles with negative area are considered front-facing.
        #[display("clockwise")]
        ClockWise = vk::FrontFace::CLOCKWISE,
    }

    /// Specifies primitive topology.
    ///
    /// The default is [`triangle list`][1].
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPrimitiveTopology.html>
    ///
    /// [1]: Self::TRIANGLE_LIST
    #[derive(Default)]
    pub enum PrimitiveTopology: i32 {
        /// Specifies a series of separate point primities.
        #[display("point list")]
        PointList = vk::PrimitiveTopology::POINT_LIST,
        /// Specifies a series of separate line primities.
        #[display("line list")]
        LineList = vk::PrimitiveTopology::LINE_LIST,
        /// Specifies a series of connected line primities.
        #[display("line strip")]
        LineStrip = vk::PrimitiveTopology::LINE_STRIP,
        /// Specifies a series of separate triangle primities.
        #[display("triangle list")]
        #[default]
        TriangleList = vk::PrimitiveTopology::TRIANGLE_LIST,
        /// Specifies a series of connected triangle primities.
        #[display("triangle strip")]
        TriangleStrip = vk::PrimitiveTopology::TRIANGLE_STRIP,
        /// Specifies a series of connected triangle primitives with all triangles sharing a common
        /// vertex.
        #[display("triangle fan")]
        TriangleFan = vk::PrimitiveTopology::TRIANGLE_FAN,
        /// Specifies a series of separate line primitives with adjacency.
        #[display("line list with adjacency")]
        LineListWithAdjacency = vk::PrimitiveTopology::LINE_LIST_WITH_ADJACENCY,
        /// Specifies a series of connected line primitives with adjacency, with consecutive
        /// primitives sharing three vertices.
        #[display("line strip with adjacency")]
        LineStripWithAdjacency = vk::PrimitiveTopology::LINE_STRIP_WITH_ADJACENCY,
        /// Specifies a series of separate triangle primitives with adjacency.
        #[display("triangle list with adjacency")]
        TriangleListWithAdjacency = vk::PrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY,
        /// Specifies connected triangle primitives with adjacency, with consecutive triangles sharing an edge.
        #[display("triangle strip with adjacency")]
        TriangleStripWithAdjacency = vk::PrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY,
        /// Specifies separate patch primitives.
        #[display("patch list")]
        PatchList = vk::PrimitiveTopology::PATCH_LIST,
    }


    /// Stencil comparison function.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilOp.html>
    pub enum StencilOp: i32 {
        /// Keeps the current value.
        #[display("keep")]
        Keep = vk::StencilOp::KEEP,
        /// Sets the value to 0.
        #[display("zero")]
        Zero = vk::StencilOp::ZERO,
        /// Sets the value to the reference.
        #[display("replace")]
        Replace = vk::StencilOp::REPLACE,
        /// Increments the current value, saturating at the maximum representable unsigned value.
        #[display("saturating increment")]
        SaturatingIncrement = vk::StencilOp::INCREMENT_AND_CLAMP,
        /// Increments the current value, saturating at 0.
        #[display("saturating decrement")]
        SaturatingDecrement = vk::StencilOp::DECREMENT_AND_CLAMP,
        /// Bitwise-inverts the current value.
        #[display("invert")]
        Invert = vk::StencilOp::INVERT,
        /// Increments the current value, wrapping around at the maximum representable unsigned
        /// value.
        #[display("wrapping increment")]
        WrappingIncrement = vk::StencilOp::INCREMENT_AND_WRAP,
        /// Decrements the current value, wrapping around at 0.
        #[display("wrapping decrement")]
        WrappingDecrement = vk::StencilOp::DECREMENT_AND_WRAP,
    }

    /// Specifies polygon rasterization mode.
    ///
    /// The default value is ['fill`][1].
    ///
    /// [1]: Self::FILL
    #[derive(Default)]
    pub enum PolygonMode: i32 {
        /// Specifies fill mode.
        #[display("fill")]
        #[default]
        Fill = vk::PolygonMode::FILL,
        /// Specifies that polygon edges are drawn as line segments.
        #[display("line")]
        Line = vk::PolygonMode::LINE,
        /// Specifies that polygon vertices are drawn as points.
        #[display("point")]
        Point = vk::PolygonMode::POINT,
        /// Provided by VK_NV_fill_rectangle.
        #[display("fille rectangel nv")]
        FillRectangleNv = vk::PolygonMode::FILL_RECTANGLE_NV,
    }

    /// Specifies framebuffer blending factors.
    pub enum BlendFactor: i32 {
        /// RGB: (0, 0, 0)
        /// A: 0
        #[display("zero")]
        Zero = vk::BlendFactor::ZERO,
        /// RGB: (1, 1, 1)
        /// A: 1
        #[display("one")]
        One = vk::BlendFactor::ONE,
        /// RGB: (R0, G0, B0)
        /// A: A0
        #[display("source color")]
        SrcColor = vk::BlendFactor::SRC_COLOR,
        /// RGB: (1 - R0, 1 - G0, 1 - B0)
        /// A: 1 - A0
        #[display("one minus source color")]
        OneMinusSrcColor = vk::BlendFactor::ONE_MINUS_SRC_COLOR,
        /// RGB: (R1, G1, B1)
        /// A: A1
        #[display("destination color")]
        DstColor = vk::BlendFactor::DST_COLOR,
        /// RGB: (1 - R1, 1 - R1, 1 - R1)
        /// A: 1 - A1
        #[display("one minus destination color")]
        OneMinusDstColor = vk::BlendFactor::ONE_MINUS_DST_COLOR,
        /// RGB: (A0, A0, A0)
        /// A: A0
        #[display("source alpha")]
        SrcAlpha = vk::BlendFactor::SRC_ALPHA,
        /// RGB: (1 - A0, 1 - A0, 1 - A0)
        /// A: 1 - A0
        #[display("one minus source alpha")]
        OneMinusSrcAlpha = vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        /// RGB: (A1, A1, A1)
        /// A: A1
        #[display("destination alpha")]
        DstAlpha = vk::BlendFactor::DST_ALPHA,
        /// RGB: (1 - A1, 1 - A1, 1 - A1)
        /// A: 1 - A1
        #[display("one minus destination alpha")]
        OneMinusDstAlpha = vk::BlendFactor::ONE_MINUS_DST_ALPHA,
        /// RGB: (Rc, Gc, Bc)
        /// A: Ac
        #[display("const color")]
        ConstColor = vk::BlendFactor::CONSTANT_COLOR,
        /// RGB: (1 - Rc, 1 - Gc, 1 - Bc)
        /// A: 1 - Ac
        #[display("one minus const color")]
        OneMinusConstColor = vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR,
        /// RGB: (Ac, Ac, A )
        /// A: Ac
        #[display("const alpha")]
        ConstAlpha = vk::BlendFactor::CONSTANT_ALPHA,
        /// RGB: (1 - Ac, 1 - Ac, 1 - Ac)
        /// A: 1 - Ac
        #[display("one minus const alpha")]
        OneMinusConstAlpha = vk::BlendFactor::ONE_MINUS_CONSTANT_ALPHA,
        /// RGB: (f,f,f); f = min(A0,1-A1)
        /// A: 1
        #[display("src alpha saturate")]
        SrcAlphaSaturate = vk::BlendFactor::SRC_ALPHA_SATURATE,
        /// RGB: (R01, G01, B01)
        /// A: A01
        #[display("src1 color")]
        Src1Color = vk::BlendFactor::SRC1_COLOR,
        /// RGB: (1 - R01, 1 - G01, 1 - B01)
        /// A: 1 - A01
        #[display("one minus src1 color")]
        OneMinusSrc1Color = vk::BlendFactor::ONE_MINUS_SRC1_COLOR,
        /// RGB: (A01, A01, A01)
        /// A: A01
        #[display("src1 alpha")]
        Src1Alpha = vk::BlendFactor::SRC1_ALPHA,
        /// RGB: (1 - A01, 1 - A01, 1 - A01)
        /// A: 1- A01
        #[display("oe minus src1 alpha")]
        OneMinusSrc1Alpha = vk::BlendFactor::ONE_MINUS_SRC1_ALPHA,
    }

    /// Specifies framebuffer blending operations
    pub enum BlendOp : i32 {
        /// See the [`Vulkan docs`][1].
        ///
        /// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkBlendOp.html
        #[display("add")]
        Add = vk::BlendOp::ADD,
        /// See the [`Vulkan docs`][1].
        ///
        /// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkBlendOp.html
        #[display("subtract")]
        Sub = vk::BlendOp::SUBTRACT,
        /// See the [`Vulkan docs`][1].
        ///
        /// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkBlendOp.html
        #[display("reverse subtract")]
        RevSub = vk::BlendOp::REVERSE_SUBTRACT,
        /// See the [`Vulkan docs`][1].
        ///
        /// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkBlendOp.html
        #[display("min")]
        Min = vk::BlendOp::MIN,
        /// See the [`Vulkan docs`][1].
        ///
        /// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkBlendOp.html
        #[display("max")]
        Max = vk::BlendOp::MAX,
    }
    
    /// Specifies a logical operation in a framebuffer.
    pub enum LogicOp: i32 {
        /// 0
        #[display("clear")]
        Clear = vk::LogicOp::CLEAR,
        /// s & d
        #[display("and")]
        And = vk::LogicOp::AND,
        /// s ^ !d
        #[display("and reverse")]
        AndReverse = vk::LogicOp::AND_REVERSE,
        /// s
        #[display("copy")]
        Copy = vk::LogicOp::COPY,
        /// !s & d
        #[display("and inverted")]
        AndInverted = vk::LogicOp::AND_INVERTED,
        /// d
        #[display("no op")]
        NoOp = vk::LogicOp::NO_OP,
        /// s ^ d
        #[display("xor")]
        Xor = vk::LogicOp::XOR,
        /// s | d
        #[display("or")]
        Or = vk::LogicOp::OR,
        /// !(s | d)
        #[display("nor")]
        Nor = vk::LogicOp::NOR,
        /// !(s ^ d)
        #[display("equivalent")]
        Equivalent = vk::LogicOp::EQUIVALENT,
        /// !d
        #[display("invert")]
        Invert = vk::LogicOp::INVERT,
        /// s | !d
        #[display("or reverse")]
        OrReverse = vk::LogicOp::OR_REVERSE,
        /// !d
        #[display("copy inverted")]
        CopyInverted = vk::LogicOp::COPY_INVERTED,
        /// ! (s & d)
        #[display("nand")]
        Nand = vk::LogicOp::NAND,
        /// All 1s
        #[display("set")]
        Set = vk::LogicOp::SET,
    }
}

impl From<vk::PhysicalDeviceType> for PhysicalDeviceType {

    #[inline(always)]
    fn from(value: vk::PhysicalDeviceType) -> Self {
        unsafe {
            Self::from_raw(value.as_raw())
        }
    }
}

/// An enumeration of [`primitive topology classes`][PrimitiveTopology::class].
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveTopologyClass {
    /// Primitives are points.
    Point,
    /// Primitives are lines.
    Line,
    /// Primitives are triangles
    Triangle,
    /// Primitives are patches.
    Patch,
}

impl PrimitiveTopology {

    /// Returns whether this topology type can [`restart`][1].
    ///
    /// [1]: DrawPipelineCommands::set_primitive_restart_enable
    #[inline]
    pub fn can_restart(self) -> bool {
        matches!(self,
            Self::LINE_STRIP | Self::TRIANGLE_STRIP |
            Self::LINE_STRIP_WITH_ADJACENCY  | Self::TRIANGLE_STRIP_WITH_ADJACENCY
        )
    }

    /// Gets the topology class of self.
    #[inline]
    pub const fn class(self) -> PrimitiveTopologyClass {
        match self {
            Self::POINT_LIST => PrimitiveTopologyClass::Point,
            Self::LINE_LIST |
            Self::LINE_STRIP |
            Self::LINE_LIST_WITH_ADJACENCY |
            Self::LINE_STRIP_WITH_ADJACENCY => PrimitiveTopologyClass::Line,
            Self::TRIANGLE_LIST |
            Self::TRIANGLE_STRIP |
            Self::TRIANGLE_FAN |
            Self::TRIANGLE_LIST_WITH_ADJACENCY |
            Self::TRIANGLE_STRIP_WITH_ADJACENCY => PrimitiveTopologyClass::Triangle,
            Self::PATCH_LIST => PrimitiveTopologyClass::Patch,
            _ => PrimitiveTopologyClass::Point,
        }
    }
}

/// Specifies the type of indices in an index buffer.
///
/// The default value is [`U32`][1].
///
/// # Vulkan docs
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndexType.html>
///
/// [1]: Self::U32
#[repr(i32)]
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum IndexType {
    /// Specifies indices that are 8-bit unsigned integer values.
    ///
    /// Note that this requires enabling [`ext::index_type_uint8`] device extension.
    U8 = vk::IndexType::UINT8.as_raw(),
    /// Specifies indices that are 16-bit unsigned integer values.
    U16 = vk::IndexType::UINT16.as_raw(),
    /// Specifies indices that are 32-bit unsigned integer values.
    #[default]
    U32 = vk::IndexType::UINT32.as_raw(),
}

impl IndexType {
    
    /// Gets the underlying value of this [`IndexType`].
    #[inline]
    pub fn as_raw(self) -> i32 {
        self as i32
    }

    /// Returns the index size of this [`IndexType`].
    #[inline]
    pub fn index_size(self) -> DeviceSize {
        match self {
            Self::U8 => 1,
            Self::U16 => 2,
            Self::U32 => 4,
        }
    }
}

impl Display for IndexType {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::U8 => write!(f, "u8"),
            Self::U16 => write!(f, "u16"),
            Self::U32 => write!(f, "u32"),
        }
    }
}

/// Specifies the type of an [`image`][1].
///
/// [1]: ImageCreateInfo
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ImageType {
    /// Specifies that the type is inferred from the image's [`dimensions`][1].
    ///
    /// [1]: ImageCreateInfo::with_dimensions
    #[default]
    Infer,
    /// Specifies a one-dimensional image.
    ///
    /// # Valid usage
    /// - The [`height and depth`][1] of the image **must** both be equal to 1.
    ///
    /// [1]: ImageCreateInfo::with_dimensions
    Type1d,
    /// Specifies a two-dimensional image.
    ///
    /// # Valid usage
    /// - The [`depth`][1] of the image **must** be equal to 1.
    ///
    /// [1]: ImageCreateInfo::with_dimensions
    Type2d,
    /// Specifies a three-dimensional image.
    Type3d,
}

impl Display for ImageType {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Infer => write!(f, "infer"),
            Self::Type1d => write!(f, "1d"),
            Self::Type2d => write!(f, "2d"),
            Self::Type3d => write!(f, "3d"),
        }
    }
}

/// Specifies the type of an [`image view`][1].
///
/// [1]: ImageViewCreateInfo::view_type
#[repr(i32)]
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ImageViewType {
    /// Specifies that the type is inferred from the image's [`type`][1].
    ///
    /// [1]: ImageType
    #[default]
    Infer = i32::MIN,
    /// Specifies a one-dimensional view.
    Type1d = vk::ImageViewType::TYPE_1D.as_raw(),
    /// Specifies a two-dimensional view.
    Type2d = vk::ImageViewType::TYPE_2D.as_raw(),
    /// Specifies a three-dimensional view.
    Type3d = vk::ImageViewType::TYPE_3D.as_raw(),
    /// Specifies a cube view.
    /// 
    /// # Valid usage
    /// - The image **must** be [`cube compatible`][1].
    /// - The [`array layers`][2] of the created view **must** be 6.
    ///
    /// [1]: ImageCreateFlags::CUBE_COMPATIBLE
    /// [2]: ImageViewCreateInfo::subresource_range
    Cube = vk::ImageViewType::CUBE.as_raw(),
    /// Specifies an arrayed one-dimensional view.
    Type1dArray = vk::ImageViewType::TYPE_1D_ARRAY.as_raw(),
    /// Specifies an arrayed two-dimensional view.
    Type2dArray = vk::ImageViewType::TYPE_2D_ARRAY.as_raw(),
    /// Specifies an arrayed cube view.
    ///
    /// # Valid usage
    /// - The image **must** be [`cube compatible`][1].
    /// - The [`array layers`][2] of the created view **must** be a multiple of 6.
    ///
    /// [1]: ImageCreateFlags::CUBE_COMPATIBLE
    /// [2]: ImageViewCreateInfo::subresource_range
    CubeArray = vk::ImageViewType::CUBE_ARRAY.as_raw(),
}

impl ImageViewType {

    #[inline]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    /// Returns whether the view type is compatible given an [`image type`][1] and image
    /// [`create flags`][2]
    ///
    /// [1]: ImageType
    /// [2]: ImageCreateFlags
    #[inline]
    pub fn is_compatible(
        self,
        image_type: ImageType,
        create_flags: ImageCreateFlags,
    ) -> bool {
        match self {
            Self::Infer => true,
            Self::Type1d | Self::Type1dArray
                => matches!(image_type, ImageType::Type1d),
            Self::Type2d => matches!(image_type, ImageType::Type2d),
            Self::Type2dArray =>
                matches!(image_type, ImageType::Type2d) ||
                (create_flags.contains(ImageCreateFlags::TYPE_2D_ARRAY_COMPATIBLE) &&
                    matches!(image_type, ImageType::Type3d)
                ),
            Self::Cube | Self::CubeArray => matches!(image_type, ImageType::Type2d),
            Self::Type3d => matches!(image_type, ImageType::Type3d),
        }
    }
}

macro_rules! impl_convert_vk {
    ($([$name:ident, vk::$vk:ident]),+ $(,)?) => {
        $(
            impl From<$name> for vk::$vk {

                #[inline(always)]
                fn from(value: $name) -> Self {
                    Self::from_raw(value.as_raw())
                }
            }
        )+
    };
}

impl_convert_vk! {
    [MsaaSamples, vk::SampleCountFlags],
    [BufferUsages, vk::BufferUsageFlags],
    [ImageUsages, vk::ImageUsageFlags],
    [ImageAspects, vk::ImageAspectFlags],
    [ComponentSwizzle, vk::ComponentSwizzle],
    [Filter, vk::Filter],
    [MipmapMode, vk::SamplerMipmapMode],
    [SamplerAddressMode, vk::SamplerAddressMode],
    [BorderColor, vk::BorderColor],
    [BlendFactor, vk::BlendFactor],
    [BlendOp, vk::BlendOp],
    [CompareOp, vk::CompareOp],
    [StencilFaces, vk::StencilFaceFlags],
    [StencilOp, vk::StencilOp],
    [PolygonMode, vk::PolygonMode],
    [DynamicState, vk::DynamicState],
    [FrontFace, vk::FrontFace],
    [PrimitiveTopology, vk::PrimitiveTopology],
    [IndexType, vk::IndexType],
    [PipelineRobustnessBufferBehavior, vk::PipelineRobustnessBufferBehavior],
    [PipelineRobustnessImageBehavior, vk::PipelineRobustnessImageBehavior],
    [ResolveModes, vk::ResolveModeFlags],
    [ColorComponents, vk::ColorComponentFlags],
    [CullModes, vk::CullModeFlags],
    [LogicOp, vk::LogicOp],
    [ImageViewType, vk::ImageViewType],
}

impl From<vk::SampleCountFlags> for MsaaSamples {

    #[inline(always)]
    fn from(value: vk::SampleCountFlags) -> Self {
        Self::from_raw(value.as_raw())
    }
}

/// Specifies a resolve aspect.
#[derive(Clone, Copy, Debug)]
pub enum ResolveAspect {
    /// Specifies color resolve.
    Color,
    /// Specifies depth resolve.
    Depth,
    /// Specifies stencil resolve.
    Stencil,
}

impl Display for ResolveAspect {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Color => write!(f, "color"),
            Self::Depth => write!(f, "depth"),
            Self::Stencil => write!(f, "stencil"),
        }
    }
}

impl Format {
    
    /// Gets the underlying value of this [`Format`].
    #[inline(always)]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    /// Returns whether self is [`compatible`][1] with `other`.
    ///
    /// [1]: FormatCompatibility
    #[inline(always)]
    pub fn is_compatible_with(self, other: Self) -> bool {
        self.compatibility() == other.compatibility()
    }

    /// Returns all [`ImageAspects`] of this [`Format`].
    #[inline(always)]
    pub fn aspects(self) -> ImageAspects {
        let plane_count = self.plane_count();
        if plane_count == 3 {
            return 
                ImageAspects::PLANE_0 |
                ImageAspects::PLANE_1 |
                ImageAspects::PLANE_2
        }
        if plane_count == 2 {
            return
                ImageAspects::PLANE_0 |
                ImageAspects::PLANE_1 
        }
        let info = self.info();
        if info.numeric_color.is_some() {
            return ImageAspects::COLOR
        }
        let is_depth = info.numeric_depth.is_some();
        let is_stencil = info.numeric_stencil.is_some();
        if is_depth && is_stencil {
            return ImageAspects::DEPTH | ImageAspects::STENCIL
        }
        if is_depth {
            return ImageAspects::DEPTH
        }
        if is_stencil {
            return ImageAspects::STENCIL
        }
        ImageAspects::empty()
    }

    /// Returns which single-plane formats are compatible with the relative plane index in the
    /// returned array.
    ///
    /// For 2-plane formats, the last index is [`Format::Undefined`].
    ///
    /// For single-plane formats, index 0 is [`self`] and indices 1 and 2 are [Format::Undefined`].
    #[inline(always)]
    pub fn plane_formats(self) -> [Self; 3] {
        match self {
            Self::G8_B8_R8_3plane_420_Unorm =>
                [Format::R8_Unorm; 3],
            Self::G8_B8r8_2plane_420_Unorm =>
                [Format::R8_Unorm, Format::R8g8_Unorm, Format::Undefined],
            Self::G8_B8_R8_3plane_422_Unorm =>
                [Format::R8_Unorm; 3],
            Self::G8_B8r8_2plane_422_Unorm =>
                [Format::R8_Unorm, Format::R8g8_Unorm, Format::Undefined],
            Self::G8_B8_R8_3plane_444_Unorm =>
                [Format::R8_Unorm; 3],
            Self::G8_B8r8_2plane_444_Unorm =>
                [Format::R8_Unorm, Format::R8g8_Unorm, Format::Undefined],
            Self::G10x6_B10x6_R10x6_3plane_420_Unorm_3pack16 =>
                [Format::R10x6_Unorm_Pack16; 3],
            Self::G10x6_B10x6r10x6_2plane_420_Unorm_3pack16 =>
                [
                    Format::R10x6_Unorm_Pack16,
                    Format::R10x6g10x6_Unorm_2pack16,
                    Format::Undefined,
                ],
            Self::G10x6_B10x6_R10x6_3plane_422_Unorm_3pack16 =>
                [Format::R10x6_Unorm_Pack16; 3],
            Self::G10x6_B10x6r10x6_2plane_422_Unorm_3pack16 =>
                [
                    Format::R10x6_Unorm_Pack16,
                    Format::R10x6g10x6_Unorm_2pack16,
                    Format::Undefined,
                ],
            Self::G10x6_B10x6_R10x6_3plane_444_Unorm_3pack16 =>
                [Format::R10x6_Unorm_Pack16; 3],
            Self::G10x6_B10x6r10x6_2plane_444_Unorm_3pack16 =>
                [
                    Format::R10x6_Unorm_Pack16,
                    Format::R10x6g10x6_Unorm_2pack16,
                    Format::Undefined,
                ],
            Self::G12x4_B12x4_R12x4_3plane_420_Unorm_3pack16 =>
                [Format::R12x4_Unorm_Pack16; 3],
            Self::G12x4_B12x4r12x4_2plane_420_Unorm_3pack16 =>
                [
                    Format::R12x4_Unorm_Pack16,
                    Format::R12x4g12x4_Unorm_2pack16,
                    Format::Undefined,
                ],
            Self::G12x4_B12x4_R12x4_3plane_422_Unorm_3pack16 =>
                [Format::R12x4_Unorm_Pack16; 3],
            Self::G12x4_B12x4r12x4_2plane_422_Unorm_3pack16 =>
                [
                    Format::R12x4_Unorm_Pack16,
                    Format::R12x4g12x4_Unorm_2pack16,
                    Format::Undefined,
                ],
            Self::G12x4_B12x4_R12x4_3plane_444_Unorm_3pack16 =>
                [Format::R12x4_Unorm_Pack16; 3],
            Self::G12x4_B12x4r12x4_2plane_444_Unorm_3pack16 =>
                [
                    Format::R12x4_Unorm_Pack16,
                    Format::R12x4g12x4_Unorm_2pack16,
                    Format::Undefined,
                ],
            Self::G16_B16_R16_3plane_420_Unorm =>
                [Format::R16_Unorm; 3],
            Self::G16_B16r16_2plane_420_Unorm =>
                [Format::R16_Unorm, Format::R16g16_Unorm, Format::Undefined],
            Self::G16_B16_R16_3plane_422_Unorm =>
                [Format::R16_Unorm; 3],
            Self::G16_B16r16_2plane_422_Unorm =>
                [Format::R16_Unorm, Format::R16g16_Unorm, Format::Undefined],
            Self::G16_B16_R16_3plane_444_Unorm =>
                [Format::R16_Unorm; 3],
            Self::G16_B16r16_2plane_444_Unorm =>
                [Format::R16_Unorm, Format::R16g16_Unorm, Format::Undefined],
            _ => [self, Format::Undefined, Format::Undefined],
        }
    }

    /// Returns supported [`FormatResolveModes`].
    #[inline(always)]
    pub fn resolve_modes(self) -> FormatResolveModes {
        let info = self.info();
        let is_depth = info.numeric_depth.is_some();
        let is_stencil = info.numeric_stencil.is_some();
        if is_depth && is_stencil {
            FormatResolveModes {
                depth:
                    ResolveModes::AVERAGE |
                    ResolveModes::MIN |
                    ResolveModes::MAX |
                    ResolveModes::SAMPLE_ZERO
                ,
                stencil:
                    ResolveModes::MIN |
                    ResolveModes::MAX |
                    ResolveModes::SAMPLE_ZERO,
                ..Default::default()
            } 
        } else if is_depth {
            FormatResolveModes {
                depth:
                    ResolveModes::AVERAGE |
                    ResolveModes::MIN |
                    ResolveModes::MAX |
                    ResolveModes::SAMPLE_ZERO
                ,
                ..Default::default()
            }
        } else if is_stencil {
            FormatResolveModes {
                stencil:
                    ResolveModes::MIN |
                    ResolveModes::MAX |
                    ResolveModes::SAMPLE_ZERO,
                ..Default::default()
            }
        } else if let Some(numeric_format) = info.numeric_color {
            if numeric_format.is_floating_point() {
                FormatResolveModes {
                    color: ResolveModes::AVERAGE,
                    ..Default::default()
                }
            } else {
                FormatResolveModes {
                    color: ResolveModes::SAMPLE_ZERO,
                    ..Default::default()
                }
            }
        } else {
            Default::default()
        }
    }
}

impl Default for Format {

    #[inline]
    fn default() -> Self {
        Self::Undefined
    }
}

impl From<Format> for vk::Format {

    #[inline(always)]
    fn from(value: Format) -> Self {
        Self::from_raw(value.as_raw())
    }
}

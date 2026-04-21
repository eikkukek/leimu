use core::{
    fmt::{self, Display},
    hash::Hash,
};

use tuhka::vk;

use crate::{bitflags, c_enum};

use crate::gpu::prelude::*;

pub type Flags32 = u32;
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
    /// Describes what a buffer *can* be used for.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits.html>
    #[default = Self::empty()]
    pub struct BufferUsages: Flags32 {
        /// Specifies that the buffer *can* be used as the source of transfer operations.
        TRANSFER_SRC = vk::BufferUsageFlags::TRANSFER_SRC.as_raw(),
        /// Specifies that the buffer *can* be used as the destination of transfer operations.
        TRANSFER_DST = vk::BufferUsageFlags::TRANSFER_DST.as_raw(), 
        /// Specifies that the buffer *can* be used as a uniform texel buffer.
        UNIFORM_TEXEL_BUFFER = vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER.as_raw(),
        /// Specifies that the buffer *can* be used as a storage texel buffer.
        STORAGE_TEXEL_BUFFER = vk::BufferUsageFlags::STORAGE_TEXEL_BUFFER.as_raw(),
        /// Specifies that the buffer *can* be used as a uniform buffer.
        UNIFORM_BUFFER = vk::BufferUsageFlags::UNIFORM_BUFFER.as_raw(),
        /// Specifies that the buffer *can* be used as a storage buffer.
        STORAGE_BUFFER = vk::BufferUsageFlags::STORAGE_BUFFER.as_raw(),
        /// Specifies that the buffer *can* be used as an index buffer.
        INDEX_BUFFER = vk::BufferUsageFlags::INDEX_BUFFER.as_raw(),
        /// Specifies that the buffer *can* be used as a vertex buffer.
        VERTEX_BUFFER = vk::BufferUsageFlags::VERTEX_BUFFER.as_raw(),
        /// Specifies that the buffer *can* be used as in indirect commands.
        INDIRECT_BUFFER = vk::BufferUsageFlags::INDIRECT_BUFFER.as_raw(),
    }
    /// Specifies what an [`Image`] can be used for.
    ///
    /// Default value is [`ImageUsages::empty()`].
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlagBits.html>
    #[default = Self::empty()]
    pub struct ImageUsages: Flags32 {
        /// Specifies that the image *can* be used as the source of transfer operations.
        TRANSFER_SRC = vk::ImageUsageFlags::TRANSFER_SRC.as_raw(),
        /// Specifies that the image *can* be used as the destination of transfer operations.
        TRANSFER_DST = vk::ImageUsageFlags::TRANSFER_DST.as_raw(),
        /// Specifies that the image *can* be used sampled from in a shader.
        SAMPLED = vk::ImageUsageFlags::SAMPLED.as_raw(),
        /// Specifies that the image *can* be used as a storage image in a shader.
        STORAGE = vk::ImageUsageFlags::STORAGE.as_raw(),
        /// Specifies that the image *can* be used as a color attachment in rendering.
        COLOR_ATTACHMENT = vk::ImageUsageFlags::COLOR_ATTACHMENT.as_raw(),
        /// Specifies that the image *can* be used as a depth/stencil attachment in rendering.
        DEPTH_STENCIL_ATTACHMENT = vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT.as_raw(),
        /// Specifies that the image *can* be used as an input attachment in rendering.
        INPUT_ATTACHMENT = vk::ImageUsageFlags::INPUT_ATTACHMENT.as_raw(),
    }

    /// Specifies which image aspect to use for e.g. [`ImageSubresourceRange`].
    ///
    /// Default value is [`ImageAspects::empty()`].
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageAspectFlagBits.html#>
    #[default = Self::empty()]
    pub struct ImageAspects: Flags32 {
        COLOR = vk::ImageAspectFlags::COLOR.as_raw(),
        DEPTH = vk::ImageAspectFlags::DEPTH.as_raw(),
        STENCIL = vk::ImageAspectFlags::STENCIL.as_raw(),
        PLANE_0 = vk::ImageAspectFlags::PLANE_0.as_raw(),
        PLANE_1 = vk::ImageAspectFlags::PLANE_1.as_raw(),
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
        /// Specifies that an image view *can* be sampled from.
        SAMPLED_IMAGE = vk::FormatFeatureFlags2::SAMPLED_IMAGE.as_raw(),
        /// Specifies that an image view *can* be used as a storage image.
        STORAGE_IMAGE = vk::FormatFeatureFlags2::STORAGE_IMAGE.as_raw(),
        /// Specifies that an image view *can* be used as a storage image that supports atomic
        /// operations.
        STORAGE_IMAGE_ATOMIC = vk::FormatFeatureFlags2::STORAGE_IMAGE_ATOMIC.as_raw(),
        /// Specifies that the format *can* be used as a vertex attribute format.
        VERTEX_BUFFER = vk::FormatFeatureFlags2::VERTEX_BUFFER.as_raw(),
        /// Specifies that an image view *can* be used as a color attachment.
        COLOR_ATTACHMENT = vk::FormatFeatureFlags2::COLOR_ATTACHMENT.as_raw(),
        /// Specifies that an image view *can* be used as a color attachment that supports blending.
        COLOR_ATTACHMENT_BLEND = vk::FormatFeatureFlags2::COLOR_ATTACHMENT_BLEND.as_raw(),
        /// Specifies that an image view *can* be used as a depth/stencil attachment and as
        /// an input attachment.
        DEPTH_STENCIL_ATTACHMENT = vk::FormatFeatureFlags2::DEPTH_STENCIL_ATTACHMENT.as_raw(),
        /// Specifies an image *can* be used as the source of a blitting.
        BLIT_SRC = vk::FormatFeatureFlags2::BLIT_SRC.as_raw(),
        /// Specifies an image *can* be used as the destination of a blitting.
        BLIT_DST = vk::FormatFeatureFlags2::BLIT_DST.as_raw(),
        /// Specifies that an image *can* be sampled from with a linear [`Filter`].
        SAMPLED_IMAGE_FILTER_LINEAR = vk::FormatFeatureFlags2::SAMPLED_IMAGE_FILTER_LINEAR.as_raw(),
        /// Specifies that an image *can* be used as the source image of copy commands.
        TRANSFER_SRC = vk::FormatFeatureFlags2::TRANSFER_SRC.as_raw(),
        /// Specifies that an image *can* be used as the destionation image of copy commands and
        /// clear commands.
        TRANSFER_DST = vk::FormatFeatureFlags2::TRANSFER_DST.as_raw(),
    }

    /// Bitmask specifying image resolve modes.
    #[default = Self::NONE]
    pub struct ResolveModes: Flags32 {
        NONE = vk::ResolveModeFlags::NONE.as_raw(),
        SAMPLE_ZERO = vk::ResolveModeFlags::SAMPLE_ZERO.as_raw(),
        AVERAGE = vk::ResolveModeFlags::AVERAGE.as_raw(),
        MIN = vk::ResolveModeFlags::MIN.as_raw(),
        MAX = vk::ResolveModeFlags::MAX.as_raw(),
    }

    #[default = Self::RGBA]
    pub struct ColorComponents: Flags32 {
        R = vk::ColorComponentFlags::R.as_raw(),
        G = vk::ColorComponentFlags::G.as_raw(),
        B = vk::ColorComponentFlags::B.as_raw(),
        A = vk::ColorComponentFlags::A.as_raw(),
        RGBA =
            Self::R.as_raw() |
            Self::G.as_raw() |
            Self::B.as_raw() |
            Self::A.as_raw(),
    }

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

c_enum! {
    /// Specifies the type of a gpu.
    pub struct PhysicalDeviceType: i32 {
        /// Specifies that the device doesn't match any available types.
        #[display("other")]
        OTHER = vk::PhysicalDeviceType::OTHER.as_raw(),
        /// Specifies an integrated gpu.
        #[display("integrated gpu")]
        INTEGRATED_GPU = vk::PhysicalDeviceType::INTEGRATED_GPU.as_raw(),
        /// Specifies a discrete gpu.
        #[display("discrete gpu")]
        DISCRETE_GPU = vk::PhysicalDeviceType::DISCRETE_GPU.as_raw(),
        /// Specifies a virtual gpu.
        #[display("virtual gpu")]
        VIRTUAL_GPU = vk::PhysicalDeviceType::VIRTUAL_GPU.as_raw(),
        /// Specifies a cpu instead of a gpu.
        #[display("cpu")]
        CPU = vk::PhysicalDeviceType::CPU.as_raw(),
    }

    /// Specifies how a component is swizzled.
    ///
    /// Default value is [`ComponentSwizzle::IDENTITY`].
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkComponentSwizzle.html>
    #[default = Self::IDENTITY]
    pub struct ComponentSwizzle: i32 {
        #[display("identity")]
        IDENTITY = vk::ComponentSwizzle::IDENTITY.as_raw(),
        #[display("zero")]
        ZERO = vk::ComponentSwizzle::ZERO.as_raw(),
        #[display("one")]
        ONE = vk::ComponentSwizzle::ONE.as_raw(),
        #[display("r")]
        R = vk::ComponentSwizzle::R.as_raw(),
        #[display("g")]
        G = vk::ComponentSwizzle::G.as_raw(),
        #[display("b")]
        B = vk::ComponentSwizzle::B.as_raw(),
        #[display("a")]
        A = vk::ComponentSwizzle::A.as_raw(),
    }
    /// Specifies filters used for texture lookups.
    ///
    /// Default value is [`Filter::NEAREST`].
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFilter.html>
    #[default = Self::NEAREST]
    pub struct Filter: i32 {
        /// Specifies nearest filtering.
        #[display("nearest")]
        NEAREST = vk::Filter::NEAREST.as_raw(),
        /// Specifies linear filtering.
        #[display("linear")]
        LINEAR = vk::Filter::LINEAR.as_raw(),
    }

    /// Specifies mipmap mode used for texture lookups.
    ///
    /// Default value is [`MipmapMode::NEAREST`].
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerMipmapMode.html>
    #[default = Self::NEAREST]
    pub struct MipmapMode: i32 {
        /// Specifies nearest filtering.
        #[display("nearest")]
        NEAREST = vk::SamplerMipmapMode::NEAREST.as_raw(),
        /// Specifies linear filtering.
        #[display("linear")]
        LINEAR = vk::SamplerMipmapMode::LINEAR.as_raw(),
    }

    /// Specifies behaviour of sampling with texture coordinates outside an image.
    ///
    /// Default value is [`SamplerAddressMode::REPEAT`].
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerAddressMode.html>
    #[default = Self::REPEAT]
    pub struct SamplerAddressMode: i32 {
        /// Specifies repeat wrap mode.
        #[display("repeat")]
        REPEAT = vk::SamplerAddressMode::REPEAT.as_raw(),
        /// Specifies mirrored repeat wrap mode.
        #[display("mirrored repeat")]
        MIRRORED_REPEAT = vk::SamplerAddressMode::MIRRORED_REPEAT.as_raw(),
        /// Specifies clamp to edge wrap mode.
        #[display("clamp to edge")]
        CLAMP_TO_EDGE = vk::SamplerAddressMode::CLAMP_TO_EDGE.as_raw(),
        /// Specifies clamp to border wrap mode.
        #[display("clamp to border")]
        CLAMP_TO_BORDER = vk::SamplerAddressMode::CLAMP_TO_BORDER.as_raw(),
    }

    /// Specifies the border color used for texture lookup.
    ///
    /// Default value is [`BorderColor::FloatTransparentBlack`].
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBorderColor.html>
    #[default = Self::FLOAT_TRANSPARENT_BLACK]
    pub struct BorderColor: i32 {
        #[display("float transparent black")]
        FLOAT_TRANSPARENT_BLACK = vk::BorderColor::FLOAT_TRANSPARENT_BLACK.as_raw(),
        #[display("int transparent black")]
        INT_TRANSPARENT_BLACK = vk::BorderColor::INT_TRANSPARENT_BLACK.as_raw(),
        #[display("float opaque black")]
        FLOAT_OPAQUE_BLACK = vk::BorderColor::FLOAT_OPAQUE_BLACK.as_raw(),
        #[display("int opaque black")]
        INT_OPAQUE_BLACK = vk::BorderColor::INT_OPAQUE_BLACK.as_raw(),
        #[display("float opaque white")]
        FLOAT_OPAQUE_WHITE = vk::BorderColor::FLOAT_OPAQUE_WHITE.as_raw(),
        #[display("int opaque white")]
        INT_OPAQUE_WHITE = vk::BorderColor::INT_OPAQUE_WHITE.as_raw(),
    }

    /// Specifies comparison operator for depth, stencil and sampler operations.
    ///
    /// Default value is [`CompareOp::Never`].
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCompareOp.html>
    #[default = Self::NEVER]
    pub struct CompareOp: i32 {
        #[display("never")]
        NEVER = vk::CompareOp::NEVER.as_raw(),
        #[display("less")]
        LESS = vk::CompareOp::LESS.as_raw(),
        #[display("equal")]
        EQUAL = vk::CompareOp::EQUAL.as_raw(),
        #[display("less or equal")]
        LESS_OR_EQUAL = vk::CompareOp::LESS_OR_EQUAL.as_raw(),
        #[display("greater")]
        GREATER = vk::CompareOp::GREATER.as_raw(),
        #[display("not equal")]
        NOT_EQUAL = vk::CompareOp::NOT_EQUAL.as_raw(),
        #[display("greater or equal")]
        GREATER_OR_EQUAL = vk::CompareOp::GREATER_OR_EQUAL.as_raw(),
        #[display("always")]
        ALWAYS = vk::CompareOp::ALWAYS.as_raw(),
    }

    /// Specifies the robustness of buffer accesses in a pipeline.
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessBufferBehavior.html>
    #[default = Self::DEVICE_DEFAULT]
    pub struct PipelineRobustnessBufferBehavior: i32 {
        /// Specifies that out of bounds buffer accesses follow the behavior of robust buffer access
        /// features enabled for the device.
        #[display("device default")]
        DEVICE_DEFAULT = vk::PipelineRobustnessBufferBehavior::DEVICE_DEFAULT.as_raw(),
        /// Specifies that buffer accesses *must* not be out of bounds.
        #[display("disabled")]
        DISABLED = vk::PipelineRobustnessBufferBehavior::DISABLED.as_raw(),
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
        ROBUST_BUFFER_ACCESS = vk::PipelineRobustnessBufferBehavior::ROBUST_BUFFER_ACCESS.as_raw(),
        /// Specifies that stricter bounds checks to shader buffers are performed.
        ///
        /// Out of bounds reads will produce zero values (with some caveats described in the docs).
        ///
        /// Out of bounds writes will not modify any memory.
        ///
        /// Atomic read-modify-write operations will behave the same as writes outside bounds, but
        /// will return *undefined* values.
        #[display("robust buffer access 2")]
        ROBUST_BUFFER_ACCESS2 = vk::PipelineRobustnessBufferBehavior::ROBUST_BUFFER_ACCESS_2.as_raw(),
    }

    /// Specifies the robustness of image accesses in a pipeline.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessImageBehavior.html>
    #[default = Self::DEVICE_DEFAULT]
    pub struct PipelineRobustnessImageBehavior: i32 {
        /// Specifies that out of bounds image accesses follow the behavior of robust image access features
        /// enabled for the device.
        #[display("device default")]
        DEVICE_DEFAULT = vk::PipelineRobustnessImageBehavior::DEVICE_DEFAULT.as_raw(),
        /// Specifies that image accesses *must* not be out of bounds.
        #[display("disabled")]
        DISABLED = vk::PipelineRobustnessImageBehavior::DISABLED.as_raw(),
        /// Specifies that out of bounds checks to shader images are performed.
        ///
        /// Out of bounds writes and atomic read-modify-write operations will not modify any
        /// memory.
        ///
        /// Reads, atomic read-modify-write operations, or fetches from images outside bounds will
        /// return zero values with (0,0,1) or (0,0,0) values inserted for missing G, B or A
        /// components based on the format.
        #[display("robust image access")]
        ROBUST_IMAGE_ACCESS = vk::PipelineRobustnessImageBehavior::ROBUST_IMAGE_ACCESS.as_raw(),
        /// Specifies that out of bounds checks to shader images are performed.
        ///
        /// Out of bounds writes and atomic read-modify-write operations will not modify any
        /// memory.
        ///
        /// Reads, atomic read-modify-write operations, or fetches from images outside bounds will
        /// return zero values with (0,0,1) values inserted for missing G, B or A components based
        /// on the format.
        #[display("robust image access 2")]
        ROBUST_IMAGE_ACCESS2 = vk::PipelineRobustnessImageBehavior::ROBUST_IMAGE_ACCESS_2.as_raw(),
    }

    /// Describes what parts of a pipeline can (and must) be dynamically changed.
    ///
    /// Doesn't include viewport or scissor since they are always enabled.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDynamicState.html>
    pub struct DynamicState: i32 {
        #[display("line width")]
        LINE_WIDTH = vk::DynamicState::LINE_WIDTH.as_raw(),
        #[display("depth bias")]
        DEPTH_BIAS = vk::DynamicState::DEPTH_BIAS.as_raw(),
        #[display("blend constants")]
        BLEND_CONSTANTS = vk::DynamicState::BLEND_CONSTANTS.as_raw(),
        #[display("depth bounds")]
        DEPTH_BOUNDS = vk::DynamicState::DEPTH_BOUNDS.as_raw(),
        #[display("stencil compare mask")]
        STENCIL_COMPARE_MASK = vk::DynamicState::STENCIL_COMPARE_MASK.as_raw(),
        #[display("stencil write mask")]
        STENCIL_WRITE_MASK = vk::DynamicState::STENCIL_WRITE_MASK.as_raw(),
        #[display("stencil reference")]
        STENCIL_REFERENCE = vk::DynamicState::STENCIL_REFERENCE.as_raw(),
        #[display("cull mode")]
        CULL_MODE = vk::DynamicState::CULL_MODE.as_raw(),
        #[display("front face")]
        FRONT_FACE = vk::DynamicState::FRONT_FACE.as_raw(),
        #[display("primitive topology")]
        PRIMITIVE_TOPOLOGY = vk::DynamicState::PRIMITIVE_TOPOLOGY.as_raw(),
        #[display("vertex input binding stride")]
        VERTEX_INPUT_BINDING_STRIDE = vk::DynamicState::VERTEX_INPUT_BINDING_STRIDE.as_raw(),
        #[display("depth test enable")]
        DEPTH_TEST_ENABLE =  vk::DynamicState::DEPTH_TEST_ENABLE.as_raw(),
        #[display("depth write enable")]
        DEPTH_WRITE_ENABLE = vk::DynamicState::DEPTH_WRITE_ENABLE.as_raw(),
        #[display("depth compare op")]
        DEPTH_COMPARE_OP = vk::DynamicState::DEPTH_COMPARE_OP.as_raw(),
        #[display("depth bounds test enable")]
        DEPTH_BOUNDS_TEST_ENABLE = vk::DynamicState::DEPTH_BOUNDS_TEST_ENABLE.as_raw(),
        #[display("stencil test enable")]
        STENCIL_TEST_ENABLE = vk::DynamicState::STENCIL_TEST_ENABLE.as_raw(),
        #[display("stencil op")]
        STENCIL_OP = vk::DynamicState::STENCIL_OP.as_raw(),
        #[display("rasterizer discard enable")]
        RASTERIZER_DISCARD_ENABLE = vk::DynamicState::RASTERIZER_DISCARD_ENABLE.as_raw(),
        #[display("depth bias enable")]
        DEPTH_BIAS_ENABLE = vk::DynamicState::DEPTH_BIAS_ENABLE.as_raw(),
        #[display("primitive restart enable")]
        PRIMITIVE_RESTART_ENABLE = vk::DynamicState::PRIMITIVE_RESTART_ENABLE.as_raw(),
    }

    /// Specifies polygon front-facing orientation.
    ///
    /// The default value is [`counter clockwise`][1].
    /// 
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFrontFace.html>
    ///
    /// [1]: Self::COUNTER_CLOCKWISE
    #[default = Self::COUNTER_CLOCKWISE]
    pub struct FrontFace: i32 {
        /// Specifies that triangles with positive area are considered front-facing.
        #[display("counter clockwise")]
        COUNTER_CLOCKWISE = vk::FrontFace::COUNTER_CLOCKWISE.as_raw(),
        /// Specifies that triangles with negative area are considered front-facing.
        #[display("clockwise")]
        CLOCK_WISE = vk::FrontFace::CLOCKWISE.as_raw(),
    }

    /// Specifies primitive topology.
    ///
    /// The default is [`triangle list`][1].
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPrimitiveTopology.html>
    ///
    /// [1]: Self::TRIANGLE_LIST
    #[default = Self::TRIANGLE_LIST]
    pub struct PrimitiveTopology: i32 {
        /// Specifies a series of separate point primities.
        #[display("point list")]
        POINT_LIST = vk::PrimitiveTopology::POINT_LIST.as_raw(),
        /// Specifies a series of separate line primities.
        #[display("line list")]
        LINE_LIST = vk::PrimitiveTopology::LINE_LIST.as_raw(),
        /// Specifies a series of connected line primities.
        #[display("line strip")]
        LINE_STRIP = vk::PrimitiveTopology::LINE_STRIP.as_raw(),
        /// Specifies a series of separate triangle primities.
        #[display("triangle list")]
        TRIANGLE_LIST = vk::PrimitiveTopology::TRIANGLE_LIST.as_raw(),
        /// Specifies a series of connected triangle primities.
        #[display("triangle strip")]
        TRIANGLE_STRIP = vk::PrimitiveTopology::TRIANGLE_STRIP.as_raw(),
        /// Specifies a series of connected triangle primitives with all triangles sharing a common
        /// vertex.
        #[display("triangle fan")]
        TRIANGLE_FAN = vk::PrimitiveTopology::TRIANGLE_FAN.as_raw(),
        /// Specifies a series of separate line primitives with adjacency.
        #[display("line list with adjacency")]
        LINE_LIST_WITH_ADJACENCY = vk::PrimitiveTopology::LINE_LIST_WITH_ADJACENCY.as_raw(),
        /// Specifies a series of connected line primitives with adjacency, with consecutive
        /// primitives sharing three vertices.
        #[display("line strip with adjacency")]
        LINE_STRIP_WITH_ADJACENCY = vk::PrimitiveTopology::LINE_STRIP_WITH_ADJACENCY.as_raw(),
        /// Specifies a series of separate triangle primitives with adjacency.
        #[display("triangle list with adjacency")]
        TRIANGLE_LIST_WITH_ADJACENCY = vk::PrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY.as_raw(),
        /// Specifies connected triangle primitives with adjacency, with consecutive triangles sharing an edge.
        #[display("triangle strip with adjacency")]
        TRIANGLE_STRIP_WITH_ADJACENCY = vk::PrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY.as_raw(),
        /// Specifies separate patch primitives.
        #[display("patch list")]
        PATCH_LIST = vk::PrimitiveTopology::PATCH_LIST.as_raw(),
    }


    /// Stencil comparison function.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilOp.html>
    pub struct StencilOp: i32 {
        /// Keeps the current value.
        #[display("keep")]
        KEEP = vk::StencilOp::KEEP.as_raw(),
        /// Sets the value to 0.
        #[display("zero")]
        ZERO = vk::StencilOp::ZERO.as_raw(),
        /// Sets the value to the reference.
        #[display("replace")]
        REPLACE = vk::StencilOp::REPLACE.as_raw(),
        /// Increments the current value, saturating at the maximum representable unsigned value.
        #[display("saturating increment")]
        SATURATING_INCREMENT = vk::StencilOp::INCREMENT_AND_CLAMP.as_raw(),
        /// Increments the current value, saturating at 0.
        #[display("saturating decrement")]
        SATURATING_DECREMENT = vk::StencilOp::DECREMENT_AND_CLAMP.as_raw(),
        /// Bitwise-inverts the current value.
        #[display("invert")]
        INVERT = vk::StencilOp::INVERT.as_raw(),
        /// Increments the current value, wrapping around at the maximum representable unsigned
        /// value.
        #[display("wrapping increment")]
        WRAPPING_INCREMENT = vk::StencilOp::INCREMENT_AND_WRAP.as_raw(),
        /// Decrements the current value, wrapping around at 0.
        #[display("wrapping decrement")]
        WRAPPING_DECREMENT = vk::StencilOp::DECREMENT_AND_WRAP.as_raw(),
    }

    /// Specifies polygon rasterization mode.
    ///
    /// The default value is ['fill`][1].
    ///
    /// [1]: Self::FILL
    #[default = Self::FILL]
    pub struct PolygonMode: i32 {
        #[display("fill")]
        FILL = vk::PolygonMode::FILL.as_raw(),
        #[display("line")]
        LINE = vk::PolygonMode::LINE.as_raw(),
        #[display("point")]
        POINT = vk::PolygonMode::POINT.as_raw(),
    }

    /// Specifies framebuffer blending factors.
    pub struct BlendFactor: i32 {
        #[display("zero")]
        ZERO = vk::BlendFactor::ZERO.as_raw(),
        #[display("one")]
        ONE = vk::BlendFactor::ONE.as_raw(),
        #[display("source color")]
        SRC_COLOR = vk::BlendFactor::SRC_COLOR.as_raw(),
        #[display("one minus source color")]
        ONE_MINUS_SRC_COLOR = vk::BlendFactor::ONE_MINUS_SRC_COLOR.as_raw(),
        #[display("destination color")]
        DST_COLOR = vk::BlendFactor::DST_COLOR.as_raw(),
        #[display("one minus destination color")]
        ONE_MINUS_DST_COLOR = vk::BlendFactor::ONE_MINUS_DST_COLOR.as_raw(),
        #[display("source alpha")]
        SRC_ALPHA = vk::BlendFactor::SRC_ALPHA.as_raw(),
        #[display("one minus source alpha")]
        ONE_MINUS_SRC_ALPHA = vk::BlendFactor::ONE_MINUS_SRC_ALPHA.as_raw(),
        #[display("destination alpha")]
        DST_ALPHA = vk::BlendFactor::DST_ALPHA.as_raw(),
        #[display("one minus destination alpha")]
        ONE_MINUS_DST_ALPHA = vk::BlendFactor::ONE_MINUS_DST_ALPHA.as_raw(),
        #[display("const color")]
        CONST_COLOR = vk::BlendFactor::CONSTANT_COLOR.as_raw(),
        #[display("one minus const color")]
        ONE_MINUS_CONST_COLOR = vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR.as_raw(),
        #[display("const alpha")]
        CONST_ALPHA = vk::BlendFactor::CONSTANT_ALPHA.as_raw(),
        #[display("one minus const alpha")]
        ONE_MINUS_CONST_ALPHA = vk::BlendFactor::ONE_MINUS_CONSTANT_ALPHA.as_raw(),
    }

    /// Specifies framebuffer blending operations
    pub struct BlendOp : i32 {
        #[display("add")]
        ADD = vk::BlendOp::ADD.as_raw(),
        #[display("subtract")]
        SUB = vk::BlendOp::SUBTRACT.as_raw(),
        #[display("reverse subtract")]
        REV_SUB = vk::BlendOp::REVERSE_SUBTRACT.as_raw(),
        #[display("min")]
        MIN = vk::BlendOp::MIN.as_raw(),
        #[display("max")]
        MAX = vk::BlendOp::MAX.as_raw(),
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

impl PrimitiveTopology {

    #[inline(always)]
    pub fn can_restart(self) -> bool {
        matches!(self,
            Self::LINE_STRIP | Self::TRIANGLE_STRIP |
            Self::LINE_STRIP_WITH_ADJACENCY  | Self::TRIANGLE_STRIP_WITH_ADJACENCY
        )
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

    #[inline]
    pub fn as_raw(self) -> i32 {
        self as i32
    }

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
}

impl From<vk::SampleCountFlags> for MsaaSamples {

    #[inline(always)]
    fn from(value: vk::SampleCountFlags) -> Self {
        Self::from_raw(value.as_raw())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ResolveAspect {
    Color,
    Depth,
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
    
    #[inline(always)]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    #[inline(always)]
    pub fn is_compatible_with(self, other: Self) -> bool {
        self.compatibility() == other.compatibility()
    }

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

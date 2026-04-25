use super::*;
use crate::bitflags;
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
pub const UUID_SIZE: u32 = 16;
pub const LUID_SIZE: u32 = 8;
pub const MAX_EXTENSION_NAME_SIZE: u32 = 256;
pub const MAX_DESCRIPTION_SIZE: u32 = 256;
pub const MAX_MEMORY_TYPES: u32 = 32;
pub const MAX_MEMORY_HEAPS: u32 = 16;
pub const LOD_CLAMP_NONE: f32 = 1000.0;
pub const REMAINING_MIP_LEVELS: u32 = !0;
pub const REMAINING_ARRAY_LAYERS: u32 = !0;
pub const REMAINING_3D_SLICES_EXT: u32 = !0;
pub const WHOLE_SIZE: u64 = !0;
pub const ATTACHMENT_UNUSED: u32 = !0;
pub const TRUE: u32 = 1;
pub const FALSE: u32 = 0;
pub const QUEUE_FAMILY_IGNORED: u32 = !0;
pub const QUEUE_FAMILY_EXTERNAL: u32 = !1;
pub const QUEUE_FAMILY_FOREIGN_EXT: u32 = !2;
pub const SUBPASS_EXTERNAL: u32 = !0;
pub const MAX_DEVICE_GROUP_SIZE: u32 = 32;
pub const MAX_DRIVER_NAME_SIZE: u32 = 256;
pub const MAX_DRIVER_INFO_SIZE: u32 = 256;
pub const SHADER_UNUSED_KHR: u32 = !0;
pub const MAX_GLOBAL_PRIORITY_SIZE: u32 = 16;
pub const MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: u32 = 32;
pub const MAX_PIPELINE_BINARY_KEY_SIZE_KHR: u32 = 32;
pub const MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR: u32 = 7;
pub const MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR: u32 = 3;
pub const SHADER_INDEX_UNUSED_AMDX: u32 = !0;
pub const PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV: u32 = !0;
pub const COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX: u32 = 128;
pub const COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX: u32 = 128;
pub const MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM: u32 = 128;
pub const DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM: u32 = 3;
pub const COMPUTE_OCCUPANCY_PRIORITY_LOW_NV: f32 = 0.25;
pub const COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV: f32 = 0.50;
pub const COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV: f32 = 0.75;
pub const MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM: u32 = 128;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentLoadOp.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AttachmentLoadOp(i32);
impl AttachmentLoadOp {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl AttachmentLoadOp {
    pub const LOAD: Self = Self(0);
    pub const CLEAR: Self = Self(1);
    pub const DONT_CARE: Self = Self(2);
    pub const NONE: Self = Self(1000400000);
    pub const NONE_EXT: Self = Self::NONE;
    pub const NONE_KHR: Self = Self::NONE;
}
impl fmt::Display for AttachmentLoadOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LOAD => write!(f, "LOAD"),
            Self::CLEAR => write!(f, "CLEAR"),
            Self::DONT_CARE => write!(f, "DONT_CARE"),
            Self::NONE => write!(f, "NONE"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentStoreOp.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AttachmentStoreOp(i32);
impl AttachmentStoreOp {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl AttachmentStoreOp {
    pub const STORE: Self = Self(0);
    pub const DONT_CARE: Self = Self(1);
    pub const NONE: Self = Self(1000301000);
    pub const NONE_KHR: Self = Self::NONE;
    pub const NONE_QCOM: Self = Self::NONE;
    pub const NONE_EXT: Self = Self::NONE;
}
impl fmt::Display for AttachmentStoreOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::STORE => write!(f, "STORE"),
            Self::DONT_CARE => write!(f, "DONT_CARE"),
            Self::NONE => write!(f, "NONE"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBlendFactor.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BlendFactor(i32);
impl BlendFactor {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl BlendFactor {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const SRC_COLOR: Self = Self(2);
    pub const ONE_MINUS_SRC_COLOR: Self = Self(3);
    pub const DST_COLOR: Self = Self(4);
    pub const ONE_MINUS_DST_COLOR: Self = Self(5);
    pub const SRC_ALPHA: Self = Self(6);
    pub const ONE_MINUS_SRC_ALPHA: Self = Self(7);
    pub const DST_ALPHA: Self = Self(8);
    pub const ONE_MINUS_DST_ALPHA: Self = Self(9);
    pub const CONSTANT_COLOR: Self = Self(10);
    pub const ONE_MINUS_CONSTANT_COLOR: Self = Self(11);
    pub const CONSTANT_ALPHA: Self = Self(12);
    pub const ONE_MINUS_CONSTANT_ALPHA: Self = Self(13);
    pub const SRC_ALPHA_SATURATE: Self = Self(14);
    pub const SRC1_COLOR: Self = Self(15);
    pub const ONE_MINUS_SRC1_COLOR: Self = Self(16);
    pub const SRC1_ALPHA: Self = Self(17);
    pub const ONE_MINUS_SRC1_ALPHA: Self = Self(18);
}
impl fmt::Display for BlendFactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ZERO => write!(f, "ZERO"),
            Self::ONE => write!(f, "ONE"),
            Self::SRC_COLOR => write!(f, "SRC_COLOR"),
            Self::ONE_MINUS_SRC_COLOR => write!(f, "ONE_MINUS_SRC_COLOR"),
            Self::DST_COLOR => write!(f, "DST_COLOR"),
            Self::ONE_MINUS_DST_COLOR => write!(f, "ONE_MINUS_DST_COLOR"),
            Self::SRC_ALPHA => write!(f, "SRC_ALPHA"),
            Self::ONE_MINUS_SRC_ALPHA => write!(f, "ONE_MINUS_SRC_ALPHA"),
            Self::DST_ALPHA => write!(f, "DST_ALPHA"),
            Self::ONE_MINUS_DST_ALPHA => write!(f, "ONE_MINUS_DST_ALPHA"),
            Self::CONSTANT_COLOR => write!(f, "CONSTANT_COLOR"),
            Self::ONE_MINUS_CONSTANT_COLOR => write!(f, "ONE_MINUS_CONSTANT_COLOR"),
            Self::CONSTANT_ALPHA => write!(f, "CONSTANT_ALPHA"),
            Self::ONE_MINUS_CONSTANT_ALPHA => write!(f, "ONE_MINUS_CONSTANT_ALPHA"),
            Self::SRC_ALPHA_SATURATE => write!(f, "SRC_ALPHA_SATURATE"),
            Self::SRC1_COLOR => write!(f, "SRC1_COLOR"),
            Self::ONE_MINUS_SRC1_COLOR => write!(f, "ONE_MINUS_SRC1_COLOR"),
            Self::SRC1_ALPHA => write!(f, "SRC1_ALPHA"),
            Self::ONE_MINUS_SRC1_ALPHA => write!(f, "ONE_MINUS_SRC1_ALPHA"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBlendOp.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BlendOp(i32);
impl BlendOp {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl BlendOp {
    pub const ADD: Self = Self(0);
    pub const SUBTRACT: Self = Self(1);
    pub const REVERSE_SUBTRACT: Self = Self(2);
    pub const MIN: Self = Self(3);
    pub const MAX: Self = Self(4);
    pub const ZERO_EXT: Self = Self(1000148000);
    pub const SRC_EXT: Self = Self(1000148001);
    pub const DST_EXT: Self = Self(1000148002);
    pub const SRC_OVER_EXT: Self = Self(1000148003);
    pub const DST_OVER_EXT: Self = Self(1000148004);
    pub const SRC_IN_EXT: Self = Self(1000148005);
    pub const DST_IN_EXT: Self = Self(1000148006);
    pub const SRC_OUT_EXT: Self = Self(1000148007);
    pub const DST_OUT_EXT: Self = Self(1000148008);
    pub const SRC_ATOP_EXT: Self = Self(1000148009);
    pub const DST_ATOP_EXT: Self = Self(1000148010);
    pub const XOR_EXT: Self = Self(1000148011);
    pub const MULTIPLY_EXT: Self = Self(1000148012);
    pub const SCREEN_EXT: Self = Self(1000148013);
    pub const OVERLAY_EXT: Self = Self(1000148014);
    pub const DARKEN_EXT: Self = Self(1000148015);
    pub const LIGHTEN_EXT: Self = Self(1000148016);
    pub const COLORDODGE_EXT: Self = Self(1000148017);
    pub const COLORBURN_EXT: Self = Self(1000148018);
    pub const HARDLIGHT_EXT: Self = Self(1000148019);
    pub const SOFTLIGHT_EXT: Self = Self(1000148020);
    pub const DIFFERENCE_EXT: Self = Self(1000148021);
    pub const EXCLUSION_EXT: Self = Self(1000148022);
    pub const INVERT_EXT: Self = Self(1000148023);
    pub const INVERT_RGB_EXT: Self = Self(1000148024);
    pub const LINEARDODGE_EXT: Self = Self(1000148025);
    pub const LINEARBURN_EXT: Self = Self(1000148026);
    pub const VIVIDLIGHT_EXT: Self = Self(1000148027);
    pub const LINEARLIGHT_EXT: Self = Self(1000148028);
    pub const PINLIGHT_EXT: Self = Self(1000148029);
    pub const HARDMIX_EXT: Self = Self(1000148030);
    pub const HSL_HUE_EXT: Self = Self(1000148031);
    pub const HSL_SATURATION_EXT: Self = Self(1000148032);
    pub const HSL_COLOR_EXT: Self = Self(1000148033);
    pub const HSL_LUMINOSITY_EXT: Self = Self(1000148034);
    pub const PLUS_EXT: Self = Self(1000148035);
    pub const PLUS_CLAMPED_EXT: Self = Self(1000148036);
    pub const PLUS_CLAMPED_ALPHA_EXT: Self = Self(1000148037);
    pub const PLUS_DARKER_EXT: Self = Self(1000148038);
    pub const MINUS_EXT: Self = Self(1000148039);
    pub const MINUS_CLAMPED_EXT: Self = Self(1000148040);
    pub const CONTRAST_EXT: Self = Self(1000148041);
    pub const INVERT_OVG_EXT: Self = Self(1000148042);
    pub const RED_EXT: Self = Self(1000148043);
    pub const GREEN_EXT: Self = Self(1000148044);
    pub const BLUE_EXT: Self = Self(1000148045);
}
impl fmt::Display for BlendOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ADD => write!(f, "ADD"),
            Self::SUBTRACT => write!(f, "SUBTRACT"),
            Self::REVERSE_SUBTRACT => write!(f, "REVERSE_SUBTRACT"),
            Self::MIN => write!(f, "MIN"),
            Self::MAX => write!(f, "MAX"),
            Self::ZERO_EXT => write!(f, "ZERO_EXT"),
            Self::SRC_EXT => write!(f, "SRC_EXT"),
            Self::DST_EXT => write!(f, "DST_EXT"),
            Self::SRC_OVER_EXT => write!(f, "SRC_OVER_EXT"),
            Self::DST_OVER_EXT => write!(f, "DST_OVER_EXT"),
            Self::SRC_IN_EXT => write!(f, "SRC_IN_EXT"),
            Self::DST_IN_EXT => write!(f, "DST_IN_EXT"),
            Self::SRC_OUT_EXT => write!(f, "SRC_OUT_EXT"),
            Self::DST_OUT_EXT => write!(f, "DST_OUT_EXT"),
            Self::SRC_ATOP_EXT => write!(f, "SRC_ATOP_EXT"),
            Self::DST_ATOP_EXT => write!(f, "DST_ATOP_EXT"),
            Self::XOR_EXT => write!(f, "XOR_EXT"),
            Self::MULTIPLY_EXT => write!(f, "MULTIPLY_EXT"),
            Self::SCREEN_EXT => write!(f, "SCREEN_EXT"),
            Self::OVERLAY_EXT => write!(f, "OVERLAY_EXT"),
            Self::DARKEN_EXT => write!(f, "DARKEN_EXT"),
            Self::LIGHTEN_EXT => write!(f, "LIGHTEN_EXT"),
            Self::COLORDODGE_EXT => write!(f, "COLORDODGE_EXT"),
            Self::COLORBURN_EXT => write!(f, "COLORBURN_EXT"),
            Self::HARDLIGHT_EXT => write!(f, "HARDLIGHT_EXT"),
            Self::SOFTLIGHT_EXT => write!(f, "SOFTLIGHT_EXT"),
            Self::DIFFERENCE_EXT => write!(f, "DIFFERENCE_EXT"),
            Self::EXCLUSION_EXT => write!(f, "EXCLUSION_EXT"),
            Self::INVERT_EXT => write!(f, "INVERT_EXT"),
            Self::INVERT_RGB_EXT => write!(f, "INVERT_RGB_EXT"),
            Self::LINEARDODGE_EXT => write!(f, "LINEARDODGE_EXT"),
            Self::LINEARBURN_EXT => write!(f, "LINEARBURN_EXT"),
            Self::VIVIDLIGHT_EXT => write!(f, "VIVIDLIGHT_EXT"),
            Self::LINEARLIGHT_EXT => write!(f, "LINEARLIGHT_EXT"),
            Self::PINLIGHT_EXT => write!(f, "PINLIGHT_EXT"),
            Self::HARDMIX_EXT => write!(f, "HARDMIX_EXT"),
            Self::HSL_HUE_EXT => write!(f, "HSL_HUE_EXT"),
            Self::HSL_SATURATION_EXT => write!(f, "HSL_SATURATION_EXT"),
            Self::HSL_COLOR_EXT => write!(f, "HSL_COLOR_EXT"),
            Self::HSL_LUMINOSITY_EXT => write!(f, "HSL_LUMINOSITY_EXT"),
            Self::PLUS_EXT => write!(f, "PLUS_EXT"),
            Self::PLUS_CLAMPED_EXT => write!(f, "PLUS_CLAMPED_EXT"),
            Self::PLUS_CLAMPED_ALPHA_EXT => write!(f, "PLUS_CLAMPED_ALPHA_EXT"),
            Self::PLUS_DARKER_EXT => write!(f, "PLUS_DARKER_EXT"),
            Self::MINUS_EXT => write!(f, "MINUS_EXT"),
            Self::MINUS_CLAMPED_EXT => write!(f, "MINUS_CLAMPED_EXT"),
            Self::CONTRAST_EXT => write!(f, "CONTRAST_EXT"),
            Self::INVERT_OVG_EXT => write!(f, "INVERT_OVG_EXT"),
            Self::RED_EXT => write!(f, "RED_EXT"),
            Self::GREEN_EXT => write!(f, "GREEN_EXT"),
            Self::BLUE_EXT => write!(f, "BLUE_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBorderColor.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BorderColor(i32);
impl BorderColor {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl BorderColor {
    pub const FLOAT_TRANSPARENT_BLACK: Self = Self(0);
    pub const INT_TRANSPARENT_BLACK: Self = Self(1);
    pub const FLOAT_OPAQUE_BLACK: Self = Self(2);
    pub const INT_OPAQUE_BLACK: Self = Self(3);
    pub const FLOAT_OPAQUE_WHITE: Self = Self(4);
    pub const INT_OPAQUE_WHITE: Self = Self(5);
    pub const FLOAT_CUSTOM_EXT: Self = Self(1000287003);
    pub const INT_CUSTOM_EXT: Self = Self(1000287004);
}
impl fmt::Display for BorderColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::FLOAT_TRANSPARENT_BLACK => write!(f, "FLOAT_TRANSPARENT_BLACK"),
            Self::INT_TRANSPARENT_BLACK => write!(f, "INT_TRANSPARENT_BLACK"),
            Self::FLOAT_OPAQUE_BLACK => write!(f, "FLOAT_OPAQUE_BLACK"),
            Self::INT_OPAQUE_BLACK => write!(f, "INT_OPAQUE_BLACK"),
            Self::FLOAT_OPAQUE_WHITE => write!(f, "FLOAT_OPAQUE_WHITE"),
            Self::INT_OPAQUE_WHITE => write!(f, "INT_OPAQUE_WHITE"),
            Self::FLOAT_CUSTOM_EXT => write!(f, "FLOAT_CUSTOM_EXT"),
            Self::INT_CUSTOM_EXT => write!(f, "INT_CUSTOM_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheHeaderVersion.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PipelineCacheHeaderVersion(i32);
impl PipelineCacheHeaderVersion {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PipelineCacheHeaderVersion {
    pub const ONE: Self = Self(1);
    pub const DATA_GRAPH_QCOM: Self = Self(1000629000);
}
impl fmt::Display for PipelineCacheHeaderVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ONE => write!(f, "ONE"),
            Self::DATA_GRAPH_QCOM => write!(f, "DATA_GRAPH_QCOM"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkComponentSwizzle.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ComponentSwizzle(i32);
impl ComponentSwizzle {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ComponentSwizzle {
    pub const IDENTITY: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const ONE: Self = Self(2);
    pub const R: Self = Self(3);
    pub const G: Self = Self(4);
    pub const B: Self = Self(5);
    pub const A: Self = Self(6);
}
impl fmt::Display for ComponentSwizzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::IDENTITY => write!(f, "IDENTITY"),
            Self::ZERO => write!(f, "ZERO"),
            Self::ONE => write!(f, "ONE"),
            Self::R => write!(f, "R"),
            Self::G => write!(f, "G"),
            Self::B => write!(f, "B"),
            Self::A => write!(f, "A"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferLevel.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CommandBufferLevel(i32);
impl CommandBufferLevel {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl CommandBufferLevel {
    pub const PRIMARY: Self = Self(0);
    pub const SECONDARY: Self = Self(1);
}
impl fmt::Display for CommandBufferLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::PRIMARY => write!(f, "PRIMARY"),
            Self::SECONDARY => write!(f, "SECONDARY"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCompareOp.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CompareOp(i32);
impl CompareOp {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl CompareOp {
    pub const NEVER: Self = Self(0);
    pub const LESS: Self = Self(1);
    pub const EQUAL: Self = Self(2);
    pub const LESS_OR_EQUAL: Self = Self(3);
    pub const GREATER: Self = Self(4);
    pub const NOT_EQUAL: Self = Self(5);
    pub const GREATER_OR_EQUAL: Self = Self(6);
    pub const ALWAYS: Self = Self(7);
}
impl fmt::Display for CompareOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NEVER => write!(f, "NEVER"),
            Self::LESS => write!(f, "LESS"),
            Self::EQUAL => write!(f, "EQUAL"),
            Self::LESS_OR_EQUAL => write!(f, "LESS_OR_EQUAL"),
            Self::GREATER => write!(f, "GREATER"),
            Self::NOT_EQUAL => write!(f, "NOT_EQUAL"),
            Self::GREATER_OR_EQUAL => write!(f, "GREATER_OR_EQUAL"),
            Self::ALWAYS => write!(f, "ALWAYS"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorType.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DescriptorType(i32);
impl DescriptorType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DescriptorType {
    pub const SAMPLER: Self = Self(0);
    pub const COMBINED_IMAGE_SAMPLER: Self = Self(1);
    pub const SAMPLED_IMAGE: Self = Self(2);
    pub const STORAGE_IMAGE: Self = Self(3);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(4);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(5);
    pub const UNIFORM_BUFFER: Self = Self(6);
    pub const STORAGE_BUFFER: Self = Self(7);
    pub const UNIFORM_BUFFER_DYNAMIC: Self = Self(8);
    pub const STORAGE_BUFFER_DYNAMIC: Self = Self(9);
    pub const INPUT_ATTACHMENT: Self = Self(10);
    pub const INLINE_UNIFORM_BLOCK: Self = Self(1000138000);
    pub const INLINE_UNIFORM_BLOCK_EXT: Self = Self::INLINE_UNIFORM_BLOCK;
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
    pub const MUTABLE_VALVE: Self = Self::MUTABLE_EXT;
    pub const SAMPLE_WEIGHT_IMAGE_QCOM: Self = Self(1000440000);
    pub const BLOCK_MATCH_IMAGE_QCOM: Self = Self(1000440001);
    pub const TENSOR_ARM: Self = Self(1000460000);
    pub const MUTABLE_EXT: Self = Self(1000351000);
    pub const PARTITIONED_ACCELERATION_STRUCTURE_NV: Self = Self(1000570000);
}
impl fmt::Display for DescriptorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::SAMPLER => write!(f, "SAMPLER"),
            Self::COMBINED_IMAGE_SAMPLER => write!(f, "COMBINED_IMAGE_SAMPLER"),
            Self::SAMPLED_IMAGE => write!(f, "SAMPLED_IMAGE"),
            Self::STORAGE_IMAGE => write!(f, "STORAGE_IMAGE"),
            Self::UNIFORM_TEXEL_BUFFER => write!(f, "UNIFORM_TEXEL_BUFFER"),
            Self::STORAGE_TEXEL_BUFFER => write!(f, "STORAGE_TEXEL_BUFFER"),
            Self::UNIFORM_BUFFER => write!(f, "UNIFORM_BUFFER"),
            Self::STORAGE_BUFFER => write!(f, "STORAGE_BUFFER"),
            Self::UNIFORM_BUFFER_DYNAMIC => write!(f, "UNIFORM_BUFFER_DYNAMIC"),
            Self::STORAGE_BUFFER_DYNAMIC => write!(f, "STORAGE_BUFFER_DYNAMIC"),
            Self::INPUT_ATTACHMENT => write!(f, "INPUT_ATTACHMENT"),
            Self::INLINE_UNIFORM_BLOCK => write!(f, "INLINE_UNIFORM_BLOCK"),
            Self::ACCELERATION_STRUCTURE_KHR => write!(f, "ACCELERATION_STRUCTURE_KHR"),
            Self::ACCELERATION_STRUCTURE_NV => write!(f, "ACCELERATION_STRUCTURE_NV"),
            Self::SAMPLE_WEIGHT_IMAGE_QCOM => write!(f, "SAMPLE_WEIGHT_IMAGE_QCOM"),
            Self::BLOCK_MATCH_IMAGE_QCOM => write!(f, "BLOCK_MATCH_IMAGE_QCOM"),
            Self::TENSOR_ARM => write!(f, "TENSOR_ARM"),
            Self::MUTABLE_EXT => write!(f, "MUTABLE_EXT"),
            Self::PARTITIONED_ACCELERATION_STRUCTURE_NV => {
                write!(f, "PARTITIONED_ACCELERATION_STRUCTURE_NV")
            }
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDynamicState.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DynamicState(i32);
impl DynamicState {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DynamicState {
    pub const VIEWPORT: Self = Self(0);
    pub const SCISSOR: Self = Self(1);
    pub const LINE_WIDTH: Self = Self(2);
    pub const DEPTH_BIAS: Self = Self(3);
    pub const BLEND_CONSTANTS: Self = Self(4);
    pub const DEPTH_BOUNDS: Self = Self(5);
    pub const STENCIL_COMPARE_MASK: Self = Self(6);
    pub const STENCIL_WRITE_MASK: Self = Self(7);
    pub const STENCIL_REFERENCE: Self = Self(8);
    pub const CULL_MODE: Self = Self(1000267000);
    pub const FRONT_FACE: Self = Self(1000267001);
    pub const PRIMITIVE_TOPOLOGY: Self = Self(1000267002);
    pub const VIEWPORT_WITH_COUNT: Self = Self(1000267003);
    pub const SCISSOR_WITH_COUNT: Self = Self(1000267004);
    pub const VERTEX_INPUT_BINDING_STRIDE: Self = Self(1000267005);
    pub const DEPTH_TEST_ENABLE: Self = Self(1000267006);
    pub const DEPTH_WRITE_ENABLE: Self = Self(1000267007);
    pub const DEPTH_COMPARE_OP: Self = Self(1000267008);
    pub const DEPTH_BOUNDS_TEST_ENABLE: Self = Self(1000267009);
    pub const STENCIL_TEST_ENABLE: Self = Self(1000267010);
    pub const STENCIL_OP: Self = Self(1000267011);
    pub const RASTERIZER_DISCARD_ENABLE: Self = Self(1000377001);
    pub const DEPTH_BIAS_ENABLE: Self = Self(1000377002);
    pub const PRIMITIVE_RESTART_ENABLE: Self = Self(1000377004);
    pub const LINE_STIPPLE: Self = Self(1000259000);
    pub const VIEWPORT_W_SCALING_NV: Self = Self(1000087000);
    pub const DISCARD_RECTANGLE_EXT: Self = Self(1000099000);
    pub const DISCARD_RECTANGLE_ENABLE_EXT: Self = Self(1000099001);
    pub const DISCARD_RECTANGLE_MODE_EXT: Self = Self(1000099002);
    pub const SAMPLE_LOCATIONS_EXT: Self = Self(1000143000);
    pub const RAY_TRACING_PIPELINE_STACK_SIZE_KHR: Self = Self(1000347000);
    pub const VIEWPORT_SHADING_RATE_PALETTE_NV: Self = Self(1000164004);
    pub const VIEWPORT_COARSE_SAMPLE_ORDER_NV: Self = Self(1000164006);
    pub const EXCLUSIVE_SCISSOR_ENABLE_NV: Self = Self(1000205000);
    pub const EXCLUSIVE_SCISSOR_NV: Self = Self(1000205001);
    pub const FRAGMENT_SHADING_RATE_KHR: Self = Self(1000226000);
    pub const LINE_STIPPLE_EXT: Self = Self::LINE_STIPPLE;
    pub const CULL_MODE_EXT: Self = Self::CULL_MODE;
    pub const FRONT_FACE_EXT: Self = Self::FRONT_FACE;
    pub const PRIMITIVE_TOPOLOGY_EXT: Self = Self::PRIMITIVE_TOPOLOGY;
    pub const VIEWPORT_WITH_COUNT_EXT: Self = Self::VIEWPORT_WITH_COUNT;
    pub const SCISSOR_WITH_COUNT_EXT: Self = Self::SCISSOR_WITH_COUNT;
    pub const VERTEX_INPUT_BINDING_STRIDE_EXT: Self = Self::VERTEX_INPUT_BINDING_STRIDE;
    pub const DEPTH_TEST_ENABLE_EXT: Self = Self::DEPTH_TEST_ENABLE;
    pub const DEPTH_WRITE_ENABLE_EXT: Self = Self::DEPTH_WRITE_ENABLE;
    pub const DEPTH_COMPARE_OP_EXT: Self = Self::DEPTH_COMPARE_OP;
    pub const DEPTH_BOUNDS_TEST_ENABLE_EXT: Self = Self::DEPTH_BOUNDS_TEST_ENABLE;
    pub const STENCIL_TEST_ENABLE_EXT: Self = Self::STENCIL_TEST_ENABLE;
    pub const STENCIL_OP_EXT: Self = Self::STENCIL_OP;
    pub const VERTEX_INPUT_EXT: Self = Self(1000352000);
    pub const PATCH_CONTROL_POINTS_EXT: Self = Self(1000377000);
    pub const RASTERIZER_DISCARD_ENABLE_EXT: Self = Self::RASTERIZER_DISCARD_ENABLE;
    pub const DEPTH_BIAS_ENABLE_EXT: Self = Self::DEPTH_BIAS_ENABLE;
    pub const LOGIC_OP_EXT: Self = Self(1000377003);
    pub const PRIMITIVE_RESTART_ENABLE_EXT: Self = Self::PRIMITIVE_RESTART_ENABLE;
    pub const COLOR_WRITE_ENABLE_EXT: Self = Self(1000381000);
    pub const DEPTH_CLAMP_ENABLE_EXT: Self = Self(1000455003);
    pub const POLYGON_MODE_EXT: Self = Self(1000455004);
    pub const RASTERIZATION_SAMPLES_EXT: Self = Self(1000455005);
    pub const SAMPLE_MASK_EXT: Self = Self(1000455006);
    pub const ALPHA_TO_COVERAGE_ENABLE_EXT: Self = Self(1000455007);
    pub const ALPHA_TO_ONE_ENABLE_EXT: Self = Self(1000455008);
    pub const LOGIC_OP_ENABLE_EXT: Self = Self(1000455009);
    pub const COLOR_BLEND_ENABLE_EXT: Self = Self(1000455010);
    pub const COLOR_BLEND_EQUATION_EXT: Self = Self(1000455011);
    pub const COLOR_WRITE_MASK_EXT: Self = Self(1000455012);
    pub const TESSELLATION_DOMAIN_ORIGIN_EXT: Self = Self(1000455002);
    pub const RASTERIZATION_STREAM_EXT: Self = Self(1000455013);
    pub const CONSERVATIVE_RASTERIZATION_MODE_EXT: Self = Self(1000455014);
    pub const EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT: Self = Self(1000455015);
    pub const DEPTH_CLIP_ENABLE_EXT: Self = Self(1000455016);
    pub const SAMPLE_LOCATIONS_ENABLE_EXT: Self = Self(1000455017);
    pub const COLOR_BLEND_ADVANCED_EXT: Self = Self(1000455018);
    pub const PROVOKING_VERTEX_MODE_EXT: Self = Self(1000455019);
    pub const LINE_RASTERIZATION_MODE_EXT: Self = Self(1000455020);
    pub const LINE_STIPPLE_ENABLE_EXT: Self = Self(1000455021);
    pub const DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT: Self = Self(1000455022);
    pub const VIEWPORT_W_SCALING_ENABLE_NV: Self = Self(1000455023);
    pub const VIEWPORT_SWIZZLE_NV: Self = Self(1000455024);
    pub const COVERAGE_TO_COLOR_ENABLE_NV: Self = Self(1000455025);
    pub const COVERAGE_TO_COLOR_LOCATION_NV: Self = Self(1000455026);
    pub const COVERAGE_MODULATION_MODE_NV: Self = Self(1000455027);
    pub const COVERAGE_MODULATION_TABLE_ENABLE_NV: Self = Self(1000455028);
    pub const COVERAGE_MODULATION_TABLE_NV: Self = Self(1000455029);
    pub const SHADING_RATE_IMAGE_ENABLE_NV: Self = Self(1000455030);
    pub const REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV: Self = Self(1000455031);
    pub const COVERAGE_REDUCTION_MODE_NV: Self = Self(1000455032);
    pub const ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT: Self = Self(1000524000);
    pub const LINE_STIPPLE_KHR: Self = Self::LINE_STIPPLE;
    pub const DEPTH_CLAMP_RANGE_EXT: Self = Self(1000582000);
}
impl fmt::Display for DynamicState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::VIEWPORT => write!(f, "VIEWPORT"),
            Self::SCISSOR => write!(f, "SCISSOR"),
            Self::LINE_WIDTH => write!(f, "LINE_WIDTH"),
            Self::DEPTH_BIAS => write!(f, "DEPTH_BIAS"),
            Self::BLEND_CONSTANTS => write!(f, "BLEND_CONSTANTS"),
            Self::DEPTH_BOUNDS => write!(f, "DEPTH_BOUNDS"),
            Self::STENCIL_COMPARE_MASK => write!(f, "STENCIL_COMPARE_MASK"),
            Self::STENCIL_WRITE_MASK => write!(f, "STENCIL_WRITE_MASK"),
            Self::STENCIL_REFERENCE => write!(f, "STENCIL_REFERENCE"),
            Self::CULL_MODE => write!(f, "CULL_MODE"),
            Self::FRONT_FACE => write!(f, "FRONT_FACE"),
            Self::PRIMITIVE_TOPOLOGY => write!(f, "PRIMITIVE_TOPOLOGY"),
            Self::VIEWPORT_WITH_COUNT => write!(f, "VIEWPORT_WITH_COUNT"),
            Self::SCISSOR_WITH_COUNT => write!(f, "SCISSOR_WITH_COUNT"),
            Self::VERTEX_INPUT_BINDING_STRIDE => write!(f, "VERTEX_INPUT_BINDING_STRIDE"),
            Self::DEPTH_TEST_ENABLE => write!(f, "DEPTH_TEST_ENABLE"),
            Self::DEPTH_WRITE_ENABLE => write!(f, "DEPTH_WRITE_ENABLE"),
            Self::DEPTH_COMPARE_OP => write!(f, "DEPTH_COMPARE_OP"),
            Self::DEPTH_BOUNDS_TEST_ENABLE => write!(f, "DEPTH_BOUNDS_TEST_ENABLE"),
            Self::STENCIL_TEST_ENABLE => write!(f, "STENCIL_TEST_ENABLE"),
            Self::STENCIL_OP => write!(f, "STENCIL_OP"),
            Self::RASTERIZER_DISCARD_ENABLE => write!(f, "RASTERIZER_DISCARD_ENABLE"),
            Self::DEPTH_BIAS_ENABLE => write!(f, "DEPTH_BIAS_ENABLE"),
            Self::PRIMITIVE_RESTART_ENABLE => write!(f, "PRIMITIVE_RESTART_ENABLE"),
            Self::LINE_STIPPLE => write!(f, "LINE_STIPPLE"),
            Self::VIEWPORT_W_SCALING_NV => write!(f, "VIEWPORT_W_SCALING_NV"),
            Self::DISCARD_RECTANGLE_EXT => write!(f, "DISCARD_RECTANGLE_EXT"),
            Self::DISCARD_RECTANGLE_ENABLE_EXT => write!(f, "DISCARD_RECTANGLE_ENABLE_EXT"),
            Self::DISCARD_RECTANGLE_MODE_EXT => write!(f, "DISCARD_RECTANGLE_MODE_EXT"),
            Self::SAMPLE_LOCATIONS_EXT => write!(f, "SAMPLE_LOCATIONS_EXT"),
            Self::RAY_TRACING_PIPELINE_STACK_SIZE_KHR => {
                write!(f, "RAY_TRACING_PIPELINE_STACK_SIZE_KHR")
            }
            Self::VIEWPORT_SHADING_RATE_PALETTE_NV => write!(f, "VIEWPORT_SHADING_RATE_PALETTE_NV"),
            Self::VIEWPORT_COARSE_SAMPLE_ORDER_NV => write!(f, "VIEWPORT_COARSE_SAMPLE_ORDER_NV"),
            Self::EXCLUSIVE_SCISSOR_ENABLE_NV => write!(f, "EXCLUSIVE_SCISSOR_ENABLE_NV"),
            Self::EXCLUSIVE_SCISSOR_NV => write!(f, "EXCLUSIVE_SCISSOR_NV"),
            Self::FRAGMENT_SHADING_RATE_KHR => write!(f, "FRAGMENT_SHADING_RATE_KHR"),
            Self::VERTEX_INPUT_EXT => write!(f, "VERTEX_INPUT_EXT"),
            Self::PATCH_CONTROL_POINTS_EXT => write!(f, "PATCH_CONTROL_POINTS_EXT"),
            Self::LOGIC_OP_EXT => write!(f, "LOGIC_OP_EXT"),
            Self::COLOR_WRITE_ENABLE_EXT => write!(f, "COLOR_WRITE_ENABLE_EXT"),
            Self::DEPTH_CLAMP_ENABLE_EXT => write!(f, "DEPTH_CLAMP_ENABLE_EXT"),
            Self::POLYGON_MODE_EXT => write!(f, "POLYGON_MODE_EXT"),
            Self::RASTERIZATION_SAMPLES_EXT => write!(f, "RASTERIZATION_SAMPLES_EXT"),
            Self::SAMPLE_MASK_EXT => write!(f, "SAMPLE_MASK_EXT"),
            Self::ALPHA_TO_COVERAGE_ENABLE_EXT => write!(f, "ALPHA_TO_COVERAGE_ENABLE_EXT"),
            Self::ALPHA_TO_ONE_ENABLE_EXT => write!(f, "ALPHA_TO_ONE_ENABLE_EXT"),
            Self::LOGIC_OP_ENABLE_EXT => write!(f, "LOGIC_OP_ENABLE_EXT"),
            Self::COLOR_BLEND_ENABLE_EXT => write!(f, "COLOR_BLEND_ENABLE_EXT"),
            Self::COLOR_BLEND_EQUATION_EXT => write!(f, "COLOR_BLEND_EQUATION_EXT"),
            Self::COLOR_WRITE_MASK_EXT => write!(f, "COLOR_WRITE_MASK_EXT"),
            Self::TESSELLATION_DOMAIN_ORIGIN_EXT => write!(f, "TESSELLATION_DOMAIN_ORIGIN_EXT"),
            Self::RASTERIZATION_STREAM_EXT => write!(f, "RASTERIZATION_STREAM_EXT"),
            Self::CONSERVATIVE_RASTERIZATION_MODE_EXT => {
                write!(f, "CONSERVATIVE_RASTERIZATION_MODE_EXT")
            }
            Self::EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT => {
                write!(f, "EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT")
            }
            Self::DEPTH_CLIP_ENABLE_EXT => write!(f, "DEPTH_CLIP_ENABLE_EXT"),
            Self::SAMPLE_LOCATIONS_ENABLE_EXT => write!(f, "SAMPLE_LOCATIONS_ENABLE_EXT"),
            Self::COLOR_BLEND_ADVANCED_EXT => write!(f, "COLOR_BLEND_ADVANCED_EXT"),
            Self::PROVOKING_VERTEX_MODE_EXT => write!(f, "PROVOKING_VERTEX_MODE_EXT"),
            Self::LINE_RASTERIZATION_MODE_EXT => write!(f, "LINE_RASTERIZATION_MODE_EXT"),
            Self::LINE_STIPPLE_ENABLE_EXT => write!(f, "LINE_STIPPLE_ENABLE_EXT"),
            Self::DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT => {
                write!(f, "DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT")
            }
            Self::VIEWPORT_W_SCALING_ENABLE_NV => write!(f, "VIEWPORT_W_SCALING_ENABLE_NV"),
            Self::VIEWPORT_SWIZZLE_NV => write!(f, "VIEWPORT_SWIZZLE_NV"),
            Self::COVERAGE_TO_COLOR_ENABLE_NV => write!(f, "COVERAGE_TO_COLOR_ENABLE_NV"),
            Self::COVERAGE_TO_COLOR_LOCATION_NV => write!(f, "COVERAGE_TO_COLOR_LOCATION_NV"),
            Self::COVERAGE_MODULATION_MODE_NV => write!(f, "COVERAGE_MODULATION_MODE_NV"),
            Self::COVERAGE_MODULATION_TABLE_ENABLE_NV => {
                write!(f, "COVERAGE_MODULATION_TABLE_ENABLE_NV")
            }
            Self::COVERAGE_MODULATION_TABLE_NV => write!(f, "COVERAGE_MODULATION_TABLE_NV"),
            Self::SHADING_RATE_IMAGE_ENABLE_NV => write!(f, "SHADING_RATE_IMAGE_ENABLE_NV"),
            Self::REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV => {
                write!(f, "REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV")
            }
            Self::COVERAGE_REDUCTION_MODE_NV => write!(f, "COVERAGE_REDUCTION_MODE_NV"),
            Self::ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT => {
                write!(f, "ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT")
            }
            Self::DEPTH_CLAMP_RANGE_EXT => write!(f, "DEPTH_CLAMP_RANGE_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPolygonMode.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PolygonMode(i32);
impl PolygonMode {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PolygonMode {
    pub const FILL: Self = Self(0);
    pub const LINE: Self = Self(1);
    pub const POINT: Self = Self(2);
    pub const FILL_RECTANGLE_NV: Self = Self(1000153000);
}
impl fmt::Display for PolygonMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::FILL => write!(f, "FILL"),
            Self::LINE => write!(f, "LINE"),
            Self::POINT => write!(f, "POINT"),
            Self::FILL_RECTANGLE_NV => write!(f, "FILL_RECTANGLE_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFormat.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Format(i32);
impl Format {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl Format {
    pub const UNDEFINED: Self = Self(0);
    pub const R4G4_UNORM_PACK8: Self = Self(1);
    pub const R4G4B4A4_UNORM_PACK16: Self = Self(2);
    pub const B4G4R4A4_UNORM_PACK16: Self = Self(3);
    pub const R5G6B5_UNORM_PACK16: Self = Self(4);
    pub const B5G6R5_UNORM_PACK16: Self = Self(5);
    pub const R5G5B5A1_UNORM_PACK16: Self = Self(6);
    pub const B5G5R5A1_UNORM_PACK16: Self = Self(7);
    pub const A1R5G5B5_UNORM_PACK16: Self = Self(8);
    pub const R8_UNORM: Self = Self(9);
    pub const R8_SNORM: Self = Self(10);
    pub const R8_USCALED: Self = Self(11);
    pub const R8_SSCALED: Self = Self(12);
    pub const R8_UINT: Self = Self(13);
    pub const R8_SINT: Self = Self(14);
    pub const R8_SRGB: Self = Self(15);
    pub const R8G8_UNORM: Self = Self(16);
    pub const R8G8_SNORM: Self = Self(17);
    pub const R8G8_USCALED: Self = Self(18);
    pub const R8G8_SSCALED: Self = Self(19);
    pub const R8G8_UINT: Self = Self(20);
    pub const R8G8_SINT: Self = Self(21);
    pub const R8G8_SRGB: Self = Self(22);
    pub const R8G8B8_UNORM: Self = Self(23);
    pub const R8G8B8_SNORM: Self = Self(24);
    pub const R8G8B8_USCALED: Self = Self(25);
    pub const R8G8B8_SSCALED: Self = Self(26);
    pub const R8G8B8_UINT: Self = Self(27);
    pub const R8G8B8_SINT: Self = Self(28);
    pub const R8G8B8_SRGB: Self = Self(29);
    pub const B8G8R8_UNORM: Self = Self(30);
    pub const B8G8R8_SNORM: Self = Self(31);
    pub const B8G8R8_USCALED: Self = Self(32);
    pub const B8G8R8_SSCALED: Self = Self(33);
    pub const B8G8R8_UINT: Self = Self(34);
    pub const B8G8R8_SINT: Self = Self(35);
    pub const B8G8R8_SRGB: Self = Self(36);
    pub const R8G8B8A8_UNORM: Self = Self(37);
    pub const R8G8B8A8_SNORM: Self = Self(38);
    pub const R8G8B8A8_USCALED: Self = Self(39);
    pub const R8G8B8A8_SSCALED: Self = Self(40);
    pub const R8G8B8A8_UINT: Self = Self(41);
    pub const R8G8B8A8_SINT: Self = Self(42);
    pub const R8G8B8A8_SRGB: Self = Self(43);
    pub const B8G8R8A8_UNORM: Self = Self(44);
    pub const B8G8R8A8_SNORM: Self = Self(45);
    pub const B8G8R8A8_USCALED: Self = Self(46);
    pub const B8G8R8A8_SSCALED: Self = Self(47);
    pub const B8G8R8A8_UINT: Self = Self(48);
    pub const B8G8R8A8_SINT: Self = Self(49);
    pub const B8G8R8A8_SRGB: Self = Self(50);
    pub const A8B8G8R8_UNORM_PACK32: Self = Self(51);
    pub const A8B8G8R8_SNORM_PACK32: Self = Self(52);
    pub const A8B8G8R8_USCALED_PACK32: Self = Self(53);
    pub const A8B8G8R8_SSCALED_PACK32: Self = Self(54);
    pub const A8B8G8R8_UINT_PACK32: Self = Self(55);
    pub const A8B8G8R8_SINT_PACK32: Self = Self(56);
    pub const A8B8G8R8_SRGB_PACK32: Self = Self(57);
    pub const A2R10G10B10_UNORM_PACK32: Self = Self(58);
    pub const A2R10G10B10_SNORM_PACK32: Self = Self(59);
    pub const A2R10G10B10_USCALED_PACK32: Self = Self(60);
    pub const A2R10G10B10_SSCALED_PACK32: Self = Self(61);
    pub const A2R10G10B10_UINT_PACK32: Self = Self(62);
    pub const A2R10G10B10_SINT_PACK32: Self = Self(63);
    pub const A2B10G10R10_UNORM_PACK32: Self = Self(64);
    pub const A2B10G10R10_SNORM_PACK32: Self = Self(65);
    pub const A2B10G10R10_USCALED_PACK32: Self = Self(66);
    pub const A2B10G10R10_SSCALED_PACK32: Self = Self(67);
    pub const A2B10G10R10_UINT_PACK32: Self = Self(68);
    pub const A2B10G10R10_SINT_PACK32: Self = Self(69);
    pub const R16_UNORM: Self = Self(70);
    pub const R16_SNORM: Self = Self(71);
    pub const R16_USCALED: Self = Self(72);
    pub const R16_SSCALED: Self = Self(73);
    pub const R16_UINT: Self = Self(74);
    pub const R16_SINT: Self = Self(75);
    pub const R16_SFLOAT: Self = Self(76);
    pub const R16G16_UNORM: Self = Self(77);
    pub const R16G16_SNORM: Self = Self(78);
    pub const R16G16_USCALED: Self = Self(79);
    pub const R16G16_SSCALED: Self = Self(80);
    pub const R16G16_UINT: Self = Self(81);
    pub const R16G16_SINT: Self = Self(82);
    pub const R16G16_SFLOAT: Self = Self(83);
    pub const R16G16B16_UNORM: Self = Self(84);
    pub const R16G16B16_SNORM: Self = Self(85);
    pub const R16G16B16_USCALED: Self = Self(86);
    pub const R16G16B16_SSCALED: Self = Self(87);
    pub const R16G16B16_UINT: Self = Self(88);
    pub const R16G16B16_SINT: Self = Self(89);
    pub const R16G16B16_SFLOAT: Self = Self(90);
    pub const R16G16B16A16_UNORM: Self = Self(91);
    pub const R16G16B16A16_SNORM: Self = Self(92);
    pub const R16G16B16A16_USCALED: Self = Self(93);
    pub const R16G16B16A16_SSCALED: Self = Self(94);
    pub const R16G16B16A16_UINT: Self = Self(95);
    pub const R16G16B16A16_SINT: Self = Self(96);
    pub const R16G16B16A16_SFLOAT: Self = Self(97);
    pub const R32_UINT: Self = Self(98);
    pub const R32_SINT: Self = Self(99);
    pub const R32_SFLOAT: Self = Self(100);
    pub const R32G32_UINT: Self = Self(101);
    pub const R32G32_SINT: Self = Self(102);
    pub const R32G32_SFLOAT: Self = Self(103);
    pub const R32G32B32_UINT: Self = Self(104);
    pub const R32G32B32_SINT: Self = Self(105);
    pub const R32G32B32_SFLOAT: Self = Self(106);
    pub const R32G32B32A32_UINT: Self = Self(107);
    pub const R32G32B32A32_SINT: Self = Self(108);
    pub const R32G32B32A32_SFLOAT: Self = Self(109);
    pub const R64_UINT: Self = Self(110);
    pub const R64_SINT: Self = Self(111);
    pub const R64_SFLOAT: Self = Self(112);
    pub const R64G64_UINT: Self = Self(113);
    pub const R64G64_SINT: Self = Self(114);
    pub const R64G64_SFLOAT: Self = Self(115);
    pub const R64G64B64_UINT: Self = Self(116);
    pub const R64G64B64_SINT: Self = Self(117);
    pub const R64G64B64_SFLOAT: Self = Self(118);
    pub const R64G64B64A64_UINT: Self = Self(119);
    pub const R64G64B64A64_SINT: Self = Self(120);
    pub const R64G64B64A64_SFLOAT: Self = Self(121);
    pub const B10G11R11_UFLOAT_PACK32: Self = Self(122);
    pub const E5B9G9R9_UFLOAT_PACK32: Self = Self(123);
    pub const D16_UNORM: Self = Self(124);
    pub const X8_D24_UNORM_PACK32: Self = Self(125);
    pub const D32_SFLOAT: Self = Self(126);
    pub const S8_UINT: Self = Self(127);
    pub const D16_UNORM_S8_UINT: Self = Self(128);
    pub const D24_UNORM_S8_UINT: Self = Self(129);
    pub const D32_SFLOAT_S8_UINT: Self = Self(130);
    pub const BC1_RGB_UNORM_BLOCK: Self = Self(131);
    pub const BC1_RGB_SRGB_BLOCK: Self = Self(132);
    pub const BC1_RGBA_UNORM_BLOCK: Self = Self(133);
    pub const BC1_RGBA_SRGB_BLOCK: Self = Self(134);
    pub const BC2_UNORM_BLOCK: Self = Self(135);
    pub const BC2_SRGB_BLOCK: Self = Self(136);
    pub const BC3_UNORM_BLOCK: Self = Self(137);
    pub const BC3_SRGB_BLOCK: Self = Self(138);
    pub const BC4_UNORM_BLOCK: Self = Self(139);
    pub const BC4_SNORM_BLOCK: Self = Self(140);
    pub const BC5_UNORM_BLOCK: Self = Self(141);
    pub const BC5_SNORM_BLOCK: Self = Self(142);
    pub const BC6H_UFLOAT_BLOCK: Self = Self(143);
    pub const BC6H_SFLOAT_BLOCK: Self = Self(144);
    pub const BC7_UNORM_BLOCK: Self = Self(145);
    pub const BC7_SRGB_BLOCK: Self = Self(146);
    pub const ETC2_R8G8B8_UNORM_BLOCK: Self = Self(147);
    pub const ETC2_R8G8B8_SRGB_BLOCK: Self = Self(148);
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: Self = Self(149);
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: Self = Self(150);
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: Self = Self(151);
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: Self = Self(152);
    pub const EAC_R11_UNORM_BLOCK: Self = Self(153);
    pub const EAC_R11_SNORM_BLOCK: Self = Self(154);
    pub const EAC_R11G11_UNORM_BLOCK: Self = Self(155);
    pub const EAC_R11G11_SNORM_BLOCK: Self = Self(156);
    pub const ASTC_4X4_UNORM_BLOCK: Self = Self(157);
    pub const ASTC_4X4_SRGB_BLOCK: Self = Self(158);
    pub const ASTC_5X4_UNORM_BLOCK: Self = Self(159);
    pub const ASTC_5X4_SRGB_BLOCK: Self = Self(160);
    pub const ASTC_5X5_UNORM_BLOCK: Self = Self(161);
    pub const ASTC_5X5_SRGB_BLOCK: Self = Self(162);
    pub const ASTC_6X5_UNORM_BLOCK: Self = Self(163);
    pub const ASTC_6X5_SRGB_BLOCK: Self = Self(164);
    pub const ASTC_6X6_UNORM_BLOCK: Self = Self(165);
    pub const ASTC_6X6_SRGB_BLOCK: Self = Self(166);
    pub const ASTC_8X5_UNORM_BLOCK: Self = Self(167);
    pub const ASTC_8X5_SRGB_BLOCK: Self = Self(168);
    pub const ASTC_8X6_UNORM_BLOCK: Self = Self(169);
    pub const ASTC_8X6_SRGB_BLOCK: Self = Self(170);
    pub const ASTC_8X8_UNORM_BLOCK: Self = Self(171);
    pub const ASTC_8X8_SRGB_BLOCK: Self = Self(172);
    pub const ASTC_10X5_UNORM_BLOCK: Self = Self(173);
    pub const ASTC_10X5_SRGB_BLOCK: Self = Self(174);
    pub const ASTC_10X6_UNORM_BLOCK: Self = Self(175);
    pub const ASTC_10X6_SRGB_BLOCK: Self = Self(176);
    pub const ASTC_10X8_UNORM_BLOCK: Self = Self(177);
    pub const ASTC_10X8_SRGB_BLOCK: Self = Self(178);
    pub const ASTC_10X10_UNORM_BLOCK: Self = Self(179);
    pub const ASTC_10X10_SRGB_BLOCK: Self = Self(180);
    pub const ASTC_12X10_UNORM_BLOCK: Self = Self(181);
    pub const ASTC_12X10_SRGB_BLOCK: Self = Self(182);
    pub const ASTC_12X12_UNORM_BLOCK: Self = Self(183);
    pub const ASTC_12X12_SRGB_BLOCK: Self = Self(184);
    pub const G8B8G8R8_422_UNORM: Self = Self(1000156000);
    pub const B8G8R8G8_422_UNORM: Self = Self(1000156001);
    pub const G8_B8_R8_3PLANE_420_UNORM: Self = Self(1000156002);
    pub const G8_B8R8_2PLANE_420_UNORM: Self = Self(1000156003);
    pub const G8_B8_R8_3PLANE_422_UNORM: Self = Self(1000156004);
    pub const G8_B8R8_2PLANE_422_UNORM: Self = Self(1000156005);
    pub const G8_B8_R8_3PLANE_444_UNORM: Self = Self(1000156006);
    pub const R10X6_UNORM_PACK16: Self = Self(1000156007);
    pub const R10X6G10X6_UNORM_2PACK16: Self = Self(1000156008);
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16: Self = Self(1000156009);
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: Self = Self(1000156010);
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: Self = Self(1000156011);
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: Self = Self(1000156012);
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: Self = Self(1000156013);
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: Self = Self(1000156014);
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: Self = Self(1000156015);
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: Self = Self(1000156016);
    pub const R12X4_UNORM_PACK16: Self = Self(1000156017);
    pub const R12X4G12X4_UNORM_2PACK16: Self = Self(1000156018);
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16: Self = Self(1000156019);
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: Self = Self(1000156020);
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: Self = Self(1000156021);
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: Self = Self(1000156022);
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: Self = Self(1000156023);
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: Self = Self(1000156024);
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: Self = Self(1000156025);
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: Self = Self(1000156026);
    pub const G16B16G16R16_422_UNORM: Self = Self(1000156027);
    pub const B16G16R16G16_422_UNORM: Self = Self(1000156028);
    pub const G16_B16_R16_3PLANE_420_UNORM: Self = Self(1000156029);
    pub const G16_B16R16_2PLANE_420_UNORM: Self = Self(1000156030);
    pub const G16_B16_R16_3PLANE_422_UNORM: Self = Self(1000156031);
    pub const G16_B16R16_2PLANE_422_UNORM: Self = Self(1000156032);
    pub const G16_B16_R16_3PLANE_444_UNORM: Self = Self(1000156033);
    pub const G8_B8R8_2PLANE_444_UNORM: Self = Self(1000330000);
    pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16: Self = Self(1000330001);
    pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16: Self = Self(1000330002);
    pub const G16_B16R16_2PLANE_444_UNORM: Self = Self(1000330003);
    pub const A4R4G4B4_UNORM_PACK16: Self = Self(1000340000);
    pub const A4B4G4R4_UNORM_PACK16: Self = Self(1000340001);
    pub const ASTC_4X4_SFLOAT_BLOCK: Self = Self(1000066000);
    pub const ASTC_5X4_SFLOAT_BLOCK: Self = Self(1000066001);
    pub const ASTC_5X5_SFLOAT_BLOCK: Self = Self(1000066002);
    pub const ASTC_6X5_SFLOAT_BLOCK: Self = Self(1000066003);
    pub const ASTC_6X6_SFLOAT_BLOCK: Self = Self(1000066004);
    pub const ASTC_8X5_SFLOAT_BLOCK: Self = Self(1000066005);
    pub const ASTC_8X6_SFLOAT_BLOCK: Self = Self(1000066006);
    pub const ASTC_8X8_SFLOAT_BLOCK: Self = Self(1000066007);
    pub const ASTC_10X5_SFLOAT_BLOCK: Self = Self(1000066008);
    pub const ASTC_10X6_SFLOAT_BLOCK: Self = Self(1000066009);
    pub const ASTC_10X8_SFLOAT_BLOCK: Self = Self(1000066010);
    pub const ASTC_10X10_SFLOAT_BLOCK: Self = Self(1000066011);
    pub const ASTC_12X10_SFLOAT_BLOCK: Self = Self(1000066012);
    pub const ASTC_12X12_SFLOAT_BLOCK: Self = Self(1000066013);
    pub const A1B5G5R5_UNORM_PACK16: Self = Self(1000470000);
    pub const A8_UNORM: Self = Self(1000470001);
    pub const PVRTC1_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054000);
    pub const PVRTC1_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054001);
    pub const PVRTC2_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054002);
    pub const PVRTC2_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054003);
    pub const PVRTC1_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054004);
    pub const PVRTC1_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054005);
    pub const PVRTC2_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054006);
    pub const PVRTC2_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054007);
    pub const ASTC_4X4_SFLOAT_BLOCK_EXT: Self = Self::ASTC_4X4_SFLOAT_BLOCK;
    pub const ASTC_5X4_SFLOAT_BLOCK_EXT: Self = Self::ASTC_5X4_SFLOAT_BLOCK;
    pub const ASTC_5X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_5X5_SFLOAT_BLOCK;
    pub const ASTC_6X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_6X5_SFLOAT_BLOCK;
    pub const ASTC_6X6_SFLOAT_BLOCK_EXT: Self = Self::ASTC_6X6_SFLOAT_BLOCK;
    pub const ASTC_8X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_8X5_SFLOAT_BLOCK;
    pub const ASTC_8X6_SFLOAT_BLOCK_EXT: Self = Self::ASTC_8X6_SFLOAT_BLOCK;
    pub const ASTC_8X8_SFLOAT_BLOCK_EXT: Self = Self::ASTC_8X8_SFLOAT_BLOCK;
    pub const ASTC_10X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X5_SFLOAT_BLOCK;
    pub const ASTC_10X6_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X6_SFLOAT_BLOCK;
    pub const ASTC_10X8_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X8_SFLOAT_BLOCK;
    pub const ASTC_10X10_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X10_SFLOAT_BLOCK;
    pub const ASTC_12X10_SFLOAT_BLOCK_EXT: Self = Self::ASTC_12X10_SFLOAT_BLOCK;
    pub const ASTC_12X12_SFLOAT_BLOCK_EXT: Self = Self::ASTC_12X12_SFLOAT_BLOCK;
    pub const G8B8G8R8_422_UNORM_KHR: Self = Self::G8B8G8R8_422_UNORM;
    pub const B8G8R8G8_422_UNORM_KHR: Self = Self::B8G8R8G8_422_UNORM;
    pub const G8_B8_R8_3PLANE_420_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_420_UNORM;
    pub const G8_B8R8_2PLANE_420_UNORM_KHR: Self = Self::G8_B8R8_2PLANE_420_UNORM;
    pub const G8_B8_R8_3PLANE_422_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_422_UNORM;
    pub const G8_B8R8_2PLANE_422_UNORM_KHR: Self = Self::G8_B8R8_2PLANE_422_UNORM;
    pub const G8_B8_R8_3PLANE_444_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_444_UNORM;
    pub const R10X6_UNORM_PACK16_KHR: Self = Self::R10X6_UNORM_PACK16;
    pub const R10X6G10X6_UNORM_2PACK16_KHR: Self = Self::R10X6G10X6_UNORM_2PACK16;
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: Self =
        Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16;
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: Self =
        Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16;
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: Self =
        Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16;
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16;
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16;
    pub const R12X4_UNORM_PACK16_KHR: Self = Self::R12X4_UNORM_PACK16;
    pub const R12X4G12X4_UNORM_2PACK16_KHR: Self = Self::R12X4G12X4_UNORM_2PACK16;
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: Self =
        Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16;
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: Self =
        Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16;
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: Self =
        Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16;
    pub const G16B16G16R16_422_UNORM_KHR: Self = Self::G16B16G16R16_422_UNORM;
    pub const B16G16R16G16_422_UNORM_KHR: Self = Self::B16G16R16G16_422_UNORM;
    pub const G16_B16_R16_3PLANE_420_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_420_UNORM;
    pub const G16_B16R16_2PLANE_420_UNORM_KHR: Self = Self::G16_B16R16_2PLANE_420_UNORM;
    pub const G16_B16_R16_3PLANE_422_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_422_UNORM;
    pub const G16_B16R16_2PLANE_422_UNORM_KHR: Self = Self::G16_B16R16_2PLANE_422_UNORM;
    pub const G16_B16_R16_3PLANE_444_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_444_UNORM;
    pub const ASTC_3X3X3_UNORM_BLOCK_EXT: Self = Self(1000288000);
    pub const ASTC_3X3X3_SRGB_BLOCK_EXT: Self = Self(1000288001);
    pub const ASTC_3X3X3_SFLOAT_BLOCK_EXT: Self = Self(1000288002);
    pub const ASTC_4X3X3_UNORM_BLOCK_EXT: Self = Self(1000288003);
    pub const ASTC_4X3X3_SRGB_BLOCK_EXT: Self = Self(1000288004);
    pub const ASTC_4X3X3_SFLOAT_BLOCK_EXT: Self = Self(1000288005);
    pub const ASTC_4X4X3_UNORM_BLOCK_EXT: Self = Self(1000288006);
    pub const ASTC_4X4X3_SRGB_BLOCK_EXT: Self = Self(1000288007);
    pub const ASTC_4X4X3_SFLOAT_BLOCK_EXT: Self = Self(1000288008);
    pub const ASTC_4X4X4_UNORM_BLOCK_EXT: Self = Self(1000288009);
    pub const ASTC_4X4X4_SRGB_BLOCK_EXT: Self = Self(1000288010);
    pub const ASTC_4X4X4_SFLOAT_BLOCK_EXT: Self = Self(1000288011);
    pub const ASTC_5X4X4_UNORM_BLOCK_EXT: Self = Self(1000288012);
    pub const ASTC_5X4X4_SRGB_BLOCK_EXT: Self = Self(1000288013);
    pub const ASTC_5X4X4_SFLOAT_BLOCK_EXT: Self = Self(1000288014);
    pub const ASTC_5X5X4_UNORM_BLOCK_EXT: Self = Self(1000288015);
    pub const ASTC_5X5X4_SRGB_BLOCK_EXT: Self = Self(1000288016);
    pub const ASTC_5X5X4_SFLOAT_BLOCK_EXT: Self = Self(1000288017);
    pub const ASTC_5X5X5_UNORM_BLOCK_EXT: Self = Self(1000288018);
    pub const ASTC_5X5X5_SRGB_BLOCK_EXT: Self = Self(1000288019);
    pub const ASTC_5X5X5_SFLOAT_BLOCK_EXT: Self = Self(1000288020);
    pub const ASTC_6X5X5_UNORM_BLOCK_EXT: Self = Self(1000288021);
    pub const ASTC_6X5X5_SRGB_BLOCK_EXT: Self = Self(1000288022);
    pub const ASTC_6X5X5_SFLOAT_BLOCK_EXT: Self = Self(1000288023);
    pub const ASTC_6X6X5_UNORM_BLOCK_EXT: Self = Self(1000288024);
    pub const ASTC_6X6X5_SRGB_BLOCK_EXT: Self = Self(1000288025);
    pub const ASTC_6X6X5_SFLOAT_BLOCK_EXT: Self = Self(1000288026);
    pub const ASTC_6X6X6_UNORM_BLOCK_EXT: Self = Self(1000288027);
    pub const ASTC_6X6X6_SRGB_BLOCK_EXT: Self = Self(1000288028);
    pub const ASTC_6X6X6_SFLOAT_BLOCK_EXT: Self = Self(1000288029);
    pub const G8_B8R8_2PLANE_444_UNORM_EXT: Self = Self::G8_B8R8_2PLANE_444_UNORM;
    pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT: Self =
        Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT: Self =
        Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16;
    pub const G16_B16R16_2PLANE_444_UNORM_EXT: Self = Self::G16_B16R16_2PLANE_444_UNORM;
    pub const A4R4G4B4_UNORM_PACK16_EXT: Self = Self::A4R4G4B4_UNORM_PACK16;
    pub const A4B4G4R4_UNORM_PACK16_EXT: Self = Self::A4B4G4R4_UNORM_PACK16;
    pub const R8_BOOL_ARM: Self = Self(1000460000);
    pub const R16_SFLOAT_FPENCODING_BFLOAT16_ARM: Self = Self(1000460001);
    pub const R8_SFLOAT_FPENCODING_FLOAT8E4M3_ARM: Self = Self(1000460002);
    pub const R8_SFLOAT_FPENCODING_FLOAT8E5M2_ARM: Self = Self(1000460003);
    pub const R16G16_SFIXED5_NV: Self = Self(1000464000);
    #[deprecated = "aliased"]
    pub const R16G16_S10_5_NV: Self = Self::R16G16_SFIXED5_NV;
    pub const A1B5G5R5_UNORM_PACK16_KHR: Self = Self::A1B5G5R5_UNORM_PACK16;
    pub const A8_UNORM_KHR: Self = Self::A8_UNORM;
    pub const R10X6_UINT_PACK16_ARM: Self = Self(1000609000);
    pub const R10X6G10X6_UINT_2PACK16_ARM: Self = Self(1000609001);
    pub const R10X6G10X6B10X6A10X6_UINT_4PACK16_ARM: Self = Self(1000609002);
    pub const R12X4_UINT_PACK16_ARM: Self = Self(1000609003);
    pub const R12X4G12X4_UINT_2PACK16_ARM: Self = Self(1000609004);
    pub const R12X4G12X4B12X4A12X4_UINT_4PACK16_ARM: Self = Self(1000609005);
    pub const R14X2_UINT_PACK16_ARM: Self = Self(1000609006);
    pub const R14X2G14X2_UINT_2PACK16_ARM: Self = Self(1000609007);
    pub const R14X2G14X2B14X2A14X2_UINT_4PACK16_ARM: Self = Self(1000609008);
    pub const R14X2_UNORM_PACK16_ARM: Self = Self(1000609009);
    pub const R14X2G14X2_UNORM_2PACK16_ARM: Self = Self(1000609010);
    pub const R14X2G14X2B14X2A14X2_UNORM_4PACK16_ARM: Self = Self(1000609011);
    pub const G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16_ARM: Self = Self(1000609012);
    pub const G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16_ARM: Self = Self(1000609013);
}
impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UNDEFINED => write!(f, "UNDEFINED"),
            Self::R4G4_UNORM_PACK8 => write!(f, "R4G4_UNORM_PACK8"),
            Self::R4G4B4A4_UNORM_PACK16 => write!(f, "R4G4B4A4_UNORM_PACK16"),
            Self::B4G4R4A4_UNORM_PACK16 => write!(f, "B4G4R4A4_UNORM_PACK16"),
            Self::R5G6B5_UNORM_PACK16 => write!(f, "R5G6B5_UNORM_PACK16"),
            Self::B5G6R5_UNORM_PACK16 => write!(f, "B5G6R5_UNORM_PACK16"),
            Self::R5G5B5A1_UNORM_PACK16 => write!(f, "R5G5B5A1_UNORM_PACK16"),
            Self::B5G5R5A1_UNORM_PACK16 => write!(f, "B5G5R5A1_UNORM_PACK16"),
            Self::A1R5G5B5_UNORM_PACK16 => write!(f, "A1R5G5B5_UNORM_PACK16"),
            Self::R8_UNORM => write!(f, "R8_UNORM"),
            Self::R8_SNORM => write!(f, "R8_SNORM"),
            Self::R8_USCALED => write!(f, "R8_USCALED"),
            Self::R8_SSCALED => write!(f, "R8_SSCALED"),
            Self::R8_UINT => write!(f, "R8_UINT"),
            Self::R8_SINT => write!(f, "R8_SINT"),
            Self::R8_SRGB => write!(f, "R8_SRGB"),
            Self::R8G8_UNORM => write!(f, "R8G8_UNORM"),
            Self::R8G8_SNORM => write!(f, "R8G8_SNORM"),
            Self::R8G8_USCALED => write!(f, "R8G8_USCALED"),
            Self::R8G8_SSCALED => write!(f, "R8G8_SSCALED"),
            Self::R8G8_UINT => write!(f, "R8G8_UINT"),
            Self::R8G8_SINT => write!(f, "R8G8_SINT"),
            Self::R8G8_SRGB => write!(f, "R8G8_SRGB"),
            Self::R8G8B8_UNORM => write!(f, "R8G8B8_UNORM"),
            Self::R8G8B8_SNORM => write!(f, "R8G8B8_SNORM"),
            Self::R8G8B8_USCALED => write!(f, "R8G8B8_USCALED"),
            Self::R8G8B8_SSCALED => write!(f, "R8G8B8_SSCALED"),
            Self::R8G8B8_UINT => write!(f, "R8G8B8_UINT"),
            Self::R8G8B8_SINT => write!(f, "R8G8B8_SINT"),
            Self::R8G8B8_SRGB => write!(f, "R8G8B8_SRGB"),
            Self::B8G8R8_UNORM => write!(f, "B8G8R8_UNORM"),
            Self::B8G8R8_SNORM => write!(f, "B8G8R8_SNORM"),
            Self::B8G8R8_USCALED => write!(f, "B8G8R8_USCALED"),
            Self::B8G8R8_SSCALED => write!(f, "B8G8R8_SSCALED"),
            Self::B8G8R8_UINT => write!(f, "B8G8R8_UINT"),
            Self::B8G8R8_SINT => write!(f, "B8G8R8_SINT"),
            Self::B8G8R8_SRGB => write!(f, "B8G8R8_SRGB"),
            Self::R8G8B8A8_UNORM => write!(f, "R8G8B8A8_UNORM"),
            Self::R8G8B8A8_SNORM => write!(f, "R8G8B8A8_SNORM"),
            Self::R8G8B8A8_USCALED => write!(f, "R8G8B8A8_USCALED"),
            Self::R8G8B8A8_SSCALED => write!(f, "R8G8B8A8_SSCALED"),
            Self::R8G8B8A8_UINT => write!(f, "R8G8B8A8_UINT"),
            Self::R8G8B8A8_SINT => write!(f, "R8G8B8A8_SINT"),
            Self::R8G8B8A8_SRGB => write!(f, "R8G8B8A8_SRGB"),
            Self::B8G8R8A8_UNORM => write!(f, "B8G8R8A8_UNORM"),
            Self::B8G8R8A8_SNORM => write!(f, "B8G8R8A8_SNORM"),
            Self::B8G8R8A8_USCALED => write!(f, "B8G8R8A8_USCALED"),
            Self::B8G8R8A8_SSCALED => write!(f, "B8G8R8A8_SSCALED"),
            Self::B8G8R8A8_UINT => write!(f, "B8G8R8A8_UINT"),
            Self::B8G8R8A8_SINT => write!(f, "B8G8R8A8_SINT"),
            Self::B8G8R8A8_SRGB => write!(f, "B8G8R8A8_SRGB"),
            Self::A8B8G8R8_UNORM_PACK32 => write!(f, "A8B8G8R8_UNORM_PACK32"),
            Self::A8B8G8R8_SNORM_PACK32 => write!(f, "A8B8G8R8_SNORM_PACK32"),
            Self::A8B8G8R8_USCALED_PACK32 => write!(f, "A8B8G8R8_USCALED_PACK32"),
            Self::A8B8G8R8_SSCALED_PACK32 => write!(f, "A8B8G8R8_SSCALED_PACK32"),
            Self::A8B8G8R8_UINT_PACK32 => write!(f, "A8B8G8R8_UINT_PACK32"),
            Self::A8B8G8R8_SINT_PACK32 => write!(f, "A8B8G8R8_SINT_PACK32"),
            Self::A8B8G8R8_SRGB_PACK32 => write!(f, "A8B8G8R8_SRGB_PACK32"),
            Self::A2R10G10B10_UNORM_PACK32 => write!(f, "A2R10G10B10_UNORM_PACK32"),
            Self::A2R10G10B10_SNORM_PACK32 => write!(f, "A2R10G10B10_SNORM_PACK32"),
            Self::A2R10G10B10_USCALED_PACK32 => write!(f, "A2R10G10B10_USCALED_PACK32"),
            Self::A2R10G10B10_SSCALED_PACK32 => write!(f, "A2R10G10B10_SSCALED_PACK32"),
            Self::A2R10G10B10_UINT_PACK32 => write!(f, "A2R10G10B10_UINT_PACK32"),
            Self::A2R10G10B10_SINT_PACK32 => write!(f, "A2R10G10B10_SINT_PACK32"),
            Self::A2B10G10R10_UNORM_PACK32 => write!(f, "A2B10G10R10_UNORM_PACK32"),
            Self::A2B10G10R10_SNORM_PACK32 => write!(f, "A2B10G10R10_SNORM_PACK32"),
            Self::A2B10G10R10_USCALED_PACK32 => write!(f, "A2B10G10R10_USCALED_PACK32"),
            Self::A2B10G10R10_SSCALED_PACK32 => write!(f, "A2B10G10R10_SSCALED_PACK32"),
            Self::A2B10G10R10_UINT_PACK32 => write!(f, "A2B10G10R10_UINT_PACK32"),
            Self::A2B10G10R10_SINT_PACK32 => write!(f, "A2B10G10R10_SINT_PACK32"),
            Self::R16_UNORM => write!(f, "R16_UNORM"),
            Self::R16_SNORM => write!(f, "R16_SNORM"),
            Self::R16_USCALED => write!(f, "R16_USCALED"),
            Self::R16_SSCALED => write!(f, "R16_SSCALED"),
            Self::R16_UINT => write!(f, "R16_UINT"),
            Self::R16_SINT => write!(f, "R16_SINT"),
            Self::R16_SFLOAT => write!(f, "R16_SFLOAT"),
            Self::R16G16_UNORM => write!(f, "R16G16_UNORM"),
            Self::R16G16_SNORM => write!(f, "R16G16_SNORM"),
            Self::R16G16_USCALED => write!(f, "R16G16_USCALED"),
            Self::R16G16_SSCALED => write!(f, "R16G16_SSCALED"),
            Self::R16G16_UINT => write!(f, "R16G16_UINT"),
            Self::R16G16_SINT => write!(f, "R16G16_SINT"),
            Self::R16G16_SFLOAT => write!(f, "R16G16_SFLOAT"),
            Self::R16G16B16_UNORM => write!(f, "R16G16B16_UNORM"),
            Self::R16G16B16_SNORM => write!(f, "R16G16B16_SNORM"),
            Self::R16G16B16_USCALED => write!(f, "R16G16B16_USCALED"),
            Self::R16G16B16_SSCALED => write!(f, "R16G16B16_SSCALED"),
            Self::R16G16B16_UINT => write!(f, "R16G16B16_UINT"),
            Self::R16G16B16_SINT => write!(f, "R16G16B16_SINT"),
            Self::R16G16B16_SFLOAT => write!(f, "R16G16B16_SFLOAT"),
            Self::R16G16B16A16_UNORM => write!(f, "R16G16B16A16_UNORM"),
            Self::R16G16B16A16_SNORM => write!(f, "R16G16B16A16_SNORM"),
            Self::R16G16B16A16_USCALED => write!(f, "R16G16B16A16_USCALED"),
            Self::R16G16B16A16_SSCALED => write!(f, "R16G16B16A16_SSCALED"),
            Self::R16G16B16A16_UINT => write!(f, "R16G16B16A16_UINT"),
            Self::R16G16B16A16_SINT => write!(f, "R16G16B16A16_SINT"),
            Self::R16G16B16A16_SFLOAT => write!(f, "R16G16B16A16_SFLOAT"),
            Self::R32_UINT => write!(f, "R32_UINT"),
            Self::R32_SINT => write!(f, "R32_SINT"),
            Self::R32_SFLOAT => write!(f, "R32_SFLOAT"),
            Self::R32G32_UINT => write!(f, "R32G32_UINT"),
            Self::R32G32_SINT => write!(f, "R32G32_SINT"),
            Self::R32G32_SFLOAT => write!(f, "R32G32_SFLOAT"),
            Self::R32G32B32_UINT => write!(f, "R32G32B32_UINT"),
            Self::R32G32B32_SINT => write!(f, "R32G32B32_SINT"),
            Self::R32G32B32_SFLOAT => write!(f, "R32G32B32_SFLOAT"),
            Self::R32G32B32A32_UINT => write!(f, "R32G32B32A32_UINT"),
            Self::R32G32B32A32_SINT => write!(f, "R32G32B32A32_SINT"),
            Self::R32G32B32A32_SFLOAT => write!(f, "R32G32B32A32_SFLOAT"),
            Self::R64_UINT => write!(f, "R64_UINT"),
            Self::R64_SINT => write!(f, "R64_SINT"),
            Self::R64_SFLOAT => write!(f, "R64_SFLOAT"),
            Self::R64G64_UINT => write!(f, "R64G64_UINT"),
            Self::R64G64_SINT => write!(f, "R64G64_SINT"),
            Self::R64G64_SFLOAT => write!(f, "R64G64_SFLOAT"),
            Self::R64G64B64_UINT => write!(f, "R64G64B64_UINT"),
            Self::R64G64B64_SINT => write!(f, "R64G64B64_SINT"),
            Self::R64G64B64_SFLOAT => write!(f, "R64G64B64_SFLOAT"),
            Self::R64G64B64A64_UINT => write!(f, "R64G64B64A64_UINT"),
            Self::R64G64B64A64_SINT => write!(f, "R64G64B64A64_SINT"),
            Self::R64G64B64A64_SFLOAT => write!(f, "R64G64B64A64_SFLOAT"),
            Self::B10G11R11_UFLOAT_PACK32 => write!(f, "B10G11R11_UFLOAT_PACK32"),
            Self::E5B9G9R9_UFLOAT_PACK32 => write!(f, "E5B9G9R9_UFLOAT_PACK32"),
            Self::D16_UNORM => write!(f, "D16_UNORM"),
            Self::X8_D24_UNORM_PACK32 => write!(f, "X8_D24_UNORM_PACK32"),
            Self::D32_SFLOAT => write!(f, "D32_SFLOAT"),
            Self::S8_UINT => write!(f, "S8_UINT"),
            Self::D16_UNORM_S8_UINT => write!(f, "D16_UNORM_S8_UINT"),
            Self::D24_UNORM_S8_UINT => write!(f, "D24_UNORM_S8_UINT"),
            Self::D32_SFLOAT_S8_UINT => write!(f, "D32_SFLOAT_S8_UINT"),
            Self::BC1_RGB_UNORM_BLOCK => write!(f, "BC1_RGB_UNORM_BLOCK"),
            Self::BC1_RGB_SRGB_BLOCK => write!(f, "BC1_RGB_SRGB_BLOCK"),
            Self::BC1_RGBA_UNORM_BLOCK => write!(f, "BC1_RGBA_UNORM_BLOCK"),
            Self::BC1_RGBA_SRGB_BLOCK => write!(f, "BC1_RGBA_SRGB_BLOCK"),
            Self::BC2_UNORM_BLOCK => write!(f, "BC2_UNORM_BLOCK"),
            Self::BC2_SRGB_BLOCK => write!(f, "BC2_SRGB_BLOCK"),
            Self::BC3_UNORM_BLOCK => write!(f, "BC3_UNORM_BLOCK"),
            Self::BC3_SRGB_BLOCK => write!(f, "BC3_SRGB_BLOCK"),
            Self::BC4_UNORM_BLOCK => write!(f, "BC4_UNORM_BLOCK"),
            Self::BC4_SNORM_BLOCK => write!(f, "BC4_SNORM_BLOCK"),
            Self::BC5_UNORM_BLOCK => write!(f, "BC5_UNORM_BLOCK"),
            Self::BC5_SNORM_BLOCK => write!(f, "BC5_SNORM_BLOCK"),
            Self::BC6H_UFLOAT_BLOCK => write!(f, "BC6H_UFLOAT_BLOCK"),
            Self::BC6H_SFLOAT_BLOCK => write!(f, "BC6H_SFLOAT_BLOCK"),
            Self::BC7_UNORM_BLOCK => write!(f, "BC7_UNORM_BLOCK"),
            Self::BC7_SRGB_BLOCK => write!(f, "BC7_SRGB_BLOCK"),
            Self::ETC2_R8G8B8_UNORM_BLOCK => write!(f, "ETC2_R8G8B8_UNORM_BLOCK"),
            Self::ETC2_R8G8B8_SRGB_BLOCK => write!(f, "ETC2_R8G8B8_SRGB_BLOCK"),
            Self::ETC2_R8G8B8A1_UNORM_BLOCK => write!(f, "ETC2_R8G8B8A1_UNORM_BLOCK"),
            Self::ETC2_R8G8B8A1_SRGB_BLOCK => write!(f, "ETC2_R8G8B8A1_SRGB_BLOCK"),
            Self::ETC2_R8G8B8A8_UNORM_BLOCK => write!(f, "ETC2_R8G8B8A8_UNORM_BLOCK"),
            Self::ETC2_R8G8B8A8_SRGB_BLOCK => write!(f, "ETC2_R8G8B8A8_SRGB_BLOCK"),
            Self::EAC_R11_UNORM_BLOCK => write!(f, "EAC_R11_UNORM_BLOCK"),
            Self::EAC_R11_SNORM_BLOCK => write!(f, "EAC_R11_SNORM_BLOCK"),
            Self::EAC_R11G11_UNORM_BLOCK => write!(f, "EAC_R11G11_UNORM_BLOCK"),
            Self::EAC_R11G11_SNORM_BLOCK => write!(f, "EAC_R11G11_SNORM_BLOCK"),
            Self::ASTC_4X4_UNORM_BLOCK => write!(f, "ASTC_4X4_UNORM_BLOCK"),
            Self::ASTC_4X4_SRGB_BLOCK => write!(f, "ASTC_4X4_SRGB_BLOCK"),
            Self::ASTC_5X4_UNORM_BLOCK => write!(f, "ASTC_5X4_UNORM_BLOCK"),
            Self::ASTC_5X4_SRGB_BLOCK => write!(f, "ASTC_5X4_SRGB_BLOCK"),
            Self::ASTC_5X5_UNORM_BLOCK => write!(f, "ASTC_5X5_UNORM_BLOCK"),
            Self::ASTC_5X5_SRGB_BLOCK => write!(f, "ASTC_5X5_SRGB_BLOCK"),
            Self::ASTC_6X5_UNORM_BLOCK => write!(f, "ASTC_6X5_UNORM_BLOCK"),
            Self::ASTC_6X5_SRGB_BLOCK => write!(f, "ASTC_6X5_SRGB_BLOCK"),
            Self::ASTC_6X6_UNORM_BLOCK => write!(f, "ASTC_6X6_UNORM_BLOCK"),
            Self::ASTC_6X6_SRGB_BLOCK => write!(f, "ASTC_6X6_SRGB_BLOCK"),
            Self::ASTC_8X5_UNORM_BLOCK => write!(f, "ASTC_8X5_UNORM_BLOCK"),
            Self::ASTC_8X5_SRGB_BLOCK => write!(f, "ASTC_8X5_SRGB_BLOCK"),
            Self::ASTC_8X6_UNORM_BLOCK => write!(f, "ASTC_8X6_UNORM_BLOCK"),
            Self::ASTC_8X6_SRGB_BLOCK => write!(f, "ASTC_8X6_SRGB_BLOCK"),
            Self::ASTC_8X8_UNORM_BLOCK => write!(f, "ASTC_8X8_UNORM_BLOCK"),
            Self::ASTC_8X8_SRGB_BLOCK => write!(f, "ASTC_8X8_SRGB_BLOCK"),
            Self::ASTC_10X5_UNORM_BLOCK => write!(f, "ASTC_10X5_UNORM_BLOCK"),
            Self::ASTC_10X5_SRGB_BLOCK => write!(f, "ASTC_10X5_SRGB_BLOCK"),
            Self::ASTC_10X6_UNORM_BLOCK => write!(f, "ASTC_10X6_UNORM_BLOCK"),
            Self::ASTC_10X6_SRGB_BLOCK => write!(f, "ASTC_10X6_SRGB_BLOCK"),
            Self::ASTC_10X8_UNORM_BLOCK => write!(f, "ASTC_10X8_UNORM_BLOCK"),
            Self::ASTC_10X8_SRGB_BLOCK => write!(f, "ASTC_10X8_SRGB_BLOCK"),
            Self::ASTC_10X10_UNORM_BLOCK => write!(f, "ASTC_10X10_UNORM_BLOCK"),
            Self::ASTC_10X10_SRGB_BLOCK => write!(f, "ASTC_10X10_SRGB_BLOCK"),
            Self::ASTC_12X10_UNORM_BLOCK => write!(f, "ASTC_12X10_UNORM_BLOCK"),
            Self::ASTC_12X10_SRGB_BLOCK => write!(f, "ASTC_12X10_SRGB_BLOCK"),
            Self::ASTC_12X12_UNORM_BLOCK => write!(f, "ASTC_12X12_UNORM_BLOCK"),
            Self::ASTC_12X12_SRGB_BLOCK => write!(f, "ASTC_12X12_SRGB_BLOCK"),
            Self::G8B8G8R8_422_UNORM => write!(f, "G8B8G8R8_422_UNORM"),
            Self::B8G8R8G8_422_UNORM => write!(f, "B8G8R8G8_422_UNORM"),
            Self::G8_B8_R8_3PLANE_420_UNORM => write!(f, "G8_B8_R8_3PLANE_420_UNORM"),
            Self::G8_B8R8_2PLANE_420_UNORM => write!(f, "G8_B8R8_2PLANE_420_UNORM"),
            Self::G8_B8_R8_3PLANE_422_UNORM => write!(f, "G8_B8_R8_3PLANE_422_UNORM"),
            Self::G8_B8R8_2PLANE_422_UNORM => write!(f, "G8_B8R8_2PLANE_422_UNORM"),
            Self::G8_B8_R8_3PLANE_444_UNORM => write!(f, "G8_B8_R8_3PLANE_444_UNORM"),
            Self::R10X6_UNORM_PACK16 => write!(f, "R10X6_UNORM_PACK16"),
            Self::R10X6G10X6_UNORM_2PACK16 => write!(f, "R10X6G10X6_UNORM_2PACK16"),
            Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => {
                write!(f, "R10X6G10X6B10X6A10X6_UNORM_4PACK16")
            }
            Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => {
                write!(f, "G10X6B10X6G10X6R10X6_422_UNORM_4PACK16")
            }
            Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => {
                write!(f, "B10X6G10X6R10X6G10X6_422_UNORM_4PACK16")
            }
            Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => {
                write!(f, "G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16")
            }
            Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => {
                write!(f, "G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16")
            }
            Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => {
                write!(f, "G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16")
            }
            Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => {
                write!(f, "G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16")
            }
            Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => {
                write!(f, "G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16")
            }
            Self::R12X4_UNORM_PACK16 => write!(f, "R12X4_UNORM_PACK16"),
            Self::R12X4G12X4_UNORM_2PACK16 => write!(f, "R12X4G12X4_UNORM_2PACK16"),
            Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => {
                write!(f, "R12X4G12X4B12X4A12X4_UNORM_4PACK16")
            }
            Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => {
                write!(f, "G12X4B12X4G12X4R12X4_422_UNORM_4PACK16")
            }
            Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => {
                write!(f, "B12X4G12X4R12X4G12X4_422_UNORM_4PACK16")
            }
            Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => {
                write!(f, "G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16")
            }
            Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => {
                write!(f, "G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16")
            }
            Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => {
                write!(f, "G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16")
            }
            Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => {
                write!(f, "G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16")
            }
            Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => {
                write!(f, "G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16")
            }
            Self::G16B16G16R16_422_UNORM => write!(f, "G16B16G16R16_422_UNORM"),
            Self::B16G16R16G16_422_UNORM => write!(f, "B16G16R16G16_422_UNORM"),
            Self::G16_B16_R16_3PLANE_420_UNORM => write!(f, "G16_B16_R16_3PLANE_420_UNORM"),
            Self::G16_B16R16_2PLANE_420_UNORM => write!(f, "G16_B16R16_2PLANE_420_UNORM"),
            Self::G16_B16_R16_3PLANE_422_UNORM => write!(f, "G16_B16_R16_3PLANE_422_UNORM"),
            Self::G16_B16R16_2PLANE_422_UNORM => write!(f, "G16_B16R16_2PLANE_422_UNORM"),
            Self::G16_B16_R16_3PLANE_444_UNORM => write!(f, "G16_B16_R16_3PLANE_444_UNORM"),
            Self::G8_B8R8_2PLANE_444_UNORM => write!(f, "G8_B8R8_2PLANE_444_UNORM"),
            Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16 => {
                write!(f, "G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16")
            }
            Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16 => {
                write!(f, "G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16")
            }
            Self::G16_B16R16_2PLANE_444_UNORM => write!(f, "G16_B16R16_2PLANE_444_UNORM"),
            Self::A4R4G4B4_UNORM_PACK16 => write!(f, "A4R4G4B4_UNORM_PACK16"),
            Self::A4B4G4R4_UNORM_PACK16 => write!(f, "A4B4G4R4_UNORM_PACK16"),
            Self::ASTC_4X4_SFLOAT_BLOCK => write!(f, "ASTC_4X4_SFLOAT_BLOCK"),
            Self::ASTC_5X4_SFLOAT_BLOCK => write!(f, "ASTC_5X4_SFLOAT_BLOCK"),
            Self::ASTC_5X5_SFLOAT_BLOCK => write!(f, "ASTC_5X5_SFLOAT_BLOCK"),
            Self::ASTC_6X5_SFLOAT_BLOCK => write!(f, "ASTC_6X5_SFLOAT_BLOCK"),
            Self::ASTC_6X6_SFLOAT_BLOCK => write!(f, "ASTC_6X6_SFLOAT_BLOCK"),
            Self::ASTC_8X5_SFLOAT_BLOCK => write!(f, "ASTC_8X5_SFLOAT_BLOCK"),
            Self::ASTC_8X6_SFLOAT_BLOCK => write!(f, "ASTC_8X6_SFLOAT_BLOCK"),
            Self::ASTC_8X8_SFLOAT_BLOCK => write!(f, "ASTC_8X8_SFLOAT_BLOCK"),
            Self::ASTC_10X5_SFLOAT_BLOCK => write!(f, "ASTC_10X5_SFLOAT_BLOCK"),
            Self::ASTC_10X6_SFLOAT_BLOCK => write!(f, "ASTC_10X6_SFLOAT_BLOCK"),
            Self::ASTC_10X8_SFLOAT_BLOCK => write!(f, "ASTC_10X8_SFLOAT_BLOCK"),
            Self::ASTC_10X10_SFLOAT_BLOCK => write!(f, "ASTC_10X10_SFLOAT_BLOCK"),
            Self::ASTC_12X10_SFLOAT_BLOCK => write!(f, "ASTC_12X10_SFLOAT_BLOCK"),
            Self::ASTC_12X12_SFLOAT_BLOCK => write!(f, "ASTC_12X12_SFLOAT_BLOCK"),
            Self::A1B5G5R5_UNORM_PACK16 => write!(f, "A1B5G5R5_UNORM_PACK16"),
            Self::A8_UNORM => write!(f, "A8_UNORM"),
            Self::PVRTC1_2BPP_UNORM_BLOCK_IMG => write!(f, "PVRTC1_2BPP_UNORM_BLOCK_IMG"),
            Self::PVRTC1_4BPP_UNORM_BLOCK_IMG => write!(f, "PVRTC1_4BPP_UNORM_BLOCK_IMG"),
            Self::PVRTC2_2BPP_UNORM_BLOCK_IMG => write!(f, "PVRTC2_2BPP_UNORM_BLOCK_IMG"),
            Self::PVRTC2_4BPP_UNORM_BLOCK_IMG => write!(f, "PVRTC2_4BPP_UNORM_BLOCK_IMG"),
            Self::PVRTC1_2BPP_SRGB_BLOCK_IMG => write!(f, "PVRTC1_2BPP_SRGB_BLOCK_IMG"),
            Self::PVRTC1_4BPP_SRGB_BLOCK_IMG => write!(f, "PVRTC1_4BPP_SRGB_BLOCK_IMG"),
            Self::PVRTC2_2BPP_SRGB_BLOCK_IMG => write!(f, "PVRTC2_2BPP_SRGB_BLOCK_IMG"),
            Self::PVRTC2_4BPP_SRGB_BLOCK_IMG => write!(f, "PVRTC2_4BPP_SRGB_BLOCK_IMG"),
            Self::ASTC_3X3X3_UNORM_BLOCK_EXT => write!(f, "ASTC_3X3X3_UNORM_BLOCK_EXT"),
            Self::ASTC_3X3X3_SRGB_BLOCK_EXT => write!(f, "ASTC_3X3X3_SRGB_BLOCK_EXT"),
            Self::ASTC_3X3X3_SFLOAT_BLOCK_EXT => write!(f, "ASTC_3X3X3_SFLOAT_BLOCK_EXT"),
            Self::ASTC_4X3X3_UNORM_BLOCK_EXT => write!(f, "ASTC_4X3X3_UNORM_BLOCK_EXT"),
            Self::ASTC_4X3X3_SRGB_BLOCK_EXT => write!(f, "ASTC_4X3X3_SRGB_BLOCK_EXT"),
            Self::ASTC_4X3X3_SFLOAT_BLOCK_EXT => write!(f, "ASTC_4X3X3_SFLOAT_BLOCK_EXT"),
            Self::ASTC_4X4X3_UNORM_BLOCK_EXT => write!(f, "ASTC_4X4X3_UNORM_BLOCK_EXT"),
            Self::ASTC_4X4X3_SRGB_BLOCK_EXT => write!(f, "ASTC_4X4X3_SRGB_BLOCK_EXT"),
            Self::ASTC_4X4X3_SFLOAT_BLOCK_EXT => write!(f, "ASTC_4X4X3_SFLOAT_BLOCK_EXT"),
            Self::ASTC_4X4X4_UNORM_BLOCK_EXT => write!(f, "ASTC_4X4X4_UNORM_BLOCK_EXT"),
            Self::ASTC_4X4X4_SRGB_BLOCK_EXT => write!(f, "ASTC_4X4X4_SRGB_BLOCK_EXT"),
            Self::ASTC_4X4X4_SFLOAT_BLOCK_EXT => write!(f, "ASTC_4X4X4_SFLOAT_BLOCK_EXT"),
            Self::ASTC_5X4X4_UNORM_BLOCK_EXT => write!(f, "ASTC_5X4X4_UNORM_BLOCK_EXT"),
            Self::ASTC_5X4X4_SRGB_BLOCK_EXT => write!(f, "ASTC_5X4X4_SRGB_BLOCK_EXT"),
            Self::ASTC_5X4X4_SFLOAT_BLOCK_EXT => write!(f, "ASTC_5X4X4_SFLOAT_BLOCK_EXT"),
            Self::ASTC_5X5X4_UNORM_BLOCK_EXT => write!(f, "ASTC_5X5X4_UNORM_BLOCK_EXT"),
            Self::ASTC_5X5X4_SRGB_BLOCK_EXT => write!(f, "ASTC_5X5X4_SRGB_BLOCK_EXT"),
            Self::ASTC_5X5X4_SFLOAT_BLOCK_EXT => write!(f, "ASTC_5X5X4_SFLOAT_BLOCK_EXT"),
            Self::ASTC_5X5X5_UNORM_BLOCK_EXT => write!(f, "ASTC_5X5X5_UNORM_BLOCK_EXT"),
            Self::ASTC_5X5X5_SRGB_BLOCK_EXT => write!(f, "ASTC_5X5X5_SRGB_BLOCK_EXT"),
            Self::ASTC_5X5X5_SFLOAT_BLOCK_EXT => write!(f, "ASTC_5X5X5_SFLOAT_BLOCK_EXT"),
            Self::ASTC_6X5X5_UNORM_BLOCK_EXT => write!(f, "ASTC_6X5X5_UNORM_BLOCK_EXT"),
            Self::ASTC_6X5X5_SRGB_BLOCK_EXT => write!(f, "ASTC_6X5X5_SRGB_BLOCK_EXT"),
            Self::ASTC_6X5X5_SFLOAT_BLOCK_EXT => write!(f, "ASTC_6X5X5_SFLOAT_BLOCK_EXT"),
            Self::ASTC_6X6X5_UNORM_BLOCK_EXT => write!(f, "ASTC_6X6X5_UNORM_BLOCK_EXT"),
            Self::ASTC_6X6X5_SRGB_BLOCK_EXT => write!(f, "ASTC_6X6X5_SRGB_BLOCK_EXT"),
            Self::ASTC_6X6X5_SFLOAT_BLOCK_EXT => write!(f, "ASTC_6X6X5_SFLOAT_BLOCK_EXT"),
            Self::ASTC_6X6X6_UNORM_BLOCK_EXT => write!(f, "ASTC_6X6X6_UNORM_BLOCK_EXT"),
            Self::ASTC_6X6X6_SRGB_BLOCK_EXT => write!(f, "ASTC_6X6X6_SRGB_BLOCK_EXT"),
            Self::ASTC_6X6X6_SFLOAT_BLOCK_EXT => write!(f, "ASTC_6X6X6_SFLOAT_BLOCK_EXT"),
            Self::R8_BOOL_ARM => write!(f, "R8_BOOL_ARM"),
            Self::R16_SFLOAT_FPENCODING_BFLOAT16_ARM => {
                write!(f, "R16_SFLOAT_FPENCODING_BFLOAT16_ARM")
            }
            Self::R8_SFLOAT_FPENCODING_FLOAT8E4M3_ARM => {
                write!(f, "R8_SFLOAT_FPENCODING_FLOAT8E4M3_ARM")
            }
            Self::R8_SFLOAT_FPENCODING_FLOAT8E5M2_ARM => {
                write!(f, "R8_SFLOAT_FPENCODING_FLOAT8E5M2_ARM")
            }
            Self::R16G16_SFIXED5_NV => write!(f, "R16G16_SFIXED5_NV"),
            Self::R10X6_UINT_PACK16_ARM => write!(f, "R10X6_UINT_PACK16_ARM"),
            Self::R10X6G10X6_UINT_2PACK16_ARM => write!(f, "R10X6G10X6_UINT_2PACK16_ARM"),
            Self::R10X6G10X6B10X6A10X6_UINT_4PACK16_ARM => {
                write!(f, "R10X6G10X6B10X6A10X6_UINT_4PACK16_ARM")
            }
            Self::R12X4_UINT_PACK16_ARM => write!(f, "R12X4_UINT_PACK16_ARM"),
            Self::R12X4G12X4_UINT_2PACK16_ARM => write!(f, "R12X4G12X4_UINT_2PACK16_ARM"),
            Self::R12X4G12X4B12X4A12X4_UINT_4PACK16_ARM => {
                write!(f, "R12X4G12X4B12X4A12X4_UINT_4PACK16_ARM")
            }
            Self::R14X2_UINT_PACK16_ARM => write!(f, "R14X2_UINT_PACK16_ARM"),
            Self::R14X2G14X2_UINT_2PACK16_ARM => write!(f, "R14X2G14X2_UINT_2PACK16_ARM"),
            Self::R14X2G14X2B14X2A14X2_UINT_4PACK16_ARM => {
                write!(f, "R14X2G14X2B14X2A14X2_UINT_4PACK16_ARM")
            }
            Self::R14X2_UNORM_PACK16_ARM => write!(f, "R14X2_UNORM_PACK16_ARM"),
            Self::R14X2G14X2_UNORM_2PACK16_ARM => write!(f, "R14X2G14X2_UNORM_2PACK16_ARM"),
            Self::R14X2G14X2B14X2A14X2_UNORM_4PACK16_ARM => {
                write!(f, "R14X2G14X2B14X2A14X2_UNORM_4PACK16_ARM")
            }
            Self::G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16_ARM => {
                write!(f, "G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16_ARM")
            }
            Self::G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16_ARM => {
                write!(f, "G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16_ARM")
            }
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFrontFace.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FrontFace(i32);
impl FrontFace {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl FrontFace {
    pub const COUNTER_CLOCKWISE: Self = Self(0);
    pub const CLOCKWISE: Self = Self(1);
}
impl fmt::Display for FrontFace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::COUNTER_CLOCKWISE => write!(f, "COUNTER_CLOCKWISE"),
            Self::CLOCKWISE => write!(f, "CLOCKWISE"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageLayout.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ImageLayout(i32);
impl ImageLayout {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ImageLayout {
    pub const UNDEFINED: Self = Self(0);
    pub const GENERAL: Self = Self(1);
    pub const COLOR_ATTACHMENT_OPTIMAL: Self = Self(2);
    pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(3);
    pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = Self(4);
    pub const SHADER_READ_ONLY_OPTIMAL: Self = Self(5);
    pub const TRANSFER_SRC_OPTIMAL: Self = Self(6);
    pub const TRANSFER_DST_OPTIMAL: Self = Self(7);
    pub const PREINITIALIZED: Self = Self(8);
    pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(1000117000);
    pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: Self = Self(1000117001);
    pub const DEPTH_ATTACHMENT_OPTIMAL: Self = Self(1000241000);
    pub const DEPTH_READ_ONLY_OPTIMAL: Self = Self(1000241001);
    pub const STENCIL_ATTACHMENT_OPTIMAL: Self = Self(1000241002);
    pub const STENCIL_READ_ONLY_OPTIMAL: Self = Self(1000241003);
    pub const READ_ONLY_OPTIMAL: Self = Self(1000314000);
    pub const ATTACHMENT_OPTIMAL: Self = Self(1000314001);
    pub const RENDERING_LOCAL_READ: Self = Self(1000232000);
    pub const PRESENT_SRC_KHR: Self = Self(1000001002);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(1000024000);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(1000024001);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(1000024002);
    pub const SHARED_PRESENT_KHR: Self = Self(1000111000);
    pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR: Self =
        Self::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL;
    pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR: Self =
        Self::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL;
    pub const SHADING_RATE_OPTIMAL_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR;
    pub const FRAGMENT_DENSITY_MAP_OPTIMAL_EXT: Self = Self(1000218000);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000164003);
    pub const RENDERING_LOCAL_READ_KHR: Self = Self::RENDERING_LOCAL_READ;
    pub const DEPTH_ATTACHMENT_OPTIMAL_KHR: Self = Self::DEPTH_ATTACHMENT_OPTIMAL;
    pub const DEPTH_READ_ONLY_OPTIMAL_KHR: Self = Self::DEPTH_READ_ONLY_OPTIMAL;
    pub const STENCIL_ATTACHMENT_OPTIMAL_KHR: Self = Self::STENCIL_ATTACHMENT_OPTIMAL;
    pub const STENCIL_READ_ONLY_OPTIMAL_KHR: Self = Self::STENCIL_READ_ONLY_OPTIMAL;
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(1000299000);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1000299001);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1000299002);
    pub const READ_ONLY_OPTIMAL_KHR: Self = Self::READ_ONLY_OPTIMAL;
    pub const ATTACHMENT_OPTIMAL_KHR: Self = Self::ATTACHMENT_OPTIMAL;
    pub const ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT: Self = Self(1000339000);
    pub const TENSOR_ALIASING_ARM: Self = Self(1000460000);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_KHR: Self = Self(1000553000);
    pub const ZERO_INITIALIZED_EXT: Self = Self(1000620000);
}
impl fmt::Display for ImageLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UNDEFINED => write!(f, "UNDEFINED"),
            Self::GENERAL => write!(f, "GENERAL"),
            Self::COLOR_ATTACHMENT_OPTIMAL => write!(f, "COLOR_ATTACHMENT_OPTIMAL"),
            Self::DEPTH_STENCIL_ATTACHMENT_OPTIMAL => write!(f, "DEPTH_STENCIL_ATTACHMENT_OPTIMAL"),
            Self::DEPTH_STENCIL_READ_ONLY_OPTIMAL => write!(f, "DEPTH_STENCIL_READ_ONLY_OPTIMAL"),
            Self::SHADER_READ_ONLY_OPTIMAL => write!(f, "SHADER_READ_ONLY_OPTIMAL"),
            Self::TRANSFER_SRC_OPTIMAL => write!(f, "TRANSFER_SRC_OPTIMAL"),
            Self::TRANSFER_DST_OPTIMAL => write!(f, "TRANSFER_DST_OPTIMAL"),
            Self::PREINITIALIZED => write!(f, "PREINITIALIZED"),
            Self::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL => {
                write!(f, "DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL")
            }
            Self::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL => {
                write!(f, "DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL")
            }
            Self::DEPTH_ATTACHMENT_OPTIMAL => write!(f, "DEPTH_ATTACHMENT_OPTIMAL"),
            Self::DEPTH_READ_ONLY_OPTIMAL => write!(f, "DEPTH_READ_ONLY_OPTIMAL"),
            Self::STENCIL_ATTACHMENT_OPTIMAL => write!(f, "STENCIL_ATTACHMENT_OPTIMAL"),
            Self::STENCIL_READ_ONLY_OPTIMAL => write!(f, "STENCIL_READ_ONLY_OPTIMAL"),
            Self::READ_ONLY_OPTIMAL => write!(f, "READ_ONLY_OPTIMAL"),
            Self::ATTACHMENT_OPTIMAL => write!(f, "ATTACHMENT_OPTIMAL"),
            Self::RENDERING_LOCAL_READ => write!(f, "RENDERING_LOCAL_READ"),
            Self::PRESENT_SRC_KHR => write!(f, "PRESENT_SRC_KHR"),
            Self::VIDEO_DECODE_DST_KHR => write!(f, "VIDEO_DECODE_DST_KHR"),
            Self::VIDEO_DECODE_SRC_KHR => write!(f, "VIDEO_DECODE_SRC_KHR"),
            Self::VIDEO_DECODE_DPB_KHR => write!(f, "VIDEO_DECODE_DPB_KHR"),
            Self::SHARED_PRESENT_KHR => write!(f, "SHARED_PRESENT_KHR"),
            Self::FRAGMENT_DENSITY_MAP_OPTIMAL_EXT => write!(f, "FRAGMENT_DENSITY_MAP_OPTIMAL_EXT"),
            Self::FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR => {
                write!(f, "FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR")
            }
            Self::VIDEO_ENCODE_DST_KHR => write!(f, "VIDEO_ENCODE_DST_KHR"),
            Self::VIDEO_ENCODE_SRC_KHR => write!(f, "VIDEO_ENCODE_SRC_KHR"),
            Self::VIDEO_ENCODE_DPB_KHR => write!(f, "VIDEO_ENCODE_DPB_KHR"),
            Self::ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT => {
                write!(f, "ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT")
            }
            Self::TENSOR_ALIASING_ARM => write!(f, "TENSOR_ALIASING_ARM"),
            Self::VIDEO_ENCODE_QUANTIZATION_MAP_KHR => {
                write!(f, "VIDEO_ENCODE_QUANTIZATION_MAP_KHR")
            }
            Self::ZERO_INITIALIZED_EXT => write!(f, "ZERO_INITIALIZED_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageTiling.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ImageTiling(i32);
impl ImageTiling {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ImageTiling {
    pub const OPTIMAL: Self = Self(0);
    pub const LINEAR: Self = Self(1);
    pub const DRM_FORMAT_MODIFIER_EXT: Self = Self(1000158000);
}
impl fmt::Display for ImageTiling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::OPTIMAL => write!(f, "OPTIMAL"),
            Self::LINEAR => write!(f, "LINEAR"),
            Self::DRM_FORMAT_MODIFIER_EXT => write!(f, "DRM_FORMAT_MODIFIER_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageType.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ImageType(i32);
impl ImageType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ImageType {
    pub const TYPE_1D: Self = Self(0);
    pub const TYPE_2D: Self = Self(1);
    pub const TYPE_3D: Self = Self(2);
}
impl fmt::Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TYPE_1D => write!(f, "TYPE_1D"),
            Self::TYPE_2D => write!(f, "TYPE_2D"),
            Self::TYPE_3D => write!(f, "TYPE_3D"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewType.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ImageViewType(i32);
impl ImageViewType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ImageViewType {
    pub const TYPE_1D: Self = Self(0);
    pub const TYPE_2D: Self = Self(1);
    pub const TYPE_3D: Self = Self(2);
    pub const CUBE: Self = Self(3);
    pub const TYPE_1D_ARRAY: Self = Self(4);
    pub const TYPE_2D_ARRAY: Self = Self(5);
    pub const CUBE_ARRAY: Self = Self(6);
}
impl fmt::Display for ImageViewType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TYPE_1D => write!(f, "TYPE_1D"),
            Self::TYPE_2D => write!(f, "TYPE_2D"),
            Self::TYPE_3D => write!(f, "TYPE_3D"),
            Self::CUBE => write!(f, "CUBE"),
            Self::TYPE_1D_ARRAY => write!(f, "TYPE_1D_ARRAY"),
            Self::TYPE_2D_ARRAY => write!(f, "TYPE_2D_ARRAY"),
            Self::CUBE_ARRAY => write!(f, "CUBE_ARRAY"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsTokenTypeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct IndirectCommandsTokenTypeEXT(i32);
impl IndirectCommandsTokenTypeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl IndirectCommandsTokenTypeEXT {
    pub const EXECUTION_SET_EXT: Self = Self(0);
    pub const PUSH_CONSTANT_EXT: Self = Self(1);
    pub const SEQUENCE_INDEX_EXT: Self = Self(2);
    pub const INDEX_BUFFER_EXT: Self = Self(3);
    pub const VERTEX_BUFFER_EXT: Self = Self(4);
    pub const DRAW_INDEXED_EXT: Self = Self(5);
    pub const DRAW_EXT: Self = Self(6);
    pub const DRAW_INDEXED_COUNT_EXT: Self = Self(7);
    pub const DRAW_COUNT_EXT: Self = Self(8);
    pub const DISPATCH_EXT: Self = Self(9);
    pub const PUSH_DATA_EXT: Self = Self(1000135000);
    pub const PUSH_DATA_SEQUENCE_INDEX_EXT: Self = Self(1000135001);
    pub const DRAW_MESH_TASKS_NV_EXT: Self = Self(1000202002);
    pub const DRAW_MESH_TASKS_COUNT_NV_EXT: Self = Self(1000202003);
    pub const DRAW_MESH_TASKS_EXT: Self = Self(1000328000);
    pub const DRAW_MESH_TASKS_COUNT_EXT: Self = Self(1000328001);
    pub const TRACE_RAYS2_EXT: Self = Self(1000386004);
}
impl fmt::Display for IndirectCommandsTokenTypeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::EXECUTION_SET_EXT => write!(f, "EXECUTION_SET_EXT"),
            Self::PUSH_CONSTANT_EXT => write!(f, "PUSH_CONSTANT_EXT"),
            Self::SEQUENCE_INDEX_EXT => write!(f, "SEQUENCE_INDEX_EXT"),
            Self::INDEX_BUFFER_EXT => write!(f, "INDEX_BUFFER_EXT"),
            Self::VERTEX_BUFFER_EXT => write!(f, "VERTEX_BUFFER_EXT"),
            Self::DRAW_INDEXED_EXT => write!(f, "DRAW_INDEXED_EXT"),
            Self::DRAW_EXT => write!(f, "DRAW_EXT"),
            Self::DRAW_INDEXED_COUNT_EXT => write!(f, "DRAW_INDEXED_COUNT_EXT"),
            Self::DRAW_COUNT_EXT => write!(f, "DRAW_COUNT_EXT"),
            Self::DISPATCH_EXT => write!(f, "DISPATCH_EXT"),
            Self::PUSH_DATA_EXT => write!(f, "PUSH_DATA_EXT"),
            Self::PUSH_DATA_SEQUENCE_INDEX_EXT => write!(f, "PUSH_DATA_SEQUENCE_INDEX_EXT"),
            Self::DRAW_MESH_TASKS_NV_EXT => write!(f, "DRAW_MESH_TASKS_NV_EXT"),
            Self::DRAW_MESH_TASKS_COUNT_NV_EXT => write!(f, "DRAW_MESH_TASKS_COUNT_NV_EXT"),
            Self::DRAW_MESH_TASKS_EXT => write!(f, "DRAW_MESH_TASKS_EXT"),
            Self::DRAW_MESH_TASKS_COUNT_EXT => write!(f, "DRAW_MESH_TASKS_COUNT_EXT"),
            Self::TRACE_RAYS2_EXT => write!(f, "TRACE_RAYS2_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSharingMode.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SharingMode(i32);
impl SharingMode {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl SharingMode {
    pub const EXCLUSIVE: Self = Self(0);
    pub const CONCURRENT: Self = Self(1);
}
impl fmt::Display for SharingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::EXCLUSIVE => write!(f, "EXCLUSIVE"),
            Self::CONCURRENT => write!(f, "CONCURRENT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndexType.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct IndexType(i32);
impl IndexType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl IndexType {
    pub const UINT16: Self = Self(0);
    pub const UINT32: Self = Self(1);
    pub const UINT8: Self = Self(1000265000);
    pub const NONE_KHR: Self = Self(1000165000);
    pub const NONE_NV: Self = Self::NONE_KHR;
    pub const UINT8_EXT: Self = Self::UINT8;
    pub const UINT8_KHR: Self = Self::UINT8;
}
impl fmt::Display for IndexType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UINT16 => write!(f, "UINT16"),
            Self::UINT32 => write!(f, "UINT32"),
            Self::UINT8 => write!(f, "UINT8"),
            Self::NONE_KHR => write!(f, "NONE_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkLogicOp.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LogicOp(i32);
impl LogicOp {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl LogicOp {
    pub const CLEAR: Self = Self(0);
    pub const AND: Self = Self(1);
    pub const AND_REVERSE: Self = Self(2);
    pub const COPY: Self = Self(3);
    pub const AND_INVERTED: Self = Self(4);
    pub const NO_OP: Self = Self(5);
    pub const XOR: Self = Self(6);
    pub const OR: Self = Self(7);
    pub const NOR: Self = Self(8);
    pub const EQUIVALENT: Self = Self(9);
    pub const INVERT: Self = Self(10);
    pub const OR_REVERSE: Self = Self(11);
    pub const COPY_INVERTED: Self = Self(12);
    pub const OR_INVERTED: Self = Self(13);
    pub const NAND: Self = Self(14);
    pub const SET: Self = Self(15);
}
impl fmt::Display for LogicOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::CLEAR => write!(f, "CLEAR"),
            Self::AND => write!(f, "AND"),
            Self::AND_REVERSE => write!(f, "AND_REVERSE"),
            Self::COPY => write!(f, "COPY"),
            Self::AND_INVERTED => write!(f, "AND_INVERTED"),
            Self::NO_OP => write!(f, "NO_OP"),
            Self::XOR => write!(f, "XOR"),
            Self::OR => write!(f, "OR"),
            Self::NOR => write!(f, "NOR"),
            Self::EQUIVALENT => write!(f, "EQUIVALENT"),
            Self::INVERT => write!(f, "INVERT"),
            Self::OR_REVERSE => write!(f, "OR_REVERSE"),
            Self::COPY_INVERTED => write!(f, "COPY_INVERTED"),
            Self::OR_INVERTED => write!(f, "OR_INVERTED"),
            Self::NAND => write!(f, "NAND"),
            Self::SET => write!(f, "SET"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceType.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PhysicalDeviceType(i32);
impl PhysicalDeviceType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PhysicalDeviceType {
    pub const OTHER: Self = Self(0);
    pub const INTEGRATED_GPU: Self = Self(1);
    pub const DISCRETE_GPU: Self = Self(2);
    pub const VIRTUAL_GPU: Self = Self(3);
    pub const CPU: Self = Self(4);
}
impl fmt::Display for PhysicalDeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::OTHER => write!(f, "OTHER"),
            Self::INTEGRATED_GPU => write!(f, "INTEGRATED_GPU"),
            Self::DISCRETE_GPU => write!(f, "DISCRETE_GPU"),
            Self::VIRTUAL_GPU => write!(f, "VIRTUAL_GPU"),
            Self::CPU => write!(f, "CPU"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBindPoint.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PipelineBindPoint(i32);
impl PipelineBindPoint {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PipelineBindPoint {
    pub const GRAPHICS: Self = Self(0);
    pub const COMPUTE: Self = Self(1);
    pub const EXECUTION_GRAPH_AMDX: Self = Self(1000134000);
    pub const RAY_TRACING_KHR: Self = Self(1000165000);
    pub const RAY_TRACING_NV: Self = Self::RAY_TRACING_KHR;
    pub const SUBPASS_SHADING_HUAWEI: Self = Self(1000369003);
    pub const DATA_GRAPH_ARM: Self = Self(1000507000);
}
impl fmt::Display for PipelineBindPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::GRAPHICS => write!(f, "GRAPHICS"),
            Self::COMPUTE => write!(f, "COMPUTE"),
            Self::EXECUTION_GRAPH_AMDX => write!(f, "EXECUTION_GRAPH_AMDX"),
            Self::RAY_TRACING_KHR => write!(f, "RAY_TRACING_KHR"),
            Self::SUBPASS_SHADING_HUAWEI => write!(f, "SUBPASS_SHADING_HUAWEI"),
            Self::DATA_GRAPH_ARM => write!(f, "DATA_GRAPH_ARM"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPrimitiveTopology.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PrimitiveTopology(i32);
impl PrimitiveTopology {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PrimitiveTopology {
    pub const POINT_LIST: Self = Self(0);
    pub const LINE_LIST: Self = Self(1);
    pub const LINE_STRIP: Self = Self(2);
    pub const TRIANGLE_LIST: Self = Self(3);
    pub const TRIANGLE_STRIP: Self = Self(4);
    pub const TRIANGLE_FAN: Self = Self(5);
    pub const LINE_LIST_WITH_ADJACENCY: Self = Self(6);
    pub const LINE_STRIP_WITH_ADJACENCY: Self = Self(7);
    pub const TRIANGLE_LIST_WITH_ADJACENCY: Self = Self(8);
    pub const TRIANGLE_STRIP_WITH_ADJACENCY: Self = Self(9);
    pub const PATCH_LIST: Self = Self(10);
}
impl fmt::Display for PrimitiveTopology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::POINT_LIST => write!(f, "POINT_LIST"),
            Self::LINE_LIST => write!(f, "LINE_LIST"),
            Self::LINE_STRIP => write!(f, "LINE_STRIP"),
            Self::TRIANGLE_LIST => write!(f, "TRIANGLE_LIST"),
            Self::TRIANGLE_STRIP => write!(f, "TRIANGLE_STRIP"),
            Self::TRIANGLE_FAN => write!(f, "TRIANGLE_FAN"),
            Self::LINE_LIST_WITH_ADJACENCY => write!(f, "LINE_LIST_WITH_ADJACENCY"),
            Self::LINE_STRIP_WITH_ADJACENCY => write!(f, "LINE_STRIP_WITH_ADJACENCY"),
            Self::TRIANGLE_LIST_WITH_ADJACENCY => write!(f, "TRIANGLE_LIST_WITH_ADJACENCY"),
            Self::TRIANGLE_STRIP_WITH_ADJACENCY => write!(f, "TRIANGLE_STRIP_WITH_ADJACENCY"),
            Self::PATCH_LIST => write!(f, "PATCH_LIST"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryType.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct QueryType(i32);
impl QueryType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl QueryType {
    pub const OCCLUSION: Self = Self(0);
    pub const PIPELINE_STATISTICS: Self = Self(1);
    pub const TIMESTAMP: Self = Self(2);
    pub const RESULT_STATUS_ONLY_KHR: Self = Self(1000023000);
    pub const TRANSFORM_FEEDBACK_STREAM_EXT: Self = Self(1000028004);
    pub const PERFORMANCE_QUERY_KHR: Self = Self(1000116000);
    pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR: Self = Self(1000150001);
    pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV: Self = Self(1000165000);
    pub const PERFORMANCE_QUERY_INTEL: Self = Self(1000210000);
    pub const VIDEO_ENCODE_FEEDBACK_KHR: Self = Self(1000299000);
    pub const MESH_PRIMITIVES_GENERATED_EXT: Self = Self(1000328000);
    pub const PRIMITIVES_GENERATED_EXT: Self = Self(1000382000);
    pub const ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR: Self =
        Self(1000386000);
    pub const ACCELERATION_STRUCTURE_SIZE_KHR: Self = Self(1000386001);
    pub const MICROMAP_SERIALIZATION_SIZE_EXT: Self = Self(1000396000);
    pub const MICROMAP_COMPACTED_SIZE_EXT: Self = Self(1000396001);
}
impl fmt::Display for QueryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::OCCLUSION => write!(f, "OCCLUSION"),
            Self::PIPELINE_STATISTICS => write!(f, "PIPELINE_STATISTICS"),
            Self::TIMESTAMP => write!(f, "TIMESTAMP"),
            Self::RESULT_STATUS_ONLY_KHR => write!(f, "RESULT_STATUS_ONLY_KHR"),
            Self::TRANSFORM_FEEDBACK_STREAM_EXT => write!(f, "TRANSFORM_FEEDBACK_STREAM_EXT"),
            Self::PERFORMANCE_QUERY_KHR => write!(f, "PERFORMANCE_QUERY_KHR"),
            Self::ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR => {
                write!(f, "ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR")
            }
            Self::ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR => {
                write!(f, "ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR")
            }
            Self::ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV => {
                write!(f, "ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV")
            }
            Self::PERFORMANCE_QUERY_INTEL => write!(f, "PERFORMANCE_QUERY_INTEL"),
            Self::VIDEO_ENCODE_FEEDBACK_KHR => write!(f, "VIDEO_ENCODE_FEEDBACK_KHR"),
            Self::MESH_PRIMITIVES_GENERATED_EXT => write!(f, "MESH_PRIMITIVES_GENERATED_EXT"),
            Self::PRIMITIVES_GENERATED_EXT => write!(f, "PRIMITIVES_GENERATED_EXT"),
            Self::ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR => write!(
                f,
                "ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR"
            ),
            Self::ACCELERATION_STRUCTURE_SIZE_KHR => write!(f, "ACCELERATION_STRUCTURE_SIZE_KHR"),
            Self::MICROMAP_SERIALIZATION_SIZE_EXT => write!(f, "MICROMAP_SERIALIZATION_SIZE_EXT"),
            Self::MICROMAP_COMPACTED_SIZE_EXT => write!(f, "MICROMAP_COMPACTED_SIZE_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassContents.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SubpassContents(i32);
impl SubpassContents {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl SubpassContents {
    pub const INLINE: Self = Self(0);
    pub const SECONDARY_COMMAND_BUFFERS: Self = Self(1);
    pub const INLINE_AND_SECONDARY_COMMAND_BUFFERS_EXT: Self =
        Self::INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR;
    pub const INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR: Self = Self(1000451000);
}
impl fmt::Display for SubpassContents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::INLINE => write!(f, "INLINE"),
            Self::SECONDARY_COMMAND_BUFFERS => write!(f, "SECONDARY_COMMAND_BUFFERS"),
            Self::INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR => {
                write!(f, "INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR")
            }
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkResult.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Result(i32);
impl Result {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl Result {
    pub const SUCCESS: Self = Self(0);
    pub const NOT_READY: Self = Self(1);
    pub const TIMEOUT: Self = Self(2);
    pub const EVENT_SET: Self = Self(3);
    pub const EVENT_RESET: Self = Self(4);
    pub const INCOMPLETE: Self = Self(5);
    pub const ERROR_OUT_OF_HOST_MEMORY: Self = Self(-1);
    pub const ERROR_OUT_OF_DEVICE_MEMORY: Self = Self(-2);
    pub const ERROR_INITIALIZATION_FAILED: Self = Self(-3);
    pub const ERROR_DEVICE_LOST: Self = Self(-4);
    pub const ERROR_MEMORY_MAP_FAILED: Self = Self(-5);
    pub const ERROR_LAYER_NOT_PRESENT: Self = Self(-6);
    pub const ERROR_EXTENSION_NOT_PRESENT: Self = Self(-7);
    pub const ERROR_FEATURE_NOT_PRESENT: Self = Self(-8);
    pub const ERROR_INCOMPATIBLE_DRIVER: Self = Self(-9);
    pub const ERROR_TOO_MANY_OBJECTS: Self = Self(-10);
    pub const ERROR_FORMAT_NOT_SUPPORTED: Self = Self(-11);
    pub const ERROR_FRAGMENTED_POOL: Self = Self(-12);
    pub const ERROR_UNKNOWN: Self = Self(-13);
    pub const ERROR_VALIDATION_FAILED: Self = Self(-1000011001);
    pub const ERROR_OUT_OF_POOL_MEMORY: Self = Self(-1000069000);
    pub const ERROR_INVALID_EXTERNAL_HANDLE: Self = Self(-1000072003);
    pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: Self = Self(-1000257000);
    pub const ERROR_FRAGMENTATION: Self = Self(-1000161000);
    pub const PIPELINE_COMPILE_REQUIRED: Self = Self(1000297000);
    pub const ERROR_NOT_PERMITTED: Self = Self(-1000174001);
    pub const ERROR_SURFACE_LOST_KHR: Self = Self(-1000000000);
    pub const ERROR_NATIVE_WINDOW_IN_USE_KHR: Self = Self(-1000000001);
    pub const SUBOPTIMAL_KHR: Self = Self(1000001003);
    pub const ERROR_OUT_OF_DATE_KHR: Self = Self(-1000001004);
    pub const ERROR_INCOMPATIBLE_DISPLAY_KHR: Self = Self(-1000003001);
    pub const ERROR_VALIDATION_FAILED_EXT: Self = Self::ERROR_VALIDATION_FAILED;
    pub const ERROR_INVALID_SHADER_NV: Self = Self(-1000012000);
    pub const ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR: Self = Self(-1000023000);
    pub const ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR: Self = Self(-1000023001);
    pub const ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR: Self = Self(-1000023002);
    pub const ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR: Self = Self(-1000023003);
    pub const ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR: Self = Self(-1000023004);
    pub const ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR: Self = Self(-1000023005);
    pub const ERROR_OUT_OF_POOL_MEMORY_KHR: Self = Self::ERROR_OUT_OF_POOL_MEMORY;
    pub const ERROR_INVALID_EXTERNAL_HANDLE_KHR: Self = Self::ERROR_INVALID_EXTERNAL_HANDLE;
    pub const ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: Self = Self(-1000158000);
    pub const ERROR_FRAGMENTATION_EXT: Self = Self::ERROR_FRAGMENTATION;
    pub const ERROR_NOT_PERMITTED_EXT: Self = Self::ERROR_NOT_PERMITTED;
    pub const ERROR_NOT_PERMITTED_KHR: Self = Self::ERROR_NOT_PERMITTED;
    pub const ERROR_PRESENT_TIMING_QUEUE_FULL_EXT: Self = Self(-1000208000);
    pub const ERROR_INVALID_DEVICE_ADDRESS_EXT: Self = Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
    pub const ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: Self = Self(-1000255000);
    pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: Self =
        Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
    pub const THREAD_IDLE_KHR: Self = Self(1000268000);
    pub const THREAD_DONE_KHR: Self = Self(1000268001);
    pub const OPERATION_DEFERRED_KHR: Self = Self(1000268002);
    pub const OPERATION_NOT_DEFERRED_KHR: Self = Self(1000268003);
    pub const PIPELINE_COMPILE_REQUIRED_EXT: Self = Self::PIPELINE_COMPILE_REQUIRED;
    pub const ERROR_PIPELINE_COMPILE_REQUIRED_EXT: Self = Self::PIPELINE_COMPILE_REQUIRED;
    pub const ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR: Self = Self(-1000299000);
    pub const ERROR_COMPRESSION_EXHAUSTED_EXT: Self = Self(-1000338000);
    pub const INCOMPATIBLE_SHADER_BINARY_EXT: Self = Self(1000482000);
    #[deprecated = "aliased"]
    pub const ERROR_INCOMPATIBLE_SHADER_BINARY_EXT: Self = Self::INCOMPATIBLE_SHADER_BINARY_EXT;
    pub const PIPELINE_BINARY_MISSING_KHR: Self = Self(1000483000);
    pub const ERROR_NOT_ENOUGH_SPACE_KHR: Self = Self(-1000483000);
}
impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::SUCCESS => write!(f, "SUCCESS"),
            Self::NOT_READY => write!(f, "NOT_READY"),
            Self::TIMEOUT => write!(f, "TIMEOUT"),
            Self::EVENT_SET => write!(f, "EVENT_SET"),
            Self::EVENT_RESET => write!(f, "EVENT_RESET"),
            Self::INCOMPLETE => write!(f, "INCOMPLETE"),
            Self::ERROR_OUT_OF_HOST_MEMORY => write!(f, "ERROR_OUT_OF_HOST_MEMORY"),
            Self::ERROR_OUT_OF_DEVICE_MEMORY => write!(f, "ERROR_OUT_OF_DEVICE_MEMORY"),
            Self::ERROR_INITIALIZATION_FAILED => write!(f, "ERROR_INITIALIZATION_FAILED"),
            Self::ERROR_DEVICE_LOST => write!(f, "ERROR_DEVICE_LOST"),
            Self::ERROR_MEMORY_MAP_FAILED => write!(f, "ERROR_MEMORY_MAP_FAILED"),
            Self::ERROR_LAYER_NOT_PRESENT => write!(f, "ERROR_LAYER_NOT_PRESENT"),
            Self::ERROR_EXTENSION_NOT_PRESENT => write!(f, "ERROR_EXTENSION_NOT_PRESENT"),
            Self::ERROR_FEATURE_NOT_PRESENT => write!(f, "ERROR_FEATURE_NOT_PRESENT"),
            Self::ERROR_INCOMPATIBLE_DRIVER => write!(f, "ERROR_INCOMPATIBLE_DRIVER"),
            Self::ERROR_TOO_MANY_OBJECTS => write!(f, "ERROR_TOO_MANY_OBJECTS"),
            Self::ERROR_FORMAT_NOT_SUPPORTED => write!(f, "ERROR_FORMAT_NOT_SUPPORTED"),
            Self::ERROR_FRAGMENTED_POOL => write!(f, "ERROR_FRAGMENTED_POOL"),
            Self::ERROR_UNKNOWN => write!(f, "ERROR_UNKNOWN"),
            Self::ERROR_VALIDATION_FAILED => write!(f, "ERROR_VALIDATION_FAILED"),
            Self::ERROR_OUT_OF_POOL_MEMORY => write!(f, "ERROR_OUT_OF_POOL_MEMORY"),
            Self::ERROR_INVALID_EXTERNAL_HANDLE => write!(f, "ERROR_INVALID_EXTERNAL_HANDLE"),
            Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS => {
                write!(f, "ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS")
            }
            Self::ERROR_FRAGMENTATION => write!(f, "ERROR_FRAGMENTATION"),
            Self::PIPELINE_COMPILE_REQUIRED => write!(f, "PIPELINE_COMPILE_REQUIRED"),
            Self::ERROR_NOT_PERMITTED => write!(f, "ERROR_NOT_PERMITTED"),
            Self::ERROR_SURFACE_LOST_KHR => write!(f, "ERROR_SURFACE_LOST_KHR"),
            Self::ERROR_NATIVE_WINDOW_IN_USE_KHR => write!(f, "ERROR_NATIVE_WINDOW_IN_USE_KHR"),
            Self::SUBOPTIMAL_KHR => write!(f, "SUBOPTIMAL_KHR"),
            Self::ERROR_OUT_OF_DATE_KHR => write!(f, "ERROR_OUT_OF_DATE_KHR"),
            Self::ERROR_INCOMPATIBLE_DISPLAY_KHR => write!(f, "ERROR_INCOMPATIBLE_DISPLAY_KHR"),
            Self::ERROR_INVALID_SHADER_NV => write!(f, "ERROR_INVALID_SHADER_NV"),
            Self::ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR => {
                write!(f, "ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR")
            }
            Self::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR => {
                write!(f, "ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR")
            }
            Self::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR => {
                write!(f, "ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR")
            }
            Self::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR => {
                write!(f, "ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR")
            }
            Self::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR => {
                write!(f, "ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR")
            }
            Self::ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR => {
                write!(f, "ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR")
            }
            Self::ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT => {
                write!(f, "ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT")
            }
            Self::ERROR_PRESENT_TIMING_QUEUE_FULL_EXT => {
                write!(f, "ERROR_PRESENT_TIMING_QUEUE_FULL_EXT")
            }
            Self::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => {
                write!(f, "ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT")
            }
            Self::THREAD_IDLE_KHR => write!(f, "THREAD_IDLE_KHR"),
            Self::THREAD_DONE_KHR => write!(f, "THREAD_DONE_KHR"),
            Self::OPERATION_DEFERRED_KHR => write!(f, "OPERATION_DEFERRED_KHR"),
            Self::OPERATION_NOT_DEFERRED_KHR => write!(f, "OPERATION_NOT_DEFERRED_KHR"),
            Self::ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR => {
                write!(f, "ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR")
            }
            Self::ERROR_COMPRESSION_EXHAUSTED_EXT => write!(f, "ERROR_COMPRESSION_EXHAUSTED_EXT"),
            Self::INCOMPATIBLE_SHADER_BINARY_EXT => write!(f, "INCOMPATIBLE_SHADER_BINARY_EXT"),
            Self::PIPELINE_BINARY_MISSING_KHR => write!(f, "PIPELINE_BINARY_MISSING_KHR"),
            Self::ERROR_NOT_ENOUGH_SPACE_KHR => write!(f, "ERROR_NOT_ENOUGH_SPACE_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilOp.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StencilOp(i32);
impl StencilOp {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl StencilOp {
    pub const KEEP: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const REPLACE: Self = Self(2);
    pub const INCREMENT_AND_CLAMP: Self = Self(3);
    pub const DECREMENT_AND_CLAMP: Self = Self(4);
    pub const INVERT: Self = Self(5);
    pub const INCREMENT_AND_WRAP: Self = Self(6);
    pub const DECREMENT_AND_WRAP: Self = Self(7);
}
impl fmt::Display for StencilOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::KEEP => write!(f, "KEEP"),
            Self::ZERO => write!(f, "ZERO"),
            Self::REPLACE => write!(f, "REPLACE"),
            Self::INCREMENT_AND_CLAMP => write!(f, "INCREMENT_AND_CLAMP"),
            Self::DECREMENT_AND_CLAMP => write!(f, "DECREMENT_AND_CLAMP"),
            Self::INVERT => write!(f, "INVERT"),
            Self::INCREMENT_AND_WRAP => write!(f, "INCREMENT_AND_WRAP"),
            Self::DECREMENT_AND_WRAP => write!(f, "DECREMENT_AND_WRAP"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkStructureType.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StructureType(i32);
impl StructureType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl StructureType {
    pub const APPLICATION_INFO: Self = Self(0);
    pub const INSTANCE_CREATE_INFO: Self = Self(1);
    pub const DEVICE_QUEUE_CREATE_INFO: Self = Self(2);
    pub const DEVICE_CREATE_INFO: Self = Self(3);
    pub const SUBMIT_INFO: Self = Self(4);
    pub const MEMORY_ALLOCATE_INFO: Self = Self(5);
    pub const MAPPED_MEMORY_RANGE: Self = Self(6);
    pub const BIND_SPARSE_INFO: Self = Self(7);
    pub const FENCE_CREATE_INFO: Self = Self(8);
    pub const SEMAPHORE_CREATE_INFO: Self = Self(9);
    pub const EVENT_CREATE_INFO: Self = Self(10);
    pub const QUERY_POOL_CREATE_INFO: Self = Self(11);
    pub const BUFFER_CREATE_INFO: Self = Self(12);
    pub const BUFFER_VIEW_CREATE_INFO: Self = Self(13);
    pub const IMAGE_CREATE_INFO: Self = Self(14);
    pub const IMAGE_VIEW_CREATE_INFO: Self = Self(15);
    pub const SHADER_MODULE_CREATE_INFO: Self = Self(16);
    pub const PIPELINE_CACHE_CREATE_INFO: Self = Self(17);
    pub const PIPELINE_SHADER_STAGE_CREATE_INFO: Self = Self(18);
    pub const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: Self = Self(19);
    pub const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: Self = Self(20);
    pub const PIPELINE_TESSELLATION_STATE_CREATE_INFO: Self = Self(21);
    pub const PIPELINE_VIEWPORT_STATE_CREATE_INFO: Self = Self(22);
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_INFO: Self = Self(23);
    pub const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: Self = Self(24);
    pub const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: Self = Self(25);
    pub const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: Self = Self(26);
    pub const PIPELINE_DYNAMIC_STATE_CREATE_INFO: Self = Self(27);
    pub const GRAPHICS_PIPELINE_CREATE_INFO: Self = Self(28);
    pub const COMPUTE_PIPELINE_CREATE_INFO: Self = Self(29);
    pub const PIPELINE_LAYOUT_CREATE_INFO: Self = Self(30);
    pub const SAMPLER_CREATE_INFO: Self = Self(31);
    pub const DESCRIPTOR_SET_LAYOUT_CREATE_INFO: Self = Self(32);
    pub const DESCRIPTOR_POOL_CREATE_INFO: Self = Self(33);
    pub const DESCRIPTOR_SET_ALLOCATE_INFO: Self = Self(34);
    pub const WRITE_DESCRIPTOR_SET: Self = Self(35);
    pub const COPY_DESCRIPTOR_SET: Self = Self(36);
    pub const FRAMEBUFFER_CREATE_INFO: Self = Self(37);
    pub const RENDER_PASS_CREATE_INFO: Self = Self(38);
    pub const COMMAND_POOL_CREATE_INFO: Self = Self(39);
    pub const COMMAND_BUFFER_ALLOCATE_INFO: Self = Self(40);
    pub const COMMAND_BUFFER_INHERITANCE_INFO: Self = Self(41);
    pub const COMMAND_BUFFER_BEGIN_INFO: Self = Self(42);
    pub const RENDER_PASS_BEGIN_INFO: Self = Self(43);
    pub const BUFFER_MEMORY_BARRIER: Self = Self(44);
    pub const IMAGE_MEMORY_BARRIER: Self = Self(45);
    pub const MEMORY_BARRIER: Self = Self(46);
    pub const LOADER_INSTANCE_CREATE_INFO: Self = Self(47);
    pub const LOADER_DEVICE_CREATE_INFO: Self = Self(48);
    pub const BIND_BUFFER_MEMORY_INFO: Self = Self(1000157000);
    pub const BIND_IMAGE_MEMORY_INFO: Self = Self(1000157001);
    pub const MEMORY_DEDICATED_REQUIREMENTS: Self = Self(1000127000);
    pub const MEMORY_DEDICATED_ALLOCATE_INFO: Self = Self(1000127001);
    pub const MEMORY_ALLOCATE_FLAGS_INFO: Self = Self(1000060000);
    pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: Self = Self(1000060004);
    pub const DEVICE_GROUP_SUBMIT_INFO: Self = Self(1000060005);
    pub const DEVICE_GROUP_BIND_SPARSE_INFO: Self = Self(1000060006);
    pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: Self = Self(1000060013);
    pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: Self = Self(1000060014);
    pub const PHYSICAL_DEVICE_GROUP_PROPERTIES: Self = Self(1000070000);
    pub const DEVICE_GROUP_DEVICE_CREATE_INFO: Self = Self(1000070001);
    pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146000);
    pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146001);
    pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146002);
    pub const MEMORY_REQUIREMENTS_2: Self = Self(1000146003);
    pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: Self = Self(1000146004);
    pub const PHYSICAL_DEVICE_FEATURES_2: Self = Self(1000059000);
    pub const PHYSICAL_DEVICE_PROPERTIES_2: Self = Self(1000059001);
    pub const FORMAT_PROPERTIES_2: Self = Self(1000059002);
    pub const IMAGE_FORMAT_PROPERTIES_2: Self = Self(1000059003);
    pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: Self = Self(1000059004);
    pub const QUEUE_FAMILY_PROPERTIES_2: Self = Self(1000059005);
    pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: Self = Self(1000059006);
    pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2: Self = Self(1000059007);
    pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: Self = Self(1000059008);
    pub const IMAGE_VIEW_USAGE_CREATE_INFO: Self = Self(1000117002);
    pub const PROTECTED_SUBMIT_INFO: Self = Self(1000145000);
    pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: Self = Self(1000145001);
    pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: Self = Self(1000145002);
    pub const DEVICE_QUEUE_INFO_2: Self = Self(1000145003);
    pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: Self = Self(1000071000);
    pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES: Self = Self(1000071001);
    pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: Self = Self(1000071002);
    pub const EXTERNAL_BUFFER_PROPERTIES: Self = Self(1000071003);
    pub const PHYSICAL_DEVICE_ID_PROPERTIES: Self = Self(1000071004);
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO: Self = Self(1000072000);
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO: Self = Self(1000072001);
    pub const EXPORT_MEMORY_ALLOCATE_INFO: Self = Self(1000072002);
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: Self = Self(1000112000);
    pub const EXTERNAL_FENCE_PROPERTIES: Self = Self(1000112001);
    pub const EXPORT_FENCE_CREATE_INFO: Self = Self(1000113000);
    pub const EXPORT_SEMAPHORE_CREATE_INFO: Self = Self(1000077000);
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: Self = Self(1000076000);
    pub const EXTERNAL_SEMAPHORE_PROPERTIES: Self = Self(1000076001);
    pub const PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: Self = Self(1000094000);
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: Self = Self(1000083000);
    pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES: Self = Self(1000120000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: Self = Self(1000085000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: Self = Self(1000168000);
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT: Self = Self(1000168001);
    pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO: Self = Self(1000156000);
    pub const SAMPLER_YCBCR_CONVERSION_INFO: Self = Self(1000156001);
    pub const BIND_IMAGE_PLANE_MEMORY_INFO: Self = Self(1000156002);
    pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: Self = Self(1000156003);
    pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: Self = Self(1000156004);
    pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: Self = Self(1000156005);
    pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: Self = Self(1000060003);
    pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: Self = Self(1000117000);
    pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: Self = Self(1000117001);
    pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: Self = Self(1000117003);
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO: Self = Self(1000053000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES: Self = Self(1000053001);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: Self = Self(1000053002);
    pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES: Self = Self(1000063000);
    pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES: Self = Self(1000196000);
    pub const PHYSICAL_DEVICE_VULKAN_1_1_FEATURES: Self = Self(49);
    pub const PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES: Self = Self(50);
    pub const PHYSICAL_DEVICE_VULKAN_1_2_FEATURES: Self = Self(51);
    pub const PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES: Self = Self(52);
    pub const IMAGE_FORMAT_LIST_CREATE_INFO: Self = Self(1000147000);
    pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES: Self = Self(1000211000);
    pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES: Self = Self(1000261000);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES: Self = Self(1000207000);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES: Self = Self(1000207001);
    pub const SEMAPHORE_TYPE_CREATE_INFO: Self = Self(1000207002);
    pub const TIMELINE_SEMAPHORE_SUBMIT_INFO: Self = Self(1000207003);
    pub const SEMAPHORE_WAIT_INFO: Self = Self(1000207004);
    pub const SEMAPHORE_SIGNAL_INFO: Self = Self(1000207005);
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES: Self = Self(1000257000);
    pub const BUFFER_DEVICE_ADDRESS_INFO: Self = Self(1000244001);
    pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO: Self = Self(1000257002);
    pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO: Self = Self(1000257003);
    pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO: Self = Self(1000257004);
    pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES: Self = Self(1000177000);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES: Self = Self(1000180000);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES: Self = Self(1000082000);
    pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES: Self = Self(1000197000);
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO: Self = Self(1000161000);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES: Self = Self(1000161001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES: Self = Self(1000161002);
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO: Self = Self(1000161003);
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT: Self = Self(1000161004);
    pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES: Self = Self(1000221000);
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES: Self = Self(1000130000);
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO: Self = Self(1000130001);
    pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES: Self = Self(1000253000);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES: Self = Self(1000175000);
    pub const ATTACHMENT_DESCRIPTION_2: Self = Self(1000109000);
    pub const ATTACHMENT_REFERENCE_2: Self = Self(1000109001);
    pub const SUBPASS_DESCRIPTION_2: Self = Self(1000109002);
    pub const SUBPASS_DEPENDENCY_2: Self = Self(1000109003);
    pub const RENDER_PASS_CREATE_INFO_2: Self = Self(1000109004);
    pub const SUBPASS_BEGIN_INFO: Self = Self(1000109005);
    pub const SUBPASS_END_INFO: Self = Self(1000109006);
    pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES: Self = Self(1000199000);
    pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE: Self = Self(1000199001);
    pub const IMAGE_STENCIL_USAGE_CREATE_INFO: Self = Self(1000246000);
    pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES: Self = Self(1000108000);
    pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO: Self = Self(1000108001);
    pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO: Self = Self(1000108002);
    pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO: Self = Self(1000108003);
    pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES: Self = Self(1000241000);
    pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT: Self = Self(1000241001);
    pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT: Self = Self(1000241002);
    pub const PHYSICAL_DEVICE_VULKAN_1_3_FEATURES: Self = Self(53);
    pub const PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES: Self = Self(54);
    pub const PHYSICAL_DEVICE_TOOL_PROPERTIES: Self = Self(1000245000);
    pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES: Self = Self(1000295000);
    pub const DEVICE_PRIVATE_DATA_CREATE_INFO: Self = Self(1000295001);
    pub const PRIVATE_DATA_SLOT_CREATE_INFO: Self = Self(1000295002);
    pub const MEMORY_BARRIER_2: Self = Self(1000314000);
    pub const BUFFER_MEMORY_BARRIER_2: Self = Self(1000314001);
    pub const IMAGE_MEMORY_BARRIER_2: Self = Self(1000314002);
    pub const DEPENDENCY_INFO: Self = Self(1000314003);
    pub const SUBMIT_INFO_2: Self = Self(1000314004);
    pub const SEMAPHORE_SUBMIT_INFO: Self = Self(1000314005);
    pub const COMMAND_BUFFER_SUBMIT_INFO: Self = Self(1000314006);
    pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES: Self = Self(1000314007);
    pub const COPY_BUFFER_INFO_2: Self = Self(1000337000);
    pub const COPY_IMAGE_INFO_2: Self = Self(1000337001);
    pub const COPY_BUFFER_TO_IMAGE_INFO_2: Self = Self(1000337002);
    pub const COPY_IMAGE_TO_BUFFER_INFO_2: Self = Self(1000337003);
    pub const BUFFER_COPY_2: Self = Self(1000337006);
    pub const IMAGE_COPY_2: Self = Self(1000337007);
    pub const BUFFER_IMAGE_COPY_2: Self = Self(1000337009);
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES: Self = Self(1000066000);
    pub const FORMAT_PROPERTIES_3: Self = Self(1000360000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES: Self = Self(1000413000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES: Self = Self(1000413001);
    pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS: Self = Self(1000413002);
    pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS: Self = Self(1000413003);
    pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO: Self = Self(1000192000);
    pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES: Self = Self(1000215000);
    pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES: Self = Self(1000276000);
    pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES: Self = Self(1000297000);
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES: Self = Self(1000325000);
    pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES: Self = Self(1000335000);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES: Self = Self(1000225000);
    pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO: Self = Self(1000225001);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES: Self = Self(1000225002);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES: Self = Self(1000138000);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES: Self = Self(1000138001);
    pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK: Self = Self(1000138002);
    pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO: Self = Self(1000138003);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES: Self = Self(1000280000);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES: Self = Self(1000280001);
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES: Self = Self(1000281001);
    pub const BLIT_IMAGE_INFO_2: Self = Self(1000337004);
    pub const RESOLVE_IMAGE_INFO_2: Self = Self(1000337005);
    pub const IMAGE_BLIT_2: Self = Self(1000337008);
    pub const IMAGE_RESOLVE_2: Self = Self(1000337010);
    pub const RENDERING_INFO: Self = Self(1000044000);
    pub const RENDERING_ATTACHMENT_INFO: Self = Self(1000044001);
    pub const PIPELINE_RENDERING_CREATE_INFO: Self = Self(1000044002);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES: Self = Self(1000044003);
    pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO: Self = Self(1000044004);
    pub const PHYSICAL_DEVICE_VULKAN_1_4_FEATURES: Self = Self(55);
    pub const PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES: Self = Self(56);
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO: Self = Self(1000174000);
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES: Self = Self(1000388000);
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES: Self = Self(1000388001);
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES: Self = Self(1000265000);
    pub const MEMORY_MAP_INFO: Self = Self(1000271000);
    pub const MEMORY_UNMAP_INFO: Self = Self(1000271001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES: Self = Self(1000470000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES: Self = Self(1000470001);
    pub const DEVICE_IMAGE_SUBRESOURCE_INFO: Self = Self(1000470004);
    pub const SUBRESOURCE_LAYOUT_2: Self = Self(1000338002);
    pub const IMAGE_SUBRESOURCE_2: Self = Self(1000338003);
    pub const BUFFER_USAGE_FLAGS_2_CREATE_INFO: Self = Self(1000470006);
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES: Self = Self(1000545000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES: Self = Self(1000545001);
    pub const BIND_MEMORY_STATUS: Self = Self(1000545002);
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES: Self = Self(1000270000);
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES: Self = Self(1000270001);
    pub const MEMORY_TO_IMAGE_COPY: Self = Self(1000270002);
    pub const IMAGE_TO_MEMORY_COPY: Self = Self(1000270003);
    pub const COPY_IMAGE_TO_MEMORY_INFO: Self = Self(1000270004);
    pub const COPY_MEMORY_TO_IMAGE_INFO: Self = Self(1000270005);
    pub const HOST_IMAGE_LAYOUT_TRANSITION_INFO: Self = Self(1000270006);
    pub const COPY_IMAGE_TO_IMAGE_INFO: Self = Self(1000270007);
    pub const SUBRESOURCE_HOST_MEMCPY_SIZE: Self = Self(1000270008);
    pub const HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY: Self = Self(1000270009);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES: Self = Self(1000416000);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES: Self = Self(1000528000);
    pub const PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES: Self = Self(1000544000);
    pub const PIPELINE_CREATE_FLAGS_2_CREATE_INFO: Self = Self(1000470005);
    pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES: Self = Self(1000080000);
    pub const BIND_DESCRIPTOR_SETS_INFO: Self = Self(1000545003);
    pub const PUSH_CONSTANTS_INFO: Self = Self(1000545004);
    pub const PUSH_DESCRIPTOR_SET_INFO: Self = Self(1000545005);
    pub const PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO: Self = Self(1000545006);
    pub const PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES: Self = Self(1000466000);
    pub const PIPELINE_ROBUSTNESS_CREATE_INFO: Self = Self(1000068000);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES: Self = Self(1000068001);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES: Self = Self(1000068002);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES: Self = Self(1000259000);
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO: Self = Self(1000259001);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES: Self = Self(1000259002);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES: Self = Self(1000525000);
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO: Self = Self(1000190001);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES: Self = Self(1000190002);
    pub const RENDERING_AREA_INFO: Self = Self(1000470003);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES: Self = Self(1000232000);
    pub const RENDERING_ATTACHMENT_LOCATION_INFO: Self = Self(1000232001);
    pub const RENDERING_INPUT_ATTACHMENT_INDEX_INFO: Self = Self(1000232002);
    pub const SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000001000);
    pub const PRESENT_INFO_KHR: Self = Self(1000001001);
    pub const DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: Self = Self(1000060007);
    pub const IMAGE_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000060008);
    pub const BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: Self = Self(1000060009);
    pub const ACQUIRE_NEXT_IMAGE_INFO_KHR: Self = Self(1000060010);
    pub const DEVICE_GROUP_PRESENT_INFO_KHR: Self = Self(1000060011);
    pub const DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000060012);
    pub const DISPLAY_MODE_CREATE_INFO_KHR: Self = Self(1000002000);
    pub const DISPLAY_SURFACE_CREATE_INFO_KHR: Self = Self(1000002001);
    pub const DISPLAY_PRESENT_INFO_KHR: Self = Self(1000003000);
    pub const XLIB_SURFACE_CREATE_INFO_KHR: Self = Self(1000004000);
    pub const XCB_SURFACE_CREATE_INFO_KHR: Self = Self(1000005000);
    pub const WAYLAND_SURFACE_CREATE_INFO_KHR: Self = Self(1000006000);
    pub const ANDROID_SURFACE_CREATE_INFO_KHR: Self = Self(1000008000);
    pub const WIN32_SURFACE_CREATE_INFO_KHR: Self = Self(1000009000);
    pub const DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT: Self = Self(1000011000);
    #[deprecated = "aliased"]
    pub const DEBUG_REPORT_CREATE_INFO_EXT: Self = Self::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT;
    pub const PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD: Self = Self(1000018000);
    pub const DEBUG_MARKER_OBJECT_NAME_INFO_EXT: Self = Self(1000022000);
    pub const DEBUG_MARKER_OBJECT_TAG_INFO_EXT: Self = Self(1000022001);
    pub const DEBUG_MARKER_MARKER_INFO_EXT: Self = Self(1000022002);
    pub const VIDEO_PROFILE_INFO_KHR: Self = Self(1000023000);
    pub const VIDEO_CAPABILITIES_KHR: Self = Self(1000023001);
    pub const VIDEO_PICTURE_RESOURCE_INFO_KHR: Self = Self(1000023002);
    pub const VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR: Self = Self(1000023003);
    pub const BIND_VIDEO_SESSION_MEMORY_INFO_KHR: Self = Self(1000023004);
    pub const VIDEO_SESSION_CREATE_INFO_KHR: Self = Self(1000023005);
    pub const VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000023006);
    pub const VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR: Self = Self(1000023007);
    pub const VIDEO_BEGIN_CODING_INFO_KHR: Self = Self(1000023008);
    pub const VIDEO_END_CODING_INFO_KHR: Self = Self(1000023009);
    pub const VIDEO_CODING_CONTROL_INFO_KHR: Self = Self(1000023010);
    pub const VIDEO_REFERENCE_SLOT_INFO_KHR: Self = Self(1000023011);
    pub const QUEUE_FAMILY_VIDEO_PROPERTIES_KHR: Self = Self(1000023012);
    pub const VIDEO_PROFILE_LIST_INFO_KHR: Self = Self(1000023013);
    pub const PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR: Self = Self(1000023014);
    pub const VIDEO_FORMAT_PROPERTIES_KHR: Self = Self(1000023015);
    pub const QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR: Self = Self(1000023016);
    pub const VIDEO_DECODE_INFO_KHR: Self = Self(1000024000);
    pub const VIDEO_DECODE_CAPABILITIES_KHR: Self = Self(1000024001);
    pub const VIDEO_DECODE_USAGE_INFO_KHR: Self = Self(1000024002);
    pub const DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: Self = Self(1000026000);
    pub const DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: Self = Self(1000026001);
    pub const DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV: Self = Self(1000026002);
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT: Self = Self(1000028000);
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT: Self = Self(1000028001);
    pub const PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT: Self = Self(1000028002);
    pub const CU_MODULE_CREATE_INFO_NVX: Self = Self(1000029000);
    pub const CU_FUNCTION_CREATE_INFO_NVX: Self = Self(1000029001);
    pub const CU_LAUNCH_INFO_NVX: Self = Self(1000029002);
    pub const CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX: Self = Self(1000029004);
    pub const IMAGE_VIEW_HANDLE_INFO_NVX: Self = Self(1000030000);
    pub const IMAGE_VIEW_ADDRESS_PROPERTIES_NVX: Self = Self(1000030001);
    pub const VIDEO_ENCODE_H264_CAPABILITIES_KHR: Self = Self(1000038000);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000038001);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000038002);
    pub const VIDEO_ENCODE_H264_PICTURE_INFO_KHR: Self = Self(1000038003);
    pub const VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR: Self = Self(1000038004);
    pub const VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR: Self = Self(1000038005);
    pub const VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000038006);
    pub const VIDEO_ENCODE_H264_PROFILE_INFO_KHR: Self = Self(1000038007);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR: Self = Self(1000038008);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000038009);
    pub const VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR: Self = Self(1000038010);
    pub const VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000038011);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000038012);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000038013);
    pub const VIDEO_ENCODE_H265_CAPABILITIES_KHR: Self = Self(1000039000);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000039001);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000039002);
    pub const VIDEO_ENCODE_H265_PICTURE_INFO_KHR: Self = Self(1000039003);
    pub const VIDEO_ENCODE_H265_DPB_SLOT_INFO_KHR: Self = Self(1000039004);
    pub const VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_KHR: Self = Self(1000039005);
    pub const VIDEO_ENCODE_H265_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000039006);
    pub const VIDEO_ENCODE_H265_PROFILE_INFO_KHR: Self = Self(1000039007);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_INFO_KHR: Self = Self(1000039009);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000039010);
    pub const VIDEO_ENCODE_H265_SESSION_CREATE_INFO_KHR: Self = Self(1000039011);
    pub const VIDEO_ENCODE_H265_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000039012);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000039013);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000039014);
    pub const VIDEO_DECODE_H264_CAPABILITIES_KHR: Self = Self(1000040000);
    pub const VIDEO_DECODE_H264_PICTURE_INFO_KHR: Self = Self(1000040001);
    pub const VIDEO_DECODE_H264_PROFILE_INFO_KHR: Self = Self(1000040003);
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000040004);
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000040005);
    pub const VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR: Self = Self(1000040006);
    pub const TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: Self = Self(1000041000);
    pub const RENDERING_INFO_KHR: Self = Self::RENDERING_INFO;
    pub const RENDERING_ATTACHMENT_INFO_KHR: Self = Self::RENDERING_ATTACHMENT_INFO;
    pub const PIPELINE_RENDERING_CREATE_INFO_KHR: Self = Self::PIPELINE_RENDERING_CREATE_INFO;
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES;
    pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR: Self =
        Self::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO;
    pub const STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP: Self = Self(1000049000);
    pub const PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV: Self = Self(1000050000);
    pub const PRIVATE_VENDOR_INFO_PLACEHOLDER_OFFSET_0_NV: Self = Self(1000051000);
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR: Self = Self::RENDER_PASS_MULTIVIEW_CREATE_INFO;
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_MULTIVIEW_FEATURES;
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES;
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV: Self = Self(1000056000);
    pub const EXPORT_MEMORY_ALLOCATE_INFO_NV: Self = Self(1000056001);
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057000);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057001);
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: Self = Self(1000058000);
    pub const PHYSICAL_DEVICE_FEATURES_2_KHR: Self = Self::PHYSICAL_DEVICE_FEATURES_2;
    pub const PHYSICAL_DEVICE_PROPERTIES_2_KHR: Self = Self::PHYSICAL_DEVICE_PROPERTIES_2;
    pub const FORMAT_PROPERTIES_2_KHR: Self = Self::FORMAT_PROPERTIES_2;
    pub const IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self::IMAGE_FORMAT_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR: Self =
        Self::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2;
    pub const QUEUE_FAMILY_PROPERTIES_2_KHR: Self = Self::QUEUE_FAMILY_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR: Self =
        Self::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2;
    pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self::SPARSE_IMAGE_FORMAT_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR: Self =
        Self::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2;
    pub const MEMORY_ALLOCATE_FLAGS_INFO_KHR: Self = Self::MEMORY_ALLOCATE_FLAGS_INFO;
    pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR: Self =
        Self::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO;
    pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR: Self =
        Self::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO;
    pub const DEVICE_GROUP_SUBMIT_INFO_KHR: Self = Self::DEVICE_GROUP_SUBMIT_INFO;
    pub const DEVICE_GROUP_BIND_SPARSE_INFO_KHR: Self = Self::DEVICE_GROUP_BIND_SPARSE_INFO;
    pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR: Self =
        Self::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO;
    pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR: Self =
        Self::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO;
    pub const VALIDATION_FLAGS_EXT: Self = Self(1000061000);
    pub const VI_SURFACE_CREATE_INFO_NN: Self = Self(1000062000);
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES;
    pub const IMAGE_VIEW_ASTC_DECODE_MODE_EXT: Self = Self(1000067000);
    pub const PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT: Self = Self(1000067001);
    pub const PIPELINE_ROBUSTNESS_CREATE_INFO_EXT: Self = Self::PIPELINE_ROBUSTNESS_CREATE_INFO;
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES;
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES;
    pub const PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_GROUP_PROPERTIES;
    pub const DEVICE_GROUP_DEVICE_CREATE_INFO_KHR: Self = Self::DEVICE_GROUP_DEVICE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO;
    pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR: Self = Self::EXTERNAL_IMAGE_FORMAT_PROPERTIES;
    pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO;
    pub const EXTERNAL_BUFFER_PROPERTIES_KHR: Self = Self::EXTERNAL_BUFFER_PROPERTIES;
    pub const PHYSICAL_DEVICE_ID_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_ID_PROPERTIES;
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR: Self =
        Self::EXTERNAL_MEMORY_BUFFER_CREATE_INFO;
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR: Self = Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO;
    pub const EXPORT_MEMORY_ALLOCATE_INFO_KHR: Self = Self::EXPORT_MEMORY_ALLOCATE_INFO;
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1000073000);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1000073001);
    pub const MEMORY_WIN32_HANDLE_PROPERTIES_KHR: Self = Self(1000073002);
    pub const MEMORY_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000073003);
    pub const IMPORT_MEMORY_FD_INFO_KHR: Self = Self(1000074000);
    pub const MEMORY_FD_PROPERTIES_KHR: Self = Self(1000074001);
    pub const MEMORY_GET_FD_INFO_KHR: Self = Self(1000074002);
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: Self = Self(1000075000);
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO;
    pub const EXTERNAL_SEMAPHORE_PROPERTIES_KHR: Self = Self::EXTERNAL_SEMAPHORE_PROPERTIES;
    pub const EXPORT_SEMAPHORE_CREATE_INFO_KHR: Self = Self::EXPORT_SEMAPHORE_CREATE_INFO;
    pub const IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1000078000);
    pub const EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1000078001);
    pub const D3D12_FENCE_SUBMIT_INFO_KHR: Self = Self(1000078002);
    pub const SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000078003);
    pub const IMPORT_SEMAPHORE_FD_INFO_KHR: Self = Self(1000079000);
    pub const SEMAPHORE_GET_FD_INFO_KHR: Self = Self(1000079001);
    pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES;
    pub const COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT: Self = Self(1000081000);
    pub const PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT: Self = Self(1000081001);
    pub const CONDITIONAL_RENDERING_BEGIN_INFO_EXT: Self = Self(1000081002);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
    pub const PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES;
    pub const PRESENT_REGIONS_KHR: Self = Self(1000084000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR: Self =
        Self::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO;
    pub const PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV: Self = Self(1000087000);
    pub const SURFACE_CAPABILITIES_2_EXT: Self = Self(1000090000);
    #[deprecated = "aliased"]
    pub const SURFACE_CAPABILITIES2_EXT: Self = Self::SURFACE_CAPABILITIES_2_EXT;
    pub const DISPLAY_POWER_INFO_EXT: Self = Self(1000091000);
    pub const DEVICE_EVENT_INFO_EXT: Self = Self(1000091001);
    pub const DISPLAY_EVENT_INFO_EXT: Self = Self(1000091002);
    pub const SWAPCHAIN_COUNTER_CREATE_INFO_EXT: Self = Self(1000091003);
    pub const PRESENT_TIMES_INFO_GOOGLE: Self = Self(1000092000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX: Self = Self(1000097000);
    pub const MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX: Self = Self(1000044009);
    pub const PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV: Self = Self(1000098000);
    pub const PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT: Self = Self(1000099000);
    pub const PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT: Self = Self(1000099001);
    pub const PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT: Self = Self(1000101000);
    pub const PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT: Self = Self(1000101001);
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT: Self = Self(1000102000);
    pub const PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT: Self = Self(1000102001);
    pub const HDR_METADATA_EXT: Self = Self(1000105000);
    pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES;
    pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR: Self =
        Self::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO;
    pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR: Self = Self::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO;
    pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR: Self = Self::RENDER_PASS_ATTACHMENT_BEGIN_INFO;
    pub const ATTACHMENT_DESCRIPTION_2_KHR: Self = Self::ATTACHMENT_DESCRIPTION_2;
    pub const ATTACHMENT_REFERENCE_2_KHR: Self = Self::ATTACHMENT_REFERENCE_2;
    pub const SUBPASS_DESCRIPTION_2_KHR: Self = Self::SUBPASS_DESCRIPTION_2;
    pub const SUBPASS_DEPENDENCY_2_KHR: Self = Self::SUBPASS_DEPENDENCY_2;
    pub const RENDER_PASS_CREATE_INFO_2_KHR: Self = Self::RENDER_PASS_CREATE_INFO_2;
    pub const SUBPASS_BEGIN_INFO_KHR: Self = Self::SUBPASS_BEGIN_INFO;
    pub const SUBPASS_END_INFO_KHR: Self = Self::SUBPASS_END_INFO;
    pub const PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG: Self = Self(1000110000);
    pub const SHARED_PRESENT_SURFACE_CAPABILITIES_KHR: Self = Self(1000111000);
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO;
    pub const EXTERNAL_FENCE_PROPERTIES_KHR: Self = Self::EXTERNAL_FENCE_PROPERTIES;
    pub const EXPORT_FENCE_CREATE_INFO_KHR: Self = Self::EXPORT_FENCE_CREATE_INFO;
    pub const IMPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1000114000);
    pub const EXPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1000114001);
    pub const FENCE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000114002);
    pub const IMPORT_FENCE_FD_INFO_KHR: Self = Self(1000115000);
    pub const FENCE_GET_FD_INFO_KHR: Self = Self(1000115001);
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR: Self = Self(1000116000);
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR: Self = Self(1000116001);
    pub const QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR: Self = Self(1000116002);
    pub const PERFORMANCE_QUERY_SUBMIT_INFO_KHR: Self = Self(1000116003);
    pub const ACQUIRE_PROFILING_LOCK_INFO_KHR: Self = Self(1000116004);
    pub const PERFORMANCE_COUNTER_KHR: Self = Self(1000116005);
    pub const PERFORMANCE_COUNTER_DESCRIPTION_KHR: Self = Self(1000116006);
    pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES;
    pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR: Self =
        Self::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO;
    pub const IMAGE_VIEW_USAGE_CREATE_INFO_KHR: Self = Self::IMAGE_VIEW_USAGE_CREATE_INFO;
    pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR: Self =
        Self::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: Self = Self(1000119000);
    pub const SURFACE_CAPABILITIES_2_KHR: Self = Self(1000119001);
    pub const SURFACE_FORMAT_2_KHR: Self = Self(1000119002);
    pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
    pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR;
    pub const DISPLAY_PROPERTIES_2_KHR: Self = Self(1000121000);
    pub const DISPLAY_PLANE_PROPERTIES_2_KHR: Self = Self(1000121001);
    pub const DISPLAY_MODE_PROPERTIES_2_KHR: Self = Self(1000121002);
    pub const DISPLAY_PLANE_INFO_2_KHR: Self = Self(1000121003);
    pub const DISPLAY_PLANE_CAPABILITIES_2_KHR: Self = Self(1000121004);
    pub const IOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000122000);
    pub const MACOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000123000);
    pub const MEMORY_DEDICATED_REQUIREMENTS_KHR: Self = Self::MEMORY_DEDICATED_REQUIREMENTS;
    pub const MEMORY_DEDICATED_ALLOCATE_INFO_KHR: Self = Self::MEMORY_DEDICATED_ALLOCATE_INFO;
    pub const DEBUG_UTILS_OBJECT_NAME_INFO_EXT: Self = Self(1000128000);
    pub const DEBUG_UTILS_OBJECT_TAG_INFO_EXT: Self = Self(1000128001);
    pub const DEBUG_UTILS_LABEL_EXT: Self = Self(1000128002);
    pub const DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: Self = Self(1000128003);
    pub const DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: Self = Self(1000128004);
    pub const ANDROID_HARDWARE_BUFFER_USAGE_ANDROID: Self = Self(1000129000);
    pub const ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: Self = Self(1000129001);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID: Self = Self(1000129002);
    pub const IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1000129003);
    pub const MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1000129004);
    pub const EXTERNAL_FORMAT_ANDROID: Self = Self(1000129005);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID: Self = Self(1000129006);
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES;
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT: Self =
        Self::SAMPLER_REDUCTION_MODE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX: Self = Self(1000134000);
    pub const PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX: Self = Self(1000134001);
    pub const EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX: Self = Self(1000134002);
    pub const EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX: Self = Self(1000134003);
    pub const PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX: Self = Self(1000134004);
    pub const TEXEL_BUFFER_DESCRIPTOR_INFO_EXT: Self = Self(1000135000);
    pub const IMAGE_DESCRIPTOR_INFO_EXT: Self = Self(1000135001);
    pub const RESOURCE_DESCRIPTOR_INFO_EXT: Self = Self(1000135002);
    pub const BIND_HEAP_INFO_EXT: Self = Self(1000135003);
    pub const PUSH_DATA_INFO_EXT: Self = Self(1000135004);
    pub const DESCRIPTOR_SET_AND_BINDING_MAPPING_EXT: Self = Self(1000135005);
    pub const SHADER_DESCRIPTOR_SET_AND_BINDING_MAPPING_INFO_EXT: Self = Self(1000135006);
    pub const OPAQUE_CAPTURE_DATA_CREATE_INFO_EXT: Self = Self(1000135007);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_PROPERTIES_EXT: Self = Self(1000135008);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_FEATURES_EXT: Self = Self(1000135009);
    pub const COMMAND_BUFFER_INHERITANCE_DESCRIPTOR_HEAP_INFO_EXT: Self = Self(1000135010);
    pub const SAMPLER_CUSTOM_BORDER_COLOR_INDEX_CREATE_INFO_EXT: Self = Self(1000135011);
    pub const INDIRECT_COMMANDS_LAYOUT_PUSH_DATA_TOKEN_NV: Self = Self(1000135012);
    pub const SUBSAMPLED_IMAGE_FORMAT_PROPERTIES_EXT: Self = Self(1000135013);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_TENSOR_PROPERTIES_ARM: Self = Self(1000135014);
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_AMD: Self = Self(1000044008);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES;
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES;
    pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT: Self =
        Self::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK;
    pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT: Self =
        Self::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO;
    pub const PHYSICAL_DEVICE_SHADER_BFLOAT16_FEATURES_KHR: Self = Self(1000141000);
    pub const SAMPLE_LOCATIONS_INFO_EXT: Self = Self(1000143000);
    pub const RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT: Self = Self(1000143001);
    pub const PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT: Self = Self(1000143002);
    pub const PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT: Self = Self(1000143003);
    pub const MULTISAMPLE_PROPERTIES_EXT: Self = Self(1000143004);
    pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self::BUFFER_MEMORY_REQUIREMENTS_INFO_2;
    pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self::IMAGE_MEMORY_REQUIREMENTS_INFO_2;
    pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self =
        Self::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2;
    pub const MEMORY_REQUIREMENTS_2_KHR: Self = Self::MEMORY_REQUIREMENTS_2;
    pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR: Self =
        Self::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2;
    pub const IMAGE_FORMAT_LIST_CREATE_INFO_KHR: Self = Self::IMAGE_FORMAT_LIST_CREATE_INFO;
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: Self = Self(1000148000);
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: Self = Self(1000148001);
    pub const PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: Self = Self(1000148002);
    pub const PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV: Self = Self(1000149000);
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR: Self = Self(1000150007);
    pub const ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR: Self = Self(1000150002);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR: Self = Self(1000150003);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR: Self = Self(1000150004);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR: Self = Self(1000150005);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_KHR: Self = Self(1000150006);
    pub const ACCELERATION_STRUCTURE_VERSION_INFO_KHR: Self = Self(1000150009);
    pub const COPY_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150010);
    pub const COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR: Self = Self(1000150011);
    pub const COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150012);
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR: Self = Self(1000150013);
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR: Self = Self(1000150014);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_KHR: Self = Self(1000150017);
    pub const ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR: Self = Self(1000150020);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR: Self = Self(1000347000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR: Self = Self(1000347001);
    pub const RAY_TRACING_PIPELINE_CREATE_INFO_KHR: Self = Self(1000150015);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR: Self = Self(1000150016);
    pub const RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR: Self = Self(1000150018);
    pub const PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR: Self = Self(1000348013);
    pub const PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV: Self = Self(1000152000);
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_NV: Self = Self::ATTACHMENT_SAMPLE_COUNT_INFO_AMD;
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV: Self = Self(1000154000);
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV: Self = Self(1000154001);
    pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR: Self =
        Self::SAMPLER_YCBCR_CONVERSION_CREATE_INFO;
    pub const SAMPLER_YCBCR_CONVERSION_INFO_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION_INFO;
    pub const BIND_IMAGE_PLANE_MEMORY_INFO_KHR: Self = Self::BIND_IMAGE_PLANE_MEMORY_INFO;
    pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR: Self =
        Self::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO;
    pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES;
    pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR: Self =
        Self::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES;
    pub const BIND_BUFFER_MEMORY_INFO_KHR: Self = Self::BIND_BUFFER_MEMORY_INFO;
    pub const BIND_IMAGE_MEMORY_INFO_KHR: Self = Self::BIND_IMAGE_MEMORY_INFO;
    pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT: Self = Self(1000158000);
    pub const PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT: Self = Self(1000158002);
    pub const IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT: Self = Self(1000158003);
    pub const IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT: Self = Self(1000158004);
    pub const IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: Self = Self(1000158005);
    pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT: Self = Self(1000158006);
    pub const VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160000);
    pub const SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160001);
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT: Self =
        Self::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO;
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES;
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES;
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT: Self =
        Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO;
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT: Self =
        Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT;
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR: Self = Self(1000163000);
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR: Self = Self(1000163001);
    pub const PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV: Self = Self(1000164000);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV: Self = Self(1000164001);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV: Self = Self(1000164002);
    pub const PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV: Self = Self(1000164005);
    pub const RAY_TRACING_PIPELINE_CREATE_INFO_NV: Self = Self(1000165000);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_NV: Self = Self(1000165001);
    pub const GEOMETRY_NV: Self = Self(1000165003);
    pub const GEOMETRY_TRIANGLES_NV: Self = Self(1000165004);
    pub const GEOMETRY_AABB_NV: Self = Self(1000165005);
    pub const BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV: Self = Self(1000165006);
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV: Self = Self(1000165007);
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1000165008);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV: Self = Self(1000165009);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1000165011);
    pub const ACCELERATION_STRUCTURE_INFO_NV: Self = Self(1000165012);
    pub const PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV: Self = Self(1000166000);
    pub const PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV: Self = Self(1000166001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES;
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: Self = Self::DESCRIPTOR_SET_LAYOUT_SUPPORT;
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT: Self = Self(1000170000);
    pub const FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT: Self = Self(1000170001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_CONVERSION_FEATURES_QCOM: Self = Self(1000172000);
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: Self =
        Self::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO;
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES;
    pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES;
    pub const IMPORT_MEMORY_HOST_POINTER_INFO_EXT: Self = Self(1000178000);
    pub const MEMORY_HOST_POINTER_PROPERTIES_EXT: Self = Self(1000178001);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT: Self = Self(1000178002);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR: Self = Self(1000181000);
    pub const PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD: Self = Self(1000183000);
    pub const CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self::CALIBRATED_TIMESTAMP_INFO_KHR;
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD: Self = Self(1000185000);
    pub const VIDEO_DECODE_H265_CAPABILITIES_KHR: Self = Self(1000187000);
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000187001);
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000187002);
    pub const VIDEO_DECODE_H265_PROFILE_INFO_KHR: Self = Self(1000187003);
    pub const VIDEO_DECODE_H265_PICTURE_INFO_KHR: Self = Self(1000187004);
    pub const VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR: Self = Self(1000187005);
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR: Self =
        Self::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO;
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES;
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR: Self =
        Self::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES;
    pub const DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD: Self = Self(1000189000);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT: Self = Self(1000190000);
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT: Self =
        Self::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES;
    pub const PRESENT_FRAME_TOKEN_GGP: Self = Self(1000191000);
    pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT: Self =
        Self::PIPELINE_CREATION_FEEDBACK_CREATE_INFO;
    pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_DRIVER_PROPERTIES;
    pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES;
    pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES;
    pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR: Self =
        Self::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE;
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV: Self =
        Self::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR;
    pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV: Self = Self(1000202000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV: Self = Self(1000202001);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV: Self =
        Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR;
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV: Self = Self(1000204000);
    pub const PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV: Self = Self(1000205000);
    pub const PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV: Self = Self(1000205002);
    pub const CHECKPOINT_DATA_NV: Self = Self(1000206000);
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV: Self = Self(1000206001);
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV: Self = Self(1000314008);
    pub const CHECKPOINT_DATA_2_NV: Self = Self(1000314009);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES;
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES;
    pub const SEMAPHORE_TYPE_CREATE_INFO_KHR: Self = Self::SEMAPHORE_TYPE_CREATE_INFO;
    pub const TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR: Self = Self::TIMELINE_SEMAPHORE_SUBMIT_INFO;
    pub const SEMAPHORE_WAIT_INFO_KHR: Self = Self::SEMAPHORE_WAIT_INFO;
    pub const SEMAPHORE_SIGNAL_INFO_KHR: Self = Self::SEMAPHORE_SIGNAL_INFO;
    pub const PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT: Self = Self(1000208000);
    pub const SWAPCHAIN_TIMING_PROPERTIES_EXT: Self = Self(1000208001);
    pub const SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT: Self = Self(1000208002);
    pub const PRESENT_TIMINGS_INFO_EXT: Self = Self(1000208003);
    pub const PRESENT_TIMING_INFO_EXT: Self = Self(1000208004);
    pub const PAST_PRESENTATION_TIMING_INFO_EXT: Self = Self(1000208005);
    pub const PAST_PRESENTATION_TIMING_PROPERTIES_EXT: Self = Self(1000208006);
    pub const PAST_PRESENTATION_TIMING_EXT: Self = Self(1000208007);
    pub const PRESENT_TIMING_SURFACE_CAPABILITIES_EXT: Self = Self(1000208008);
    pub const SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self(1000208009);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL: Self = Self(1000209000);
    pub const QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL: Self = Self(1000210000);
    #[deprecated = "aliased"]
    pub const QUERY_POOL_CREATE_INFO_INTEL: Self =
        Self::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL;
    pub const INITIALIZE_PERFORMANCE_API_INFO_INTEL: Self = Self(1000210001);
    pub const PERFORMANCE_MARKER_INFO_INTEL: Self = Self(1000210002);
    pub const PERFORMANCE_STREAM_MARKER_INFO_INTEL: Self = Self(1000210003);
    pub const PERFORMANCE_OVERRIDE_INFO_INTEL: Self = Self(1000210004);
    pub const PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL: Self = Self(1000210005);
    pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES;
    pub const PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT: Self = Self(1000212000);
    pub const DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD: Self = Self(1000213000);
    pub const SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD: Self = Self(1000213001);
    pub const IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA: Self = Self(1000214000);
    pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES;
    pub const METAL_SURFACE_CREATE_INFO_EXT: Self = Self(1000217000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT: Self = Self(1000218000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT: Self = Self(1000218001);
    pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT: Self = Self(1000218002);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT: Self = Self(1000044007);
    pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES;
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES;
    pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self =
        Self::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES;
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1000226000);
    pub const PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR: Self = Self(1000226001);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR: Self = Self(1000226002);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR: Self = Self(1000226003);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR: Self = Self(1000226004);
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1000044006);
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD: Self = Self(1000227000);
    pub const PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD: Self = Self(1000229000);
    pub const PHYSICAL_DEVICE_SHADER_CONSTANT_DATA_FEATURES_KHR: Self = Self(1000231000);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES;
    pub const RENDERING_ATTACHMENT_LOCATION_INFO_KHR: Self =
        Self::RENDERING_ATTACHMENT_LOCATION_INFO;
    pub const RENDERING_INPUT_ATTACHMENT_INDEX_INFO_KHR: Self =
        Self::RENDERING_INPUT_ATTACHMENT_INDEX_INFO;
    pub const PHYSICAL_DEVICE_SHADER_ABORT_FEATURES_KHR: Self = Self(1000233000);
    pub const DEVICE_FAULT_SHADER_ABORT_MESSAGE_INFO_KHR: Self = Self(1000233001);
    pub const PHYSICAL_DEVICE_SHADER_ABORT_PROPERTIES_KHR: Self = Self(1000233002);
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT: Self = Self(1000234000);
    pub const PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR: Self = Self(1000235000);
    pub const PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT: Self = Self(1000237000);
    pub const PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT: Self = Self(1000238000);
    pub const MEMORY_PRIORITY_ALLOCATE_INFO_EXT: Self = Self(1000238001);
    pub const SURFACE_PROTECTED_CAPABILITIES_KHR: Self = Self(1000239000);
    pub const PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV: Self =
        Self(1000240000);
    pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES;
    pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR: Self =
        Self::ATTACHMENT_REFERENCE_STENCIL_LAYOUT;
    pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR: Self =
        Self::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT;
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT: Self = Self(1000244000);
    pub const PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT;
    pub const BUFFER_DEVICE_ADDRESS_INFO_EXT: Self = Self::BUFFER_DEVICE_ADDRESS_INFO;
    pub const BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT: Self = Self(1000244002);
    pub const PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT: Self = Self::PHYSICAL_DEVICE_TOOL_PROPERTIES;
    pub const IMAGE_STENCIL_USAGE_CREATE_INFO_EXT: Self = Self::IMAGE_STENCIL_USAGE_CREATE_INFO;
    pub const VALIDATION_FEATURES_EXT: Self = Self(1000247000);
    pub const PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR: Self = Self(1000248000);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV: Self = Self(1000249000);
    pub const COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249002);
    pub const PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV: Self = Self(1000250000);
    pub const PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV: Self = Self(1000250001);
    pub const FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV: Self = Self(1000250002);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT: Self = Self(1000251000);
    pub const PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT: Self = Self(1000252000);
    pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES;
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT: Self = Self(1000254000);
    pub const PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT: Self =
        Self(1000254001);
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT: Self = Self(1000254002);
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT: Self = Self(1000255000);
    pub const SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT: Self = Self(1000255002);
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT: Self = Self(1000255001);
    pub const HEADLESS_SURFACE_CREATE_INFO_EXT: Self = Self(1000256000);
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;
    pub const BUFFER_DEVICE_ADDRESS_INFO_KHR: Self = Self::BUFFER_DEVICE_ADDRESS_INFO;
    pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR: Self =
        Self::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO;
    pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR: Self =
        Self::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO;
    pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR: Self =
        Self::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO;
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES;
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT: Self =
        Self::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES;
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT: Self = Self(1000260000);
    pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES;
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES;
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1000267000);
    pub const PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR: Self = Self(1000269000);
    pub const PIPELINE_INFO_KHR: Self = Self(1000269001);
    pub const PIPELINE_EXECUTABLE_PROPERTIES_KHR: Self = Self(1000269002);
    pub const PIPELINE_EXECUTABLE_INFO_KHR: Self = Self(1000269003);
    pub const PIPELINE_EXECUTABLE_STATISTIC_KHR: Self = Self(1000269004);
    pub const PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR: Self = Self(1000269005);
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES;
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES;
    pub const MEMORY_TO_IMAGE_COPY_EXT: Self = Self::MEMORY_TO_IMAGE_COPY;
    pub const IMAGE_TO_MEMORY_COPY_EXT: Self = Self::IMAGE_TO_MEMORY_COPY;
    pub const COPY_IMAGE_TO_MEMORY_INFO_EXT: Self = Self::COPY_IMAGE_TO_MEMORY_INFO;
    pub const COPY_MEMORY_TO_IMAGE_INFO_EXT: Self = Self::COPY_MEMORY_TO_IMAGE_INFO;
    pub const HOST_IMAGE_LAYOUT_TRANSITION_INFO_EXT: Self = Self::HOST_IMAGE_LAYOUT_TRANSITION_INFO;
    pub const COPY_IMAGE_TO_IMAGE_INFO_EXT: Self = Self::COPY_IMAGE_TO_IMAGE_INFO;
    pub const SUBRESOURCE_HOST_MEMCPY_SIZE_EXT: Self = Self::SUBRESOURCE_HOST_MEMCPY_SIZE;
    pub const HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY_EXT: Self =
        Self::HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY;
    pub const MEMORY_MAP_INFO_KHR: Self = Self::MEMORY_MAP_INFO;
    pub const MEMORY_UNMAP_INFO_KHR: Self = Self::MEMORY_UNMAP_INFO;
    pub const PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT: Self = Self(1000272000);
    pub const PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT: Self = Self(1000272001);
    pub const MEMORY_MAP_PLACED_INFO_EXT: Self = Self(1000272002);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT: Self = Self(1000273000);
    pub const SURFACE_PRESENT_MODE_EXT: Self = Self::SURFACE_PRESENT_MODE_KHR;
    pub const SURFACE_PRESENT_SCALING_CAPABILITIES_EXT: Self =
        Self::SURFACE_PRESENT_SCALING_CAPABILITIES_KHR;
    pub const SURFACE_PRESENT_MODE_COMPATIBILITY_EXT: Self =
        Self::SURFACE_PRESENT_MODE_COMPATIBILITY_KHR;
    pub const PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR;
    pub const SWAPCHAIN_PRESENT_FENCE_INFO_EXT: Self = Self::SWAPCHAIN_PRESENT_FENCE_INFO_KHR;
    pub const SWAPCHAIN_PRESENT_MODES_CREATE_INFO_EXT: Self =
        Self::SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR;
    pub const SWAPCHAIN_PRESENT_MODE_INFO_EXT: Self = Self::SWAPCHAIN_PRESENT_MODE_INFO_KHR;
    pub const SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_EXT: Self =
        Self::SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR;
    pub const RELEASE_SWAPCHAIN_IMAGES_INFO_EXT: Self = Self::RELEASE_SWAPCHAIN_IMAGES_INFO_KHR;
    pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES;
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV: Self = Self(1000277000);
    pub const GRAPHICS_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1000277001);
    pub const GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV: Self = Self(1000277002);
    pub const INDIRECT_COMMANDS_LAYOUT_TOKEN_NV: Self = Self(1000277003);
    pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV: Self = Self(1000277004);
    pub const GENERATED_COMMANDS_INFO_NV: Self = Self(1000277005);
    pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1000277006);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV: Self = Self(1000277007);
    pub const PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV: Self = Self(1000278000);
    pub const COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV: Self = Self(1000278001);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES;
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT: Self = Self(1000281000);
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES;
    pub const COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM: Self = Self(1000282000);
    pub const RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM: Self = Self(1000282001);
    pub const PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT: Self = Self(1000283000);
    pub const DEPTH_BIAS_INFO_EXT: Self = Self(1000283001);
    pub const DEPTH_BIAS_REPRESENTATION_INFO_EXT: Self = Self(1000283002);
    pub const PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT: Self = Self(1000284000);
    pub const DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT: Self = Self(1000284001);
    pub const DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT: Self = Self(1000284002);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR;
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR;
    pub const SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT: Self = Self(1000287000);
    pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT: Self = Self(1000287001);
    pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT: Self = Self(1000287002);
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_3D_FEATURES_EXT: Self = Self(1000288000);
    pub const PIPELINE_LIBRARY_CREATE_INFO_KHR: Self = Self(1000290000);
    pub const PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV: Self = Self(1000292000);
    pub const SURFACE_CAPABILITIES_PRESENT_BARRIER_NV: Self = Self(1000292001);
    pub const SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV: Self = Self(1000292002);
    pub const PRESENT_ID_KHR: Self = Self(1000294000);
    pub const PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR: Self = Self(1000294001);
    pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES;
    pub const DEVICE_PRIVATE_DATA_CREATE_INFO_EXT: Self = Self::DEVICE_PRIVATE_DATA_CREATE_INFO;
    pub const PRIVATE_DATA_SLOT_CREATE_INFO_EXT: Self = Self::PRIVATE_DATA_SLOT_CREATE_INFO;
    pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES;
    pub const VIDEO_ENCODE_INFO_KHR: Self = Self(1000299000);
    pub const VIDEO_ENCODE_RATE_CONTROL_INFO_KHR: Self = Self(1000299001);
    pub const VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000299002);
    pub const VIDEO_ENCODE_CAPABILITIES_KHR: Self = Self(1000299003);
    pub const VIDEO_ENCODE_USAGE_INFO_KHR: Self = Self(1000299004);
    pub const QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR: Self = Self(1000299005);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR: Self = Self(1000299006);
    pub const VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000299007);
    pub const VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR: Self = Self(1000299008);
    pub const VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000299009);
    pub const VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000299010);
    pub const PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV: Self = Self(1000300000);
    pub const DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV: Self = Self(1000300001);
    pub const PERF_HINT_INFO_QCOM: Self = Self(1000302000);
    pub const PHYSICAL_DEVICE_QUEUE_PERF_HINT_FEATURES_QCOM: Self = Self(1000302001);
    pub const PHYSICAL_DEVICE_QUEUE_PERF_HINT_PROPERTIES_QCOM: Self = Self(1000302002);
    pub const CUDA_MODULE_CREATE_INFO_NV: Self = Self(1000307000);
    pub const CUDA_FUNCTION_CREATE_INFO_NV: Self = Self(1000307001);
    pub const CUDA_LAUNCH_INFO_NV: Self = Self(1000307002);
    pub const PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV: Self = Self(1000307003);
    pub const PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV: Self = Self(1000307004);
    pub const REFRESH_OBJECT_LIST_KHR: Self = Self(1000308000);
    pub const PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM: Self = Self(1000309000);
    pub const PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM: Self = Self(1000309001);
    pub const RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM: Self = Self(1000309002);
    pub const PER_TILE_BEGIN_INFO_QCOM: Self = Self(1000309003);
    pub const PER_TILE_END_INFO_QCOM: Self = Self(1000309004);
    pub const DISPATCH_TILE_INFO_QCOM: Self = Self(1000309005);
    pub const QUERY_LOW_LATENCY_SUPPORT_NV: Self = Self(1000310000);
    pub const EXPORT_METAL_OBJECT_CREATE_INFO_EXT: Self = Self(1000311000);
    pub const EXPORT_METAL_OBJECTS_INFO_EXT: Self = Self(1000311001);
    pub const EXPORT_METAL_DEVICE_INFO_EXT: Self = Self(1000311002);
    pub const EXPORT_METAL_COMMAND_QUEUE_INFO_EXT: Self = Self(1000311003);
    pub const EXPORT_METAL_BUFFER_INFO_EXT: Self = Self(1000311004);
    pub const IMPORT_METAL_BUFFER_INFO_EXT: Self = Self(1000311005);
    pub const EXPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1000311006);
    pub const IMPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1000311007);
    pub const EXPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1000311008);
    pub const IMPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1000311009);
    pub const EXPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1000311010);
    pub const IMPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1000311011);
    pub const MEMORY_BARRIER_2_KHR: Self = Self::MEMORY_BARRIER_2;
    pub const BUFFER_MEMORY_BARRIER_2_KHR: Self = Self::BUFFER_MEMORY_BARRIER_2;
    pub const IMAGE_MEMORY_BARRIER_2_KHR: Self = Self::IMAGE_MEMORY_BARRIER_2;
    pub const DEPENDENCY_INFO_KHR: Self = Self::DEPENDENCY_INFO;
    pub const SUBMIT_INFO_2_KHR: Self = Self::SUBMIT_INFO_2;
    pub const SEMAPHORE_SUBMIT_INFO_KHR: Self = Self::SEMAPHORE_SUBMIT_INFO;
    pub const COMMAND_BUFFER_SUBMIT_INFO_KHR: Self = Self::COMMAND_BUFFER_SUBMIT_INFO;
    pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES;
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT: Self = Self(1000316000);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT: Self = Self(1000316001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT: Self = Self(1000316002);
    pub const DESCRIPTOR_ADDRESS_INFO_EXT: Self = Self(1000316003);
    pub const DESCRIPTOR_GET_INFO_EXT: Self = Self(1000316004);
    pub const BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316005);
    pub const IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316006);
    pub const IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316007);
    pub const SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316008);
    pub const OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT: Self = Self(1000316010);
    pub const DESCRIPTOR_BUFFER_BINDING_INFO_EXT: Self = Self(1000316011);
    pub const DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT: Self = Self(1000316012);
    pub const ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316009);
    pub const DEVICE_MEMORY_COPY_KHR: Self = Self(1000318000);
    pub const COPY_DEVICE_MEMORY_INFO_KHR: Self = Self(1000318001);
    pub const DEVICE_MEMORY_IMAGE_COPY_KHR: Self = Self(1000318002);
    pub const COPY_DEVICE_MEMORY_IMAGE_INFO_KHR: Self = Self(1000318003);
    pub const MEMORY_RANGE_BARRIERS_INFO_KHR: Self = Self(1000318004);
    pub const MEMORY_RANGE_BARRIER_KHR: Self = Self(1000318005);
    pub const PHYSICAL_DEVICE_DEVICE_ADDRESS_COMMANDS_FEATURES_KHR: Self = Self(1000318006);
    pub const BIND_INDEX_BUFFER_3_INFO_KHR: Self = Self(1000318007);
    pub const BIND_VERTEX_BUFFER_3_INFO_KHR: Self = Self(1000318008);
    pub const DRAW_INDIRECT_2_INFO_KHR: Self = Self(1000318009);
    pub const DRAW_INDIRECT_COUNT_2_INFO_KHR: Self = Self(1000318010);
    pub const DISPATCH_INDIRECT_2_INFO_KHR: Self = Self(1000318011);
    pub const CONDITIONAL_RENDERING_BEGIN_INFO_2_EXT: Self = Self(1000318012);
    pub const BIND_TRANSFORM_FEEDBACK_BUFFER_2_INFO_EXT: Self = Self(1000318013);
    pub const MEMORY_MARKER_INFO_AMD: Self = Self(1000318014);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_2_KHR: Self = Self(1000318015);
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT: Self = Self(1000320000);
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT: Self = Self(1000320001);
    pub const GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT: Self = Self(1000320002);
    pub const PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD: Self =
        Self(1000321000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR: Self = Self(1000203000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR: Self = Self(1000322000);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR: Self =
        Self(1000323000);
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES;
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV: Self = Self(1000326000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV: Self = Self(1000326001);
    pub const PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV: Self = Self(1000326002);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV: Self = Self(1000327000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV: Self = Self(1000327001);
    pub const ACCELERATION_STRUCTURE_MOTION_INFO_NV: Self = Self(1000327002);
    pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT: Self = Self(1000328000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT: Self = Self(1000328001);
    pub const PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT: Self = Self(1000330000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT: Self = Self(1000332000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT: Self = Self(1000332001);
    pub const COPY_COMMAND_TRANSFORM_INFO_QCOM: Self = Self(1000333000);
    pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES;
    pub const PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR: Self =
        Self(1000336000);
    pub const COPY_BUFFER_INFO_2_KHR: Self = Self::COPY_BUFFER_INFO_2;
    pub const COPY_IMAGE_INFO_2_KHR: Self = Self::COPY_IMAGE_INFO_2;
    pub const COPY_BUFFER_TO_IMAGE_INFO_2_KHR: Self = Self::COPY_BUFFER_TO_IMAGE_INFO_2;
    pub const COPY_IMAGE_TO_BUFFER_INFO_2_KHR: Self = Self::COPY_IMAGE_TO_BUFFER_INFO_2;
    pub const BLIT_IMAGE_INFO_2_KHR: Self = Self::BLIT_IMAGE_INFO_2;
    pub const RESOLVE_IMAGE_INFO_2_KHR: Self = Self::RESOLVE_IMAGE_INFO_2;
    pub const BUFFER_COPY_2_KHR: Self = Self::BUFFER_COPY_2;
    pub const IMAGE_COPY_2_KHR: Self = Self::IMAGE_COPY_2;
    pub const IMAGE_BLIT_2_KHR: Self = Self::IMAGE_BLIT_2;
    pub const BUFFER_IMAGE_COPY_2_KHR: Self = Self::BUFFER_IMAGE_COPY_2;
    pub const IMAGE_RESOLVE_2_KHR: Self = Self::IMAGE_RESOLVE_2;
    pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT: Self = Self(1000338000);
    pub const IMAGE_COMPRESSION_CONTROL_EXT: Self = Self(1000338001);
    pub const SUBRESOURCE_LAYOUT_2_EXT: Self = Self::SUBRESOURCE_LAYOUT_2;
    pub const IMAGE_SUBRESOURCE_2_EXT: Self = Self::IMAGE_SUBRESOURCE_2;
    pub const IMAGE_COMPRESSION_PROPERTIES_EXT: Self = Self(1000338004);
    pub const PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT: Self = Self(1000339000);
    pub const PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT: Self = Self(1000340000);
    pub const PHYSICAL_DEVICE_FAULT_FEATURES_EXT: Self = Self(1000341000);
    pub const DEVICE_FAULT_COUNTS_EXT: Self = Self(1000341001);
    pub const DEVICE_FAULT_INFO_EXT: Self = Self(1000341002);
    pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM: Self =
        Self::PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT;
    pub const PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT: Self = Self(1000344000);
    pub const DIRECTFB_SURFACE_CREATE_INFO_EXT: Self = Self(1000346000);
    pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE: Self =
        Self::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT;
    pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE: Self =
        Self::MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT;
    pub const PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1000352000);
    pub const VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT: Self = Self(1000352001);
    pub const VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT: Self = Self(1000352002);
    pub const PHYSICAL_DEVICE_DRM_PROPERTIES_EXT: Self = Self(1000353000);
    pub const PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT: Self = Self(1000354000);
    pub const DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT: Self = Self(1000354001);
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT: Self = Self(1000355000);
    pub const PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT: Self = Self(1000355001);
    pub const PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT: Self = Self(1000356000);
    pub const FORMAT_PROPERTIES_3_KHR: Self = Self::FORMAT_PROPERTIES_3;
    pub const PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR;
    pub const IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364000);
    pub const MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA: Self = Self(1000364001);
    pub const MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364002);
    pub const IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365000);
    pub const SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365001);
    pub const BUFFER_COLLECTION_CREATE_INFO_FUCHSIA: Self = Self(1000366000);
    pub const IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366001);
    pub const BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA: Self = Self(1000366002);
    pub const BUFFER_COLLECTION_PROPERTIES_FUCHSIA: Self = Self(1000366003);
    pub const BUFFER_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366004);
    pub const BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA: Self = Self(1000366005);
    pub const IMAGE_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366006);
    pub const IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366007);
    pub const SYSMEM_COLOR_SPACE_FUCHSIA: Self = Self(1000366008);
    pub const BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366009);
    pub const SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI: Self = Self(1000369000);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI: Self = Self(1000369001);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI: Self = Self(1000369002);
    pub const PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI: Self = Self(1000370000);
    pub const MEMORY_GET_REMOTE_ADDRESS_INFO_NV: Self = Self(1000371000);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV: Self = Self(1000371001);
    pub const PIPELINE_PROPERTIES_IDENTIFIER_EXT: Self = Self(1000372000);
    pub const PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT: Self = Self(1000372001);
    pub const PIPELINE_INFO_EXT: Self = Self::PIPELINE_INFO_KHR;
    pub const IMPORT_FENCE_SCI_SYNC_INFO_NV: Self = Self(1000373000);
    pub const EXPORT_FENCE_SCI_SYNC_INFO_NV: Self = Self(1000373001);
    pub const FENCE_GET_SCI_SYNC_INFO_NV: Self = Self(1000373002);
    pub const SCI_SYNC_ATTRIBUTES_INFO_NV: Self = Self(1000373003);
    pub const IMPORT_SEMAPHORE_SCI_SYNC_INFO_NV: Self = Self(1000373004);
    pub const EXPORT_SEMAPHORE_SCI_SYNC_INFO_NV: Self = Self(1000373005);
    pub const SEMAPHORE_GET_SCI_SYNC_INFO_NV: Self = Self(1000373006);
    pub const PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_FEATURES_NV: Self = Self(1000373007);
    pub const IMPORT_MEMORY_SCI_BUF_INFO_NV: Self = Self(1000374000);
    pub const EXPORT_MEMORY_SCI_BUF_INFO_NV: Self = Self(1000374001);
    pub const MEMORY_GET_SCI_BUF_INFO_NV: Self = Self(1000374002);
    pub const MEMORY_SCI_BUF_PROPERTIES_NV: Self = Self(1000374003);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV: Self = Self(1000374004);
    pub const PHYSICAL_DEVICE_EXTERNAL_SCI_BUF_FEATURES_NV: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV;
    pub const PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT: Self = Self(1000375000);
    pub const FRAME_BOUNDARY_EXT: Self = Self(1000375001);
    pub const PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT: Self =
        Self(1000376000);
    pub const SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT: Self = Self(1000376001);
    pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT: Self = Self(1000376002);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT: Self = Self(1000377000);
    pub const SCREEN_SURFACE_CREATE_INFO_QNX: Self = Self(1000378000);
    pub const PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT: Self = Self(1000381000);
    pub const PIPELINE_COLOR_WRITE_CREATE_INFO_EXT: Self = Self(1000381001);
    pub const PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT: Self = Self(1000382000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000386000);
    pub const PHYSICAL_DEVICE_SHADER_UNTYPED_POINTERS_FEATURES_KHR: Self = Self(1000387000);
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES;
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT: Self =
        Self::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES;
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE: Self = Self(1000390000);
    pub const VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE: Self = Self(1000390001);
    pub const VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE: Self = Self(1000390002);
    pub const VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE: Self = Self(1000390003);
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT: Self = Self(1000391000);
    pub const IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT: Self = Self(1000391001);
    pub const PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT: Self = Self(1000392000);
    pub const PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT: Self = Self(1000392001);
    pub const PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT: Self = Self(1000393000);
    pub const PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT: Self = Self(1000395000);
    pub const PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT: Self = Self(1000395001);
    pub const MICROMAP_BUILD_INFO_EXT: Self = Self(1000396000);
    pub const MICROMAP_VERSION_INFO_EXT: Self = Self(1000396001);
    pub const COPY_MICROMAP_INFO_EXT: Self = Self(1000396002);
    pub const COPY_MICROMAP_TO_MEMORY_INFO_EXT: Self = Self(1000396003);
    pub const COPY_MEMORY_TO_MICROMAP_INFO_EXT: Self = Self(1000396004);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT: Self = Self(1000396005);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT: Self = Self(1000396006);
    pub const MICROMAP_CREATE_INFO_EXT: Self = Self(1000396007);
    pub const MICROMAP_BUILD_SIZES_INFO_EXT: Self = Self(1000396008);
    pub const ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT: Self = Self(1000396009);
    pub const PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV: Self = Self(1000397000);
    pub const PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV: Self = Self(1000397001);
    pub const ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV: Self = Self(1000397002);
    pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI: Self = Self(1000404000);
    pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI: Self = Self(1000404001);
    pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI: Self = Self(1000404002);
    pub const PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT: Self = Self(1000411000);
    pub const SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT: Self = Self(1000411001);
    pub const PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT: Self = Self(1000412000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES;
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES;
    pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR: Self = Self::DEVICE_BUFFER_MEMORY_REQUIREMENTS;
    pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR: Self = Self::DEVICE_IMAGE_MEMORY_REQUIREMENTS;
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM: Self = Self(1000415000);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES;
    pub const DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM: Self = Self(1000417000);
    pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM: Self = Self(1000417001);
    pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM: Self = Self(1000417002);
    pub const DISPATCH_PARAMETERS_ARM: Self = Self(1000417003);
    pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_PROPERTIES_ARM: Self =
        Self(1000417004);
    pub const PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT: Self = Self(1000418000);
    pub const IMAGE_VIEW_SLICED_CREATE_INFO_EXT: Self = Self(1000418001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE: Self = Self(1000420000);
    pub const DESCRIPTOR_SET_BINDING_REFERENCE_VALVE: Self = Self(1000420001);
    pub const DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE: Self = Self(1000420002);
    pub const PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR;
    pub const PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT: Self = Self(1000422000);
    pub const PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM: Self = Self(1000424000);
    pub const PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM: Self = Self(1000424001);
    pub const RENDER_PASS_STRIPE_BEGIN_INFO_ARM: Self = Self(1000424002);
    pub const RENDER_PASS_STRIPE_INFO_ARM: Self = Self(1000424003);
    pub const RENDER_PASS_STRIPE_SUBMIT_INFO_ARM: Self = Self(1000424004);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM: Self =
        Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT;
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM: Self =
        Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT;
    pub const SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM: Self =
        Self::RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT;
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV: Self = Self(1000426000);
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV: Self =
        Self::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR;
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV: Self =
        Self::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT;
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV: Self =
        Self::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT;
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV: Self =
        Self(1000428000);
    pub const COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV: Self = Self(1000428001);
    pub const PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV: Self = Self(1000428002);
    pub const PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV: Self = Self(1000429008);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV: Self = Self(1000429009);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV: Self = Self(1000429010);
    pub const PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV: Self = Self(1000430000);
    pub const PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR: Self = Self(1000434000);
    pub const APPLICATION_PARAMETERS_EXT: Self = Self(1000435000);
    pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT: Self =
        Self(1000437000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM: Self = Self(1000440000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM: Self = Self(1000440001);
    pub const IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM: Self = Self(1000440002);
    pub const PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT: Self = Self(1000451000);
    pub const PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT: Self = Self(1000451001);
    pub const NATIVE_BUFFER_USAGE_OHOS: Self = Self(1000452000);
    pub const NATIVE_BUFFER_PROPERTIES_OHOS: Self = Self(1000452001);
    pub const NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS: Self = Self(1000452002);
    pub const IMPORT_NATIVE_BUFFER_INFO_OHOS: Self = Self(1000452003);
    pub const MEMORY_GET_NATIVE_BUFFER_INFO_OHOS: Self = Self(1000452004);
    pub const EXTERNAL_FORMAT_OHOS: Self = Self(1000452005);
    pub const EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT: Self = Self(1000453000);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT: Self = Self(1000455000);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT: Self = Self(1000455001);
    pub const PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT: Self = Self(1000458000);
    pub const RENDER_PASS_CREATION_CONTROL_EXT: Self = Self(1000458001);
    pub const RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000458002);
    pub const RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000458003);
    pub const DIRECT_DRIVER_LOADING_INFO_LUNARG: Self = Self(1000459000);
    pub const DIRECT_DRIVER_LOADING_LIST_LUNARG: Self = Self(1000459001);
    pub const TENSOR_CREATE_INFO_ARM: Self = Self(1000460000);
    pub const TENSOR_VIEW_CREATE_INFO_ARM: Self = Self(1000460001);
    pub const BIND_TENSOR_MEMORY_INFO_ARM: Self = Self(1000460002);
    pub const WRITE_DESCRIPTOR_SET_TENSOR_ARM: Self = Self(1000460003);
    pub const PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM: Self = Self(1000460004);
    pub const TENSOR_FORMAT_PROPERTIES_ARM: Self = Self(1000460005);
    pub const TENSOR_DESCRIPTION_ARM: Self = Self(1000460006);
    pub const TENSOR_MEMORY_REQUIREMENTS_INFO_ARM: Self = Self(1000460007);
    pub const TENSOR_MEMORY_BARRIER_ARM: Self = Self(1000460008);
    pub const PHYSICAL_DEVICE_TENSOR_FEATURES_ARM: Self = Self(1000460009);
    pub const DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM: Self = Self(1000460010);
    pub const COPY_TENSOR_INFO_ARM: Self = Self(1000460011);
    pub const TENSOR_COPY_ARM: Self = Self(1000460012);
    pub const TENSOR_DEPENDENCY_INFO_ARM: Self = Self(1000460013);
    pub const MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM: Self = Self(1000460014);
    pub const PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM: Self = Self(1000460015);
    pub const EXTERNAL_TENSOR_PROPERTIES_ARM: Self = Self(1000460016);
    pub const EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM: Self = Self(1000460017);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM: Self = Self(1000460018);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM: Self = Self(1000460019);
    pub const DESCRIPTOR_GET_TENSOR_INFO_ARM: Self = Self(1000460020);
    pub const TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM: Self = Self(1000460021);
    pub const TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM: Self = Self(1000460022);
    pub const FRAME_BOUNDARY_TENSORS_ARM: Self = Self(1000460023);
    pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT: Self = Self(1000462000);
    pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT: Self = Self(1000462001);
    pub const PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT: Self = Self(1000462002);
    pub const SHADER_MODULE_IDENTIFIER_EXT: Self = Self(1000462003);
    pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT: Self =
        Self(1000342000);
    pub const PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV: Self = Self(1000464000);
    pub const PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV: Self = Self(1000464001);
    pub const OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV: Self = Self(1000464002);
    pub const OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV: Self = Self(1000464003);
    pub const OPTICAL_FLOW_SESSION_CREATE_INFO_NV: Self = Self(1000464004);
    pub const OPTICAL_FLOW_EXECUTE_INFO_NV: Self = Self(1000464005);
    pub const OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV: Self = Self(1000464010);
    pub const PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT: Self = Self(1000465000);
    pub const PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES;
    pub const PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID: Self = Self(1000468000);
    pub const PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID: Self = Self(1000468001);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID: Self = Self(1000468002);
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES;
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES;
    pub const RENDERING_AREA_INFO_KHR: Self = Self::RENDERING_AREA_INFO;
    pub const DEVICE_IMAGE_SUBRESOURCE_INFO_KHR: Self = Self::DEVICE_IMAGE_SUBRESOURCE_INFO;
    pub const SUBRESOURCE_LAYOUT_2_KHR: Self = Self::SUBRESOURCE_LAYOUT_2;
    pub const IMAGE_SUBRESOURCE_2_KHR: Self = Self::IMAGE_SUBRESOURCE_2;
    pub const PIPELINE_CREATE_FLAGS_2_CREATE_INFO_KHR: Self =
        Self::PIPELINE_CREATE_FLAGS_2_CREATE_INFO;
    pub const BUFFER_USAGE_FLAGS_2_CREATE_INFO_KHR: Self = Self::BUFFER_USAGE_FLAGS_2_CREATE_INFO;
    pub const PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD: Self = Self(1000476000);
    pub const ANTI_LAG_DATA_AMD: Self = Self(1000476001);
    pub const ANTI_LAG_PRESENTATION_INFO_AMD: Self = Self(1000476002);
    pub const PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX: Self = Self(1000478000);
    pub const ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX: Self =
        Self(1000478001);
    pub const SURFACE_CAPABILITIES_PRESENT_ID_2_KHR: Self = Self(1000479000);
    pub const PRESENT_ID_2_KHR: Self = Self(1000479001);
    pub const PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR: Self = Self(1000479002);
    pub const SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR: Self = Self(1000480000);
    pub const PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR: Self = Self(1000480001);
    pub const PRESENT_WAIT_2_INFO_KHR: Self = Self(1000480002);
    pub const PHYSICAL_DEVICE_RAY_TRACING_POSITION_FETCH_FEATURES_KHR: Self = Self(1000481000);
    pub const PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT: Self = Self(1000482000);
    pub const PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT: Self = Self(1000482001);
    pub const SHADER_CREATE_INFO_EXT: Self = Self(1000482002);
    pub const SHADER_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self =
        Self::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR: Self = Self(1000483000);
    pub const PIPELINE_BINARY_CREATE_INFO_KHR: Self = Self(1000483001);
    pub const PIPELINE_BINARY_INFO_KHR: Self = Self(1000483002);
    pub const PIPELINE_BINARY_KEY_KHR: Self = Self(1000483003);
    pub const PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR: Self = Self(1000483004);
    pub const RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR: Self = Self(1000483005);
    pub const PIPELINE_BINARY_DATA_INFO_KHR: Self = Self(1000483006);
    pub const PIPELINE_CREATE_INFO_KHR: Self = Self(1000483007);
    pub const DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR: Self = Self(1000483008);
    pub const PIPELINE_BINARY_HANDLES_INFO_KHR: Self = Self(1000483009);
    pub const PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM: Self = Self(1000484000);
    pub const TILE_PROPERTIES_QCOM: Self = Self(1000484001);
    pub const PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC: Self = Self(1000485000);
    pub const AMIGO_PROFILING_SUBMIT_INFO_SEC: Self = Self(1000485001);
    pub const SURFACE_PRESENT_MODE_KHR: Self = Self(1000274000);
    pub const SURFACE_PRESENT_SCALING_CAPABILITIES_KHR: Self = Self(1000274001);
    pub const SURFACE_PRESENT_MODE_COMPATIBILITY_KHR: Self = Self(1000274002);
    pub const PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000275000);
    pub const SWAPCHAIN_PRESENT_FENCE_INFO_KHR: Self = Self(1000275001);
    pub const SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR: Self = Self(1000275002);
    pub const SWAPCHAIN_PRESENT_MODE_INFO_KHR: Self = Self(1000275003);
    pub const SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR: Self = Self(1000275004);
    pub const RELEASE_SWAPCHAIN_IMAGES_INFO_KHR: Self = Self(1000275005);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM: Self = Self(1000488000);
    pub const SEMAPHORE_SCI_SYNC_POOL_CREATE_INFO_NV: Self = Self(1000489000);
    pub const SEMAPHORE_SCI_SYNC_CREATE_INFO_NV: Self = Self(1000489001);
    pub const PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_2_FEATURES_NV: Self = Self(1000489002);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV: Self = Self(1000490000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV: Self = Self(1000490001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV: Self = Self(1000491000);
    pub const PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV: Self = Self(1000491001);
    pub const COOPERATIVE_VECTOR_PROPERTIES_NV: Self = Self(1000491002);
    pub const CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV: Self = Self(1000491004);
    pub const PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV: Self = Self(1000492000);
    pub const PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV: Self = Self(1000492001);
    pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT: Self = Self(1000351000);
    pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT: Self = Self(1000351002);
    pub const PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_FEATURES_EXT: Self = Self(1000495000);
    pub const PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_PROPERTIES_EXT: Self = Self(1000495001);
    pub const LAYER_SETTINGS_CREATE_INFO_EXT: Self = Self(1000496000);
    pub const PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM: Self = Self(1000497000);
    pub const PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM: Self = Self(1000497001);
    pub const PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT: Self = Self(1000498000);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT: Self =
        Self(1000499000);
    pub const PHYSICAL_DEVICE_INTERNALLY_SYNCHRONIZED_QUEUES_FEATURES_KHR: Self = Self(1000504000);
    pub const LATENCY_SLEEP_MODE_INFO_NV: Self = Self(1000505000);
    pub const LATENCY_SLEEP_INFO_NV: Self = Self(1000505001);
    pub const SET_LATENCY_MARKER_INFO_NV: Self = Self(1000505002);
    pub const GET_LATENCY_MARKER_INFO_NV: Self = Self(1000505003);
    pub const LATENCY_TIMINGS_FRAME_REPORT_NV: Self = Self(1000505004);
    pub const LATENCY_SUBMISSION_PRESENT_ID_NV: Self = Self(1000505005);
    pub const OUT_OF_BAND_QUEUE_TYPE_INFO_NV: Self = Self(1000505006);
    pub const SWAPCHAIN_LATENCY_CREATE_INFO_NV: Self = Self(1000505007);
    pub const LATENCY_SURFACE_CAPABILITIES_NV: Self = Self(1000505008);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR: Self = Self(1000506000);
    pub const COOPERATIVE_MATRIX_PROPERTIES_KHR: Self = Self(1000506001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR: Self = Self(1000506002);
    pub const DATA_GRAPH_PIPELINE_CREATE_INFO_ARM: Self = Self(1000507000);
    pub const DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM: Self = Self(1000507001);
    pub const DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM: Self = Self(1000507002);
    pub const DATA_GRAPH_PIPELINE_CONSTANT_ARM: Self = Self(1000507003);
    pub const DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM: Self = Self(1000507004);
    pub const BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM: Self = Self(1000507005);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM: Self = Self(1000507006);
    pub const DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM: Self = Self(1000507007);
    pub const DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM: Self = Self(1000507008);
    pub const DATA_GRAPH_PIPELINE_INFO_ARM: Self = Self(1000507009);
    pub const DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM: Self = Self(1000507010);
    pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM: Self = Self(1000507011);
    pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM: Self = Self(1000507012);
    pub const DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM: Self = Self(1000507013);
    pub const DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM: Self = Self(1000507014);
    pub const DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM: Self = Self(1000507016);
    pub const QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM: Self = Self(1000507017);
    pub const QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM: Self = Self(1000507018);
    pub const PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM: Self =
        Self(1000507019);
    pub const DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM: Self =
        Self(1000507015);
    pub const QUEUE_FAMILY_DATA_GRAPH_TOSA_PROPERTIES_ARM: Self = Self(1000508000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM: Self =
        Self(1000510000);
    pub const MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM: Self = Self(1000510001);
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR: Self = Self(1000201000);
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_PROPERTIES_KHR: Self = Self(1000511000);
    pub const VIDEO_DECODE_AV1_CAPABILITIES_KHR: Self = Self(1000512000);
    pub const VIDEO_DECODE_AV1_PICTURE_INFO_KHR: Self = Self(1000512001);
    pub const VIDEO_DECODE_AV1_PROFILE_INFO_KHR: Self = Self(1000512003);
    pub const VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000512004);
    pub const VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR: Self = Self(1000512005);
    pub const VIDEO_ENCODE_AV1_CAPABILITIES_KHR: Self = Self(1000513000);
    pub const VIDEO_ENCODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000513001);
    pub const VIDEO_ENCODE_AV1_PICTURE_INFO_KHR: Self = Self(1000513002);
    pub const VIDEO_ENCODE_AV1_DPB_SLOT_INFO_KHR: Self = Self(1000513003);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_AV1_FEATURES_KHR: Self = Self(1000513004);
    pub const VIDEO_ENCODE_AV1_PROFILE_INFO_KHR: Self = Self(1000513005);
    pub const VIDEO_ENCODE_AV1_RATE_CONTROL_INFO_KHR: Self = Self(1000513006);
    pub const VIDEO_ENCODE_AV1_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000513007);
    pub const VIDEO_ENCODE_AV1_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000513008);
    pub const VIDEO_ENCODE_AV1_SESSION_CREATE_INFO_KHR: Self = Self(1000513009);
    pub const VIDEO_ENCODE_AV1_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000513010);
    pub const PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR: Self = Self(1000514000);
    pub const VIDEO_DECODE_VP9_CAPABILITIES_KHR: Self = Self(1000514001);
    pub const VIDEO_DECODE_VP9_PICTURE_INFO_KHR: Self = Self(1000514002);
    pub const VIDEO_DECODE_VP9_PROFILE_INFO_KHR: Self = Self(1000514003);
    pub const PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000515000);
    pub const VIDEO_INLINE_QUERY_INFO_KHR: Self = Self(1000515001);
    pub const PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV: Self = Self(1000516000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM: Self = Self(1000518000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM: Self = Self(1000518001);
    pub const SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM: Self = Self(1000518002);
    pub const SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM: Self = Self(1000519000);
    pub const PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM: Self = Self(1000519001);
    pub const BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM: Self = Self(1000519002);
    pub const PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM: Self = Self(1000520000);
    pub const SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM: Self = Self(1000520001);
    pub const PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM: Self = Self(1000521000);
    pub const PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT: Self =
        Self(1000524000);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES;
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR: Self =
        Self::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES;
    pub const PHYSICAL_DEVICE_UNIFIED_IMAGE_LAYOUTS_FEATURES_KHR: Self = Self(1000527000);
    pub const ATTACHMENT_FEEDBACK_LOOP_INFO_EXT: Self = Self(1000527001);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES;
    pub const SCREEN_BUFFER_PROPERTIES_QNX: Self = Self(1000529000);
    pub const SCREEN_BUFFER_FORMAT_PROPERTIES_QNX: Self = Self(1000529001);
    pub const IMPORT_SCREEN_BUFFER_INFO_QNX: Self = Self(1000529002);
    pub const EXTERNAL_FORMAT_QNX: Self = Self(1000529003);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX: Self = Self(1000529004);
    pub const PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT: Self = Self(1000530000);
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES;
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES;
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR: Self =
        Self::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES;
    pub const CALIBRATED_TIMESTAMP_INFO_KHR: Self = Self(1000184000);
    pub const PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES;
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES;
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES;
    pub const BIND_MEMORY_STATUS_KHR: Self = Self::BIND_MEMORY_STATUS;
    pub const BIND_DESCRIPTOR_SETS_INFO_KHR: Self = Self::BIND_DESCRIPTOR_SETS_INFO;
    pub const PUSH_CONSTANTS_INFO_KHR: Self = Self::PUSH_CONSTANTS_INFO;
    pub const PUSH_DESCRIPTOR_SET_INFO_KHR: Self = Self::PUSH_DESCRIPTOR_SET_INFO;
    pub const PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO_KHR: Self =
        Self::PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO;
    pub const SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT: Self = Self(1000545007);
    pub const BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT: Self = Self(1000545008);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV: Self = Self(1000546000);
    pub const PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM: Self = Self(1000547000);
    pub const PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM: Self = Self(1000547001);
    pub const TILE_MEMORY_REQUIREMENTS_QCOM: Self = Self(1000547002);
    pub const TILE_MEMORY_BIND_INFO_QCOM: Self = Self(1000547003);
    pub const TILE_MEMORY_SIZE_INFO_QCOM: Self = Self(1000547004);
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_KHR: Self = Self(1000549000);
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR: Self = Self(1000426001);
    pub const COPY_MEMORY_INDIRECT_INFO_KHR: Self = Self(1000549002);
    pub const COPY_MEMORY_TO_IMAGE_INDIRECT_INFO_KHR: Self = Self(1000549003);
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT: Self = Self(1000427000);
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT: Self = Self(1000427001);
    pub const DECOMPRESS_MEMORY_INFO_EXT: Self = Self(1000550002);
    pub const DISPLAY_SURFACE_STEREO_CREATE_INFO_NV: Self = Self(1000551000);
    pub const DISPLAY_MODE_STEREO_PROPERTIES_NV: Self = Self(1000551001);
    pub const VIDEO_ENCODE_INTRA_REFRESH_CAPABILITIES_KHR: Self = Self(1000552000);
    pub const VIDEO_ENCODE_SESSION_INTRA_REFRESH_CREATE_INFO_KHR: Self = Self(1000552001);
    pub const VIDEO_ENCODE_INTRA_REFRESH_INFO_KHR: Self = Self(1000552002);
    pub const VIDEO_REFERENCE_INTRA_REFRESH_INFO_KHR: Self = Self(1000552003);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_INTRA_REFRESH_FEATURES_KHR: Self = Self(1000552004);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553000);
    pub const VIDEO_FORMAT_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553001);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_INFO_KHR: Self = Self(1000553002);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_SESSION_PARAMETERS_CREATE_INFO_KHR: Self =
        Self(1000553005);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_QUANTIZATION_MAP_FEATURES_KHR: Self = Self(1000553009);
    pub const VIDEO_ENCODE_H264_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553003);
    pub const VIDEO_ENCODE_H265_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553004);
    pub const VIDEO_FORMAT_H265_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553006);
    pub const VIDEO_ENCODE_AV1_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553007);
    pub const VIDEO_FORMAT_AV1_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553008);
    pub const PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV: Self = Self(1000555000);
    pub const EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV: Self = Self(1000556000);
    pub const EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV: Self = Self(1000556001);
    pub const EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV: Self = Self(1000556002);
    pub const PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV: Self = Self(1000556003);
    pub const PHYSICAL_DEVICE_SHADER_RELAXED_EXTENDED_INSTRUCTION_FEATURES_KHR: Self =
        Self(1000558000);
    pub const PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV: Self = Self(1000559000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR: Self = Self(1000562000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR: Self = Self(1000562001);
    pub const PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR: Self = Self(1000562002);
    pub const PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR: Self = Self(1000562003);
    pub const PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR: Self = Self(1000562004);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV: Self = Self(1000563000);
    pub const PHYSICAL_DEVICE_SHADER_REPLICATED_COMPOSITES_FEATURES_EXT: Self = Self(1000564000);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT: Self = Self(1000567000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV: Self = Self(1000568000);
    pub const PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV: Self = Self(1000569000);
    pub const PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV: Self = Self(1000569001);
    pub const CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV: Self =
        Self(1000569002);
    pub const CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV: Self = Self(1000569003);
    pub const CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV: Self = Self(1000569004);
    pub const CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV: Self = Self(1000569005);
    pub const CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV: Self = Self(1000569006);
    pub const RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV: Self =
        Self(1000569007);
    pub const PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV: Self =
        Self(1000570000);
    pub const PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV: Self =
        Self(1000570001);
    pub const WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV: Self = Self(1000570002);
    pub const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV: Self = Self(1000570003);
    pub const BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV: Self = Self(1000570004);
    pub const PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV: Self = Self(1000570005);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_EXT: Self = Self(1000572000);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_EXT: Self = Self(1000572001);
    pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_EXT: Self = Self(1000572002);
    pub const INDIRECT_EXECUTION_SET_CREATE_INFO_EXT: Self = Self(1000572003);
    pub const GENERATED_COMMANDS_INFO_EXT: Self = Self(1000572004);
    pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_EXT: Self = Self(1000572006);
    pub const INDIRECT_COMMANDS_LAYOUT_TOKEN_EXT: Self = Self(1000572007);
    pub const WRITE_INDIRECT_EXECUTION_SET_PIPELINE_EXT: Self = Self(1000572008);
    pub const WRITE_INDIRECT_EXECUTION_SET_SHADER_EXT: Self = Self(1000572009);
    pub const INDIRECT_EXECUTION_SET_PIPELINE_INFO_EXT: Self = Self(1000572010);
    pub const INDIRECT_EXECUTION_SET_SHADER_INFO_EXT: Self = Self(1000572011);
    pub const INDIRECT_EXECUTION_SET_SHADER_LAYOUT_INFO_EXT: Self = Self(1000572012);
    pub const GENERATED_COMMANDS_PIPELINE_INFO_EXT: Self = Self(1000572013);
    pub const GENERATED_COMMANDS_SHADER_INFO_EXT: Self = Self(1000572014);
    pub const PHYSICAL_DEVICE_FAULT_FEATURES_KHR: Self = Self(1000573000);
    pub const PHYSICAL_DEVICE_FAULT_PROPERTIES_KHR: Self = Self(1000573001);
    pub const DEVICE_FAULT_INFO_KHR: Self = Self(1000573002);
    pub const DEVICE_FAULT_DEBUG_INFO_KHR: Self = Self(1000573003);
    pub const PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR: Self = Self(1000574000);
    pub const MEMORY_BARRIER_ACCESS_FLAGS_3_KHR: Self = Self(1000574002);
    pub const PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA: Self = Self(1000575000);
    pub const PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA: Self = Self(1000575001);
    pub const IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA: Self = Self(1000575002);
    pub const PHYSICAL_DEVICE_SHADER_FMA_FEATURES_KHR: Self = Self(1000579000);
    pub const PUSH_CONSTANT_BANK_INFO_NV: Self = Self(1000580000);
    pub const PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV: Self = Self(1000580001);
    pub const PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV: Self = Self(1000580002);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_EXT: Self = Self(1000581000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_EXT: Self =
        Self(1000581001);
    pub const PHYSICAL_DEVICE_DEPTH_CLAMP_CONTROL_FEATURES_EXT: Self = Self(1000582000);
    pub const PIPELINE_VIEWPORT_DEPTH_CLAMP_CONTROL_CREATE_INFO_EXT: Self = Self(1000582001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR: Self = Self(1000584000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR: Self = Self(1000584001);
    pub const QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR: Self = Self(1000584002);
    pub const PHYSICAL_DEVICE_VIDEO_MAINTENANCE_2_FEATURES_KHR: Self = Self(1000586000);
    pub const VIDEO_DECODE_H264_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586001);
    pub const VIDEO_DECODE_H265_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586002);
    pub const VIDEO_DECODE_AV1_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586003);
    pub const SURFACE_CREATE_INFO_OHOS: Self = Self(1000685000);
    pub const PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI: Self = Self(1000590000);
    pub const HDR_VIVID_DYNAMIC_METADATA_HUAWEI: Self = Self(1000590001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV: Self = Self(1000593000);
    pub const COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV: Self = Self(1000593001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV: Self = Self(1000593002);
    pub const PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM: Self = Self(1000596000);
    pub const IMPORT_MEMORY_METAL_HANDLE_INFO_EXT: Self = Self(1000602000);
    pub const MEMORY_METAL_HANDLE_PROPERTIES_EXT: Self = Self(1000602001);
    pub const MEMORY_GET_METAL_HANDLE_INFO_EXT: Self = Self(1000602002);
    pub const PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR: Self = Self(1000421000);
    pub const PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM: Self = Self(1000605000);
    pub const PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM: Self =
        Self(1000605001);
    pub const PERFORMANCE_COUNTER_ARM: Self = Self(1000605002);
    pub const PERFORMANCE_COUNTER_DESCRIPTION_ARM: Self = Self(1000605003);
    pub const RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM: Self = Self(1000605004);
    pub const PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_FEATURES_ARM: Self = Self(1000607000);
    pub const PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_PROPERTIES_ARM: Self = Self(1000607001);
    pub const SHADER_INSTRUMENTATION_CREATE_INFO_ARM: Self = Self(1000607002);
    pub const SHADER_INSTRUMENTATION_METRIC_DESCRIPTION_ARM: Self = Self(1000607003);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_ROBUSTNESS_FEATURES_EXT: Self = Self(1000608000);
    pub const PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM: Self = Self(1000609000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE: Self = Self(1000611000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE: Self =
        Self(1000611001);
    pub const PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE: Self = Self(1000611002);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR: Self = Self(1000286000);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR: Self = Self(1000286001);
    pub const SET_PRESENT_CONFIG_NV: Self = Self(1000613000);
    pub const PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV: Self = Self(1000613001);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT: Self = Self(1000425000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT: Self = Self(1000425001);
    pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT: Self = Self(1000425002);
    pub const RENDERING_END_INFO_EXT: Self = Self::RENDERING_END_INFO_KHR;
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_DEVICE_MEMORY_FEATURES_EXT: Self = Self(1000620000);
    pub const PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR: Self = Self(1000361000);
    pub const PHYSICAL_DEVICE_SHADER_64_BIT_INDEXING_FEATURES_EXT: Self = Self(1000627000);
    pub const PHYSICAL_DEVICE_CUSTOM_RESOLVE_FEATURES_EXT: Self = Self(1000628000);
    pub const BEGIN_CUSTOM_RESOLVE_INFO_EXT: Self = Self(1000628001);
    pub const CUSTOM_RESOLVE_CREATE_INFO_EXT: Self = Self(1000628002);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM: Self = Self(1000629000);
    pub const DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM: Self = Self(1000629001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_10_FEATURES_KHR: Self = Self(1000630000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_10_PROPERTIES_KHR: Self = Self(1000630001);
    pub const RENDERING_ATTACHMENT_FLAGS_INFO_KHR: Self = Self(1000630002);
    pub const RENDERING_END_INFO_KHR: Self = Self(1000619003);
    pub const RESOLVE_IMAGE_MODE_INFO_KHR: Self = Self(1000630004);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_OPTICAL_FLOW_FEATURES_ARM: Self = Self(1000631000);
    pub const QUEUE_FAMILY_DATA_GRAPH_OPTICAL_FLOW_PROPERTIES_ARM: Self = Self(1000631001);
    pub const DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_INFO_ARM: Self = Self(1000631003);
    pub const DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_ARM: Self = Self(1000631004);
    pub const DATA_GRAPH_PIPELINE_OPTICAL_FLOW_DISPATCH_INFO_ARM: Self = Self(1000631005);
    pub const DATA_GRAPH_PIPELINE_OPTICAL_FLOW_CREATE_INFO_ARM: Self = Self(1000631002);
    pub const DATA_GRAPH_PIPELINE_RESOURCE_INFO_IMAGE_LAYOUT_ARM: Self = Self(1000631006);
    pub const DATA_GRAPH_PIPELINE_SINGLE_NODE_CREATE_INFO_ARM: Self = Self(1000631007);
    pub const DATA_GRAPH_PIPELINE_SINGLE_NODE_CONNECTION_ARM: Self = Self(1000631008);
    pub const PHYSICAL_DEVICE_SHADER_LONG_VECTOR_FEATURES_EXT: Self = Self(1000635000);
    pub const PHYSICAL_DEVICE_SHADER_LONG_VECTOR_PROPERTIES_EXT: Self = Self(1000635001);
    pub const PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC: Self = Self(1000637000);
    pub const PHYSICAL_DEVICE_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_FEATURES_EXT: Self =
        Self(1000642000);
    pub const COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV: Self = Self(1000645000);
    pub const PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV: Self = Self(1000645001);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_PARTITIONED_FEATURES_EXT: Self = Self(1000662000);
    pub const UBM_SURFACE_CREATE_INFO_SEC: Self = Self(1000664000);
    pub const PHYSICAL_DEVICE_SHADER_MIXED_FLOAT_DOT_PRODUCT_FEATURES_VALVE: Self =
        Self(1000673000);
    pub const PHYSICAL_DEVICE_PRIMITIVE_RESTART_INDEX_FEATURES_EXT: Self = Self(1000678000);
}
impl fmt::Display for StructureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::APPLICATION_INFO => write!(f, "APPLICATION_INFO"),
            Self::INSTANCE_CREATE_INFO => write!(f, "INSTANCE_CREATE_INFO"),
            Self::DEVICE_QUEUE_CREATE_INFO => write!(f, "DEVICE_QUEUE_CREATE_INFO"),
            Self::DEVICE_CREATE_INFO => write!(f, "DEVICE_CREATE_INFO"),
            Self::SUBMIT_INFO => write!(f, "SUBMIT_INFO"),
            Self::MEMORY_ALLOCATE_INFO => write!(f, "MEMORY_ALLOCATE_INFO"),
            Self::MAPPED_MEMORY_RANGE => write!(f, "MAPPED_MEMORY_RANGE"),
            Self::BIND_SPARSE_INFO => write!(f, "BIND_SPARSE_INFO"),
            Self::FENCE_CREATE_INFO => write!(f, "FENCE_CREATE_INFO"),
            Self::SEMAPHORE_CREATE_INFO => write!(f, "SEMAPHORE_CREATE_INFO"),
            Self::EVENT_CREATE_INFO => write!(f, "EVENT_CREATE_INFO"),
            Self::QUERY_POOL_CREATE_INFO => write!(f, "QUERY_POOL_CREATE_INFO"),
            Self::BUFFER_CREATE_INFO => write!(f, "BUFFER_CREATE_INFO"),
            Self::BUFFER_VIEW_CREATE_INFO => write!(f, "BUFFER_VIEW_CREATE_INFO"),
            Self::IMAGE_CREATE_INFO => write!(f, "IMAGE_CREATE_INFO"),
            Self::IMAGE_VIEW_CREATE_INFO => write!(f, "IMAGE_VIEW_CREATE_INFO"),
            Self::SHADER_MODULE_CREATE_INFO => write!(f, "SHADER_MODULE_CREATE_INFO"),
            Self::PIPELINE_CACHE_CREATE_INFO => write!(f, "PIPELINE_CACHE_CREATE_INFO"),
            Self::PIPELINE_SHADER_STAGE_CREATE_INFO => {
                write!(f, "PIPELINE_SHADER_STAGE_CREATE_INFO")
            }
            Self::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO => {
                write!(f, "PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO")
            }
            Self::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO => {
                write!(f, "PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO")
            }
            Self::PIPELINE_TESSELLATION_STATE_CREATE_INFO => {
                write!(f, "PIPELINE_TESSELLATION_STATE_CREATE_INFO")
            }
            Self::PIPELINE_VIEWPORT_STATE_CREATE_INFO => {
                write!(f, "PIPELINE_VIEWPORT_STATE_CREATE_INFO")
            }
            Self::PIPELINE_RASTERIZATION_STATE_CREATE_INFO => {
                write!(f, "PIPELINE_RASTERIZATION_STATE_CREATE_INFO")
            }
            Self::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO => {
                write!(f, "PIPELINE_MULTISAMPLE_STATE_CREATE_INFO")
            }
            Self::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO => {
                write!(f, "PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO")
            }
            Self::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO => {
                write!(f, "PIPELINE_COLOR_BLEND_STATE_CREATE_INFO")
            }
            Self::PIPELINE_DYNAMIC_STATE_CREATE_INFO => {
                write!(f, "PIPELINE_DYNAMIC_STATE_CREATE_INFO")
            }
            Self::GRAPHICS_PIPELINE_CREATE_INFO => write!(f, "GRAPHICS_PIPELINE_CREATE_INFO"),
            Self::COMPUTE_PIPELINE_CREATE_INFO => write!(f, "COMPUTE_PIPELINE_CREATE_INFO"),
            Self::PIPELINE_LAYOUT_CREATE_INFO => write!(f, "PIPELINE_LAYOUT_CREATE_INFO"),
            Self::SAMPLER_CREATE_INFO => write!(f, "SAMPLER_CREATE_INFO"),
            Self::DESCRIPTOR_SET_LAYOUT_CREATE_INFO => {
                write!(f, "DESCRIPTOR_SET_LAYOUT_CREATE_INFO")
            }
            Self::DESCRIPTOR_POOL_CREATE_INFO => write!(f, "DESCRIPTOR_POOL_CREATE_INFO"),
            Self::DESCRIPTOR_SET_ALLOCATE_INFO => write!(f, "DESCRIPTOR_SET_ALLOCATE_INFO"),
            Self::WRITE_DESCRIPTOR_SET => write!(f, "WRITE_DESCRIPTOR_SET"),
            Self::COPY_DESCRIPTOR_SET => write!(f, "COPY_DESCRIPTOR_SET"),
            Self::FRAMEBUFFER_CREATE_INFO => write!(f, "FRAMEBUFFER_CREATE_INFO"),
            Self::RENDER_PASS_CREATE_INFO => write!(f, "RENDER_PASS_CREATE_INFO"),
            Self::COMMAND_POOL_CREATE_INFO => write!(f, "COMMAND_POOL_CREATE_INFO"),
            Self::COMMAND_BUFFER_ALLOCATE_INFO => write!(f, "COMMAND_BUFFER_ALLOCATE_INFO"),
            Self::COMMAND_BUFFER_INHERITANCE_INFO => write!(f, "COMMAND_BUFFER_INHERITANCE_INFO"),
            Self::COMMAND_BUFFER_BEGIN_INFO => write!(f, "COMMAND_BUFFER_BEGIN_INFO"),
            Self::RENDER_PASS_BEGIN_INFO => write!(f, "RENDER_PASS_BEGIN_INFO"),
            Self::BUFFER_MEMORY_BARRIER => write!(f, "BUFFER_MEMORY_BARRIER"),
            Self::IMAGE_MEMORY_BARRIER => write!(f, "IMAGE_MEMORY_BARRIER"),
            Self::MEMORY_BARRIER => write!(f, "MEMORY_BARRIER"),
            Self::LOADER_INSTANCE_CREATE_INFO => write!(f, "LOADER_INSTANCE_CREATE_INFO"),
            Self::LOADER_DEVICE_CREATE_INFO => write!(f, "LOADER_DEVICE_CREATE_INFO"),
            Self::BIND_BUFFER_MEMORY_INFO => write!(f, "BIND_BUFFER_MEMORY_INFO"),
            Self::BIND_IMAGE_MEMORY_INFO => write!(f, "BIND_IMAGE_MEMORY_INFO"),
            Self::MEMORY_DEDICATED_REQUIREMENTS => write!(f, "MEMORY_DEDICATED_REQUIREMENTS"),
            Self::MEMORY_DEDICATED_ALLOCATE_INFO => write!(f, "MEMORY_DEDICATED_ALLOCATE_INFO"),
            Self::MEMORY_ALLOCATE_FLAGS_INFO => write!(f, "MEMORY_ALLOCATE_FLAGS_INFO"),
            Self::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO => {
                write!(f, "DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO")
            }
            Self::DEVICE_GROUP_SUBMIT_INFO => write!(f, "DEVICE_GROUP_SUBMIT_INFO"),
            Self::DEVICE_GROUP_BIND_SPARSE_INFO => write!(f, "DEVICE_GROUP_BIND_SPARSE_INFO"),
            Self::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO => {
                write!(f, "BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO")
            }
            Self::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO => {
                write!(f, "BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO")
            }
            Self::PHYSICAL_DEVICE_GROUP_PROPERTIES => write!(f, "PHYSICAL_DEVICE_GROUP_PROPERTIES"),
            Self::DEVICE_GROUP_DEVICE_CREATE_INFO => write!(f, "DEVICE_GROUP_DEVICE_CREATE_INFO"),
            Self::BUFFER_MEMORY_REQUIREMENTS_INFO_2 => {
                write!(f, "BUFFER_MEMORY_REQUIREMENTS_INFO_2")
            }
            Self::IMAGE_MEMORY_REQUIREMENTS_INFO_2 => write!(f, "IMAGE_MEMORY_REQUIREMENTS_INFO_2"),
            Self::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2 => {
                write!(f, "IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2")
            }
            Self::MEMORY_REQUIREMENTS_2 => write!(f, "MEMORY_REQUIREMENTS_2"),
            Self::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2 => {
                write!(f, "SPARSE_IMAGE_MEMORY_REQUIREMENTS_2")
            }
            Self::PHYSICAL_DEVICE_FEATURES_2 => write!(f, "PHYSICAL_DEVICE_FEATURES_2"),
            Self::PHYSICAL_DEVICE_PROPERTIES_2 => write!(f, "PHYSICAL_DEVICE_PROPERTIES_2"),
            Self::FORMAT_PROPERTIES_2 => write!(f, "FORMAT_PROPERTIES_2"),
            Self::IMAGE_FORMAT_PROPERTIES_2 => write!(f, "IMAGE_FORMAT_PROPERTIES_2"),
            Self::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2 => {
                write!(f, "PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2")
            }
            Self::QUEUE_FAMILY_PROPERTIES_2 => write!(f, "QUEUE_FAMILY_PROPERTIES_2"),
            Self::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2 => {
                write!(f, "PHYSICAL_DEVICE_MEMORY_PROPERTIES_2")
            }
            Self::SPARSE_IMAGE_FORMAT_PROPERTIES_2 => write!(f, "SPARSE_IMAGE_FORMAT_PROPERTIES_2"),
            Self::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2 => {
                write!(f, "PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2")
            }
            Self::IMAGE_VIEW_USAGE_CREATE_INFO => write!(f, "IMAGE_VIEW_USAGE_CREATE_INFO"),
            Self::PROTECTED_SUBMIT_INFO => write!(f, "PROTECTED_SUBMIT_INFO"),
            Self::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES")
            }
            Self::PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES")
            }
            Self::DEVICE_QUEUE_INFO_2 => write!(f, "DEVICE_QUEUE_INFO_2"),
            Self::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO => {
                write!(f, "PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO")
            }
            Self::EXTERNAL_IMAGE_FORMAT_PROPERTIES => write!(f, "EXTERNAL_IMAGE_FORMAT_PROPERTIES"),
            Self::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO => {
                write!(f, "PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO")
            }
            Self::EXTERNAL_BUFFER_PROPERTIES => write!(f, "EXTERNAL_BUFFER_PROPERTIES"),
            Self::PHYSICAL_DEVICE_ID_PROPERTIES => write!(f, "PHYSICAL_DEVICE_ID_PROPERTIES"),
            Self::EXTERNAL_MEMORY_BUFFER_CREATE_INFO => {
                write!(f, "EXTERNAL_MEMORY_BUFFER_CREATE_INFO")
            }
            Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO => {
                write!(f, "EXTERNAL_MEMORY_IMAGE_CREATE_INFO")
            }
            Self::EXPORT_MEMORY_ALLOCATE_INFO => write!(f, "EXPORT_MEMORY_ALLOCATE_INFO"),
            Self::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO => {
                write!(f, "PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO")
            }
            Self::EXTERNAL_FENCE_PROPERTIES => write!(f, "EXTERNAL_FENCE_PROPERTIES"),
            Self::EXPORT_FENCE_CREATE_INFO => write!(f, "EXPORT_FENCE_CREATE_INFO"),
            Self::EXPORT_SEMAPHORE_CREATE_INFO => write!(f, "EXPORT_SEMAPHORE_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO => {
                write!(f, "PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO")
            }
            Self::EXTERNAL_SEMAPHORE_PROPERTIES => write!(f, "EXTERNAL_SEMAPHORE_PROPERTIES"),
            Self::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_SUBGROUP_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES")
            }
            Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES")
            }
            Self::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO => {
                write!(f, "DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES")
            }
            Self::DESCRIPTOR_SET_LAYOUT_SUPPORT => write!(f, "DESCRIPTOR_SET_LAYOUT_SUPPORT"),
            Self::SAMPLER_YCBCR_CONVERSION_CREATE_INFO => {
                write!(f, "SAMPLER_YCBCR_CONVERSION_CREATE_INFO")
            }
            Self::SAMPLER_YCBCR_CONVERSION_INFO => write!(f, "SAMPLER_YCBCR_CONVERSION_INFO"),
            Self::BIND_IMAGE_PLANE_MEMORY_INFO => write!(f, "BIND_IMAGE_PLANE_MEMORY_INFO"),
            Self::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO => {
                write!(f, "IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO")
            }
            Self::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES")
            }
            Self::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES => {
                write!(f, "SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES")
            }
            Self::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO => {
                write!(f, "DEVICE_GROUP_RENDER_PASS_BEGIN_INFO")
            }
            Self::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES")
            }
            Self::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO => {
                write!(f, "RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO")
            }
            Self::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO => {
                write!(f, "PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO")
            }
            Self::RENDER_PASS_MULTIVIEW_CREATE_INFO => {
                write!(f, "RENDER_PASS_MULTIVIEW_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_MULTIVIEW_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_MULTIVIEW_FEATURES")
            }
            Self::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES")
            }
            Self::PHYSICAL_DEVICE_DRIVER_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_DRIVER_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_VULKAN_1_1_FEATURES")
            }
            Self::PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_VULKAN_1_2_FEATURES")
            }
            Self::PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES")
            }
            Self::IMAGE_FORMAT_LIST_CREATE_INFO => write!(f, "IMAGE_FORMAT_LIST_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES")
            }
            Self::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES")
            }
            Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES")
            }
            Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES")
            }
            Self::SEMAPHORE_TYPE_CREATE_INFO => write!(f, "SEMAPHORE_TYPE_CREATE_INFO"),
            Self::TIMELINE_SEMAPHORE_SUBMIT_INFO => write!(f, "TIMELINE_SEMAPHORE_SUBMIT_INFO"),
            Self::SEMAPHORE_WAIT_INFO => write!(f, "SEMAPHORE_WAIT_INFO"),
            Self::SEMAPHORE_SIGNAL_INFO => write!(f, "SEMAPHORE_SIGNAL_INFO"),
            Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES")
            }
            Self::BUFFER_DEVICE_ADDRESS_INFO => write!(f, "BUFFER_DEVICE_ADDRESS_INFO"),
            Self::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO => {
                write!(f, "BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO")
            }
            Self::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO => {
                write!(f, "MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO")
            }
            Self::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO => {
                write!(f, "DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO")
            }
            Self::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES")
            }
            Self::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES")
            }
            Self::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO => {
                write!(f, "DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES")
            }
            Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO => {
                write!(f, "DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO")
            }
            Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT => {
                write!(f, "DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT")
            }
            Self::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES")
            }
            Self::SAMPLER_REDUCTION_MODE_CREATE_INFO => {
                write!(f, "SAMPLER_REDUCTION_MODE_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES")
            }
            Self::ATTACHMENT_DESCRIPTION_2 => write!(f, "ATTACHMENT_DESCRIPTION_2"),
            Self::ATTACHMENT_REFERENCE_2 => write!(f, "ATTACHMENT_REFERENCE_2"),
            Self::SUBPASS_DESCRIPTION_2 => write!(f, "SUBPASS_DESCRIPTION_2"),
            Self::SUBPASS_DEPENDENCY_2 => write!(f, "SUBPASS_DEPENDENCY_2"),
            Self::RENDER_PASS_CREATE_INFO_2 => write!(f, "RENDER_PASS_CREATE_INFO_2"),
            Self::SUBPASS_BEGIN_INFO => write!(f, "SUBPASS_BEGIN_INFO"),
            Self::SUBPASS_END_INFO => write!(f, "SUBPASS_END_INFO"),
            Self::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES")
            }
            Self::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE => {
                write!(f, "SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE")
            }
            Self::IMAGE_STENCIL_USAGE_CREATE_INFO => write!(f, "IMAGE_STENCIL_USAGE_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES")
            }
            Self::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO => {
                write!(f, "FRAMEBUFFER_ATTACHMENTS_CREATE_INFO")
            }
            Self::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO => {
                write!(f, "FRAMEBUFFER_ATTACHMENT_IMAGE_INFO")
            }
            Self::RENDER_PASS_ATTACHMENT_BEGIN_INFO => {
                write!(f, "RENDER_PASS_ATTACHMENT_BEGIN_INFO")
            }
            Self::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES")
            }
            Self::ATTACHMENT_REFERENCE_STENCIL_LAYOUT => {
                write!(f, "ATTACHMENT_REFERENCE_STENCIL_LAYOUT")
            }
            Self::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT => {
                write!(f, "ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT")
            }
            Self::PHYSICAL_DEVICE_VULKAN_1_3_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_VULKAN_1_3_FEATURES")
            }
            Self::PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_TOOL_PROPERTIES => write!(f, "PHYSICAL_DEVICE_TOOL_PROPERTIES"),
            Self::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES")
            }
            Self::DEVICE_PRIVATE_DATA_CREATE_INFO => write!(f, "DEVICE_PRIVATE_DATA_CREATE_INFO"),
            Self::PRIVATE_DATA_SLOT_CREATE_INFO => write!(f, "PRIVATE_DATA_SLOT_CREATE_INFO"),
            Self::MEMORY_BARRIER_2 => write!(f, "MEMORY_BARRIER_2"),
            Self::BUFFER_MEMORY_BARRIER_2 => write!(f, "BUFFER_MEMORY_BARRIER_2"),
            Self::IMAGE_MEMORY_BARRIER_2 => write!(f, "IMAGE_MEMORY_BARRIER_2"),
            Self::DEPENDENCY_INFO => write!(f, "DEPENDENCY_INFO"),
            Self::SUBMIT_INFO_2 => write!(f, "SUBMIT_INFO_2"),
            Self::SEMAPHORE_SUBMIT_INFO => write!(f, "SEMAPHORE_SUBMIT_INFO"),
            Self::COMMAND_BUFFER_SUBMIT_INFO => write!(f, "COMMAND_BUFFER_SUBMIT_INFO"),
            Self::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES")
            }
            Self::COPY_BUFFER_INFO_2 => write!(f, "COPY_BUFFER_INFO_2"),
            Self::COPY_IMAGE_INFO_2 => write!(f, "COPY_IMAGE_INFO_2"),
            Self::COPY_BUFFER_TO_IMAGE_INFO_2 => write!(f, "COPY_BUFFER_TO_IMAGE_INFO_2"),
            Self::COPY_IMAGE_TO_BUFFER_INFO_2 => write!(f, "COPY_IMAGE_TO_BUFFER_INFO_2"),
            Self::BUFFER_COPY_2 => write!(f, "BUFFER_COPY_2"),
            Self::IMAGE_COPY_2 => write!(f, "IMAGE_COPY_2"),
            Self::BUFFER_IMAGE_COPY_2 => write!(f, "BUFFER_IMAGE_COPY_2"),
            Self::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES")
            }
            Self::FORMAT_PROPERTIES_3 => write!(f, "FORMAT_PROPERTIES_3"),
            Self::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES")
            }
            Self::DEVICE_BUFFER_MEMORY_REQUIREMENTS => {
                write!(f, "DEVICE_BUFFER_MEMORY_REQUIREMENTS")
            }
            Self::DEVICE_IMAGE_MEMORY_REQUIREMENTS => write!(f, "DEVICE_IMAGE_MEMORY_REQUIREMENTS"),
            Self::PIPELINE_CREATION_FEEDBACK_CREATE_INFO => {
                write!(f, "PIPELINE_CREATION_FEEDBACK_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES => write!(
                f,
                "PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES"
            ),
            Self::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES => write!(
                f,
                "PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES"
            ),
            Self::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES => write!(
                f,
                "PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES"
            ),
            Self::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES")
            }
            Self::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO => write!(
                f,
                "PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO"
            ),
            Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES")
            }
            Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES")
            }
            Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES")
            }
            Self::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK => {
                write!(f, "WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK")
            }
            Self::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO => {
                write!(f, "DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES")
            }
            Self::BLIT_IMAGE_INFO_2 => write!(f, "BLIT_IMAGE_INFO_2"),
            Self::RESOLVE_IMAGE_INFO_2 => write!(f, "RESOLVE_IMAGE_INFO_2"),
            Self::IMAGE_BLIT_2 => write!(f, "IMAGE_BLIT_2"),
            Self::IMAGE_RESOLVE_2 => write!(f, "IMAGE_RESOLVE_2"),
            Self::RENDERING_INFO => write!(f, "RENDERING_INFO"),
            Self::RENDERING_ATTACHMENT_INFO => write!(f, "RENDERING_ATTACHMENT_INFO"),
            Self::PIPELINE_RENDERING_CREATE_INFO => write!(f, "PIPELINE_RENDERING_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES")
            }
            Self::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO => {
                write!(f, "COMMAND_BUFFER_INHERITANCE_RENDERING_INFO")
            }
            Self::PHYSICAL_DEVICE_VULKAN_1_4_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_VULKAN_1_4_FEATURES")
            }
            Self::PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES")
            }
            Self::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO => {
                write!(f, "DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES")
            }
            Self::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES => {
                write!(f, "QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES")
            }
            Self::MEMORY_MAP_INFO => write!(f, "MEMORY_MAP_INFO"),
            Self::MEMORY_UNMAP_INFO => write!(f, "MEMORY_UNMAP_INFO"),
            Self::PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES")
            }
            Self::DEVICE_IMAGE_SUBRESOURCE_INFO => write!(f, "DEVICE_IMAGE_SUBRESOURCE_INFO"),
            Self::SUBRESOURCE_LAYOUT_2 => write!(f, "SUBRESOURCE_LAYOUT_2"),
            Self::IMAGE_SUBRESOURCE_2 => write!(f, "IMAGE_SUBRESOURCE_2"),
            Self::BUFFER_USAGE_FLAGS_2_CREATE_INFO => write!(f, "BUFFER_USAGE_FLAGS_2_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES")
            }
            Self::BIND_MEMORY_STATUS => write!(f, "BIND_MEMORY_STATUS"),
            Self::PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES")
            }
            Self::PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES")
            }
            Self::MEMORY_TO_IMAGE_COPY => write!(f, "MEMORY_TO_IMAGE_COPY"),
            Self::IMAGE_TO_MEMORY_COPY => write!(f, "IMAGE_TO_MEMORY_COPY"),
            Self::COPY_IMAGE_TO_MEMORY_INFO => write!(f, "COPY_IMAGE_TO_MEMORY_INFO"),
            Self::COPY_MEMORY_TO_IMAGE_INFO => write!(f, "COPY_MEMORY_TO_IMAGE_INFO"),
            Self::HOST_IMAGE_LAYOUT_TRANSITION_INFO => {
                write!(f, "HOST_IMAGE_LAYOUT_TRANSITION_INFO")
            }
            Self::COPY_IMAGE_TO_IMAGE_INFO => write!(f, "COPY_IMAGE_TO_IMAGE_INFO"),
            Self::SUBRESOURCE_HOST_MEMCPY_SIZE => write!(f, "SUBRESOURCE_HOST_MEMCPY_SIZE"),
            Self::HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY => {
                write!(f, "HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY")
            }
            Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES")
            }
            Self::PIPELINE_CREATE_FLAGS_2_CREATE_INFO => {
                write!(f, "PIPELINE_CREATE_FLAGS_2_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES")
            }
            Self::BIND_DESCRIPTOR_SETS_INFO => write!(f, "BIND_DESCRIPTOR_SETS_INFO"),
            Self::PUSH_CONSTANTS_INFO => write!(f, "PUSH_CONSTANTS_INFO"),
            Self::PUSH_DESCRIPTOR_SET_INFO => write!(f, "PUSH_DESCRIPTOR_SET_INFO"),
            Self::PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO => {
                write!(f, "PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO")
            }
            Self::PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES")
            }
            Self::PIPELINE_ROBUSTNESS_CREATE_INFO => write!(f, "PIPELINE_ROBUSTNESS_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES")
            }
            Self::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES")
            }
            Self::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO => {
                write!(f, "PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES => {
                write!(f, "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES")
            }
            Self::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO => {
                write!(f, "PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES")
            }
            Self::RENDERING_AREA_INFO => write!(f, "RENDERING_AREA_INFO"),
            Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES => {
                write!(f, "PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES")
            }
            Self::RENDERING_ATTACHMENT_LOCATION_INFO => {
                write!(f, "RENDERING_ATTACHMENT_LOCATION_INFO")
            }
            Self::RENDERING_INPUT_ATTACHMENT_INDEX_INFO => {
                write!(f, "RENDERING_INPUT_ATTACHMENT_INDEX_INFO")
            }
            Self::SWAPCHAIN_CREATE_INFO_KHR => write!(f, "SWAPCHAIN_CREATE_INFO_KHR"),
            Self::PRESENT_INFO_KHR => write!(f, "PRESENT_INFO_KHR"),
            Self::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR => {
                write!(f, "DEVICE_GROUP_PRESENT_CAPABILITIES_KHR")
            }
            Self::IMAGE_SWAPCHAIN_CREATE_INFO_KHR => write!(f, "IMAGE_SWAPCHAIN_CREATE_INFO_KHR"),
            Self::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR => {
                write!(f, "BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR")
            }
            Self::ACQUIRE_NEXT_IMAGE_INFO_KHR => write!(f, "ACQUIRE_NEXT_IMAGE_INFO_KHR"),
            Self::DEVICE_GROUP_PRESENT_INFO_KHR => write!(f, "DEVICE_GROUP_PRESENT_INFO_KHR"),
            Self::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR => {
                write!(f, "DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR")
            }
            Self::DISPLAY_MODE_CREATE_INFO_KHR => write!(f, "DISPLAY_MODE_CREATE_INFO_KHR"),
            Self::DISPLAY_SURFACE_CREATE_INFO_KHR => write!(f, "DISPLAY_SURFACE_CREATE_INFO_KHR"),
            Self::DISPLAY_PRESENT_INFO_KHR => write!(f, "DISPLAY_PRESENT_INFO_KHR"),
            Self::XLIB_SURFACE_CREATE_INFO_KHR => write!(f, "XLIB_SURFACE_CREATE_INFO_KHR"),
            Self::XCB_SURFACE_CREATE_INFO_KHR => write!(f, "XCB_SURFACE_CREATE_INFO_KHR"),
            Self::WAYLAND_SURFACE_CREATE_INFO_KHR => write!(f, "WAYLAND_SURFACE_CREATE_INFO_KHR"),
            Self::ANDROID_SURFACE_CREATE_INFO_KHR => write!(f, "ANDROID_SURFACE_CREATE_INFO_KHR"),
            Self::WIN32_SURFACE_CREATE_INFO_KHR => write!(f, "WIN32_SURFACE_CREATE_INFO_KHR"),
            Self::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT => {
                write!(f, "DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT")
            }
            Self::PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD => {
                write!(f, "PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD")
            }
            Self::DEBUG_MARKER_OBJECT_NAME_INFO_EXT => {
                write!(f, "DEBUG_MARKER_OBJECT_NAME_INFO_EXT")
            }
            Self::DEBUG_MARKER_OBJECT_TAG_INFO_EXT => write!(f, "DEBUG_MARKER_OBJECT_TAG_INFO_EXT"),
            Self::DEBUG_MARKER_MARKER_INFO_EXT => write!(f, "DEBUG_MARKER_MARKER_INFO_EXT"),
            Self::VIDEO_PROFILE_INFO_KHR => write!(f, "VIDEO_PROFILE_INFO_KHR"),
            Self::VIDEO_CAPABILITIES_KHR => write!(f, "VIDEO_CAPABILITIES_KHR"),
            Self::VIDEO_PICTURE_RESOURCE_INFO_KHR => write!(f, "VIDEO_PICTURE_RESOURCE_INFO_KHR"),
            Self::VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR => {
                write!(f, "VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR")
            }
            Self::BIND_VIDEO_SESSION_MEMORY_INFO_KHR => {
                write!(f, "BIND_VIDEO_SESSION_MEMORY_INFO_KHR")
            }
            Self::VIDEO_SESSION_CREATE_INFO_KHR => write!(f, "VIDEO_SESSION_CREATE_INFO_KHR"),
            Self::VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                write!(f, "VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR")
            }
            Self::VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR => {
                write!(f, "VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR")
            }
            Self::VIDEO_BEGIN_CODING_INFO_KHR => write!(f, "VIDEO_BEGIN_CODING_INFO_KHR"),
            Self::VIDEO_END_CODING_INFO_KHR => write!(f, "VIDEO_END_CODING_INFO_KHR"),
            Self::VIDEO_CODING_CONTROL_INFO_KHR => write!(f, "VIDEO_CODING_CONTROL_INFO_KHR"),
            Self::VIDEO_REFERENCE_SLOT_INFO_KHR => write!(f, "VIDEO_REFERENCE_SLOT_INFO_KHR"),
            Self::QUEUE_FAMILY_VIDEO_PROPERTIES_KHR => {
                write!(f, "QUEUE_FAMILY_VIDEO_PROPERTIES_KHR")
            }
            Self::VIDEO_PROFILE_LIST_INFO_KHR => write!(f, "VIDEO_PROFILE_LIST_INFO_KHR"),
            Self::PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR => {
                write!(f, "PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR")
            }
            Self::VIDEO_FORMAT_PROPERTIES_KHR => write!(f, "VIDEO_FORMAT_PROPERTIES_KHR"),
            Self::QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR => {
                write!(f, "QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR")
            }
            Self::VIDEO_DECODE_INFO_KHR => write!(f, "VIDEO_DECODE_INFO_KHR"),
            Self::VIDEO_DECODE_CAPABILITIES_KHR => write!(f, "VIDEO_DECODE_CAPABILITIES_KHR"),
            Self::VIDEO_DECODE_USAGE_INFO_KHR => write!(f, "VIDEO_DECODE_USAGE_INFO_KHR"),
            Self::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV => {
                write!(f, "DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV")
            }
            Self::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV => {
                write!(f, "DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV")
            }
            Self::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV => {
                write!(f, "DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT")
            }
            Self::PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT => {
                write!(f, "PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT")
            }
            Self::CU_MODULE_CREATE_INFO_NVX => write!(f, "CU_MODULE_CREATE_INFO_NVX"),
            Self::CU_FUNCTION_CREATE_INFO_NVX => write!(f, "CU_FUNCTION_CREATE_INFO_NVX"),
            Self::CU_LAUNCH_INFO_NVX => write!(f, "CU_LAUNCH_INFO_NVX"),
            Self::CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX => {
                write!(f, "CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX")
            }
            Self::IMAGE_VIEW_HANDLE_INFO_NVX => write!(f, "IMAGE_VIEW_HANDLE_INFO_NVX"),
            Self::IMAGE_VIEW_ADDRESS_PROPERTIES_NVX => {
                write!(f, "IMAGE_VIEW_ADDRESS_PROPERTIES_NVX")
            }
            Self::VIDEO_ENCODE_H264_CAPABILITIES_KHR => {
                write!(f, "VIDEO_ENCODE_H264_CAPABILITIES_KHR")
            }
            Self::VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_PICTURE_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H264_PICTURE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_PROFILE_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H264_PROFILE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR => {
                write!(f, "VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR")
            }
            Self::VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_CAPABILITIES_KHR => {
                write!(f, "VIDEO_ENCODE_H265_CAPABILITIES_KHR")
            }
            Self::VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_PICTURE_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H265_PICTURE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_DPB_SLOT_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H265_DPB_SLOT_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_GOP_REMAINING_FRAME_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H265_GOP_REMAINING_FRAME_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_PROFILE_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H265_PROFILE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_RATE_CONTROL_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H265_RATE_CONTROL_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_SESSION_CREATE_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H265_SESSION_CREATE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_QUALITY_LEVEL_PROPERTIES_KHR => {
                write!(f, "VIDEO_ENCODE_H265_QUALITY_LEVEL_PROPERTIES_KHR")
            }
            Self::VIDEO_ENCODE_H265_SESSION_PARAMETERS_GET_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H265_SESSION_PARAMETERS_GET_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_SESSION_PARAMETERS_FEEDBACK_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_H265_SESSION_PARAMETERS_FEEDBACK_INFO_KHR")
            }
            Self::VIDEO_DECODE_H264_CAPABILITIES_KHR => {
                write!(f, "VIDEO_DECODE_H264_CAPABILITIES_KHR")
            }
            Self::VIDEO_DECODE_H264_PICTURE_INFO_KHR => {
                write!(f, "VIDEO_DECODE_H264_PICTURE_INFO_KHR")
            }
            Self::VIDEO_DECODE_H264_PROFILE_INFO_KHR => {
                write!(f, "VIDEO_DECODE_H264_PROFILE_INFO_KHR")
            }
            Self::VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                write!(f, "VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR")
            }
            Self::VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR => {
                write!(f, "VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR")
            }
            Self::VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR => {
                write!(f, "VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR")
            }
            Self::TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD => {
                write!(f, "TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD")
            }
            Self::STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP => {
                write!(f, "STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP")
            }
            Self::PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV")
            }
            Self::PRIVATE_VENDOR_INFO_PLACEHOLDER_OFFSET_0_NV => {
                write!(f, "PRIVATE_VENDOR_INFO_PLACEHOLDER_OFFSET_0_NV")
            }
            Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV => {
                write!(f, "EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV")
            }
            Self::EXPORT_MEMORY_ALLOCATE_INFO_NV => write!(f, "EXPORT_MEMORY_ALLOCATE_INFO_NV"),
            Self::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV => {
                write!(f, "IMPORT_MEMORY_WIN32_HANDLE_INFO_NV")
            }
            Self::EXPORT_MEMORY_WIN32_HANDLE_INFO_NV => {
                write!(f, "EXPORT_MEMORY_WIN32_HANDLE_INFO_NV")
            }
            Self::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV => {
                write!(f, "WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV")
            }
            Self::VALIDATION_FLAGS_EXT => write!(f, "VALIDATION_FLAGS_EXT"),
            Self::VI_SURFACE_CREATE_INFO_NN => write!(f, "VI_SURFACE_CREATE_INFO_NN"),
            Self::IMAGE_VIEW_ASTC_DECODE_MODE_EXT => write!(f, "IMAGE_VIEW_ASTC_DECODE_MODE_EXT"),
            Self::PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT")
            }
            Self::IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR => {
                write!(f, "IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR")
            }
            Self::EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR => {
                write!(f, "EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR")
            }
            Self::MEMORY_WIN32_HANDLE_PROPERTIES_KHR => {
                write!(f, "MEMORY_WIN32_HANDLE_PROPERTIES_KHR")
            }
            Self::MEMORY_GET_WIN32_HANDLE_INFO_KHR => write!(f, "MEMORY_GET_WIN32_HANDLE_INFO_KHR"),
            Self::IMPORT_MEMORY_FD_INFO_KHR => write!(f, "IMPORT_MEMORY_FD_INFO_KHR"),
            Self::MEMORY_FD_PROPERTIES_KHR => write!(f, "MEMORY_FD_PROPERTIES_KHR"),
            Self::MEMORY_GET_FD_INFO_KHR => write!(f, "MEMORY_GET_FD_INFO_KHR"),
            Self::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR => {
                write!(f, "WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR")
            }
            Self::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR => {
                write!(f, "IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR")
            }
            Self::EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR => {
                write!(f, "EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR")
            }
            Self::D3D12_FENCE_SUBMIT_INFO_KHR => write!(f, "D3D12_FENCE_SUBMIT_INFO_KHR"),
            Self::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR => {
                write!(f, "SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR")
            }
            Self::IMPORT_SEMAPHORE_FD_INFO_KHR => write!(f, "IMPORT_SEMAPHORE_FD_INFO_KHR"),
            Self::SEMAPHORE_GET_FD_INFO_KHR => write!(f, "SEMAPHORE_GET_FD_INFO_KHR"),
            Self::COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT => write!(
                f,
                "COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT"
            ),
            Self::PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT")
            }
            Self::CONDITIONAL_RENDERING_BEGIN_INFO_EXT => {
                write!(f, "CONDITIONAL_RENDERING_BEGIN_INFO_EXT")
            }
            Self::PRESENT_REGIONS_KHR => write!(f, "PRESENT_REGIONS_KHR"),
            Self::PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV => {
                write!(f, "PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV")
            }
            Self::SURFACE_CAPABILITIES_2_EXT => write!(f, "SURFACE_CAPABILITIES_2_EXT"),
            Self::DISPLAY_POWER_INFO_EXT => write!(f, "DISPLAY_POWER_INFO_EXT"),
            Self::DEVICE_EVENT_INFO_EXT => write!(f, "DEVICE_EVENT_INFO_EXT"),
            Self::DISPLAY_EVENT_INFO_EXT => write!(f, "DISPLAY_EVENT_INFO_EXT"),
            Self::SWAPCHAIN_COUNTER_CREATE_INFO_EXT => {
                write!(f, "SWAPCHAIN_COUNTER_CREATE_INFO_EXT")
            }
            Self::PRESENT_TIMES_INFO_GOOGLE => write!(f, "PRESENT_TIMES_INFO_GOOGLE"),
            Self::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX => write!(
                f,
                "PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX"
            ),
            Self::MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX => {
                write!(f, "MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX")
            }
            Self::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV => {
                write!(f, "PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT")
            }
            Self::PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT => {
                write!(f, "PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT"
            ),
            Self::PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT => write!(
                f,
                "PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT"
            ),
            Self::PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT")
            }
            Self::PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT => {
                write!(f, "PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT")
            }
            Self::HDR_METADATA_EXT => write!(f, "HDR_METADATA_EXT"),
            Self::PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG => {
                write!(f, "PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG")
            }
            Self::SHARED_PRESENT_SURFACE_CAPABILITIES_KHR => {
                write!(f, "SHARED_PRESENT_SURFACE_CAPABILITIES_KHR")
            }
            Self::IMPORT_FENCE_WIN32_HANDLE_INFO_KHR => {
                write!(f, "IMPORT_FENCE_WIN32_HANDLE_INFO_KHR")
            }
            Self::EXPORT_FENCE_WIN32_HANDLE_INFO_KHR => {
                write!(f, "EXPORT_FENCE_WIN32_HANDLE_INFO_KHR")
            }
            Self::FENCE_GET_WIN32_HANDLE_INFO_KHR => write!(f, "FENCE_GET_WIN32_HANDLE_INFO_KHR"),
            Self::IMPORT_FENCE_FD_INFO_KHR => write!(f, "IMPORT_FENCE_FD_INFO_KHR"),
            Self::FENCE_GET_FD_INFO_KHR => write!(f, "FENCE_GET_FD_INFO_KHR"),
            Self::PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR")
            }
            Self::QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR => {
                write!(f, "QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR")
            }
            Self::PERFORMANCE_QUERY_SUBMIT_INFO_KHR => {
                write!(f, "PERFORMANCE_QUERY_SUBMIT_INFO_KHR")
            }
            Self::ACQUIRE_PROFILING_LOCK_INFO_KHR => write!(f, "ACQUIRE_PROFILING_LOCK_INFO_KHR"),
            Self::PERFORMANCE_COUNTER_KHR => write!(f, "PERFORMANCE_COUNTER_KHR"),
            Self::PERFORMANCE_COUNTER_DESCRIPTION_KHR => {
                write!(f, "PERFORMANCE_COUNTER_DESCRIPTION_KHR")
            }
            Self::PHYSICAL_DEVICE_SURFACE_INFO_2_KHR => {
                write!(f, "PHYSICAL_DEVICE_SURFACE_INFO_2_KHR")
            }
            Self::SURFACE_CAPABILITIES_2_KHR => write!(f, "SURFACE_CAPABILITIES_2_KHR"),
            Self::SURFACE_FORMAT_2_KHR => write!(f, "SURFACE_FORMAT_2_KHR"),
            Self::DISPLAY_PROPERTIES_2_KHR => write!(f, "DISPLAY_PROPERTIES_2_KHR"),
            Self::DISPLAY_PLANE_PROPERTIES_2_KHR => write!(f, "DISPLAY_PLANE_PROPERTIES_2_KHR"),
            Self::DISPLAY_MODE_PROPERTIES_2_KHR => write!(f, "DISPLAY_MODE_PROPERTIES_2_KHR"),
            Self::DISPLAY_PLANE_INFO_2_KHR => write!(f, "DISPLAY_PLANE_INFO_2_KHR"),
            Self::DISPLAY_PLANE_CAPABILITIES_2_KHR => write!(f, "DISPLAY_PLANE_CAPABILITIES_2_KHR"),
            Self::IOS_SURFACE_CREATE_INFO_MVK => write!(f, "IOS_SURFACE_CREATE_INFO_MVK"),
            Self::MACOS_SURFACE_CREATE_INFO_MVK => write!(f, "MACOS_SURFACE_CREATE_INFO_MVK"),
            Self::DEBUG_UTILS_OBJECT_NAME_INFO_EXT => write!(f, "DEBUG_UTILS_OBJECT_NAME_INFO_EXT"),
            Self::DEBUG_UTILS_OBJECT_TAG_INFO_EXT => write!(f, "DEBUG_UTILS_OBJECT_TAG_INFO_EXT"),
            Self::DEBUG_UTILS_LABEL_EXT => write!(f, "DEBUG_UTILS_LABEL_EXT"),
            Self::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT => {
                write!(f, "DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT")
            }
            Self::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT => {
                write!(f, "DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT")
            }
            Self::ANDROID_HARDWARE_BUFFER_USAGE_ANDROID => {
                write!(f, "ANDROID_HARDWARE_BUFFER_USAGE_ANDROID")
            }
            Self::ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID => {
                write!(f, "ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID")
            }
            Self::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID => {
                write!(f, "ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID")
            }
            Self::IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID => {
                write!(f, "IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID")
            }
            Self::MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID => {
                write!(f, "MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID")
            }
            Self::EXTERNAL_FORMAT_ANDROID => write!(f, "EXTERNAL_FORMAT_ANDROID"),
            Self::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID => {
                write!(f, "ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID")
            }
            Self::PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX => {
                write!(f, "PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX")
            }
            Self::PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX => {
                write!(f, "PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX")
            }
            Self::EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX => {
                write!(f, "EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX")
            }
            Self::EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX => {
                write!(f, "EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX")
            }
            Self::PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX => {
                write!(f, "PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX")
            }
            Self::TEXEL_BUFFER_DESCRIPTOR_INFO_EXT => write!(f, "TEXEL_BUFFER_DESCRIPTOR_INFO_EXT"),
            Self::IMAGE_DESCRIPTOR_INFO_EXT => write!(f, "IMAGE_DESCRIPTOR_INFO_EXT"),
            Self::RESOURCE_DESCRIPTOR_INFO_EXT => write!(f, "RESOURCE_DESCRIPTOR_INFO_EXT"),
            Self::BIND_HEAP_INFO_EXT => write!(f, "BIND_HEAP_INFO_EXT"),
            Self::PUSH_DATA_INFO_EXT => write!(f, "PUSH_DATA_INFO_EXT"),
            Self::DESCRIPTOR_SET_AND_BINDING_MAPPING_EXT => {
                write!(f, "DESCRIPTOR_SET_AND_BINDING_MAPPING_EXT")
            }
            Self::SHADER_DESCRIPTOR_SET_AND_BINDING_MAPPING_INFO_EXT => {
                write!(f, "SHADER_DESCRIPTOR_SET_AND_BINDING_MAPPING_INFO_EXT")
            }
            Self::OPAQUE_CAPTURE_DATA_CREATE_INFO_EXT => {
                write!(f, "OPAQUE_CAPTURE_DATA_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_HEAP_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_DESCRIPTOR_HEAP_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_HEAP_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_DESCRIPTOR_HEAP_FEATURES_EXT")
            }
            Self::COMMAND_BUFFER_INHERITANCE_DESCRIPTOR_HEAP_INFO_EXT => {
                write!(f, "COMMAND_BUFFER_INHERITANCE_DESCRIPTOR_HEAP_INFO_EXT")
            }
            Self::SAMPLER_CUSTOM_BORDER_COLOR_INDEX_CREATE_INFO_EXT => {
                write!(f, "SAMPLER_CUSTOM_BORDER_COLOR_INDEX_CREATE_INFO_EXT")
            }
            Self::INDIRECT_COMMANDS_LAYOUT_PUSH_DATA_TOKEN_NV => {
                write!(f, "INDIRECT_COMMANDS_LAYOUT_PUSH_DATA_TOKEN_NV")
            }
            Self::SUBSAMPLED_IMAGE_FORMAT_PROPERTIES_EXT => {
                write!(f, "SUBSAMPLED_IMAGE_FORMAT_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_HEAP_TENSOR_PROPERTIES_ARM => {
                write!(f, "PHYSICAL_DEVICE_DESCRIPTOR_HEAP_TENSOR_PROPERTIES_ARM")
            }
            Self::ATTACHMENT_SAMPLE_COUNT_INFO_AMD => write!(f, "ATTACHMENT_SAMPLE_COUNT_INFO_AMD"),
            Self::PHYSICAL_DEVICE_SHADER_BFLOAT16_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_SHADER_BFLOAT16_FEATURES_KHR")
            }
            Self::SAMPLE_LOCATIONS_INFO_EXT => write!(f, "SAMPLE_LOCATIONS_INFO_EXT"),
            Self::RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT => {
                write!(f, "RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT")
            }
            Self::PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT => {
                write!(f, "PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT")
            }
            Self::MULTISAMPLE_PROPERTIES_EXT => write!(f, "MULTISAMPLE_PROPERTIES_EXT"),
            Self::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT")
            }
            Self::PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT => {
                write!(f, "PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT")
            }
            Self::PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV => {
                write!(f, "PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV")
            }
            Self::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR => {
                write!(f, "WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR")
            }
            Self::ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR => {
                write!(f, "ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR")
            }
            Self::ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR => {
                write!(f, "ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR")
            }
            Self::ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR => {
                write!(f, "ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR")
            }
            Self::ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR => {
                write!(f, "ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR")
            }
            Self::ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR => {
                write!(f, "ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR")
            }
            Self::ACCELERATION_STRUCTURE_GEOMETRY_KHR => {
                write!(f, "ACCELERATION_STRUCTURE_GEOMETRY_KHR")
            }
            Self::ACCELERATION_STRUCTURE_VERSION_INFO_KHR => {
                write!(f, "ACCELERATION_STRUCTURE_VERSION_INFO_KHR")
            }
            Self::COPY_ACCELERATION_STRUCTURE_INFO_KHR => {
                write!(f, "COPY_ACCELERATION_STRUCTURE_INFO_KHR")
            }
            Self::COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR => {
                write!(f, "COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR")
            }
            Self::COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR => {
                write!(f, "COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR")
            }
            Self::ACCELERATION_STRUCTURE_CREATE_INFO_KHR => {
                write!(f, "ACCELERATION_STRUCTURE_CREATE_INFO_KHR")
            }
            Self::ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR => {
                write!(f, "ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR")
            }
            Self::RAY_TRACING_PIPELINE_CREATE_INFO_KHR => {
                write!(f, "RAY_TRACING_PIPELINE_CREATE_INFO_KHR")
            }
            Self::RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR => {
                write!(f, "RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR")
            }
            Self::RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR => {
                write!(f, "RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR")
            }
            Self::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV => {
                write!(f, "PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV => {
                write!(f, "PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV")
            }
            Self::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT => {
                write!(f, "DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT")
            }
            Self::PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT => {
                write!(f, "PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT")
            }
            Self::IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT => {
                write!(f, "IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT")
            }
            Self::IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT => {
                write!(f, "IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT")
            }
            Self::IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT => {
                write!(f, "IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT")
            }
            Self::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT => {
                write!(f, "DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT")
            }
            Self::VALIDATION_CACHE_CREATE_INFO_EXT => write!(f, "VALIDATION_CACHE_CREATE_INFO_EXT"),
            Self::SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT => {
                write!(f, "SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR")
            }
            Self::PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV => write!(
                f,
                "PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV"
            ),
            Self::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV => {
                write!(f, "PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV")
            }
            Self::PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV => write!(
                f,
                "PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV"
            ),
            Self::RAY_TRACING_PIPELINE_CREATE_INFO_NV => {
                write!(f, "RAY_TRACING_PIPELINE_CREATE_INFO_NV")
            }
            Self::ACCELERATION_STRUCTURE_CREATE_INFO_NV => {
                write!(f, "ACCELERATION_STRUCTURE_CREATE_INFO_NV")
            }
            Self::GEOMETRY_NV => write!(f, "GEOMETRY_NV"),
            Self::GEOMETRY_TRIANGLES_NV => write!(f, "GEOMETRY_TRIANGLES_NV"),
            Self::GEOMETRY_AABB_NV => write!(f, "GEOMETRY_AABB_NV"),
            Self::BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV => {
                write!(f, "BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV")
            }
            Self::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV => {
                write!(f, "WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV")
            }
            Self::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV => {
                write!(f, "ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV => {
                write!(f, "PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV")
            }
            Self::RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV => {
                write!(f, "RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV")
            }
            Self::ACCELERATION_STRUCTURE_INFO_NV => write!(f, "ACCELERATION_STRUCTURE_INFO_NV"),
            Self::PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV => write!(
                f,
                "PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV"
            ),
            Self::PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV => write!(
                f,
                "PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV"
            ),
            Self::PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT => {
                write!(f, "PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT")
            }
            Self::FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT => {
                write!(f, "FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_CONVERSION_FEATURES_QCOM => write!(
                f,
                "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_CONVERSION_FEATURES_QCOM"
            ),
            Self::IMPORT_MEMORY_HOST_POINTER_INFO_EXT => {
                write!(f, "IMPORT_MEMORY_HOST_POINTER_INFO_EXT")
            }
            Self::MEMORY_HOST_POINTER_PROPERTIES_EXT => {
                write!(f, "MEMORY_HOST_POINTER_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR")
            }
            Self::PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD => {
                write!(f, "PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD")
            }
            Self::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD => {
                write!(f, "PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD")
            }
            Self::VIDEO_DECODE_H265_CAPABILITIES_KHR => {
                write!(f, "VIDEO_DECODE_H265_CAPABILITIES_KHR")
            }
            Self::VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                write!(f, "VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR")
            }
            Self::VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR => {
                write!(f, "VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR")
            }
            Self::VIDEO_DECODE_H265_PROFILE_INFO_KHR => {
                write!(f, "VIDEO_DECODE_H265_PROFILE_INFO_KHR")
            }
            Self::VIDEO_DECODE_H265_PICTURE_INFO_KHR => {
                write!(f, "VIDEO_DECODE_H265_PICTURE_INFO_KHR")
            }
            Self::VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR => {
                write!(f, "VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR")
            }
            Self::DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD => {
                write!(f, "DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD")
            }
            Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT")
            }
            Self::PRESENT_FRAME_TOKEN_GGP => write!(f, "PRESENT_FRAME_TOKEN_GGP"),
            Self::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV => {
                write!(f, "PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV")
            }
            Self::PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV => write!(
                f,
                "PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV"
            ),
            Self::PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV")
            }
            Self::CHECKPOINT_DATA_NV => write!(f, "CHECKPOINT_DATA_NV"),
            Self::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV => {
                write!(f, "QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV")
            }
            Self::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV => {
                write!(f, "QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV")
            }
            Self::CHECKPOINT_DATA_2_NV => write!(f, "CHECKPOINT_DATA_2_NV"),
            Self::PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT")
            }
            Self::SWAPCHAIN_TIMING_PROPERTIES_EXT => write!(f, "SWAPCHAIN_TIMING_PROPERTIES_EXT"),
            Self::SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT => {
                write!(f, "SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT")
            }
            Self::PRESENT_TIMINGS_INFO_EXT => write!(f, "PRESENT_TIMINGS_INFO_EXT"),
            Self::PRESENT_TIMING_INFO_EXT => write!(f, "PRESENT_TIMING_INFO_EXT"),
            Self::PAST_PRESENTATION_TIMING_INFO_EXT => {
                write!(f, "PAST_PRESENTATION_TIMING_INFO_EXT")
            }
            Self::PAST_PRESENTATION_TIMING_PROPERTIES_EXT => {
                write!(f, "PAST_PRESENTATION_TIMING_PROPERTIES_EXT")
            }
            Self::PAST_PRESENTATION_TIMING_EXT => write!(f, "PAST_PRESENTATION_TIMING_EXT"),
            Self::PRESENT_TIMING_SURFACE_CAPABILITIES_EXT => {
                write!(f, "PRESENT_TIMING_SURFACE_CAPABILITIES_EXT")
            }
            Self::SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT => {
                write!(f, "SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL => write!(
                f,
                "PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL"
            ),
            Self::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL => {
                write!(f, "QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL")
            }
            Self::INITIALIZE_PERFORMANCE_API_INFO_INTEL => {
                write!(f, "INITIALIZE_PERFORMANCE_API_INFO_INTEL")
            }
            Self::PERFORMANCE_MARKER_INFO_INTEL => write!(f, "PERFORMANCE_MARKER_INFO_INTEL"),
            Self::PERFORMANCE_STREAM_MARKER_INFO_INTEL => {
                write!(f, "PERFORMANCE_STREAM_MARKER_INFO_INTEL")
            }
            Self::PERFORMANCE_OVERRIDE_INFO_INTEL => write!(f, "PERFORMANCE_OVERRIDE_INFO_INTEL"),
            Self::PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL => {
                write!(f, "PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL")
            }
            Self::PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT")
            }
            Self::DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD => {
                write!(f, "DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD")
            }
            Self::SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD => {
                write!(f, "SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD")
            }
            Self::IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA => {
                write!(f, "IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA")
            }
            Self::METAL_SURFACE_CREATE_INFO_EXT => write!(f, "METAL_SURFACE_CREATE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT")
            }
            Self::RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT => {
                write!(f, "RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT")
            }
            Self::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT => {
                write!(f, "RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT")
            }
            Self::FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR => {
                write!(f, "FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR")
            }
            Self::PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR => {
                write!(f, "PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR => {
                write!(f, "PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR")
            }
            Self::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR => {
                write!(f, "RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD => {
                write!(f, "PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD")
            }
            Self::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD => {
                write!(f, "PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD")
            }
            Self::PHYSICAL_DEVICE_SHADER_CONSTANT_DATA_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_SHADER_CONSTANT_DATA_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_ABORT_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_SHADER_ABORT_FEATURES_KHR")
            }
            Self::DEVICE_FAULT_SHADER_ABORT_MESSAGE_INFO_KHR => {
                write!(f, "DEVICE_FAULT_SHADER_ABORT_MESSAGE_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_ABORT_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_SHADER_ABORT_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT")
            }
            Self::MEMORY_PRIORITY_ALLOCATE_INFO_EXT => {
                write!(f, "MEMORY_PRIORITY_ALLOCATE_INFO_EXT")
            }
            Self::SURFACE_PROTECTED_CAPABILITIES_KHR => {
                write!(f, "SURFACE_PROTECTED_CAPABILITIES_KHR")
            }
            Self::PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV => write!(
                f,
                "PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV"
            ),
            Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT")
            }
            Self::BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT => {
                write!(f, "BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT")
            }
            Self::VALIDATION_FEATURES_EXT => write!(f, "VALIDATION_FEATURES_EXT"),
            Self::PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV")
            }
            Self::COOPERATIVE_MATRIX_PROPERTIES_NV => write!(f, "COOPERATIVE_MATRIX_PROPERTIES_NV"),
            Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV => {
                write!(f, "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV")
            }
            Self::PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV => {
                write!(f, "PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV")
            }
            Self::FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV => {
                write!(f, "FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT")
            }
            Self::PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT => write!(
                f,
                "PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT"
            ),
            Self::PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT")
            }
            Self::SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT => {
                write!(f, "SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT")
            }
            Self::SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT => {
                write!(f, "SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT")
            }
            Self::SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT => {
                write!(f, "SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT")
            }
            Self::HEADLESS_SURFACE_CREATE_INFO_EXT => write!(f, "HEADLESS_SURFACE_CREATE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR => write!(
                f,
                "PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR"
            ),
            Self::PIPELINE_INFO_KHR => write!(f, "PIPELINE_INFO_KHR"),
            Self::PIPELINE_EXECUTABLE_PROPERTIES_KHR => {
                write!(f, "PIPELINE_EXECUTABLE_PROPERTIES_KHR")
            }
            Self::PIPELINE_EXECUTABLE_INFO_KHR => write!(f, "PIPELINE_EXECUTABLE_INFO_KHR"),
            Self::PIPELINE_EXECUTABLE_STATISTIC_KHR => {
                write!(f, "PIPELINE_EXECUTABLE_STATISTIC_KHR")
            }
            Self::PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR => {
                write!(f, "PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR")
            }
            Self::PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT")
            }
            Self::MEMORY_MAP_PLACED_INFO_EXT => write!(f, "MEMORY_MAP_PLACED_INFO_EXT"),
            Self::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV => {
                write!(f, "PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV")
            }
            Self::GRAPHICS_SHADER_GROUP_CREATE_INFO_NV => {
                write!(f, "GRAPHICS_SHADER_GROUP_CREATE_INFO_NV")
            }
            Self::GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV => {
                write!(f, "GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV")
            }
            Self::INDIRECT_COMMANDS_LAYOUT_TOKEN_NV => {
                write!(f, "INDIRECT_COMMANDS_LAYOUT_TOKEN_NV")
            }
            Self::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV => {
                write!(f, "INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV")
            }
            Self::GENERATED_COMMANDS_INFO_NV => write!(f, "GENERATED_COMMANDS_INFO_NV"),
            Self::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV => {
                write!(f, "GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV")
            }
            Self::COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV => {
                write!(f, "COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT")
            }
            Self::COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM => write!(
                f,
                "COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM"
            ),
            Self::RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM => {
                write!(f, "RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM")
            }
            Self::PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT")
            }
            Self::DEPTH_BIAS_INFO_EXT => write!(f, "DEPTH_BIAS_INFO_EXT"),
            Self::DEPTH_BIAS_REPRESENTATION_INFO_EXT => {
                write!(f, "DEPTH_BIAS_REPRESENTATION_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT")
            }
            Self::DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT => {
                write!(f, "DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT")
            }
            Self::DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT => {
                write!(f, "DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT")
            }
            Self::SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT => {
                write!(f, "SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_3D_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_3D_FEATURES_EXT"
            ),
            Self::PIPELINE_LIBRARY_CREATE_INFO_KHR => write!(f, "PIPELINE_LIBRARY_CREATE_INFO_KHR"),
            Self::PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV")
            }
            Self::SURFACE_CAPABILITIES_PRESENT_BARRIER_NV => {
                write!(f, "SURFACE_CAPABILITIES_PRESENT_BARRIER_NV")
            }
            Self::SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV => {
                write!(f, "SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV")
            }
            Self::PRESENT_ID_KHR => write!(f, "PRESENT_ID_KHR"),
            Self::PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR")
            }
            Self::VIDEO_ENCODE_INFO_KHR => write!(f, "VIDEO_ENCODE_INFO_KHR"),
            Self::VIDEO_ENCODE_RATE_CONTROL_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_RATE_CONTROL_INFO_KHR")
            }
            Self::VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR")
            }
            Self::VIDEO_ENCODE_CAPABILITIES_KHR => write!(f, "VIDEO_ENCODE_CAPABILITIES_KHR"),
            Self::VIDEO_ENCODE_USAGE_INFO_KHR => write!(f, "VIDEO_ENCODE_USAGE_INFO_KHR"),
            Self::QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR => {
                write!(f, "QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR => {
                write!(f, "PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR")
            }
            Self::VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR => {
                write!(f, "VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR")
            }
            Self::VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR")
            }
            Self::VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR")
            }
            Self::VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV")
            }
            Self::DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV => {
                write!(f, "DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV")
            }
            Self::PERF_HINT_INFO_QCOM => write!(f, "PERF_HINT_INFO_QCOM"),
            Self::PHYSICAL_DEVICE_QUEUE_PERF_HINT_FEATURES_QCOM => {
                write!(f, "PHYSICAL_DEVICE_QUEUE_PERF_HINT_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_QUEUE_PERF_HINT_PROPERTIES_QCOM => {
                write!(f, "PHYSICAL_DEVICE_QUEUE_PERF_HINT_PROPERTIES_QCOM")
            }
            Self::CUDA_MODULE_CREATE_INFO_NV => write!(f, "CUDA_MODULE_CREATE_INFO_NV"),
            Self::CUDA_FUNCTION_CREATE_INFO_NV => write!(f, "CUDA_FUNCTION_CREATE_INFO_NV"),
            Self::CUDA_LAUNCH_INFO_NV => write!(f, "CUDA_LAUNCH_INFO_NV"),
            Self::PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV => {
                write!(f, "PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV")
            }
            Self::REFRESH_OBJECT_LIST_KHR => write!(f, "REFRESH_OBJECT_LIST_KHR"),
            Self::PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM => {
                write!(f, "PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM => {
                write!(f, "PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM")
            }
            Self::RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM => {
                write!(f, "RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM")
            }
            Self::PER_TILE_BEGIN_INFO_QCOM => write!(f, "PER_TILE_BEGIN_INFO_QCOM"),
            Self::PER_TILE_END_INFO_QCOM => write!(f, "PER_TILE_END_INFO_QCOM"),
            Self::DISPATCH_TILE_INFO_QCOM => write!(f, "DISPATCH_TILE_INFO_QCOM"),
            Self::QUERY_LOW_LATENCY_SUPPORT_NV => write!(f, "QUERY_LOW_LATENCY_SUPPORT_NV"),
            Self::EXPORT_METAL_OBJECT_CREATE_INFO_EXT => {
                write!(f, "EXPORT_METAL_OBJECT_CREATE_INFO_EXT")
            }
            Self::EXPORT_METAL_OBJECTS_INFO_EXT => write!(f, "EXPORT_METAL_OBJECTS_INFO_EXT"),
            Self::EXPORT_METAL_DEVICE_INFO_EXT => write!(f, "EXPORT_METAL_DEVICE_INFO_EXT"),
            Self::EXPORT_METAL_COMMAND_QUEUE_INFO_EXT => {
                write!(f, "EXPORT_METAL_COMMAND_QUEUE_INFO_EXT")
            }
            Self::EXPORT_METAL_BUFFER_INFO_EXT => write!(f, "EXPORT_METAL_BUFFER_INFO_EXT"),
            Self::IMPORT_METAL_BUFFER_INFO_EXT => write!(f, "IMPORT_METAL_BUFFER_INFO_EXT"),
            Self::EXPORT_METAL_TEXTURE_INFO_EXT => write!(f, "EXPORT_METAL_TEXTURE_INFO_EXT"),
            Self::IMPORT_METAL_TEXTURE_INFO_EXT => write!(f, "IMPORT_METAL_TEXTURE_INFO_EXT"),
            Self::EXPORT_METAL_IO_SURFACE_INFO_EXT => write!(f, "EXPORT_METAL_IO_SURFACE_INFO_EXT"),
            Self::IMPORT_METAL_IO_SURFACE_INFO_EXT => write!(f, "IMPORT_METAL_IO_SURFACE_INFO_EXT"),
            Self::EXPORT_METAL_SHARED_EVENT_INFO_EXT => {
                write!(f, "EXPORT_METAL_SHARED_EVENT_INFO_EXT")
            }
            Self::IMPORT_METAL_SHARED_EVENT_INFO_EXT => {
                write!(f, "IMPORT_METAL_SHARED_EVENT_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT"
            ),
            Self::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT")
            }
            Self::DESCRIPTOR_ADDRESS_INFO_EXT => write!(f, "DESCRIPTOR_ADDRESS_INFO_EXT"),
            Self::DESCRIPTOR_GET_INFO_EXT => write!(f, "DESCRIPTOR_GET_INFO_EXT"),
            Self::BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => {
                write!(f, "BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT")
            }
            Self::IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => {
                write!(f, "IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT")
            }
            Self::IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => {
                write!(f, "IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT")
            }
            Self::SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => {
                write!(f, "SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT")
            }
            Self::OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT => {
                write!(f, "OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT")
            }
            Self::DESCRIPTOR_BUFFER_BINDING_INFO_EXT => {
                write!(f, "DESCRIPTOR_BUFFER_BINDING_INFO_EXT")
            }
            Self::DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT => write!(
                f,
                "DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT"
            ),
            Self::ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => {
                write!(f, "ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT")
            }
            Self::DEVICE_MEMORY_COPY_KHR => write!(f, "DEVICE_MEMORY_COPY_KHR"),
            Self::COPY_DEVICE_MEMORY_INFO_KHR => write!(f, "COPY_DEVICE_MEMORY_INFO_KHR"),
            Self::DEVICE_MEMORY_IMAGE_COPY_KHR => write!(f, "DEVICE_MEMORY_IMAGE_COPY_KHR"),
            Self::COPY_DEVICE_MEMORY_IMAGE_INFO_KHR => {
                write!(f, "COPY_DEVICE_MEMORY_IMAGE_INFO_KHR")
            }
            Self::MEMORY_RANGE_BARRIERS_INFO_KHR => write!(f, "MEMORY_RANGE_BARRIERS_INFO_KHR"),
            Self::MEMORY_RANGE_BARRIER_KHR => write!(f, "MEMORY_RANGE_BARRIER_KHR"),
            Self::PHYSICAL_DEVICE_DEVICE_ADDRESS_COMMANDS_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_DEVICE_ADDRESS_COMMANDS_FEATURES_KHR")
            }
            Self::BIND_INDEX_BUFFER_3_INFO_KHR => write!(f, "BIND_INDEX_BUFFER_3_INFO_KHR"),
            Self::BIND_VERTEX_BUFFER_3_INFO_KHR => write!(f, "BIND_VERTEX_BUFFER_3_INFO_KHR"),
            Self::DRAW_INDIRECT_2_INFO_KHR => write!(f, "DRAW_INDIRECT_2_INFO_KHR"),
            Self::DRAW_INDIRECT_COUNT_2_INFO_KHR => write!(f, "DRAW_INDIRECT_COUNT_2_INFO_KHR"),
            Self::DISPATCH_INDIRECT_2_INFO_KHR => write!(f, "DISPATCH_INDIRECT_2_INFO_KHR"),
            Self::CONDITIONAL_RENDERING_BEGIN_INFO_2_EXT => {
                write!(f, "CONDITIONAL_RENDERING_BEGIN_INFO_2_EXT")
            }
            Self::BIND_TRANSFORM_FEEDBACK_BUFFER_2_INFO_EXT => {
                write!(f, "BIND_TRANSFORM_FEEDBACK_BUFFER_2_INFO_EXT")
            }
            Self::MEMORY_MARKER_INFO_AMD => write!(f, "MEMORY_MARKER_INFO_AMD"),
            Self::ACCELERATION_STRUCTURE_CREATE_INFO_2_KHR => {
                write!(f, "ACCELERATION_STRUCTURE_CREATE_INFO_2_KHR")
            }
            Self::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT"
            ),
            Self::GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT => {
                write!(f, "GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD => write!(
                f,
                "PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD"
            ),
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR => write!(
                f,
                "PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR"
            ),
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR => write!(
                f,
                "PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR"
            ),
            Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR => write!(
                f,
                "PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR"
            ),
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV => write!(
                f,
                "PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV"
            ),
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV")
            }
            Self::PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV => write!(
                f,
                "PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV"
            ),
            Self::ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV => write!(
                f,
                "ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV"
            ),
            Self::PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV")
            }
            Self::ACCELERATION_STRUCTURE_MOTION_INFO_NV => {
                write!(f, "ACCELERATION_STRUCTURE_MOTION_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT")
            }
            Self::COPY_COMMAND_TRANSFORM_INFO_QCOM => write!(f, "COPY_COMMAND_TRANSFORM_INFO_QCOM"),
            Self::PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR => write!(
                f,
                "PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR"
            ),
            Self::PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT")
            }
            Self::IMAGE_COMPRESSION_CONTROL_EXT => write!(f, "IMAGE_COMPRESSION_CONTROL_EXT"),
            Self::IMAGE_COMPRESSION_PROPERTIES_EXT => write!(f, "IMAGE_COMPRESSION_PROPERTIES_EXT"),
            Self::PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT"
            ),
            Self::PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_FAULT_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_FAULT_FEATURES_EXT")
            }
            Self::DEVICE_FAULT_COUNTS_EXT => write!(f, "DEVICE_FAULT_COUNTS_EXT"),
            Self::DEVICE_FAULT_INFO_EXT => write!(f, "DEVICE_FAULT_INFO_EXT"),
            Self::PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT")
            }
            Self::DIRECTFB_SURFACE_CREATE_INFO_EXT => write!(f, "DIRECTFB_SURFACE_CREATE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT")
            }
            Self::VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT => {
                write!(f, "VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT")
            }
            Self::VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT => {
                write!(f, "VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT")
            }
            Self::PHYSICAL_DEVICE_DRM_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_DRM_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT")
            }
            Self::DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT => {
                write!(f, "DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT")
            }
            Self::PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT")
            }
            Self::PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT => {
                write!(f, "PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT"
            ),
            Self::IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA => {
                write!(f, "IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA")
            }
            Self::MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA => {
                write!(f, "MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA")
            }
            Self::MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA => {
                write!(f, "MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA")
            }
            Self::IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA => {
                write!(f, "IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA")
            }
            Self::SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA => {
                write!(f, "SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA")
            }
            Self::BUFFER_COLLECTION_CREATE_INFO_FUCHSIA => {
                write!(f, "BUFFER_COLLECTION_CREATE_INFO_FUCHSIA")
            }
            Self::IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA => {
                write!(f, "IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA")
            }
            Self::BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA => {
                write!(f, "BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA")
            }
            Self::BUFFER_COLLECTION_PROPERTIES_FUCHSIA => {
                write!(f, "BUFFER_COLLECTION_PROPERTIES_FUCHSIA")
            }
            Self::BUFFER_CONSTRAINTS_INFO_FUCHSIA => write!(f, "BUFFER_CONSTRAINTS_INFO_FUCHSIA"),
            Self::BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA => {
                write!(f, "BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA")
            }
            Self::IMAGE_CONSTRAINTS_INFO_FUCHSIA => write!(f, "IMAGE_CONSTRAINTS_INFO_FUCHSIA"),
            Self::IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA => {
                write!(f, "IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA")
            }
            Self::SYSMEM_COLOR_SPACE_FUCHSIA => write!(f, "SYSMEM_COLOR_SPACE_FUCHSIA"),
            Self::BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA => {
                write!(f, "BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA")
            }
            Self::SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI => {
                write!(f, "SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI")
            }
            Self::PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI => {
                write!(f, "PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI")
            }
            Self::PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI => {
                write!(f, "PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI")
            }
            Self::PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI => {
                write!(f, "PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI")
            }
            Self::MEMORY_GET_REMOTE_ADDRESS_INFO_NV => {
                write!(f, "MEMORY_GET_REMOTE_ADDRESS_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV")
            }
            Self::PIPELINE_PROPERTIES_IDENTIFIER_EXT => {
                write!(f, "PIPELINE_PROPERTIES_IDENTIFIER_EXT")
            }
            Self::PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT")
            }
            Self::IMPORT_FENCE_SCI_SYNC_INFO_NV => write!(f, "IMPORT_FENCE_SCI_SYNC_INFO_NV"),
            Self::EXPORT_FENCE_SCI_SYNC_INFO_NV => write!(f, "EXPORT_FENCE_SCI_SYNC_INFO_NV"),
            Self::FENCE_GET_SCI_SYNC_INFO_NV => write!(f, "FENCE_GET_SCI_SYNC_INFO_NV"),
            Self::SCI_SYNC_ATTRIBUTES_INFO_NV => write!(f, "SCI_SYNC_ATTRIBUTES_INFO_NV"),
            Self::IMPORT_SEMAPHORE_SCI_SYNC_INFO_NV => {
                write!(f, "IMPORT_SEMAPHORE_SCI_SYNC_INFO_NV")
            }
            Self::EXPORT_SEMAPHORE_SCI_SYNC_INFO_NV => {
                write!(f, "EXPORT_SEMAPHORE_SCI_SYNC_INFO_NV")
            }
            Self::SEMAPHORE_GET_SCI_SYNC_INFO_NV => write!(f, "SEMAPHORE_GET_SCI_SYNC_INFO_NV"),
            Self::PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_FEATURES_NV")
            }
            Self::IMPORT_MEMORY_SCI_BUF_INFO_NV => write!(f, "IMPORT_MEMORY_SCI_BUF_INFO_NV"),
            Self::EXPORT_MEMORY_SCI_BUF_INFO_NV => write!(f, "EXPORT_MEMORY_SCI_BUF_INFO_NV"),
            Self::MEMORY_GET_SCI_BUF_INFO_NV => write!(f, "MEMORY_GET_SCI_BUF_INFO_NV"),
            Self::MEMORY_SCI_BUF_PROPERTIES_NV => write!(f, "MEMORY_SCI_BUF_PROPERTIES_NV"),
            Self::PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT")
            }
            Self::FRAME_BOUNDARY_EXT => write!(f, "FRAME_BOUNDARY_EXT"),
            Self::PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT"
            ),
            Self::SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT => {
                write!(f, "SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT")
            }
            Self::MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT => {
                write!(f, "MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT")
            }
            Self::SCREEN_SURFACE_CREATE_INFO_QNX => write!(f, "SCREEN_SURFACE_CREATE_INFO_QNX"),
            Self::PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT")
            }
            Self::PIPELINE_COLOR_WRITE_CREATE_INFO_EXT => {
                write!(f, "PIPELINE_COLOR_WRITE_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_UNTYPED_POINTERS_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_SHADER_UNTYPED_POINTERS_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE => write!(
                f,
                "PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE"
            ),
            Self::VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE => {
                write!(f, "VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE")
            }
            Self::VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE => {
                write!(f, "VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE")
            }
            Self::VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE => {
                write!(f, "VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE")
            }
            Self::PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT")
            }
            Self::IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT => {
                write!(f, "IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT")
            }
            Self::MICROMAP_BUILD_INFO_EXT => write!(f, "MICROMAP_BUILD_INFO_EXT"),
            Self::MICROMAP_VERSION_INFO_EXT => write!(f, "MICROMAP_VERSION_INFO_EXT"),
            Self::COPY_MICROMAP_INFO_EXT => write!(f, "COPY_MICROMAP_INFO_EXT"),
            Self::COPY_MICROMAP_TO_MEMORY_INFO_EXT => write!(f, "COPY_MICROMAP_TO_MEMORY_INFO_EXT"),
            Self::COPY_MEMORY_TO_MICROMAP_INFO_EXT => write!(f, "COPY_MEMORY_TO_MICROMAP_INFO_EXT"),
            Self::PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT")
            }
            Self::MICROMAP_CREATE_INFO_EXT => write!(f, "MICROMAP_CREATE_INFO_EXT"),
            Self::MICROMAP_BUILD_SIZES_INFO_EXT => write!(f, "MICROMAP_BUILD_SIZES_INFO_EXT"),
            Self::ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT => {
                write!(f, "ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT")
            }
            Self::PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV => {
                write!(f, "PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV")
            }
            Self::ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV => write!(
                f,
                "ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV"
            ),
            Self::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI => {
                write!(f, "PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI")
            }
            Self::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI => write!(
                f,
                "PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI"
            ),
            Self::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI => write!(
                f,
                "PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI"
            ),
            Self::PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT")
            }
            Self::SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT => {
                write!(f, "SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT"
            ),
            Self::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM => {
                write!(f, "PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM")
            }
            Self::DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM => {
                write!(f, "DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM")
            }
            Self::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM => {
                write!(f, "PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM")
            }
            Self::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM => {
                write!(f, "PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM")
            }
            Self::DISPATCH_PARAMETERS_ARM => write!(f, "DISPATCH_PARAMETERS_ARM"),
            Self::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_PROPERTIES_ARM => write!(
                f,
                "PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_PROPERTIES_ARM"
            ),
            Self::PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT")
            }
            Self::IMAGE_VIEW_SLICED_CREATE_INFO_EXT => {
                write!(f, "IMAGE_VIEW_SLICED_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE => write!(
                f,
                "PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE"
            ),
            Self::DESCRIPTOR_SET_BINDING_REFERENCE_VALVE => {
                write!(f, "DESCRIPTOR_SET_BINDING_REFERENCE_VALVE")
            }
            Self::DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE => {
                write!(f, "DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE")
            }
            Self::PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM => {
                write!(f, "PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM")
            }
            Self::PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM => {
                write!(f, "PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM")
            }
            Self::RENDER_PASS_STRIPE_BEGIN_INFO_ARM => {
                write!(f, "RENDER_PASS_STRIPE_BEGIN_INFO_ARM")
            }
            Self::RENDER_PASS_STRIPE_INFO_ARM => write!(f, "RENDER_PASS_STRIPE_INFO_ARM"),
            Self::RENDER_PASS_STRIPE_SUBMIT_INFO_ARM => {
                write!(f, "RENDER_PASS_STRIPE_SUBMIT_INFO_ARM")
            }
            Self::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV => write!(
                f,
                "PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV"
            ),
            Self::COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV => {
                write!(f, "COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV")
            }
            Self::PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV => {
                write!(f, "PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV => write!(
                f,
                "PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV"
            ),
            Self::ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV => write!(
                f,
                "ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV"
            ),
            Self::ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV => {
                write!(f, "ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV")
            }
            Self::PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR => write!(
                f,
                "PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR"
            ),
            Self::APPLICATION_PARAMETERS_EXT => write!(f, "APPLICATION_PARAMETERS_EXT"),
            Self::PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT"
            ),
            Self::PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM => {
                write!(f, "PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM => {
                write!(f, "PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM")
            }
            Self::IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM => {
                write!(f, "IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM")
            }
            Self::PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT")
            }
            Self::NATIVE_BUFFER_USAGE_OHOS => write!(f, "NATIVE_BUFFER_USAGE_OHOS"),
            Self::NATIVE_BUFFER_PROPERTIES_OHOS => write!(f, "NATIVE_BUFFER_PROPERTIES_OHOS"),
            Self::NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS => {
                write!(f, "NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS")
            }
            Self::IMPORT_NATIVE_BUFFER_INFO_OHOS => write!(f, "IMPORT_NATIVE_BUFFER_INFO_OHOS"),
            Self::MEMORY_GET_NATIVE_BUFFER_INFO_OHOS => {
                write!(f, "MEMORY_GET_NATIVE_BUFFER_INFO_OHOS")
            }
            Self::EXTERNAL_FORMAT_OHOS => write!(f, "EXTERNAL_FORMAT_OHOS"),
            Self::EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT => {
                write!(f, "EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT")
            }
            Self::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT")
            }
            Self::RENDER_PASS_CREATION_CONTROL_EXT => write!(f, "RENDER_PASS_CREATION_CONTROL_EXT"),
            Self::RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT => {
                write!(f, "RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT")
            }
            Self::RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT => {
                write!(f, "RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT")
            }
            Self::DIRECT_DRIVER_LOADING_INFO_LUNARG => {
                write!(f, "DIRECT_DRIVER_LOADING_INFO_LUNARG")
            }
            Self::DIRECT_DRIVER_LOADING_LIST_LUNARG => {
                write!(f, "DIRECT_DRIVER_LOADING_LIST_LUNARG")
            }
            Self::TENSOR_CREATE_INFO_ARM => write!(f, "TENSOR_CREATE_INFO_ARM"),
            Self::TENSOR_VIEW_CREATE_INFO_ARM => write!(f, "TENSOR_VIEW_CREATE_INFO_ARM"),
            Self::BIND_TENSOR_MEMORY_INFO_ARM => write!(f, "BIND_TENSOR_MEMORY_INFO_ARM"),
            Self::WRITE_DESCRIPTOR_SET_TENSOR_ARM => write!(f, "WRITE_DESCRIPTOR_SET_TENSOR_ARM"),
            Self::PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM => {
                write!(f, "PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM")
            }
            Self::TENSOR_FORMAT_PROPERTIES_ARM => write!(f, "TENSOR_FORMAT_PROPERTIES_ARM"),
            Self::TENSOR_DESCRIPTION_ARM => write!(f, "TENSOR_DESCRIPTION_ARM"),
            Self::TENSOR_MEMORY_REQUIREMENTS_INFO_ARM => {
                write!(f, "TENSOR_MEMORY_REQUIREMENTS_INFO_ARM")
            }
            Self::TENSOR_MEMORY_BARRIER_ARM => write!(f, "TENSOR_MEMORY_BARRIER_ARM"),
            Self::PHYSICAL_DEVICE_TENSOR_FEATURES_ARM => {
                write!(f, "PHYSICAL_DEVICE_TENSOR_FEATURES_ARM")
            }
            Self::DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM => {
                write!(f, "DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM")
            }
            Self::COPY_TENSOR_INFO_ARM => write!(f, "COPY_TENSOR_INFO_ARM"),
            Self::TENSOR_COPY_ARM => write!(f, "TENSOR_COPY_ARM"),
            Self::TENSOR_DEPENDENCY_INFO_ARM => write!(f, "TENSOR_DEPENDENCY_INFO_ARM"),
            Self::MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM => {
                write!(f, "MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM")
            }
            Self::PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM => {
                write!(f, "PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM")
            }
            Self::EXTERNAL_TENSOR_PROPERTIES_ARM => write!(f, "EXTERNAL_TENSOR_PROPERTIES_ARM"),
            Self::EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM => {
                write!(f, "EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM => {
                write!(f, "PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM => {
                write!(f, "PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM")
            }
            Self::DESCRIPTOR_GET_TENSOR_INFO_ARM => write!(f, "DESCRIPTOR_GET_TENSOR_INFO_ARM"),
            Self::TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM => {
                write!(f, "TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM")
            }
            Self::TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM => {
                write!(f, "TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM")
            }
            Self::FRAME_BOUNDARY_TENSORS_ARM => write!(f, "FRAME_BOUNDARY_TENSORS_ARM"),
            Self::PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT")
            }
            Self::PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT => {
                write!(f, "PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT")
            }
            Self::SHADER_MODULE_IDENTIFIER_EXT => write!(f, "SHADER_MODULE_IDENTIFIER_EXT"),
            Self::PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT"
            ),
            Self::PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV => {
                write!(f, "PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV")
            }
            Self::OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV => {
                write!(f, "OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV")
            }
            Self::OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV => {
                write!(f, "OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV")
            }
            Self::OPTICAL_FLOW_SESSION_CREATE_INFO_NV => {
                write!(f, "OPTICAL_FLOW_SESSION_CREATE_INFO_NV")
            }
            Self::OPTICAL_FLOW_EXECUTE_INFO_NV => write!(f, "OPTICAL_FLOW_EXECUTE_INFO_NV"),
            Self::OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV => {
                write!(f, "OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID => write!(
                f,
                "PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID"
            ),
            Self::PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID => write!(
                f,
                "PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID"
            ),
            Self::ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID => write!(
                f,
                "ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID"
            ),
            Self::PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD => {
                write!(f, "PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD")
            }
            Self::ANTI_LAG_DATA_AMD => write!(f, "ANTI_LAG_DATA_AMD"),
            Self::ANTI_LAG_PRESENTATION_INFO_AMD => write!(f, "ANTI_LAG_PRESENTATION_INFO_AMD"),
            Self::PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX => {
                write!(f, "PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX")
            }
            Self::ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX => write!(
                f,
                "ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX"
            ),
            Self::SURFACE_CAPABILITIES_PRESENT_ID_2_KHR => {
                write!(f, "SURFACE_CAPABILITIES_PRESENT_ID_2_KHR")
            }
            Self::PRESENT_ID_2_KHR => write!(f, "PRESENT_ID_2_KHR"),
            Self::PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR")
            }
            Self::SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR => {
                write!(f, "SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR")
            }
            Self::PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR")
            }
            Self::PRESENT_WAIT_2_INFO_KHR => write!(f, "PRESENT_WAIT_2_INFO_KHR"),
            Self::PHYSICAL_DEVICE_RAY_TRACING_POSITION_FETCH_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_RAY_TRACING_POSITION_FETCH_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT")
            }
            Self::SHADER_CREATE_INFO_EXT => write!(f, "SHADER_CREATE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR")
            }
            Self::PIPELINE_BINARY_CREATE_INFO_KHR => write!(f, "PIPELINE_BINARY_CREATE_INFO_KHR"),
            Self::PIPELINE_BINARY_INFO_KHR => write!(f, "PIPELINE_BINARY_INFO_KHR"),
            Self::PIPELINE_BINARY_KEY_KHR => write!(f, "PIPELINE_BINARY_KEY_KHR"),
            Self::PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR")
            }
            Self::RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR => {
                write!(f, "RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR")
            }
            Self::PIPELINE_BINARY_DATA_INFO_KHR => write!(f, "PIPELINE_BINARY_DATA_INFO_KHR"),
            Self::PIPELINE_CREATE_INFO_KHR => write!(f, "PIPELINE_CREATE_INFO_KHR"),
            Self::DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR => {
                write!(f, "DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR")
            }
            Self::PIPELINE_BINARY_HANDLES_INFO_KHR => write!(f, "PIPELINE_BINARY_HANDLES_INFO_KHR"),
            Self::PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM => {
                write!(f, "PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM")
            }
            Self::TILE_PROPERTIES_QCOM => write!(f, "TILE_PROPERTIES_QCOM"),
            Self::PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC => {
                write!(f, "PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC")
            }
            Self::AMIGO_PROFILING_SUBMIT_INFO_SEC => write!(f, "AMIGO_PROFILING_SUBMIT_INFO_SEC"),
            Self::SURFACE_PRESENT_MODE_KHR => write!(f, "SURFACE_PRESENT_MODE_KHR"),
            Self::SURFACE_PRESENT_SCALING_CAPABILITIES_KHR => {
                write!(f, "SURFACE_PRESENT_SCALING_CAPABILITIES_KHR")
            }
            Self::SURFACE_PRESENT_MODE_COMPATIBILITY_KHR => {
                write!(f, "SURFACE_PRESENT_MODE_COMPATIBILITY_KHR")
            }
            Self::PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR")
            }
            Self::SWAPCHAIN_PRESENT_FENCE_INFO_KHR => write!(f, "SWAPCHAIN_PRESENT_FENCE_INFO_KHR"),
            Self::SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR => {
                write!(f, "SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR")
            }
            Self::SWAPCHAIN_PRESENT_MODE_INFO_KHR => write!(f, "SWAPCHAIN_PRESENT_MODE_INFO_KHR"),
            Self::SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR => {
                write!(f, "SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR")
            }
            Self::RELEASE_SWAPCHAIN_IMAGES_INFO_KHR => {
                write!(f, "RELEASE_SWAPCHAIN_IMAGES_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM => write!(
                f,
                "PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM"
            ),
            Self::SEMAPHORE_SCI_SYNC_POOL_CREATE_INFO_NV => {
                write!(f, "SEMAPHORE_SCI_SYNC_POOL_CREATE_INFO_NV")
            }
            Self::SEMAPHORE_SCI_SYNC_CREATE_INFO_NV => {
                write!(f, "SEMAPHORE_SCI_SYNC_CREATE_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_2_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_2_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV => write!(
                f,
                "PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV"
            ),
            Self::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV => write!(
                f,
                "PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV"
            ),
            Self::PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV => {
                write!(f, "PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV")
            }
            Self::COOPERATIVE_VECTOR_PROPERTIES_NV => write!(f, "COOPERATIVE_VECTOR_PROPERTIES_NV"),
            Self::CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV => {
                write!(f, "CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV => write!(
                f,
                "PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV"
            ),
            Self::PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV => write!(
                f,
                "PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV"
            ),
            Self::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT")
            }
            Self::MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT => {
                write!(f, "MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_PROPERTIES_EXT")
            }
            Self::LAYER_SETTINGS_CREATE_INFO_EXT => write!(f, "LAYER_SETTINGS_CREATE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM => {
                write!(f, "PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM")
            }
            Self::PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM => {
                write!(f, "PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM")
            }
            Self::PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT"
            ),
            Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT"
            ),
            Self::PHYSICAL_DEVICE_INTERNALLY_SYNCHRONIZED_QUEUES_FEATURES_KHR => write!(
                f,
                "PHYSICAL_DEVICE_INTERNALLY_SYNCHRONIZED_QUEUES_FEATURES_KHR"
            ),
            Self::LATENCY_SLEEP_MODE_INFO_NV => write!(f, "LATENCY_SLEEP_MODE_INFO_NV"),
            Self::LATENCY_SLEEP_INFO_NV => write!(f, "LATENCY_SLEEP_INFO_NV"),
            Self::SET_LATENCY_MARKER_INFO_NV => write!(f, "SET_LATENCY_MARKER_INFO_NV"),
            Self::GET_LATENCY_MARKER_INFO_NV => write!(f, "GET_LATENCY_MARKER_INFO_NV"),
            Self::LATENCY_TIMINGS_FRAME_REPORT_NV => write!(f, "LATENCY_TIMINGS_FRAME_REPORT_NV"),
            Self::LATENCY_SUBMISSION_PRESENT_ID_NV => write!(f, "LATENCY_SUBMISSION_PRESENT_ID_NV"),
            Self::OUT_OF_BAND_QUEUE_TYPE_INFO_NV => write!(f, "OUT_OF_BAND_QUEUE_TYPE_INFO_NV"),
            Self::SWAPCHAIN_LATENCY_CREATE_INFO_NV => write!(f, "SWAPCHAIN_LATENCY_CREATE_INFO_NV"),
            Self::LATENCY_SURFACE_CAPABILITIES_NV => write!(f, "LATENCY_SURFACE_CAPABILITIES_NV"),
            Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR")
            }
            Self::COOPERATIVE_MATRIX_PROPERTIES_KHR => {
                write!(f, "COOPERATIVE_MATRIX_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR")
            }
            Self::DATA_GRAPH_PIPELINE_CREATE_INFO_ARM => {
                write!(f, "DATA_GRAPH_PIPELINE_CREATE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM => {
                write!(f, "DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM => {
                write!(f, "DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_CONSTANT_ARM => write!(f, "DATA_GRAPH_PIPELINE_CONSTANT_ARM"),
            Self::DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM => write!(
                f,
                "DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM"
            ),
            Self::BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM => {
                write!(f, "BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM")
            }
            Self::PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM => {
                write!(f, "PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM => {
                write!(f, "DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM => {
                write!(f, "DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_INFO_ARM => write!(f, "DATA_GRAPH_PIPELINE_INFO_ARM"),
            Self::DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM => {
                write!(f, "DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM => write!(
                f,
                "DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM"
            ),
            Self::DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM => {
                write!(f, "DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM => {
                write!(f, "DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM => {
                write!(f, "DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM")
            }
            Self::DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM => {
                write!(f, "DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM")
            }
            Self::QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM => write!(
                f,
                "QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM"
            ),
            Self::QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM => {
                write!(f, "QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM")
            }
            Self::PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM => write!(
                f,
                "PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM"
            ),
            Self::DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM => write!(
                f,
                "DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM"
            ),
            Self::QUEUE_FAMILY_DATA_GRAPH_TOSA_PROPERTIES_ARM => {
                write!(f, "QUEUE_FAMILY_DATA_GRAPH_TOSA_PROPERTIES_ARM")
            }
            Self::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM => write!(
                f,
                "PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM"
            ),
            Self::MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM => write!(
                f,
                "MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM"
            ),
            Self::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_PROPERTIES_KHR => write!(
                f,
                "PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_PROPERTIES_KHR"
            ),
            Self::VIDEO_DECODE_AV1_CAPABILITIES_KHR => {
                write!(f, "VIDEO_DECODE_AV1_CAPABILITIES_KHR")
            }
            Self::VIDEO_DECODE_AV1_PICTURE_INFO_KHR => {
                write!(f, "VIDEO_DECODE_AV1_PICTURE_INFO_KHR")
            }
            Self::VIDEO_DECODE_AV1_PROFILE_INFO_KHR => {
                write!(f, "VIDEO_DECODE_AV1_PROFILE_INFO_KHR")
            }
            Self::VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                write!(f, "VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR")
            }
            Self::VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR => {
                write!(f, "VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR")
            }
            Self::VIDEO_ENCODE_AV1_CAPABILITIES_KHR => {
                write!(f, "VIDEO_ENCODE_AV1_CAPABILITIES_KHR")
            }
            Self::VIDEO_ENCODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_AV1_PICTURE_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_AV1_PICTURE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_AV1_DPB_SLOT_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_AV1_DPB_SLOT_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_VIDEO_ENCODE_AV1_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_VIDEO_ENCODE_AV1_FEATURES_KHR")
            }
            Self::VIDEO_ENCODE_AV1_PROFILE_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_AV1_PROFILE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_AV1_RATE_CONTROL_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_AV1_RATE_CONTROL_INFO_KHR")
            }
            Self::VIDEO_ENCODE_AV1_RATE_CONTROL_LAYER_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_AV1_RATE_CONTROL_LAYER_INFO_KHR")
            }
            Self::VIDEO_ENCODE_AV1_QUALITY_LEVEL_PROPERTIES_KHR => {
                write!(f, "VIDEO_ENCODE_AV1_QUALITY_LEVEL_PROPERTIES_KHR")
            }
            Self::VIDEO_ENCODE_AV1_SESSION_CREATE_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_AV1_SESSION_CREATE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_AV1_GOP_REMAINING_FRAME_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_AV1_GOP_REMAINING_FRAME_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR")
            }
            Self::VIDEO_DECODE_VP9_CAPABILITIES_KHR => {
                write!(f, "VIDEO_DECODE_VP9_CAPABILITIES_KHR")
            }
            Self::VIDEO_DECODE_VP9_PICTURE_INFO_KHR => {
                write!(f, "VIDEO_DECODE_VP9_PICTURE_INFO_KHR")
            }
            Self::VIDEO_DECODE_VP9_PROFILE_INFO_KHR => {
                write!(f, "VIDEO_DECODE_VP9_PROFILE_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR")
            }
            Self::VIDEO_INLINE_QUERY_INFO_KHR => write!(f, "VIDEO_INLINE_QUERY_INFO_KHR"),
            Self::PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM => {
                write!(f, "PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM => {
                write!(f, "PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM")
            }
            Self::SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM => {
                write!(f, "SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM")
            }
            Self::SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM => {
                write!(f, "SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM")
            }
            Self::PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM => {
                write!(f, "PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM")
            }
            Self::BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM => {
                write!(f, "BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM")
            }
            Self::PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM => {
                write!(f, "PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM")
            }
            Self::SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM => {
                write!(f, "SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM")
            }
            Self::PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM => {
                write!(f, "PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT"
            ),
            Self::PHYSICAL_DEVICE_UNIFIED_IMAGE_LAYOUTS_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_UNIFIED_IMAGE_LAYOUTS_FEATURES_KHR")
            }
            Self::ATTACHMENT_FEEDBACK_LOOP_INFO_EXT => {
                write!(f, "ATTACHMENT_FEEDBACK_LOOP_INFO_EXT")
            }
            Self::SCREEN_BUFFER_PROPERTIES_QNX => write!(f, "SCREEN_BUFFER_PROPERTIES_QNX"),
            Self::SCREEN_BUFFER_FORMAT_PROPERTIES_QNX => {
                write!(f, "SCREEN_BUFFER_FORMAT_PROPERTIES_QNX")
            }
            Self::IMPORT_SCREEN_BUFFER_INFO_QNX => write!(f, "IMPORT_SCREEN_BUFFER_INFO_QNX"),
            Self::EXTERNAL_FORMAT_QNX => write!(f, "EXTERNAL_FORMAT_QNX"),
            Self::PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX => write!(
                f,
                "PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX"
            ),
            Self::PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT => {
                write!(f, "PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT")
            }
            Self::CALIBRATED_TIMESTAMP_INFO_KHR => write!(f, "CALIBRATED_TIMESTAMP_INFO_KHR"),
            Self::SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT => {
                write!(f, "SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT")
            }
            Self::BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT => {
                write!(f, "BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV => write!(
                f,
                "PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV"
            ),
            Self::PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM => {
                write!(f, "PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM => {
                write!(f, "PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM")
            }
            Self::TILE_MEMORY_REQUIREMENTS_QCOM => write!(f, "TILE_MEMORY_REQUIREMENTS_QCOM"),
            Self::TILE_MEMORY_BIND_INFO_QCOM => write!(f, "TILE_MEMORY_BIND_INFO_QCOM"),
            Self::TILE_MEMORY_SIZE_INFO_QCOM => write!(f, "TILE_MEMORY_SIZE_INFO_QCOM"),
            Self::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR")
            }
            Self::COPY_MEMORY_INDIRECT_INFO_KHR => write!(f, "COPY_MEMORY_INDIRECT_INFO_KHR"),
            Self::COPY_MEMORY_TO_IMAGE_INDIRECT_INFO_KHR => {
                write!(f, "COPY_MEMORY_TO_IMAGE_INDIRECT_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT")
            }
            Self::DECOMPRESS_MEMORY_INFO_EXT => write!(f, "DECOMPRESS_MEMORY_INFO_EXT"),
            Self::DISPLAY_SURFACE_STEREO_CREATE_INFO_NV => {
                write!(f, "DISPLAY_SURFACE_STEREO_CREATE_INFO_NV")
            }
            Self::DISPLAY_MODE_STEREO_PROPERTIES_NV => {
                write!(f, "DISPLAY_MODE_STEREO_PROPERTIES_NV")
            }
            Self::VIDEO_ENCODE_INTRA_REFRESH_CAPABILITIES_KHR => {
                write!(f, "VIDEO_ENCODE_INTRA_REFRESH_CAPABILITIES_KHR")
            }
            Self::VIDEO_ENCODE_SESSION_INTRA_REFRESH_CREATE_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_SESSION_INTRA_REFRESH_CREATE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_INTRA_REFRESH_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_INTRA_REFRESH_INFO_KHR")
            }
            Self::VIDEO_REFERENCE_INTRA_REFRESH_INFO_KHR => {
                write!(f, "VIDEO_REFERENCE_INTRA_REFRESH_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_VIDEO_ENCODE_INTRA_REFRESH_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_VIDEO_ENCODE_INTRA_REFRESH_FEATURES_KHR")
            }
            Self::VIDEO_ENCODE_QUANTIZATION_MAP_CAPABILITIES_KHR => {
                write!(f, "VIDEO_ENCODE_QUANTIZATION_MAP_CAPABILITIES_KHR")
            }
            Self::VIDEO_FORMAT_QUANTIZATION_MAP_PROPERTIES_KHR => {
                write!(f, "VIDEO_FORMAT_QUANTIZATION_MAP_PROPERTIES_KHR")
            }
            Self::VIDEO_ENCODE_QUANTIZATION_MAP_INFO_KHR => {
                write!(f, "VIDEO_ENCODE_QUANTIZATION_MAP_INFO_KHR")
            }
            Self::VIDEO_ENCODE_QUANTIZATION_MAP_SESSION_PARAMETERS_CREATE_INFO_KHR => write!(
                f,
                "VIDEO_ENCODE_QUANTIZATION_MAP_SESSION_PARAMETERS_CREATE_INFO_KHR"
            ),
            Self::PHYSICAL_DEVICE_VIDEO_ENCODE_QUANTIZATION_MAP_FEATURES_KHR => write!(
                f,
                "PHYSICAL_DEVICE_VIDEO_ENCODE_QUANTIZATION_MAP_FEATURES_KHR"
            ),
            Self::VIDEO_ENCODE_H264_QUANTIZATION_MAP_CAPABILITIES_KHR => {
                write!(f, "VIDEO_ENCODE_H264_QUANTIZATION_MAP_CAPABILITIES_KHR")
            }
            Self::VIDEO_ENCODE_H265_QUANTIZATION_MAP_CAPABILITIES_KHR => {
                write!(f, "VIDEO_ENCODE_H265_QUANTIZATION_MAP_CAPABILITIES_KHR")
            }
            Self::VIDEO_FORMAT_H265_QUANTIZATION_MAP_PROPERTIES_KHR => {
                write!(f, "VIDEO_FORMAT_H265_QUANTIZATION_MAP_PROPERTIES_KHR")
            }
            Self::VIDEO_ENCODE_AV1_QUANTIZATION_MAP_CAPABILITIES_KHR => {
                write!(f, "VIDEO_ENCODE_AV1_QUANTIZATION_MAP_CAPABILITIES_KHR")
            }
            Self::VIDEO_FORMAT_AV1_QUANTIZATION_MAP_PROPERTIES_KHR => {
                write!(f, "VIDEO_FORMAT_AV1_QUANTIZATION_MAP_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV")
            }
            Self::EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV => {
                write!(f, "EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV")
            }
            Self::EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV => {
                write!(f, "EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV")
            }
            Self::EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV => {
                write!(f, "EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV")
            }
            Self::PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV => {
                write!(f, "PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_SHADER_RELAXED_EXTENDED_INSTRUCTION_FEATURES_KHR => write!(
                f,
                "PHYSICAL_DEVICE_SHADER_RELAXED_EXTENDED_INSTRUCTION_FEATURES_KHR"
            ),
            Self::PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR => {
                write!(f, "PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR")
            }
            Self::PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV => write!(
                f,
                "PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV"
            ),
            Self::PHYSICAL_DEVICE_SHADER_REPLICATED_COMPOSITES_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_SHADER_REPLICATED_COMPOSITES_FEATURES_EXT"
            ),
            Self::PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV => write!(
                f,
                "PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV"
            ),
            Self::PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV => write!(
                f,
                "PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV"
            ),
            Self::CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV => write!(
                f,
                "CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV"
            ),
            Self::CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV => write!(
                f,
                "CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV"
            ),
            Self::CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV => {
                write!(f, "CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV")
            }
            Self::CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV => {
                write!(f, "CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV")
            }
            Self::CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV => {
                write!(f, "CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV")
            }
            Self::RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV => write!(
                f,
                "RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV"
            ),
            Self::PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV => write!(
                f,
                "PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV"
            ),
            Self::PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV => write!(
                f,
                "PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV"
            ),
            Self::WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV => write!(
                f,
                "WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV"
            ),
            Self::PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV => {
                write!(f, "PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV")
            }
            Self::BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV => {
                write!(f, "BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV")
            }
            Self::PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV => {
                write!(f, "PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV")
            }
            Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_EXT"
            ),
            Self::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_EXT => {
                write!(f, "GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_EXT")
            }
            Self::INDIRECT_EXECUTION_SET_CREATE_INFO_EXT => {
                write!(f, "INDIRECT_EXECUTION_SET_CREATE_INFO_EXT")
            }
            Self::GENERATED_COMMANDS_INFO_EXT => write!(f, "GENERATED_COMMANDS_INFO_EXT"),
            Self::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_EXT => {
                write!(f, "INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_EXT")
            }
            Self::INDIRECT_COMMANDS_LAYOUT_TOKEN_EXT => {
                write!(f, "INDIRECT_COMMANDS_LAYOUT_TOKEN_EXT")
            }
            Self::WRITE_INDIRECT_EXECUTION_SET_PIPELINE_EXT => {
                write!(f, "WRITE_INDIRECT_EXECUTION_SET_PIPELINE_EXT")
            }
            Self::WRITE_INDIRECT_EXECUTION_SET_SHADER_EXT => {
                write!(f, "WRITE_INDIRECT_EXECUTION_SET_SHADER_EXT")
            }
            Self::INDIRECT_EXECUTION_SET_PIPELINE_INFO_EXT => {
                write!(f, "INDIRECT_EXECUTION_SET_PIPELINE_INFO_EXT")
            }
            Self::INDIRECT_EXECUTION_SET_SHADER_INFO_EXT => {
                write!(f, "INDIRECT_EXECUTION_SET_SHADER_INFO_EXT")
            }
            Self::INDIRECT_EXECUTION_SET_SHADER_LAYOUT_INFO_EXT => {
                write!(f, "INDIRECT_EXECUTION_SET_SHADER_LAYOUT_INFO_EXT")
            }
            Self::GENERATED_COMMANDS_PIPELINE_INFO_EXT => {
                write!(f, "GENERATED_COMMANDS_PIPELINE_INFO_EXT")
            }
            Self::GENERATED_COMMANDS_SHADER_INFO_EXT => {
                write!(f, "GENERATED_COMMANDS_SHADER_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_FAULT_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_FAULT_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_FAULT_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_FAULT_PROPERTIES_KHR")
            }
            Self::DEVICE_FAULT_INFO_KHR => write!(f, "DEVICE_FAULT_INFO_KHR"),
            Self::DEVICE_FAULT_DEBUG_INFO_KHR => write!(f, "DEVICE_FAULT_DEBUG_INFO_KHR"),
            Self::PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR")
            }
            Self::MEMORY_BARRIER_ACCESS_FLAGS_3_KHR => {
                write!(f, "MEMORY_BARRIER_ACCESS_FLAGS_3_KHR")
            }
            Self::PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA => {
                write!(f, "PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA")
            }
            Self::PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA => {
                write!(f, "PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA")
            }
            Self::IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA => {
                write!(f, "IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA")
            }
            Self::PHYSICAL_DEVICE_SHADER_FMA_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_SHADER_FMA_FEATURES_KHR")
            }
            Self::PUSH_CONSTANT_BANK_INFO_NV => write!(f, "PUSH_CONSTANT_BANK_INFO_NV"),
            Self::PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV => {
                write!(f, "PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_EXT"
            ),
            Self::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_EXT"
            ),
            Self::PHYSICAL_DEVICE_DEPTH_CLAMP_CONTROL_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_DEPTH_CLAMP_CONTROL_FEATURES_EXT")
            }
            Self::PIPELINE_VIEWPORT_DEPTH_CLAMP_CONTROL_CREATE_INFO_EXT => {
                write!(f, "PIPELINE_VIEWPORT_DEPTH_CLAMP_CONTROL_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR")
            }
            Self::QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR => {
                write!(f, "QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_VIDEO_MAINTENANCE_2_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_VIDEO_MAINTENANCE_2_FEATURES_KHR")
            }
            Self::VIDEO_DECODE_H264_INLINE_SESSION_PARAMETERS_INFO_KHR => {
                write!(f, "VIDEO_DECODE_H264_INLINE_SESSION_PARAMETERS_INFO_KHR")
            }
            Self::VIDEO_DECODE_H265_INLINE_SESSION_PARAMETERS_INFO_KHR => {
                write!(f, "VIDEO_DECODE_H265_INLINE_SESSION_PARAMETERS_INFO_KHR")
            }
            Self::VIDEO_DECODE_AV1_INLINE_SESSION_PARAMETERS_INFO_KHR => {
                write!(f, "VIDEO_DECODE_AV1_INLINE_SESSION_PARAMETERS_INFO_KHR")
            }
            Self::SURFACE_CREATE_INFO_OHOS => write!(f, "SURFACE_CREATE_INFO_OHOS"),
            Self::PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI => {
                write!(f, "PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI")
            }
            Self::HDR_VIVID_DYNAMIC_METADATA_HUAWEI => {
                write!(f, "HDR_VIVID_DYNAMIC_METADATA_HUAWEI")
            }
            Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV")
            }
            Self::COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV => {
                write!(f, "COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV => {
                write!(f, "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM => {
                write!(f, "PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM")
            }
            Self::IMPORT_MEMORY_METAL_HANDLE_INFO_EXT => {
                write!(f, "IMPORT_MEMORY_METAL_HANDLE_INFO_EXT")
            }
            Self::MEMORY_METAL_HANDLE_PROPERTIES_EXT => {
                write!(f, "MEMORY_METAL_HANDLE_PROPERTIES_EXT")
            }
            Self::MEMORY_GET_METAL_HANDLE_INFO_EXT => write!(f, "MEMORY_GET_METAL_HANDLE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM => write!(
                f,
                "PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM"
            ),
            Self::PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM => write!(
                f,
                "PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM"
            ),
            Self::PERFORMANCE_COUNTER_ARM => write!(f, "PERFORMANCE_COUNTER_ARM"),
            Self::PERFORMANCE_COUNTER_DESCRIPTION_ARM => {
                write!(f, "PERFORMANCE_COUNTER_DESCRIPTION_ARM")
            }
            Self::RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM => write!(
                f,
                "RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM"
            ),
            Self::PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_FEATURES_ARM => {
                write!(f, "PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_FEATURES_ARM")
            }
            Self::PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_PROPERTIES_ARM => {
                write!(f, "PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_PROPERTIES_ARM")
            }
            Self::SHADER_INSTRUMENTATION_CREATE_INFO_ARM => {
                write!(f, "SHADER_INSTRUMENTATION_CREATE_INFO_ARM")
            }
            Self::SHADER_INSTRUMENTATION_METRIC_DESCRIPTION_ARM => {
                write!(f, "SHADER_INSTRUMENTATION_METRIC_DESCRIPTION_ARM")
            }
            Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_ROBUSTNESS_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_ROBUSTNESS_FEATURES_EXT"
            ),
            Self::PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM => {
                write!(f, "PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE => write!(
                f,
                "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE"
            ),
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE => write!(
                f,
                "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE"
            ),
            Self::PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE => {
                write!(f, "PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE")
            }
            Self::PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR")
            }
            Self::SET_PRESENT_CONFIG_NV => write!(f, "SET_PRESENT_CONFIG_NV"),
            Self::PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT"
            ),
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT"
            ),
            Self::RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT => {
                write!(f, "RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_ZERO_INITIALIZE_DEVICE_MEMORY_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_ZERO_INITIALIZE_DEVICE_MEMORY_FEATURES_EXT"
            ),
            Self::PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR => write!(
                f,
                "PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR"
            ),
            Self::PHYSICAL_DEVICE_SHADER_64_BIT_INDEXING_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_SHADER_64_BIT_INDEXING_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_CUSTOM_RESOLVE_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_CUSTOM_RESOLVE_FEATURES_EXT")
            }
            Self::BEGIN_CUSTOM_RESOLVE_INFO_EXT => write!(f, "BEGIN_CUSTOM_RESOLVE_INFO_EXT"),
            Self::CUSTOM_RESOLVE_CREATE_INFO_EXT => write!(f, "CUSTOM_RESOLVE_CREATE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM => {
                write!(f, "PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM")
            }
            Self::DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM => {
                write!(f, "DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_10_FEATURES_KHR => {
                write!(f, "PHYSICAL_DEVICE_MAINTENANCE_10_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_10_PROPERTIES_KHR => {
                write!(f, "PHYSICAL_DEVICE_MAINTENANCE_10_PROPERTIES_KHR")
            }
            Self::RENDERING_ATTACHMENT_FLAGS_INFO_KHR => {
                write!(f, "RENDERING_ATTACHMENT_FLAGS_INFO_KHR")
            }
            Self::RENDERING_END_INFO_KHR => write!(f, "RENDERING_END_INFO_KHR"),
            Self::RESOLVE_IMAGE_MODE_INFO_KHR => write!(f, "RESOLVE_IMAGE_MODE_INFO_KHR"),
            Self::PHYSICAL_DEVICE_DATA_GRAPH_OPTICAL_FLOW_FEATURES_ARM => {
                write!(f, "PHYSICAL_DEVICE_DATA_GRAPH_OPTICAL_FLOW_FEATURES_ARM")
            }
            Self::QUEUE_FAMILY_DATA_GRAPH_OPTICAL_FLOW_PROPERTIES_ARM => {
                write!(f, "QUEUE_FAMILY_DATA_GRAPH_OPTICAL_FLOW_PROPERTIES_ARM")
            }
            Self::DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_INFO_ARM => {
                write!(f, "DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_INFO_ARM")
            }
            Self::DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_ARM => {
                write!(f, "DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_OPTICAL_FLOW_DISPATCH_INFO_ARM => {
                write!(f, "DATA_GRAPH_PIPELINE_OPTICAL_FLOW_DISPATCH_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_OPTICAL_FLOW_CREATE_INFO_ARM => {
                write!(f, "DATA_GRAPH_PIPELINE_OPTICAL_FLOW_CREATE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_RESOURCE_INFO_IMAGE_LAYOUT_ARM => {
                write!(f, "DATA_GRAPH_PIPELINE_RESOURCE_INFO_IMAGE_LAYOUT_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_SINGLE_NODE_CREATE_INFO_ARM => {
                write!(f, "DATA_GRAPH_PIPELINE_SINGLE_NODE_CREATE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_SINGLE_NODE_CONNECTION_ARM => {
                write!(f, "DATA_GRAPH_PIPELINE_SINGLE_NODE_CONNECTION_ARM")
            }
            Self::PHYSICAL_DEVICE_SHADER_LONG_VECTOR_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_SHADER_LONG_VECTOR_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_LONG_VECTOR_PROPERTIES_EXT => {
                write!(f, "PHYSICAL_DEVICE_SHADER_LONG_VECTOR_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC => write!(
                f,
                "PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC"
            ),
            Self::PHYSICAL_DEVICE_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_FEATURES_EXT"
            ),
            Self::COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV => {
                write!(f, "COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV")
            }
            Self::PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV => {
                write!(f, "PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_PARTITIONED_FEATURES_EXT => write!(
                f,
                "PHYSICAL_DEVICE_SHADER_SUBGROUP_PARTITIONED_FEATURES_EXT"
            ),
            Self::UBM_SURFACE_CREATE_INFO_SEC => write!(f, "UBM_SURFACE_CREATE_INFO_SEC"),
            Self::PHYSICAL_DEVICE_SHADER_MIXED_FLOAT_DOT_PRODUCT_FEATURES_VALVE => write!(
                f,
                "PHYSICAL_DEVICE_SHADER_MIXED_FLOAT_DOT_PRODUCT_FEATURES_VALVE"
            ),
            Self::PHYSICAL_DEVICE_PRIMITIVE_RESTART_INDEX_FEATURES_EXT => {
                write!(f, "PHYSICAL_DEVICE_PRIMITIVE_RESTART_INDEX_FEATURES_EXT")
            }
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSystemAllocationScope.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SystemAllocationScope(i32);
impl SystemAllocationScope {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl SystemAllocationScope {
    pub const COMMAND: Self = Self(0);
    pub const OBJECT: Self = Self(1);
    pub const CACHE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const INSTANCE: Self = Self(4);
}
impl fmt::Display for SystemAllocationScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::COMMAND => write!(f, "COMMAND"),
            Self::OBJECT => write!(f, "OBJECT"),
            Self::CACHE => write!(f, "CACHE"),
            Self::DEVICE => write!(f, "DEVICE"),
            Self::INSTANCE => write!(f, "INSTANCE"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkInternalAllocationType.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct InternalAllocationType(i32);
impl InternalAllocationType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl InternalAllocationType {
    pub const EXECUTABLE: Self = Self(0);
}
impl fmt::Display for InternalAllocationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::EXECUTABLE {
            write!(f, "EXECUTABLE")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerAddressMode.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SamplerAddressMode(i32);
impl SamplerAddressMode {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl SamplerAddressMode {
    pub const REPEAT: Self = Self(0);
    pub const MIRRORED_REPEAT: Self = Self(1);
    pub const CLAMP_TO_EDGE: Self = Self(2);
    pub const CLAMP_TO_BORDER: Self = Self(3);
    pub const MIRROR_CLAMP_TO_EDGE: Self = Self(4);
    #[deprecated = "aliased"]
    pub const MIRROR_CLAMP_TO_EDGE_KHR: Self = Self::MIRROR_CLAMP_TO_EDGE;
}
impl fmt::Display for SamplerAddressMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::REPEAT => write!(f, "REPEAT"),
            Self::MIRRORED_REPEAT => write!(f, "MIRRORED_REPEAT"),
            Self::CLAMP_TO_EDGE => write!(f, "CLAMP_TO_EDGE"),
            Self::CLAMP_TO_BORDER => write!(f, "CLAMP_TO_BORDER"),
            Self::MIRROR_CLAMP_TO_EDGE => write!(f, "MIRROR_CLAMP_TO_EDGE"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFilter.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Filter(i32);
impl Filter {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl Filter {
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
    pub const CUBIC_IMG: Self = Self::CUBIC_EXT;
    pub const CUBIC_EXT: Self = Self(1000015000);
}
impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NEAREST => write!(f, "NEAREST"),
            Self::LINEAR => write!(f, "LINEAR"),
            Self::CUBIC_EXT => write!(f, "CUBIC_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerMipmapMode.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SamplerMipmapMode(i32);
impl SamplerMipmapMode {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl SamplerMipmapMode {
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}
impl fmt::Display for SamplerMipmapMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NEAREST => write!(f, "NEAREST"),
            Self::LINEAR => write!(f, "LINEAR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputRate.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct VertexInputRate(i32);
impl VertexInputRate {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl VertexInputRate {
    pub const VERTEX: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
}
impl fmt::Display for VertexInputRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::VERTEX => write!(f, "VERTEX"),
            Self::INSTANCE => write!(f, "INSTANCE"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureTypeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ClusterAccelerationStructureTypeNV(i32);
impl ClusterAccelerationStructureTypeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ClusterAccelerationStructureTypeNV {
    pub const CLUSTERS_BOTTOM_LEVEL_NV: Self = Self(0);
    pub const TRIANGLE_CLUSTER_NV: Self = Self(1);
    pub const TRIANGLE_CLUSTER_TEMPLATE_NV: Self = Self(2);
}
impl fmt::Display for ClusterAccelerationStructureTypeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::CLUSTERS_BOTTOM_LEVEL_NV => write!(f, "CLUSTERS_BOTTOM_LEVEL_NV"),
            Self::TRIANGLE_CLUSTER_NV => write!(f, "TRIANGLE_CLUSTER_NV"),
            Self::TRIANGLE_CLUSTER_TEMPLATE_NV => write!(f, "TRIANGLE_CLUSTER_TEMPLATE_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureOpTypeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ClusterAccelerationStructureOpTypeNV(i32);
impl ClusterAccelerationStructureOpTypeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ClusterAccelerationStructureOpTypeNV {
    pub const MOVE_OBJECTS_NV: Self = Self(0);
    pub const BUILD_CLUSTERS_BOTTOM_LEVEL_NV: Self = Self(1);
    pub const BUILD_TRIANGLE_CLUSTER_NV: Self = Self(2);
    pub const BUILD_TRIANGLE_CLUSTER_TEMPLATE_NV: Self = Self(3);
    pub const INSTANTIATE_TRIANGLE_CLUSTER_NV: Self = Self(4);
    pub const GET_CLUSTER_TEMPLATE_INDICES_NV: Self = Self(5);
}
impl fmt::Display for ClusterAccelerationStructureOpTypeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::MOVE_OBJECTS_NV => write!(f, "MOVE_OBJECTS_NV"),
            Self::BUILD_CLUSTERS_BOTTOM_LEVEL_NV => write!(f, "BUILD_CLUSTERS_BOTTOM_LEVEL_NV"),
            Self::BUILD_TRIANGLE_CLUSTER_NV => write!(f, "BUILD_TRIANGLE_CLUSTER_NV"),
            Self::BUILD_TRIANGLE_CLUSTER_TEMPLATE_NV => {
                write!(f, "BUILD_TRIANGLE_CLUSTER_TEMPLATE_NV")
            }
            Self::INSTANTIATE_TRIANGLE_CLUSTER_NV => write!(f, "INSTANTIATE_TRIANGLE_CLUSTER_NV"),
            Self::GET_CLUSTER_TEMPLATE_INDICES_NV => write!(f, "GET_CLUSTER_TEMPLATE_INDICES_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureOpModeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ClusterAccelerationStructureOpModeNV(i32);
impl ClusterAccelerationStructureOpModeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ClusterAccelerationStructureOpModeNV {
    pub const IMPLICIT_DESTINATIONS_NV: Self = Self(0);
    pub const EXPLICIT_DESTINATIONS_NV: Self = Self(1);
    pub const COMPUTE_SIZES_NV: Self = Self(2);
}
impl fmt::Display for ClusterAccelerationStructureOpModeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::IMPLICIT_DESTINATIONS_NV => write!(f, "IMPLICIT_DESTINATIONS_NV"),
            Self::EXPLICIT_DESTINATIONS_NV => write!(f, "EXPLICIT_DESTINATIONS_NV"),
            Self::COMPUTE_SIZES_NV => write!(f, "COMPUTE_SIZES_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkObjectType.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ObjectType(i32);
impl ObjectType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ObjectType {
    pub const UNKNOWN: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
    pub const PHYSICAL_DEVICE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const QUEUE: Self = Self(4);
    pub const SEMAPHORE: Self = Self(5);
    pub const COMMAND_BUFFER: Self = Self(6);
    pub const FENCE: Self = Self(7);
    pub const DEVICE_MEMORY: Self = Self(8);
    pub const BUFFER: Self = Self(9);
    pub const IMAGE: Self = Self(10);
    pub const EVENT: Self = Self(11);
    pub const QUERY_POOL: Self = Self(12);
    pub const BUFFER_VIEW: Self = Self(13);
    pub const IMAGE_VIEW: Self = Self(14);
    pub const SHADER_MODULE: Self = Self(15);
    pub const PIPELINE_CACHE: Self = Self(16);
    pub const PIPELINE_LAYOUT: Self = Self(17);
    pub const RENDER_PASS: Self = Self(18);
    pub const PIPELINE: Self = Self(19);
    pub const DESCRIPTOR_SET_LAYOUT: Self = Self(20);
    pub const SAMPLER: Self = Self(21);
    pub const DESCRIPTOR_POOL: Self = Self(22);
    pub const DESCRIPTOR_SET: Self = Self(23);
    pub const FRAMEBUFFER: Self = Self(24);
    pub const COMMAND_POOL: Self = Self(25);
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1000085000);
    pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1000156000);
    pub const PRIVATE_DATA_SLOT: Self = Self(1000295000);
    pub const SURFACE_KHR: Self = Self(1000000000);
    pub const SWAPCHAIN_KHR: Self = Self(1000001000);
    pub const DISPLAY_KHR: Self = Self(1000002000);
    pub const DISPLAY_MODE_KHR: Self = Self(1000002001);
    pub const DEBUG_REPORT_CALLBACK_EXT: Self = Self(1000011000);
    pub const VIDEO_SESSION_KHR: Self = Self(1000023000);
    pub const VIDEO_SESSION_PARAMETERS_KHR: Self = Self(1000023001);
    pub const CU_MODULE_NVX: Self = Self(1000029000);
    pub const CU_FUNCTION_NVX: Self = Self(1000029001);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE;
    pub const DEBUG_UTILS_MESSENGER_EXT: Self = Self(1000128000);
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
    pub const SAMPLER_YCBCR_CONVERSION_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION;
    pub const VALIDATION_CACHE_EXT: Self = Self(1000160000);
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
    pub const PERFORMANCE_CONFIGURATION_INTEL: Self = Self(1000210000);
    pub const DEFERRED_OPERATION_KHR: Self = Self(1000268000);
    pub const INDIRECT_COMMANDS_LAYOUT_NV: Self = Self(1000277000);
    pub const PRIVATE_DATA_SLOT_EXT: Self = Self::PRIVATE_DATA_SLOT;
    pub const CUDA_MODULE_NV: Self = Self(1000307000);
    pub const CUDA_FUNCTION_NV: Self = Self(1000307001);
    pub const BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366000);
    pub const MICROMAP_EXT: Self = Self(1000396000);
    pub const TENSOR_ARM: Self = Self(1000460000);
    pub const TENSOR_VIEW_ARM: Self = Self(1000460001);
    pub const OPTICAL_FLOW_SESSION_NV: Self = Self(1000464000);
    pub const SHADER_EXT: Self = Self(1000482000);
    pub const PIPELINE_BINARY_KHR: Self = Self(1000483000);
    pub const SEMAPHORE_SCI_SYNC_POOL_NV: Self = Self(1000489000);
    pub const DATA_GRAPH_PIPELINE_SESSION_ARM: Self = Self(1000507000);
    pub const EXTERNAL_COMPUTE_QUEUE_NV: Self = Self(1000556000);
    pub const INDIRECT_COMMANDS_LAYOUT_EXT: Self = Self(1000572000);
    pub const INDIRECT_EXECUTION_SET_EXT: Self = Self(1000572001);
    pub const SHADER_INSTRUMENTATION_ARM: Self = Self(1000607000);
}
impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UNKNOWN => write!(f, "UNKNOWN"),
            Self::INSTANCE => write!(f, "INSTANCE"),
            Self::PHYSICAL_DEVICE => write!(f, "PHYSICAL_DEVICE"),
            Self::DEVICE => write!(f, "DEVICE"),
            Self::QUEUE => write!(f, "QUEUE"),
            Self::SEMAPHORE => write!(f, "SEMAPHORE"),
            Self::COMMAND_BUFFER => write!(f, "COMMAND_BUFFER"),
            Self::FENCE => write!(f, "FENCE"),
            Self::DEVICE_MEMORY => write!(f, "DEVICE_MEMORY"),
            Self::BUFFER => write!(f, "BUFFER"),
            Self::IMAGE => write!(f, "IMAGE"),
            Self::EVENT => write!(f, "EVENT"),
            Self::QUERY_POOL => write!(f, "QUERY_POOL"),
            Self::BUFFER_VIEW => write!(f, "BUFFER_VIEW"),
            Self::IMAGE_VIEW => write!(f, "IMAGE_VIEW"),
            Self::SHADER_MODULE => write!(f, "SHADER_MODULE"),
            Self::PIPELINE_CACHE => write!(f, "PIPELINE_CACHE"),
            Self::PIPELINE_LAYOUT => write!(f, "PIPELINE_LAYOUT"),
            Self::RENDER_PASS => write!(f, "RENDER_PASS"),
            Self::PIPELINE => write!(f, "PIPELINE"),
            Self::DESCRIPTOR_SET_LAYOUT => write!(f, "DESCRIPTOR_SET_LAYOUT"),
            Self::SAMPLER => write!(f, "SAMPLER"),
            Self::DESCRIPTOR_POOL => write!(f, "DESCRIPTOR_POOL"),
            Self::DESCRIPTOR_SET => write!(f, "DESCRIPTOR_SET"),
            Self::FRAMEBUFFER => write!(f, "FRAMEBUFFER"),
            Self::COMMAND_POOL => write!(f, "COMMAND_POOL"),
            Self::DESCRIPTOR_UPDATE_TEMPLATE => write!(f, "DESCRIPTOR_UPDATE_TEMPLATE"),
            Self::SAMPLER_YCBCR_CONVERSION => write!(f, "SAMPLER_YCBCR_CONVERSION"),
            Self::PRIVATE_DATA_SLOT => write!(f, "PRIVATE_DATA_SLOT"),
            Self::SURFACE_KHR => write!(f, "SURFACE_KHR"),
            Self::SWAPCHAIN_KHR => write!(f, "SWAPCHAIN_KHR"),
            Self::DISPLAY_KHR => write!(f, "DISPLAY_KHR"),
            Self::DISPLAY_MODE_KHR => write!(f, "DISPLAY_MODE_KHR"),
            Self::DEBUG_REPORT_CALLBACK_EXT => write!(f, "DEBUG_REPORT_CALLBACK_EXT"),
            Self::VIDEO_SESSION_KHR => write!(f, "VIDEO_SESSION_KHR"),
            Self::VIDEO_SESSION_PARAMETERS_KHR => write!(f, "VIDEO_SESSION_PARAMETERS_KHR"),
            Self::CU_MODULE_NVX => write!(f, "CU_MODULE_NVX"),
            Self::CU_FUNCTION_NVX => write!(f, "CU_FUNCTION_NVX"),
            Self::DEBUG_UTILS_MESSENGER_EXT => write!(f, "DEBUG_UTILS_MESSENGER_EXT"),
            Self::ACCELERATION_STRUCTURE_KHR => write!(f, "ACCELERATION_STRUCTURE_KHR"),
            Self::VALIDATION_CACHE_EXT => write!(f, "VALIDATION_CACHE_EXT"),
            Self::ACCELERATION_STRUCTURE_NV => write!(f, "ACCELERATION_STRUCTURE_NV"),
            Self::PERFORMANCE_CONFIGURATION_INTEL => write!(f, "PERFORMANCE_CONFIGURATION_INTEL"),
            Self::DEFERRED_OPERATION_KHR => write!(f, "DEFERRED_OPERATION_KHR"),
            Self::INDIRECT_COMMANDS_LAYOUT_NV => write!(f, "INDIRECT_COMMANDS_LAYOUT_NV"),
            Self::CUDA_MODULE_NV => write!(f, "CUDA_MODULE_NV"),
            Self::CUDA_FUNCTION_NV => write!(f, "CUDA_FUNCTION_NV"),
            Self::BUFFER_COLLECTION_FUCHSIA => write!(f, "BUFFER_COLLECTION_FUCHSIA"),
            Self::MICROMAP_EXT => write!(f, "MICROMAP_EXT"),
            Self::TENSOR_ARM => write!(f, "TENSOR_ARM"),
            Self::TENSOR_VIEW_ARM => write!(f, "TENSOR_VIEW_ARM"),
            Self::OPTICAL_FLOW_SESSION_NV => write!(f, "OPTICAL_FLOW_SESSION_NV"),
            Self::SHADER_EXT => write!(f, "SHADER_EXT"),
            Self::PIPELINE_BINARY_KHR => write!(f, "PIPELINE_BINARY_KHR"),
            Self::SEMAPHORE_SCI_SYNC_POOL_NV => write!(f, "SEMAPHORE_SCI_SYNC_POOL_NV"),
            Self::DATA_GRAPH_PIPELINE_SESSION_ARM => write!(f, "DATA_GRAPH_PIPELINE_SESSION_ARM"),
            Self::EXTERNAL_COMPUTE_QUEUE_NV => write!(f, "EXTERNAL_COMPUTE_QUEUE_NV"),
            Self::INDIRECT_COMMANDS_LAYOUT_EXT => write!(f, "INDIRECT_COMMANDS_LAYOUT_EXT"),
            Self::INDIRECT_EXECUTION_SET_EXT => write!(f, "INDIRECT_EXECUTION_SET_EXT"),
            Self::SHADER_INSTRUMENTATION_ARM => write!(f, "SHADER_INSTRUMENTATION_ARM"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingInvocationReorderModeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct RayTracingInvocationReorderModeEXT(i32);
impl RayTracingInvocationReorderModeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl RayTracingInvocationReorderModeEXT {
    pub const NONE_EXT: Self = Self(0);
    pub const REORDER_EXT: Self = Self(1);
    pub const NONE_NV: Self = Self::NONE_EXT;
    pub const REORDER_NV: Self = Self::REORDER_EXT;
}
impl fmt::Display for RayTracingInvocationReorderModeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NONE_EXT => write!(f, "NONE_EXT"),
            Self::REORDER_EXT => write!(f, "REORDER_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsTokenTypeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct IndirectCommandsTokenTypeNV(i32);
impl IndirectCommandsTokenTypeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl IndirectCommandsTokenTypeNV {
    pub const SHADER_GROUP_NV: Self = Self(0);
    pub const STATE_FLAGS_NV: Self = Self(1);
    pub const INDEX_BUFFER_NV: Self = Self(2);
    pub const VERTEX_BUFFER_NV: Self = Self(3);
    pub const PUSH_CONSTANT_NV: Self = Self(4);
    pub const DRAW_INDEXED_NV: Self = Self(5);
    pub const DRAW_NV: Self = Self(6);
    pub const DRAW_TASKS_NV: Self = Self(7);
    pub const PUSH_DATA_NV: Self = Self(1000135000);
    pub const DRAW_MESH_TASKS_NV: Self = Self(1000328000);
    pub const PIPELINE_NV: Self = Self(1000428003);
    pub const DISPATCH_NV: Self = Self(1000428004);
}
impl fmt::Display for IndirectCommandsTokenTypeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::SHADER_GROUP_NV => write!(f, "SHADER_GROUP_NV"),
            Self::STATE_FLAGS_NV => write!(f, "STATE_FLAGS_NV"),
            Self::INDEX_BUFFER_NV => write!(f, "INDEX_BUFFER_NV"),
            Self::VERTEX_BUFFER_NV => write!(f, "VERTEX_BUFFER_NV"),
            Self::PUSH_CONSTANT_NV => write!(f, "PUSH_CONSTANT_NV"),
            Self::DRAW_INDEXED_NV => write!(f, "DRAW_INDEXED_NV"),
            Self::DRAW_NV => write!(f, "DRAW_NV"),
            Self::DRAW_TASKS_NV => write!(f, "DRAW_TASKS_NV"),
            Self::PUSH_DATA_NV => write!(f, "PUSH_DATA_NV"),
            Self::DRAW_MESH_TASKS_NV => write!(f, "DRAW_MESH_TASKS_NV"),
            Self::PIPELINE_NV => write!(f, "PIPELINE_NV"),
            Self::DISPATCH_NV => write!(f, "DISPATCH_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateType.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DescriptorUpdateTemplateType(i32);
impl DescriptorUpdateTemplateType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DescriptorUpdateTemplateType {
    pub const DESCRIPTOR_SET: Self = Self(0);
    pub const PUSH_DESCRIPTORS: Self = Self(1);
    pub const PUSH_DESCRIPTORS_KHR: Self = Self::PUSH_DESCRIPTORS;
    pub const DESCRIPTOR_SET_KHR: Self = Self::DESCRIPTOR_SET;
}
impl fmt::Display for DescriptorUpdateTemplateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DESCRIPTOR_SET => write!(f, "DESCRIPTOR_SET"),
            Self::PUSH_DESCRIPTORS => write!(f, "PUSH_DESCRIPTORS"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkViewportCoordinateSwizzleNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ViewportCoordinateSwizzleNV(i32);
impl ViewportCoordinateSwizzleNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ViewportCoordinateSwizzleNV {
    pub const POSITIVE_X_NV: Self = Self(0);
    pub const NEGATIVE_X_NV: Self = Self(1);
    pub const POSITIVE_Y_NV: Self = Self(2);
    pub const NEGATIVE_Y_NV: Self = Self(3);
    pub const POSITIVE_Z_NV: Self = Self(4);
    pub const NEGATIVE_Z_NV: Self = Self(5);
    pub const POSITIVE_W_NV: Self = Self(6);
    pub const NEGATIVE_W_NV: Self = Self(7);
}
impl fmt::Display for ViewportCoordinateSwizzleNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::POSITIVE_X_NV => write!(f, "POSITIVE_X_NV"),
            Self::NEGATIVE_X_NV => write!(f, "NEGATIVE_X_NV"),
            Self::POSITIVE_Y_NV => write!(f, "POSITIVE_Y_NV"),
            Self::NEGATIVE_Y_NV => write!(f, "NEGATIVE_Y_NV"),
            Self::POSITIVE_Z_NV => write!(f, "POSITIVE_Z_NV"),
            Self::NEGATIVE_Z_NV => write!(f, "NEGATIVE_Z_NV"),
            Self::POSITIVE_W_NV => write!(f, "POSITIVE_W_NV"),
            Self::NEGATIVE_W_NV => write!(f, "NEGATIVE_W_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDiscardRectangleModeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DiscardRectangleModeEXT(i32);
impl DiscardRectangleModeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DiscardRectangleModeEXT {
    pub const INCLUSIVE_EXT: Self = Self(0);
    pub const EXCLUSIVE_EXT: Self = Self(1);
}
impl fmt::Display for DiscardRectangleModeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::INCLUSIVE_EXT => write!(f, "INCLUSIVE_EXT"),
            Self::EXCLUSIVE_EXT => write!(f, "EXCLUSIVE_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPointClippingBehavior.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PointClippingBehavior(i32);
impl PointClippingBehavior {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PointClippingBehavior {
    pub const ALL_CLIP_PLANES: Self = Self(0);
    pub const USER_CLIP_PLANES_ONLY: Self = Self(1);
    pub const ALL_CLIP_PLANES_KHR: Self = Self::ALL_CLIP_PLANES;
    pub const USER_CLIP_PLANES_ONLY_KHR: Self = Self::USER_CLIP_PLANES_ONLY;
}
impl fmt::Display for PointClippingBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ALL_CLIP_PLANES => write!(f, "ALL_CLIP_PLANES"),
            Self::USER_CLIP_PLANES_ONLY => write!(f, "USER_CLIP_PLANES_ONLY"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCoverageModulationModeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CoverageModulationModeNV(i32);
impl CoverageModulationModeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl CoverageModulationModeNV {
    pub const NONE_NV: Self = Self(0);
    pub const RGB_NV: Self = Self(1);
    pub const ALPHA_NV: Self = Self(2);
    pub const RGBA_NV: Self = Self(3);
}
impl fmt::Display for CoverageModulationModeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NONE_NV => write!(f, "NONE_NV"),
            Self::RGB_NV => write!(f, "RGB_NV"),
            Self::ALPHA_NV => write!(f, "ALPHA_NV"),
            Self::RGBA_NV => write!(f, "RGBA_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCoverageReductionModeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CoverageReductionModeNV(i32);
impl CoverageReductionModeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl CoverageReductionModeNV {
    pub const MERGE_NV: Self = Self(0);
    pub const TRUNCATE_NV: Self = Self(1);
}
impl fmt::Display for CoverageReductionModeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::MERGE_NV => write!(f, "MERGE_NV"),
            Self::TRUNCATE_NV => write!(f, "TRUNCATE_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheHeaderVersionEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ValidationCacheHeaderVersionEXT(i32);
impl ValidationCacheHeaderVersionEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ValidationCacheHeaderVersionEXT {
    pub const ONE_EXT: Self = Self(1);
}
impl fmt::Display for ValidationCacheHeaderVersionEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::ONE_EXT {
            write!(f, "ONE_EXT")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInfoTypeAMD.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ShaderInfoTypeAMD(i32);
impl ShaderInfoTypeAMD {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ShaderInfoTypeAMD {
    pub const STATISTICS_AMD: Self = Self(0);
    pub const BINARY_AMD: Self = Self(1);
    pub const DISASSEMBLY_AMD: Self = Self(2);
}
impl fmt::Display for ShaderInfoTypeAMD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::STATISTICS_AMD => write!(f, "STATISTICS_AMD"),
            Self::BINARY_AMD => write!(f, "BINARY_AMD"),
            Self::DISASSEMBLY_AMD => write!(f, "DISASSEMBLY_AMD"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueGlobalPriority.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct QueueGlobalPriority(i32);
impl QueueGlobalPriority {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl QueueGlobalPriority {
    pub const LOW: Self = Self(128);
    pub const MEDIUM: Self = Self(256);
    pub const HIGH: Self = Self(512);
    pub const REALTIME: Self = Self(1024);
    pub const LOW_EXT: Self = Self::LOW;
    pub const MEDIUM_EXT: Self = Self::MEDIUM;
    pub const HIGH_EXT: Self = Self::HIGH;
    pub const REALTIME_EXT: Self = Self::REALTIME;
    pub const LOW_KHR: Self = Self::LOW;
    pub const MEDIUM_KHR: Self = Self::MEDIUM;
    pub const HIGH_KHR: Self = Self::HIGH;
    pub const REALTIME_KHR: Self = Self::REALTIME;
}
impl fmt::Display for QueueGlobalPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LOW => write!(f, "LOW"),
            Self::MEDIUM => write!(f, "MEDIUM"),
            Self::HIGH => write!(f, "HIGH"),
            Self::REALTIME => write!(f, "REALTIME"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTimeDomainKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TimeDomainKHR(i32);
impl TimeDomainKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl TimeDomainKHR {
    pub const DEVICE_KHR: Self = Self(0);
    pub const CLOCK_MONOTONIC_KHR: Self = Self(1);
    pub const CLOCK_MONOTONIC_RAW_KHR: Self = Self(2);
    pub const QUERY_PERFORMANCE_COUNTER_KHR: Self = Self(3);
    pub const DEVICE_EXT: Self = Self::DEVICE_KHR;
    pub const CLOCK_MONOTONIC_EXT: Self = Self::CLOCK_MONOTONIC_KHR;
    pub const CLOCK_MONOTONIC_RAW_EXT: Self = Self::CLOCK_MONOTONIC_RAW_KHR;
    pub const QUERY_PERFORMANCE_COUNTER_EXT: Self = Self::QUERY_PERFORMANCE_COUNTER_KHR;
    pub const PRESENT_STAGE_LOCAL_EXT: Self = Self(1000208000);
    pub const SWAPCHAIN_LOCAL_EXT: Self = Self(1000208001);
}
impl fmt::Display for TimeDomainKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DEVICE_KHR => write!(f, "DEVICE_KHR"),
            Self::CLOCK_MONOTONIC_KHR => write!(f, "CLOCK_MONOTONIC_KHR"),
            Self::CLOCK_MONOTONIC_RAW_KHR => write!(f, "CLOCK_MONOTONIC_RAW_KHR"),
            Self::QUERY_PERFORMANCE_COUNTER_KHR => write!(f, "QUERY_PERFORMANCE_COUNTER_KHR"),
            Self::PRESENT_STAGE_LOCAL_EXT => write!(f, "PRESENT_STAGE_LOCAL_EXT"),
            Self::SWAPCHAIN_LOCAL_EXT => write!(f, "SWAPCHAIN_LOCAL_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkConservativeRasterizationModeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ConservativeRasterizationModeEXT(i32);
impl ConservativeRasterizationModeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ConservativeRasterizationModeEXT {
    pub const DISABLED_EXT: Self = Self(0);
    pub const OVERESTIMATE_EXT: Self = Self(1);
    pub const UNDERESTIMATE_EXT: Self = Self(2);
}
impl fmt::Display for ConservativeRasterizationModeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DISABLED_EXT => write!(f, "DISABLED_EXT"),
            Self::OVERESTIMATE_EXT => write!(f, "OVERESTIMATE_EXT"),
            Self::UNDERESTIMATE_EXT => write!(f, "UNDERESTIMATE_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreType.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SemaphoreType(i32);
impl SemaphoreType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl SemaphoreType {
    pub const BINARY: Self = Self(0);
    pub const TIMELINE: Self = Self(1);
    pub const BINARY_KHR: Self = Self::BINARY;
    pub const TIMELINE_KHR: Self = Self::TIMELINE;
}
impl fmt::Display for SemaphoreType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::BINARY => write!(f, "BINARY"),
            Self::TIMELINE => write!(f, "TIMELINE"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureModeKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BuildAccelerationStructureModeKHR(i32);
impl BuildAccelerationStructureModeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl BuildAccelerationStructureModeKHR {
    pub const BUILD_KHR: Self = Self(0);
    pub const UPDATE_KHR: Self = Self(1);
}
impl fmt::Display for BuildAccelerationStructureModeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::BUILD_KHR => write!(f, "BUILD_KHR"),
            Self::UPDATE_KHR => write!(f, "UPDATE_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyAccelerationStructureModeKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CopyAccelerationStructureModeKHR(i32);
impl CopyAccelerationStructureModeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl CopyAccelerationStructureModeKHR {
    pub const CLONE_KHR: Self = Self(0);
    pub const COMPACT_KHR: Self = Self(1);
    pub const SERIALIZE_KHR: Self = Self(2);
    pub const DESERIALIZE_KHR: Self = Self(3);
    pub const CLONE_NV: Self = Self::CLONE_KHR;
    pub const COMPACT_NV: Self = Self::COMPACT_KHR;
}
impl fmt::Display for CopyAccelerationStructureModeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::CLONE_KHR => write!(f, "CLONE_KHR"),
            Self::COMPACT_KHR => write!(f, "COMPACT_KHR"),
            Self::SERIALIZE_KHR => write!(f, "SERIALIZE_KHR"),
            Self::DESERIALIZE_KHR => write!(f, "DESERIALIZE_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureTypeKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AccelerationStructureTypeKHR(i32);
impl AccelerationStructureTypeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl AccelerationStructureTypeKHR {
    pub const TOP_LEVEL_KHR: Self = Self(0);
    pub const BOTTOM_LEVEL_KHR: Self = Self(1);
    pub const GENERIC_KHR: Self = Self(2);
    pub const TOP_LEVEL_NV: Self = Self::TOP_LEVEL_KHR;
    pub const BOTTOM_LEVEL_NV: Self = Self::BOTTOM_LEVEL_KHR;
}
impl fmt::Display for AccelerationStructureTypeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TOP_LEVEL_KHR => write!(f, "TOP_LEVEL_KHR"),
            Self::BOTTOM_LEVEL_KHR => write!(f, "BOTTOM_LEVEL_KHR"),
            Self::GENERIC_KHR => write!(f, "GENERIC_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryTypeKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct GeometryTypeKHR(i32);
impl GeometryTypeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl GeometryTypeKHR {
    pub const TRIANGLES_KHR: Self = Self(0);
    pub const AABBS_KHR: Self = Self(1);
    pub const INSTANCES_KHR: Self = Self(2);
    pub const TRIANGLES_NV: Self = Self::TRIANGLES_KHR;
    pub const AABBS_NV: Self = Self::AABBS_KHR;
    pub const SPHERES_NV: Self = Self(1000429004);
    pub const LINEAR_SWEPT_SPHERES_NV: Self = Self(1000429005);
    pub const DENSE_GEOMETRY_FORMAT_TRIANGLES_AMDX: Self = Self(1000478000);
}
impl fmt::Display for GeometryTypeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TRIANGLES_KHR => write!(f, "TRIANGLES_KHR"),
            Self::AABBS_KHR => write!(f, "AABBS_KHR"),
            Self::INSTANCES_KHR => write!(f, "INSTANCES_KHR"),
            Self::SPHERES_NV => write!(f, "SPHERES_NV"),
            Self::LINEAR_SWEPT_SPHERES_NV => write!(f, "LINEAR_SWEPT_SPHERES_NV"),
            Self::DENSE_GEOMETRY_FORMAT_TRIANGLES_AMDX => {
                write!(f, "DENSE_GEOMETRY_FORMAT_TRIANGLES_AMDX")
            }
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingShaderGroupTypeKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct RayTracingShaderGroupTypeKHR(i32);
impl RayTracingShaderGroupTypeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl RayTracingShaderGroupTypeKHR {
    pub const GENERAL_KHR: Self = Self(0);
    pub const TRIANGLES_HIT_GROUP_KHR: Self = Self(1);
    pub const PROCEDURAL_HIT_GROUP_KHR: Self = Self(2);
    pub const GENERAL_NV: Self = Self::GENERAL_KHR;
    pub const TRIANGLES_HIT_GROUP_NV: Self = Self::TRIANGLES_HIT_GROUP_KHR;
    pub const PROCEDURAL_HIT_GROUP_NV: Self = Self::PROCEDURAL_HIT_GROUP_KHR;
}
impl fmt::Display for RayTracingShaderGroupTypeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::GENERAL_KHR => write!(f, "GENERAL_KHR"),
            Self::TRIANGLES_HIT_GROUP_KHR => write!(f, "TRIANGLES_HIT_GROUP_KHR"),
            Self::PROCEDURAL_HIT_GROUP_KHR => write!(f, "PROCEDURAL_HIT_GROUP_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMemoryRequirementsTypeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AccelerationStructureMemoryRequirementsTypeNV(i32);
impl AccelerationStructureMemoryRequirementsTypeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl AccelerationStructureMemoryRequirementsTypeNV {
    pub const OBJECT_NV: Self = Self(0);
    pub const BUILD_SCRATCH_NV: Self = Self(1);
    pub const UPDATE_SCRATCH_NV: Self = Self(2);
}
impl fmt::Display for AccelerationStructureMemoryRequirementsTypeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::OBJECT_NV => write!(f, "OBJECT_NV"),
            Self::BUILD_SCRATCH_NV => write!(f, "BUILD_SCRATCH_NV"),
            Self::UPDATE_SCRATCH_NV => write!(f, "UPDATE_SCRATCH_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureBuildTypeKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AccelerationStructureBuildTypeKHR(i32);
impl AccelerationStructureBuildTypeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl AccelerationStructureBuildTypeKHR {
    pub const HOST_KHR: Self = Self(0);
    pub const DEVICE_KHR: Self = Self(1);
    pub const HOST_OR_DEVICE_KHR: Self = Self(2);
}
impl fmt::Display for AccelerationStructureBuildTypeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::HOST_KHR => write!(f, "HOST_KHR"),
            Self::DEVICE_KHR => write!(f, "DEVICE_KHR"),
            Self::HOST_OR_DEVICE_KHR => write!(f, "HOST_OR_DEVICE_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCompatibilityKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AccelerationStructureCompatibilityKHR(i32);
impl AccelerationStructureCompatibilityKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl AccelerationStructureCompatibilityKHR {
    pub const COMPATIBLE_KHR: Self = Self(0);
    pub const INCOMPATIBLE_KHR: Self = Self(1);
}
impl fmt::Display for AccelerationStructureCompatibilityKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::COMPATIBLE_KHR => write!(f, "COMPATIBLE_KHR"),
            Self::INCOMPATIBLE_KHR => write!(f, "INCOMPATIBLE_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingLssIndexingModeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct RayTracingLssIndexingModeNV(i32);
impl RayTracingLssIndexingModeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl RayTracingLssIndexingModeNV {
    pub const LIST_NV: Self = Self(0);
    pub const SUCCESSIVE_NV: Self = Self(1);
}
impl fmt::Display for RayTracingLssIndexingModeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LIST_NV => write!(f, "LIST_NV"),
            Self::SUCCESSIVE_NV => write!(f, "SUCCESSIVE_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingLssPrimitiveEndCapsModeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct RayTracingLssPrimitiveEndCapsModeNV(i32);
impl RayTracingLssPrimitiveEndCapsModeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl RayTracingLssPrimitiveEndCapsModeNV {
    pub const NONE_NV: Self = Self(0);
    pub const CHAINED_NV: Self = Self(1);
}
impl fmt::Display for RayTracingLssPrimitiveEndCapsModeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NONE_NV => write!(f, "NONE_NV"),
            Self::CHAINED_NV => write!(f, "CHAINED_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderGroupShaderKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ShaderGroupShaderKHR(i32);
impl ShaderGroupShaderKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ShaderGroupShaderKHR {
    pub const GENERAL_KHR: Self = Self(0);
    pub const CLOSEST_HIT_KHR: Self = Self(1);
    pub const ANY_HIT_KHR: Self = Self(2);
    pub const INTERSECTION_KHR: Self = Self(3);
}
impl fmt::Display for ShaderGroupShaderKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::GENERAL_KHR => write!(f, "GENERAL_KHR"),
            Self::CLOSEST_HIT_KHR => write!(f, "CLOSEST_HIT_KHR"),
            Self::ANY_HIT_KHR => write!(f, "ANY_HIT_KHR"),
            Self::INTERSECTION_KHR => write!(f, "INTERSECTION_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryOverallocationBehaviorAMD.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct MemoryOverallocationBehaviorAMD(i32);
impl MemoryOverallocationBehaviorAMD {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl MemoryOverallocationBehaviorAMD {
    pub const DEFAULT_AMD: Self = Self(0);
    pub const ALLOWED_AMD: Self = Self(1);
    pub const DISALLOWED_AMD: Self = Self(2);
}
impl fmt::Display for MemoryOverallocationBehaviorAMD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DEFAULT_AMD => write!(f, "DEFAULT_AMD"),
            Self::ALLOWED_AMD => write!(f, "ALLOWED_AMD"),
            Self::DISALLOWED_AMD => write!(f, "DISALLOWED_AMD"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterScopeKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PerformanceCounterScopeKHR(i32);
impl PerformanceCounterScopeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PerformanceCounterScopeKHR {
    pub const COMMAND_BUFFER_KHR: Self = Self(0);
    pub const RENDER_PASS_KHR: Self = Self(1);
    pub const COMMAND_KHR: Self = Self(2);
    #[deprecated = "aliased"]
    pub const QUERY_SCOPE_COMMAND_BUFFER_KHR: Self = Self::COMMAND_BUFFER_KHR;
    #[deprecated = "aliased"]
    pub const QUERY_SCOPE_RENDER_PASS_KHR: Self = Self::RENDER_PASS_KHR;
    #[deprecated = "aliased"]
    pub const QUERY_SCOPE_COMMAND_KHR: Self = Self::COMMAND_KHR;
}
impl fmt::Display for PerformanceCounterScopeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::COMMAND_BUFFER_KHR => write!(f, "COMMAND_BUFFER_KHR"),
            Self::RENDER_PASS_KHR => write!(f, "RENDER_PASS_KHR"),
            Self::COMMAND_KHR => write!(f, "COMMAND_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterUnitKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PerformanceCounterUnitKHR(i32);
impl PerformanceCounterUnitKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PerformanceCounterUnitKHR {
    pub const GENERIC_KHR: Self = Self(0);
    pub const PERCENTAGE_KHR: Self = Self(1);
    pub const NANOSECONDS_KHR: Self = Self(2);
    pub const BYTES_KHR: Self = Self(3);
    pub const BYTES_PER_SECOND_KHR: Self = Self(4);
    pub const KELVIN_KHR: Self = Self(5);
    pub const WATTS_KHR: Self = Self(6);
    pub const VOLTS_KHR: Self = Self(7);
    pub const AMPS_KHR: Self = Self(8);
    pub const HERTZ_KHR: Self = Self(9);
    pub const CYCLES_KHR: Self = Self(10);
}
impl fmt::Display for PerformanceCounterUnitKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::GENERIC_KHR => write!(f, "GENERIC_KHR"),
            Self::PERCENTAGE_KHR => write!(f, "PERCENTAGE_KHR"),
            Self::NANOSECONDS_KHR => write!(f, "NANOSECONDS_KHR"),
            Self::BYTES_KHR => write!(f, "BYTES_KHR"),
            Self::BYTES_PER_SECOND_KHR => write!(f, "BYTES_PER_SECOND_KHR"),
            Self::KELVIN_KHR => write!(f, "KELVIN_KHR"),
            Self::WATTS_KHR => write!(f, "WATTS_KHR"),
            Self::VOLTS_KHR => write!(f, "VOLTS_KHR"),
            Self::AMPS_KHR => write!(f, "AMPS_KHR"),
            Self::HERTZ_KHR => write!(f, "HERTZ_KHR"),
            Self::CYCLES_KHR => write!(f, "CYCLES_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterStorageKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PerformanceCounterStorageKHR(i32);
impl PerformanceCounterStorageKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PerformanceCounterStorageKHR {
    pub const INT32_KHR: Self = Self(0);
    pub const INT64_KHR: Self = Self(1);
    pub const UINT32_KHR: Self = Self(2);
    pub const UINT64_KHR: Self = Self(3);
    pub const FLOAT32_KHR: Self = Self(4);
    pub const FLOAT64_KHR: Self = Self(5);
}
impl fmt::Display for PerformanceCounterStorageKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::INT32_KHR => write!(f, "INT32_KHR"),
            Self::INT64_KHR => write!(f, "INT64_KHR"),
            Self::UINT32_KHR => write!(f, "UINT32_KHR"),
            Self::UINT64_KHR => write!(f, "UINT64_KHR"),
            Self::FLOAT32_KHR => write!(f, "FLOAT32_KHR"),
            Self::FLOAT64_KHR => write!(f, "FLOAT64_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceConfigurationTypeINTEL.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PerformanceConfigurationTypeINTEL(i32);
impl PerformanceConfigurationTypeINTEL {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PerformanceConfigurationTypeINTEL {
    pub const COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: Self = Self(0);
}
impl fmt::Display for PerformanceConfigurationTypeINTEL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL {
            write!(f, "COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolSamplingModeINTEL.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct QueryPoolSamplingModeINTEL(i32);
impl QueryPoolSamplingModeINTEL {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl QueryPoolSamplingModeINTEL {
    pub const MANUAL_INTEL: Self = Self(0);
}
impl fmt::Display for QueryPoolSamplingModeINTEL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::MANUAL_INTEL {
            write!(f, "MANUAL_INTEL")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceOverrideTypeINTEL.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PerformanceOverrideTypeINTEL(i32);
impl PerformanceOverrideTypeINTEL {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PerformanceOverrideTypeINTEL {
    pub const NULL_HARDWARE_INTEL: Self = Self(0);
    pub const FLUSH_GPU_CACHES_INTEL: Self = Self(1);
}
impl fmt::Display for PerformanceOverrideTypeINTEL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NULL_HARDWARE_INTEL => write!(f, "NULL_HARDWARE_INTEL"),
            Self::FLUSH_GPU_CACHES_INTEL => write!(f, "FLUSH_GPU_CACHES_INTEL"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceParameterTypeINTEL.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PerformanceParameterTypeINTEL(i32);
impl PerformanceParameterTypeINTEL {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PerformanceParameterTypeINTEL {
    pub const HW_COUNTERS_SUPPORTED_INTEL: Self = Self(0);
    pub const STREAM_MARKER_VALID_BITS_INTEL: Self = Self(1);
}
impl fmt::Display for PerformanceParameterTypeINTEL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::HW_COUNTERS_SUPPORTED_INTEL => write!(f, "HW_COUNTERS_SUPPORTED_INTEL"),
            Self::STREAM_MARKER_VALID_BITS_INTEL => write!(f, "STREAM_MARKER_VALID_BITS_INTEL"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceValueTypeINTEL.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PerformanceValueTypeINTEL(i32);
impl PerformanceValueTypeINTEL {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PerformanceValueTypeINTEL {
    pub const UINT32_INTEL: Self = Self(0);
    pub const UINT64_INTEL: Self = Self(1);
    pub const FLOAT_INTEL: Self = Self(2);
    pub const BOOL_INTEL: Self = Self(3);
    pub const STRING_INTEL: Self = Self(4);
}
impl fmt::Display for PerformanceValueTypeINTEL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UINT32_INTEL => write!(f, "UINT32_INTEL"),
            Self::UINT64_INTEL => write!(f, "UINT64_INTEL"),
            Self::FLOAT_INTEL => write!(f, "FLOAT_INTEL"),
            Self::BOOL_INTEL => write!(f, "BOOL_INTEL"),
            Self::STRING_INTEL => write!(f, "STRING_INTEL"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkLineRasterizationMode.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LineRasterizationMode(i32);
impl LineRasterizationMode {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl LineRasterizationMode {
    pub const DEFAULT: Self = Self(0);
    pub const RECTANGULAR: Self = Self(1);
    pub const BRESENHAM: Self = Self(2);
    pub const RECTANGULAR_SMOOTH: Self = Self(3);
    pub const DEFAULT_EXT: Self = Self::DEFAULT;
    pub const RECTANGULAR_EXT: Self = Self::RECTANGULAR;
    pub const BRESENHAM_EXT: Self = Self::BRESENHAM;
    pub const RECTANGULAR_SMOOTH_EXT: Self = Self::RECTANGULAR_SMOOTH;
    pub const DEFAULT_KHR: Self = Self::DEFAULT;
    pub const RECTANGULAR_KHR: Self = Self::RECTANGULAR;
    pub const BRESENHAM_KHR: Self = Self::BRESENHAM;
    pub const RECTANGULAR_SMOOTH_KHR: Self = Self::RECTANGULAR_SMOOTH;
}
impl fmt::Display for LineRasterizationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DEFAULT => write!(f, "DEFAULT"),
            Self::RECTANGULAR => write!(f, "RECTANGULAR"),
            Self::BRESENHAM => write!(f, "BRESENHAM"),
            Self::RECTANGULAR_SMOOTH => write!(f, "RECTANGULAR_SMOOTH"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFaultLevel.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FaultLevel(i32);
impl FaultLevel {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl FaultLevel {
    pub const UNASSIGNED: Self = Self(0);
    pub const CRITICAL: Self = Self(1);
    pub const RECOVERABLE: Self = Self(2);
    pub const WARNING: Self = Self(3);
}
impl fmt::Display for FaultLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UNASSIGNED => write!(f, "UNASSIGNED"),
            Self::CRITICAL => write!(f, "CRITICAL"),
            Self::RECOVERABLE => write!(f, "RECOVERABLE"),
            Self::WARNING => write!(f, "WARNING"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFaultType.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FaultType(i32);
impl FaultType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl FaultType {
    pub const INVALID: Self = Self(0);
    pub const UNASSIGNED: Self = Self(1);
    pub const IMPLEMENTATION: Self = Self(2);
    pub const SYSTEM: Self = Self(3);
    pub const PHYSICAL_DEVICE: Self = Self(4);
    pub const COMMAND_BUFFER_FULL: Self = Self(5);
    pub const INVALID_API_USAGE: Self = Self(6);
}
impl fmt::Display for FaultType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::INVALID => write!(f, "INVALID"),
            Self::UNASSIGNED => write!(f, "UNASSIGNED"),
            Self::IMPLEMENTATION => write!(f, "IMPLEMENTATION"),
            Self::SYSTEM => write!(f, "SYSTEM"),
            Self::PHYSICAL_DEVICE => write!(f, "PHYSICAL_DEVICE"),
            Self::COMMAND_BUFFER_FULL => write!(f, "COMMAND_BUFFER_FULL"),
            Self::INVALID_API_USAGE => write!(f, "INVALID_API_USAGE"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFaultQueryBehavior.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FaultQueryBehavior(i32);
impl FaultQueryBehavior {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl FaultQueryBehavior {
    pub const GET_AND_CLEAR_ALL_FAULTS: Self = Self(0);
}
impl fmt::Display for FaultQueryBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::GET_AND_CLEAR_ALL_FAULTS {
            write!(f, "GET_AND_CLEAR_ALL_FAULTS")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineMatchControl.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PipelineMatchControl(i32);
impl PipelineMatchControl {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PipelineMatchControl {
    pub const APPLICATION_UUID_EXACT_MATCH: Self = Self(0);
}
impl fmt::Display for PipelineMatchControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::APPLICATION_UUID_EXACT_MATCH {
            write!(f, "APPLICATION_UUID_EXACT_MATCH")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSciSyncClientTypeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SciSyncClientTypeNV(i32);
impl SciSyncClientTypeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl SciSyncClientTypeNV {
    pub const SIGNALER_NV: Self = Self(0);
    pub const WAITER_NV: Self = Self(1);
    pub const SIGNALER_WAITER_NV: Self = Self(2);
}
impl fmt::Display for SciSyncClientTypeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::SIGNALER_NV => write!(f, "SIGNALER_NV"),
            Self::WAITER_NV => write!(f, "WAITER_NV"),
            Self::SIGNALER_WAITER_NV => write!(f, "SIGNALER_WAITER_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSciSyncPrimitiveTypeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SciSyncPrimitiveTypeNV(i32);
impl SciSyncPrimitiveTypeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl SciSyncPrimitiveTypeNV {
    pub const FENCE_NV: Self = Self(0);
    pub const SEMAPHORE_NV: Self = Self(1);
}
impl fmt::Display for SciSyncPrimitiveTypeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::FENCE_NV => write!(f, "FENCE_NV"),
            Self::SEMAPHORE_NV => write!(f, "SEMAPHORE_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFragmentShadingRateNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FragmentShadingRateNV(i32);
impl FragmentShadingRateNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl FragmentShadingRateNV {
    pub const TYPE_1_INVOCATION_PER_PIXEL_NV: Self = Self(0);
    pub const TYPE_1_INVOCATION_PER_1X2_PIXELS_NV: Self = Self(1);
    pub const TYPE_1_INVOCATION_PER_2X1_PIXELS_NV: Self = Self(4);
    pub const TYPE_1_INVOCATION_PER_2X2_PIXELS_NV: Self = Self(5);
    pub const TYPE_1_INVOCATION_PER_2X4_PIXELS_NV: Self = Self(6);
    pub const TYPE_1_INVOCATION_PER_4X2_PIXELS_NV: Self = Self(9);
    pub const TYPE_1_INVOCATION_PER_4X4_PIXELS_NV: Self = Self(10);
    pub const TYPE_2_INVOCATIONS_PER_PIXEL_NV: Self = Self(11);
    pub const TYPE_4_INVOCATIONS_PER_PIXEL_NV: Self = Self(12);
    pub const TYPE_8_INVOCATIONS_PER_PIXEL_NV: Self = Self(13);
    pub const TYPE_16_INVOCATIONS_PER_PIXEL_NV: Self = Self(14);
    pub const NO_INVOCATIONS_NV: Self = Self(15);
}
impl fmt::Display for FragmentShadingRateNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TYPE_1_INVOCATION_PER_PIXEL_NV => write!(f, "TYPE_1_INVOCATION_PER_PIXEL_NV"),
            Self::TYPE_1_INVOCATION_PER_1X2_PIXELS_NV => {
                write!(f, "TYPE_1_INVOCATION_PER_1X2_PIXELS_NV")
            }
            Self::TYPE_1_INVOCATION_PER_2X1_PIXELS_NV => {
                write!(f, "TYPE_1_INVOCATION_PER_2X1_PIXELS_NV")
            }
            Self::TYPE_1_INVOCATION_PER_2X2_PIXELS_NV => {
                write!(f, "TYPE_1_INVOCATION_PER_2X2_PIXELS_NV")
            }
            Self::TYPE_1_INVOCATION_PER_2X4_PIXELS_NV => {
                write!(f, "TYPE_1_INVOCATION_PER_2X4_PIXELS_NV")
            }
            Self::TYPE_1_INVOCATION_PER_4X2_PIXELS_NV => {
                write!(f, "TYPE_1_INVOCATION_PER_4X2_PIXELS_NV")
            }
            Self::TYPE_1_INVOCATION_PER_4X4_PIXELS_NV => {
                write!(f, "TYPE_1_INVOCATION_PER_4X4_PIXELS_NV")
            }
            Self::TYPE_2_INVOCATIONS_PER_PIXEL_NV => write!(f, "TYPE_2_INVOCATIONS_PER_PIXEL_NV"),
            Self::TYPE_4_INVOCATIONS_PER_PIXEL_NV => write!(f, "TYPE_4_INVOCATIONS_PER_PIXEL_NV"),
            Self::TYPE_8_INVOCATIONS_PER_PIXEL_NV => write!(f, "TYPE_8_INVOCATIONS_PER_PIXEL_NV"),
            Self::TYPE_16_INVOCATIONS_PER_PIXEL_NV => write!(f, "TYPE_16_INVOCATIONS_PER_PIXEL_NV"),
            Self::NO_INVOCATIONS_NV => write!(f, "NO_INVOCATIONS_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFragmentShadingRateTypeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FragmentShadingRateTypeNV(i32);
impl FragmentShadingRateTypeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl FragmentShadingRateTypeNV {
    pub const FRAGMENT_SIZE_NV: Self = Self(0);
    pub const ENUMS_NV: Self = Self(1);
}
impl fmt::Display for FragmentShadingRateTypeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::FRAGMENT_SIZE_NV => write!(f, "FRAGMENT_SIZE_NV"),
            Self::ENUMS_NV => write!(f, "ENUMS_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassMergeStatusEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SubpassMergeStatusEXT(i32);
impl SubpassMergeStatusEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl SubpassMergeStatusEXT {
    pub const MERGED_EXT: Self = Self(0);
    pub const DISALLOWED_EXT: Self = Self(1);
    pub const NOT_MERGED_SIDE_EFFECTS_EXT: Self = Self(2);
    pub const NOT_MERGED_SAMPLES_MISMATCH_EXT: Self = Self(3);
    pub const NOT_MERGED_VIEWS_MISMATCH_EXT: Self = Self(4);
    pub const NOT_MERGED_ALIASING_EXT: Self = Self(5);
    pub const NOT_MERGED_DEPENDENCIES_EXT: Self = Self(6);
    pub const NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT: Self = Self(7);
    pub const NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT: Self = Self(8);
    pub const NOT_MERGED_INSUFFICIENT_STORAGE_EXT: Self = Self(9);
    pub const NOT_MERGED_DEPTH_STENCIL_COUNT_EXT: Self = Self(10);
    pub const NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT: Self = Self(11);
    pub const NOT_MERGED_SINGLE_SUBPASS_EXT: Self = Self(12);
    pub const NOT_MERGED_UNSPECIFIED_EXT: Self = Self(13);
}
impl fmt::Display for SubpassMergeStatusEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::MERGED_EXT => write!(f, "MERGED_EXT"),
            Self::DISALLOWED_EXT => write!(f, "DISALLOWED_EXT"),
            Self::NOT_MERGED_SIDE_EFFECTS_EXT => write!(f, "NOT_MERGED_SIDE_EFFECTS_EXT"),
            Self::NOT_MERGED_SAMPLES_MISMATCH_EXT => write!(f, "NOT_MERGED_SAMPLES_MISMATCH_EXT"),
            Self::NOT_MERGED_VIEWS_MISMATCH_EXT => write!(f, "NOT_MERGED_VIEWS_MISMATCH_EXT"),
            Self::NOT_MERGED_ALIASING_EXT => write!(f, "NOT_MERGED_ALIASING_EXT"),
            Self::NOT_MERGED_DEPENDENCIES_EXT => write!(f, "NOT_MERGED_DEPENDENCIES_EXT"),
            Self::NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT => {
                write!(f, "NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT")
            }
            Self::NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT => {
                write!(f, "NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT")
            }
            Self::NOT_MERGED_INSUFFICIENT_STORAGE_EXT => {
                write!(f, "NOT_MERGED_INSUFFICIENT_STORAGE_EXT")
            }
            Self::NOT_MERGED_DEPTH_STENCIL_COUNT_EXT => {
                write!(f, "NOT_MERGED_DEPTH_STENCIL_COUNT_EXT")
            }
            Self::NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT => {
                write!(f, "NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT")
            }
            Self::NOT_MERGED_SINGLE_SUBPASS_EXT => write!(f, "NOT_MERGED_SINGLE_SUBPASS_EXT"),
            Self::NOT_MERGED_UNSPECIFIED_EXT => write!(f, "NOT_MERGED_UNSPECIFIED_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkProvokingVertexModeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ProvokingVertexModeEXT(i32);
impl ProvokingVertexModeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ProvokingVertexModeEXT {
    pub const FIRST_VERTEX_EXT: Self = Self(0);
    pub const LAST_VERTEX_EXT: Self = Self(1);
}
impl fmt::Display for ProvokingVertexModeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::FIRST_VERTEX_EXT => write!(f, "FIRST_VERTEX_EXT"),
            Self::LAST_VERTEX_EXT => write!(f, "LAST_VERTEX_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheValidationVersion.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PipelineCacheValidationVersion(i32);
impl PipelineCacheValidationVersion {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PipelineCacheValidationVersion {
    pub const SAFETY_CRITICAL_ONE: Self = Self(1);
}
impl fmt::Display for PipelineCacheValidationVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::SAFETY_CRITICAL_ONE {
            write!(f, "SAFETY_CRITICAL_ONE")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessBufferBehavior.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PipelineRobustnessBufferBehavior(i32);
impl PipelineRobustnessBufferBehavior {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PipelineRobustnessBufferBehavior {
    pub const DEVICE_DEFAULT: Self = Self(0);
    pub const DISABLED: Self = Self(1);
    pub const ROBUST_BUFFER_ACCESS: Self = Self(2);
    pub const ROBUST_BUFFER_ACCESS_2: Self = Self(3);
    pub const DEVICE_DEFAULT_EXT: Self = Self::DEVICE_DEFAULT;
    pub const DISABLED_EXT: Self = Self::DISABLED;
    pub const ROBUST_BUFFER_ACCESS_EXT: Self = Self::ROBUST_BUFFER_ACCESS;
    pub const ROBUST_BUFFER_ACCESS_2_EXT: Self = Self::ROBUST_BUFFER_ACCESS_2;
}
impl fmt::Display for PipelineRobustnessBufferBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DEVICE_DEFAULT => write!(f, "DEVICE_DEFAULT"),
            Self::DISABLED => write!(f, "DISABLED"),
            Self::ROBUST_BUFFER_ACCESS => write!(f, "ROBUST_BUFFER_ACCESS"),
            Self::ROBUST_BUFFER_ACCESS_2 => write!(f, "ROBUST_BUFFER_ACCESS_2"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessImageBehavior.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PipelineRobustnessImageBehavior(i32);
impl PipelineRobustnessImageBehavior {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PipelineRobustnessImageBehavior {
    pub const DEVICE_DEFAULT: Self = Self(0);
    pub const DISABLED: Self = Self(1);
    pub const ROBUST_IMAGE_ACCESS: Self = Self(2);
    pub const ROBUST_IMAGE_ACCESS_2: Self = Self(3);
    pub const DEVICE_DEFAULT_EXT: Self = Self::DEVICE_DEFAULT;
    pub const DISABLED_EXT: Self = Self::DISABLED;
    pub const ROBUST_IMAGE_ACCESS_EXT: Self = Self::ROBUST_IMAGE_ACCESS;
    pub const ROBUST_IMAGE_ACCESS_2_EXT: Self = Self::ROBUST_IMAGE_ACCESS_2;
}
impl fmt::Display for PipelineRobustnessImageBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DEVICE_DEFAULT => write!(f, "DEVICE_DEFAULT"),
            Self::DISABLED => write!(f, "DISABLED"),
            Self::ROBUST_IMAGE_ACCESS => write!(f, "ROBUST_IMAGE_ACCESS"),
            Self::ROBUST_IMAGE_ACCESS_2 => write!(f, "ROBUST_IMAGE_ACCESS_2"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddressBindingTypeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DeviceAddressBindingTypeEXT(i32);
impl DeviceAddressBindingTypeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DeviceAddressBindingTypeEXT {
    pub const BIND_EXT: Self = Self(0);
    pub const UNBIND_EXT: Self = Self(1);
}
impl fmt::Display for DeviceAddressBindingTypeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::BIND_EXT => write!(f, "BIND_EXT"),
            Self::UNBIND_EXT => write!(f, "UNBIND_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapTypeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct MicromapTypeEXT(i32);
impl MicromapTypeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl MicromapTypeEXT {
    pub const OPACITY_MICROMAP_EXT: Self = Self(0);
    pub const DISPLACEMENT_MICROMAP_NV: Self = Self(1000397000);
}
impl fmt::Display for MicromapTypeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::OPACITY_MICROMAP_EXT => write!(f, "OPACITY_MICROMAP_EXT"),
            Self::DISPLACEMENT_MICROMAP_NV => write!(f, "DISPLACEMENT_MICROMAP_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildMicromapModeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BuildMicromapModeEXT(i32);
impl BuildMicromapModeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl BuildMicromapModeEXT {
    pub const BUILD_EXT: Self = Self(0);
}
impl fmt::Display for BuildMicromapModeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::BUILD_EXT {
            write!(f, "BUILD_EXT")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMicromapModeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CopyMicromapModeEXT(i32);
impl CopyMicromapModeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl CopyMicromapModeEXT {
    pub const CLONE_EXT: Self = Self(0);
    pub const SERIALIZE_EXT: Self = Self(1);
    pub const DESERIALIZE_EXT: Self = Self(2);
    pub const COMPACT_EXT: Self = Self(3);
}
impl fmt::Display for CopyMicromapModeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::CLONE_EXT => write!(f, "CLONE_EXT"),
            Self::SERIALIZE_EXT => write!(f, "SERIALIZE_EXT"),
            Self::DESERIALIZE_EXT => write!(f, "DESERIALIZE_EXT"),
            Self::COMPACT_EXT => write!(f, "COMPACT_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkOpacityMicromapFormatEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct OpacityMicromapFormatEXT(i32);
impl OpacityMicromapFormatEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl OpacityMicromapFormatEXT {
    pub const TYPE_2_STATE_EXT: Self = Self(1);
    pub const TYPE_4_STATE_EXT: Self = Self(2);
}
impl fmt::Display for OpacityMicromapFormatEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TYPE_2_STATE_EXT => write!(f, "TYPE_2_STATE_EXT"),
            Self::TYPE_4_STATE_EXT => write!(f, "TYPE_4_STATE_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkOpacityMicromapSpecialIndexEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct OpacityMicromapSpecialIndexEXT(i32);
impl OpacityMicromapSpecialIndexEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl OpacityMicromapSpecialIndexEXT {
    pub const FULLY_TRANSPARENT_EXT: Self = Self(-1);
    pub const FULLY_OPAQUE_EXT: Self = Self(-2);
    pub const FULLY_UNKNOWN_TRANSPARENT_EXT: Self = Self(-3);
    pub const FULLY_UNKNOWN_OPAQUE_EXT: Self = Self(-4);
    pub const CLUSTER_GEOMETRY_DISABLE_OPACITY_MICROMAP_NV: Self = Self(-5);
}
impl fmt::Display for OpacityMicromapSpecialIndexEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::FULLY_TRANSPARENT_EXT => write!(f, "FULLY_TRANSPARENT_EXT"),
            Self::FULLY_OPAQUE_EXT => write!(f, "FULLY_OPAQUE_EXT"),
            Self::FULLY_UNKNOWN_TRANSPARENT_EXT => write!(f, "FULLY_UNKNOWN_TRANSPARENT_EXT"),
            Self::FULLY_UNKNOWN_OPAQUE_EXT => write!(f, "FULLY_UNKNOWN_OPAQUE_EXT"),
            Self::CLUSTER_GEOMETRY_DISABLE_OPACITY_MICROMAP_NV => {
                write!(f, "CLUSTER_GEOMETRY_DISABLE_OPACITY_MICROMAP_NV")
            }
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetInfoTypeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct IndirectExecutionSetInfoTypeEXT(i32);
impl IndirectExecutionSetInfoTypeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl IndirectExecutionSetInfoTypeEXT {
    pub const PIPELINES_EXT: Self = Self(0);
    pub const SHADER_OBJECTS_EXT: Self = Self(1);
}
impl fmt::Display for IndirectExecutionSetInfoTypeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::PIPELINES_EXT => write!(f, "PIPELINES_EXT"),
            Self::SHADER_OBJECTS_EXT => write!(f, "SHADER_OBJECTS_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultVendorBinaryHeaderVersionKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DeviceFaultVendorBinaryHeaderVersionKHR(i32);
impl DeviceFaultVendorBinaryHeaderVersionKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DeviceFaultVendorBinaryHeaderVersionKHR {
    pub const ONE_KHR: Self = Self(1);
    pub const ONE_EXT: Self = Self::ONE_KHR;
}
impl fmt::Display for DeviceFaultVendorBinaryHeaderVersionKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ONE_KHR => write!(f, "ONE_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDepthBiasRepresentationEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DepthBiasRepresentationEXT(i32);
impl DepthBiasRepresentationEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DepthBiasRepresentationEXT {
    pub const LEAST_REPRESENTABLE_VALUE_FORMAT_EXT: Self = Self(0);
    pub const LEAST_REPRESENTABLE_VALUE_FORCE_UNORM_EXT: Self = Self(1);
    pub const FLOAT_EXT: Self = Self(2);
}
impl fmt::Display for DepthBiasRepresentationEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LEAST_REPRESENTABLE_VALUE_FORMAT_EXT => {
                write!(f, "LEAST_REPRESENTABLE_VALUE_FORMAT_EXT")
            }
            Self::LEAST_REPRESENTABLE_VALUE_FORCE_UNORM_EXT => {
                write!(f, "LEAST_REPRESENTABLE_VALUE_FORCE_UNORM_EXT")
            }
            Self::FLOAT_EXT => write!(f, "FLOAT_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectDriverLoadingModeLUNARG.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DirectDriverLoadingModeLUNARG(i32);
impl DirectDriverLoadingModeLUNARG {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DirectDriverLoadingModeLUNARG {
    pub const EXCLUSIVE_LUNARG: Self = Self(0);
    pub const INCLUSIVE_LUNARG: Self = Self(1);
}
impl fmt::Display for DirectDriverLoadingModeLUNARG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::EXCLUSIVE_LUNARG => write!(f, "EXCLUSIVE_LUNARG"),
            Self::INCLUSIVE_LUNARG => write!(f, "INCLUSIVE_LUNARG"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureOpTypeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PartitionedAccelerationStructureOpTypeNV(i32);
impl PartitionedAccelerationStructureOpTypeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PartitionedAccelerationStructureOpTypeNV {
    pub const WRITE_INSTANCE_NV: Self = Self(0);
    pub const UPDATE_INSTANCE_NV: Self = Self(1);
    pub const WRITE_PARTITION_TRANSLATION_NV: Self = Self(2);
}
impl fmt::Display for PartitionedAccelerationStructureOpTypeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::WRITE_INSTANCE_NV => write!(f, "WRITE_INSTANCE_NV"),
            Self::UPDATE_INSTANCE_NV => write!(f, "UPDATE_INSTANCE_NV"),
            Self::WRITE_PARTITION_TRANSLATION_NV => write!(f, "WRITE_PARTITION_TRANSLATION_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAntiLagModeAMD.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AntiLagModeAMD(i32);
impl AntiLagModeAMD {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl AntiLagModeAMD {
    pub const DRIVER_CONTROL_AMD: Self = Self(0);
    pub const ON_AMD: Self = Self(1);
    pub const OFF_AMD: Self = Self(2);
}
impl fmt::Display for AntiLagModeAMD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DRIVER_CONTROL_AMD => write!(f, "DRIVER_CONTROL_AMD"),
            Self::ON_AMD => write!(f, "ON_AMD"),
            Self::OFF_AMD => write!(f, "OFF_AMD"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAntiLagStageAMD.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AntiLagStageAMD(i32);
impl AntiLagStageAMD {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl AntiLagStageAMD {
    pub const INPUT_AMD: Self = Self(0);
    pub const PRESENT_AMD: Self = Self(1);
}
impl fmt::Display for AntiLagStageAMD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::INPUT_AMD => write!(f, "INPUT_AMD"),
            Self::PRESENT_AMD => write!(f, "PRESENT_AMD"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplacementMicromapFormatNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DisplacementMicromapFormatNV(i32);
impl DisplacementMicromapFormatNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DisplacementMicromapFormatNV {
    pub const TYPE_64_TRIANGLES_64_BYTES_NV: Self = Self(1);
    pub const TYPE_256_TRIANGLES_128_BYTES_NV: Self = Self(2);
    pub const TYPE_1024_TRIANGLES_128_BYTES_NV: Self = Self(3);
}
impl fmt::Display for DisplacementMicromapFormatNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TYPE_64_TRIANGLES_64_BYTES_NV => write!(f, "TYPE_64_TRIANGLES_64_BYTES_NV"),
            Self::TYPE_256_TRIANGLES_128_BYTES_NV => write!(f, "TYPE_256_TRIANGLES_128_BYTES_NV"),
            Self::TYPE_1024_TRIANGLES_128_BYTES_NV => write!(f, "TYPE_1024_TRIANGLES_128_BYTES_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCodeTypeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ShaderCodeTypeEXT(i32);
impl ShaderCodeTypeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ShaderCodeTypeEXT {
    pub const BINARY_EXT: Self = Self(0);
    pub const SPIRV_EXT: Self = Self(1);
}
impl fmt::Display for ShaderCodeTypeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::BINARY_EXT => write!(f, "BINARY_EXT"),
            Self::SPIRV_EXT => write!(f, "SPIRV_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkScopeKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ScopeKHR(i32);
impl ScopeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ScopeKHR {
    pub const DEVICE_KHR: Self = Self(1);
    pub const WORKGROUP_KHR: Self = Self(2);
    pub const SUBGROUP_KHR: Self = Self(3);
    pub const QUEUE_FAMILY_KHR: Self = Self(5);
    pub const DEVICE_NV: Self = Self::DEVICE_KHR;
    pub const WORKGROUP_NV: Self = Self::WORKGROUP_KHR;
    pub const SUBGROUP_NV: Self = Self::SUBGROUP_KHR;
    pub const QUEUE_FAMILY_NV: Self = Self::QUEUE_FAMILY_KHR;
}
impl fmt::Display for ScopeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DEVICE_KHR => write!(f, "DEVICE_KHR"),
            Self::WORKGROUP_KHR => write!(f, "WORKGROUP_KHR"),
            Self::SUBGROUP_KHR => write!(f, "SUBGROUP_KHR"),
            Self::QUEUE_FAMILY_KHR => write!(f, "QUEUE_FAMILY_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkComponentTypeKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ComponentTypeKHR(i32);
impl ComponentTypeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ComponentTypeKHR {
    pub const FLOAT16_KHR: Self = Self(0);
    pub const FLOAT32_KHR: Self = Self(1);
    pub const FLOAT64_KHR: Self = Self(2);
    pub const SINT8_KHR: Self = Self(3);
    pub const SINT16_KHR: Self = Self(4);
    pub const SINT32_KHR: Self = Self(5);
    pub const SINT64_KHR: Self = Self(6);
    pub const UINT8_KHR: Self = Self(7);
    pub const UINT16_KHR: Self = Self(8);
    pub const UINT32_KHR: Self = Self(9);
    pub const UINT64_KHR: Self = Self(10);
    pub const BFLOAT16_KHR: Self = Self(1000141000);
    pub const FLOAT16_NV: Self = Self::FLOAT16_KHR;
    pub const FLOAT32_NV: Self = Self::FLOAT32_KHR;
    pub const FLOAT64_NV: Self = Self::FLOAT64_KHR;
    pub const SINT8_NV: Self = Self::SINT8_KHR;
    pub const SINT16_NV: Self = Self::SINT16_KHR;
    pub const SINT32_NV: Self = Self::SINT32_KHR;
    pub const SINT64_NV: Self = Self::SINT64_KHR;
    pub const UINT8_NV: Self = Self::UINT8_KHR;
    pub const UINT16_NV: Self = Self::UINT16_KHR;
    pub const UINT32_NV: Self = Self::UINT32_KHR;
    pub const UINT64_NV: Self = Self::UINT64_KHR;
    pub const SINT8_PACKED_NV: Self = Self(1000491000);
    pub const UINT8_PACKED_NV: Self = Self(1000491001);
    pub const FLOAT_E4M3_NV: Self = Self::FLOAT8_E4M3_EXT;
    pub const FLOAT_E5M2_NV: Self = Self::FLOAT8_E5M2_EXT;
    pub const FLOAT8_E4M3_EXT: Self = Self(1000491002);
    pub const FLOAT8_E5M2_EXT: Self = Self(1000491003);
}
impl fmt::Display for ComponentTypeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::FLOAT16_KHR => write!(f, "FLOAT16_KHR"),
            Self::FLOAT32_KHR => write!(f, "FLOAT32_KHR"),
            Self::FLOAT64_KHR => write!(f, "FLOAT64_KHR"),
            Self::SINT8_KHR => write!(f, "SINT8_KHR"),
            Self::SINT16_KHR => write!(f, "SINT16_KHR"),
            Self::SINT32_KHR => write!(f, "SINT32_KHR"),
            Self::SINT64_KHR => write!(f, "SINT64_KHR"),
            Self::UINT8_KHR => write!(f, "UINT8_KHR"),
            Self::UINT16_KHR => write!(f, "UINT16_KHR"),
            Self::UINT32_KHR => write!(f, "UINT32_KHR"),
            Self::UINT64_KHR => write!(f, "UINT64_KHR"),
            Self::BFLOAT16_KHR => write!(f, "BFLOAT16_KHR"),
            Self::SINT8_PACKED_NV => write!(f, "SINT8_PACKED_NV"),
            Self::UINT8_PACKED_NV => write!(f, "UINT8_PACKED_NV"),
            Self::FLOAT8_E4M3_EXT => write!(f, "FLOAT8_E4M3_EXT"),
            Self::FLOAT8_E5M2_EXT => write!(f, "FLOAT8_E5M2_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCubicFilterWeightsQCOM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CubicFilterWeightsQCOM(i32);
impl CubicFilterWeightsQCOM {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl CubicFilterWeightsQCOM {
    pub const CATMULL_ROM_QCOM: Self = Self(0);
    pub const ZERO_TANGENT_CARDINAL_QCOM: Self = Self(1);
    pub const B_SPLINE_QCOM: Self = Self(2);
    pub const MITCHELL_NETRAVALI_QCOM: Self = Self(3);
}
impl fmt::Display for CubicFilterWeightsQCOM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::CATMULL_ROM_QCOM => write!(f, "CATMULL_ROM_QCOM"),
            Self::ZERO_TANGENT_CARDINAL_QCOM => write!(f, "ZERO_TANGENT_CARDINAL_QCOM"),
            Self::B_SPLINE_QCOM => write!(f, "B_SPLINE_QCOM"),
            Self::MITCHELL_NETRAVALI_QCOM => write!(f, "MITCHELL_NETRAVALI_QCOM"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBlockMatchWindowCompareModeQCOM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BlockMatchWindowCompareModeQCOM(i32);
impl BlockMatchWindowCompareModeQCOM {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl BlockMatchWindowCompareModeQCOM {
    pub const MIN_QCOM: Self = Self(0);
    pub const MAX_QCOM: Self = Self(1);
}
impl fmt::Display for BlockMatchWindowCompareModeQCOM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::MIN_QCOM => write!(f, "MIN_QCOM"),
            Self::MAX_QCOM => write!(f, "MAX_QCOM"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkLayeredDriverUnderlyingApiMSFT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LayeredDriverUnderlyingApiMSFT(i32);
impl LayeredDriverUnderlyingApiMSFT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl LayeredDriverUnderlyingApiMSFT {
    pub const NONE_MSFT: Self = Self(0);
    pub const D3D12_MSFT: Self = Self(1);
}
impl fmt::Display for LayeredDriverUnderlyingApiMSFT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NONE_MSFT => write!(f, "NONE_MSFT"),
            Self::D3D12_MSFT => write!(f, "D3D12_MSFT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLayeredApiKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PhysicalDeviceLayeredApiKHR(i32);
impl PhysicalDeviceLayeredApiKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PhysicalDeviceLayeredApiKHR {
    pub const VULKAN_KHR: Self = Self(0);
    pub const D3D12_KHR: Self = Self(1);
    pub const METAL_KHR: Self = Self(2);
    pub const OPENGL_KHR: Self = Self(3);
    pub const OPENGLES_KHR: Self = Self(4);
}
impl fmt::Display for PhysicalDeviceLayeredApiKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::VULKAN_KHR => write!(f, "VULKAN_KHR"),
            Self::D3D12_KHR => write!(f, "D3D12_KHR"),
            Self::METAL_KHR => write!(f, "METAL_KHR"),
            Self::OPENGL_KHR => write!(f, "OPENGL_KHR"),
            Self::OPENGLES_KHR => write!(f, "OPENGLES_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCompressedTriangleFormatAMDX.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CompressedTriangleFormatAMDX(i32);
impl CompressedTriangleFormatAMDX {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl CompressedTriangleFormatAMDX {
    pub const DGF1_AMDX: Self = Self(0);
}
impl fmt::Display for CompressedTriangleFormatAMDX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::DGF1_AMDX {
            write!(f, "DGF1_AMDX")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDepthClampModeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DepthClampModeEXT(i32);
impl DepthClampModeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DepthClampModeEXT {
    pub const VIEWPORT_RANGE_EXT: Self = Self(0);
    pub const USER_DEFINED_RANGE_EXT: Self = Self(1);
}
impl fmt::Display for DepthClampModeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::VIEWPORT_RANGE_EXT => write!(f, "VIEWPORT_RANGE_EXT"),
            Self::USER_DEFINED_RANGE_EXT => write!(f, "USER_DEFINED_RANGE_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeVectorMatrixLayoutNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CooperativeVectorMatrixLayoutNV(i32);
impl CooperativeVectorMatrixLayoutNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl CooperativeVectorMatrixLayoutNV {
    pub const ROW_MAJOR_NV: Self = Self(0);
    pub const COLUMN_MAJOR_NV: Self = Self(1);
    pub const INFERENCING_OPTIMAL_NV: Self = Self(2);
    pub const TRAINING_OPTIMAL_NV: Self = Self(3);
}
impl fmt::Display for CooperativeVectorMatrixLayoutNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ROW_MAJOR_NV => write!(f, "ROW_MAJOR_NV"),
            Self::COLUMN_MAJOR_NV => write!(f, "COLUMN_MAJOR_NV"),
            Self::INFERENCING_OPTIMAL_NV => write!(f, "INFERENCING_OPTIMAL_NV"),
            Self::TRAINING_OPTIMAL_NV => write!(f, "TRAINING_OPTIMAL_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorTilingARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TensorTilingARM(i32);
impl TensorTilingARM {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl TensorTilingARM {
    pub const OPTIMAL_ARM: Self = Self(0);
    pub const LINEAR_ARM: Self = Self(1);
}
impl fmt::Display for TensorTilingARM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::OPTIMAL_ARM => write!(f, "OPTIMAL_ARM"),
            Self::LINEAR_ARM => write!(f, "LINEAR_ARM"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelinePropertyARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DataGraphPipelinePropertyARM(i32);
impl DataGraphPipelinePropertyARM {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DataGraphPipelinePropertyARM {
    pub const CREATION_LOG_ARM: Self = Self(0);
    pub const IDENTIFIER_ARM: Self = Self(1);
}
impl fmt::Display for DataGraphPipelinePropertyARM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::CREATION_LOG_ARM => write!(f, "CREATION_LOG_ARM"),
            Self::IDENTIFIER_ARM => write!(f, "IDENTIFIER_ARM"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionBindPointARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DataGraphPipelineSessionBindPointARM(i32);
impl DataGraphPipelineSessionBindPointARM {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DataGraphPipelineSessionBindPointARM {
    pub const TRANSIENT_ARM: Self = Self(0);
    pub const OPTICAL_FLOW_CACHE_ARM: Self = Self(1000631001);
}
impl fmt::Display for DataGraphPipelineSessionBindPointARM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TRANSIENT_ARM => write!(f, "TRANSIENT_ARM"),
            Self::OPTICAL_FLOW_CACHE_ARM => write!(f, "OPTICAL_FLOW_CACHE_ARM"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionBindPointTypeARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DataGraphPipelineSessionBindPointTypeARM(i32);
impl DataGraphPipelineSessionBindPointTypeARM {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DataGraphPipelineSessionBindPointTypeARM {
    pub const MEMORY_ARM: Self = Self(0);
}
impl fmt::Display for DataGraphPipelineSessionBindPointTypeARM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::MEMORY_ARM {
            write!(f, "MEMORY_ARM")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphProcessingEngineTypeARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PhysicalDeviceDataGraphProcessingEngineTypeARM(i32);
impl PhysicalDeviceDataGraphProcessingEngineTypeARM {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PhysicalDeviceDataGraphProcessingEngineTypeARM {
    pub const DEFAULT_ARM: Self = Self(0);
    pub const NEURAL_QCOM: Self = Self(1000629000);
    pub const COMPUTE_QCOM: Self = Self(1000629001);
}
impl fmt::Display for PhysicalDeviceDataGraphProcessingEngineTypeARM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DEFAULT_ARM => write!(f, "DEFAULT_ARM"),
            Self::NEURAL_QCOM => write!(f, "NEURAL_QCOM"),
            Self::COMPUTE_QCOM => write!(f, "COMPUTE_QCOM"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphOperationTypeARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PhysicalDeviceDataGraphOperationTypeARM(i32);
impl PhysicalDeviceDataGraphOperationTypeARM {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PhysicalDeviceDataGraphOperationTypeARM {
    pub const SPIRV_EXTENDED_INSTRUCTION_SET_ARM: Self = Self(0);
    pub const NEURAL_MODEL_QCOM: Self = Self(1000629000);
    pub const BUILTIN_MODEL_QCOM: Self = Self(1000629001);
    pub const OPTICAL_FLOW_ARM: Self = Self(1000631000);
}
impl fmt::Display for PhysicalDeviceDataGraphOperationTypeARM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::SPIRV_EXTENDED_INSTRUCTION_SET_ARM => {
                write!(f, "SPIRV_EXTENDED_INSTRUCTION_SET_ARM")
            }
            Self::NEURAL_MODEL_QCOM => write!(f, "NEURAL_MODEL_QCOM"),
            Self::BUILTIN_MODEL_QCOM => write!(f, "BUILTIN_MODEL_QCOM"),
            Self::OPTICAL_FLOW_ARM => write!(f, "OPTICAL_FLOW_ARM"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphModelCacheTypeQCOM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DataGraphModelCacheTypeQCOM(i32);
impl DataGraphModelCacheTypeQCOM {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DataGraphModelCacheTypeQCOM {
    pub const GENERIC_BINARY_QCOM: Self = Self(0);
}
impl fmt::Display for DataGraphModelCacheTypeQCOM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::GENERIC_BINARY_QCOM {
            write!(f, "GENERIC_BINARY_QCOM")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPerfHintTypeQCOM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PerfHintTypeQCOM(i32);
impl PerfHintTypeQCOM {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PerfHintTypeQCOM {
    pub const DEFAULT_QCOM: Self = Self(0);
    pub const FREQUENCY_MIN_QCOM: Self = Self(1);
    pub const FREQUENCY_MAX_QCOM: Self = Self(2);
    pub const FREQUENCY_SCALED_QCOM: Self = Self(3);
}
impl fmt::Display for PerfHintTypeQCOM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DEFAULT_QCOM => write!(f, "DEFAULT_QCOM"),
            Self::FREQUENCY_MIN_QCOM => write!(f, "FREQUENCY_MIN_QCOM"),
            Self::FREQUENCY_MAX_QCOM => write!(f, "FREQUENCY_MAX_QCOM"),
            Self::FREQUENCY_SCALED_QCOM => write!(f, "FREQUENCY_SCALED_QCOM"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorMappingSourceEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DescriptorMappingSourceEXT(i32);
impl DescriptorMappingSourceEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DescriptorMappingSourceEXT {
    pub const HEAP_WITH_CONSTANT_OFFSET_EXT: Self = Self(0);
    pub const HEAP_WITH_PUSH_INDEX_EXT: Self = Self(1);
    pub const HEAP_WITH_INDIRECT_INDEX_EXT: Self = Self(2);
    pub const HEAP_WITH_INDIRECT_INDEX_ARRAY_EXT: Self = Self(3);
    pub const RESOURCE_HEAP_DATA_EXT: Self = Self(4);
    pub const PUSH_DATA_EXT: Self = Self(5);
    pub const PUSH_ADDRESS_EXT: Self = Self(6);
    pub const INDIRECT_ADDRESS_EXT: Self = Self(7);
    pub const HEAP_WITH_SHADER_RECORD_INDEX_EXT: Self = Self(8);
    pub const SHADER_RECORD_DATA_EXT: Self = Self(9);
    pub const SHADER_RECORD_ADDRESS_EXT: Self = Self(10);
}
impl fmt::Display for DescriptorMappingSourceEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::HEAP_WITH_CONSTANT_OFFSET_EXT => write!(f, "HEAP_WITH_CONSTANT_OFFSET_EXT"),
            Self::HEAP_WITH_PUSH_INDEX_EXT => write!(f, "HEAP_WITH_PUSH_INDEX_EXT"),
            Self::HEAP_WITH_INDIRECT_INDEX_EXT => write!(f, "HEAP_WITH_INDIRECT_INDEX_EXT"),
            Self::HEAP_WITH_INDIRECT_INDEX_ARRAY_EXT => {
                write!(f, "HEAP_WITH_INDIRECT_INDEX_ARRAY_EXT")
            }
            Self::RESOURCE_HEAP_DATA_EXT => write!(f, "RESOURCE_HEAP_DATA_EXT"),
            Self::PUSH_DATA_EXT => write!(f, "PUSH_DATA_EXT"),
            Self::PUSH_ADDRESS_EXT => write!(f, "PUSH_ADDRESS_EXT"),
            Self::INDIRECT_ADDRESS_EXT => write!(f, "INDIRECT_ADDRESS_EXT"),
            Self::HEAP_WITH_SHADER_RECORD_INDEX_EXT => {
                write!(f, "HEAP_WITH_SHADER_RECORD_INDEX_EXT")
            }
            Self::SHADER_RECORD_DATA_EXT => write!(f, "SHADER_RECORD_DATA_EXT"),
            Self::SHADER_RECORD_ADDRESS_EXT => write!(f, "SHADER_RECORD_ADDRESS_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphTOSALevelARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DataGraphTOSALevelARM(i32);
impl DataGraphTOSALevelARM {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DataGraphTOSALevelARM {
    pub const OSA_LEVEL_NONE_ARM: Self = Self(0);
    pub const OSA_LEVEL_8K_ARM: Self = Self(1);
}
impl fmt::Display for DataGraphTOSALevelARM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::OSA_LEVEL_NONE_ARM => write!(f, "OSA_LEVEL_NONE_ARM"),
            Self::OSA_LEVEL_8K_ARM => write!(f, "OSA_LEVEL_8K_ARM"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowPerformanceLevelARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DataGraphOpticalFlowPerformanceLevelARM(i32);
impl DataGraphOpticalFlowPerformanceLevelARM {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DataGraphOpticalFlowPerformanceLevelARM {
    pub const UNKNOWN_ARM: Self = Self(0);
    pub const SLOW_ARM: Self = Self(1);
    pub const MEDIUM_ARM: Self = Self(2);
    pub const FAST_ARM: Self = Self(3);
}
impl fmt::Display for DataGraphOpticalFlowPerformanceLevelARM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UNKNOWN_ARM => write!(f, "UNKNOWN_ARM"),
            Self::SLOW_ARM => write!(f, "SLOW_ARM"),
            Self::MEDIUM_ARM => write!(f, "MEDIUM_ARM"),
            Self::FAST_ARM => write!(f, "FAST_ARM"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineNodeConnectionTypeARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DataGraphPipelineNodeConnectionTypeARM(i32);
impl DataGraphPipelineNodeConnectionTypeARM {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DataGraphPipelineNodeConnectionTypeARM {
    pub const OPTICAL_FLOW_INPUT_ARM: Self = Self(1000631000);
    pub const OPTICAL_FLOW_REFERENCE_ARM: Self = Self(1000631001);
    pub const OPTICAL_FLOW_HINT_ARM: Self = Self(1000631002);
    pub const OPTICAL_FLOW_FLOW_VECTOR_ARM: Self = Self(1000631003);
    pub const OPTICAL_FLOW_COST_ARM: Self = Self(1000631004);
}
impl fmt::Display for DataGraphPipelineNodeConnectionTypeARM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::OPTICAL_FLOW_INPUT_ARM => write!(f, "OPTICAL_FLOW_INPUT_ARM"),
            Self::OPTICAL_FLOW_REFERENCE_ARM => write!(f, "OPTICAL_FLOW_REFERENCE_ARM"),
            Self::OPTICAL_FLOW_HINT_ARM => write!(f, "OPTICAL_FLOW_HINT_ARM"),
            Self::OPTICAL_FLOW_FLOW_VECTOR_ARM => write!(f, "OPTICAL_FLOW_FLOW_VECTOR_ARM"),
            Self::OPTICAL_FLOW_COST_ARM => write!(f, "OPTICAL_FLOW_COST_ARM"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineNodeTypeARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DataGraphPipelineNodeTypeARM(i32);
impl DataGraphPipelineNodeTypeARM {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DataGraphPipelineNodeTypeARM {
    pub const OPTICAL_FLOW_ARM: Self = Self(1000631000);
}
impl fmt::Display for DataGraphPipelineNodeTypeARM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::OPTICAL_FLOW_ARM {
            write!(f, "OPTICAL_FLOW_ARM")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkColorSpaceKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ColorSpaceKHR(i32);
impl ColorSpaceKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ColorSpaceKHR {
    pub const SRGB_NONLINEAR_KHR: Self = Self(0);
    #[deprecated = "aliased"]
    pub const COLORSPACE_SRGB_NONLINEAR_KHR: Self = Self::SRGB_NONLINEAR_KHR;
    pub const DISPLAY_P3_NONLINEAR_EXT: Self = Self(1000104001);
    pub const EXTENDED_SRGB_LINEAR_EXT: Self = Self(1000104002);
    pub const DISPLAY_P3_LINEAR_EXT: Self = Self(1000104003);
    pub const DCI_P3_NONLINEAR_EXT: Self = Self(1000104004);
    pub const BT709_LINEAR_EXT: Self = Self(1000104005);
    pub const BT709_NONLINEAR_EXT: Self = Self(1000104006);
    pub const BT2020_LINEAR_EXT: Self = Self(1000104007);
    pub const HDR10_ST2084_EXT: Self = Self(1000104008);
    pub const DOLBYVISION_EXT: Self = Self(1000104009);
    pub const HDR10_HLG_EXT: Self = Self(1000104010);
    pub const ADOBERGB_LINEAR_EXT: Self = Self(1000104011);
    pub const ADOBERGB_NONLINEAR_EXT: Self = Self(1000104012);
    pub const PASS_THROUGH_EXT: Self = Self(1000104013);
    pub const EXTENDED_SRGB_NONLINEAR_EXT: Self = Self(1000104014);
    #[deprecated = "aliased"]
    pub const DCI_P3_LINEAR_EXT: Self = Self::DISPLAY_P3_LINEAR_EXT;
    pub const DISPLAY_NATIVE_AMD: Self = Self(1000213000);
}
impl fmt::Display for ColorSpaceKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::SRGB_NONLINEAR_KHR => write!(f, "SRGB_NONLINEAR_KHR"),
            Self::DISPLAY_P3_NONLINEAR_EXT => write!(f, "DISPLAY_P3_NONLINEAR_EXT"),
            Self::EXTENDED_SRGB_LINEAR_EXT => write!(f, "EXTENDED_SRGB_LINEAR_EXT"),
            Self::DISPLAY_P3_LINEAR_EXT => write!(f, "DISPLAY_P3_LINEAR_EXT"),
            Self::DCI_P3_NONLINEAR_EXT => write!(f, "DCI_P3_NONLINEAR_EXT"),
            Self::BT709_LINEAR_EXT => write!(f, "BT709_LINEAR_EXT"),
            Self::BT709_NONLINEAR_EXT => write!(f, "BT709_NONLINEAR_EXT"),
            Self::BT2020_LINEAR_EXT => write!(f, "BT2020_LINEAR_EXT"),
            Self::HDR10_ST2084_EXT => write!(f, "HDR10_ST2084_EXT"),
            Self::DOLBYVISION_EXT => write!(f, "DOLBYVISION_EXT"),
            Self::HDR10_HLG_EXT => write!(f, "HDR10_HLG_EXT"),
            Self::ADOBERGB_LINEAR_EXT => write!(f, "ADOBERGB_LINEAR_EXT"),
            Self::ADOBERGB_NONLINEAR_EXT => write!(f, "ADOBERGB_NONLINEAR_EXT"),
            Self::PASS_THROUGH_EXT => write!(f, "PASS_THROUGH_EXT"),
            Self::EXTENDED_SRGB_NONLINEAR_EXT => write!(f, "EXTENDED_SRGB_NONLINEAR_EXT"),
            Self::DISPLAY_NATIVE_AMD => write!(f, "DISPLAY_NATIVE_AMD"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentModeKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PresentModeKHR(i32);
impl PresentModeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PresentModeKHR {
    pub const IMMEDIATE_KHR: Self = Self(0);
    pub const MAILBOX_KHR: Self = Self(1);
    pub const FIFO_KHR: Self = Self(2);
    pub const FIFO_RELAXED_KHR: Self = Self(3);
    pub const SHARED_DEMAND_REFRESH_KHR: Self = Self(1000111000);
    pub const SHARED_CONTINUOUS_REFRESH_KHR: Self = Self(1000111001);
    pub const FIFO_LATEST_READY_EXT: Self = Self::FIFO_LATEST_READY_KHR;
    pub const FIFO_LATEST_READY_KHR: Self = Self(1000361000);
}
impl fmt::Display for PresentModeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::IMMEDIATE_KHR => write!(f, "IMMEDIATE_KHR"),
            Self::MAILBOX_KHR => write!(f, "MAILBOX_KHR"),
            Self::FIFO_KHR => write!(f, "FIFO_KHR"),
            Self::FIFO_RELAXED_KHR => write!(f, "FIFO_RELAXED_KHR"),
            Self::SHARED_DEMAND_REFRESH_KHR => write!(f, "SHARED_DEMAND_REFRESH_KHR"),
            Self::SHARED_CONTINUOUS_REFRESH_KHR => write!(f, "SHARED_CONTINUOUS_REFRESH_KHR"),
            Self::FIFO_LATEST_READY_KHR => write!(f, "FIFO_LATEST_READY_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplaySurfaceStereoTypeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DisplaySurfaceStereoTypeNV(i32);
impl DisplaySurfaceStereoTypeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DisplaySurfaceStereoTypeNV {
    pub const NONE_NV: Self = Self(0);
    pub const ONBOARD_DIN_NV: Self = Self(1);
    pub const HDMI_3D_NV: Self = Self(2);
    pub const INBAND_DISPLAYPORT_NV: Self = Self(3);
}
impl fmt::Display for DisplaySurfaceStereoTypeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NONE_NV => write!(f, "NONE_NV"),
            Self::ONBOARD_DIN_NV => write!(f, "ONBOARD_DIN_NV"),
            Self::HDMI_3D_NV => write!(f, "HDMI_3D_NV"),
            Self::INBAND_DISPLAYPORT_NV => write!(f, "INBAND_DISPLAYPORT_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportObjectTypeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DebugReportObjectTypeEXT(i32);
impl DebugReportObjectTypeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DebugReportObjectTypeEXT {
    pub const UNKNOWN_EXT: Self = Self(0);
    pub const INSTANCE_EXT: Self = Self(1);
    pub const PHYSICAL_DEVICE_EXT: Self = Self(2);
    pub const DEVICE_EXT: Self = Self(3);
    pub const QUEUE_EXT: Self = Self(4);
    pub const SEMAPHORE_EXT: Self = Self(5);
    pub const COMMAND_BUFFER_EXT: Self = Self(6);
    pub const FENCE_EXT: Self = Self(7);
    pub const DEVICE_MEMORY_EXT: Self = Self(8);
    pub const BUFFER_EXT: Self = Self(9);
    pub const IMAGE_EXT: Self = Self(10);
    pub const EVENT_EXT: Self = Self(11);
    pub const QUERY_POOL_EXT: Self = Self(12);
    pub const BUFFER_VIEW_EXT: Self = Self(13);
    pub const IMAGE_VIEW_EXT: Self = Self(14);
    pub const SHADER_MODULE_EXT: Self = Self(15);
    pub const PIPELINE_CACHE_EXT: Self = Self(16);
    pub const PIPELINE_LAYOUT_EXT: Self = Self(17);
    pub const RENDER_PASS_EXT: Self = Self(18);
    pub const PIPELINE_EXT: Self = Self(19);
    pub const DESCRIPTOR_SET_LAYOUT_EXT: Self = Self(20);
    pub const SAMPLER_EXT: Self = Self(21);
    pub const DESCRIPTOR_POOL_EXT: Self = Self(22);
    pub const DESCRIPTOR_SET_EXT: Self = Self(23);
    pub const FRAMEBUFFER_EXT: Self = Self(24);
    pub const COMMAND_POOL_EXT: Self = Self(25);
    pub const SURFACE_KHR_EXT: Self = Self(26);
    pub const SWAPCHAIN_KHR_EXT: Self = Self(27);
    pub const DEBUG_REPORT_CALLBACK_EXT_EXT: Self = Self(28);
    #[deprecated = "aliased"]
    pub const DEBUG_REPORT_EXT: Self = Self::DEBUG_REPORT_CALLBACK_EXT_EXT;
    pub const DISPLAY_KHR_EXT: Self = Self(29);
    pub const DISPLAY_MODE_KHR_EXT: Self = Self(30);
    pub const VALIDATION_CACHE_EXT_EXT: Self = Self(33);
    #[deprecated = "aliased"]
    pub const VALIDATION_CACHE_EXT: Self = Self::VALIDATION_CACHE_EXT_EXT;
    pub const SAMPLER_YCBCR_CONVERSION_EXT: Self = Self(1000156000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_EXT: Self = Self(1000085000);
    pub const CU_MODULE_NVX_EXT: Self = Self(1000029000);
    pub const CU_FUNCTION_NVX_EXT: Self = Self(1000029001);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE_EXT;
    pub const ACCELERATION_STRUCTURE_KHR_EXT: Self = Self(1000150000);
    pub const SAMPLER_YCBCR_CONVERSION_KHR_EXT: Self = Self::SAMPLER_YCBCR_CONVERSION_EXT;
    pub const ACCELERATION_STRUCTURE_NV_EXT: Self = Self(1000165000);
    pub const CUDA_MODULE_NV_EXT: Self = Self(1000307000);
    pub const CUDA_FUNCTION_NV_EXT: Self = Self(1000307001);
    pub const BUFFER_COLLECTION_FUCHSIA_EXT: Self = Self(1000366000);
}
impl fmt::Display for DebugReportObjectTypeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UNKNOWN_EXT => write!(f, "UNKNOWN_EXT"),
            Self::INSTANCE_EXT => write!(f, "INSTANCE_EXT"),
            Self::PHYSICAL_DEVICE_EXT => write!(f, "PHYSICAL_DEVICE_EXT"),
            Self::DEVICE_EXT => write!(f, "DEVICE_EXT"),
            Self::QUEUE_EXT => write!(f, "QUEUE_EXT"),
            Self::SEMAPHORE_EXT => write!(f, "SEMAPHORE_EXT"),
            Self::COMMAND_BUFFER_EXT => write!(f, "COMMAND_BUFFER_EXT"),
            Self::FENCE_EXT => write!(f, "FENCE_EXT"),
            Self::DEVICE_MEMORY_EXT => write!(f, "DEVICE_MEMORY_EXT"),
            Self::BUFFER_EXT => write!(f, "BUFFER_EXT"),
            Self::IMAGE_EXT => write!(f, "IMAGE_EXT"),
            Self::EVENT_EXT => write!(f, "EVENT_EXT"),
            Self::QUERY_POOL_EXT => write!(f, "QUERY_POOL_EXT"),
            Self::BUFFER_VIEW_EXT => write!(f, "BUFFER_VIEW_EXT"),
            Self::IMAGE_VIEW_EXT => write!(f, "IMAGE_VIEW_EXT"),
            Self::SHADER_MODULE_EXT => write!(f, "SHADER_MODULE_EXT"),
            Self::PIPELINE_CACHE_EXT => write!(f, "PIPELINE_CACHE_EXT"),
            Self::PIPELINE_LAYOUT_EXT => write!(f, "PIPELINE_LAYOUT_EXT"),
            Self::RENDER_PASS_EXT => write!(f, "RENDER_PASS_EXT"),
            Self::PIPELINE_EXT => write!(f, "PIPELINE_EXT"),
            Self::DESCRIPTOR_SET_LAYOUT_EXT => write!(f, "DESCRIPTOR_SET_LAYOUT_EXT"),
            Self::SAMPLER_EXT => write!(f, "SAMPLER_EXT"),
            Self::DESCRIPTOR_POOL_EXT => write!(f, "DESCRIPTOR_POOL_EXT"),
            Self::DESCRIPTOR_SET_EXT => write!(f, "DESCRIPTOR_SET_EXT"),
            Self::FRAMEBUFFER_EXT => write!(f, "FRAMEBUFFER_EXT"),
            Self::COMMAND_POOL_EXT => write!(f, "COMMAND_POOL_EXT"),
            Self::SURFACE_KHR_EXT => write!(f, "SURFACE_KHR_EXT"),
            Self::SWAPCHAIN_KHR_EXT => write!(f, "SWAPCHAIN_KHR_EXT"),
            Self::DEBUG_REPORT_CALLBACK_EXT_EXT => write!(f, "DEBUG_REPORT_CALLBACK_EXT_EXT"),
            Self::DISPLAY_KHR_EXT => write!(f, "DISPLAY_KHR_EXT"),
            Self::DISPLAY_MODE_KHR_EXT => write!(f, "DISPLAY_MODE_KHR_EXT"),
            Self::VALIDATION_CACHE_EXT_EXT => write!(f, "VALIDATION_CACHE_EXT_EXT"),
            Self::SAMPLER_YCBCR_CONVERSION_EXT => write!(f, "SAMPLER_YCBCR_CONVERSION_EXT"),
            Self::DESCRIPTOR_UPDATE_TEMPLATE_EXT => write!(f, "DESCRIPTOR_UPDATE_TEMPLATE_EXT"),
            Self::CU_MODULE_NVX_EXT => write!(f, "CU_MODULE_NVX_EXT"),
            Self::CU_FUNCTION_NVX_EXT => write!(f, "CU_FUNCTION_NVX_EXT"),
            Self::ACCELERATION_STRUCTURE_KHR_EXT => write!(f, "ACCELERATION_STRUCTURE_KHR_EXT"),
            Self::ACCELERATION_STRUCTURE_NV_EXT => write!(f, "ACCELERATION_STRUCTURE_NV_EXT"),
            Self::CUDA_MODULE_NV_EXT => write!(f, "CUDA_MODULE_NV_EXT"),
            Self::CUDA_FUNCTION_NV_EXT => write!(f, "CUDA_FUNCTION_NV_EXT"),
            Self::BUFFER_COLLECTION_FUCHSIA_EXT => write!(f, "BUFFER_COLLECTION_FUCHSIA_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemoryReportEventTypeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DeviceMemoryReportEventTypeEXT(i32);
impl DeviceMemoryReportEventTypeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DeviceMemoryReportEventTypeEXT {
    pub const ALLOCATE_EXT: Self = Self(0);
    pub const FREE_EXT: Self = Self(1);
    pub const IMPORT_EXT: Self = Self(2);
    pub const UNIMPORT_EXT: Self = Self(3);
    pub const ALLOCATION_FAILED_EXT: Self = Self(4);
}
impl fmt::Display for DeviceMemoryReportEventTypeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ALLOCATE_EXT => write!(f, "ALLOCATE_EXT"),
            Self::FREE_EXT => write!(f, "FREE_EXT"),
            Self::IMPORT_EXT => write!(f, "IMPORT_EXT"),
            Self::UNIMPORT_EXT => write!(f, "UNIMPORT_EXT"),
            Self::ALLOCATION_FAILED_EXT => write!(f, "ALLOCATION_FAILED_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRasterizationOrderAMD.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct RasterizationOrderAMD(i32);
impl RasterizationOrderAMD {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl RasterizationOrderAMD {
    pub const STRICT_AMD: Self = Self(0);
    pub const RELAXED_AMD: Self = Self(1);
}
impl fmt::Display for RasterizationOrderAMD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::STRICT_AMD => write!(f, "STRICT_AMD"),
            Self::RELAXED_AMD => write!(f, "RELAXED_AMD"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCheckEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ValidationCheckEXT(i32);
impl ValidationCheckEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ValidationCheckEXT {
    pub const ALL_EXT: Self = Self(0);
    pub const SHADERS_EXT: Self = Self(1);
}
impl fmt::Display for ValidationCheckEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ALL_EXT => write!(f, "ALL_EXT"),
            Self::SHADERS_EXT => write!(f, "SHADERS_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationFeatureEnableEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ValidationFeatureEnableEXT(i32);
impl ValidationFeatureEnableEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ValidationFeatureEnableEXT {
    pub const GPU_ASSISTED_EXT: Self = Self(0);
    pub const GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT: Self = Self(1);
    pub const BEST_PRACTICES_EXT: Self = Self(2);
    pub const DEBUG_PRINTF_EXT: Self = Self(3);
    pub const SYNCHRONIZATION_VALIDATION_EXT: Self = Self(4);
}
impl fmt::Display for ValidationFeatureEnableEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::GPU_ASSISTED_EXT => write!(f, "GPU_ASSISTED_EXT"),
            Self::GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT => {
                write!(f, "GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT")
            }
            Self::BEST_PRACTICES_EXT => write!(f, "BEST_PRACTICES_EXT"),
            Self::DEBUG_PRINTF_EXT => write!(f, "DEBUG_PRINTF_EXT"),
            Self::SYNCHRONIZATION_VALIDATION_EXT => write!(f, "SYNCHRONIZATION_VALIDATION_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationFeatureDisableEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ValidationFeatureDisableEXT(i32);
impl ValidationFeatureDisableEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ValidationFeatureDisableEXT {
    pub const ALL_EXT: Self = Self(0);
    pub const SHADERS_EXT: Self = Self(1);
    pub const THREAD_SAFETY_EXT: Self = Self(2);
    pub const API_PARAMETERS_EXT: Self = Self(3);
    pub const OBJECT_LIFETIMES_EXT: Self = Self(4);
    pub const CORE_CHECKS_EXT: Self = Self(5);
    pub const UNIQUE_HANDLES_EXT: Self = Self(6);
    pub const SHADER_VALIDATION_CACHE_EXT: Self = Self(7);
}
impl fmt::Display for ValidationFeatureDisableEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ALL_EXT => write!(f, "ALL_EXT"),
            Self::SHADERS_EXT => write!(f, "SHADERS_EXT"),
            Self::THREAD_SAFETY_EXT => write!(f, "THREAD_SAFETY_EXT"),
            Self::API_PARAMETERS_EXT => write!(f, "API_PARAMETERS_EXT"),
            Self::OBJECT_LIFETIMES_EXT => write!(f, "OBJECT_LIFETIMES_EXT"),
            Self::CORE_CHECKS_EXT => write!(f, "CORE_CHECKS_EXT"),
            Self::UNIQUE_HANDLES_EXT => write!(f, "UNIQUE_HANDLES_EXT"),
            Self::SHADER_VALIDATION_CACHE_EXT => write!(f, "SHADER_VALIDATION_CACHE_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPowerStateEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DisplayPowerStateEXT(i32);
impl DisplayPowerStateEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DisplayPowerStateEXT {
    pub const OFF_EXT: Self = Self(0);
    pub const SUSPEND_EXT: Self = Self(1);
    pub const ON_EXT: Self = Self(2);
}
impl fmt::Display for DisplayPowerStateEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::OFF_EXT => write!(f, "OFF_EXT"),
            Self::SUSPEND_EXT => write!(f, "SUSPEND_EXT"),
            Self::ON_EXT => write!(f, "ON_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceEventTypeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DeviceEventTypeEXT(i32);
impl DeviceEventTypeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DeviceEventTypeEXT {
    pub const DISPLAY_HOTPLUG_EXT: Self = Self(0);
}
impl fmt::Display for DeviceEventTypeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::DISPLAY_HOTPLUG_EXT {
            write!(f, "DISPLAY_HOTPLUG_EXT")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayEventTypeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DisplayEventTypeEXT(i32);
impl DisplayEventTypeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DisplayEventTypeEXT {
    pub const FIRST_PIXEL_OUT_EXT: Self = Self(0);
}
impl fmt::Display for DisplayEventTypeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::FIRST_PIXEL_OUT_EXT {
            write!(f, "FIRST_PIXEL_OUT_EXT")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTessellationDomainOrigin.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TessellationDomainOrigin(i32);
impl TessellationDomainOrigin {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl TessellationDomainOrigin {
    pub const UPPER_LEFT: Self = Self(0);
    pub const LOWER_LEFT: Self = Self(1);
    pub const UPPER_LEFT_KHR: Self = Self::UPPER_LEFT;
    pub const LOWER_LEFT_KHR: Self = Self::LOWER_LEFT;
}
impl fmt::Display for TessellationDomainOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UPPER_LEFT => write!(f, "UPPER_LEFT"),
            Self::LOWER_LEFT => write!(f, "LOWER_LEFT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrModelConversion.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SamplerYcbcrModelConversion(i32);
impl SamplerYcbcrModelConversion {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl SamplerYcbcrModelConversion {
    pub const RGB_IDENTITY: Self = Self(0);
    pub const YCBCR_IDENTITY: Self = Self(1);
    pub const YCBCR_709: Self = Self(2);
    pub const YCBCR_601: Self = Self(3);
    pub const YCBCR_2020: Self = Self(4);
    pub const RGB_IDENTITY_KHR: Self = Self::RGB_IDENTITY;
    pub const YCBCR_IDENTITY_KHR: Self = Self::YCBCR_IDENTITY;
    pub const YCBCR_709_KHR: Self = Self::YCBCR_709;
    pub const YCBCR_601_KHR: Self = Self::YCBCR_601;
    pub const YCBCR_2020_KHR: Self = Self::YCBCR_2020;
}
impl fmt::Display for SamplerYcbcrModelConversion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::RGB_IDENTITY => write!(f, "RGB_IDENTITY"),
            Self::YCBCR_IDENTITY => write!(f, "YCBCR_IDENTITY"),
            Self::YCBCR_709 => write!(f, "YCBCR_709"),
            Self::YCBCR_601 => write!(f, "YCBCR_601"),
            Self::YCBCR_2020 => write!(f, "YCBCR_2020"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrRange.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SamplerYcbcrRange(i32);
impl SamplerYcbcrRange {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl SamplerYcbcrRange {
    pub const ITU_FULL: Self = Self(0);
    pub const ITU_NARROW: Self = Self(1);
    pub const ITU_FULL_KHR: Self = Self::ITU_FULL;
    pub const ITU_NARROW_KHR: Self = Self::ITU_NARROW;
}
impl fmt::Display for SamplerYcbcrRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ITU_FULL => write!(f, "ITU_FULL"),
            Self::ITU_NARROW => write!(f, "ITU_NARROW"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkChromaLocation.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ChromaLocation(i32);
impl ChromaLocation {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ChromaLocation {
    pub const COSITED_EVEN: Self = Self(0);
    pub const MIDPOINT: Self = Self(1);
    pub const COSITED_EVEN_KHR: Self = Self::COSITED_EVEN;
    pub const MIDPOINT_KHR: Self = Self::MIDPOINT;
}
impl fmt::Display for ChromaLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::COSITED_EVEN => write!(f, "COSITED_EVEN"),
            Self::MIDPOINT => write!(f, "MIDPOINT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerReductionMode.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SamplerReductionMode(i32);
impl SamplerReductionMode {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl SamplerReductionMode {
    pub const WEIGHTED_AVERAGE: Self = Self(0);
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(2);
    pub const WEIGHTED_AVERAGE_EXT: Self = Self::WEIGHTED_AVERAGE;
    pub const MIN_EXT: Self = Self::MIN;
    pub const MAX_EXT: Self = Self::MAX;
    pub const WEIGHTED_AVERAGE_RANGECLAMP_QCOM: Self = Self(1000521000);
}
impl fmt::Display for SamplerReductionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::WEIGHTED_AVERAGE => write!(f, "WEIGHTED_AVERAGE"),
            Self::MIN => write!(f, "MIN"),
            Self::MAX => write!(f, "MAX"),
            Self::WEIGHTED_AVERAGE_RANGECLAMP_QCOM => write!(f, "WEIGHTED_AVERAGE_RANGECLAMP_QCOM"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBlendOverlapEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BlendOverlapEXT(i32);
impl BlendOverlapEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl BlendOverlapEXT {
    pub const UNCORRELATED_EXT: Self = Self(0);
    pub const DISJOINT_EXT: Self = Self(1);
    pub const CONJOINT_EXT: Self = Self(2);
}
impl fmt::Display for BlendOverlapEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UNCORRELATED_EXT => write!(f, "UNCORRELATED_EXT"),
            Self::DISJOINT_EXT => write!(f, "DISJOINT_EXT"),
            Self::CONJOINT_EXT => write!(f, "CONJOINT_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFullScreenExclusiveEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FullScreenExclusiveEXT(i32);
impl FullScreenExclusiveEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl FullScreenExclusiveEXT {
    pub const DEFAULT_EXT: Self = Self(0);
    pub const ALLOWED_EXT: Self = Self(1);
    pub const DISALLOWED_EXT: Self = Self(2);
    pub const APPLICATION_CONTROLLED_EXT: Self = Self(3);
}
impl fmt::Display for FullScreenExclusiveEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DEFAULT_EXT => write!(f, "DEFAULT_EXT"),
            Self::ALLOWED_EXT => write!(f, "ALLOWED_EXT"),
            Self::DISALLOWED_EXT => write!(f, "DISALLOWED_EXT"),
            Self::APPLICATION_CONTROLLED_EXT => write!(f, "APPLICATION_CONTROLLED_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderFloatControlsIndependence.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ShaderFloatControlsIndependence(i32);
impl ShaderFloatControlsIndependence {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ShaderFloatControlsIndependence {
    pub const TYPE_32_BIT_ONLY: Self = Self(0);
    pub const ALL: Self = Self(1);
    pub const NONE: Self = Self(2);
    pub const TYPE_32_BIT_ONLY_KHR: Self = Self::TYPE_32_BIT_ONLY;
    pub const ALL_KHR: Self = Self::ALL;
    pub const NONE_KHR: Self = Self::NONE;
}
impl fmt::Display for ShaderFloatControlsIndependence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TYPE_32_BIT_ONLY => write!(f, "TYPE_32_BIT_ONLY"),
            Self::ALL => write!(f, "ALL"),
            Self::NONE => write!(f, "NONE"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFragmentShadingRateCombinerOpKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FragmentShadingRateCombinerOpKHR(i32);
impl FragmentShadingRateCombinerOpKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl FragmentShadingRateCombinerOpKHR {
    pub const KEEP_KHR: Self = Self(0);
    pub const REPLACE_KHR: Self = Self(1);
    pub const MIN_KHR: Self = Self(2);
    pub const MAX_KHR: Self = Self(3);
    pub const MUL_KHR: Self = Self(4);
}
impl fmt::Display for FragmentShadingRateCombinerOpKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::KEEP_KHR => write!(f, "KEEP_KHR"),
            Self::REPLACE_KHR => write!(f, "REPLACE_KHR"),
            Self::MIN_KHR => write!(f, "MIN_KHR"),
            Self::MAX_KHR => write!(f, "MAX_KHR"),
            Self::MUL_KHR => write!(f, "MUL_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowPerformanceLevelNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct OpticalFlowPerformanceLevelNV(i32);
impl OpticalFlowPerformanceLevelNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl OpticalFlowPerformanceLevelNV {
    pub const UNKNOWN_NV: Self = Self(0);
    pub const SLOW_NV: Self = Self(1);
    pub const MEDIUM_NV: Self = Self(2);
    pub const FAST_NV: Self = Self(3);
}
impl fmt::Display for OpticalFlowPerformanceLevelNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UNKNOWN_NV => write!(f, "UNKNOWN_NV"),
            Self::SLOW_NV => write!(f, "SLOW_NV"),
            Self::MEDIUM_NV => write!(f, "MEDIUM_NV"),
            Self::FAST_NV => write!(f, "FAST_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionBindingPointNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct OpticalFlowSessionBindingPointNV(i32);
impl OpticalFlowSessionBindingPointNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl OpticalFlowSessionBindingPointNV {
    pub const UNKNOWN_NV: Self = Self(0);
    pub const INPUT_NV: Self = Self(1);
    pub const REFERENCE_NV: Self = Self(2);
    pub const HINT_NV: Self = Self(3);
    pub const FLOW_VECTOR_NV: Self = Self(4);
    pub const BACKWARD_FLOW_VECTOR_NV: Self = Self(5);
    pub const COST_NV: Self = Self(6);
    pub const BACKWARD_COST_NV: Self = Self(7);
    pub const GLOBAL_FLOW_NV: Self = Self(8);
}
impl fmt::Display for OpticalFlowSessionBindingPointNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UNKNOWN_NV => write!(f, "UNKNOWN_NV"),
            Self::INPUT_NV => write!(f, "INPUT_NV"),
            Self::REFERENCE_NV => write!(f, "REFERENCE_NV"),
            Self::HINT_NV => write!(f, "HINT_NV"),
            Self::FLOW_VECTOR_NV => write!(f, "FLOW_VECTOR_NV"),
            Self::BACKWARD_FLOW_VECTOR_NV => write!(f, "BACKWARD_FLOW_VECTOR_NV"),
            Self::COST_NV => write!(f, "COST_NV"),
            Self::BACKWARD_COST_NV => write!(f, "BACKWARD_COST_NV"),
            Self::GLOBAL_FLOW_NV => write!(f, "GLOBAL_FLOW_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultAddressTypeKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DeviceFaultAddressTypeKHR(i32);
impl DeviceFaultAddressTypeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DeviceFaultAddressTypeKHR {
    pub const NONE_KHR: Self = Self(0);
    pub const READ_INVALID_KHR: Self = Self(1);
    pub const WRITE_INVALID_KHR: Self = Self(2);
    pub const EXECUTE_INVALID_KHR: Self = Self(3);
    pub const INSTRUCTION_POINTER_UNKNOWN_KHR: Self = Self(4);
    pub const INSTRUCTION_POINTER_INVALID_KHR: Self = Self(5);
    pub const INSTRUCTION_POINTER_FAULT_KHR: Self = Self(6);
    pub const NONE_EXT: Self = Self::NONE_KHR;
    pub const READ_INVALID_EXT: Self = Self::READ_INVALID_KHR;
    pub const WRITE_INVALID_EXT: Self = Self::WRITE_INVALID_KHR;
    pub const EXECUTE_INVALID_EXT: Self = Self::EXECUTE_INVALID_KHR;
    pub const INSTRUCTION_POINTER_UNKNOWN_EXT: Self = Self::INSTRUCTION_POINTER_UNKNOWN_KHR;
    pub const INSTRUCTION_POINTER_INVALID_EXT: Self = Self::INSTRUCTION_POINTER_INVALID_KHR;
    pub const INSTRUCTION_POINTER_FAULT_EXT: Self = Self::INSTRUCTION_POINTER_FAULT_KHR;
}
impl fmt::Display for DeviceFaultAddressTypeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NONE_KHR => write!(f, "NONE_KHR"),
            Self::READ_INVALID_KHR => write!(f, "READ_INVALID_KHR"),
            Self::WRITE_INVALID_KHR => write!(f, "WRITE_INVALID_KHR"),
            Self::EXECUTE_INVALID_KHR => write!(f, "EXECUTE_INVALID_KHR"),
            Self::INSTRUCTION_POINTER_UNKNOWN_KHR => write!(f, "INSTRUCTION_POINTER_UNKNOWN_KHR"),
            Self::INSTRUCTION_POINTER_INVALID_KHR => write!(f, "INSTRUCTION_POINTER_INVALID_KHR"),
            Self::INSTRUCTION_POINTER_FAULT_KHR => write!(f, "INSTRUCTION_POINTER_FAULT_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkLayerSettingTypeEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LayerSettingTypeEXT(i32);
impl LayerSettingTypeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl LayerSettingTypeEXT {
    pub const BOOL32_EXT: Self = Self(0);
    pub const INT32_EXT: Self = Self(1);
    pub const INT64_EXT: Self = Self(2);
    pub const UINT32_EXT: Self = Self(3);
    pub const UINT64_EXT: Self = Self(4);
    pub const FLOAT32_EXT: Self = Self(5);
    pub const FLOAT64_EXT: Self = Self(6);
    pub const STRING_EXT: Self = Self(7);
}
impl fmt::Display for LayerSettingTypeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::BOOL32_EXT => write!(f, "BOOL32_EXT"),
            Self::INT32_EXT => write!(f, "INT32_EXT"),
            Self::INT64_EXT => write!(f, "INT64_EXT"),
            Self::UINT32_EXT => write!(f, "UINT32_EXT"),
            Self::UINT64_EXT => write!(f, "UINT64_EXT"),
            Self::FLOAT32_EXT => write!(f, "FLOAT32_EXT"),
            Self::FLOAT64_EXT => write!(f, "FLOAT64_EXT"),
            Self::STRING_EXT => write!(f, "STRING_EXT"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkLatencyMarkerNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LatencyMarkerNV(i32);
impl LatencyMarkerNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl LatencyMarkerNV {
    pub const SIMULATION_START_NV: Self = Self(0);
    pub const SIMULATION_END_NV: Self = Self(1);
    pub const RENDERSUBMIT_START_NV: Self = Self(2);
    pub const RENDERSUBMIT_END_NV: Self = Self(3);
    pub const PRESENT_START_NV: Self = Self(4);
    pub const PRESENT_END_NV: Self = Self(5);
    pub const INPUT_SAMPLE_NV: Self = Self(6);
    pub const TRIGGER_FLASH_NV: Self = Self(7);
    pub const OUT_OF_BAND_RENDERSUBMIT_START_NV: Self = Self(8);
    pub const OUT_OF_BAND_RENDERSUBMIT_END_NV: Self = Self(9);
    pub const OUT_OF_BAND_PRESENT_START_NV: Self = Self(10);
    pub const OUT_OF_BAND_PRESENT_END_NV: Self = Self(11);
}
impl fmt::Display for LatencyMarkerNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::SIMULATION_START_NV => write!(f, "SIMULATION_START_NV"),
            Self::SIMULATION_END_NV => write!(f, "SIMULATION_END_NV"),
            Self::RENDERSUBMIT_START_NV => write!(f, "RENDERSUBMIT_START_NV"),
            Self::RENDERSUBMIT_END_NV => write!(f, "RENDERSUBMIT_END_NV"),
            Self::PRESENT_START_NV => write!(f, "PRESENT_START_NV"),
            Self::PRESENT_END_NV => write!(f, "PRESENT_END_NV"),
            Self::INPUT_SAMPLE_NV => write!(f, "INPUT_SAMPLE_NV"),
            Self::TRIGGER_FLASH_NV => write!(f, "TRIGGER_FLASH_NV"),
            Self::OUT_OF_BAND_RENDERSUBMIT_START_NV => {
                write!(f, "OUT_OF_BAND_RENDERSUBMIT_START_NV")
            }
            Self::OUT_OF_BAND_RENDERSUBMIT_END_NV => write!(f, "OUT_OF_BAND_RENDERSUBMIT_END_NV"),
            Self::OUT_OF_BAND_PRESENT_START_NV => write!(f, "OUT_OF_BAND_PRESENT_START_NV"),
            Self::OUT_OF_BAND_PRESENT_END_NV => write!(f, "OUT_OF_BAND_PRESENT_END_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkOutOfBandQueueTypeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct OutOfBandQueueTypeNV(i32);
impl OutOfBandQueueTypeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl OutOfBandQueueTypeNV {
    pub const RENDER_NV: Self = Self(0);
    pub const PRESENT_NV: Self = Self(1);
}
impl fmt::Display for OutOfBandQueueTypeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::RENDER_NV => write!(f, "RENDER_NV"),
            Self::PRESENT_NV => write!(f, "PRESENT_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVendorId.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct VendorId(i32);
impl VendorId {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl VendorId {
    pub const KHRONOS: Self = Self(0x10000);
    pub const VIV: Self = Self(0x10001);
    pub const VSI: Self = Self(0x10002);
    pub const KAZAN: Self = Self(0x10003);
    pub const CODEPLAY: Self = Self(0x10004);
    pub const MESA: Self = Self(0x10005);
    pub const POCL: Self = Self(0x10006);
    pub const MOBILEYE: Self = Self(0x10007);
}
impl fmt::Display for VendorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::KHRONOS => write!(f, "KHRONOS"),
            Self::VIV => write!(f, "VIV"),
            Self::VSI => write!(f, "VSI"),
            Self::KAZAN => write!(f, "KAZAN"),
            Self::CODEPLAY => write!(f, "CODEPLAY"),
            Self::MESA => write!(f, "MESA"),
            Self::POCL => write!(f, "POCL"),
            Self::MOBILEYE => write!(f, "MOBILEYE"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDriverId.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DriverId(i32);
impl DriverId {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DriverId {
    pub const AMD_PROPRIETARY: Self = Self(1);
    pub const AMD_OPEN_SOURCE: Self = Self(2);
    pub const MESA_RADV: Self = Self(3);
    pub const NVIDIA_PROPRIETARY: Self = Self(4);
    pub const INTEL_PROPRIETARY_WINDOWS: Self = Self(5);
    pub const INTEL_OPEN_SOURCE_MESA: Self = Self(6);
    pub const IMAGINATION_PROPRIETARY: Self = Self(7);
    pub const QUALCOMM_PROPRIETARY: Self = Self(8);
    pub const ARM_PROPRIETARY: Self = Self(9);
    pub const GOOGLE_SWIFTSHADER: Self = Self(10);
    pub const GGP_PROPRIETARY: Self = Self(11);
    pub const BROADCOM_PROPRIETARY: Self = Self(12);
    pub const MESA_LLVMPIPE: Self = Self(13);
    pub const MOLTENVK: Self = Self(14);
    pub const COREAVI_PROPRIETARY: Self = Self(15);
    pub const JUICE_PROPRIETARY: Self = Self(16);
    pub const VERISILICON_PROPRIETARY: Self = Self(17);
    pub const MESA_TURNIP: Self = Self(18);
    pub const MESA_V3DV: Self = Self(19);
    pub const MESA_PANVK: Self = Self(20);
    pub const SAMSUNG_PROPRIETARY: Self = Self(21);
    pub const MESA_VENUS: Self = Self(22);
    pub const MESA_DOZEN: Self = Self(23);
    pub const MESA_NVK: Self = Self(24);
    pub const IMAGINATION_OPEN_SOURCE_MESA: Self = Self(25);
    pub const MESA_HONEYKRISP: Self = Self(26);
    pub const VULKAN_SC_EMULATION_ON_VULKAN: Self = Self(27);
    pub const MESA_KOSMICKRISP: Self = Self(28);
    pub const AMD_PROPRIETARY_KHR: Self = Self::AMD_PROPRIETARY;
    pub const AMD_OPEN_SOURCE_KHR: Self = Self::AMD_OPEN_SOURCE;
    pub const MESA_RADV_KHR: Self = Self::MESA_RADV;
    pub const NVIDIA_PROPRIETARY_KHR: Self = Self::NVIDIA_PROPRIETARY;
    pub const INTEL_PROPRIETARY_WINDOWS_KHR: Self = Self::INTEL_PROPRIETARY_WINDOWS;
    pub const INTEL_OPEN_SOURCE_MESA_KHR: Self = Self::INTEL_OPEN_SOURCE_MESA;
    pub const IMAGINATION_PROPRIETARY_KHR: Self = Self::IMAGINATION_PROPRIETARY;
    pub const QUALCOMM_PROPRIETARY_KHR: Self = Self::QUALCOMM_PROPRIETARY;
    pub const ARM_PROPRIETARY_KHR: Self = Self::ARM_PROPRIETARY;
    pub const GOOGLE_SWIFTSHADER_KHR: Self = Self::GOOGLE_SWIFTSHADER;
    pub const GGP_PROPRIETARY_KHR: Self = Self::GGP_PROPRIETARY;
    pub const BROADCOM_PROPRIETARY_KHR: Self = Self::BROADCOM_PROPRIETARY;
}
impl fmt::Display for DriverId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::AMD_PROPRIETARY => write!(f, "AMD_PROPRIETARY"),
            Self::AMD_OPEN_SOURCE => write!(f, "AMD_OPEN_SOURCE"),
            Self::MESA_RADV => write!(f, "MESA_RADV"),
            Self::NVIDIA_PROPRIETARY => write!(f, "NVIDIA_PROPRIETARY"),
            Self::INTEL_PROPRIETARY_WINDOWS => write!(f, "INTEL_PROPRIETARY_WINDOWS"),
            Self::INTEL_OPEN_SOURCE_MESA => write!(f, "INTEL_OPEN_SOURCE_MESA"),
            Self::IMAGINATION_PROPRIETARY => write!(f, "IMAGINATION_PROPRIETARY"),
            Self::QUALCOMM_PROPRIETARY => write!(f, "QUALCOMM_PROPRIETARY"),
            Self::ARM_PROPRIETARY => write!(f, "ARM_PROPRIETARY"),
            Self::GOOGLE_SWIFTSHADER => write!(f, "GOOGLE_SWIFTSHADER"),
            Self::GGP_PROPRIETARY => write!(f, "GGP_PROPRIETARY"),
            Self::BROADCOM_PROPRIETARY => write!(f, "BROADCOM_PROPRIETARY"),
            Self::MESA_LLVMPIPE => write!(f, "MESA_LLVMPIPE"),
            Self::MOLTENVK => write!(f, "MOLTENVK"),
            Self::COREAVI_PROPRIETARY => write!(f, "COREAVI_PROPRIETARY"),
            Self::JUICE_PROPRIETARY => write!(f, "JUICE_PROPRIETARY"),
            Self::VERISILICON_PROPRIETARY => write!(f, "VERISILICON_PROPRIETARY"),
            Self::MESA_TURNIP => write!(f, "MESA_TURNIP"),
            Self::MESA_V3DV => write!(f, "MESA_V3DV"),
            Self::MESA_PANVK => write!(f, "MESA_PANVK"),
            Self::SAMSUNG_PROPRIETARY => write!(f, "SAMSUNG_PROPRIETARY"),
            Self::MESA_VENUS => write!(f, "MESA_VENUS"),
            Self::MESA_DOZEN => write!(f, "MESA_DOZEN"),
            Self::MESA_NVK => write!(f, "MESA_NVK"),
            Self::IMAGINATION_OPEN_SOURCE_MESA => write!(f, "IMAGINATION_OPEN_SOURCE_MESA"),
            Self::MESA_HONEYKRISP => write!(f, "MESA_HONEYKRISP"),
            Self::VULKAN_SC_EMULATION_ON_VULKAN => write!(f, "VULKAN_SC_EMULATION_ON_VULKAN"),
            Self::MESA_KOSMICKRISP => write!(f, "MESA_KOSMICKRISP"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShadingRatePaletteEntryNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ShadingRatePaletteEntryNV(i32);
impl ShadingRatePaletteEntryNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl ShadingRatePaletteEntryNV {
    pub const NO_INVOCATIONS_NV: Self = Self(0);
    pub const TYPE_16_INVOCATIONS_PER_PIXEL_NV: Self = Self(1);
    pub const TYPE_8_INVOCATIONS_PER_PIXEL_NV: Self = Self(2);
    pub const TYPE_4_INVOCATIONS_PER_PIXEL_NV: Self = Self(3);
    pub const TYPE_2_INVOCATIONS_PER_PIXEL_NV: Self = Self(4);
    pub const TYPE_1_INVOCATION_PER_PIXEL_NV: Self = Self(5);
    pub const TYPE_1_INVOCATION_PER_2X1_PIXELS_NV: Self = Self(6);
    pub const TYPE_1_INVOCATION_PER_1X2_PIXELS_NV: Self = Self(7);
    pub const TYPE_1_INVOCATION_PER_2X2_PIXELS_NV: Self = Self(8);
    pub const TYPE_1_INVOCATION_PER_4X2_PIXELS_NV: Self = Self(9);
    pub const TYPE_1_INVOCATION_PER_2X4_PIXELS_NV: Self = Self(10);
    pub const TYPE_1_INVOCATION_PER_4X4_PIXELS_NV: Self = Self(11);
}
impl fmt::Display for ShadingRatePaletteEntryNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NO_INVOCATIONS_NV => write!(f, "NO_INVOCATIONS_NV"),
            Self::TYPE_16_INVOCATIONS_PER_PIXEL_NV => write!(f, "TYPE_16_INVOCATIONS_PER_PIXEL_NV"),
            Self::TYPE_8_INVOCATIONS_PER_PIXEL_NV => write!(f, "TYPE_8_INVOCATIONS_PER_PIXEL_NV"),
            Self::TYPE_4_INVOCATIONS_PER_PIXEL_NV => write!(f, "TYPE_4_INVOCATIONS_PER_PIXEL_NV"),
            Self::TYPE_2_INVOCATIONS_PER_PIXEL_NV => write!(f, "TYPE_2_INVOCATIONS_PER_PIXEL_NV"),
            Self::TYPE_1_INVOCATION_PER_PIXEL_NV => write!(f, "TYPE_1_INVOCATION_PER_PIXEL_NV"),
            Self::TYPE_1_INVOCATION_PER_2X1_PIXELS_NV => {
                write!(f, "TYPE_1_INVOCATION_PER_2X1_PIXELS_NV")
            }
            Self::TYPE_1_INVOCATION_PER_1X2_PIXELS_NV => {
                write!(f, "TYPE_1_INVOCATION_PER_1X2_PIXELS_NV")
            }
            Self::TYPE_1_INVOCATION_PER_2X2_PIXELS_NV => {
                write!(f, "TYPE_1_INVOCATION_PER_2X2_PIXELS_NV")
            }
            Self::TYPE_1_INVOCATION_PER_4X2_PIXELS_NV => {
                write!(f, "TYPE_1_INVOCATION_PER_4X2_PIXELS_NV")
            }
            Self::TYPE_1_INVOCATION_PER_2X4_PIXELS_NV => {
                write!(f, "TYPE_1_INVOCATION_PER_2X4_PIXELS_NV")
            }
            Self::TYPE_1_INVOCATION_PER_4X4_PIXELS_NV => {
                write!(f, "TYPE_1_INVOCATION_PER_4X4_PIXELS_NV")
            }
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCoarseSampleOrderTypeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CoarseSampleOrderTypeNV(i32);
impl CoarseSampleOrderTypeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl CoarseSampleOrderTypeNV {
    pub const DEFAULT_NV: Self = Self(0);
    pub const CUSTOM_NV: Self = Self(1);
    pub const PIXEL_MAJOR_NV: Self = Self(2);
    pub const SAMPLE_MAJOR_NV: Self = Self(3);
}
impl fmt::Display for CoarseSampleOrderTypeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DEFAULT_NV => write!(f, "DEFAULT_NV"),
            Self::CUSTOM_NV => write!(f, "CUSTOM_NV"),
            Self::PIXEL_MAJOR_NV => write!(f, "PIXEL_MAJOR_NV"),
            Self::SAMPLE_MAJOR_NV => write!(f, "SAMPLE_MAJOR_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineExecutableStatisticFormatKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PipelineExecutableStatisticFormatKHR(i32);
impl PipelineExecutableStatisticFormatKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl PipelineExecutableStatisticFormatKHR {
    pub const BOOL32_KHR: Self = Self(0);
    pub const INT64_KHR: Self = Self(1);
    pub const UINT64_KHR: Self = Self(2);
    pub const FLOAT64_KHR: Self = Self(3);
}
impl fmt::Display for PipelineExecutableStatisticFormatKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::BOOL32_KHR => write!(f, "BOOL32_KHR"),
            Self::INT64_KHR => write!(f, "INT64_KHR"),
            Self::UINT64_KHR => write!(f, "UINT64_KHR"),
            Self::FLOAT64_KHR => write!(f, "FLOAT64_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryResultStatusKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct QueryResultStatusKHR(i32);
impl QueryResultStatusKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl QueryResultStatusKHR {
    pub const ERROR_KHR: Self = Self(-1);
    pub const NOT_READY_KHR: Self = Self(0);
    pub const COMPLETE_KHR: Self = Self(1);
    pub const INSUFFICIENT_BITSTREAM_BUFFER_RANGE_KHR: Self = Self(-1000299000);
}
impl fmt::Display for QueryResultStatusKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ERROR_KHR => write!(f, "ERROR_KHR"),
            Self::NOT_READY_KHR => write!(f, "NOT_READY_KHR"),
            Self::COMPLETE_KHR => write!(f, "COMPLETE_KHR"),
            Self::INSUFFICIENT_BITSTREAM_BUFFER_RANGE_KHR => {
                write!(f, "INSUFFICIENT_BITSTREAM_BUFFER_RANGE_KHR")
            }
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeTuningModeKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct VideoEncodeTuningModeKHR(i32);
impl VideoEncodeTuningModeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl VideoEncodeTuningModeKHR {
    pub const DEFAULT_KHR: Self = Self(0);
    pub const HIGH_QUALITY_KHR: Self = Self(1);
    pub const LOW_LATENCY_KHR: Self = Self(2);
    pub const ULTRA_LOW_LATENCY_KHR: Self = Self(3);
    pub const LOSSLESS_KHR: Self = Self(4);
}
impl fmt::Display for VideoEncodeTuningModeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::DEFAULT_KHR => write!(f, "DEFAULT_KHR"),
            Self::HIGH_QUALITY_KHR => write!(f, "HIGH_QUALITY_KHR"),
            Self::LOW_LATENCY_KHR => write!(f, "LOW_LATENCY_KHR"),
            Self::ULTRA_LOW_LATENCY_KHR => write!(f, "ULTRA_LOW_LATENCY_KHR"),
            Self::LOSSLESS_KHR => write!(f, "LOSSLESS_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1PredictionModeKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct VideoEncodeAV1PredictionModeKHR(i32);
impl VideoEncodeAV1PredictionModeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl VideoEncodeAV1PredictionModeKHR {
    pub const V1_PREDICTION_MODE_INTRA_ONLY_KHR: Self = Self(0);
    pub const V1_PREDICTION_MODE_SINGLE_REFERENCE_KHR: Self = Self(1);
    pub const V1_PREDICTION_MODE_UNIDIRECTIONAL_COMPOUND_KHR: Self = Self(2);
    pub const V1_PREDICTION_MODE_BIDIRECTIONAL_COMPOUND_KHR: Self = Self(3);
}
impl fmt::Display for VideoEncodeAV1PredictionModeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::V1_PREDICTION_MODE_INTRA_ONLY_KHR => {
                write!(f, "V1_PREDICTION_MODE_INTRA_ONLY_KHR")
            }
            Self::V1_PREDICTION_MODE_SINGLE_REFERENCE_KHR => {
                write!(f, "V1_PREDICTION_MODE_SINGLE_REFERENCE_KHR")
            }
            Self::V1_PREDICTION_MODE_UNIDIRECTIONAL_COMPOUND_KHR => {
                write!(f, "V1_PREDICTION_MODE_UNIDIRECTIONAL_COMPOUND_KHR")
            }
            Self::V1_PREDICTION_MODE_BIDIRECTIONAL_COMPOUND_KHR => {
                write!(f, "V1_PREDICTION_MODE_BIDIRECTIONAL_COMPOUND_KHR")
            }
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1RateControlGroupKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct VideoEncodeAV1RateControlGroupKHR(i32);
impl VideoEncodeAV1RateControlGroupKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl VideoEncodeAV1RateControlGroupKHR {
    pub const V1_RATE_CONTROL_GROUP_INTRA_KHR: Self = Self(0);
    pub const V1_RATE_CONTROL_GROUP_PREDICTIVE_KHR: Self = Self(1);
    pub const V1_RATE_CONTROL_GROUP_BIPREDICTIVE_KHR: Self = Self(2);
}
impl fmt::Display for VideoEncodeAV1RateControlGroupKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::V1_RATE_CONTROL_GROUP_INTRA_KHR => write!(f, "V1_RATE_CONTROL_GROUP_INTRA_KHR"),
            Self::V1_RATE_CONTROL_GROUP_PREDICTIVE_KHR => {
                write!(f, "V1_RATE_CONTROL_GROUP_PREDICTIVE_KHR")
            }
            Self::V1_RATE_CONTROL_GROUP_BIPREDICTIVE_KHR => {
                write!(f, "V1_RATE_CONTROL_GROUP_BIPREDICTIVE_KHR")
            }
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDefaultVertexAttributeValueKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DefaultVertexAttributeValueKHR(i32);
impl DefaultVertexAttributeValueKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl DefaultVertexAttributeValueKHR {
    pub const ZERO_ZERO_ZERO_ZERO_KHR: Self = Self(0);
    pub const ZERO_ZERO_ZERO_ONE_KHR: Self = Self(1);
}
impl fmt::Display for DefaultVertexAttributeValueKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ZERO_ZERO_ZERO_ZERO_KHR => write!(f, "ZERO_ZERO_ZERO_ZERO_KHR"),
            Self::ZERO_ZERO_ZERO_ONE_KHR => write!(f, "ZERO_ZERO_ZERO_ONE_KHR"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInstanceTypeNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AccelerationStructureMotionInstanceTypeNV(i32);
impl AccelerationStructureMotionInstanceTypeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}
impl AccelerationStructureMotionInstanceTypeNV {
    pub const STATIC_NV: Self = Self(0);
    pub const MATRIX_MOTION_NV: Self = Self(1);
    pub const SRT_MOTION_NV: Self = Self(2);
}
impl fmt::Display for AccelerationStructureMotionInstanceTypeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::STATIC_NV => write!(f, "STATIC_NV"),
            Self::MATRIX_MOTION_NV => write!(f, "MATRIX_MOTION_NV"),
            Self::SRT_MOTION_NV => write!(f, "SRT_MOTION_NV"),
            x => write!(f, "{x}"),
        }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FramebufferCreateFlags(Flags);
bitflags!(FramebufferCreateFlags, Flags);
impl FramebufferCreateFlags {
    pub const IMAGELESS: Self = Self(0x1);
}
impl fmt::Display for FramebufferCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "IMAGELESS")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryPoolCreateFlags(Flags);
bitflags!(QueryPoolCreateFlags, Flags);
impl QueryPoolCreateFlags {
    pub const RESET_KHR: Self = Self(0x1);
}
impl fmt::Display for QueryPoolCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "RESET_KHR")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderPassCreateFlags(Flags);
bitflags!(RenderPassCreateFlags, Flags);
impl RenderPassCreateFlags {
    pub const TRANSFORM_QCOM: Self = Self(0x2);
    pub const PER_LAYER_FRAGMENT_DENSITY_VALVE: Self = Self(0x4);
}
impl fmt::Display for RenderPassCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (2u64, "TRANSFORM_QCOM"),
            (4u64, "PER_LAYER_FRAGMENT_DENSITY_VALVE"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerCreateFlags(Flags);
bitflags!(SamplerCreateFlags, Flags);
impl SamplerCreateFlags {
    pub const SUBSAMPLED_EXT: Self = Self(0x1);
    pub const SUBSAMPLED_COARSE_RECONSTRUCTION_EXT: Self = Self(0x2);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(0x8);
    pub const NON_SEAMLESS_CUBE_MAP_EXT: Self = Self(0x4);
    pub const IMAGE_PROCESSING_QCOM: Self = Self(0x10);
}
impl fmt::Display for SamplerCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "SUBSAMPLED_EXT"),
            (2u64, "SUBSAMPLED_COARSE_RECONSTRUCTION_EXT"),
            (8u64, "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT"),
            (4u64, "NON_SEAMLESS_CUBE_MAP_EXT"),
            (16u64, "IMAGE_PROCESSING_QCOM"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayoutCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineLayoutCreateFlags(Flags);
bitflags!(PipelineLayoutCreateFlags, Flags);
impl PipelineLayoutCreateFlags {
    pub const INDEPENDENT_SETS_EXT: Self = Self(0x2);
}
impl fmt::Display for PipelineLayoutCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(2u64, "INDEPENDENT_SETS_EXT")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCacheCreateFlags(Flags);
bitflags!(PipelineCacheCreateFlags, Flags);
impl PipelineCacheCreateFlags {
    pub const EXTERNALLY_SYNCHRONIZED: Self = Self(0x1);
    pub const INTERNALLY_SYNCHRONIZED_MERGE_KHR: Self = Self(0x8);
}
impl fmt::Display for PipelineCacheCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "EXTERNALLY_SYNCHRONIZED"),
            (8u64, "INTERNALLY_SYNCHRONIZED_MERGE_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDepthStencilStateCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineDepthStencilStateCreateFlags(Flags);
bitflags!(PipelineDepthStencilStateCreateFlags, Flags);
impl PipelineDepthStencilStateCreateFlags {
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT: Self = Self(0x1);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT: Self = Self(0x2);
}
impl fmt::Display for PipelineDepthStencilStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT"),
            (2u64, "RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDynamicStateCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineDynamicStateCreateFlags(Flags);
bitflags!(PipelineDynamicStateCreateFlags, Flags);
impl PipelineDynamicStateCreateFlags {}
impl fmt::Display for PipelineDynamicStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineColorBlendStateCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineColorBlendStateCreateFlags(Flags);
bitflags!(PipelineColorBlendStateCreateFlags, Flags);
impl PipelineColorBlendStateCreateFlags {
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT: Self = Self(0x1);
}
impl fmt::Display for PipelineColorBlendStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineMultisampleStateCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineMultisampleStateCreateFlags(Flags);
bitflags!(PipelineMultisampleStateCreateFlags, Flags);
impl PipelineMultisampleStateCreateFlags {}
impl fmt::Display for PipelineMultisampleStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationStateCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineRasterizationStateCreateFlags(Flags);
bitflags!(PipelineRasterizationStateCreateFlags, Flags);
impl PipelineRasterizationStateCreateFlags {}
impl fmt::Display for PipelineRasterizationStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportStateCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineViewportStateCreateFlags(Flags);
bitflags!(PipelineViewportStateCreateFlags, Flags);
impl PipelineViewportStateCreateFlags {}
impl fmt::Display for PipelineViewportStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineTessellationStateCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineTessellationStateCreateFlags(Flags);
bitflags!(PipelineTessellationStateCreateFlags, Flags);
impl PipelineTessellationStateCreateFlags {}
impl fmt::Display for PipelineTessellationStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineInputAssemblyStateCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineInputAssemblyStateCreateFlags(Flags);
bitflags!(PipelineInputAssemblyStateCreateFlags, Flags);
impl PipelineInputAssemblyStateCreateFlags {}
impl fmt::Display for PipelineInputAssemblyStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineVertexInputStateCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineVertexInputStateCreateFlags(Flags);
bitflags!(PipelineVertexInputStateCreateFlags, Flags);
impl PipelineVertexInputStateCreateFlags {}
impl fmt::Display for PipelineVertexInputStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineShaderStageCreateFlags(Flags);
bitflags!(PipelineShaderStageCreateFlags, Flags);
impl PipelineShaderStageCreateFlags {
    pub const ALLOW_VARYING_SUBGROUP_SIZE: Self = Self(0x1);
    pub const REQUIRE_FULL_SUBGROUPS: Self = Self(0x2);
}
impl fmt::Display for PipelineShaderStageCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "ALLOW_VARYING_SUBGROUP_SIZE"),
            (2u64, "REQUIRE_FULL_SUBGROUPS"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorSetLayoutCreateFlags(Flags);
bitflags!(DescriptorSetLayoutCreateFlags, Flags);
impl DescriptorSetLayoutCreateFlags {
    pub const UPDATE_AFTER_BIND_POOL: Self = Self(0x2);
    pub const PUSH_DESCRIPTOR: Self = Self(0x1);
    pub const DESCRIPTOR_BUFFER_EXT: Self = Self(0x10);
    pub const EMBEDDED_IMMUTABLE_SAMPLERS_EXT: Self = Self(0x20);
    pub const INDIRECT_BINDABLE_NV: Self = Self(0x80);
    pub const HOST_ONLY_POOL_EXT: Self = Self(0x4);
    pub const PER_STAGE_NV: Self = Self(0x40);
}
impl fmt::Display for DescriptorSetLayoutCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (2u64, "UPDATE_AFTER_BIND_POOL"),
            (1u64, "PUSH_DESCRIPTOR"),
            (16u64, "DESCRIPTOR_BUFFER_EXT"),
            (32u64, "EMBEDDED_IMMUTABLE_SAMPLERS_EXT"),
            (128u64, "INDIRECT_BINDABLE_NV"),
            (4u64, "HOST_ONLY_POOL_EXT"),
            (64u64, "PER_STAGE_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferViewCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferViewCreateFlags(Flags);
bitflags!(BufferViewCreateFlags, Flags);
impl BufferViewCreateFlags {}
impl fmt::Display for BufferViewCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkInstanceCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstanceCreateFlags(Flags);
bitflags!(InstanceCreateFlags, Flags);
impl InstanceCreateFlags {
    pub const ENUMERATE_PORTABILITY_KHR: Self = Self(0x1);
}
impl fmt::Display for InstanceCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "ENUMERATE_PORTABILITY_KHR")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceCreateFlags(Flags);
bitflags!(DeviceCreateFlags, Flags);
impl DeviceCreateFlags {}
impl fmt::Display for DeviceCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceQueueCreateFlags(Flags);
bitflags!(DeviceQueueCreateFlags, Flags);
impl DeviceQueueCreateFlags {
    #[doc = "Queue is a protected-capable device queue"]
    pub const PROTECTED: Self = Self(0x1);
    pub const INTERNALLY_SYNCHRONIZED_KHR: Self = Self(0x4);
}
impl fmt::Display for DeviceQueueCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "PROTECTED"), (4u64, "INTERNALLY_SYNCHRONIZED_KHR")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueueFlags(Flags);
bitflags!(QueueFlags, Flags);
impl QueueFlags {
    #[doc = "Queue supports graphics operations"]
    pub const GRAPHICS: Self = Self(0x1);
    #[doc = "Queue supports compute operations"]
    pub const COMPUTE: Self = Self(0x2);
    #[doc = "Queue supports transfer operations"]
    pub const TRANSFER: Self = Self(0x4);
    #[doc = "Queue supports sparse resource memory management operations"]
    pub const SPARSE_BINDING: Self = Self(0x8);
    #[doc = "Queues may support protected operations"]
    pub const PROTECTED: Self = Self(0x10);
    pub const VIDEO_DECODE_KHR: Self = Self(0x20);
    pub const VIDEO_ENCODE_KHR: Self = Self(0x40);
    pub const OPTICAL_FLOW_NV: Self = Self(0x100);
    pub const DATA_GRAPH_ARM: Self = Self(0x400);
}
impl fmt::Display for QueueFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "GRAPHICS"),
            (2u64, "COMPUTE"),
            (4u64, "TRANSFER"),
            (8u64, "SPARSE_BINDING"),
            (16u64, "PROTECTED"),
            (32u64, "VIDEO_DECODE_KHR"),
            (64u64, "VIDEO_ENCODE_KHR"),
            (256u64, "OPTICAL_FLOW_NV"),
            (1024u64, "DATA_GRAPH_ARM"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryPropertyFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryPropertyFlags(Flags);
bitflags!(MemoryPropertyFlags, Flags);
impl MemoryPropertyFlags {
    #[doc = "If otherwise stated, then allocate memory on device"]
    pub const DEVICE_LOCAL: Self = Self(0x1);
    #[doc = "Memory is mappable by host"]
    pub const HOST_VISIBLE: Self = Self(0x2);
    #[doc = "Memory will have i/o coherency. If not set, application may need to use vkFlushMappedMemoryRanges and vkInvalidateMappedMemoryRanges to flush/invalidate host cache"]
    pub const HOST_COHERENT: Self = Self(0x4);
    #[doc = "Memory will be cached by the host"]
    pub const HOST_CACHED: Self = Self(0x8);
    #[doc = "Memory may be allocated by the driver when it is required"]
    pub const LAZILY_ALLOCATED: Self = Self(0x10);
    #[doc = "Memory is protected"]
    pub const PROTECTED: Self = Self(0x20);
    pub const DEVICE_COHERENT_AMD: Self = Self(0x40);
    pub const DEVICE_UNCACHED_AMD: Self = Self(0x80);
    pub const RDMA_CAPABLE_NV: Self = Self(0x100);
}
impl fmt::Display for MemoryPropertyFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "DEVICE_LOCAL"),
            (2u64, "HOST_VISIBLE"),
            (4u64, "HOST_COHERENT"),
            (8u64, "HOST_CACHED"),
            (16u64, "LAZILY_ALLOCATED"),
            (32u64, "PROTECTED"),
            (64u64, "DEVICE_COHERENT_AMD"),
            (128u64, "DEVICE_UNCACHED_AMD"),
            (256u64, "RDMA_CAPABLE_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryHeapFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryHeapFlags(Flags);
bitflags!(MemoryHeapFlags, Flags);
impl MemoryHeapFlags {
    #[doc = "If set, heap represents device memory"]
    pub const DEVICE_LOCAL: Self = Self(0x1);
    #[doc = "If set, heap allocations allocate multiple instances by default"]
    pub const MULTI_INSTANCE: Self = Self(0x2);
    pub const TILE_MEMORY_QCOM: Self = Self(0x8);
}
impl fmt::Display for MemoryHeapFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "DEVICE_LOCAL"),
            (2u64, "MULTI_INSTANCE"),
            (8u64, "TILE_MEMORY_QCOM"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccessFlags(Flags);
bitflags!(AccessFlags, Flags);
impl AccessFlags {
    #[doc = "Controls coherency of indirect command reads"]
    pub const INDIRECT_COMMAND_READ: Self = Self(0x1);
    #[doc = "Controls coherency of index reads"]
    pub const INDEX_READ: Self = Self(0x2);
    #[doc = "Controls coherency of vertex attribute reads"]
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(0x4);
    #[doc = "Controls coherency of uniform buffer reads"]
    pub const UNIFORM_READ: Self = Self(0x8);
    #[doc = "Controls coherency of input attachment reads"]
    pub const INPUT_ATTACHMENT_READ: Self = Self(0x10);
    #[doc = "Controls coherency of shader reads"]
    pub const SHADER_READ: Self = Self(0x20);
    #[doc = "Controls coherency of shader writes"]
    pub const SHADER_WRITE: Self = Self(0x40);
    #[doc = "Controls coherency of color attachment reads"]
    pub const COLOR_ATTACHMENT_READ: Self = Self(0x80);
    #[doc = "Controls coherency of color attachment writes"]
    pub const COLOR_ATTACHMENT_WRITE: Self = Self(0x100);
    #[doc = "Controls coherency of depth/stencil attachment reads"]
    pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(0x200);
    #[doc = "Controls coherency of depth/stencil attachment writes"]
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(0x400);
    #[doc = "Controls coherency of transfer reads"]
    pub const TRANSFER_READ: Self = Self(0x800);
    #[doc = "Controls coherency of transfer writes"]
    pub const TRANSFER_WRITE: Self = Self(0x1000);
    #[doc = "Controls coherency of host reads"]
    pub const HOST_READ: Self = Self(0x2000);
    #[doc = "Controls coherency of host writes"]
    pub const HOST_WRITE: Self = Self(0x4000);
    #[doc = "Controls coherency of memory reads"]
    pub const MEMORY_READ: Self = Self(0x8000);
    #[doc = "Controls coherency of memory writes"]
    pub const MEMORY_WRITE: Self = Self(0x10000);
    pub const NONE: Self = Self(0x0);
    pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(0x2000000);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(0x4000000);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self = Self(0x8000000);
    #[doc = "read access flag for reading conditional rendering predicate"]
    pub const CONDITIONAL_RENDERING_READ_EXT: Self = Self(0x100000);
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(0x80000);
    pub const ACCELERATION_STRUCTURE_READ_KHR: Self = Self(0x200000);
    pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(0x400000);
    pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(0x1000000);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self = Self(0x800000);
    pub const COMMAND_PREPROCESS_READ_EXT: Self = Self(0x20000);
    pub const COMMAND_PREPROCESS_WRITE_EXT: Self = Self(0x40000);
}
impl fmt::Display for AccessFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "INDIRECT_COMMAND_READ"),
            (2u64, "INDEX_READ"),
            (4u64, "VERTEX_ATTRIBUTE_READ"),
            (8u64, "UNIFORM_READ"),
            (16u64, "INPUT_ATTACHMENT_READ"),
            (32u64, "SHADER_READ"),
            (64u64, "SHADER_WRITE"),
            (128u64, "COLOR_ATTACHMENT_READ"),
            (256u64, "COLOR_ATTACHMENT_WRITE"),
            (512u64, "DEPTH_STENCIL_ATTACHMENT_READ"),
            (1024u64, "DEPTH_STENCIL_ATTACHMENT_WRITE"),
            (2048u64, "TRANSFER_READ"),
            (4096u64, "TRANSFER_WRITE"),
            (8192u64, "HOST_READ"),
            (16384u64, "HOST_WRITE"),
            (32768u64, "MEMORY_READ"),
            (65536u64, "MEMORY_WRITE"),
            (33554432u64, "TRANSFORM_FEEDBACK_WRITE_EXT"),
            (67108864u64, "TRANSFORM_FEEDBACK_COUNTER_READ_EXT"),
            (134217728u64, "TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT"),
            (1048576u64, "CONDITIONAL_RENDERING_READ_EXT"),
            (524288u64, "COLOR_ATTACHMENT_READ_NONCOHERENT_EXT"),
            (2097152u64, "ACCELERATION_STRUCTURE_READ_KHR"),
            (4194304u64, "ACCELERATION_STRUCTURE_WRITE_KHR"),
            (16777216u64, "FRAGMENT_DENSITY_MAP_READ_EXT"),
            (8388608u64, "FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR"),
            (131072u64, "COMMAND_PREPROCESS_READ_EXT"),
            (262144u64, "COMMAND_PREPROCESS_WRITE_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferUsageFlags(Flags);
bitflags!(BufferUsageFlags, Flags);
impl BufferUsageFlags {
    #[doc = "Can be used as a source of transfer operations"]
    pub const TRANSFER_SRC: Self = Self(0x1);
    #[doc = "Can be used as a destination of transfer operations"]
    pub const TRANSFER_DST: Self = Self(0x2);
    #[doc = "Can be used as TBO"]
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(0x4);
    #[doc = "Can be used as IBO"]
    pub const STORAGE_TEXEL_BUFFER: Self = Self(0x8);
    #[doc = "Can be used as UBO"]
    pub const UNIFORM_BUFFER: Self = Self(0x10);
    #[doc = "Can be used as SSBO"]
    pub const STORAGE_BUFFER: Self = Self(0x20);
    #[doc = "Can be used as source of fixed-function index fetch (index buffer)"]
    pub const INDEX_BUFFER: Self = Self(0x40);
    #[doc = "Can be used as source of fixed-function vertex fetch (VBO)"]
    pub const VERTEX_BUFFER: Self = Self(0x80);
    #[doc = "Can be the source of indirect parameters (e.g. indirect buffer, parameter buffer)"]
    pub const INDIRECT_BUFFER: Self = Self(0x100);
    pub const SHADER_DEVICE_ADDRESS: Self = Self(0x20000);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(0x2000);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(0x4000);
    pub const TRANSFORM_FEEDBACK_BUFFER_EXT: Self = Self(0x800);
    pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT: Self = Self(0x1000);
    #[doc = "Specifies the buffer can be used as predicate in conditional rendering"]
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(0x200);
    pub const EXECUTION_GRAPH_SCRATCH_AMDX: Self = Self(0x2000000);
    pub const DESCRIPTOR_HEAP_EXT: Self = Self(0x10000000);
    pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR: Self = Self(0x80000);
    pub const ACCELERATION_STRUCTURE_STORAGE_KHR: Self = Self(0x100000);
    pub const SHADER_BINDING_TABLE_KHR: Self = Self(0x400);
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(0x8000);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(0x10000);
    pub const SAMPLER_DESCRIPTOR_BUFFER_EXT: Self = Self(0x200000);
    pub const RESOURCE_DESCRIPTOR_BUFFER_EXT: Self = Self(0x400000);
    pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT: Self = Self(0x4000000);
    pub const MICROMAP_BUILD_INPUT_READ_ONLY_EXT: Self = Self(0x800000);
    pub const MICROMAP_STORAGE_EXT: Self = Self(0x1000000);
    pub const TILE_MEMORY_QCOM: Self = Self(0x8000000);
}
impl fmt::Display for BufferUsageFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TRANSFER_SRC"),
            (2u64, "TRANSFER_DST"),
            (4u64, "UNIFORM_TEXEL_BUFFER"),
            (8u64, "STORAGE_TEXEL_BUFFER"),
            (16u64, "UNIFORM_BUFFER"),
            (32u64, "STORAGE_BUFFER"),
            (64u64, "INDEX_BUFFER"),
            (128u64, "VERTEX_BUFFER"),
            (256u64, "INDIRECT_BUFFER"),
            (131072u64, "SHADER_DEVICE_ADDRESS"),
            (8192u64, "VIDEO_DECODE_SRC_KHR"),
            (16384u64, "VIDEO_DECODE_DST_KHR"),
            (2048u64, "TRANSFORM_FEEDBACK_BUFFER_EXT"),
            (4096u64, "TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT"),
            (512u64, "CONDITIONAL_RENDERING_EXT"),
            (33554432u64, "EXECUTION_GRAPH_SCRATCH_AMDX"),
            (268435456u64, "DESCRIPTOR_HEAP_EXT"),
            (
                524288u64,
                "ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR",
            ),
            (1048576u64, "ACCELERATION_STRUCTURE_STORAGE_KHR"),
            (1024u64, "SHADER_BINDING_TABLE_KHR"),
            (32768u64, "VIDEO_ENCODE_DST_KHR"),
            (65536u64, "VIDEO_ENCODE_SRC_KHR"),
            (2097152u64, "SAMPLER_DESCRIPTOR_BUFFER_EXT"),
            (4194304u64, "RESOURCE_DESCRIPTOR_BUFFER_EXT"),
            (67108864u64, "PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT"),
            (8388608u64, "MICROMAP_BUILD_INPUT_READ_ONLY_EXT"),
            (16777216u64, "MICROMAP_STORAGE_EXT"),
            (134217728u64, "TILE_MEMORY_QCOM"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferCreateFlags(Flags);
bitflags!(BufferCreateFlags, Flags);
impl BufferCreateFlags {
    #[doc = "Buffer should support sparse backing"]
    pub const SPARSE_BINDING: Self = Self(0x1);
    #[doc = "Buffer should support sparse backing with partial residency"]
    pub const SPARSE_RESIDENCY: Self = Self(0x2);
    #[doc = "Buffer should support constant data access to physical memory ranges mapped into multiple locations of sparse buffers"]
    pub const SPARSE_ALIASED: Self = Self(0x4);
    #[doc = "Buffer requires protected memory"]
    pub const PROTECTED: Self = Self(0x8);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(0x10);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(0x20);
    pub const VIDEO_PROFILE_INDEPENDENT_KHR: Self = Self(0x40);
}
impl fmt::Display for BufferCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "SPARSE_BINDING"),
            (2u64, "SPARSE_RESIDENCY"),
            (4u64, "SPARSE_ALIASED"),
            (8u64, "PROTECTED"),
            (16u64, "DEVICE_ADDRESS_CAPTURE_REPLAY"),
            (32u64, "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT"),
            (64u64, "VIDEO_PROFILE_INDEPENDENT_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderStageFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderStageFlags(Flags);
bitflags!(ShaderStageFlags, Flags);
impl ShaderStageFlags {
    pub const VERTEX: Self = Self(0x1);
    pub const TESSELLATION_CONTROL: Self = Self(0x2);
    pub const TESSELLATION_EVALUATION: Self = Self(0x4);
    pub const GEOMETRY: Self = Self(0x8);
    pub const FRAGMENT: Self = Self(0x10);
    pub const COMPUTE: Self = Self(0x20);
    pub const ALL_GRAPHICS: Self = Self(0x1f);
    pub const ALL: Self = Self(0x7fffffff);
    pub const RAYGEN_KHR: Self = Self(0x100);
    pub const ANY_HIT_KHR: Self = Self(0x200);
    pub const CLOSEST_HIT_KHR: Self = Self(0x400);
    pub const MISS_KHR: Self = Self(0x800);
    pub const INTERSECTION_KHR: Self = Self(0x1000);
    pub const CALLABLE_KHR: Self = Self(0x2000);
    pub const TASK_EXT: Self = Self(0x40);
    pub const MESH_EXT: Self = Self(0x80);
    pub const SUBPASS_SHADING_HUAWEI: Self = Self(0x4000);
    pub const CLUSTER_CULLING_HUAWEI: Self = Self(0x80000);
}
impl fmt::Display for ShaderStageFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "VERTEX"),
            (2u64, "TESSELLATION_CONTROL"),
            (4u64, "TESSELLATION_EVALUATION"),
            (8u64, "GEOMETRY"),
            (16u64, "FRAGMENT"),
            (32u64, "COMPUTE"),
            (31u64, "ALL_GRAPHICS"),
            (2147483647u64, "ALL"),
            (256u64, "RAYGEN_KHR"),
            (512u64, "ANY_HIT_KHR"),
            (1024u64, "CLOSEST_HIT_KHR"),
            (2048u64, "MISS_KHR"),
            (4096u64, "INTERSECTION_KHR"),
            (8192u64, "CALLABLE_KHR"),
            (64u64, "TASK_EXT"),
            (128u64, "MESH_EXT"),
            (16384u64, "SUBPASS_SHADING_HUAWEI"),
            (524288u64, "CLUSTER_CULLING_HUAWEI"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageUsageFlags(Flags);
bitflags!(ImageUsageFlags, Flags);
impl ImageUsageFlags {
    #[doc = "Can be used as a source of transfer operations"]
    pub const TRANSFER_SRC: Self = Self(0x1);
    #[doc = "Can be used as a destination of transfer operations"]
    pub const TRANSFER_DST: Self = Self(0x2);
    #[doc = "Can be sampled from (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)"]
    pub const SAMPLED: Self = Self(0x4);
    #[doc = "Can be used as storage image (STORAGE_IMAGE descriptor type)"]
    pub const STORAGE: Self = Self(0x8);
    #[doc = "Can be used as framebuffer color attachment"]
    pub const COLOR_ATTACHMENT: Self = Self(0x10);
    #[doc = "Can be used as framebuffer depth/stencil attachment"]
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(0x20);
    #[doc = "Image data not needed outside of rendering"]
    pub const TRANSIENT_ATTACHMENT: Self = Self(0x40);
    #[doc = "Can be used as framebuffer input attachment"]
    pub const INPUT_ATTACHMENT: Self = Self(0x80);
    pub const HOST_TRANSFER: Self = Self(0x400000);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(0x400);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(0x800);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(0x1000);
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(0x200);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(0x100);
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(0x2000);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(0x4000);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(0x8000);
    pub const ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(0x80000);
    pub const INVOCATION_MASK_HUAWEI: Self = Self(0x40000);
    pub const SAMPLE_WEIGHT_QCOM: Self = Self(0x100000);
    pub const SAMPLE_BLOCK_MATCH_QCOM: Self = Self(0x200000);
    pub const TENSOR_ALIASING_ARM: Self = Self(0x800000);
    pub const TILE_MEMORY_QCOM: Self = Self(0x8000000);
    pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(0x2000000);
    pub const VIDEO_ENCODE_EMPHASIS_MAP_KHR: Self = Self(0x4000000);
}
impl fmt::Display for ImageUsageFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TRANSFER_SRC"),
            (2u64, "TRANSFER_DST"),
            (4u64, "SAMPLED"),
            (8u64, "STORAGE"),
            (16u64, "COLOR_ATTACHMENT"),
            (32u64, "DEPTH_STENCIL_ATTACHMENT"),
            (64u64, "TRANSIENT_ATTACHMENT"),
            (128u64, "INPUT_ATTACHMENT"),
            (4194304u64, "HOST_TRANSFER"),
            (1024u64, "VIDEO_DECODE_DST_KHR"),
            (2048u64, "VIDEO_DECODE_SRC_KHR"),
            (4096u64, "VIDEO_DECODE_DPB_KHR"),
            (512u64, "FRAGMENT_DENSITY_MAP_EXT"),
            (256u64, "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR"),
            (8192u64, "VIDEO_ENCODE_DST_KHR"),
            (16384u64, "VIDEO_ENCODE_SRC_KHR"),
            (32768u64, "VIDEO_ENCODE_DPB_KHR"),
            (524288u64, "ATTACHMENT_FEEDBACK_LOOP_EXT"),
            (262144u64, "INVOCATION_MASK_HUAWEI"),
            (1048576u64, "SAMPLE_WEIGHT_QCOM"),
            (2097152u64, "SAMPLE_BLOCK_MATCH_QCOM"),
            (8388608u64, "TENSOR_ALIASING_ARM"),
            (134217728u64, "TILE_MEMORY_QCOM"),
            (33554432u64, "VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR"),
            (67108864u64, "VIDEO_ENCODE_EMPHASIS_MAP_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageCreateFlags(Flags);
bitflags!(ImageCreateFlags, Flags);
impl ImageCreateFlags {
    #[doc = "Image should support sparse backing"]
    pub const SPARSE_BINDING: Self = Self(0x1);
    #[doc = "Image should support sparse backing with partial residency"]
    pub const SPARSE_RESIDENCY: Self = Self(0x2);
    #[doc = "Image should support constant data access to physical memory ranges mapped into multiple locations of sparse images"]
    pub const SPARSE_ALIASED: Self = Self(0x4);
    #[doc = "Allows image views to have different format than the base image"]
    pub const MUTABLE_FORMAT: Self = Self(0x8);
    #[doc = "Allows creating image views with cube type from the created image"]
    pub const CUBE_COMPATIBLE: Self = Self(0x10);
    pub const ALIAS: Self = Self(0x400);
    #[doc = "Allows using VkBindImageMemoryDeviceGroupInfo::pSplitInstanceBindRegions when binding memory to the image"]
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(0x40);
    #[doc = "The 3D image can be viewed as a 2D or 2D array image"]
    pub const TYPE_2D_ARRAY_COMPATIBLE: Self = Self(0x20);
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE: Self = Self(0x80);
    pub const EXTENDED_USAGE: Self = Self(0x100);
    #[doc = "Image requires protected memory"]
    pub const PROTECTED: Self = Self(0x800);
    pub const DISJOINT: Self = Self(0x200);
    pub const CORNER_SAMPLED_NV: Self = Self(0x2000);
    pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT: Self = Self(0x10000);
    pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT: Self = Self(0x1000);
    pub const SUBSAMPLED_EXT: Self = Self(0x4000);
    pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT: Self = Self(0x40000);
    #[doc = "Image is created with a layout where individual slices are capable of being used as 2D images"]
    pub const TYPE_2D_VIEW_COMPATIBLE_EXT: Self = Self(0x20000);
    pub const VIDEO_PROFILE_INDEPENDENT_KHR: Self = Self(0x100000);
    pub const FRAGMENT_DENSITY_MAP_OFFSET_EXT: Self = Self(0x8000);
}
impl fmt::Display for ImageCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "SPARSE_BINDING"),
            (2u64, "SPARSE_RESIDENCY"),
            (4u64, "SPARSE_ALIASED"),
            (8u64, "MUTABLE_FORMAT"),
            (16u64, "CUBE_COMPATIBLE"),
            (1024u64, "ALIAS"),
            (64u64, "SPLIT_INSTANCE_BIND_REGIONS"),
            (32u64, "TYPE_2D_ARRAY_COMPATIBLE"),
            (128u64, "BLOCK_TEXEL_VIEW_COMPATIBLE"),
            (256u64, "EXTENDED_USAGE"),
            (2048u64, "PROTECTED"),
            (512u64, "DISJOINT"),
            (8192u64, "CORNER_SAMPLED_NV"),
            (65536u64, "DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT"),
            (4096u64, "SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT"),
            (16384u64, "SUBSAMPLED_EXT"),
            (262144u64, "MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT"),
            (131072u64, "TYPE_2D_VIEW_COMPATIBLE_EXT"),
            (1048576u64, "VIDEO_PROFILE_INDEPENDENT_KHR"),
            (32768u64, "FRAGMENT_DENSITY_MAP_OFFSET_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageViewCreateFlags(Flags);
bitflags!(ImageViewCreateFlags, Flags);
impl ImageViewCreateFlags {
    pub const FRAGMENT_DENSITY_MAP_DYNAMIC_EXT: Self = Self(0x1);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(0x4);
    pub const FRAGMENT_DENSITY_MAP_DEFERRED_EXT: Self = Self(0x2);
}
impl fmt::Display for ImageViewCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "FRAGMENT_DENSITY_MAP_DYNAMIC_EXT"),
            (4u64, "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT"),
            (2u64, "FRAGMENT_DENSITY_MAP_DEFERRED_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCreateFlags(Flags);
bitflags!(PipelineCreateFlags, Flags);
impl PipelineCreateFlags {
    pub const DISABLE_OPTIMIZATION: Self = Self(0x1);
    pub const ALLOW_DERIVATIVES: Self = Self(0x2);
    pub const DERIVATIVE: Self = Self(0x4);
    pub const DISPATCH_BASE: Self = Self(0x10);
    pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(0x8);
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(0x100);
    pub const EARLY_RETURN_ON_FAILURE: Self = Self(0x200);
    pub const NO_PROTECTED_ACCESS: Self = Self(0x8000000);
    pub const PROTECTED_ACCESS_ONLY: Self = Self(0x40000000);
    pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR: Self = Self(0x4000);
    pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR: Self = Self(0x8000);
    pub const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR: Self = Self(0x10000);
    pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR: Self = Self(0x20000);
    pub const RAY_TRACING_SKIP_TRIANGLES_KHR: Self = Self(0x1000);
    pub const RAY_TRACING_SKIP_AABBS_KHR: Self = Self(0x2000);
    pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR: Self = Self(0x80000);
    pub const DEFER_COMPILE_NV: Self = Self(0x20);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self = Self(0x400000);
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(0x200000);
    pub const CAPTURE_STATISTICS_KHR: Self = Self(0x40);
    pub const CAPTURE_INTERNAL_REPRESENTATIONS_KHR: Self = Self(0x80);
    pub const INDIRECT_BINDABLE_NV: Self = Self(0x40000);
    pub const LIBRARY_KHR: Self = Self(0x800);
    pub const DESCRIPTOR_BUFFER_EXT: Self = Self(0x20000000);
    pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT: Self = Self(0x800000);
    pub const LINK_TIME_OPTIMIZATION_EXT: Self = Self(0x400);
    pub const RAY_TRACING_ALLOW_MOTION_NV: Self = Self(0x100000);
    pub const COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(0x2000000);
    pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(0x4000000);
    pub const RAY_TRACING_OPACITY_MICROMAP_EXT: Self = Self(0x1000000);
    pub const RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Self = Self(0x10000000);
}
impl fmt::Display for PipelineCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "DISABLE_OPTIMIZATION"),
            (2u64, "ALLOW_DERIVATIVES"),
            (4u64, "DERIVATIVE"),
            (16u64, "DISPATCH_BASE"),
            (8u64, "VIEW_INDEX_FROM_DEVICE_INDEX"),
            (256u64, "FAIL_ON_PIPELINE_COMPILE_REQUIRED"),
            (512u64, "EARLY_RETURN_ON_FAILURE"),
            (134217728u64, "NO_PROTECTED_ACCESS"),
            (1073741824u64, "PROTECTED_ACCESS_ONLY"),
            (16384u64, "RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR"),
            (32768u64, "RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR"),
            (65536u64, "RAY_TRACING_NO_NULL_MISS_SHADERS_KHR"),
            (131072u64, "RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR"),
            (4096u64, "RAY_TRACING_SKIP_TRIANGLES_KHR"),
            (8192u64, "RAY_TRACING_SKIP_AABBS_KHR"),
            (
                524288u64,
                "RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR",
            ),
            (32u64, "DEFER_COMPILE_NV"),
            (4194304u64, "RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT"),
            (2097152u64, "RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR"),
            (64u64, "CAPTURE_STATISTICS_KHR"),
            (128u64, "CAPTURE_INTERNAL_REPRESENTATIONS_KHR"),
            (262144u64, "INDIRECT_BINDABLE_NV"),
            (2048u64, "LIBRARY_KHR"),
            (536870912u64, "DESCRIPTOR_BUFFER_EXT"),
            (8388608u64, "RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT"),
            (1024u64, "LINK_TIME_OPTIMIZATION_EXT"),
            (1048576u64, "RAY_TRACING_ALLOW_MOTION_NV"),
            (33554432u64, "COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT"),
            (67108864u64, "DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT"),
            (16777216u64, "RAY_TRACING_OPACITY_MICROMAP_EXT"),
            (268435456u64, "RAY_TRACING_DISPLACEMENT_MICROMAP_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkColorComponentFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorComponentFlags(Flags);
bitflags!(ColorComponentFlags, Flags);
impl ColorComponentFlags {
    pub const R: Self = Self(0x1);
    pub const G: Self = Self(0x2);
    pub const B: Self = Self(0x4);
    pub const A: Self = Self(0x8);
}
impl fmt::Display for ColorComponentFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "R"), (2u64, "G"), (4u64, "B"), (8u64, "A")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FenceCreateFlags(Flags);
bitflags!(FenceCreateFlags, Flags);
impl FenceCreateFlags {
    pub const SIGNALED: Self = Self(0x1);
}
impl fmt::Display for FenceCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "SIGNALED")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreCreateFlags(Flags);
bitflags!(SemaphoreCreateFlags, Flags);
impl SemaphoreCreateFlags {}
impl fmt::Display for SemaphoreCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FormatFeatureFlags(Flags);
bitflags!(FormatFeatureFlags, Flags);
impl FormatFeatureFlags {
    #[doc = "Format can be used for sampled images (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)"]
    pub const SAMPLED_IMAGE: Self = Self(0x1);
    #[doc = "Format can be used for storage images (STORAGE_IMAGE descriptor type)"]
    pub const STORAGE_IMAGE: Self = Self(0x2);
    #[doc = "Format supports atomic operations in case it is used for storage images"]
    pub const STORAGE_IMAGE_ATOMIC: Self = Self(0x4);
    #[doc = "Format can be used for uniform texel buffers (TBOs)"]
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(0x8);
    #[doc = "Format can be used for storage texel buffers (IBOs)"]
    pub const STORAGE_TEXEL_BUFFER: Self = Self(0x10);
    #[doc = "Format supports atomic operations in case it is used for storage texel buffers"]
    pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(0x20);
    #[doc = "Format can be used for vertex buffers (VBOs)"]
    pub const VERTEX_BUFFER: Self = Self(0x40);
    #[doc = "Format can be used for color attachment images"]
    pub const COLOR_ATTACHMENT: Self = Self(0x80);
    #[doc = "Format supports blending in case it is used for color attachment images"]
    pub const COLOR_ATTACHMENT_BLEND: Self = Self(0x100);
    #[doc = "Format can be used for depth/stencil attachment images"]
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(0x200);
    #[doc = "Format can be used as the source image of blits with vkCmdBlitImage"]
    pub const BLIT_SRC: Self = Self(0x400);
    #[doc = "Format can be used as the destination image of blits with vkCmdBlitImage"]
    pub const BLIT_DST: Self = Self(0x800);
    #[doc = "Format can be filtered with VK_FILTER_LINEAR when being sampled"]
    pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(0x1000);
    #[doc = "Format can be used as the source image of image transfer commands"]
    pub const TRANSFER_SRC: Self = Self(0x4000);
    #[doc = "Format can be used as the destination image of image transfer commands"]
    pub const TRANSFER_DST: Self = Self(0x8000);
    #[doc = "Format can have midpoint rather than cosited chroma samples"]
    pub const MIDPOINT_CHROMA_SAMPLES: Self = Self(0x20000);
    #[doc = "Format can be used with linear filtering whilst color conversion is enabled"]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(0x40000);
    #[doc = "Format can have different chroma, min and mag filters"]
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self = Self(0x80000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self = Self(0x100000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self =
        Self(0x200000);
    #[doc = "Format supports disjoint planes"]
    pub const DISJOINT: Self = Self(0x400000);
    #[doc = "Format can have cosited rather than midpoint chroma samples"]
    pub const COSITED_CHROMA_SAMPLES: Self = Self(0x800000);
    #[doc = "Format can be used with min/max reduction filtering"]
    pub const SAMPLED_IMAGE_FILTER_MINMAX: Self = Self(0x10000);
    pub const VIDEO_DECODE_OUTPUT_KHR: Self = Self(0x2000000);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(0x4000000);
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self = Self(0x20000000);
    pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self(0x2000);
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(0x1000000);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(0x40000000);
    pub const VIDEO_ENCODE_INPUT_KHR: Self = Self(0x8000000);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(0x10000000);
}
impl fmt::Display for FormatFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "SAMPLED_IMAGE"),
            (2u64, "STORAGE_IMAGE"),
            (4u64, "STORAGE_IMAGE_ATOMIC"),
            (8u64, "UNIFORM_TEXEL_BUFFER"),
            (16u64, "STORAGE_TEXEL_BUFFER"),
            (32u64, "STORAGE_TEXEL_BUFFER_ATOMIC"),
            (64u64, "VERTEX_BUFFER"),
            (128u64, "COLOR_ATTACHMENT"),
            (256u64, "COLOR_ATTACHMENT_BLEND"),
            (512u64, "DEPTH_STENCIL_ATTACHMENT"),
            (1024u64, "BLIT_SRC"),
            (2048u64, "BLIT_DST"),
            (4096u64, "SAMPLED_IMAGE_FILTER_LINEAR"),
            (16384u64, "TRANSFER_SRC"),
            (32768u64, "TRANSFER_DST"),
            (131072u64, "MIDPOINT_CHROMA_SAMPLES"),
            (262144u64, "SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER"),
            (
                524288u64,
                "SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER",
            ),
            (
                1048576u64,
                "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT",
            ),
            (
                2097152u64,
                "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE",
            ),
            (4194304u64, "DISJOINT"),
            (8388608u64, "COSITED_CHROMA_SAMPLES"),
            (65536u64, "SAMPLED_IMAGE_FILTER_MINMAX"),
            (33554432u64, "VIDEO_DECODE_OUTPUT_KHR"),
            (67108864u64, "VIDEO_DECODE_DPB_KHR"),
            (536870912u64, "ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR"),
            (8192u64, "SAMPLED_IMAGE_FILTER_CUBIC_EXT"),
            (16777216u64, "FRAGMENT_DENSITY_MAP_EXT"),
            (1073741824u64, "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR"),
            (134217728u64, "VIDEO_ENCODE_INPUT_KHR"),
            (268435456u64, "VIDEO_ENCODE_DPB_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryControlFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryControlFlags(Flags);
bitflags!(QueryControlFlags, Flags);
impl QueryControlFlags {
    #[doc = "Require precise results to be collected by the query"]
    pub const PRECISE: Self = Self(0x1);
}
impl fmt::Display for QueryControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "PRECISE")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryResultFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryResultFlags(Flags);
bitflags!(QueryResultFlags, Flags);
impl QueryResultFlags {
    #[doc = "Results of the queries are written to the destination buffer as 64-bit values"]
    pub const TYPE_64: Self = Self(0x1);
    #[doc = "Results of the queries are waited on before proceeding with the result copy"]
    pub const WAIT: Self = Self(0x2);
    #[doc = "Besides the results of the query, the availability of the results is also written"]
    pub const WITH_AVAILABILITY: Self = Self(0x4);
    #[doc = "Copy the partial results of the query even if the final results are not available"]
    pub const PARTIAL: Self = Self(0x8);
    pub const WITH_STATUS_KHR: Self = Self(0x10);
}
impl fmt::Display for QueryResultFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TYPE_64"),
            (2u64, "WAIT"),
            (4u64, "WITH_AVAILABILITY"),
            (8u64, "PARTIAL"),
            (16u64, "WITH_STATUS_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModuleCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderModuleCreateFlags(Flags);
bitflags!(ShaderModuleCreateFlags, Flags);
impl ShaderModuleCreateFlags {}
impl fmt::Display for ShaderModuleCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkEventCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventCreateFlags(Flags);
bitflags!(EventCreateFlags, Flags);
impl EventCreateFlags {
    pub const DEVICE_ONLY: Self = Self(0x1);
}
impl fmt::Display for EventCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "DEVICE_ONLY")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandPoolCreateFlags(Flags);
bitflags!(CommandPoolCreateFlags, Flags);
impl CommandPoolCreateFlags {
    #[doc = "Command buffers have a short lifetime"]
    pub const TRANSIENT: Self = Self(0x1);
    #[doc = "Command buffers may release their memory individually"]
    pub const RESET_COMMAND_BUFFER: Self = Self(0x2);
    #[doc = "Command buffers allocated from pool are protected command buffers"]
    pub const PROTECTED: Self = Self(0x4);
}
impl fmt::Display for CommandPoolCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TRANSIENT"),
            (2u64, "RESET_COMMAND_BUFFER"),
            (4u64, "PROTECTED"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolResetFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandPoolResetFlags(Flags);
bitflags!(CommandPoolResetFlags, Flags);
impl CommandPoolResetFlags {
    #[doc = "Release resources owned by the pool"]
    pub const RELEASE_RESOURCES: Self = Self(0x1);
}
impl fmt::Display for CommandPoolResetFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "RELEASE_RESOURCES")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferResetFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandBufferResetFlags(Flags);
bitflags!(CommandBufferResetFlags, Flags);
impl CommandBufferResetFlags {
    #[doc = "Release resources owned by the buffer"]
    pub const RELEASE_RESOURCES: Self = Self(0x1);
}
impl fmt::Display for CommandBufferResetFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "RELEASE_RESOURCES")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferUsageFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandBufferUsageFlags(Flags);
bitflags!(CommandBufferUsageFlags, Flags);
impl CommandBufferUsageFlags {
    pub const ONE_TIME_SUBMIT: Self = Self(0x1);
    pub const RENDER_PASS_CONTINUE: Self = Self(0x2);
    #[doc = "Command buffer may be submitted/executed more than once simultaneously"]
    pub const SIMULTANEOUS_USE: Self = Self(0x4);
}
impl fmt::Display for CommandBufferUsageFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "ONE_TIME_SUBMIT"),
            (2u64, "RENDER_PASS_CONTINUE"),
            (4u64, "SIMULTANEOUS_USE"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPipelineStatisticFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryPipelineStatisticFlags(Flags);
bitflags!(QueryPipelineStatisticFlags, Flags);
impl QueryPipelineStatisticFlags {
    #[doc = "Optional"]
    pub const INPUT_ASSEMBLY_VERTICES: Self = Self(0x1);
    #[doc = "Optional"]
    pub const INPUT_ASSEMBLY_PRIMITIVES: Self = Self(0x2);
    #[doc = "Optional"]
    pub const VERTEX_SHADER_INVOCATIONS: Self = Self(0x4);
    #[doc = "Optional"]
    pub const GEOMETRY_SHADER_INVOCATIONS: Self = Self(0x8);
    #[doc = "Optional"]
    pub const GEOMETRY_SHADER_PRIMITIVES: Self = Self(0x10);
    #[doc = "Optional"]
    pub const CLIPPING_INVOCATIONS: Self = Self(0x20);
    #[doc = "Optional"]
    pub const CLIPPING_PRIMITIVES: Self = Self(0x40);
    #[doc = "Optional"]
    pub const FRAGMENT_SHADER_INVOCATIONS: Self = Self(0x80);
    #[doc = "Optional"]
    pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self = Self(0x100);
    #[doc = "Optional"]
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self = Self(0x200);
    #[doc = "Optional"]
    pub const COMPUTE_SHADER_INVOCATIONS: Self = Self(0x400);
    pub const TASK_SHADER_INVOCATIONS_EXT: Self = Self(0x800);
    pub const MESH_SHADER_INVOCATIONS_EXT: Self = Self(0x1000);
    pub const CLUSTER_CULLING_SHADER_INVOCATIONS_HUAWEI: Self = Self(0x2000);
}
impl fmt::Display for QueryPipelineStatisticFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "INPUT_ASSEMBLY_VERTICES"),
            (2u64, "INPUT_ASSEMBLY_PRIMITIVES"),
            (4u64, "VERTEX_SHADER_INVOCATIONS"),
            (8u64, "GEOMETRY_SHADER_INVOCATIONS"),
            (16u64, "GEOMETRY_SHADER_PRIMITIVES"),
            (32u64, "CLIPPING_INVOCATIONS"),
            (64u64, "CLIPPING_PRIMITIVES"),
            (128u64, "FRAGMENT_SHADER_INVOCATIONS"),
            (256u64, "TESSELLATION_CONTROL_SHADER_PATCHES"),
            (512u64, "TESSELLATION_EVALUATION_SHADER_INVOCATIONS"),
            (1024u64, "COMPUTE_SHADER_INVOCATIONS"),
            (2048u64, "TASK_SHADER_INVOCATIONS_EXT"),
            (4096u64, "MESH_SHADER_INVOCATIONS_EXT"),
            (8192u64, "CLUSTER_CULLING_SHADER_INVOCATIONS_HUAWEI"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryMapFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryMapFlags(Flags);
bitflags!(MemoryMapFlags, Flags);
impl MemoryMapFlags {
    pub const PLACED_EXT: Self = Self(0x1);
}
impl fmt::Display for MemoryMapFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "PLACED_EXT")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryUnmapFlags(Flags);
bitflags!(MemoryUnmapFlags, Flags);
impl MemoryUnmapFlags {
    pub const RESERVE_EXT: Self = Self(0x1);
}
impl fmt::Display for MemoryUnmapFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "RESERVE_EXT")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageAspectFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageAspectFlags(Flags);
bitflags!(ImageAspectFlags, Flags);
impl ImageAspectFlags {
    pub const COLOR: Self = Self(0x1);
    pub const DEPTH: Self = Self(0x2);
    pub const STENCIL: Self = Self(0x4);
    pub const METADATA: Self = Self(0x8);
    pub const PLANE_0: Self = Self(0x10);
    pub const PLANE_1: Self = Self(0x20);
    pub const PLANE_2: Self = Self(0x40);
    pub const NONE: Self = Self(0x0);
    pub const MEMORY_PLANE_0_EXT: Self = Self(0x80);
    pub const MEMORY_PLANE_1_EXT: Self = Self(0x100);
    pub const MEMORY_PLANE_2_EXT: Self = Self(0x200);
    pub const MEMORY_PLANE_3_EXT: Self = Self(0x400);
}
impl fmt::Display for ImageAspectFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "COLOR"),
            (2u64, "DEPTH"),
            (4u64, "STENCIL"),
            (8u64, "METADATA"),
            (16u64, "PLANE_0"),
            (32u64, "PLANE_1"),
            (64u64, "PLANE_2"),
            (128u64, "MEMORY_PLANE_0_EXT"),
            (256u64, "MEMORY_PLANE_1_EXT"),
            (512u64, "MEMORY_PLANE_2_EXT"),
            (1024u64, "MEMORY_PLANE_3_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseMemoryBindFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SparseMemoryBindFlags(Flags);
bitflags!(SparseMemoryBindFlags, Flags);
impl SparseMemoryBindFlags {
    #[doc = "Operation binds resource metadata to memory"]
    pub const METADATA: Self = Self(0x1);
}
impl fmt::Display for SparseMemoryBindFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "METADATA")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageFormatFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SparseImageFormatFlags(Flags);
bitflags!(SparseImageFormatFlags, Flags);
impl SparseImageFormatFlags {
    #[doc = "Image uses a single mip tail region for all array layers"]
    pub const SINGLE_MIPTAIL: Self = Self(0x1);
    #[doc = "Image requires mip level dimensions to be an integer multiple of the sparse image block dimensions for non-tail mip levels."]
    pub const ALIGNED_MIP_SIZE: Self = Self(0x2);
    #[doc = "Image uses a non-standard sparse image block dimensions"]
    pub const NONSTANDARD_BLOCK_SIZE: Self = Self(0x4);
}
impl fmt::Display for SparseImageFormatFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "SINGLE_MIPTAIL"),
            (2u64, "ALIGNED_MIP_SIZE"),
            (4u64, "NONSTANDARD_BLOCK_SIZE"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescriptionFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubpassDescriptionFlags(Flags);
bitflags!(SubpassDescriptionFlags, Flags);
impl SubpassDescriptionFlags {
    pub const PER_VIEW_ATTRIBUTES_NVX: Self = Self(0x1);
    pub const PER_VIEW_POSITION_X_ONLY_NVX: Self = Self(0x2);
    pub const TILE_SHADING_APRON_QCOM: Self = Self(0x100);
    pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT: Self = Self(0x10);
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT: Self = Self(0x20);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT: Self = Self(0x40);
    pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(0x80);
    pub const FRAGMENT_REGION_EXT: Self = Self(0x4);
    pub const CUSTOM_RESOLVE_EXT: Self = Self(0x8);
}
impl fmt::Display for SubpassDescriptionFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "PER_VIEW_ATTRIBUTES_NVX"),
            (2u64, "PER_VIEW_POSITION_X_ONLY_NVX"),
            (256u64, "TILE_SHADING_APRON_QCOM"),
            (16u64, "RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT"),
            (32u64, "RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT"),
            (64u64, "RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT"),
            (128u64, "ENABLE_LEGACY_DITHERING_EXT"),
            (4u64, "FRAGMENT_REGION_EXT"),
            (8u64, "CUSTOM_RESOLVE_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineStageFlags(Flags);
bitflags!(PipelineStageFlags, Flags);
impl PipelineStageFlags {
    #[doc = "Before subsequent commands are processed"]
    pub const TOP_OF_PIPE: Self = Self(0x1);
    #[doc = "Draw/DispatchIndirect command fetch"]
    pub const DRAW_INDIRECT: Self = Self(0x2);
    #[doc = "Vertex/index fetch"]
    pub const VERTEX_INPUT: Self = Self(0x4);
    #[doc = "Vertex shading"]
    pub const VERTEX_SHADER: Self = Self(0x8);
    #[doc = "Tessellation control shading"]
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(0x10);
    #[doc = "Tessellation evaluation shading"]
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(0x20);
    #[doc = "Geometry shading"]
    pub const GEOMETRY_SHADER: Self = Self(0x40);
    #[doc = "Fragment shading"]
    pub const FRAGMENT_SHADER: Self = Self(0x80);
    #[doc = "Early fragment (depth and stencil) tests"]
    pub const EARLY_FRAGMENT_TESTS: Self = Self(0x100);
    #[doc = "Late fragment (depth and stencil) tests"]
    pub const LATE_FRAGMENT_TESTS: Self = Self(0x200);
    #[doc = "Color attachment writes"]
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(0x400);
    #[doc = "Compute shading"]
    pub const COMPUTE_SHADER: Self = Self(0x800);
    #[doc = "Transfer/copy operations"]
    pub const TRANSFER: Self = Self(0x1000);
    #[doc = "After previous commands have completed"]
    pub const BOTTOM_OF_PIPE: Self = Self(0x2000);
    #[doc = "Indicates host (CPU) is a source/sink of the dependency"]
    pub const HOST: Self = Self(0x4000);
    #[doc = "All stages of the graphics pipeline"]
    pub const ALL_GRAPHICS: Self = Self(0x8000);
    #[doc = "All stages supported on the queue"]
    pub const ALL_COMMANDS: Self = Self(0x10000);
    pub const NONE: Self = Self(0x0);
    pub const TRANSFORM_FEEDBACK_EXT: Self = Self(0x1000000);
    #[doc = "A pipeline stage for conditional rendering predicate fetch"]
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(0x40000);
    pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(0x2000000);
    pub const RAY_TRACING_SHADER_KHR: Self = Self(0x200000);
    pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(0x800000);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(0x400000);
    pub const TASK_SHADER_EXT: Self = Self(0x80000);
    pub const MESH_SHADER_EXT: Self = Self(0x100000);
    pub const COMMAND_PREPROCESS_EXT: Self = Self(0x20000);
}
impl fmt::Display for PipelineStageFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TOP_OF_PIPE"),
            (2u64, "DRAW_INDIRECT"),
            (4u64, "VERTEX_INPUT"),
            (8u64, "VERTEX_SHADER"),
            (16u64, "TESSELLATION_CONTROL_SHADER"),
            (32u64, "TESSELLATION_EVALUATION_SHADER"),
            (64u64, "GEOMETRY_SHADER"),
            (128u64, "FRAGMENT_SHADER"),
            (256u64, "EARLY_FRAGMENT_TESTS"),
            (512u64, "LATE_FRAGMENT_TESTS"),
            (1024u64, "COLOR_ATTACHMENT_OUTPUT"),
            (2048u64, "COMPUTE_SHADER"),
            (4096u64, "TRANSFER"),
            (8192u64, "BOTTOM_OF_PIPE"),
            (16384u64, "HOST"),
            (32768u64, "ALL_GRAPHICS"),
            (65536u64, "ALL_COMMANDS"),
            (16777216u64, "TRANSFORM_FEEDBACK_EXT"),
            (262144u64, "CONDITIONAL_RENDERING_EXT"),
            (33554432u64, "ACCELERATION_STRUCTURE_BUILD_KHR"),
            (2097152u64, "RAY_TRACING_SHADER_KHR"),
            (8388608u64, "FRAGMENT_DENSITY_PROCESS_EXT"),
            (4194304u64, "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR"),
            (524288u64, "TASK_SHADER_EXT"),
            (1048576u64, "MESH_SHADER_EXT"),
            (131072u64, "COMMAND_PREPROCESS_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleCountFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SampleCountFlags(Flags);
bitflags!(SampleCountFlags, Flags);
impl SampleCountFlags {
    #[doc = "Sample count 1 supported"]
    pub const TYPE_1: Self = Self(0x1);
    #[doc = "Sample count 2 supported"]
    pub const TYPE_2: Self = Self(0x2);
    #[doc = "Sample count 4 supported"]
    pub const TYPE_4: Self = Self(0x4);
    #[doc = "Sample count 8 supported"]
    pub const TYPE_8: Self = Self(0x8);
    #[doc = "Sample count 16 supported"]
    pub const TYPE_16: Self = Self(0x10);
    #[doc = "Sample count 32 supported"]
    pub const TYPE_32: Self = Self(0x20);
    #[doc = "Sample count 64 supported"]
    pub const TYPE_64: Self = Self(0x40);
}
impl fmt::Display for SampleCountFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TYPE_1"),
            (2u64, "TYPE_2"),
            (4u64, "TYPE_4"),
            (8u64, "TYPE_8"),
            (16u64, "TYPE_16"),
            (32u64, "TYPE_32"),
            (64u64, "TYPE_64"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescriptionFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttachmentDescriptionFlags(Flags);
bitflags!(AttachmentDescriptionFlags, Flags);
impl AttachmentDescriptionFlags {
    #[doc = "The attachment may alias physical memory of another attachment in the same render pass"]
    pub const MAY_ALIAS: Self = Self(0x1);
    pub const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR: Self = Self(0x2);
    pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR: Self = Self(0x4);
}
impl fmt::Display for AttachmentDescriptionFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "MAY_ALIAS"),
            (2u64, "RESOLVE_SKIP_TRANSFER_FUNCTION_KHR"),
            (4u64, "RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilFaceFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StencilFaceFlags(Flags);
bitflags!(StencilFaceFlags, Flags);
impl StencilFaceFlags {
    #[doc = "Front face"]
    pub const FRONT: Self = Self(0x1);
    #[doc = "Back face"]
    pub const BACK: Self = Self(0x2);
    #[doc = "Front and back faces"]
    pub const FRONT_AND_BACK: Self = Self(0x3);
    #[deprecated = "aliased"]
    pub const STENCIL_FRONT_AND_BACK: Self = Self::FRONT_AND_BACK;
}
impl fmt::Display for StencilFaceFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "FRONT"), (2u64, "BACK"), (3u64, "FRONT_AND_BACK")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCullModeFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CullModeFlags(Flags);
bitflags!(CullModeFlags, Flags);
impl CullModeFlags {
    pub const NONE: Self = Self(0x0);
    pub const FRONT: Self = Self(0x1);
    pub const BACK: Self = Self(0x2);
    pub const FRONT_AND_BACK: Self = Self(0x3);
}
impl fmt::Display for CullModeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "FRONT"), (2u64, "BACK"), (3u64, "FRONT_AND_BACK")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorPoolCreateFlags(Flags);
bitflags!(DescriptorPoolCreateFlags, Flags);
impl DescriptorPoolCreateFlags {
    #[doc = "Descriptor sets may be freed individually"]
    pub const FREE_DESCRIPTOR_SET: Self = Self(0x1);
    pub const UPDATE_AFTER_BIND: Self = Self(0x2);
    pub const HOST_ONLY_EXT: Self = Self(0x4);
    pub const ALLOW_OVERALLOCATION_SETS_NV: Self = Self(0x8);
    pub const ALLOW_OVERALLOCATION_POOLS_NV: Self = Self(0x10);
}
impl fmt::Display for DescriptorPoolCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "FREE_DESCRIPTOR_SET"),
            (2u64, "UPDATE_AFTER_BIND"),
            (4u64, "HOST_ONLY_EXT"),
            (8u64, "ALLOW_OVERALLOCATION_SETS_NV"),
            (16u64, "ALLOW_OVERALLOCATION_POOLS_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolResetFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorPoolResetFlags(Flags);
bitflags!(DescriptorPoolResetFlags, Flags);
impl DescriptorPoolResetFlags {}
impl fmt::Display for DescriptorPoolResetFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDependencyFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyFlags(Flags);
bitflags!(DependencyFlags, Flags);
impl DependencyFlags {
    #[doc = "Dependency is per pixel region "]
    pub const BY_REGION: Self = Self(0x1);
    #[doc = "Dependency is across devices"]
    pub const DEVICE_GROUP: Self = Self(0x4);
    pub const VIEW_LOCAL: Self = Self(0x2);
    #[doc = "Dependency may be a feedback loop"]
    pub const FEEDBACK_LOOP_EXT: Self = Self(0x8);
    pub const QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_KHR: Self = Self(0x20);
    pub const ASYMMETRIC_EVENT_KHR: Self = Self(0x40);
}
impl fmt::Display for DependencyFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "BY_REGION"),
            (4u64, "DEVICE_GROUP"),
            (2u64, "VIEW_LOCAL"),
            (8u64, "FEEDBACK_LOOP_EXT"),
            (32u64, "QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_KHR"),
            (64u64, "ASYMMETRIC_EVENT_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubgroupFeatureFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubgroupFeatureFlags(Flags);
bitflags!(SubgroupFeatureFlags, Flags);
impl SubgroupFeatureFlags {
    #[doc = "Basic subgroup operations"]
    pub const BASIC: Self = Self(0x1);
    #[doc = "Vote subgroup operations"]
    pub const VOTE: Self = Self(0x2);
    #[doc = "Arithmetic subgroup operations"]
    pub const ARITHMETIC: Self = Self(0x4);
    #[doc = "Ballot subgroup operations"]
    pub const BALLOT: Self = Self(0x8);
    #[doc = "Shuffle subgroup operations"]
    pub const SHUFFLE: Self = Self(0x10);
    #[doc = "Shuffle relative subgroup operations"]
    pub const SHUFFLE_RELATIVE: Self = Self(0x20);
    #[doc = "Clustered subgroup operations"]
    pub const CLUSTERED: Self = Self(0x40);
    #[doc = "Quad subgroup operations"]
    pub const QUAD: Self = Self(0x80);
    pub const ROTATE: Self = Self(0x200);
    pub const ROTATE_CLUSTERED: Self = Self(0x400);
    pub const PARTITIONED_EXT: Self = Self(0x100);
}
impl fmt::Display for SubgroupFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "BASIC"),
            (2u64, "VOTE"),
            (4u64, "ARITHMETIC"),
            (8u64, "BALLOT"),
            (16u64, "SHUFFLE"),
            (32u64, "SHUFFLE_RELATIVE"),
            (64u64, "CLUSTERED"),
            (128u64, "QUAD"),
            (512u64, "ROTATE"),
            (1024u64, "ROTATE_CLUSTERED"),
            (256u64, "PARTITIONED_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutUsageFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsLayoutUsageFlagsNV(Flags);
bitflags!(IndirectCommandsLayoutUsageFlagsNV, Flags);
impl IndirectCommandsLayoutUsageFlagsNV {
    pub const EXPLICIT_PREPROCESS_NV: Self = Self(0x1);
    pub const INDEXED_SEQUENCES_NV: Self = Self(0x2);
    pub const UNORDERED_SEQUENCES_NV: Self = Self(0x4);
}
impl fmt::Display for IndirectCommandsLayoutUsageFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "EXPLICIT_PREPROCESS_NV"),
            (2u64, "INDEXED_SEQUENCES_NV"),
            (4u64, "UNORDERED_SEQUENCES_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectStateFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectStateFlagsNV(Flags);
bitflags!(IndirectStateFlagsNV, Flags);
impl IndirectStateFlagsNV {
    pub const FLAG_FRONTFACE_NV: Self = Self(0x1);
}
impl fmt::Display for IndirectStateFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "FLAG_FRONTFACE_NV")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeometryFlagsKHR(Flags);
bitflags!(GeometryFlagsKHR, Flags);
impl GeometryFlagsKHR {
    pub const OPAQUE_KHR: Self = Self(0x1);
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR: Self = Self(0x2);
}
impl fmt::Display for GeometryFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "OPAQUE_KHR"),
            (2u64, "NO_DUPLICATE_ANY_HIT_INVOCATION_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryInstanceFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeometryInstanceFlagsKHR(Flags);
bitflags!(GeometryInstanceFlagsKHR, Flags);
impl GeometryInstanceFlagsKHR {
    pub const TRIANGLE_FACING_CULL_DISABLE_KHR: Self = Self(0x1);
    pub const TRIANGLE_FLIP_FACING_KHR: Self = Self(0x2);
    pub const FORCE_OPAQUE_KHR: Self = Self(0x4);
    pub const FORCE_NO_OPAQUE_KHR: Self = Self(0x8);
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR: Self = Self::TRIANGLE_FLIP_FACING_KHR;
    pub const FORCE_OPACITY_MICROMAP_2_STATE_EXT: Self = Self(0x10);
    pub const DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(0x20);
}
impl fmt::Display for GeometryInstanceFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TRIANGLE_FACING_CULL_DISABLE_KHR"),
            (2u64, "TRIANGLE_FLIP_FACING_KHR"),
            (4u64, "FORCE_OPAQUE_KHR"),
            (8u64, "FORCE_NO_OPAQUE_KHR"),
            (16u64, "FORCE_OPACITY_MICROMAP_2_STATE_EXT"),
            (32u64, "DISABLE_OPACITY_MICROMAPS_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureGeometryFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClusterAccelerationStructureGeometryFlagsNV(Flags);
bitflags!(ClusterAccelerationStructureGeometryFlagsNV, Flags);
impl ClusterAccelerationStructureGeometryFlagsNV {
    pub const CULL_DISABLE_NV: Self = Self(0x1);
    pub const NO_DUPLICATE_ANYHIT_INVOCATION_NV: Self = Self(0x2);
    pub const OPAQUE_NV: Self = Self(0x4);
}
impl fmt::Display for ClusterAccelerationStructureGeometryFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "CULL_DISABLE_NV"),
            (2u64, "NO_DUPLICATE_ANYHIT_INVOCATION_NV"),
            (4u64, "OPAQUE_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureClusterFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClusterAccelerationStructureClusterFlagsNV(Flags);
bitflags!(ClusterAccelerationStructureClusterFlagsNV, Flags);
impl ClusterAccelerationStructureClusterFlagsNV {
    pub const ALLOW_DISABLE_OPACITY_MICROMAPS_NV: Self = Self(0x1);
}
impl fmt::Display for ClusterAccelerationStructureClusterFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "ALLOW_DISABLE_OPACITY_MICROMAPS_NV")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureAddressResolutionFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClusterAccelerationStructureAddressResolutionFlagsNV(Flags);
bitflags!(ClusterAccelerationStructureAddressResolutionFlagsNV, Flags);
impl ClusterAccelerationStructureAddressResolutionFlagsNV {
    pub const NONE_NV: Self = Self(0x0);
    pub const INDIRECTED_DST_IMPLICIT_DATA_NV: Self = Self(0x1);
    pub const INDIRECTED_SCRATCH_DATA_NV: Self = Self(0x2);
    pub const INDIRECTED_DST_ADDRESS_ARRAY_NV: Self = Self(0x4);
    pub const INDIRECTED_DST_SIZES_ARRAY_NV: Self = Self(0x8);
    pub const INDIRECTED_SRC_INFOS_ARRAY_NV: Self = Self(0x10);
    pub const INDIRECTED_SRC_INFOS_COUNT_NV: Self = Self(0x20);
}
impl fmt::Display for ClusterAccelerationStructureAddressResolutionFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "INDIRECTED_DST_IMPLICIT_DATA_NV"),
            (2u64, "INDIRECTED_SCRATCH_DATA_NV"),
            (4u64, "INDIRECTED_DST_ADDRESS_ARRAY_NV"),
            (8u64, "INDIRECTED_DST_SIZES_ARRAY_NV"),
            (16u64, "INDIRECTED_SRC_INFOS_ARRAY_NV"),
            (32u64, "INDIRECTED_SRC_INFOS_COUNT_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuildAccelerationStructureFlagsKHR(Flags);
bitflags!(BuildAccelerationStructureFlagsKHR, Flags);
impl BuildAccelerationStructureFlagsKHR {
    pub const ALLOW_UPDATE_KHR: Self = Self(0x1);
    pub const ALLOW_COMPACTION_KHR: Self = Self(0x2);
    pub const PREFER_FAST_TRACE_KHR: Self = Self(0x4);
    pub const PREFER_FAST_BUILD_KHR: Self = Self(0x8);
    pub const LOW_MEMORY_KHR: Self = Self(0x10);
    pub const MOTION_NV: Self = Self(0x20);
    pub const ALLOW_OPACITY_MICROMAP_UPDATE_EXT: Self = Self(0x40);
    pub const ALLOW_DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(0x80);
    pub const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT: Self = Self(0x100);
    pub const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV: Self = Self(0x200);
    pub const ALLOW_DATA_ACCESS_KHR: Self = Self(0x800);
    pub const ALLOW_CLUSTER_OPACITY_MICROMAPS_NV: Self = Self(0x1000);
}
impl fmt::Display for BuildAccelerationStructureFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "ALLOW_UPDATE_KHR"),
            (2u64, "ALLOW_COMPACTION_KHR"),
            (4u64, "PREFER_FAST_TRACE_KHR"),
            (8u64, "PREFER_FAST_BUILD_KHR"),
            (16u64, "LOW_MEMORY_KHR"),
            (32u64, "MOTION_NV"),
            (64u64, "ALLOW_OPACITY_MICROMAP_UPDATE_EXT"),
            (128u64, "ALLOW_DISABLE_OPACITY_MICROMAPS_EXT"),
            (256u64, "ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT"),
            (512u64, "ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV"),
            (2048u64, "ALLOW_DATA_ACCESS_KHR"),
            (4096u64, "ALLOW_CLUSTER_OPACITY_MICROMAPS_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrivateDataSlotCreateFlags(Flags);
bitflags!(PrivateDataSlotCreateFlags, Flags);
impl PrivateDataSlotCreateFlags {}
impl fmt::Display for PrivateDataSlotCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCreateFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccelerationStructureCreateFlagsKHR(Flags);
bitflags!(AccelerationStructureCreateFlagsKHR, Flags);
impl AccelerationStructureCreateFlagsKHR {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self(0x1);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(0x8);
    pub const MOTION_NV: Self = Self(0x4);
}
impl fmt::Display for AccelerationStructureCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "DEVICE_ADDRESS_CAPTURE_REPLAY_KHR"),
            (8u64, "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT"),
            (4u64, "MOTION_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateCreateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorUpdateTemplateCreateFlags(Flags);
bitflags!(DescriptorUpdateTemplateCreateFlags, Flags);
impl DescriptorUpdateTemplateCreateFlags {}
impl fmt::Display for DescriptorUpdateTemplateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCreationFeedbackFlags(Flags);
bitflags!(PipelineCreationFeedbackFlags, Flags);
impl PipelineCreationFeedbackFlags {
    pub const VALID: Self = Self(0x1);
    pub const APPLICATION_PIPELINE_CACHE_HIT: Self = Self(0x2);
    pub const BASE_PIPELINE_ACCELERATION: Self = Self(0x4);
}
impl fmt::Display for PipelineCreationFeedbackFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "VALID"),
            (2u64, "APPLICATION_PIPELINE_CACHE_HIT"),
            (4u64, "BASE_PIPELINE_ACCELERATION"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceCounterDescriptionFlagsKHR(Flags);
bitflags!(PerformanceCounterDescriptionFlagsKHR, Flags);
impl PerformanceCounterDescriptionFlagsKHR {
    pub const PERFORMANCE_IMPACTING_KHR: Self = Self(0x1);
    pub const CONCURRENTLY_IMPACTED_KHR: Self = Self(0x2);
}
impl fmt::Display for PerformanceCounterDescriptionFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "PERFORMANCE_IMPACTING_KHR"),
            (2u64, "CONCURRENTLY_IMPACTED_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAcquireProfilingLockFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AcquireProfilingLockFlagsKHR(Flags);
bitflags!(AcquireProfilingLockFlagsKHR, Flags);
impl AcquireProfilingLockFlagsKHR {}
impl fmt::Display for AcquireProfilingLockFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreWaitFlags(Flags);
bitflags!(SemaphoreWaitFlags, Flags);
impl SemaphoreWaitFlags {
    pub const ANY: Self = Self(0x1);
}
impl fmt::Display for SemaphoreWaitFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "ANY")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCompilerControlFlagsAMD.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCompilerControlFlagsAMD(Flags);
bitflags!(PipelineCompilerControlFlagsAMD, Flags);
impl PipelineCompilerControlFlagsAMD {}
impl fmt::Display for PipelineCompilerControlFlagsAMD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCorePropertiesFlagsAMD.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderCorePropertiesFlagsAMD(Flags);
bitflags!(ShaderCorePropertiesFlagsAMD, Flags);
impl ShaderCorePropertiesFlagsAMD {}
impl fmt::Display for ShaderCorePropertiesFlagsAMD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceDiagnosticsConfigFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceDiagnosticsConfigFlagsNV(Flags);
bitflags!(DeviceDiagnosticsConfigFlagsNV, Flags);
impl DeviceDiagnosticsConfigFlagsNV {
    pub const ENABLE_SHADER_DEBUG_INFO_NV: Self = Self(0x1);
    pub const ENABLE_RESOURCE_TRACKING_NV: Self = Self(0x2);
    pub const ENABLE_AUTOMATIC_CHECKPOINTS_NV: Self = Self(0x4);
    pub const ENABLE_SHADER_ERROR_REPORTING_NV: Self = Self(0x8);
}
impl fmt::Display for DeviceDiagnosticsConfigFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "ENABLE_SHADER_DEBUG_INFO_NV"),
            (2u64, "ENABLE_RESOURCE_TRACKING_NV"),
            (4u64, "ENABLE_AUTOMATIC_CHECKPOINTS_NV"),
            (8u64, "ENABLE_SHADER_ERROR_REPORTING_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRefreshObjectFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefreshObjectFlagsKHR(Flags);
bitflags!(RefreshObjectFlagsKHR, Flags);
impl RefreshObjectFlagsKHR {}
impl fmt::Display for RefreshObjectFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlags2.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccessFlags2(Flags64);
bitflags!(AccessFlags2, Flags64);
impl AccessFlags2 {
    pub const NONE: Self = Self(0x0);
    pub const INDIRECT_COMMAND_READ: Self = Self(0x1);
    pub const INDEX_READ: Self = Self(0x2);
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(0x4);
    pub const UNIFORM_READ: Self = Self(0x8);
    pub const INPUT_ATTACHMENT_READ: Self = Self(0x10);
    pub const SHADER_READ: Self = Self(0x20);
    pub const SHADER_WRITE: Self = Self(0x40);
    pub const COLOR_ATTACHMENT_READ: Self = Self(0x80);
    pub const COLOR_ATTACHMENT_WRITE: Self = Self(0x100);
    pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(0x200);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(0x400);
    pub const TRANSFER_READ: Self = Self(0x800);
    pub const TRANSFER_WRITE: Self = Self(0x1000);
    pub const HOST_READ: Self = Self(0x2000);
    pub const HOST_WRITE: Self = Self(0x4000);
    pub const MEMORY_READ: Self = Self(0x8000);
    pub const MEMORY_WRITE: Self = Self(0x10000);
    pub const SHADER_SAMPLED_READ: Self = Self(0x100000000);
    pub const SHADER_STORAGE_READ: Self = Self(0x200000000);
    pub const SHADER_STORAGE_WRITE: Self = Self(0x400000000);
    pub const VIDEO_DECODE_READ_KHR: Self = Self(0x800000000);
    pub const VIDEO_DECODE_WRITE_KHR: Self = Self(0x1000000000);
    pub const SAMPLER_HEAP_READ_EXT: Self = Self(0x200000000000000);
    pub const RESOURCE_HEAP_READ_EXT: Self = Self(0x400000000000000);
    pub const VIDEO_ENCODE_READ_KHR: Self = Self(0x2000000000);
    pub const VIDEO_ENCODE_WRITE_KHR: Self = Self(0x4000000000);
    pub const SHADER_TILE_ATTACHMENT_READ_QCOM: Self = Self(0x8000000000000);
    pub const SHADER_TILE_ATTACHMENT_WRITE_QCOM: Self = Self(0x10000000000000);
    pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(0x2000000);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(0x4000000);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self = Self(0x8000000);
    #[doc = "read access flag for reading conditional rendering predicate"]
    pub const CONDITIONAL_RENDERING_READ_EXT: Self = Self(0x100000);
    pub const COMMAND_PREPROCESS_READ_EXT: Self = Self(0x20000);
    pub const COMMAND_PREPROCESS_WRITE_EXT: Self = Self(0x40000);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self = Self(0x800000);
    pub const ACCELERATION_STRUCTURE_READ_KHR: Self = Self(0x200000);
    pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(0x400000);
    pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(0x1000000);
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(0x80000);
    pub const DESCRIPTOR_BUFFER_READ_EXT: Self = Self(0x20000000000);
    pub const INVOCATION_MASK_READ_HUAWEI: Self = Self(0x8000000000);
    pub const SHADER_BINDING_TABLE_READ_KHR: Self = Self(0x10000000000);
    pub const MICROMAP_READ_EXT: Self = Self(0x100000000000);
    pub const MICROMAP_WRITE_EXT: Self = Self(0x200000000000);
    pub const OPTICAL_FLOW_READ_NV: Self = Self(0x40000000000);
    pub const OPTICAL_FLOW_WRITE_NV: Self = Self(0x80000000000);
    pub const DATA_GRAPH_READ_ARM: Self = Self(0x800000000000);
    pub const DATA_GRAPH_WRITE_ARM: Self = Self(0x1000000000000);
    pub const MEMORY_DECOMPRESSION_READ_EXT: Self = Self(0x80000000000000);
    pub const MEMORY_DECOMPRESSION_WRITE_EXT: Self = Self(0x100000000000000);
}
impl fmt::Display for AccessFlags2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "INDIRECT_COMMAND_READ"),
            (2u64, "INDEX_READ"),
            (4u64, "VERTEX_ATTRIBUTE_READ"),
            (8u64, "UNIFORM_READ"),
            (16u64, "INPUT_ATTACHMENT_READ"),
            (32u64, "SHADER_READ"),
            (64u64, "SHADER_WRITE"),
            (128u64, "COLOR_ATTACHMENT_READ"),
            (256u64, "COLOR_ATTACHMENT_WRITE"),
            (512u64, "DEPTH_STENCIL_ATTACHMENT_READ"),
            (1024u64, "DEPTH_STENCIL_ATTACHMENT_WRITE"),
            (2048u64, "TRANSFER_READ"),
            (4096u64, "TRANSFER_WRITE"),
            (8192u64, "HOST_READ"),
            (16384u64, "HOST_WRITE"),
            (32768u64, "MEMORY_READ"),
            (65536u64, "MEMORY_WRITE"),
            (4294967296u64, "SHADER_SAMPLED_READ"),
            (8589934592u64, "SHADER_STORAGE_READ"),
            (17179869184u64, "SHADER_STORAGE_WRITE"),
            (34359738368u64, "VIDEO_DECODE_READ_KHR"),
            (68719476736u64, "VIDEO_DECODE_WRITE_KHR"),
            (144115188075855872u64, "SAMPLER_HEAP_READ_EXT"),
            (288230376151711744u64, "RESOURCE_HEAP_READ_EXT"),
            (137438953472u64, "VIDEO_ENCODE_READ_KHR"),
            (274877906944u64, "VIDEO_ENCODE_WRITE_KHR"),
            (2251799813685248u64, "SHADER_TILE_ATTACHMENT_READ_QCOM"),
            (4503599627370496u64, "SHADER_TILE_ATTACHMENT_WRITE_QCOM"),
            (33554432u64, "TRANSFORM_FEEDBACK_WRITE_EXT"),
            (67108864u64, "TRANSFORM_FEEDBACK_COUNTER_READ_EXT"),
            (134217728u64, "TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT"),
            (1048576u64, "CONDITIONAL_RENDERING_READ_EXT"),
            (131072u64, "COMMAND_PREPROCESS_READ_EXT"),
            (262144u64, "COMMAND_PREPROCESS_WRITE_EXT"),
            (8388608u64, "FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR"),
            (2097152u64, "ACCELERATION_STRUCTURE_READ_KHR"),
            (4194304u64, "ACCELERATION_STRUCTURE_WRITE_KHR"),
            (16777216u64, "FRAGMENT_DENSITY_MAP_READ_EXT"),
            (524288u64, "COLOR_ATTACHMENT_READ_NONCOHERENT_EXT"),
            (2199023255552u64, "DESCRIPTOR_BUFFER_READ_EXT"),
            (549755813888u64, "INVOCATION_MASK_READ_HUAWEI"),
            (1099511627776u64, "SHADER_BINDING_TABLE_READ_KHR"),
            (17592186044416u64, "MICROMAP_READ_EXT"),
            (35184372088832u64, "MICROMAP_WRITE_EXT"),
            (4398046511104u64, "OPTICAL_FLOW_READ_NV"),
            (8796093022208u64, "OPTICAL_FLOW_WRITE_NV"),
            (140737488355328u64, "DATA_GRAPH_READ_ARM"),
            (281474976710656u64, "DATA_GRAPH_WRITE_ARM"),
            (36028797018963968u64, "MEMORY_DECOMPRESSION_READ_EXT"),
            (72057594037927936u64, "MEMORY_DECOMPRESSION_WRITE_EXT"),
        ];
        flag_display(self.0, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlags2.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineStageFlags2(Flags64);
bitflags!(PipelineStageFlags2, Flags64);
impl PipelineStageFlags2 {
    pub const NONE: Self = Self(0x0);
    pub const TOP_OF_PIPE: Self = Self(0x1);
    pub const DRAW_INDIRECT: Self = Self(0x2);
    pub const VERTEX_INPUT: Self = Self(0x4);
    pub const VERTEX_SHADER: Self = Self(0x8);
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(0x10);
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(0x20);
    pub const GEOMETRY_SHADER: Self = Self(0x40);
    pub const FRAGMENT_SHADER: Self = Self(0x80);
    pub const EARLY_FRAGMENT_TESTS: Self = Self(0x100);
    pub const LATE_FRAGMENT_TESTS: Self = Self(0x200);
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(0x400);
    pub const COMPUTE_SHADER: Self = Self(0x800);
    pub const ALL_TRANSFER: Self = Self(0x1000);
    pub const TRANSFER: Self = Self::ALL_TRANSFER;
    pub const BOTTOM_OF_PIPE: Self = Self(0x2000);
    pub const HOST: Self = Self(0x4000);
    pub const ALL_GRAPHICS: Self = Self(0x8000);
    pub const ALL_COMMANDS: Self = Self(0x10000);
    pub const COPY: Self = Self(0x100000000);
    pub const RESOLVE: Self = Self(0x200000000);
    pub const BLIT: Self = Self(0x400000000);
    pub const CLEAR: Self = Self(0x800000000);
    pub const INDEX_INPUT: Self = Self(0x1000000000);
    pub const VERTEX_ATTRIBUTE_INPUT: Self = Self(0x2000000000);
    pub const PRE_RASTERIZATION_SHADERS: Self = Self(0x4000000000);
    pub const VIDEO_DECODE_KHR: Self = Self(0x4000000);
    pub const VIDEO_ENCODE_KHR: Self = Self(0x8000000);
    pub const TRANSFORM_FEEDBACK_EXT: Self = Self(0x1000000);
    #[doc = "A pipeline stage for conditional rendering predicate fetch"]
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(0x40000);
    pub const COMMAND_PREPROCESS_EXT: Self = Self(0x20000);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(0x400000);
    pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(0x2000000);
    pub const RAY_TRACING_SHADER_KHR: Self = Self(0x200000);
    pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(0x800000);
    pub const TASK_SHADER_EXT: Self = Self(0x80000);
    pub const MESH_SHADER_EXT: Self = Self(0x100000);
    pub const SUBPASS_SHADER_HUAWEI: Self = Self(0x8000000000);
    pub const INVOCATION_MASK_HUAWEI: Self = Self(0x10000000000);
    pub const ACCELERATION_STRUCTURE_COPY_KHR: Self = Self(0x10000000);
    pub const MICROMAP_BUILD_EXT: Self = Self(0x40000000);
    pub const CLUSTER_CULLING_SHADER_HUAWEI: Self = Self(0x20000000000);
    pub const OPTICAL_FLOW_NV: Self = Self(0x20000000);
    pub const CONVERT_COOPERATIVE_VECTOR_MATRIX_NV: Self = Self(0x100000000000);
    pub const DATA_GRAPH_ARM: Self = Self(0x40000000000);
    pub const COPY_INDIRECT_KHR: Self = Self(0x400000000000);
    pub const MEMORY_DECOMPRESSION_EXT: Self = Self(0x200000000000);
}
impl fmt::Display for PipelineStageFlags2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TOP_OF_PIPE"),
            (2u64, "DRAW_INDIRECT"),
            (4u64, "VERTEX_INPUT"),
            (8u64, "VERTEX_SHADER"),
            (16u64, "TESSELLATION_CONTROL_SHADER"),
            (32u64, "TESSELLATION_EVALUATION_SHADER"),
            (64u64, "GEOMETRY_SHADER"),
            (128u64, "FRAGMENT_SHADER"),
            (256u64, "EARLY_FRAGMENT_TESTS"),
            (512u64, "LATE_FRAGMENT_TESTS"),
            (1024u64, "COLOR_ATTACHMENT_OUTPUT"),
            (2048u64, "COMPUTE_SHADER"),
            (4096u64, "ALL_TRANSFER"),
            (8192u64, "BOTTOM_OF_PIPE"),
            (16384u64, "HOST"),
            (32768u64, "ALL_GRAPHICS"),
            (65536u64, "ALL_COMMANDS"),
            (4294967296u64, "COPY"),
            (8589934592u64, "RESOLVE"),
            (17179869184u64, "BLIT"),
            (34359738368u64, "CLEAR"),
            (68719476736u64, "INDEX_INPUT"),
            (137438953472u64, "VERTEX_ATTRIBUTE_INPUT"),
            (274877906944u64, "PRE_RASTERIZATION_SHADERS"),
            (67108864u64, "VIDEO_DECODE_KHR"),
            (134217728u64, "VIDEO_ENCODE_KHR"),
            (16777216u64, "TRANSFORM_FEEDBACK_EXT"),
            (262144u64, "CONDITIONAL_RENDERING_EXT"),
            (131072u64, "COMMAND_PREPROCESS_EXT"),
            (4194304u64, "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR"),
            (33554432u64, "ACCELERATION_STRUCTURE_BUILD_KHR"),
            (2097152u64, "RAY_TRACING_SHADER_KHR"),
            (8388608u64, "FRAGMENT_DENSITY_PROCESS_EXT"),
            (524288u64, "TASK_SHADER_EXT"),
            (1048576u64, "MESH_SHADER_EXT"),
            (549755813888u64, "SUBPASS_SHADER_HUAWEI"),
            (1099511627776u64, "INVOCATION_MASK_HUAWEI"),
            (268435456u64, "ACCELERATION_STRUCTURE_COPY_KHR"),
            (1073741824u64, "MICROMAP_BUILD_EXT"),
            (2199023255552u64, "CLUSTER_CULLING_SHADER_HUAWEI"),
            (536870912u64, "OPTICAL_FLOW_NV"),
            (17592186044416u64, "CONVERT_COOPERATIVE_VECTOR_MATRIX_NV"),
            (4398046511104u64, "DATA_GRAPH_ARM"),
            (70368744177664u64, "COPY_INDIRECT_KHR"),
            (35184372088832u64, "MEMORY_DECOMPRESSION_EXT"),
        ];
        flag_display(self.0, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInfoFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccelerationStructureMotionInfoFlagsNV(Flags);
bitflags!(AccelerationStructureMotionInfoFlagsNV, Flags);
impl AccelerationStructureMotionInfoFlagsNV {}
impl fmt::Display for AccelerationStructureMotionInfoFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInstanceFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccelerationStructureMotionInstanceFlagsNV(Flags);
bitflags!(AccelerationStructureMotionInstanceFlagsNV, Flags);
impl AccelerationStructureMotionInstanceFlagsNV {}
impl fmt::Display for AccelerationStructureMotionInstanceFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlags2.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FormatFeatureFlags2(Flags64);
bitflags!(FormatFeatureFlags2, Flags64);
impl FormatFeatureFlags2 {
    pub const SAMPLED_IMAGE: Self = Self(0x1);
    pub const STORAGE_IMAGE: Self = Self(0x2);
    pub const STORAGE_IMAGE_ATOMIC: Self = Self(0x4);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(0x8);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(0x10);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(0x20);
    pub const VERTEX_BUFFER: Self = Self(0x40);
    pub const COLOR_ATTACHMENT: Self = Self(0x80);
    pub const COLOR_ATTACHMENT_BLEND: Self = Self(0x100);
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(0x200);
    pub const BLIT_SRC: Self = Self(0x400);
    pub const BLIT_DST: Self = Self(0x800);
    pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(0x1000);
    pub const TRANSFER_SRC: Self = Self(0x4000);
    pub const TRANSFER_DST: Self = Self(0x8000);
    pub const SAMPLED_IMAGE_FILTER_MINMAX: Self = Self(0x10000);
    pub const MIDPOINT_CHROMA_SAMPLES: Self = Self(0x20000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(0x40000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self = Self(0x80000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self = Self(0x100000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self =
        Self(0x200000);
    pub const DISJOINT: Self = Self(0x400000);
    pub const COSITED_CHROMA_SAMPLES: Self = Self(0x800000);
    pub const STORAGE_READ_WITHOUT_FORMAT: Self = Self(0x80000000);
    pub const STORAGE_WRITE_WITHOUT_FORMAT: Self = Self(0x100000000);
    pub const SAMPLED_IMAGE_DEPTH_COMPARISON: Self = Self(0x200000000);
    #[doc = "This is an interaction with EXT_filter_cubic, though not tagged that way"]
    pub const SAMPLED_IMAGE_FILTER_CUBIC: Self = Self(0x2000);
    pub const HOST_IMAGE_TRANSFER: Self = Self(0x400000000000);
    pub const VIDEO_DECODE_OUTPUT_KHR: Self = Self(0x2000000);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(0x4000000);
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self = Self(0x20000000);
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(0x1000000);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(0x40000000);
    pub const VIDEO_ENCODE_INPUT_KHR: Self = Self(0x8000000);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(0x10000000);
    pub const ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV: Self = Self(0x8000000000000);
    #[doc = "Format support linear image as render target, it cannot be mixed with non linear attachment"]
    pub const LINEAR_COLOR_ATTACHMENT_NV: Self = Self(0x4000000000);
    pub const WEIGHT_IMAGE_QCOM: Self = Self(0x400000000);
    pub const WEIGHT_SAMPLED_IMAGE_QCOM: Self = Self(0x800000000);
    pub const BLOCK_MATCHING_QCOM: Self = Self(0x1000000000);
    pub const BOX_FILTER_SAMPLED_QCOM: Self = Self(0x2000000000);
    pub const TENSOR_SHADER_ARM: Self = Self(0x8000000000);
    pub const TENSOR_IMAGE_ALIASING_ARM: Self = Self(0x80000000000);
    pub const OPTICAL_FLOW_IMAGE_NV: Self = Self(0x10000000000);
    pub const OPTICAL_FLOW_VECTOR_NV: Self = Self(0x20000000000);
    pub const OPTICAL_FLOW_COST_NV: Self = Self(0x40000000000);
    pub const TENSOR_DATA_GRAPH_ARM: Self = Self(0x1000000000000);
    pub const COPY_IMAGE_INDIRECT_DST_KHR: Self = Self(0x800000000000000);
    pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(0x2000000000000);
    pub const VIDEO_ENCODE_EMPHASIS_MAP_KHR: Self = Self(0x4000000000000);
    pub const DEPTH_COPY_ON_COMPUTE_QUEUE_KHR: Self = Self(0x10000000000000);
    pub const DEPTH_COPY_ON_TRANSFER_QUEUE_KHR: Self = Self(0x20000000000000);
    pub const STENCIL_COPY_ON_COMPUTE_QUEUE_KHR: Self = Self(0x40000000000000);
    pub const STENCIL_COPY_ON_TRANSFER_QUEUE_KHR: Self = Self(0x80000000000000);
    pub const DATA_GRAPH_OPTICAL_FLOW_IMAGE_ARM: Self = Self(0x100000000000000);
    pub const DATA_GRAPH_OPTICAL_FLOW_VECTOR_ARM: Self = Self(0x200000000000000);
    pub const DATA_GRAPH_OPTICAL_FLOW_COST_ARM: Self = Self(0x400000000000000);
}
impl fmt::Display for FormatFeatureFlags2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "SAMPLED_IMAGE"),
            (2u64, "STORAGE_IMAGE"),
            (4u64, "STORAGE_IMAGE_ATOMIC"),
            (8u64, "UNIFORM_TEXEL_BUFFER"),
            (16u64, "STORAGE_TEXEL_BUFFER"),
            (32u64, "STORAGE_TEXEL_BUFFER_ATOMIC"),
            (64u64, "VERTEX_BUFFER"),
            (128u64, "COLOR_ATTACHMENT"),
            (256u64, "COLOR_ATTACHMENT_BLEND"),
            (512u64, "DEPTH_STENCIL_ATTACHMENT"),
            (1024u64, "BLIT_SRC"),
            (2048u64, "BLIT_DST"),
            (4096u64, "SAMPLED_IMAGE_FILTER_LINEAR"),
            (16384u64, "TRANSFER_SRC"),
            (32768u64, "TRANSFER_DST"),
            (65536u64, "SAMPLED_IMAGE_FILTER_MINMAX"),
            (131072u64, "MIDPOINT_CHROMA_SAMPLES"),
            (262144u64, "SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER"),
            (
                524288u64,
                "SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER",
            ),
            (
                1048576u64,
                "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT",
            ),
            (
                2097152u64,
                "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE",
            ),
            (4194304u64, "DISJOINT"),
            (8388608u64, "COSITED_CHROMA_SAMPLES"),
            (2147483648u64, "STORAGE_READ_WITHOUT_FORMAT"),
            (4294967296u64, "STORAGE_WRITE_WITHOUT_FORMAT"),
            (8589934592u64, "SAMPLED_IMAGE_DEPTH_COMPARISON"),
            (8192u64, "SAMPLED_IMAGE_FILTER_CUBIC"),
            (70368744177664u64, "HOST_IMAGE_TRANSFER"),
            (33554432u64, "VIDEO_DECODE_OUTPUT_KHR"),
            (67108864u64, "VIDEO_DECODE_DPB_KHR"),
            (536870912u64, "ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR"),
            (16777216u64, "FRAGMENT_DENSITY_MAP_EXT"),
            (1073741824u64, "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR"),
            (134217728u64, "VIDEO_ENCODE_INPUT_KHR"),
            (268435456u64, "VIDEO_ENCODE_DPB_KHR"),
            (
                2251799813685248u64,
                "ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV",
            ),
            (274877906944u64, "LINEAR_COLOR_ATTACHMENT_NV"),
            (17179869184u64, "WEIGHT_IMAGE_QCOM"),
            (34359738368u64, "WEIGHT_SAMPLED_IMAGE_QCOM"),
            (68719476736u64, "BLOCK_MATCHING_QCOM"),
            (137438953472u64, "BOX_FILTER_SAMPLED_QCOM"),
            (549755813888u64, "TENSOR_SHADER_ARM"),
            (8796093022208u64, "TENSOR_IMAGE_ALIASING_ARM"),
            (1099511627776u64, "OPTICAL_FLOW_IMAGE_NV"),
            (2199023255552u64, "OPTICAL_FLOW_VECTOR_NV"),
            (4398046511104u64, "OPTICAL_FLOW_COST_NV"),
            (281474976710656u64, "TENSOR_DATA_GRAPH_ARM"),
            (576460752303423488u64, "COPY_IMAGE_INDIRECT_DST_KHR"),
            (
                562949953421312u64,
                "VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR",
            ),
            (1125899906842624u64, "VIDEO_ENCODE_EMPHASIS_MAP_KHR"),
            (4503599627370496u64, "DEPTH_COPY_ON_COMPUTE_QUEUE_KHR"),
            (9007199254740992u64, "DEPTH_COPY_ON_TRANSFER_QUEUE_KHR"),
            (18014398509481984u64, "STENCIL_COPY_ON_COMPUTE_QUEUE_KHR"),
            (36028797018963968u64, "STENCIL_COPY_ON_TRANSFER_QUEUE_KHR"),
            (72057594037927936u64, "DATA_GRAPH_OPTICAL_FLOW_IMAGE_ARM"),
            (144115188075855872u64, "DATA_GRAPH_OPTICAL_FLOW_VECTOR_ARM"),
            (288230376151711744u64, "DATA_GRAPH_OPTICAL_FLOW_COST_ARM"),
        ];
        flag_display(self.0, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderingFlags(Flags);
bitflags!(RenderingFlags, Flags);
impl RenderingFlags {
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS: Self = Self(0x1);
    pub const SUSPENDING: Self = Self(0x2);
    pub const RESUMING: Self = Self(0x4);
    pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(0x8);
    #[doc = "Promoted from extension 452"]
    pub const CONTENTS_INLINE_KHR: Self = Self(0x10);
    pub const PER_LAYER_FRAGMENT_DENSITY_VALVE: Self = Self(0x20);
    pub const FRAGMENT_REGION_EXT: Self = Self(0x40);
    pub const CUSTOM_RESOLVE_EXT: Self = Self(0x80);
    pub const LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR: Self = Self(0x100);
}
impl fmt::Display for RenderingFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "CONTENTS_SECONDARY_COMMAND_BUFFERS"),
            (2u64, "SUSPENDING"),
            (4u64, "RESUMING"),
            (8u64, "ENABLE_LEGACY_DITHERING_EXT"),
            (16u64, "CONTENTS_INLINE_KHR"),
            (32u64, "PER_LAYER_FRAGMENT_DENSITY_VALVE"),
            (64u64, "FRAGMENT_REGION_EXT"),
            (128u64, "CUSTOM_RESOLVE_EXT"),
            (256u64, "LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDecompressionMethodFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryDecompressionMethodFlagsEXT(Flags64);
bitflags!(MemoryDecompressionMethodFlagsEXT, Flags64);
impl MemoryDecompressionMethodFlagsEXT {
    pub const GDEFLATE_1_0_EXT: Self = Self(0x1);
    pub const GDEFLATE_1_0_NV: Self = Self::GDEFLATE_1_0_EXT;
}
impl fmt::Display for MemoryDecompressionMethodFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "GDEFLATE_1_0_EXT")];
        flag_display(self.0, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceFaultFlagsKHR(Flags);
bitflags!(DeviceFaultFlagsKHR, Flags);
impl DeviceFaultFlagsKHR {
    pub const FLAG_DEVICE_LOST_KHR: Self = Self(0x1);
    pub const FLAG_MEMORY_ADDRESS_KHR: Self = Self(0x2);
    pub const FLAG_INSTRUCTION_ADDRESS_KHR: Self = Self(0x4);
    pub const FLAG_VENDOR_KHR: Self = Self(0x8);
    pub const FLAG_WATCHDOG_TIMEOUT_KHR: Self = Self(0x10);
    pub const FLAG_OVERFLOW_KHR: Self = Self(0x20);
}
impl fmt::Display for DeviceFaultFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "FLAG_DEVICE_LOST_KHR"),
            (2u64, "FLAG_MEMORY_ADDRESS_KHR"),
            (4u64, "FLAG_INSTRUCTION_ADDRESS_KHR"),
            (8u64, "FLAG_VENDOR_KHR"),
            (16u64, "FLAG_WATCHDOG_TIMEOUT_KHR"),
            (32u64, "FLAG_OVERFLOW_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildMicromapFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuildMicromapFlagsEXT(Flags);
bitflags!(BuildMicromapFlagsEXT, Flags);
impl BuildMicromapFlagsEXT {
    pub const PREFER_FAST_TRACE_EXT: Self = Self(0x1);
    pub const PREFER_FAST_BUILD_EXT: Self = Self(0x2);
    pub const ALLOW_COMPACTION_EXT: Self = Self(0x4);
}
impl fmt::Display for BuildMicromapFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "PREFER_FAST_TRACE_EXT"),
            (2u64, "PREFER_FAST_BUILD_EXT"),
            (4u64, "ALLOW_COMPACTION_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapCreateFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MicromapCreateFlagsEXT(Flags);
bitflags!(MicromapCreateFlagsEXT, Flags);
impl MicromapCreateFlagsEXT {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT: Self = Self(0x1);
}
impl fmt::Display for MicromapCreateFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "DEVICE_ADDRESS_CAPTURE_REPLAY_EXT")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutUsageFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsLayoutUsageFlagsEXT(Flags);
bitflags!(IndirectCommandsLayoutUsageFlagsEXT, Flags);
impl IndirectCommandsLayoutUsageFlagsEXT {
    pub const EXPLICIT_PREPROCESS_EXT: Self = Self(0x1);
    pub const UNORDERED_SEQUENCES_EXT: Self = Self(0x2);
}
impl fmt::Display for IndirectCommandsLayoutUsageFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "EXPLICIT_PREPROCESS_EXT"),
            (2u64, "UNORDERED_SEQUENCES_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsInputModeFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsInputModeFlagsEXT(Flags);
bitflags!(IndirectCommandsInputModeFlagsEXT, Flags);
impl IndirectCommandsInputModeFlagsEXT {
    pub const VULKAN_INDEX_BUFFER_EXT: Self = Self(0x1);
    pub const DXGI_INDEX_BUFFER_EXT: Self = Self(0x2);
}
impl fmt::Display for IndirectCommandsInputModeFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "VULKAN_INDEX_BUFFER_EXT"),
            (2u64, "DXGI_INDEX_BUFFER_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectDriverLoadingFlagsLUNARG.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirectDriverLoadingFlagsLUNARG(Flags);
bitflags!(DirectDriverLoadingFlagsLUNARG, Flags);
impl DirectDriverLoadingFlagsLUNARG {}
impl fmt::Display for DirectDriverLoadingFlagsLUNARG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlags2.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCreateFlags2(Flags64);
bitflags!(PipelineCreateFlags2, Flags64);
impl PipelineCreateFlags2 {
    pub const DISABLE_OPTIMIZATION: Self = Self(0x1);
    pub const ALLOW_DERIVATIVES: Self = Self(0x2);
    pub const DERIVATIVE: Self = Self(0x4);
    pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(0x8);
    pub const DISPATCH_BASE: Self = Self(0x10);
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(0x100);
    pub const EARLY_RETURN_ON_FAILURE: Self = Self(0x200);
    pub const NO_PROTECTED_ACCESS: Self = Self(0x8000000);
    pub const PROTECTED_ACCESS_ONLY: Self = Self(0x40000000);
    pub const EXECUTION_GRAPH_AMDX: Self = Self(0x100000000);
    pub const DESCRIPTOR_HEAP_EXT: Self = Self(0x1000000000);
    pub const RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV: Self = Self(0x200000000);
    pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(0x400000000);
    pub const DEFER_COMPILE_NV: Self = Self(0x20);
    pub const CAPTURE_STATISTICS_KHR: Self = Self(0x40);
    pub const CAPTURE_INTERNAL_REPRESENTATIONS_KHR: Self = Self(0x80);
    pub const LINK_TIME_OPTIMIZATION_EXT: Self = Self(0x400);
    pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT: Self = Self(0x800000);
    pub const LIBRARY_KHR: Self = Self(0x800);
    pub const RAY_TRACING_SKIP_TRIANGLES_KHR: Self = Self(0x1000);
    pub const RAY_TRACING_SKIP_AABBS_KHR: Self = Self(0x2000);
    pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR: Self = Self(0x4000);
    pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR: Self = Self(0x8000);
    pub const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR: Self = Self(0x10000);
    pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR: Self = Self(0x20000);
    pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR: Self = Self(0x80000);
    pub const INDIRECT_BINDABLE_NV: Self = Self(0x40000);
    pub const RAY_TRACING_ALLOW_MOTION_NV: Self = Self(0x100000);
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(0x200000);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self = Self(0x400000);
    pub const RAY_TRACING_OPACITY_MICROMAP_EXT: Self = Self(0x1000000);
    pub const COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(0x2000000);
    pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(0x4000000);
    pub const RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Self = Self(0x10000000);
    pub const DESCRIPTOR_BUFFER_EXT: Self = Self(0x20000000);
    pub const DISALLOW_OPACITY_MICROMAP_ARM: Self = Self(0x2000000000);
    pub const INSTRUMENT_SHADERS_ARM: Self = Self(0x8000000000);
    pub const CAPTURE_DATA_KHR: Self = Self(0x80000000);
    pub const INDIRECT_BINDABLE_EXT: Self = Self(0x4000000000);
    pub const PER_LAYER_FRAGMENT_DENSITY_VALVE: Self = Self(0x10000000000);
    pub const TYPE_64_INDEXING_EXT: Self = Self(0x80000000000);
}
impl fmt::Display for PipelineCreateFlags2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "DISABLE_OPTIMIZATION"),
            (2u64, "ALLOW_DERIVATIVES"),
            (4u64, "DERIVATIVE"),
            (8u64, "VIEW_INDEX_FROM_DEVICE_INDEX"),
            (16u64, "DISPATCH_BASE"),
            (256u64, "FAIL_ON_PIPELINE_COMPILE_REQUIRED"),
            (512u64, "EARLY_RETURN_ON_FAILURE"),
            (134217728u64, "NO_PROTECTED_ACCESS"),
            (1073741824u64, "PROTECTED_ACCESS_ONLY"),
            (4294967296u64, "EXECUTION_GRAPH_AMDX"),
            (68719476736u64, "DESCRIPTOR_HEAP_EXT"),
            (
                8589934592u64,
                "RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV",
            ),
            (17179869184u64, "ENABLE_LEGACY_DITHERING_EXT"),
            (32u64, "DEFER_COMPILE_NV"),
            (64u64, "CAPTURE_STATISTICS_KHR"),
            (128u64, "CAPTURE_INTERNAL_REPRESENTATIONS_KHR"),
            (1024u64, "LINK_TIME_OPTIMIZATION_EXT"),
            (8388608u64, "RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT"),
            (2048u64, "LIBRARY_KHR"),
            (4096u64, "RAY_TRACING_SKIP_TRIANGLES_KHR"),
            (8192u64, "RAY_TRACING_SKIP_AABBS_KHR"),
            (16384u64, "RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR"),
            (32768u64, "RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR"),
            (65536u64, "RAY_TRACING_NO_NULL_MISS_SHADERS_KHR"),
            (131072u64, "RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR"),
            (
                524288u64,
                "RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR",
            ),
            (262144u64, "INDIRECT_BINDABLE_NV"),
            (1048576u64, "RAY_TRACING_ALLOW_MOTION_NV"),
            (2097152u64, "RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR"),
            (4194304u64, "RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT"),
            (16777216u64, "RAY_TRACING_OPACITY_MICROMAP_EXT"),
            (33554432u64, "COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT"),
            (67108864u64, "DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT"),
            (268435456u64, "RAY_TRACING_DISPLACEMENT_MICROMAP_NV"),
            (536870912u64, "DESCRIPTOR_BUFFER_EXT"),
            (137438953472u64, "DISALLOW_OPACITY_MICROMAP_ARM"),
            (549755813888u64, "INSTRUMENT_SHADERS_ARM"),
            (2147483648u64, "CAPTURE_DATA_KHR"),
            (274877906944u64, "INDIRECT_BINDABLE_EXT"),
            (1099511627776u64, "PER_LAYER_FRAGMENT_DENSITY_VALVE"),
            (8796093022208u64, "TYPE_64_INDEXING_EXT"),
        ];
        flag_display(self.0, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlags2.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferUsageFlags2(Flags64);
bitflags!(BufferUsageFlags2, Flags64);
impl BufferUsageFlags2 {
    pub const TRANSFER_SRC: Self = Self(0x1);
    pub const TRANSFER_DST: Self = Self(0x2);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(0x4);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(0x8);
    pub const UNIFORM_BUFFER: Self = Self(0x10);
    pub const STORAGE_BUFFER: Self = Self(0x20);
    pub const INDEX_BUFFER: Self = Self(0x40);
    pub const VERTEX_BUFFER: Self = Self(0x80);
    pub const INDIRECT_BUFFER: Self = Self(0x100);
    pub const SHADER_DEVICE_ADDRESS: Self = Self(0x20000);
    pub const EXECUTION_GRAPH_SCRATCH_AMDX: Self = Self(0x2000000);
    pub const DESCRIPTOR_HEAP_EXT: Self = Self(0x10000000);
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(0x200);
    pub const SHADER_BINDING_TABLE_KHR: Self = Self(0x400);
    pub const TRANSFORM_FEEDBACK_BUFFER_EXT: Self = Self(0x800);
    pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT: Self = Self(0x1000);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(0x2000);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(0x4000);
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(0x8000);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(0x10000);
    pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR: Self = Self(0x80000);
    pub const ACCELERATION_STRUCTURE_STORAGE_KHR: Self = Self(0x100000);
    pub const SAMPLER_DESCRIPTOR_BUFFER_EXT: Self = Self(0x200000);
    pub const RESOURCE_DESCRIPTOR_BUFFER_EXT: Self = Self(0x400000);
    pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT: Self = Self(0x4000000);
    pub const MICROMAP_BUILD_INPUT_READ_ONLY_EXT: Self = Self(0x800000);
    pub const MICROMAP_STORAGE_EXT: Self = Self(0x1000000);
    pub const COMPRESSED_DATA_DGF1_AMDX: Self = Self(0x200000000);
    pub const DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM: Self = Self(0x20000000);
    pub const TILE_MEMORY_QCOM: Self = Self(0x8000000);
    pub const MEMORY_DECOMPRESSION_EXT: Self = Self(0x100000000);
    pub const PREPROCESS_BUFFER_EXT: Self = Self(0x80000000);
}
impl fmt::Display for BufferUsageFlags2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TRANSFER_SRC"),
            (2u64, "TRANSFER_DST"),
            (4u64, "UNIFORM_TEXEL_BUFFER"),
            (8u64, "STORAGE_TEXEL_BUFFER"),
            (16u64, "UNIFORM_BUFFER"),
            (32u64, "STORAGE_BUFFER"),
            (64u64, "INDEX_BUFFER"),
            (128u64, "VERTEX_BUFFER"),
            (256u64, "INDIRECT_BUFFER"),
            (131072u64, "SHADER_DEVICE_ADDRESS"),
            (33554432u64, "EXECUTION_GRAPH_SCRATCH_AMDX"),
            (268435456u64, "DESCRIPTOR_HEAP_EXT"),
            (512u64, "CONDITIONAL_RENDERING_EXT"),
            (1024u64, "SHADER_BINDING_TABLE_KHR"),
            (2048u64, "TRANSFORM_FEEDBACK_BUFFER_EXT"),
            (4096u64, "TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT"),
            (8192u64, "VIDEO_DECODE_SRC_KHR"),
            (16384u64, "VIDEO_DECODE_DST_KHR"),
            (32768u64, "VIDEO_ENCODE_DST_KHR"),
            (65536u64, "VIDEO_ENCODE_SRC_KHR"),
            (
                524288u64,
                "ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR",
            ),
            (1048576u64, "ACCELERATION_STRUCTURE_STORAGE_KHR"),
            (2097152u64, "SAMPLER_DESCRIPTOR_BUFFER_EXT"),
            (4194304u64, "RESOURCE_DESCRIPTOR_BUFFER_EXT"),
            (67108864u64, "PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT"),
            (8388608u64, "MICROMAP_BUILD_INPUT_READ_ONLY_EXT"),
            (16777216u64, "MICROMAP_STORAGE_EXT"),
            (8589934592u64, "COMPRESSED_DATA_DGF1_AMDX"),
            (536870912u64, "DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM"),
            (134217728u64, "TILE_MEMORY_QCOM"),
            (4294967296u64, "MEMORY_DECOMPRESSION_EXT"),
            (2147483648u64, "PREPROCESS_BUFFER_EXT"),
        ];
        flag_display(self.0, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAddressCopyFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddressCopyFlagsKHR(Flags);
bitflags!(AddressCopyFlagsKHR, Flags);
impl AddressCopyFlagsKHR {
    pub const DEVICE_LOCAL_KHR: Self = Self(0x1);
    pub const SPARSE_KHR: Self = Self(0x2);
    pub const PROTECTED_KHR: Self = Self(0x4);
}
impl fmt::Display for AddressCopyFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "DEVICE_LOCAL_KHR"),
            (2u64, "SPARSE_KHR"),
            (4u64, "PROTECTED_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorCreateFlagsARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorCreateFlagsARM(Flags64);
bitflags!(TensorCreateFlagsARM, Flags64);
impl TensorCreateFlagsARM {
    pub const MUTABLE_FORMAT_ARM: Self = Self(0x1);
    pub const PROTECTED_ARM: Self = Self(0x2);
    pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM: Self = Self(0x8);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM: Self = Self(0x4);
}
impl fmt::Display for TensorCreateFlagsARM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "MUTABLE_FORMAT_ARM"),
            (2u64, "PROTECTED_ARM"),
            (8u64, "DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM"),
            (4u64, "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM"),
        ];
        flag_display(self.0, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorUsageFlagsARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorUsageFlagsARM(Flags64);
bitflags!(TensorUsageFlagsARM, Flags64);
impl TensorUsageFlagsARM {
    #[doc = "Tensor written/read through shader descriptor"]
    pub const SHADER_ARM: Self = Self(0x2);
    #[doc = "Tensor can be src of a transfer operation"]
    pub const TRANSFER_SRC_ARM: Self = Self(0x4);
    #[doc = "Tensor can be dst of a transfer operation"]
    pub const TRANSFER_DST_ARM: Self = Self(0x8);
    #[doc = "Tensor can be aliased with an image"]
    pub const IMAGE_ALIASING_ARM: Self = Self(0x10);
    pub const DATA_GRAPH_ARM: Self = Self(0x20);
}
impl fmt::Display for TensorUsageFlagsARM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (2u64, "SHADER_ARM"),
            (4u64, "TRANSFER_SRC_ARM"),
            (8u64, "TRANSFER_DST_ARM"),
            (16u64, "IMAGE_ALIASING_ARM"),
            (32u64, "DATA_GRAPH_ARM"),
        ];
        flag_display(self.0, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewCreateFlagsARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorViewCreateFlagsARM(Flags64);
bitflags!(TensorViewCreateFlagsARM, Flags64);
impl TensorViewCreateFlagsARM {
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM: Self = Self(0x1);
}
impl fmt::Display for TensorViewCreateFlagsARM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM")];
        flag_display(self.0, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionCreateFlagsARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphPipelineSessionCreateFlagsARM(Flags64);
bitflags!(DataGraphPipelineSessionCreateFlagsARM, Flags64);
impl DataGraphPipelineSessionCreateFlagsARM {
    pub const PROTECTED_ARM: Self = Self(0x1);
    pub const OPTICAL_FLOW_CACHE_ARM: Self = Self(0x2);
}
impl fmt::Display for DataGraphPipelineSessionCreateFlagsARM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "PROTECTED_ARM"), (2u64, "OPTICAL_FLOW_CACHE_ARM")];
        flag_display(self.0, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineDispatchFlagsARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphPipelineDispatchFlagsARM(Flags64);
bitflags!(DataGraphPipelineDispatchFlagsARM, Flags64);
impl DataGraphPipelineDispatchFlagsARM {}
impl fmt::Display for DataGraphPipelineDispatchFlagsARM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbModelConversionFlagsVALVE.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeRgbModelConversionFlagsVALVE(Flags);
bitflags!(VideoEncodeRgbModelConversionFlagsVALVE, Flags);
impl VideoEncodeRgbModelConversionFlagsVALVE {
    pub const RGB_IDENTITY_VALVE: Self = Self(0x1);
    pub const YCBCR_IDENTITY_VALVE: Self = Self(0x2);
    pub const YCBCR_709_VALVE: Self = Self(0x4);
    pub const YCBCR_601_VALVE: Self = Self(0x8);
    pub const YCBCR_2020_VALVE: Self = Self(0x10);
}
impl fmt::Display for VideoEncodeRgbModelConversionFlagsVALVE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "RGB_IDENTITY_VALVE"),
            (2u64, "YCBCR_IDENTITY_VALVE"),
            (4u64, "YCBCR_709_VALVE"),
            (8u64, "YCBCR_601_VALVE"),
            (16u64, "YCBCR_2020_VALVE"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbRangeCompressionFlagsVALVE.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeRgbRangeCompressionFlagsVALVE(Flags);
bitflags!(VideoEncodeRgbRangeCompressionFlagsVALVE, Flags);
impl VideoEncodeRgbRangeCompressionFlagsVALVE {
    pub const FULL_RANGE_VALVE: Self = Self(0x1);
    pub const NARROW_RANGE_VALVE: Self = Self(0x2);
}
impl fmt::Display for VideoEncodeRgbRangeCompressionFlagsVALVE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "FULL_RANGE_VALVE"), (2u64, "NARROW_RANGE_VALVE")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbChromaOffsetFlagsVALVE.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeRgbChromaOffsetFlagsVALVE(Flags);
bitflags!(VideoEncodeRgbChromaOffsetFlagsVALVE, Flags);
impl VideoEncodeRgbChromaOffsetFlagsVALVE {
    pub const COSITED_EVEN_VALVE: Self = Self(0x1);
    pub const MIDPOINT_VALVE: Self = Self(0x2);
}
impl fmt::Display for VideoEncodeRgbChromaOffsetFlagsVALVE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "COSITED_EVEN_VALVE"), (2u64, "MIDPOINT_VALVE")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSpirvResourceTypeFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpirvResourceTypeFlagsEXT(Flags);
bitflags!(SpirvResourceTypeFlagsEXT, Flags);
impl SpirvResourceTypeFlagsEXT {
    pub const ALL_EXT: Self = Self(0x7fffffff);
    pub const SAMPLER_EXT: Self = Self(0x1);
    pub const SAMPLED_IMAGE_EXT: Self = Self(0x2);
    pub const READ_ONLY_IMAGE_EXT: Self = Self(0x4);
    pub const READ_WRITE_IMAGE_EXT: Self = Self(0x8);
    pub const COMBINED_SAMPLED_IMAGE_EXT: Self = Self(0x10);
    pub const UNIFORM_BUFFER_EXT: Self = Self(0x20);
    pub const READ_ONLY_STORAGE_BUFFER_EXT: Self = Self(0x40);
    pub const READ_WRITE_STORAGE_BUFFER_EXT: Self = Self(0x80);
    pub const ACCELERATION_STRUCTURE_EXT: Self = Self(0x100);
    pub const TENSOR_ARM: Self = Self(0x200);
}
impl fmt::Display for SpirvResourceTypeFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (2147483647u64, "ALL_EXT"),
            (1u64, "SAMPLER_EXT"),
            (2u64, "SAMPLED_IMAGE_EXT"),
            (4u64, "READ_ONLY_IMAGE_EXT"),
            (8u64, "READ_WRITE_IMAGE_EXT"),
            (16u64, "COMBINED_SAMPLED_IMAGE_EXT"),
            (32u64, "UNIFORM_BUFFER_EXT"),
            (64u64, "READ_ONLY_STORAGE_BUFFER_EXT"),
            (128u64, "READ_WRITE_STORAGE_BUFFER_EXT"),
            (256u64, "ACCELERATION_STRUCTURE_EXT"),
            (512u64, "TENSOR_ARM"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAddressCommandFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddressCommandFlagsKHR(Flags);
bitflags!(AddressCommandFlagsKHR, Flags);
impl AddressCommandFlagsKHR {
    pub const PROTECTED_KHR: Self = Self(0x1);
    pub const FULLY_BOUND_KHR: Self = Self(0x2);
    pub const STORAGE_BUFFER_USAGE_KHR: Self = Self(0x4);
    pub const UNKNOWN_STORAGE_BUFFER_USAGE_KHR: Self = Self(0x8);
    pub const TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR: Self = Self(0x10);
    pub const UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR: Self = Self(0x20);
}
impl fmt::Display for AddressCommandFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "PROTECTED_KHR"),
            (2u64, "FULLY_BOUND_KHR"),
            (4u64, "STORAGE_BUFFER_USAGE_KHR"),
            (8u64, "UNKNOWN_STORAGE_BUFFER_USAGE_KHR"),
            (16u64, "TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR"),
            (32u64, "UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCompositeAlphaFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompositeAlphaFlagsKHR(Flags);
bitflags!(CompositeAlphaFlagsKHR, Flags);
impl CompositeAlphaFlagsKHR {
    pub const OPAQUE_KHR: Self = Self(0x1);
    pub const PRE_MULTIPLIED_KHR: Self = Self(0x2);
    pub const POST_MULTIPLIED_KHR: Self = Self(0x4);
    pub const INHERIT_KHR: Self = Self(0x8);
}
impl fmt::Display for CompositeAlphaFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "OPAQUE_KHR"),
            (2u64, "PRE_MULTIPLIED_KHR"),
            (4u64, "POST_MULTIPLIED_KHR"),
            (8u64, "INHERIT_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPlaneAlphaFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayPlaneAlphaFlagsKHR(Flags);
bitflags!(DisplayPlaneAlphaFlagsKHR, Flags);
impl DisplayPlaneAlphaFlagsKHR {
    pub const OPAQUE_KHR: Self = Self(0x1);
    pub const GLOBAL_KHR: Self = Self(0x2);
    pub const PER_PIXEL_KHR: Self = Self(0x4);
    pub const PER_PIXEL_PREMULTIPLIED_KHR: Self = Self(0x8);
}
impl fmt::Display for DisplayPlaneAlphaFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "OPAQUE_KHR"),
            (2u64, "GLOBAL_KHR"),
            (4u64, "PER_PIXEL_KHR"),
            (8u64, "PER_PIXEL_PREMULTIPLIED_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceTransformFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceTransformFlagsKHR(Flags);
bitflags!(SurfaceTransformFlagsKHR, Flags);
impl SurfaceTransformFlagsKHR {
    pub const IDENTITY_KHR: Self = Self(0x1);
    pub const ROTATE_90_KHR: Self = Self(0x2);
    pub const ROTATE_180_KHR: Self = Self(0x4);
    pub const ROTATE_270_KHR: Self = Self(0x8);
    pub const HORIZONTAL_MIRROR_KHR: Self = Self(0x10);
    pub const HORIZONTAL_MIRROR_ROTATE_90_KHR: Self = Self(0x20);
    pub const HORIZONTAL_MIRROR_ROTATE_180_KHR: Self = Self(0x40);
    pub const HORIZONTAL_MIRROR_ROTATE_270_KHR: Self = Self(0x80);
    pub const INHERIT_KHR: Self = Self(0x100);
}
impl fmt::Display for SurfaceTransformFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "IDENTITY_KHR"),
            (2u64, "ROTATE_90_KHR"),
            (4u64, "ROTATE_180_KHR"),
            (8u64, "ROTATE_270_KHR"),
            (16u64, "HORIZONTAL_MIRROR_KHR"),
            (32u64, "HORIZONTAL_MIRROR_ROTATE_90_KHR"),
            (64u64, "HORIZONTAL_MIRROR_ROTATE_180_KHR"),
            (128u64, "HORIZONTAL_MIRROR_ROTATE_270_KHR"),
            (256u64, "INHERIT_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainCreateFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwapchainCreateFlagsKHR(Flags);
bitflags!(SwapchainCreateFlagsKHR, Flags);
impl SwapchainCreateFlagsKHR {
    #[doc = "Allow images with VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT"]
    pub const SPLIT_INSTANCE_BIND_REGIONS_KHR: Self = Self(0x1);
    #[doc = "Swapchain is protected"]
    pub const PROTECTED_KHR: Self = Self(0x2);
    pub const MUTABLE_FORMAT_KHR: Self = Self(0x4);
    pub const PRESENT_TIMING_EXT: Self = Self(0x200);
    #[doc = "Allow use of VK_KHR_present_id2 with this swapchain"]
    pub const PRESENT_ID_2_KHR: Self = Self(0x40);
    #[doc = "Allow use of VK_KHR_present_wait2 with this swapchain"]
    pub const PRESENT_WAIT_2_KHR: Self = Self(0x80);
    pub const DEFERRED_MEMORY_ALLOCATION_KHR: Self = Self(0x8);
}
impl fmt::Display for SwapchainCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "SPLIT_INSTANCE_BIND_REGIONS_KHR"),
            (2u64, "PROTECTED_KHR"),
            (4u64, "MUTABLE_FORMAT_KHR"),
            (512u64, "PRESENT_TIMING_EXT"),
            (64u64, "PRESENT_ID_2_KHR"),
            (128u64, "PRESENT_WAIT_2_KHR"),
            (8u64, "DEFERRED_MEMORY_ALLOCATION_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeCreateFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayModeCreateFlagsKHR(Flags);
bitflags!(DisplayModeCreateFlagsKHR, Flags);
impl DisplayModeCreateFlagsKHR {}
impl fmt::Display for DisplayModeCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplaySurfaceCreateFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplaySurfaceCreateFlagsKHR(Flags);
bitflags!(DisplaySurfaceCreateFlagsKHR, Flags);
impl DisplaySurfaceCreateFlagsKHR {}
impl fmt::Display for DisplaySurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAndroidSurfaceCreateFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AndroidSurfaceCreateFlagsKHR(Flags);
bitflags!(AndroidSurfaceCreateFlagsKHR, Flags);
impl AndroidSurfaceCreateFlagsKHR {}
impl fmt::Display for AndroidSurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkViSurfaceCreateFlagsNN.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViSurfaceCreateFlagsNN(Flags);
bitflags!(ViSurfaceCreateFlagsNN, Flags);
impl ViSurfaceCreateFlagsNN {}
impl fmt::Display for ViSurfaceCreateFlagsNN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkWaylandSurfaceCreateFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WaylandSurfaceCreateFlagsKHR(Flags);
bitflags!(WaylandSurfaceCreateFlagsKHR, Flags);
impl WaylandSurfaceCreateFlagsKHR {}
impl fmt::Display for WaylandSurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkUbmSurfaceCreateFlagsSEC.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UbmSurfaceCreateFlagsSEC(Flags);
bitflags!(UbmSurfaceCreateFlagsSEC, Flags);
impl UbmSurfaceCreateFlagsSEC {}
impl fmt::Display for UbmSurfaceCreateFlagsSEC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkWin32SurfaceCreateFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Win32SurfaceCreateFlagsKHR(Flags);
bitflags!(Win32SurfaceCreateFlagsKHR, Flags);
impl Win32SurfaceCreateFlagsKHR {}
impl fmt::Display for Win32SurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkXlibSurfaceCreateFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XlibSurfaceCreateFlagsKHR(Flags);
bitflags!(XlibSurfaceCreateFlagsKHR, Flags);
impl XlibSurfaceCreateFlagsKHR {}
impl fmt::Display for XlibSurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkXcbSurfaceCreateFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XcbSurfaceCreateFlagsKHR(Flags);
bitflags!(XcbSurfaceCreateFlagsKHR, Flags);
impl XcbSurfaceCreateFlagsKHR {}
impl fmt::Display for XcbSurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectFBSurfaceCreateFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirectFBSurfaceCreateFlagsEXT(Flags);
bitflags!(DirectFBSurfaceCreateFlagsEXT, Flags);
impl DirectFBSurfaceCreateFlagsEXT {}
impl fmt::Display for DirectFBSurfaceCreateFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIOSSurfaceCreateFlagsMVK.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IOSSurfaceCreateFlagsMVK(Flags);
bitflags!(IOSSurfaceCreateFlagsMVK, Flags);
impl IOSSurfaceCreateFlagsMVK {}
impl fmt::Display for IOSSurfaceCreateFlagsMVK {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMacOSSurfaceCreateFlagsMVK.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MacOSSurfaceCreateFlagsMVK(Flags);
bitflags!(MacOSSurfaceCreateFlagsMVK, Flags);
impl MacOSSurfaceCreateFlagsMVK {}
impl fmt::Display for MacOSSurfaceCreateFlagsMVK {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMetalSurfaceCreateFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetalSurfaceCreateFlagsEXT(Flags);
bitflags!(MetalSurfaceCreateFlagsEXT, Flags);
impl MetalSurfaceCreateFlagsEXT {}
impl fmt::Display for MetalSurfaceCreateFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImagePipeSurfaceCreateFlagsFUCHSIA.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImagePipeSurfaceCreateFlagsFUCHSIA(Flags);
bitflags!(ImagePipeSurfaceCreateFlagsFUCHSIA, Flags);
impl ImagePipeSurfaceCreateFlagsFUCHSIA {}
impl fmt::Display for ImagePipeSurfaceCreateFlagsFUCHSIA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkStreamDescriptorSurfaceCreateFlagsGGP.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StreamDescriptorSurfaceCreateFlagsGGP(Flags);
bitflags!(StreamDescriptorSurfaceCreateFlagsGGP, Flags);
impl StreamDescriptorSurfaceCreateFlagsGGP {}
impl fmt::Display for StreamDescriptorSurfaceCreateFlagsGGP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkHeadlessSurfaceCreateFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeadlessSurfaceCreateFlagsEXT(Flags);
bitflags!(HeadlessSurfaceCreateFlagsEXT, Flags);
impl HeadlessSurfaceCreateFlagsEXT {}
impl fmt::Display for HeadlessSurfaceCreateFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkScreenSurfaceCreateFlagsQNX.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScreenSurfaceCreateFlagsQNX(Flags);
bitflags!(ScreenSurfaceCreateFlagsQNX, Flags);
impl ScreenSurfaceCreateFlagsQNX {}
impl fmt::Display for ScreenSurfaceCreateFlagsQNX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPeerMemoryFeatureFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeerMemoryFeatureFlags(Flags);
bitflags!(PeerMemoryFeatureFlags, Flags);
impl PeerMemoryFeatureFlags {
    #[doc = "Can read with vkCmdCopy commands"]
    pub const COPY_SRC: Self = Self(0x1);
    #[doc = "Can write with vkCmdCopy commands"]
    pub const COPY_DST: Self = Self(0x2);
    #[doc = "Can read with any access type/command"]
    pub const GENERIC_SRC: Self = Self(0x4);
    #[doc = "Can write with and access type/command"]
    pub const GENERIC_DST: Self = Self(0x8);
}
impl fmt::Display for PeerMemoryFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "COPY_SRC"),
            (2u64, "COPY_DST"),
            (4u64, "GENERIC_SRC"),
            (8u64, "GENERIC_DST"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryAllocateFlags(Flags);
bitflags!(MemoryAllocateFlags, Flags);
impl MemoryAllocateFlags {
    #[doc = "Force allocation on specific devices"]
    pub const DEVICE_MASK: Self = Self(0x1);
    pub const DEVICE_ADDRESS: Self = Self(0x2);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(0x4);
    pub const ZERO_INITIALIZE_EXT: Self = Self(0x8);
}
impl fmt::Display for MemoryAllocateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "DEVICE_MASK"),
            (2u64, "DEVICE_ADDRESS"),
            (4u64, "DEVICE_ADDRESS_CAPTURE_REPLAY"),
            (8u64, "ZERO_INITIALIZE_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupPresentModeFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceGroupPresentModeFlagsKHR(Flags);
bitflags!(DeviceGroupPresentModeFlagsKHR, Flags);
impl DeviceGroupPresentModeFlagsKHR {
    #[doc = "Present from local memory"]
    pub const LOCAL_KHR: Self = Self(0x1);
    #[doc = "Present from remote memory"]
    pub const REMOTE_KHR: Self = Self(0x2);
    #[doc = "Present sum of local and/or remote memory"]
    pub const SUM_KHR: Self = Self(0x4);
    #[doc = "Each physical device presents from local memory"]
    pub const LOCAL_MULTI_DEVICE_KHR: Self = Self(0x8);
}
impl fmt::Display for DeviceGroupPresentModeFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "LOCAL_KHR"),
            (2u64, "REMOTE_KHR"),
            (4u64, "SUM_KHR"),
            (8u64, "LOCAL_MULTI_DEVICE_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugReportFlagsEXT(Flags);
bitflags!(DebugReportFlagsEXT, Flags);
impl DebugReportFlagsEXT {
    pub const INFORMATION_EXT: Self = Self(0x1);
    pub const WARNING_EXT: Self = Self(0x2);
    pub const PERFORMANCE_WARNING_EXT: Self = Self(0x4);
    pub const ERROR_EXT: Self = Self(0x8);
    pub const DEBUG_EXT: Self = Self(0x10);
}
impl fmt::Display for DebugReportFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "INFORMATION_EXT"),
            (2u64, "WARNING_EXT"),
            (4u64, "PERFORMANCE_WARNING_EXT"),
            (8u64, "ERROR_EXT"),
            (16u64, "DEBUG_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolTrimFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandPoolTrimFlags(Flags);
bitflags!(CommandPoolTrimFlags, Flags);
impl CommandPoolTrimFlags {}
impl fmt::Display for CommandPoolTrimFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryHandleTypeFlagsNV(Flags);
bitflags!(ExternalMemoryHandleTypeFlagsNV, Flags);
impl ExternalMemoryHandleTypeFlagsNV {
    pub const OPAQUE_WIN32_NV: Self = Self(0x1);
    pub const OPAQUE_WIN32_KMT_NV: Self = Self(0x2);
    pub const D3D11_IMAGE_NV: Self = Self(0x4);
    pub const D3D11_IMAGE_KMT_NV: Self = Self(0x8);
}
impl fmt::Display for ExternalMemoryHandleTypeFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "OPAQUE_WIN32_NV"),
            (2u64, "OPAQUE_WIN32_KMT_NV"),
            (4u64, "D3D11_IMAGE_NV"),
            (8u64, "D3D11_IMAGE_KMT_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureIndexFormatFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClusterAccelerationStructureIndexFormatFlagsNV(Flags);
bitflags!(ClusterAccelerationStructureIndexFormatFlagsNV, Flags);
impl ClusterAccelerationStructureIndexFormatFlagsNV {
    pub const TYPE_8BIT_NV: Self = Self(0x1);
    pub const TYPE_16BIT_NV: Self = Self(0x2);
    pub const TYPE_32BIT_NV: Self = Self(0x4);
}
impl fmt::Display for ClusterAccelerationStructureIndexFormatFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TYPE_8BIT_NV"),
            (2u64, "TYPE_16BIT_NV"),
            (4u64, "TYPE_32BIT_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryFeatureFlagsNV(Flags);
bitflags!(ExternalMemoryFeatureFlagsNV, Flags);
impl ExternalMemoryFeatureFlagsNV {
    pub const DEDICATED_ONLY_NV: Self = Self(0x1);
    pub const EXPORTABLE_NV: Self = Self(0x2);
    pub const IMPORTABLE_NV: Self = Self(0x4);
}
impl fmt::Display for ExternalMemoryFeatureFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "DEDICATED_ONLY_NV"),
            (2u64, "EXPORTABLE_NV"),
            (4u64, "IMPORTABLE_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryHandleTypeFlags(Flags);
bitflags!(ExternalMemoryHandleTypeFlags, Flags);
impl ExternalMemoryHandleTypeFlags {
    pub const OPAQUE_FD: Self = Self(0x1);
    pub const OPAQUE_WIN32: Self = Self(0x2);
    pub const OPAQUE_WIN32_KMT: Self = Self(0x4);
    pub const D3D11_TEXTURE: Self = Self(0x8);
    pub const D3D11_TEXTURE_KMT: Self = Self(0x10);
    pub const D3D12_HEAP: Self = Self(0x20);
    pub const D3D12_RESOURCE: Self = Self(0x40);
    pub const DMA_BUF_EXT: Self = Self(0x200);
    pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self = Self(0x400);
    pub const HOST_ALLOCATION_EXT: Self = Self(0x80);
    pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(0x100);
    pub const ZIRCON_VMO_FUCHSIA: Self = Self(0x800);
    pub const RDMA_ADDRESS_NV: Self = Self(0x1000);
    pub const SCI_BUF_NV: Self = Self(0x2000);
    pub const OH_NATIVE_BUFFER_OHOS: Self = Self(0x8000);
    pub const SCREEN_BUFFER_QNX: Self = Self(0x4000);
    pub const MTLBUFFER_EXT: Self = Self(0x10000);
    pub const MTLTEXTURE_EXT: Self = Self(0x20000);
    pub const MTLHEAP_EXT: Self = Self(0x40000);
}
impl fmt::Display for ExternalMemoryHandleTypeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "OPAQUE_FD"),
            (2u64, "OPAQUE_WIN32"),
            (4u64, "OPAQUE_WIN32_KMT"),
            (8u64, "D3D11_TEXTURE"),
            (16u64, "D3D11_TEXTURE_KMT"),
            (32u64, "D3D12_HEAP"),
            (64u64, "D3D12_RESOURCE"),
            (512u64, "DMA_BUF_EXT"),
            (1024u64, "ANDROID_HARDWARE_BUFFER_ANDROID"),
            (128u64, "HOST_ALLOCATION_EXT"),
            (256u64, "HOST_MAPPED_FOREIGN_MEMORY_EXT"),
            (2048u64, "ZIRCON_VMO_FUCHSIA"),
            (4096u64, "RDMA_ADDRESS_NV"),
            (8192u64, "SCI_BUF_NV"),
            (32768u64, "OH_NATIVE_BUFFER_OHOS"),
            (16384u64, "SCREEN_BUFFER_QNX"),
            (65536u64, "MTLBUFFER_EXT"),
            (131072u64, "MTLTEXTURE_EXT"),
            (262144u64, "MTLHEAP_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryFeatureFlags(Flags);
bitflags!(ExternalMemoryFeatureFlags, Flags);
impl ExternalMemoryFeatureFlags {
    pub const DEDICATED_ONLY: Self = Self(0x1);
    pub const EXPORTABLE: Self = Self(0x2);
    pub const IMPORTABLE: Self = Self(0x4);
}
impl fmt::Display for ExternalMemoryFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "DEDICATED_ONLY"),
            (2u64, "EXPORTABLE"),
            (4u64, "IMPORTABLE"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreHandleTypeFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalSemaphoreHandleTypeFlags(Flags);
bitflags!(ExternalSemaphoreHandleTypeFlags, Flags);
impl ExternalSemaphoreHandleTypeFlags {
    pub const OPAQUE_FD: Self = Self(0x1);
    pub const OPAQUE_WIN32: Self = Self(0x2);
    pub const OPAQUE_WIN32_KMT: Self = Self(0x4);
    pub const D3D12_FENCE: Self = Self(0x8);
    pub const D3D11_FENCE: Self = Self::D3D12_FENCE;
    pub const SYNC_FD: Self = Self(0x10);
    pub const ZIRCON_EVENT_FUCHSIA: Self = Self(0x80);
    pub const SCI_SYNC_OBJ_NV: Self = Self(0x20);
}
impl fmt::Display for ExternalSemaphoreHandleTypeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "OPAQUE_FD"),
            (2u64, "OPAQUE_WIN32"),
            (4u64, "OPAQUE_WIN32_KMT"),
            (8u64, "D3D12_FENCE"),
            (16u64, "SYNC_FD"),
            (128u64, "ZIRCON_EVENT_FUCHSIA"),
            (32u64, "SCI_SYNC_OBJ_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreFeatureFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalSemaphoreFeatureFlags(Flags);
bitflags!(ExternalSemaphoreFeatureFlags, Flags);
impl ExternalSemaphoreFeatureFlags {
    pub const EXPORTABLE: Self = Self(0x1);
    pub const IMPORTABLE: Self = Self(0x2);
}
impl fmt::Display for ExternalSemaphoreFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "EXPORTABLE"), (2u64, "IMPORTABLE")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreImportFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreImportFlags(Flags);
bitflags!(SemaphoreImportFlags, Flags);
impl SemaphoreImportFlags {
    pub const TEMPORARY: Self = Self(0x1);
}
impl fmt::Display for SemaphoreImportFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "TEMPORARY")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceHandleTypeFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalFenceHandleTypeFlags(Flags);
bitflags!(ExternalFenceHandleTypeFlags, Flags);
impl ExternalFenceHandleTypeFlags {
    pub const OPAQUE_FD: Self = Self(0x1);
    pub const OPAQUE_WIN32: Self = Self(0x2);
    pub const OPAQUE_WIN32_KMT: Self = Self(0x4);
    pub const SYNC_FD: Self = Self(0x8);
    pub const SCI_SYNC_OBJ_NV: Self = Self(0x10);
    pub const SCI_SYNC_FENCE_NV: Self = Self(0x20);
}
impl fmt::Display for ExternalFenceHandleTypeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "OPAQUE_FD"),
            (2u64, "OPAQUE_WIN32"),
            (4u64, "OPAQUE_WIN32_KMT"),
            (8u64, "SYNC_FD"),
            (16u64, "SCI_SYNC_OBJ_NV"),
            (32u64, "SCI_SYNC_FENCE_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceFeatureFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalFenceFeatureFlags(Flags);
bitflags!(ExternalFenceFeatureFlags, Flags);
impl ExternalFenceFeatureFlags {
    pub const EXPORTABLE: Self = Self(0x1);
    pub const IMPORTABLE: Self = Self(0x2);
}
impl fmt::Display for ExternalFenceFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "EXPORTABLE"), (2u64, "IMPORTABLE")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceImportFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FenceImportFlags(Flags);
bitflags!(FenceImportFlags, Flags);
impl FenceImportFlags {
    pub const TEMPORARY: Self = Self(0x1);
}
impl fmt::Display for FenceImportFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "TEMPORARY")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCounterFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceCounterFlagsEXT(Flags);
bitflags!(SurfaceCounterFlagsEXT, Flags);
impl SurfaceCounterFlagsEXT {
    pub const VBLANK_EXT: Self = Self(0x1);
}
impl fmt::Display for SurfaceCounterFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "VBLANK_EXT")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportSwizzleStateCreateFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineViewportSwizzleStateCreateFlagsNV(Flags);
bitflags!(PipelineViewportSwizzleStateCreateFlagsNV, Flags);
impl PipelineViewportSwizzleStateCreateFlagsNV {}
impl fmt::Display for PipelineViewportSwizzleStateCreateFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDiscardRectangleStateCreateFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineDiscardRectangleStateCreateFlagsEXT(Flags);
bitflags!(PipelineDiscardRectangleStateCreateFlagsEXT, Flags);
impl PipelineDiscardRectangleStateCreateFlagsEXT {}
impl fmt::Display for PipelineDiscardRectangleStateCreateFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageToColorStateCreateFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCoverageToColorStateCreateFlagsNV(Flags);
bitflags!(PipelineCoverageToColorStateCreateFlagsNV, Flags);
impl PipelineCoverageToColorStateCreateFlagsNV {}
impl fmt::Display for PipelineCoverageToColorStateCreateFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageModulationStateCreateFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCoverageModulationStateCreateFlagsNV(Flags);
bitflags!(PipelineCoverageModulationStateCreateFlagsNV, Flags);
impl PipelineCoverageModulationStateCreateFlagsNV {}
impl fmt::Display for PipelineCoverageModulationStateCreateFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageReductionStateCreateFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCoverageReductionStateCreateFlagsNV(Flags);
bitflags!(PipelineCoverageReductionStateCreateFlagsNV, Flags);
impl PipelineCoverageReductionStateCreateFlagsNV {}
impl fmt::Display for PipelineCoverageReductionStateCreateFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheCreateFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidationCacheCreateFlagsEXT(Flags);
bitflags!(ValidationCacheCreateFlagsEXT, Flags);
impl ValidationCacheCreateFlagsEXT {}
impl fmt::Display for ValidationCacheCreateFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageSeverityFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessageSeverityFlagsEXT(Flags);
bitflags!(DebugUtilsMessageSeverityFlagsEXT, Flags);
impl DebugUtilsMessageSeverityFlagsEXT {
    pub const VERBOSE_EXT: Self = Self(0x1);
    pub const INFO_EXT: Self = Self(0x10);
    pub const WARNING_EXT: Self = Self(0x100);
    pub const ERROR_EXT: Self = Self(0x1000);
}
impl fmt::Display for DebugUtilsMessageSeverityFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "VERBOSE_EXT"),
            (16u64, "INFO_EXT"),
            (256u64, "WARNING_EXT"),
            (4096u64, "ERROR_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageTypeFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessageTypeFlagsEXT(Flags);
bitflags!(DebugUtilsMessageTypeFlagsEXT, Flags);
impl DebugUtilsMessageTypeFlagsEXT {
    pub const GENERAL_EXT: Self = Self(0x1);
    pub const VALIDATION_EXT: Self = Self(0x2);
    pub const PERFORMANCE_EXT: Self = Self(0x4);
    pub const DEVICE_ADDRESS_BINDING_EXT: Self = Self(0x8);
}
impl fmt::Display for DebugUtilsMessageTypeFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "GENERAL_EXT"),
            (2u64, "VALIDATION_EXT"),
            (4u64, "PERFORMANCE_EXT"),
            (8u64, "DEVICE_ADDRESS_BINDING_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerCreateFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessengerCreateFlagsEXT(Flags);
bitflags!(DebugUtilsMessengerCreateFlagsEXT, Flags);
impl DebugUtilsMessengerCreateFlagsEXT {}
impl fmt::Display for DebugUtilsMessengerCreateFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerCallbackDataFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessengerCallbackDataFlagsEXT(Flags);
bitflags!(DebugUtilsMessengerCallbackDataFlagsEXT, Flags);
impl DebugUtilsMessengerCallbackDataFlagsEXT {}
impl fmt::Display for DebugUtilsMessengerCallbackDataFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemoryReportFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceMemoryReportFlagsEXT(Flags);
bitflags!(DeviceMemoryReportFlagsEXT, Flags);
impl DeviceMemoryReportFlagsEXT {}
impl fmt::Display for DeviceMemoryReportFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationConservativeStateCreateFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineRasterizationConservativeStateCreateFlagsEXT(Flags);
bitflags!(PipelineRasterizationConservativeStateCreateFlagsEXT, Flags);
impl PipelineRasterizationConservativeStateCreateFlagsEXT {}
impl fmt::Display for PipelineRasterizationConservativeStateCreateFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBindingFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorBindingFlags(Flags);
bitflags!(DescriptorBindingFlags, Flags);
impl DescriptorBindingFlags {
    pub const UPDATE_AFTER_BIND: Self = Self(0x1);
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(0x2);
    pub const PARTIALLY_BOUND: Self = Self(0x4);
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(0x8);
}
impl fmt::Display for DescriptorBindingFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "UPDATE_AFTER_BIND"),
            (2u64, "UPDATE_UNUSED_WHILE_PENDING"),
            (4u64, "PARTIALLY_BOUND"),
            (8u64, "VARIABLE_DESCRIPTOR_COUNT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkConditionalRenderingFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConditionalRenderingFlagsEXT(Flags);
bitflags!(ConditionalRenderingFlagsEXT, Flags);
impl ConditionalRenderingFlagsEXT {
    pub const INVERTED_EXT: Self = Self(0x1);
}
impl fmt::Display for ConditionalRenderingFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "INVERTED_EXT")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveModeFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResolveModeFlags(Flags);
bitflags!(ResolveModeFlags, Flags);
impl ResolveModeFlags {
    pub const NONE: Self = Self(0x0);
    pub const SAMPLE_ZERO: Self = Self(0x1);
    pub const AVERAGE: Self = Self(0x2);
    pub const MIN: Self = Self(0x4);
    pub const MAX: Self = Self(0x8);
    pub const EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID: Self = Self(0x10);
    pub const CUSTOM_EXT: Self = Self(0x20);
}
impl fmt::Display for ResolveModeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "SAMPLE_ZERO"),
            (2u64, "AVERAGE"),
            (4u64, "MIN"),
            (8u64, "MAX"),
            (16u64, "EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID"),
            (32u64, "CUSTOM_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationStateStreamCreateFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineRasterizationStateStreamCreateFlagsEXT(Flags);
bitflags!(PipelineRasterizationStateStreamCreateFlagsEXT, Flags);
impl PipelineRasterizationStateStreamCreateFlagsEXT {}
impl fmt::Display for PipelineRasterizationStateStreamCreateFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationDepthClipStateCreateFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineRasterizationDepthClipStateCreateFlagsEXT(Flags);
bitflags!(PipelineRasterizationDepthClipStateCreateFlagsEXT, Flags);
impl PipelineRasterizationDepthClipStateCreateFlagsEXT {}
impl fmt::Display for PipelineRasterizationDepthClipStateCreateFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainImageUsageFlagsANDROID.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwapchainImageUsageFlagsANDROID(Flags);
bitflags!(SwapchainImageUsageFlagsANDROID, Flags);
impl SwapchainImageUsageFlagsANDROID {
    pub const SHARED_ANDROID: Self = Self(0x1);
}
impl fmt::Display for SwapchainImageUsageFlagsANDROID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "SHARED_ANDROID")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkToolPurposeFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ToolPurposeFlags(Flags);
bitflags!(ToolPurposeFlags, Flags);
impl ToolPurposeFlags {
    pub const VALIDATION: Self = Self(0x1);
    pub const PROFILING: Self = Self(0x2);
    pub const TRACING: Self = Self(0x4);
    pub const ADDITIONAL_FEATURES: Self = Self(0x8);
    pub const MODIFYING_FEATURES: Self = Self(0x10);
    pub const DEBUG_REPORTING_EXT: Self = Self(0x20);
    pub const DEBUG_MARKERS_EXT: Self = Self(0x40);
}
impl fmt::Display for ToolPurposeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "VALIDATION"),
            (2u64, "PROFILING"),
            (4u64, "TRACING"),
            (8u64, "ADDITIONAL_FEATURES"),
            (16u64, "MODIFYING_FEATURES"),
            (32u64, "DEBUG_REPORTING_EXT"),
            (64u64, "DEBUG_MARKERS_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubmitFlags(Flags);
bitflags!(SubmitFlags, Flags);
impl SubmitFlags {
    pub const PROTECTED: Self = Self(0x1);
}
impl fmt::Display for SubmitFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "PROTECTED")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatConstraintsFlagsFUCHSIA.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageFormatConstraintsFlagsFUCHSIA(Flags);
bitflags!(ImageFormatConstraintsFlagsFUCHSIA, Flags);
impl ImageFormatConstraintsFlagsFUCHSIA {}
impl fmt::Display for ImageFormatConstraintsFlagsFUCHSIA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyFlags.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HostImageCopyFlags(Flags);
bitflags!(HostImageCopyFlags, Flags);
impl HostImageCopyFlags {
    pub const MEMCPY: Self = Self(0x1);
}
impl fmt::Display for HostImageCopyFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "MEMCPY")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureInstanceFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartitionedAccelerationStructureInstanceFlagsNV(Flags);
bitflags!(PartitionedAccelerationStructureInstanceFlagsNV, Flags);
impl PartitionedAccelerationStructureInstanceFlagsNV {
    pub const FLAG_TRIANGLE_FACING_CULL_DISABLE_NV: Self = Self(0x1);
    pub const FLAG_TRIANGLE_FLIP_FACING_NV: Self = Self(0x2);
    pub const FLAG_FORCE_OPAQUE_NV: Self = Self(0x4);
    pub const FLAG_FORCE_NO_OPAQUE_NV: Self = Self(0x8);
    pub const FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV: Self = Self(0x10);
}
impl fmt::Display for PartitionedAccelerationStructureInstanceFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "FLAG_TRIANGLE_FACING_CULL_DISABLE_NV"),
            (2u64, "FLAG_TRIANGLE_FLIP_FACING_NV"),
            (4u64, "FLAG_FORCE_OPAQUE_NV"),
            (8u64, "FLAG_FORCE_NO_OPAQUE_NV"),
            (16u64, "FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageConstraintsInfoFlagsFUCHSIA.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageConstraintsInfoFlagsFUCHSIA(Flags);
bitflags!(ImageConstraintsInfoFlagsFUCHSIA, Flags);
impl ImageConstraintsInfoFlagsFUCHSIA {
    pub const CPU_READ_RARELY_FUCHSIA: Self = Self(0x1);
    pub const CPU_READ_OFTEN_FUCHSIA: Self = Self(0x2);
    pub const CPU_WRITE_RARELY_FUCHSIA: Self = Self(0x4);
    pub const CPU_WRITE_OFTEN_FUCHSIA: Self = Self(0x8);
    pub const PROTECTED_OPTIONAL_FUCHSIA: Self = Self(0x10);
}
impl fmt::Display for ImageConstraintsInfoFlagsFUCHSIA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "CPU_READ_RARELY_FUCHSIA"),
            (2u64, "CPU_READ_OFTEN_FUCHSIA"),
            (4u64, "CPU_WRITE_RARELY_FUCHSIA"),
            (8u64, "CPU_WRITE_OFTEN_FUCHSIA"),
            (16u64, "PROTECTED_OPTIONAL_FUCHSIA"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkGraphicsPipelineLibraryFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GraphicsPipelineLibraryFlagsEXT(Flags);
bitflags!(GraphicsPipelineLibraryFlagsEXT, Flags);
impl GraphicsPipelineLibraryFlagsEXT {
    pub const VERTEX_INPUT_INTERFACE_EXT: Self = Self(0x1);
    pub const PRE_RASTERIZATION_SHADERS_EXT: Self = Self(0x2);
    pub const FRAGMENT_SHADER_EXT: Self = Self(0x4);
    pub const FRAGMENT_OUTPUT_INTERFACE_EXT: Self = Self(0x8);
}
impl fmt::Display for GraphicsPipelineLibraryFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "VERTEX_INPUT_INTERFACE_EXT"),
            (2u64, "PRE_RASTERIZATION_SHADERS_EXT"),
            (4u64, "FRAGMENT_SHADER_EXT"),
            (8u64, "FRAGMENT_OUTPUT_INTERFACE_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageCompressionFlagsEXT(Flags);
bitflags!(ImageCompressionFlagsEXT, Flags);
impl ImageCompressionFlagsEXT {
    pub const DEFAULT_EXT: Self = Self(0x0);
    pub const FIXED_RATE_DEFAULT_EXT: Self = Self(0x1);
    pub const FIXED_RATE_EXPLICIT_EXT: Self = Self(0x2);
    pub const DISABLED_EXT: Self = Self(0x4);
}
impl fmt::Display for ImageCompressionFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "FIXED_RATE_DEFAULT_EXT"),
            (2u64, "FIXED_RATE_EXPLICIT_EXT"),
            (4u64, "DISABLED_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionFixedRateFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageCompressionFixedRateFlagsEXT(Flags);
bitflags!(ImageCompressionFixedRateFlagsEXT, Flags);
impl ImageCompressionFixedRateFlagsEXT {
    pub const NONE_EXT: Self = Self(0x0);
    pub const TYPE_1BPC_EXT: Self = Self(0x1);
    pub const TYPE_2BPC_EXT: Self = Self(0x2);
    pub const TYPE_3BPC_EXT: Self = Self(0x4);
    pub const TYPE_4BPC_EXT: Self = Self(0x8);
    pub const TYPE_5BPC_EXT: Self = Self(0x10);
    pub const TYPE_6BPC_EXT: Self = Self(0x20);
    pub const TYPE_7BPC_EXT: Self = Self(0x40);
    pub const TYPE_8BPC_EXT: Self = Self(0x80);
    pub const TYPE_9BPC_EXT: Self = Self(0x100);
    pub const TYPE_10BPC_EXT: Self = Self(0x200);
    pub const TYPE_11BPC_EXT: Self = Self(0x400);
    pub const TYPE_12BPC_EXT: Self = Self(0x800);
    pub const TYPE_13BPC_EXT: Self = Self(0x1000);
    pub const TYPE_14BPC_EXT: Self = Self(0x2000);
    pub const TYPE_15BPC_EXT: Self = Self(0x4000);
    pub const TYPE_16BPC_EXT: Self = Self(0x8000);
    pub const TYPE_17BPC_EXT: Self = Self(0x10000);
    pub const TYPE_18BPC_EXT: Self = Self(0x20000);
    pub const TYPE_19BPC_EXT: Self = Self(0x40000);
    pub const TYPE_20BPC_EXT: Self = Self(0x80000);
    pub const TYPE_21BPC_EXT: Self = Self(0x100000);
    pub const TYPE_22BPC_EXT: Self = Self(0x200000);
    pub const TYPE_23BPC_EXT: Self = Self(0x400000);
    pub const TYPE_24BPC_EXT: Self = Self(0x800000);
}
impl fmt::Display for ImageCompressionFixedRateFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TYPE_1BPC_EXT"),
            (2u64, "TYPE_2BPC_EXT"),
            (4u64, "TYPE_3BPC_EXT"),
            (8u64, "TYPE_4BPC_EXT"),
            (16u64, "TYPE_5BPC_EXT"),
            (32u64, "TYPE_6BPC_EXT"),
            (64u64, "TYPE_7BPC_EXT"),
            (128u64, "TYPE_8BPC_EXT"),
            (256u64, "TYPE_9BPC_EXT"),
            (512u64, "TYPE_10BPC_EXT"),
            (1024u64, "TYPE_11BPC_EXT"),
            (2048u64, "TYPE_12BPC_EXT"),
            (4096u64, "TYPE_13BPC_EXT"),
            (8192u64, "TYPE_14BPC_EXT"),
            (16384u64, "TYPE_15BPC_EXT"),
            (32768u64, "TYPE_16BPC_EXT"),
            (65536u64, "TYPE_17BPC_EXT"),
            (131072u64, "TYPE_18BPC_EXT"),
            (262144u64, "TYPE_19BPC_EXT"),
            (524288u64, "TYPE_20BPC_EXT"),
            (1048576u64, "TYPE_21BPC_EXT"),
            (2097152u64, "TYPE_22BPC_EXT"),
            (4194304u64, "TYPE_23BPC_EXT"),
            (8388608u64, "TYPE_24BPC_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMetalObjectTypeFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExportMetalObjectTypeFlagsEXT(Flags);
bitflags!(ExportMetalObjectTypeFlagsEXT, Flags);
impl ExportMetalObjectTypeFlagsEXT {
    pub const METAL_DEVICE_EXT: Self = Self(0x1);
    pub const METAL_COMMAND_QUEUE_EXT: Self = Self(0x2);
    pub const METAL_BUFFER_EXT: Self = Self(0x4);
    pub const METAL_TEXTURE_EXT: Self = Self(0x8);
    pub const METAL_IOSURFACE_EXT: Self = Self(0x10);
    pub const METAL_SHARED_EVENT_EXT: Self = Self(0x20);
}
impl fmt::Display for ExportMetalObjectTypeFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "METAL_DEVICE_EXT"),
            (2u64, "METAL_COMMAND_QUEUE_EXT"),
            (4u64, "METAL_BUFFER_EXT"),
            (8u64, "METAL_TEXTURE_EXT"),
            (16u64, "METAL_IOSURFACE_EXT"),
            (32u64, "METAL_SHARED_EVENT_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderingAttachmentFlagsKHR(Flags);
bitflags!(RenderingAttachmentFlagsKHR, Flags);
impl RenderingAttachmentFlagsKHR {
    pub const INPUT_ATTACHMENT_FEEDBACK_KHR: Self = Self(0x1);
    pub const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR: Self = Self(0x2);
    pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR: Self = Self(0x4);
}
impl fmt::Display for RenderingAttachmentFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "INPUT_ATTACHMENT_FEEDBACK_KHR"),
            (2u64, "RESOLVE_SKIP_TRANSFER_FUNCTION_KHR"),
            (4u64, "RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveImageFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResolveImageFlagsKHR(Flags);
bitflags!(ResolveImageFlagsKHR, Flags);
impl ResolveImageFlagsKHR {
    pub const SKIP_TRANSFER_FUNCTION_KHR: Self = Self(0x1);
    pub const ENABLE_TRANSFER_FUNCTION_KHR: Self = Self(0x2);
}
impl fmt::Display for ResolveImageFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "SKIP_TRANSFER_FUNCTION_KHR"),
            (2u64, "ENABLE_TRANSFER_FUNCTION_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddressBindingFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceAddressBindingFlagsEXT(Flags);
bitflags!(DeviceAddressBindingFlagsEXT, Flags);
impl DeviceAddressBindingFlagsEXT {
    pub const INTERNAL_OBJECT_EXT: Self = Self(0x1);
}
impl fmt::Display for DeviceAddressBindingFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "INTERNAL_OBJECT_EXT")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowGridSizeFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpticalFlowGridSizeFlagsNV(Flags);
bitflags!(OpticalFlowGridSizeFlagsNV, Flags);
impl OpticalFlowGridSizeFlagsNV {
    pub const UNKNOWN_NV: Self = Self(0x0);
    pub const TYPE_1X1_NV: Self = Self(0x1);
    pub const TYPE_2X2_NV: Self = Self(0x2);
    pub const TYPE_4X4_NV: Self = Self(0x4);
    pub const TYPE_8X8_NV: Self = Self(0x8);
}
impl fmt::Display for OpticalFlowGridSizeFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TYPE_1X1_NV"),
            (2u64, "TYPE_2X2_NV"),
            (4u64, "TYPE_4X4_NV"),
            (8u64, "TYPE_8X8_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowUsageFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpticalFlowUsageFlagsNV(Flags);
bitflags!(OpticalFlowUsageFlagsNV, Flags);
impl OpticalFlowUsageFlagsNV {
    pub const UNKNOWN_NV: Self = Self(0x0);
    pub const INPUT_NV: Self = Self(0x1);
    pub const OUTPUT_NV: Self = Self(0x2);
    pub const HINT_NV: Self = Self(0x4);
    pub const COST_NV: Self = Self(0x8);
    pub const GLOBAL_FLOW_NV: Self = Self(0x10);
}
impl fmt::Display for OpticalFlowUsageFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "INPUT_NV"),
            (2u64, "OUTPUT_NV"),
            (4u64, "HINT_NV"),
            (8u64, "COST_NV"),
            (16u64, "GLOBAL_FLOW_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionCreateFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpticalFlowSessionCreateFlagsNV(Flags);
bitflags!(OpticalFlowSessionCreateFlagsNV, Flags);
impl OpticalFlowSessionCreateFlagsNV {
    pub const ENABLE_HINT_NV: Self = Self(0x1);
    pub const ENABLE_COST_NV: Self = Self(0x2);
    pub const ENABLE_GLOBAL_FLOW_NV: Self = Self(0x4);
    pub const ALLOW_REGIONS_NV: Self = Self(0x8);
    pub const BOTH_DIRECTIONS_NV: Self = Self(0x10);
}
impl fmt::Display for OpticalFlowSessionCreateFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "ENABLE_HINT_NV"),
            (2u64, "ENABLE_COST_NV"),
            (4u64, "ENABLE_GLOBAL_FLOW_NV"),
            (8u64, "ALLOW_REGIONS_NV"),
            (16u64, "BOTH_DIRECTIONS_NV"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowExecuteFlagsNV.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpticalFlowExecuteFlagsNV(Flags);
bitflags!(OpticalFlowExecuteFlagsNV, Flags);
impl OpticalFlowExecuteFlagsNV {
    pub const DISABLE_TEMPORAL_HINTS_NV: Self = Self(0x1);
}
impl fmt::Display for OpticalFlowExecuteFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "DISABLE_TEMPORAL_HINTS_NV")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFrameBoundaryFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrameBoundaryFlagsEXT(Flags);
bitflags!(FrameBoundaryFlagsEXT, Flags);
impl FrameBoundaryFlagsEXT {
    pub const FRAME_END_EXT: Self = Self(0x1);
}
impl fmt::Display for FrameBoundaryFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "FRAME_END_EXT")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentScalingFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresentScalingFlagsKHR(Flags);
bitflags!(PresentScalingFlagsKHR, Flags);
impl PresentScalingFlagsKHR {
    pub const ONE_TO_ONE_KHR: Self = Self(0x1);
    pub const ONE_TO_ONE_EXT: Self = Self::ONE_TO_ONE_KHR;
    pub const ASPECT_RATIO_STRETCH_KHR: Self = Self(0x2);
    pub const ASPECT_RATIO_STRETCH_EXT: Self = Self::ASPECT_RATIO_STRETCH_KHR;
    pub const STRETCH_KHR: Self = Self(0x4);
    pub const STRETCH_EXT: Self = Self::STRETCH_KHR;
}
impl fmt::Display for PresentScalingFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "ONE_TO_ONE_KHR"),
            (2u64, "ASPECT_RATIO_STRETCH_KHR"),
            (4u64, "STRETCH_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentGravityFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresentGravityFlagsKHR(Flags);
bitflags!(PresentGravityFlagsKHR, Flags);
impl PresentGravityFlagsKHR {
    pub const MIN_KHR: Self = Self(0x1);
    pub const MIN_EXT: Self = Self::MIN_KHR;
    pub const MAX_KHR: Self = Self(0x2);
    pub const MAX_EXT: Self = Self::MAX_KHR;
    pub const CENTERED_KHR: Self = Self(0x4);
    pub const CENTERED_EXT: Self = Self::CENTERED_KHR;
}
impl fmt::Display for PresentGravityFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] =
            &[(1u64, "MIN_KHR"), (2u64, "MAX_KHR"), (4u64, "CENTERED_KHR")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCreateFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderCreateFlagsEXT(Flags);
bitflags!(ShaderCreateFlagsEXT, Flags);
impl ShaderCreateFlagsEXT {
    pub const LINK_STAGE_EXT: Self = Self(0x1);
    pub const DESCRIPTOR_HEAP_EXT: Self = Self(0x400);
    pub const INSTRUMENT_SHADER_ARM: Self = Self(0x800);
    pub const ALLOW_VARYING_SUBGROUP_SIZE_EXT: Self = Self(0x2);
    pub const REQUIRE_FULL_SUBGROUPS_EXT: Self = Self(0x4);
    pub const NO_TASK_SHADER_EXT: Self = Self(0x8);
    pub const DISPATCH_BASE_EXT: Self = Self(0x10);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_EXT: Self = Self(0x20);
    pub const FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self = Self(0x40);
    pub const INDIRECT_BINDABLE_EXT: Self = Self(0x80);
    pub const TYPE_64_INDEXING_EXT: Self = Self(0x8000);
}
impl fmt::Display for ShaderCreateFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "LINK_STAGE_EXT"),
            (1024u64, "DESCRIPTOR_HEAP_EXT"),
            (2048u64, "INSTRUMENT_SHADER_ARM"),
            (2u64, "ALLOW_VARYING_SUBGROUP_SIZE_EXT"),
            (4u64, "REQUIRE_FULL_SUBGROUPS_EXT"),
            (8u64, "NO_TASK_SHADER_EXT"),
            (16u64, "DISPATCH_BASE_EXT"),
            (32u64, "FRAGMENT_SHADING_RATE_ATTACHMENT_EXT"),
            (64u64, "FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT"),
            (128u64, "INDIRECT_BINDABLE_EXT"),
            (32768u64, "TYPE_64_INDEXING_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTileShadingRenderPassFlagsQCOM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TileShadingRenderPassFlagsQCOM(Flags);
bitflags!(TileShadingRenderPassFlagsQCOM, Flags);
impl TileShadingRenderPassFlagsQCOM {
    pub const ENABLE_QCOM: Self = Self(0x1);
    pub const PER_TILE_EXECUTION_QCOM: Self = Self(0x2);
}
impl fmt::Display for TileShadingRenderPassFlagsQCOM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "ENABLE_QCOM"), (2u64, "PER_TILE_EXECUTION_QCOM")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSchedulingControlsFlagsARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalDeviceSchedulingControlsFlagsARM(Flags64);
bitflags!(PhysicalDeviceSchedulingControlsFlagsARM, Flags64);
impl PhysicalDeviceSchedulingControlsFlagsARM {
    pub const SHADER_CORE_COUNT_ARM: Self = Self(0x1);
    pub const DISPATCH_PARAMETERS_ARM: Self = Self(0x2);
}
impl fmt::Display for PhysicalDeviceSchedulingControlsFlagsARM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "SHADER_CORE_COUNT_ARM"),
            (2u64, "DISPATCH_PARAMETERS_ARM"),
        ];
        flag_display(self.0, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCreateFlagsOHOS.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceCreateFlagsOHOS(Flags);
bitflags!(SurfaceCreateFlagsOHOS, Flags);
impl SurfaceCreateFlagsOHOS {}
impl fmt::Display for SurfaceCreateFlagsOHOS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentStageFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresentStageFlagsEXT(Flags);
bitflags!(PresentStageFlagsEXT, Flags);
impl PresentStageFlagsEXT {
    pub const QUEUE_OPERATIONS_END_EXT: Self = Self(0x1);
    pub const REQUEST_DEQUEUED_EXT: Self = Self(0x2);
    pub const IMAGE_FIRST_PIXEL_OUT_EXT: Self = Self(0x4);
    pub const IMAGE_FIRST_PIXEL_VISIBLE_EXT: Self = Self(0x8);
}
impl fmt::Display for PresentStageFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "QUEUE_OPERATIONS_END_EXT"),
            (2u64, "REQUEST_DEQUEUED_EXT"),
            (4u64, "IMAGE_FIRST_PIXEL_OUT_EXT"),
            (8u64, "IMAGE_FIRST_PIXEL_VISIBLE_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPastPresentationTimingFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PastPresentationTimingFlagsEXT(Flags);
bitflags!(PastPresentationTimingFlagsEXT, Flags);
impl PastPresentationTimingFlagsEXT {
    pub const ALLOW_PARTIAL_RESULTS_EXT: Self = Self(0x1);
    pub const ALLOW_OUT_OF_ORDER_RESULTS_EXT: Self = Self(0x2);
}
impl fmt::Display for PastPresentationTimingFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "ALLOW_PARTIAL_RESULTS_EXT"),
            (2u64, "ALLOW_OUT_OF_ORDER_RESULTS_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentTimingInfoFlagsEXT.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresentTimingInfoFlagsEXT(Flags);
bitflags!(PresentTimingInfoFlagsEXT, Flags);
impl PresentTimingInfoFlagsEXT {
    pub const PRESENT_AT_RELATIVE_TIME_EXT: Self = Self(0x1);
    pub const PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT: Self = Self(0x2);
}
impl fmt::Display for PresentTimingInfoFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "PRESENT_AT_RELATIVE_TIME_EXT"),
            (2u64, "PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainImageUsageFlagsOHOS.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwapchainImageUsageFlagsOHOS(Flags);
bitflags!(SwapchainImageUsageFlagsOHOS, Flags);
impl SwapchainImageUsageFlagsOHOS {
    pub const SHARED_OHOS: Self = Self(0x1);
}
impl fmt::Display for SwapchainImageUsageFlagsOHOS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "SHARED_OHOS")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionFlagsARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceCounterDescriptionFlagsARM(Flags);
bitflags!(PerformanceCounterDescriptionFlagsARM, Flags);
impl PerformanceCounterDescriptionFlagsARM {}
impl fmt::Display for PerformanceCounterDescriptionFlagsARM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationValuesFlagsARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderInstrumentationValuesFlagsARM(Flags);
bitflags!(ShaderInstrumentationValuesFlagsARM, Flags);
impl ShaderInstrumentationValuesFlagsARM {}
impl fmt::Display for ShaderInstrumentationValuesFlagsARM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphTOSAQualityFlagsARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphTOSAQualityFlagsARM(Flags);
bitflags!(DataGraphTOSAQualityFlagsARM, Flags);
impl DataGraphTOSAQualityFlagsARM {
    pub const DATA_GRAPH_TOSA_QUALITY_ACCELERATED_ARM: Self = Self(0x1);
    pub const DATA_GRAPH_TOSA_QUALITY_CONFORMANT_ARM: Self = Self(0x2);
    pub const DATA_GRAPH_TOSA_QUALITY_EXPERIMENTAL_ARM: Self = Self(0x4);
    pub const DATA_GRAPH_TOSA_QUALITY_DEPRECATED_ARM: Self = Self(0x8);
}
impl fmt::Display for DataGraphTOSAQualityFlagsARM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "DATA_GRAPH_TOSA_QUALITY_ACCELERATED_ARM"),
            (2u64, "DATA_GRAPH_TOSA_QUALITY_CONFORMANT_ARM"),
            (4u64, "DATA_GRAPH_TOSA_QUALITY_EXPERIMENTAL_ARM"),
            (8u64, "DATA_GRAPH_TOSA_QUALITY_DEPRECATED_ARM"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowGridSizeFlagsARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphOpticalFlowGridSizeFlagsARM(Flags);
bitflags!(DataGraphOpticalFlowGridSizeFlagsARM, Flags);
impl DataGraphOpticalFlowGridSizeFlagsARM {
    pub const UNKNOWN_ARM: Self = Self(0x0);
    pub const TYPE_1X1_ARM: Self = Self(0x1);
    pub const TYPE_2X2_ARM: Self = Self(0x2);
    pub const TYPE_4X4_ARM: Self = Self(0x4);
    pub const TYPE_8X8_ARM: Self = Self(0x8);
}
impl fmt::Display for DataGraphOpticalFlowGridSizeFlagsARM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TYPE_1X1_ARM"),
            (2u64, "TYPE_2X2_ARM"),
            (4u64, "TYPE_4X4_ARM"),
            (8u64, "TYPE_8X8_ARM"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowImageUsageFlagsARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphOpticalFlowImageUsageFlagsARM(Flags);
bitflags!(DataGraphOpticalFlowImageUsageFlagsARM, Flags);
impl DataGraphOpticalFlowImageUsageFlagsARM {
    pub const UNKNOWN_ARM: Self = Self(0x0);
    pub const INPUT_ARM: Self = Self(0x1);
    pub const OUTPUT_ARM: Self = Self(0x2);
    pub const HINT_ARM: Self = Self(0x4);
    pub const COST_ARM: Self = Self(0x8);
}
impl fmt::Display for DataGraphOpticalFlowImageUsageFlagsARM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "INPUT_ARM"),
            (2u64, "OUTPUT_ARM"),
            (4u64, "HINT_ARM"),
            (8u64, "COST_ARM"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowCreateFlagsARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphOpticalFlowCreateFlagsARM(Flags);
bitflags!(DataGraphOpticalFlowCreateFlagsARM, Flags);
impl DataGraphOpticalFlowCreateFlagsARM {
    pub const ENABLE_HINT_ARM: Self = Self(0x1);
    pub const ENABLE_COST_ARM: Self = Self(0x2);
    pub const RESERVED_30_ARM: Self = Self(0x40000000);
}
impl fmt::Display for DataGraphOpticalFlowCreateFlagsARM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "ENABLE_HINT_ARM"),
            (2u64, "ENABLE_COST_ARM"),
            (1073741824u64, "RESERVED_30_ARM"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowExecuteFlagsARM.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphOpticalFlowExecuteFlagsARM(Flags);
bitflags!(DataGraphOpticalFlowExecuteFlagsARM, Flags);
impl DataGraphOpticalFlowExecuteFlagsARM {
    pub const DISABLE_TEMPORAL_HINTS_ARM: Self = Self(0x1);
    pub const INPUT_UNCHANGED_ARM: Self = Self(0x2);
    pub const REFERENCE_UNCHANGED_ARM: Self = Self(0x4);
    pub const INPUT_IS_PREVIOUS_REFERENCE_ARM: Self = Self(0x8);
    pub const REFERENCE_IS_PREVIOUS_INPUT_ARM: Self = Self(0x10);
}
impl fmt::Display for DataGraphOpticalFlowExecuteFlagsARM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "DISABLE_TEMPORAL_HINTS_ARM"),
            (2u64, "INPUT_UNCHANGED_ARM"),
            (4u64, "REFERENCE_UNCHANGED_ARM"),
            (8u64, "INPUT_IS_PREVIOUS_REFERENCE_ARM"),
            (16u64, "REFERENCE_IS_PREVIOUS_INPUT_ARM"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCodecOperationFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoCodecOperationFlagsKHR(Flags);
bitflags!(VideoCodecOperationFlagsKHR, Flags);
impl VideoCodecOperationFlagsKHR {
    pub const NONE_KHR: Self = Self(0x0);
    pub const ENCODE_H264_KHR: Self = Self(0x10000);
    pub const ENCODE_H265_KHR: Self = Self(0x20000);
    pub const DECODE_H264_KHR: Self = Self(0x1);
    pub const DECODE_H265_KHR: Self = Self(0x2);
    pub const DECODE_AV1_KHR: Self = Self(0x4);
    pub const ENCODE_AV1_KHR: Self = Self(0x40000);
    pub const DECODE_VP9_KHR: Self = Self(0x8);
}
impl fmt::Display for VideoCodecOperationFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (65536u64, "ENCODE_H264_KHR"),
            (131072u64, "ENCODE_H265_KHR"),
            (1u64, "DECODE_H264_KHR"),
            (2u64, "DECODE_H265_KHR"),
            (4u64, "DECODE_AV1_KHR"),
            (262144u64, "ENCODE_AV1_KHR"),
            (8u64, "DECODE_VP9_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCapabilityFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoCapabilityFlagsKHR(Flags);
bitflags!(VideoCapabilityFlagsKHR, Flags);
impl VideoCapabilityFlagsKHR {
    pub const PROTECTED_CONTENT_KHR: Self = Self(0x1);
    pub const SEPARATE_REFERENCE_IMAGES_KHR: Self = Self(0x2);
}
impl fmt::Display for VideoCapabilityFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "PROTECTED_CONTENT_KHR"),
            (2u64, "SEPARATE_REFERENCE_IMAGES_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionCreateFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoSessionCreateFlagsKHR(Flags);
bitflags!(VideoSessionCreateFlagsKHR, Flags);
impl VideoSessionCreateFlagsKHR {
    pub const PROTECTED_CONTENT_KHR: Self = Self(0x1);
    pub const ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR: Self = Self(0x2);
    pub const INLINE_QUERIES_KHR: Self = Self(0x4);
    pub const ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(0x8);
    pub const ALLOW_ENCODE_EMPHASIS_MAP_KHR: Self = Self(0x10);
    pub const INLINE_SESSION_PARAMETERS_KHR: Self = Self(0x20);
}
impl fmt::Display for VideoSessionCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "PROTECTED_CONTENT_KHR"),
            (2u64, "ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR"),
            (4u64, "INLINE_QUERIES_KHR"),
            (8u64, "ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR"),
            (16u64, "ALLOW_ENCODE_EMPHASIS_MAP_KHR"),
            (32u64, "INLINE_SESSION_PARAMETERS_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionParametersCreateFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoSessionParametersCreateFlagsKHR(Flags);
bitflags!(VideoSessionParametersCreateFlagsKHR, Flags);
impl VideoSessionParametersCreateFlagsKHR {
    pub const QUANTIZATION_MAP_COMPATIBLE_KHR: Self = Self(0x1);
}
impl fmt::Display for VideoSessionParametersCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[(1u64, "QUANTIZATION_MAP_COMPATIBLE_KHR")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoBeginCodingFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoBeginCodingFlagsKHR(Flags);
bitflags!(VideoBeginCodingFlagsKHR, Flags);
impl VideoBeginCodingFlagsKHR {}
impl fmt::Display for VideoBeginCodingFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEndCodingFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEndCodingFlagsKHR(Flags);
bitflags!(VideoEndCodingFlagsKHR, Flags);
impl VideoEndCodingFlagsKHR {}
impl fmt::Display for VideoEndCodingFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCodingControlFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoCodingControlFlagsKHR(Flags);
bitflags!(VideoCodingControlFlagsKHR, Flags);
impl VideoCodingControlFlagsKHR {
    pub const RESET_KHR: Self = Self(0x1);
    pub const ENCODE_RATE_CONTROL_KHR: Self = Self(0x2);
    pub const ENCODE_QUALITY_LEVEL_KHR: Self = Self(0x4);
}
impl fmt::Display for VideoCodingControlFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "RESET_KHR"),
            (2u64, "ENCODE_RATE_CONTROL_KHR"),
            (4u64, "ENCODE_QUALITY_LEVEL_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeUsageFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoDecodeUsageFlagsKHR(Flags);
bitflags!(VideoDecodeUsageFlagsKHR, Flags);
impl VideoDecodeUsageFlagsKHR {
    pub const DEFAULT_KHR: Self = Self(0x0);
    pub const TRANSCODING_KHR: Self = Self(0x1);
    pub const OFFLINE_KHR: Self = Self(0x2);
    pub const STREAMING_KHR: Self = Self(0x4);
}
impl fmt::Display for VideoDecodeUsageFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TRANSCODING_KHR"),
            (2u64, "OFFLINE_KHR"),
            (4u64, "STREAMING_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeCapabilityFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoDecodeCapabilityFlagsKHR(Flags);
bitflags!(VideoDecodeCapabilityFlagsKHR, Flags);
impl VideoDecodeCapabilityFlagsKHR {
    pub const DPB_AND_OUTPUT_COINCIDE_KHR: Self = Self(0x1);
    pub const DPB_AND_OUTPUT_DISTINCT_KHR: Self = Self(0x2);
}
impl fmt::Display for VideoDecodeCapabilityFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "DPB_AND_OUTPUT_COINCIDE_KHR"),
            (2u64, "DPB_AND_OUTPUT_DISTINCT_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoDecodeFlagsKHR(Flags);
bitflags!(VideoDecodeFlagsKHR, Flags);
impl VideoDecodeFlagsKHR {}
impl fmt::Display for VideoDecodeFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH264PictureLayoutFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoDecodeH264PictureLayoutFlagsKHR(Flags);
bitflags!(VideoDecodeH264PictureLayoutFlagsKHR, Flags);
impl VideoDecodeH264PictureLayoutFlagsKHR {
    pub const PROGRESSIVE_KHR: Self = Self(0x0);
    pub const INTERLACED_INTERLEAVED_LINES_KHR: Self = Self(0x1);
    pub const INTERLACED_SEPARATE_PLANES_KHR: Self = Self(0x2);
}
impl fmt::Display for VideoDecodeH264PictureLayoutFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "INTERLACED_INTERLEAVED_LINES_KHR"),
            (2u64, "INTERLACED_SEPARATE_PLANES_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeFlagsKHR(Flags);
bitflags!(VideoEncodeFlagsKHR, Flags);
impl VideoEncodeFlagsKHR {
    pub const INTRA_REFRESH_KHR: Self = Self(0x4);
    pub const WITH_QUANTIZATION_DELTA_MAP_KHR: Self = Self(0x1);
    pub const WITH_EMPHASIS_MAP_KHR: Self = Self(0x2);
}
impl fmt::Display for VideoEncodeFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (4u64, "INTRA_REFRESH_KHR"),
            (1u64, "WITH_QUANTIZATION_DELTA_MAP_KHR"),
            (2u64, "WITH_EMPHASIS_MAP_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeUsageFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeUsageFlagsKHR(Flags);
bitflags!(VideoEncodeUsageFlagsKHR, Flags);
impl VideoEncodeUsageFlagsKHR {
    pub const DEFAULT_KHR: Self = Self(0x0);
    pub const TRANSCODING_KHR: Self = Self(0x1);
    pub const STREAMING_KHR: Self = Self(0x2);
    pub const RECORDING_KHR: Self = Self(0x4);
    pub const CONFERENCING_KHR: Self = Self(0x8);
}
impl fmt::Display for VideoEncodeUsageFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TRANSCODING_KHR"),
            (2u64, "STREAMING_KHR"),
            (4u64, "RECORDING_KHR"),
            (8u64, "CONFERENCING_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeContentFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeContentFlagsKHR(Flags);
bitflags!(VideoEncodeContentFlagsKHR, Flags);
impl VideoEncodeContentFlagsKHR {
    pub const DEFAULT_KHR: Self = Self(0x0);
    pub const CAMERA_KHR: Self = Self(0x1);
    pub const DESKTOP_KHR: Self = Self(0x2);
    pub const RENDERED_KHR: Self = Self(0x4);
}
impl fmt::Display for VideoEncodeContentFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "CAMERA_KHR"),
            (2u64, "DESKTOP_KHR"),
            (4u64, "RENDERED_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeCapabilityFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeCapabilityFlagsKHR(Flags);
bitflags!(VideoEncodeCapabilityFlagsKHR, Flags);
impl VideoEncodeCapabilityFlagsKHR {
    pub const PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR: Self = Self(0x1);
    pub const INSUFFICIENTSTREAM_BUFFER_RANGE_DETECTION_KHR: Self = Self(0x2);
    pub const QUANTIZATION_DELTA_MAP_KHR: Self = Self(0x4);
    pub const EMPHASIS_MAP_KHR: Self = Self(0x8);
}
impl fmt::Display for VideoEncodeCapabilityFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR"),
            (2u64, "INSUFFICIENTSTREAM_BUFFER_RANGE_DETECTION_KHR"),
            (4u64, "QUANTIZATION_DELTA_MAP_KHR"),
            (8u64, "EMPHASIS_MAP_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeFeedbackFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeFeedbackFlagsKHR(Flags);
bitflags!(VideoEncodeFeedbackFlagsKHR, Flags);
impl VideoEncodeFeedbackFlagsKHR {
    pub const VIDEO_ENCODE_FEEDBACKSTREAM_BUFFER_OFFSET_KHR: Self = Self(0x1);
    pub const VIDEO_ENCODE_FEEDBACKSTREAM_BYTES_WRITTEN_KHR: Self = Self(0x2);
    pub const VIDEO_ENCODE_FEEDBACKSTREAM_HAS_OVERRIDES_KHR: Self = Self(0x4);
}
impl fmt::Display for VideoEncodeFeedbackFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "VIDEO_ENCODE_FEEDBACKSTREAM_BUFFER_OFFSET_KHR"),
            (2u64, "VIDEO_ENCODE_FEEDBACKSTREAM_BYTES_WRITTEN_KHR"),
            (4u64, "VIDEO_ENCODE_FEEDBACKSTREAM_HAS_OVERRIDES_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRateControlFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeRateControlFlagsKHR(Flags);
bitflags!(VideoEncodeRateControlFlagsKHR, Flags);
impl VideoEncodeRateControlFlagsKHR {}
impl fmt::Display for VideoEncodeRateControlFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRateControlModeFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeRateControlModeFlagsKHR(Flags);
bitflags!(VideoEncodeRateControlModeFlagsKHR, Flags);
impl VideoEncodeRateControlModeFlagsKHR {
    pub const DEFAULT_KHR: Self = Self(0x0);
    pub const DISABLED_KHR: Self = Self(0x1);
    pub const CBR_KHR: Self = Self(0x2);
    pub const VBR_KHR: Self = Self(0x4);
}
impl fmt::Display for VideoEncodeRateControlModeFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] =
            &[(1u64, "DISABLED_KHR"), (2u64, "CBR_KHR"), (4u64, "VBR_KHR")];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeIntraRefreshModeFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeIntraRefreshModeFlagsKHR(Flags);
bitflags!(VideoEncodeIntraRefreshModeFlagsKHR, Flags);
impl VideoEncodeIntraRefreshModeFlagsKHR {
    pub const NONE_KHR: Self = Self(0x0);
    pub const PER_PICTURE_PARTITION_KHR: Self = Self(0x1);
    pub const BLOCK_BASED_KHR: Self = Self(0x2);
    pub const BLOCK_ROW_BASED_KHR: Self = Self(0x4);
    pub const BLOCK_COLUMN_BASED_KHR: Self = Self(0x8);
}
impl fmt::Display for VideoEncodeIntraRefreshModeFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "PER_PICTURE_PARTITION_KHR"),
            (2u64, "BLOCK_BASED_KHR"),
            (4u64, "BLOCK_ROW_BASED_KHR"),
            (8u64, "BLOCK_COLUMN_BASED_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoChromaSubsamplingFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoChromaSubsamplingFlagsKHR(Flags);
bitflags!(VideoChromaSubsamplingFlagsKHR, Flags);
impl VideoChromaSubsamplingFlagsKHR {
    pub const INVALID_KHR: Self = Self(0x0);
    pub const MONOCHROME_KHR: Self = Self(0x1);
    pub const TYPE_420_KHR: Self = Self(0x2);
    pub const TYPE_422_KHR: Self = Self(0x4);
    pub const TYPE_444_KHR: Self = Self(0x8);
}
impl fmt::Display for VideoChromaSubsamplingFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "MONOCHROME_KHR"),
            (2u64, "TYPE_420_KHR"),
            (4u64, "TYPE_422_KHR"),
            (8u64, "TYPE_444_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoComponentBitDepthFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoComponentBitDepthFlagsKHR(Flags);
bitflags!(VideoComponentBitDepthFlagsKHR, Flags);
impl VideoComponentBitDepthFlagsKHR {
    pub const VIDEO_COMPONENT_DEPTH_INVALID_KHR: Self = Self(0x0);
    pub const VIDEO_COMPONENT_DEPTH_8_KHR: Self = Self(0x1);
    pub const VIDEO_COMPONENT_DEPTH_10_KHR: Self = Self(0x4);
    pub const VIDEO_COMPONENT_DEPTH_12_KHR: Self = Self(0x10);
}
impl fmt::Display for VideoComponentBitDepthFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "VIDEO_COMPONENT_DEPTH_8_KHR"),
            (4u64, "VIDEO_COMPONENT_DEPTH_10_KHR"),
            (16u64, "VIDEO_COMPONENT_DEPTH_12_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264CapabilityFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeH264CapabilityFlagsKHR(Flags);
bitflags!(VideoEncodeH264CapabilityFlagsKHR, Flags);
impl VideoEncodeH264CapabilityFlagsKHR {
    pub const HRD_COMPLIANCE_KHR: Self = Self(0x1);
    pub const PREDICTION_WEIGHT_TABLE_GENERATED_KHR: Self = Self(0x2);
    pub const ROW_UNALIGNED_SLICE_KHR: Self = Self(0x4);
    pub const DIFFERENT_SLICE_TYPE_KHR: Self = Self(0x8);
    pub const B_FRAME_IN_L0_LIST_KHR: Self = Self(0x10);
    pub const B_FRAME_IN_L1_LIST_KHR: Self = Self(0x20);
    pub const PER_PICTURE_TYPE_MIN_MAX_QP_KHR: Self = Self(0x40);
    pub const PER_SLICE_CONSTANT_QP_KHR: Self = Self(0x80);
    pub const GENERATE_PREFIX_NALU_KHR: Self = Self(0x100);
    pub const B_PICTURE_INTRA_REFRESH_KHR: Self = Self(0x400);
    pub const MB_QP_DIFF_WRAPAROUND_KHR: Self = Self(0x200);
}
impl fmt::Display for VideoEncodeH264CapabilityFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "HRD_COMPLIANCE_KHR"),
            (2u64, "PREDICTION_WEIGHT_TABLE_GENERATED_KHR"),
            (4u64, "ROW_UNALIGNED_SLICE_KHR"),
            (8u64, "DIFFERENT_SLICE_TYPE_KHR"),
            (16u64, "B_FRAME_IN_L0_LIST_KHR"),
            (32u64, "B_FRAME_IN_L1_LIST_KHR"),
            (64u64, "PER_PICTURE_TYPE_MIN_MAX_QP_KHR"),
            (128u64, "PER_SLICE_CONSTANT_QP_KHR"),
            (256u64, "GENERATE_PREFIX_NALU_KHR"),
            (1024u64, "B_PICTURE_INTRA_REFRESH_KHR"),
            (512u64, "MB_QP_DIFF_WRAPAROUND_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264StdFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeH264StdFlagsKHR(Flags);
bitflags!(VideoEncodeH264StdFlagsKHR, Flags);
impl VideoEncodeH264StdFlagsKHR {
    pub const SEPARATE_COLOR_PLANE_FLAG_SET_KHR: Self = Self(0x1);
    pub const QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR: Self = Self(0x2);
    pub const SCALING_MATRIX_PRESENT_FLAG_SET_KHR: Self = Self(0x4);
    pub const CHROMA_QP_INDEX_OFFSET_KHR: Self = Self(0x8);
    pub const SECOND_CHROMA_QP_INDEX_OFFSET_KHR: Self = Self(0x10);
    pub const PIC_INIT_QP_MINUS26_KHR: Self = Self(0x20);
    pub const WEIGHTED_PRED_FLAG_SET_KHR: Self = Self(0x40);
    pub const WEIGHTED_BIPRED_IDC_EXPLICIT_KHR: Self = Self(0x80);
    pub const WEIGHTED_BIPRED_IDC_IMPLICIT_KHR: Self = Self(0x100);
    pub const TRANSFORM_8X8_MODE_FLAG_SET_KHR: Self = Self(0x200);
    pub const DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR: Self = Self(0x400);
    pub const ENTROPY_CODING_MODE_FLAG_UNSET_KHR: Self = Self(0x800);
    pub const ENTROPY_CODING_MODE_FLAG_SET_KHR: Self = Self(0x1000);
    pub const DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR: Self = Self(0x2000);
    pub const CONSTRAINED_INTRA_PRED_FLAG_SET_KHR: Self = Self(0x4000);
    pub const DEBLOCKING_FILTER_DISABLED_KHR: Self = Self(0x8000);
    pub const DEBLOCKING_FILTER_ENABLED_KHR: Self = Self(0x10000);
    pub const DEBLOCKING_FILTER_PARTIAL_KHR: Self = Self(0x20000);
    pub const SLICE_QP_DELTA_KHR: Self = Self(0x80000);
    pub const DIFFERENT_SLICE_QP_DELTA_KHR: Self = Self(0x100000);
}
impl fmt::Display for VideoEncodeH264StdFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "SEPARATE_COLOR_PLANE_FLAG_SET_KHR"),
            (2u64, "QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR"),
            (4u64, "SCALING_MATRIX_PRESENT_FLAG_SET_KHR"),
            (8u64, "CHROMA_QP_INDEX_OFFSET_KHR"),
            (16u64, "SECOND_CHROMA_QP_INDEX_OFFSET_KHR"),
            (32u64, "PIC_INIT_QP_MINUS26_KHR"),
            (64u64, "WEIGHTED_PRED_FLAG_SET_KHR"),
            (128u64, "WEIGHTED_BIPRED_IDC_EXPLICIT_KHR"),
            (256u64, "WEIGHTED_BIPRED_IDC_IMPLICIT_KHR"),
            (512u64, "TRANSFORM_8X8_MODE_FLAG_SET_KHR"),
            (1024u64, "DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR"),
            (2048u64, "ENTROPY_CODING_MODE_FLAG_UNSET_KHR"),
            (4096u64, "ENTROPY_CODING_MODE_FLAG_SET_KHR"),
            (8192u64, "DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR"),
            (16384u64, "CONSTRAINED_INTRA_PRED_FLAG_SET_KHR"),
            (32768u64, "DEBLOCKING_FILTER_DISABLED_KHR"),
            (65536u64, "DEBLOCKING_FILTER_ENABLED_KHR"),
            (131072u64, "DEBLOCKING_FILTER_PARTIAL_KHR"),
            (524288u64, "SLICE_QP_DELTA_KHR"),
            (1048576u64, "DIFFERENT_SLICE_QP_DELTA_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264RateControlFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeH264RateControlFlagsKHR(Flags);
bitflags!(VideoEncodeH264RateControlFlagsKHR, Flags);
impl VideoEncodeH264RateControlFlagsKHR {
    pub const ATTEMPT_HRD_COMPLIANCE_KHR: Self = Self(0x1);
    pub const REGULAR_GOP_KHR: Self = Self(0x2);
    pub const REFERENCE_PATTERN_FLAT_KHR: Self = Self(0x4);
    pub const REFERENCE_PATTERN_DYADIC_KHR: Self = Self(0x8);
    pub const TEMPORAL_LAYER_PATTERN_DYADIC_KHR: Self = Self(0x10);
}
impl fmt::Display for VideoEncodeH264RateControlFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "ATTEMPT_HRD_COMPLIANCE_KHR"),
            (2u64, "REGULAR_GOP_KHR"),
            (4u64, "REFERENCE_PATTERN_FLAT_KHR"),
            (8u64, "REFERENCE_PATTERN_DYADIC_KHR"),
            (16u64, "TEMPORAL_LAYER_PATTERN_DYADIC_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265CapabilityFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeH265CapabilityFlagsKHR(Flags);
bitflags!(VideoEncodeH265CapabilityFlagsKHR, Flags);
impl VideoEncodeH265CapabilityFlagsKHR {
    pub const HRD_COMPLIANCE_KHR: Self = Self(0x1);
    pub const PREDICTION_WEIGHT_TABLE_GENERATED_KHR: Self = Self(0x2);
    pub const ROW_UNALIGNED_SLICE_SEGMENT_KHR: Self = Self(0x4);
    pub const DIFFERENT_SLICE_SEGMENT_TYPE_KHR: Self = Self(0x8);
    pub const B_FRAME_IN_L0_LIST_KHR: Self = Self(0x10);
    pub const B_FRAME_IN_L1_LIST_KHR: Self = Self(0x20);
    pub const PER_PICTURE_TYPE_MIN_MAX_QP_KHR: Self = Self(0x40);
    pub const PER_SLICE_SEGMENT_CONSTANT_QP_KHR: Self = Self(0x80);
    pub const MULTIPLE_TILES_PER_SLICE_SEGMENT_KHR: Self = Self(0x100);
    pub const MULTIPLE_SLICE_SEGMENTS_PER_TILE_KHR: Self = Self(0x200);
    pub const B_PICTURE_INTRA_REFRESH_KHR: Self = Self(0x800);
    pub const CU_QP_DIFF_WRAPAROUND_KHR: Self = Self(0x400);
}
impl fmt::Display for VideoEncodeH265CapabilityFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "HRD_COMPLIANCE_KHR"),
            (2u64, "PREDICTION_WEIGHT_TABLE_GENERATED_KHR"),
            (4u64, "ROW_UNALIGNED_SLICE_SEGMENT_KHR"),
            (8u64, "DIFFERENT_SLICE_SEGMENT_TYPE_KHR"),
            (16u64, "B_FRAME_IN_L0_LIST_KHR"),
            (32u64, "B_FRAME_IN_L1_LIST_KHR"),
            (64u64, "PER_PICTURE_TYPE_MIN_MAX_QP_KHR"),
            (128u64, "PER_SLICE_SEGMENT_CONSTANT_QP_KHR"),
            (256u64, "MULTIPLE_TILES_PER_SLICE_SEGMENT_KHR"),
            (512u64, "MULTIPLE_SLICE_SEGMENTS_PER_TILE_KHR"),
            (2048u64, "B_PICTURE_INTRA_REFRESH_KHR"),
            (1024u64, "CU_QP_DIFF_WRAPAROUND_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265StdFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeH265StdFlagsKHR(Flags);
bitflags!(VideoEncodeH265StdFlagsKHR, Flags);
impl VideoEncodeH265StdFlagsKHR {
    pub const SEPARATE_COLOR_PLANE_FLAG_SET_KHR: Self = Self(0x1);
    pub const SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_KHR: Self = Self(0x2);
    pub const SCALING_LIST_DATA_PRESENT_FLAG_SET_KHR: Self = Self(0x4);
    pub const PCM_ENABLED_FLAG_SET_KHR: Self = Self(0x8);
    pub const SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_KHR: Self = Self(0x10);
    pub const INIT_QP_MINUS26_KHR: Self = Self(0x20);
    pub const WEIGHTED_PRED_FLAG_SET_KHR: Self = Self(0x40);
    pub const WEIGHTED_BIPRED_FLAG_SET_KHR: Self = Self(0x80);
    pub const LOG2_PARALLEL_MERGE_LEVEL_MINUS2_KHR: Self = Self(0x100);
    pub const SIGN_DATA_HIDING_ENABLED_FLAG_SET_KHR: Self = Self(0x200);
    pub const TRANSFORM_SKIP_ENABLED_FLAG_SET_KHR: Self = Self(0x400);
    pub const TRANSFORM_SKIP_ENABLED_FLAG_UNSET_KHR: Self = Self(0x800);
    pub const PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_KHR: Self = Self(0x1000);
    pub const TRANSQUANT_BYPASS_ENABLED_FLAG_SET_KHR: Self = Self(0x2000);
    pub const CONSTRAINED_INTRA_PRED_FLAG_SET_KHR: Self = Self(0x4000);
    pub const ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_KHR: Self = Self(0x8000);
    pub const DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_KHR: Self = Self(0x10000);
    pub const DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_KHR: Self = Self(0x20000);
    pub const DEPENDENT_SLICE_SEGMENT_FLAG_SET_KHR: Self = Self(0x40000);
    pub const SLICE_QP_DELTA_KHR: Self = Self(0x80000);
    pub const DIFFERENT_SLICE_QP_DELTA_KHR: Self = Self(0x100000);
}
impl fmt::Display for VideoEncodeH265StdFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "SEPARATE_COLOR_PLANE_FLAG_SET_KHR"),
            (2u64, "SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_KHR"),
            (4u64, "SCALING_LIST_DATA_PRESENT_FLAG_SET_KHR"),
            (8u64, "PCM_ENABLED_FLAG_SET_KHR"),
            (16u64, "SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_KHR"),
            (32u64, "INIT_QP_MINUS26_KHR"),
            (64u64, "WEIGHTED_PRED_FLAG_SET_KHR"),
            (128u64, "WEIGHTED_BIPRED_FLAG_SET_KHR"),
            (256u64, "LOG2_PARALLEL_MERGE_LEVEL_MINUS2_KHR"),
            (512u64, "SIGN_DATA_HIDING_ENABLED_FLAG_SET_KHR"),
            (1024u64, "TRANSFORM_SKIP_ENABLED_FLAG_SET_KHR"),
            (2048u64, "TRANSFORM_SKIP_ENABLED_FLAG_UNSET_KHR"),
            (4096u64, "PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_KHR"),
            (8192u64, "TRANSQUANT_BYPASS_ENABLED_FLAG_SET_KHR"),
            (16384u64, "CONSTRAINED_INTRA_PRED_FLAG_SET_KHR"),
            (32768u64, "ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_KHR"),
            (65536u64, "DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_KHR"),
            (131072u64, "DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_KHR"),
            (262144u64, "DEPENDENT_SLICE_SEGMENT_FLAG_SET_KHR"),
            (524288u64, "SLICE_QP_DELTA_KHR"),
            (1048576u64, "DIFFERENT_SLICE_QP_DELTA_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265RateControlFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeH265RateControlFlagsKHR(Flags);
bitflags!(VideoEncodeH265RateControlFlagsKHR, Flags);
impl VideoEncodeH265RateControlFlagsKHR {
    pub const ATTEMPT_HRD_COMPLIANCE_KHR: Self = Self(0x1);
    pub const REGULAR_GOP_KHR: Self = Self(0x2);
    pub const REFERENCE_PATTERN_FLAT_KHR: Self = Self(0x4);
    pub const REFERENCE_PATTERN_DYADIC_KHR: Self = Self(0x8);
    pub const TEMPORAL_SUB_LAYER_PATTERN_DYADIC_KHR: Self = Self(0x10);
}
impl fmt::Display for VideoEncodeH265RateControlFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "ATTEMPT_HRD_COMPLIANCE_KHR"),
            (2u64, "REGULAR_GOP_KHR"),
            (4u64, "REFERENCE_PATTERN_FLAT_KHR"),
            (8u64, "REFERENCE_PATTERN_DYADIC_KHR"),
            (16u64, "TEMPORAL_SUB_LAYER_PATTERN_DYADIC_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265CtbSizeFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeH265CtbSizeFlagsKHR(Flags);
bitflags!(VideoEncodeH265CtbSizeFlagsKHR, Flags);
impl VideoEncodeH265CtbSizeFlagsKHR {
    pub const TYPE_16_KHR: Self = Self(0x1);
    pub const TYPE_32_KHR: Self = Self(0x2);
    pub const TYPE_64_KHR: Self = Self(0x4);
}
impl fmt::Display for VideoEncodeH265CtbSizeFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TYPE_16_KHR"),
            (2u64, "TYPE_32_KHR"),
            (4u64, "TYPE_64_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265TransformBlockSizeFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeH265TransformBlockSizeFlagsKHR(Flags);
bitflags!(VideoEncodeH265TransformBlockSizeFlagsKHR, Flags);
impl VideoEncodeH265TransformBlockSizeFlagsKHR {
    pub const TYPE_4_KHR: Self = Self(0x1);
    pub const TYPE_8_KHR: Self = Self(0x2);
    pub const TYPE_16_KHR: Self = Self(0x4);
    pub const TYPE_32_KHR: Self = Self(0x8);
}
impl fmt::Display for VideoEncodeH265TransformBlockSizeFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "TYPE_4_KHR"),
            (2u64, "TYPE_8_KHR"),
            (4u64, "TYPE_16_KHR"),
            (8u64, "TYPE_32_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1CapabilityFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeAV1CapabilityFlagsKHR(Flags);
bitflags!(VideoEncodeAV1CapabilityFlagsKHR, Flags);
impl VideoEncodeAV1CapabilityFlagsKHR {
    pub const VIDEO_ENCODE_AV1_CAPABILITY_PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_KHR: Self =
        Self(0x1);
    pub const VIDEO_ENCODE_AV1_CAPABILITY_GENERATE_OBU_EXTENSION_HEADER_KHR: Self = Self(0x2);
    pub const VIDEO_ENCODE_AV1_CAPABILITY_PRIMARY_REFERENCE_CDF_ONLY_KHR: Self = Self(0x4);
    pub const VIDEO_ENCODE_AV1_CAPABILITY_FRAME_SIZE_OVERRIDE_KHR: Self = Self(0x8);
    pub const VIDEO_ENCODE_AV1_CAPABILITY_MOTION_VECTOR_SCALING_KHR: Self = Self(0x10);
    pub const VIDEO_ENCODE_AV1_CAPABILITY_COMPOUND_PREDICTION_INTRA_REFRESH_KHR: Self = Self(0x20);
}
impl fmt::Display for VideoEncodeAV1CapabilityFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (
                1u64,
                "VIDEO_ENCODE_AV1_CAPABILITY_PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_KHR",
            ),
            (
                2u64,
                "VIDEO_ENCODE_AV1_CAPABILITY_GENERATE_OBU_EXTENSION_HEADER_KHR",
            ),
            (
                4u64,
                "VIDEO_ENCODE_AV1_CAPABILITY_PRIMARY_REFERENCE_CDF_ONLY_KHR",
            ),
            (8u64, "VIDEO_ENCODE_AV1_CAPABILITY_FRAME_SIZE_OVERRIDE_KHR"),
            (
                16u64,
                "VIDEO_ENCODE_AV1_CAPABILITY_MOTION_VECTOR_SCALING_KHR",
            ),
            (
                32u64,
                "VIDEO_ENCODE_AV1_CAPABILITY_COMPOUND_PREDICTION_INTRA_REFRESH_KHR",
            ),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1StdFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeAV1StdFlagsKHR(Flags);
bitflags!(VideoEncodeAV1StdFlagsKHR, Flags);
impl VideoEncodeAV1StdFlagsKHR {
    pub const VIDEO_ENCODE_AV1_STD_UNIFORM_TILE_SPACING_FLAG_SET_KHR: Self = Self(0x1);
    pub const VIDEO_ENCODE_AV1_STD_SKIP_MODE_PRESENT_UNSET_KHR: Self = Self(0x2);
    pub const VIDEO_ENCODE_AV1_STD_PRIMARY_REF_FRAME_KHR: Self = Self(0x4);
    pub const VIDEO_ENCODE_AV1_STD_DELTA_Q_KHR: Self = Self(0x8);
}
impl fmt::Display for VideoEncodeAV1StdFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (
                1u64,
                "VIDEO_ENCODE_AV1_STD_UNIFORM_TILE_SPACING_FLAG_SET_KHR",
            ),
            (2u64, "VIDEO_ENCODE_AV1_STD_SKIP_MODE_PRESENT_UNSET_KHR"),
            (4u64, "VIDEO_ENCODE_AV1_STD_PRIMARY_REF_FRAME_KHR"),
            (8u64, "VIDEO_ENCODE_AV1_STD_DELTA_Q_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1RateControlFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeAV1RateControlFlagsKHR(Flags);
bitflags!(VideoEncodeAV1RateControlFlagsKHR, Flags);
impl VideoEncodeAV1RateControlFlagsKHR {
    pub const VIDEO_ENCODE_AV1_RATE_CONTROL_REGULAR_GOP_KHR: Self = Self(0x1);
    pub const VIDEO_ENCODE_AV1_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_KHR: Self = Self(0x2);
    pub const VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_FLAT_KHR: Self = Self(0x4);
    pub const VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_KHR: Self = Self(0x8);
}
impl fmt::Display for VideoEncodeAV1RateControlFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "VIDEO_ENCODE_AV1_RATE_CONTROL_REGULAR_GOP_KHR"),
            (
                2u64,
                "VIDEO_ENCODE_AV1_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_KHR",
            ),
            (
                4u64,
                "VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_FLAT_KHR",
            ),
            (
                8u64,
                "VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_KHR",
            ),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1SuperblockSizeFlagsKHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeAV1SuperblockSizeFlagsKHR(Flags);
bitflags!(VideoEncodeAV1SuperblockSizeFlagsKHR, Flags);
impl VideoEncodeAV1SuperblockSizeFlagsKHR {
    pub const VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_64_KHR: Self = Self(0x1);
    pub const VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_128_KHR: Self = Self(0x2);
}
impl fmt::Display for VideoEncodeAV1SuperblockSizeFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[
            (1u64, "VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_64_KHR"),
            (2u64, "VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_128_KHR"),
        ];
        flag_display(self.0 as u64, BITS, f)
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlags3KHR.html>"]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccessFlags3KHR(Flags64);
bitflags!(AccessFlags3KHR, Flags64);
impl AccessFlags3KHR {
    pub const NONE_KHR: Self = Self(0x0);
}
impl fmt::Display for AccessFlags3KHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "NONE")?;
        }
        const BITS: &[(u64, &str)] = &[];
        flag_display(self.0, BITS, f)
    }
}

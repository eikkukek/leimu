use core::fmt::{self, Display};

use tuhka::vk;

use leimu_spirv::op;

use crate::bitflags;
use crate::gpu::prelude::*;

bitflags! {
    /// A bitmask of [`shader stages`][1].
    ///
    /// [1]: ShaderStage
    #[default = Self::empty()]
    pub struct ShaderStageFlags: Flags32 {
        VERTEX = vk::ShaderStageFlags::VERTEX.as_raw(),
        TESSELLATION_CONTROL = vk::ShaderStageFlags::TESSELLATION_CONTROL.as_raw(),
        TESSELLATION_EVALUATION = vk::ShaderStageFlags::TESSELLATION_EVALUATION.as_raw(),
        GEOMETRY = vk::ShaderStageFlags::GEOMETRY.as_raw(),
        FRAGMENT = vk::ShaderStageFlags::FRAGMENT.as_raw(),
        COMPUTE = vk::ShaderStageFlags::COMPUTE.as_raw(),
        /// Provided by VK_KHR_ray_tracing_pipeline.
        RAYGEN_KHR = vk::ShaderStageFlags::RAYGEN_KHR.as_raw(),
        /// Provided by VK_KHR_ray_tracing_pipeline.
        ANY_HIT_KHR = vk::ShaderStageFlags::ANY_HIT_KHR.as_raw(),
        /// Provided by VK_KHR_ray_tracing_pipeline.
        CLOSEST_HIT_KHR = vk::ShaderStageFlags::CLOSEST_HIT_KHR.as_raw(),
        /// Provided by VK_KHR_ray_tracing_pipeline.
        MISS_KHR = vk::ShaderStageFlags::MISS_KHR.as_raw(),
        /// Provided by VK_KHR_ray_tracing_pipeline.
        INTERSECTION_KHR = vk::ShaderStageFlags::INTERSECTION_KHR.as_raw(),
        /// Provided by VK_KHR_ray_tracing_pipeline.
        CALLABLE_KHR = vk::ShaderStageFlags::CALLABLE_KHR.as_raw(),
        /// Provided by [`mesh_shader`][ext::mesh_shader].
        TASK_EXT = vk::ShaderStageFlags::TASK_EXT.as_raw(),
        /// Provided by [`mesh_shader`][ext::mesh_shader].
        MESH_EXT = vk::ShaderStageFlags::MESH_EXT.as_raw(),
        /// Provided by VK_HUAWEI_subpass_shading
        SUBPASS_SHADING_HUAWEI = vk::ShaderStageFlags::SUBPASS_SHADING_HUAWEI.as_raw(),
        /// Provided by VK_HUAWEI_subpass_shading
        CLUSTER_CULLING_HUAWEI = vk::ShaderStageFlags::CLUSTER_CULLING_HUAWEI.as_raw(),
    }
}

impl ShaderStageFlags {

    #[inline(always)]
    pub fn pipeline_stage_mask(self) -> vk::PipelineStageFlags2 {
        let mut mask = vk::PipelineStageFlags2::empty();
        if self.contains(Self::VERTEX) {
            mask |= vk::PipelineStageFlags2::VERTEX_SHADER;
        }
        if self.contains(Self::TESSELLATION_CONTROL) {
            mask |= vk::PipelineStageFlags2::TESSELLATION_CONTROL_SHADER;
        }
        if self.contains(Self::TESSELLATION_EVALUATION) {
            mask |= vk::PipelineStageFlags2::TESSELLATION_EVALUATION_SHADER;
        }
        if self.contains(Self::TASK_EXT) {
            mask |= vk::PipelineStageFlags2::TASK_SHADER_EXT;
        }
        if self.contains(Self::MESH_EXT) {
            mask |= vk::PipelineStageFlags2::MESH_SHADER_EXT;
        }
        if self.contains(Self::GEOMETRY) {
            mask |= vk::PipelineStageFlags2::GEOMETRY_SHADER;
        }
        if self.contains(Self::FRAGMENT) {
            mask |= vk::PipelineStageFlags2::FRAGMENT_SHADER;
        }
        if self.contains(Self::COMPUTE) {
            mask |= vk::PipelineStageFlags2::COMPUTE_SHADER;
        }
        mask
    }
}

/// An enumeration of all supported shader stages.
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderStage {
    Vertex = ShaderStageFlags::VERTEX.as_raw(),
    TessellationControl = ShaderStageFlags::TESSELLATION_CONTROL.as_raw(),
    TessellationEvaluation = ShaderStageFlags::TESSELLATION_EVALUATION.as_raw(),
    Geometry = ShaderStageFlags::GEOMETRY.as_raw(),
    Fragment = ShaderStageFlags::FRAGMENT.as_raw(),
    Compute = ShaderStageFlags::COMPUTE.as_raw(),
    /// Provided by VK_KHR_ray_tracing_pipeline.
    RaygenKHR = vk::ShaderStageFlags::RAYGEN_KHR.as_raw(),
    /// Provided by VK_KHR_ray_tracing_pipeline.
    AnyHitKHR = vk::ShaderStageFlags::ANY_HIT_KHR.as_raw(),
    /// Provided by VK_KHR_ray_tracing_pipeline.
    ClosestHitKHR = vk::ShaderStageFlags::CLOSEST_HIT_KHR.as_raw(),
    /// Provided by VK_KHR_ray_tracing_pipeline.
    MissKHR = vk::ShaderStageFlags::MISS_KHR.as_raw(),
    /// Provided by VK_KHR_ray_tracing_pipeline.
    IntersectionKHR = vk::ShaderStageFlags::INTERSECTION_KHR.as_raw(),
    /// Provided by VK_KHR_ray_tracing_pipeline.
    CallableKHR = vk::ShaderStageFlags::CALLABLE_KHR.as_raw(),
    /// Provided by [`mesh_shader`][ext::mesh_shader].
    TaskEXT = vk::ShaderStageFlags::TASK_EXT.as_raw(),
    /// Provided by [`mesh_shader`][ext::mesh_shader].
    MeshEXT = vk::ShaderStageFlags::MESH_EXT.as_raw(),
}

impl Display for ShaderStage {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vertex => write!(f, "vertex"),
            Self::TessellationControl => write!(f, "tessellation control"),
            Self::TessellationEvaluation => write!(f, "tessellation evaluation"),
            Self::Geometry => write!(f, "geometry"),
            Self::Fragment => write!(f, "fragment"),
            Self::Compute => write!(f, "compute"),
            Self::RaygenKHR => write!(f, "raygen"),
            Self::AnyHitKHR => write!(f, "any hit"),
            Self::ClosestHitKHR => write!(f, "closest hit"),
            Self::MissKHR => write!(f, "miss"),
            Self::IntersectionKHR => write!(f, "intersection"),
            Self::CallableKHR => write!(f, "callable"),
            Self::TaskEXT => write!(f, "task"),
            Self::MeshEXT => write!(f, "mesh"),
        }
    }
}

impl ShaderStage {

    #[inline]
    pub fn as_raw(self) -> u32 {
        self as u32
    }
}

impl From<ShaderStage> for ShaderStageFlags {

    #[inline(always)]
    fn from(value: ShaderStage) -> Self {
        Self::from_raw(value.as_raw())
    }
}

impl From<ShaderStage> for vk::ShaderStageFlags {
    
    #[inline(always)]
    fn from(value: ShaderStage) -> Self {
        Self::from_raw(value.as_raw())
    }
}

impl From<ShaderStageFlags> for vk::ShaderStageFlags {

    #[inline(always)]
    fn from(value: ShaderStageFlags) -> Self {
        Self::from_raw(value.as_raw())
    }
}

impl From<ShaderStage> for shaderc::ShaderKind {

    fn from(value: ShaderStage) -> Self {
        match value {
            ShaderStage::Vertex => Self::Vertex,
            ShaderStage::TessellationControl => Self::TessControl,
            ShaderStage::TessellationEvaluation => Self::TessEvaluation,
            ShaderStage::Geometry => Self::Geometry,
            ShaderStage::Fragment => Self::Fragment,
            ShaderStage::Compute => Self::Compute,
            ShaderStage::RaygenKHR => Self::RayGeneration,
            ShaderStage::AnyHitKHR => Self::AnyHit,
            ShaderStage::MissKHR => Self::Miss,
            ShaderStage::ClosestHitKHR => Self::ClosestHit,
            ShaderStage::IntersectionKHR => Self::Intersection,
            ShaderStage::CallableKHR => Self::Callable,
            ShaderStage::TaskEXT => Self::Task,
            ShaderStage::MeshEXT => Self::Mesh,
        }
    }
}

impl From<ShaderStage> for op::ExecutionModel {

    #[inline]
    fn from(value: ShaderStage) -> Self {
        match value {
            ShaderStage::Vertex => Self::VERTEX,
            ShaderStage::TessellationControl => Self::TESSELLATION_CONTROL,
            ShaderStage::TessellationEvaluation => Self::TESSELLATION_EVALUATION,
            ShaderStage::Geometry => Self::GEOMETRY,
            ShaderStage::Fragment => Self::FRAGMENT,
            ShaderStage::Compute => Self::GLCOMPUTE,
            ShaderStage::RaygenKHR => Self::RAY_GENERATION_KHR,
            ShaderStage::AnyHitKHR => Self::ANY_HIT_KHR,
            ShaderStage::MissKHR => Self::MISS_KHR,
            ShaderStage::ClosestHitKHR => Self::CLOSEST_HIT_KHR,
            ShaderStage::IntersectionKHR => Self::INTERSECTION_KHR,
            ShaderStage::CallableKHR => Self::CALLABLE_KHR,
            ShaderStage::TaskEXT => Self::TASK_EXT,
            ShaderStage::MeshEXT => Self::MESH_EXT,
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DescriptorType {
    /// Indicates that a descriptor type is not enumerated in [`DescriptorType`].
    Unknown = -1,
    /// A type associated with a [`Sampler`].
    ///
    /// Descriptors with this descriptor type *must* be written through an [`image write`][1] that
    /// contains a [`Sampler`].
    ///
    /// # Shader declaratios
    /// Glsl: `uniform sampler ...`
    ///
    /// [1]: DescriptorInfos::images
    Sampler = vk::DescriptorType::SAMPLER.as_raw(),
    /// A type associated with a sampled image.
    ///
    /// Descriptors with this descriptor type *must* be written through an [`image_write`][1] that
    /// contains an [`ImageId`] to an image that *can* be [`sampled from`][2].
    ///
    /// # Shader declarations
    /// Glsl: `uniform texture2D ...`
    ///
    /// [1]: DescriptorInfos::images
    /// [2]: ImageUsages::SAMPLED
    SampledImage = vk::DescriptorType::SAMPLED_IMAGE.as_raw(),
    /// A type that combines both a [`Sampler`] and a sampled image.
    ///
    /// Descriptors with this descriptor type *must* be written through an [`image write`][1] that
    /// contains a [`Sampler`] and an [`ImageId`] to an image that *can* be [`sampled from`][2].
    ///
    /// # Shader declarations
    /// Glsl: `uniform sampler2D ...`
    ///
    /// [1]: DescriptorInfos::images
    /// [2]: ImageUsages::SAMPLED
    CombinedImageSampler = vk::DescriptorType::COMBINED_IMAGE_SAMPLER.as_raw(),
    /// A type associated with an image that can be used for load, store and atomic operations.
    ///
    /// Descriptors with this descriptor type *must* be written through an [`image write`][1] that
    /// contains an [`ImageId`] to an image that *can* be used as a [`storage image`][2].
    ///
    /// # Shader declarations
    /// Glsl: `uniform image2D ...`
    ///
    /// [1]: DescriptorInfos::images
    /// [2]: ImageUsages::STORAGE
    StorageImage = vk::DescriptorType::STORAGE_IMAGE.as_raw(),
    /// A type associated with a buffer that can be used for load operations.
    ///
    /// Descriptors with this descriptor type *must* be written through a [`buffer write`][1] that
    /// contains a [`BufferId`] to a buffer that *can* be used as a [`uniform buffer`][2].
    ///
    /// # Shader declarations
    /// Glsl: `uniform struct UBO { ... } ubo`
    ///
    /// [1]: DescriptorInfos::buffers
    /// [2]: BufferUsages::UNIFORM_BUFFER
    UniformBuffer = vk::DescriptorType::UNIFORM_BUFFER.as_raw(),
    /// A type associated with a buffer that can be used for load, store and atomic operations.
    ///
    /// Descriptors with this descriptor type *must* be written through a [`buffer write`][1] that
    /// contains a [`BufferId`] to a buffer that *can* be used as a [`storage buffer`][2].
    ///
    /// # Shader declarations
    /// Glsl: `buffer struct SSBO { ... } ssbo`
    ///
    /// [1]: DescriptorInfos::buffers
    /// [2]: BufferUsages::STORAGE_BUFFER
    StorageBuffer = vk::DescriptorType::STORAGE_BUFFER.as_raw(),
    /// A type associated with a buffer and a buffer view that can be used for image sampling
    /// operations.
    ///
    /// Descriptors with this descriptor type *must* be written through a [`buffer write`][1] that
    /// contains a [`BufferId`] to a buffer that *can* be used as a [`uniform texel buffer`][2].
    ///
    /// # Shader declarations
    /// Glsl: `uniform samplerBuffer ...`
    ///
    /// [1]: DescriptorInfos::buffers
    /// [2]: BufferUsages::UNIFORM_TEXEL_BUFFER
    UniformTexelBuffer = vk::DescriptorType::UNIFORM_TEXEL_BUFFER.as_raw(),
    /// A type associated with a buffer and a buffer view that can be used for image load,
    /// store and atomic operations.
    ///
    /// Descriptors with this descriptor type *must* be written through a [`buffer write`][1] that
    /// contains a [`BufferId`] to a buffer that *can* be used as a [`storage texel buffer`][2].
    ///
    /// # Shader declarations
    /// Glsl: `uniform imageBuffer ...`
    ///
    /// [1]: DescriptorInfos::buffers
    /// [2]: BufferUsages::STORAGE_TEXEL_BUFFER
    StorageTexelBuffer = vk::DescriptorType::STORAGE_TEXEL_BUFFER.as_raw(),
    /// A type similar to [`DescriptorType::UniformBuffer`] where it's storage comes directly from
    /// the descriptor set rather than from a separate buffer.
    ///
    /// Descriptors with this descriptor type *must* be written through an
    /// [`inline uniform block write`][1].
    ///
    /// # Shader declarations
    /// Glsl: `uniform struct UBO { ... } ubo`
    ///
    /// [1]: DescriptorInfos::inline_uniform_block
    InlineUniformBlock = vk::DescriptorType::INLINE_UNIFORM_BLOCK.as_raw(),
    /// A type used for input attachments, which allows render passes to read earlier rendering
    /// results within the same pass.
    ///
    /// Descriptors with this descriptor type *must* be written through an [`image write`][1] that
    /// contains an [`ImageId`] to an image that *can* be used as an [`input attachment`][2].
    ///
    /// # Shader declarations
    /// Glsl: `layout(input_attachment_index = 0, set = 0, binding = 0) uniform subpassInput ...`
    ///
    /// # Shader load
    /// Glsl: `subpassLoad(subpassInput)`
    ///
    /// [1]: DescriptorInfos::images
    /// [2]: ImageUsages::INPUT_ATTACHMENT
    InputAttachment = vk::DescriptorType::INPUT_ATTACHMENT.as_raw(),
}

/// ``` rust
/// DescriptorType::UniformBuffer |
/// DescriptorType::StorageBuffer |
/// DescriptorType::UniformTexelBuffer |
/// DescriptorType::StorageTexelBuffer
/// ```
#[macro_export]
macro_rules! buffer_descriptor_types {
    () => {
        DescriptorType::UniformBuffer |
        DescriptorType::StorageBuffer |
        DescriptorType::UniformTexelBuffer |
        DescriptorType::StorageTexelBuffer
    };
}

/// ``` rust
/// DescriptorType::Sampler |
/// DescriptorType::SampledImage |
/// DescriptorType::CombinedImageSampler |
/// DescriptorType::StorageImage |
/// DescriptorType::InputAttachment
/// ```
#[macro_export]
macro_rules! image_descriptor_types {
    () => {
        DescriptorType::Sampler |
        DescriptorType::SampledImage |
        DescriptorType::CombinedImageSampler |
        DescriptorType::StorageImage |
        DescriptorType::InputAttachment
    };
}

impl DescriptorType {

    #[inline(always)]
    pub fn as_raw(self) -> i32 {
        self as i32
    }

    #[inline(always)]
    pub fn is_buffer(self) -> bool {
        matches!(self, buffer_descriptor_types!())
    }

    #[inline(always)]
    pub fn buffer_usage(self) -> Option<BufferUsages> {
        match self {
            Self::UniformBuffer => Some(BufferUsages::UNIFORM_BUFFER),
            Self::StorageBuffer => Some(BufferUsages::STORAGE_BUFFER),
            Self::UniformTexelBuffer => Some(BufferUsages::UNIFORM_TEXEL_BUFFER),
            Self::StorageTexelBuffer => Some(BufferUsages::STORAGE_TEXEL_BUFFER),
            Self::InlineUniformBlock => Some(BufferUsages::empty()),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn requires_buffer(self) -> bool {
        matches!(self,
            Self::UniformBuffer |
            Self::StorageBuffer |
            Self::UniformTexelBuffer |
            Self::StorageTexelBuffer
        )
    }

    #[inline(always)]
    pub fn is_inline_uniform_block(self) -> bool {
        matches!(self, Self::InlineUniformBlock)
    }

    #[inline(always)]
    pub fn is_image(self) -> bool {
        matches!(self, image_descriptor_types!())
    }

    #[inline(always)]
    pub fn image_usage(self) -> Option<ImageUsages> {
        match self {
            Self::Sampler => Some(ImageUsages::empty()),
            Self::SampledImage => Some(ImageUsages::SAMPLED),
            Self::StorageImage => Some(ImageUsages::STORAGE),
            Self::CombinedImageSampler => Some(ImageUsages::SAMPLED),
            Self::InputAttachment => Some(ImageUsages::INPUT_ATTACHMENT),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn requires_sampler(self) -> bool {
        matches!(self,
            Self::Sampler |
            Self::CombinedImageSampler,
        )
    } 

    #[inline(always)]
    pub fn requires_image(self) -> bool {
        matches!(self,
            Self::SampledImage |
            Self::CombinedImageSampler |
            Self::StorageImage |
            Self::InputAttachment,
        )
    } 

    #[inline(always)]
    pub(crate) fn shader_image_layout(&self) -> Option<ShaderImageLayout> {
        match self {
            Self::SampledImage | Self::CombinedImageSampler => {
                Some(ShaderImageLayout::SampledReadOnly)
            },
            Self::StorageImage => {
                Some(ShaderImageLayout::General(vk::AccessFlags2::SHADER_READ | vk::AccessFlags2::SHADER_WRITE))
            },
            Self::InputAttachment => {
                Some(ShaderImageLayout::Attachment(AttachmentImageLayout::RenderingLocalRead { is_color: true, }))
            },
            _ => None,
        }
    }
}

impl Display for DescriptorType {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Sampler => write!(f, "sampler"),
            Self::SampledImage => write!(f, "sampled image"),
            Self::CombinedImageSampler => write!(f, "combined image sampler"),
            Self::StorageImage => write!(f, "storage image"),
            Self::UniformBuffer => write!(f, "uniform buffer"),
            Self::StorageBuffer => write!(f, "storage buffer"),
            Self::UniformTexelBuffer => write!(f, "uniform texel buffer"),
            Self::StorageTexelBuffer => write!(f, "storage texel buffer"),
            Self::InlineUniformBlock => write!(f, "inline uniform block"),
            Self::InputAttachment => write!(f, "input attachment"),
        }
    }
}

impl From<DescriptorType> for vk::DescriptorType {

    fn from(value: DescriptorType) -> Self {
        Self::from_raw(value.as_raw())
    }
}

impl From<vk::DescriptorType> for DescriptorType {

    fn from(value: vk::DescriptorType) -> Self {
        match value {
            vk::DescriptorType::SAMPLER => Self::Sampler,
            vk::DescriptorType::SAMPLED_IMAGE => Self::SampledImage,
            vk::DescriptorType::COMBINED_IMAGE_SAMPLER => Self::CombinedImageSampler,
            vk::DescriptorType::STORAGE_IMAGE => Self::StorageImage,
            vk::DescriptorType::UNIFORM_BUFFER => Self::UniformBuffer,
            vk::DescriptorType::STORAGE_BUFFER => Self::StorageBuffer,
            vk::DescriptorType::UNIFORM_TEXEL_BUFFER => Self::UniformTexelBuffer,
            vk::DescriptorType::STORAGE_TEXEL_BUFFER => Self::StorageTexelBuffer,
            vk::DescriptorType::INLINE_UNIFORM_BLOCK => Self::InlineUniformBlock,
            _ => Self::Unknown,
        }
    }
}

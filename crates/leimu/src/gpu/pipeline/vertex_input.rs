use leimu_proc::BuildStructure;
use tuhka::vk;

use crate::gpu::prelude::*;

/// Specifies what the input rate of a vertex input is.
///
/// [`Vertex rate`][1] means that the inputs are consumed per *vertex*.
///
/// [`Instance rate`][2] means that the inputs are consumed per *instance*.
///
/// # Vulkan docs
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputRate.html>
///
/// [1]: Self::Vertex
/// [2]: Self::Instance
#[repr(i32)]
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexInputRate {
    #[default]
    Vertex = vk::VertexInputRate::VERTEX.as_raw(),
    Instance = vk::VertexInputRate::INSTANCE.as_raw(),
}

impl VertexInputRate {

    #[inline]
    pub fn as_raw(self) -> i32 {
        self as i32
    }
}

impl From<VertexInputRate> for vk::VertexInputRate {

    fn from(value: VertexInputRate) -> Self {
        Self::from_raw(value.as_raw())
    }
}

/// Describes the [`location`][1], [`format`][2] and [`offset`][3] of a vertex input attribute.
///
/// This is used when creating a [`graphics pipeline`][4] and can be derived for a custom struct
/// with the [`VertexInput`] derive macro.
///
/// # Vulkan docs
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputAttributeDescription.html>
///
/// [1]: Self::location
/// [2]: Self::format
/// [3]: Self::offset
/// [4]: GraphicsPipeline
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, BuildStructure)]
pub struct VertexInputAttribute {
    pub location: u32,
    pub format: Format,
    pub offset: u32,
}

impl VertexInputAttribute {

    pub(crate) fn into_internal(self, binding: u32) -> VertexInputAttributeInternal {
        VertexInputAttributeInternal {
            location: self.location,
            binding,
            format: self.format,
            offset: self.offset,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct VertexInputAttributeInternal {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}

impl From<VertexInputAttributeInternal> for vk::VertexInputAttributeDescription {

    fn from(value: VertexInputAttributeInternal) -> Self {
        Self {
            location: value.location,
            binding: value.binding,
            format: value.format.into(),
            offset: value.offset,
        }
    }
}

/// A trait for reflecting vulkan vertex input attributes from a Rust struct.
///
/// To implement this for a repr(C) struct, you can use the [`VertexInput`] derive macro.
pub trait VertexInput<const N_ATTRIBUTES: usize> {

    fn get_attributes(first_location: u32) -> [VertexInputAttribute; N_ATTRIBUTES];
}

/// Desribes a vertex input in a [`graphics pipeline`][1].
///
/// The [`stride`][2] of a vertex input *can* be [`dynamic`][3] when creating the pipeline.
///
/// # Vulkan docs
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputBindingDescription.html>
///
/// [1]: GraphicsPipeline
/// [2]: Self::stride
/// [3]: DynamicState::VertexInputBindingStride
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, BuildStructure)]
pub struct VertexInputBinding {
    pub binding: u32,
    pub input_rate: VertexInputRate,
    #[skip]
    pub stride: u32,
}

impl VertexInputBinding {

    pub fn new<T>(binding: u32, input_rate: VertexInputRate) -> Self
    {
        Self {
            binding,
            input_rate,
            stride: size_of::<T>() as u32,
        }
    }

    pub fn stride<T>(mut self) -> Self {
        self.stride = size_of::<T>() as u32;
        self
    }
}

impl From<VertexInputBinding> for vk::VertexInputBindingDescription {

    fn from(value: VertexInputBinding) -> Self {
        Self {
            binding: value.binding,
            stride: value.stride,
            input_rate: value.input_rate.into(),
        }
    }
}

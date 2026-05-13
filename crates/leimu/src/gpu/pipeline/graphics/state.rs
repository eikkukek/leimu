use tuhka::vk;
use leimu_mem::{
    vec::{Vec32, NonNullVec32},
    alloc::LocalAlloc,
};
use leimu_proc::BuildStructure;

use crate::{
    bitflags,
    core::TryExtend,
    gpu::prelude::*,
    error::*,
};

/// Specifies the vertex input state of a [`GraphicsPipeline`].
#[derive(Default, Clone)]
pub struct PipelineVertexInputState {
    pub(crate) vertex_input_bindings: Vec32<VertexInputBinding>,
    pub(crate) vertex_input_attributes: Vec32<VertexInputAttributeInternal>,
    pub(crate) vertex_input_binding_divisors: Vec32<VertexInputBindingDivisor>,
}

impl PipelineVertexInputState {

    /// Adds vertex input to the state.
    ///
    /// The binding must be unique and input [`locations`][1].
    ///
    /// [1]: VertexInputAttribute::location
    pub fn with_vertex_input(
        mut self,
        binding: VertexInputBinding,
        attributes: &mut [VertexInputAttribute],
    ) -> Result<Self> {
        if self.vertex_input_bindings
            .iter()
            .any(|b| binding.binding == b.binding)
        {
            return Err(Error::just_context(format!(
                "binding {} already exists in pipeline", binding.binding
            )))
        }
        self.vertex_input_bindings.push(binding);
        if attributes.is_empty() {
            return Ok(self)
        }
        attributes.sort_unstable_by_key(|a| a.location);
        if let Some((_, attr)) = attributes[0..attributes.len() - 1]
            .iter().enumerate()
            .find(|&(i, a)|
                attributes[i + 1..]
                    .iter()
                    .any(|b| a.location == b.location)
            )
        {
            return Err(Error::just_context(format!(
                "location {} duplicated in attributes",
                attr.location,
            )))
        }
        let first_location = unsafe {
            attributes.first().unwrap_unchecked()
        }.location;
        let last_location = unsafe {
            attributes.last().unwrap_unchecked()
        }.location;
        for attr in self.vertex_input_attributes.iter().copied() {
            if attr.location >= first_location &&
                attr.location <= last_location
            {
                return Err(Error::just_context(format!(
                    "location {} already exists in pipeline", attr.location
                )))
            }
        }
        self.vertex_input_attributes
            .extend(attributes.iter().map(|attr|
                attr.into_internal(binding.binding
            )));
        Ok(self)
    }

    /// Specifies bindings with [`instance rate divisors`][1].
    ///
    /// # Valid usage
    /// - The [`vertex_attribute_instance_rate_divisor`][2] [`feature`][3] **must** be enabled.
    /// - For each divisor in `divisors`, [`binding`][4] **must'* be a valid binding in this state
    ///   and the referenced binding **must** have [`VertexInputRate`] [`Instance`][5].
    ///
    /// [1]: VertexInputBindingDivisor
    /// [2]: vulkan_14::vertex_attribute_instance_rate_divisor
    /// [3]: DeviceAttributes::with_features
    /// [4]: VertexInputBindingDivisor::binding
    /// [5]: VertexInputRate::Instance
    pub fn with_vertex_input_binding_divisors<I>(
        mut self,
        divisors: I,
    ) -> Self
        where I: IntoIterator<Item = VertexInputBindingDivisor>
    {
        self.vertex_input_binding_divisors.extend(divisors);
        self
    }

    pub(crate) fn prepare_infos<'a, Alloc>(
        &self,
        gpu: &Gpu,
        alloc: &'a Alloc,
    ) -> Result<PipelineVertexInputStateInfos<'a, Alloc>>
        where Alloc: LocalAlloc
    {
        if !self.vertex_input_binding_divisors.is_empty() &&
            !gpu.is_feature_enabled(vulkan_14::vertex_attribute_instance_rate_divisor::NAME)
        {
            return Err(Error::just_context(concat!(
                "vertex input binding divisors is not empty and the ",
                "vertex_attribute_instance_rate_divisor feature is not enabled",
            )))
        }
        let mut vertex_binding_divisors = NonNullVec32::with_capacity(
            self.vertex_input_binding_divisors.len(), alloc
        ).context("alloc error")?;
        vertex_binding_divisors.try_extend(self.vertex_input_binding_divisors
            .iter().map(|divisor| {
                let Some(binding) = self.vertex_input_bindings
                    .iter()
                    .find(|binding| {
                        binding.binding == divisor.binding
                    }) else {
                    return Err(Error::just_context(format!(
                        "divisor references undefined binding {}",
                        divisor.binding,
                    )))
                };
                if binding.input_rate != VertexInputRate::Instance {
                    return Err(Error::just_context(format!(
                        "divisor references binding {}, which doesn't have the Instance vertex input rate",
                        binding.binding,
                    )))
                }
                if divisor.divisor == 0 &&
                    !gpu.is_feature_enabled(vulkan_14::vertex_attribute_instance_rate_zero_divisor::NAME)
                {
                    return Err(Error::just_context(format!(
                        "divisor for binding {} has a zero value, but the {}",
                        divisor.binding,
                        "vertex_attribute_instance_rate_zero_divisor feature is not enabled"
                    )))
                }
                Ok(vk::VertexInputBindingDivisorDescription {
                    binding: divisor.binding,
                    divisor: divisor.divisor,
                })
            })
        )?;
        let mut vertex_binding_descriptions = NonNullVec32::with_capacity(
            self.vertex_input_bindings.len(), alloc
        ).context("alloc error")?;
        vertex_binding_descriptions.extend(self.vertex_input_bindings
            .iter().map(|binding| binding.into())
        );
        let mut vertex_attribute_descriptions = NonNullVec32::with_capacity(
            self.vertex_input_attributes.len(), alloc
        ).context("alloc error")?;
        vertex_attribute_descriptions.extend(self.vertex_input_attributes
            .iter().map(|attr| attr.into())
        );
        Ok(PipelineVertexInputStateInfos {
            vk_info: vk::PipelineVertexInputStateCreateInfo {
                vertex_binding_description_count: vertex_binding_descriptions
                    .len(),
                p_vertex_binding_descriptions: vertex_binding_descriptions
                    .as_ptr(),
                vertex_attribute_description_count: vertex_attribute_descriptions
                    .len(),
                p_vertex_attribute_descriptions: vertex_attribute_descriptions
                    .as_ptr(),
                ..Default::default()
            },
            divisor_state: vk::PipelineVertexInputDivisorStateCreateInfo {
                vertex_binding_divisor_count: vertex_binding_divisors.len(),
                p_vertex_binding_divisors: vertex_binding_divisors.as_ptr(),
                ..Default::default()
            },
            vertex_binding_descriptions,
            vertex_attribute_descriptions,
            vertex_binding_divisors,
            alloc,
        })
    }
}

pub(crate) struct PipelineVertexInputStateInfos<'a, Alloc>
    where Alloc: LocalAlloc,
{
    vk_info: vk::PipelineVertexInputStateCreateInfo<'static>,
    divisor_state: vk::PipelineVertexInputDivisorStateCreateInfo<'static>,
    vertex_binding_descriptions: NonNullVec32<'a, vk::VertexInputBindingDescription>,
    vertex_attribute_descriptions: NonNullVec32<'a, vk::VertexInputAttributeDescription>,
    vertex_binding_divisors: NonNullVec32<'a, vk::VertexInputBindingDivisorDescription>,
    alloc: &'a Alloc,
}

impl<Alloc> PipelineVertexInputStateInfos<'_, Alloc>
    where Alloc: LocalAlloc
{

    #[inline]
    pub fn create_info(&mut self) -> &vk::PipelineVertexInputStateCreateInfo<'_> {
        self.vk_info.p_next(::core::ptr::null());
        if self.divisor_state.vertex_binding_divisor_count != 0 {
            self.vk_info = self.vk_info.push_next(&mut self.divisor_state)
        }
        &self.vk_info
    }
}

impl<'a, Alloc> Drop for PipelineVertexInputStateInfos<'a, Alloc>
    where Alloc: LocalAlloc
{
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.vertex_binding_descriptions.drop_and_free(self.alloc);
            self.vertex_attribute_descriptions.drop_and_free(self.alloc);
            self.vertex_binding_divisors.drop_and_free(self.alloc);
        }
    }
}

/// Specifies the input assembly state of a [`GraphicsPipeline`].
#[derive(Default, Clone, Copy, BuildStructure)]
pub struct PipelineInputAssemblyState {
    /// Specifies [`PrimitiveTopology`] of this state.
    pub primitive_topology: PrimitiveTopology,
    /// Specifies whether controls whether a special vertex index value is treated as restarting the
    /// assembly of primitives.
    ///
    /// For index type [`U32`][1] the special index value is `0xFFFFFFFF`, for index type [`U16`][2]
    /// it is `0xFFFF`, and for index type [`U8`][3] it is `0xFF`.
    ///
    /// This only applies to indexed draw calls.
    ///
    /// Primitive restart is not allowed for "list" topologies unless one of
    /// [`primitive_topology_patch_list_restart`][4] (for [`PrimitiveTopology::PatchList`]) or
    /// [`primitive_topology_list_restart`][5] (for all other list topologies) is enabled.
    ///
    /// [1]: IndexType::U32
    /// [2]: IndexType::U16
    /// [3]: IndexType::U8
    /// [4]: ext::primitive_topology_patch_list_restart
    /// [5]: ext::primitive_topology_list_restart
    pub primitive_restart_enable: bool,
}

bitflags!(
    /// Specifies tessellation domain origin.
    #[default = Self::UPPER_LEFT]
    pub struct TessellationDomainOrigin: i32 {
        /// Specifies that the origin of the domain space is in the upper left corner.
        UPPER_LEFT = 0,
        /// Specifies that the origin of the domain space is in the lower left corner.
        LOWER_LEFT = 1,
    }
);

/// Specifies the tessellation state of a [`GraphicsPipeline`].
#[derive(Default, Clone, Copy, BuildStructure)]
pub struct PipelineTessellationState {
    /// The number of control points per patch.
    pub patch_control_points: u32,
    /// Controls the origin of the tessellation domain space.
    pub domain_origin: TessellationDomainOrigin,
}

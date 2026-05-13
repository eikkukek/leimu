use tuhka::vk;
use leimu_proc::BuildStructure;
use leimu_mem::{
    alloc::LocalAlloc,
    vec::FixedVec32,
};

use crate::{
    gpu::prelude::*,
    error::*,
    core::AsBytes,
};

/// Controls the robustness of pipeline [`shader stages`][1].
///
/// [1]: PipelineShaderStageCreateInfo
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PipelineRobustnessInfo {
    /// Specifies storage buffer behavior.
    pub storage_buffers: PipelineRobustnessBufferBehavior,
    /// Specifies uniform buffer behavior.
    pub uniform_buffers: PipelineRobustnessBufferBehavior,
    /// Specifies vertex input behavior.
    pub vertex_inputs: PipelineRobustnessBufferBehavior,
    /// Specifies image behavior.
    pub images: PipelineRobustnessImageBehavior,
}

impl From<PipelineRobustnessInfo> for vk::PipelineRobustnessCreateInfo<'_> {

    #[inline]
    fn from(value: PipelineRobustnessInfo) -> Self {
        Self {
            storage_buffers: value.storage_buffers.into(),
            uniform_buffers: value.uniform_buffers.into(),
            vertex_inputs: value.vertex_inputs.into(),
            images: value.images.into(),
            ..Default::default()
        }
    }
}

/// Specifies a specialization map entry.
#[repr(C)]
#[derive(Default, Clone, Copy, BuildStructure)]
pub struct SpecializationMapEntry {
    /// The constant id this entry maps to.
    pub constant_id: u32,
    /// The offset to of the specialization constant value within `data`.
    pub offset: u32,
    /// The size to of the specialization constant value within `data`.
    pub size: usize,
}

/// Creates a new specialization [`map entry`][1].
///
/// [1]: SpecializationMapEntry
pub fn specialization_map_entry(
    constant_id: u32,
    offset: u32,
    size: usize,
) -> SpecializationMapEntry {
    SpecializationMapEntry { constant_id, offset, size }
}

/// Specifies shader specialization info.
///
/// See [`specialization_info`] for full explanation.
#[derive(Clone)]
pub struct SpecializationInfo {
    pub(crate) map_entries: Box<[SpecializationMapEntry]>,
    pub(crate) data: Box<[u8]>,
}

/// Creates new [`specialization info`][1], which **can** be used to specify the values of
/// specialization constants in a shader.
///
/// # Parameters
/// - `map_entries`: Specifies how [`constant ids`][2] map to `data`.
/// - `data`: A slice to a buffer, which will be copied byte-by-byte and used as the buffer
///   `map_entries` map to.
///
/// # Valid usage
/// - Each [`offset`][3] + [`size`][4] **must** not be greater than the size of `data`, in bytes.
///
/// [1]: SpecializationInfo
/// [2]: SpecializationMapEntry::constant_id
/// [3]: SpecializationMapEntry::offset
/// [4]: SpecializationMapEntry::size
#[inline]
pub fn specialization_info<M, T>(
    map_entries: M,
    data: &[T],
) -> Result<SpecializationInfo>
    where
        M: IntoIterator<Item = SpecializationMapEntry>,
        T: Copy + 'static,
{
    let map_entries: Box<_> = map_entries
        .into_iter()
        .collect();
    let data: Box<_> = data
        .as_bytes()
        .into_iter()
        .copied()
        .collect();
    for entry in &map_entries {
        if entry.offset as usize + entry.size > data.len() {
            return Err(Error::just_context(format!(
                "entry offset {} + size {} is larger than data size {}",
                entry.offset, entry.size, data.len()
            )))
        }
    }
    Ok(SpecializationInfo { map_entries, data })
}

crate::bitflags!(
    /// Bitmask controlling how a pipeline shader stage is created.
    pub struct PipelineShaderStageCreateFlags: u32 {
        /// Specifies that `SubgroupSize` **may** vary in the shader stage.
        ///
        /// # Valid usage
        /// - The [`subgroup_size_control`][1] [`feature`][2] **must** be enabled.
        ///
        /// [1]: vulkan_13::subgroup_size_control
        /// [2]: DeviceAttributes::with_features
        ALLOW_VARYING_SUBGROUP_SIZE = vk::PipelineShaderStageCreateFlags::ALLOW_VARYING_SUBGROUP_SIZE.as_raw(),
        /// Specifies that subgroup sizes **must** be launched with all invocations active in the task, mesh, or compute stage..
        /// 
        /// # Valid usage
        /// - The [`subgroup_size_control`][1] [`feature`][2] **must** be enabled.
        ///
        /// [1]: vulkan_13::subgroup_size_control
        /// [2]: DeviceAttributes::with_features
        REQUIRE_FULL_SUBGROUPS = vk::PipelineShaderStageCreateFlags::REQUIRE_FULL_SUBGROUPS.as_raw()
    }
);

/// Specifies the parameters of a pipeline shader stage.
///
/// See [`pipeline_shader_stage_create_info`] and [`PipelineShaderStages`].
#[derive(Clone, BuildStructure)]
pub struct PipelineShaderStageCreateInfo {
    /// The stage in [`PipelineShaderStages`].
    pub stage: ShaderStage,
    /// A bitmask of [`PipelineShaderStageCreateFlags`] associated with this tage.
    pub flags: PipelineShaderStageCreateFlags,
    /// Optionally specifies the values of specialization constants.
    pub specialization_info: Option<SpecializationInfo>,
    /// Specifies the robustness of the shader stage.
    ///
    /// # Valid usage
    /// - If the [`pipeline_robustness`][1] [`feature`][2] is not enabled, this **must** be
    ///   [`None`].
    /// - Each [`buffer`][3] and [`image`][4] behavior **must** be supported by the device.
    ///
    /// [1]: vulkan_14::pipeline_robustness
    /// [2]: DeviceAttributes::with_features
    /// [3]: PipelineRobustnessBufferBehavior
    /// [4]: PipelineRobustnessImageBehavior
    pub robustness_info: Option<PipelineRobustnessInfo>,
    /// Specifies the required subgroup size of the shader stage.
    ///
    /// # Valid usage
    /// - If the [`sub_group_size_control`][1] [`feature`][2] is not enabled, this **must** be
    ///   [`None`].
    /// - `required_subgroup_size` **must** be greater than or equal [`min_subgroup_size`][3].
    /// - `required_subgroup_size` **must** be less than or equal [`max_subgroup_size`][4].
    ///
    /// [1]: vulkan_13::subgroup_size_control
    /// [2]: DeviceAttributes::with_features
    /// [3]: vulkan_13::subgroup_size_control::Properties::min_subgroup_size
    /// [4]: vulkan_13::subgroup_size_control::Properties::max_subgroup_size
    pub required_subgroup_size: Option<u32>,
}

/// Creates default [`PipelineShaderStageCreateInfo`].
#[inline]
pub fn pipeline_shader_stage_create_info(
    stage: ShaderStage
) -> PipelineShaderStageCreateInfo {
    PipelineShaderStageCreateInfo {
        stage,
        specialization_info: None,
        robustness_info: None,
        required_subgroup_size: None,
    }
}

/// Specifies pipeline shader stages.
///
/// See [`pipeline_shader_stages`] for full description.
#[derive(Clone)]
pub struct PipelineShaderStages {
    pub(crate) shader_set_id: ShaderSetId,
    pub(crate) stages: Box<[PipelineShaderStageCreateInfo]>,
}

/// Creates new [`PipelineShaderStages`].
///
/// # Parameters
/// - `shader_set_id`: Specifies the [`ShaderSet`] the stages are based on.
/// - `stages`: Specifies the [`shader stages`][1].
///
/// # Valid usage
/// - Each [`stage`][2] in `stages` **must** be a stage in the shader set and each stage in the shader
///   set **must** be specified in `stages`.
///
/// [1]: PipelineShaderStageCreateInfo
/// [2]: PipelineShaderStageCreateInfo::stage
pub fn pipeline_shader_stages<I>(
    shader_set_id: ShaderSetId,
    stages: I,
) -> PipelineShaderStages
    where I: IntoIterator<Item = PipelineShaderStageCreateInfo>
{
    PipelineShaderStages {
        shader_set_id,
        stages: stages.into_iter().collect(),
    }
}

struct PipelineShaderStageInfos {
    create_info: vk::PipelineShaderStageCreateInfo<'static>,
    robustness_info: Option<vk::PipelineRobustnessCreateInfo<'static>>,
    subgroup_info: Option<vk::PipelineShaderStageRequiredSubgroupSizeCreateInfo<'static>>,
    specialization_info: Option<vk::SpecializationInfo>,
    shader_module: ShaderModule,
}

impl PipelineShaderStages {

    pub(crate) fn prepare_infos<'a, Alloc>(
        &self,
        gpu: &Gpu,
        alloc: &'a Alloc,
    ) -> Result<FixedVec32<'a, PipelineShaderStageInfos, Alloc>>
        where Alloc: LocalAlloc
    {
        let shader_set
    }
}

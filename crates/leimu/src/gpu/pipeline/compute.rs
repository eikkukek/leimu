use tuhka::vk;

use crate::gpu::prelude::*;
use crate::error::*;

/// Contains the handle of a compute pipeline.
#[derive(Clone)]
pub struct ComputePipeline {
    handle: PipelineHandle,
}

impl ComputePipeline { 

    #[inline(always)]
    pub(crate) unsafe fn new(
        device: Device,
        handle: vk::Pipeline,
        shader_set: ShaderSet,
    ) -> Self {
        unsafe {
            Self {
                handle: PipelineHandle::new(device, handle, shader_set),
            }
        }
    }

    #[inline(always)]
    pub fn handle(&self) -> &PipelineHandle {
        &self.handle
    }
}

mod base {

    use super::*;

    #[derive(Clone)]
    pub struct Template<Meta> {
        pub(crate) meta: Meta,
        pub(crate) shader_set_id: ShaderSetId,
        pub(crate) robustness_info: vk::PipelineRobustnessCreateInfo<'static>,
    }

    impl<Meta> Template<Meta> {

        pub(crate) async fn prepare(
            &mut self,
            gpu: &Gpu,
        ) -> Result<(vk::ComputePipelineCreateInfo<'_>, ShaderSet)>
        {
            let shader_set = gpu
                .get_shader_set(self.shader_set_id)
                .await
                .context_with(|| format!(
                    "failed to get shader set {}",
                    self.shader_set_id,
                ))?.clone();
            let module = shader_set.shaders()
                .iter()
                .find(|module| module.stage() == ShaderStage::Compute)
                .ok_or_else(|| Error::just_context(format!(
                    "couldn't find compute shader from shader set {}",
                    self.shader_set_id,
                )))?;
            match self.robustness_info.images {
                vk::PipelineRobustnessImageBehavior::ROBUST_IMAGE_ACCESS 
                    if !gpu
                        .get_device_attribute(ext::robust_image_access::Attributes::IS_SUPPORTED)
                        .bool().unwrap_or_default()
                => {
                    return Err(Error::just_context(format!("{}{}",
                        "pipeline robustness image behavior must not robust image access if robust",
                        "image accesss extension is not enabled",
                    )))
                },
                vk::PipelineRobustnessImageBehavior::ROBUST_IMAGE_ACCESS_2
                    if !gpu
                        .get_device_attribute(ext::robustness2::Attributes::IS_ROBUST_IMAGE_ACCESS_2_SUPPORTED)
                        .bool().unwrap_or_default()
                => {
                    return Err(Error::just_context(
                        "pipeline robustness image behavior must not be robust image access 2 if it is not supported"
                    ))
                },
                _ => {}
            }
            for behavior in [
                    self.robustness_info.storage_buffers,
                    self.robustness_info.uniform_buffers,
                    self.robustness_info.vertex_inputs,
                ]
            {
                if behavior == vk::PipelineRobustnessBufferBehavior::ROBUST_BUFFER_ACCESS_2 &&
                    !gpu
                        .get_device_attribute(ext::robustness2::Attributes::IS_ROBUST_BUFFER_ACCESS_2_SUPPORTED)
                        .bool().unwrap_or_default()
                {
                    return Err(Error::just_context(
                        "pipeline robustness buffer behavior must not be robust buffer access 2 if its not supported"
                    ))
                }
            }
            Ok((vk::ComputePipelineCreateInfo {
                s_type: vk::StructureType::COMPUTE_PIPELINE_CREATE_INFO,
                stage: vk::PipelineShaderStageCreateInfo {
                    s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                    stage: vk::ShaderStageFlags::COMPUTE,
                    module: module.handle(),
                    p_name: module.entry_point().as_ptr(),
                    ..Default::default()
                },
                layout: shader_set.pipeline_layout(), 
                ..Default::default()
            }.push_next(&mut self.robustness_info), shader_set))
        }
    }
}

pub(crate) type ComputePipelineCreateTemplate = base::Template<()>;

pub type ComputePipelineCreateInfo<'a> = base::Template<&'a mut ComputePipelineId>;

impl<'a> ComputePipelineCreateInfo<'a> {

    /// Creates new compute pipeline create info.
    ///
    ///
    /// When added to a [`PipelineBatch`] with [`PipelineBatchBuilder::with_compute_pipelines`],
    /// the id of the to be created [`ComputePipeline`] is returned to `out_id`.
    ///
    /// # Valid usage
    /// - `shader_set_id` *must* be a valid [`ShaderSetId`] and the shader set *must* contain
    ///   [`ShaderStage::Compute`].
    #[inline(always)]
    pub fn new(out_id: &'a mut ComputePipelineId, shader_set_id: ShaderSetId) -> Self {
        Self {
            meta: out_id,
            shader_set_id,
            robustness_info: PipelineRobustnessInfo::default().into(),
        }
    }

    #[inline(always)]
    pub(crate) fn into_template(self) -> ComputePipelineCreateTemplate {
        ComputePipelineCreateTemplate {
            meta: (),
            shader_set_id: self.shader_set_id,
            robustness_info: self.robustness_info,
        }
    }

    #[inline(always)]
    pub fn with_robustness_info(
        mut self,
        robustness_info: PipelineRobustnessInfo,
    ) -> Self {
        self.robustness_info = robustness_info.into();
        self
    } 
}

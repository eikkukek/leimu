use leimu_mem::{
    vec::NonNullVec32,
    arena,
};

use crate::gpu::ext;

use super::*;

mod base {

    use super::*;

    #[derive(Clone)]
    pub struct Template<Meta: Send + Sync> {
        pub(crate) meta: Meta,
        pub(crate) shader_set_id: ShaderSetId,
        pub(crate) dynamic_states: Vec32<DynamicState>,
        pub(crate) vertex_input_bindings: Vec32<VertexInputBinding>,
        pub(crate) vertex_input_attributes: Vec32<VertexInputAttributeInternal>,
        pub(crate) polygon_mode: PolygonMode,
        pub(crate) cull_mode: CullModes,
        pub(crate) front_face: FrontFace,
        pub(crate) depth_bias_info: Option<DepthBiasInfo>,
        pub(crate) primitive_topology: (PrimitiveTopology, bool),
        pub(crate) sample_shading_info: Option<SampleShadingInfo>,
        pub(crate) depth_stencil_info: Option<DepthStencilInfo>,
        pub(crate) color_blend_info: ColorBlendInfo,
        pub(crate) color_outputs: Vec32<(Format, ColorOutputState)>,
        pub(crate) depth_output_format: Format,
        pub(crate) stencil_output_format: Format,
        pub(crate) line_width: f32,
        pub(crate) depth_clamp: bool,
        pub(crate) rasterizer_discard: bool,
        pub(crate) robustness_info: PipelineRobustnessInfo,
    } 
}

/// A clonable [`GraphicsPipelineCreateInfo`] template.
pub type GraphicsPipelineCreateTemplate = base::Template<()>;

impl GraphicsPipelineCreateTemplate {
    
    #[inline]
    pub fn new(shader_set_id: ShaderSetId) -> Self {
        Self::_new((), shader_set_id)
    }
}

/// Structure describing [`GraphicsPipeline`] creation.
pub type GraphicsPipelineCreateInfo<'a> = base::Template<&'a mut GraphicsPipelineId>;

impl<'a> GraphicsPipelineCreateInfo<'a> {

    /// Creates a new [`GraphicsPipelineCreateInfo`].
    ///
    /// When added to a [`PipelineBatch`] with [`PipelineBatchBuilder::with_graphics_pipelines`],
    /// the id of the to be created [`GraphicsPipeline`] is returned to `out_id`.
    ///
    /// # Valid usage
    /// - `shader_set_id` *must* be a valid [`ShaderSetId`].
    /// - The shader set *must* contain a shader with [`ShaderStage::Vertex`].
    /// - If [`GraphicsPipelineCreateInfo::rasterizer_discard`] is false or the dynamic state
    ///   includes [`DynamicState::RASTERIZER_DISCARD_ENABLE`], the shader set *must* contain a shader
    ///   with [`ShaderStage::Fragment`].
    #[inline]
    pub fn new(
        out_id: &'a mut GraphicsPipelineId,
        shader_set_id: ShaderSetId,
    ) -> Self {
        Self::_new(out_id, shader_set_id)
    }

    /// Creates a new [`GraphicsPipelineCreateInfo`] from a [`GraphicsPipelineCreateTemplate`].
    ///
    /// When added to a [`PipelineBatch`] with [`PipelineBatchBuilder::with_graphics_pipelines`],
    /// the id of the to be created [`GraphicsPipeline`] is returned to `out_id`.
    ///
    /// # Valid usage
    /// - [`shader_set_id`][1] *must* be a valid [`ShaderSetId`].
    /// - The shader set *must* contain a shader with [`ShaderStage::Vertex`].
    /// - If [`GraphicsPipelineCreateInfo::rasterizer_discard`] is false or the dynamic state
    ///   includes [`DynamicState::RASTERIZER_DISCARD_ENABLE`], the shader set *must* contain a shader
    ///   with [`ShaderStage::Fragment`].
    ///
    /// [1]: GraphicsPipelineCreateTemplate
    #[inline]
    pub fn from_template(
        out_id: &'a mut GraphicsPipelineId,
        template: GraphicsPipelineCreateTemplate,
    ) -> Self {
        template.cast(out_id)
    }

    /// Turns the create info into [`GraphicsPipelineCreateTemplate`].
    #[inline]
    pub fn into_template(
        self
    ) -> GraphicsPipelineCreateTemplate {
        self.cast(())
    }
}

impl<Meta: Send + Sync> base::Template<Meta> {

    fn cast<T: Send + Sync>(self, meta: T) -> base::Template<T> {
        base::Template {
            meta,
            shader_set_id: self.shader_set_id,
            dynamic_states: self.dynamic_states,
            vertex_input_bindings: self.vertex_input_bindings,
            vertex_input_attributes: self.vertex_input_attributes,
            polygon_mode: self.polygon_mode,
            cull_mode: self.cull_mode,
            front_face: self.front_face,
            depth_bias_info: self.depth_bias_info,
            primitive_topology: self.primitive_topology,
            sample_shading_info: self.sample_shading_info,
            depth_stencil_info: self.depth_stencil_info,
            color_blend_info: self.color_blend_info,
            color_outputs: self.color_outputs,
            depth_output_format: self.depth_output_format,
            stencil_output_format: self.stencil_output_format,
            line_width: self.line_width,
            depth_clamp: self.depth_clamp,
            rasterizer_discard: self.rasterizer_discard,
            robustness_info: self.robustness_info,
        }
    }

    fn _new(meta: Meta, shader_set_id: ShaderSetId) -> Self {
        Self {
            meta,
            shader_set_id,
            dynamic_states: vec32![],
            color_outputs: vec32![],
            vertex_input_bindings: vec32![],
            vertex_input_attributes: vec32![],
            polygon_mode: PolygonMode::default(),
            cull_mode: CullModes::default(),
            front_face: FrontFace::default(),
            depth_bias_info: None,
            primitive_topology: Default::default(),
            sample_shading_info: None,
            depth_stencil_info: None,
            color_blend_info: ColorBlendInfo::default(),
            depth_output_format: Format::Undefined,
            stencil_output_format: Format::Undefined,
            line_width: 1.0,
            depth_clamp: false,
            rasterizer_discard: false,
            robustness_info: PipelineRobustnessInfo::default(),
        }
    }

    /// Sets the shader set the pipeline will build its shader stages with.
    ///
    /// This *must* be specified before creating the pipeline.
    ///
    /// # Valid usage
    /// - `id` *must* be a valid [`ShaderSetId`].
    /// - The shader set *must* contain a shader with [`ShaderStage::Vertex`].
    /// - If [`GraphicsPipelineCreateInfo::rasterizer_discard`] is false or the dynamic state
    ///   includes [`DynamicState::RASTERIZER_DISCARD_ENABLE`], the shader set *must* contain a shader
    ///   with [`ShaderStage::Fragment`].
    #[inline]
    pub fn with_shader_set(mut self, id: ShaderSetId) -> Self {
        self.shader_set_id = id;
        self
    }

    /// Adds vertex input to the pipeline.
    ///
    /// The binding must be unique and input [`locations`][1].
    ///
    /// [1]: VertexInputAttribute::locationv
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

    #[inline]
    pub fn with_dynamic_states(mut self, dynamic_state: &[DynamicState]) -> Self {
        self.dynamic_states.fast_append(dynamic_state);
        self
    }

    /// Sets the width of rasterized line segments.
    ///
    /// This *must* be left as the default value `1.0`, if [`BaseDeviceFeatures`] `wide_lines` is
    /// set to `false`.
    ///
    /// If dynamic states contain [`DynamicState::LINE_WIDTH`], this value is ignored and *must* be
    /// set with [`DrawCommands`]
    #[inline]
    pub fn with_line_width(mut self, width: f32) -> Self {
        self.line_width = width;
        self
    }

    #[inline]
    pub fn with_depth_clamp(mut self, enabled: bool) -> Self {
        self.depth_clamp = enabled;
        self
    }

    #[inline]
    pub fn with_depth_stencil(mut self, info: DepthStencilInfo) -> Self {
        self.depth_stencil_info = Some(info);
        self
    }

    #[inline]
    pub fn with_rasterizer_discard(mut self, enabled: bool) -> Self {
        self.rasterizer_discard = enabled;
        self
    }

    #[inline]
    pub fn with_polygon_mode(mut self, polygon_mode: PolygonMode) -> Self {
        self.polygon_mode = polygon_mode;
        self
    }

    #[inline]
    pub fn with_cull_mode(mut self, cull_modes: CullModes) -> Self {
        self.cull_mode = cull_modes;
        self
    }

    #[inline]
    pub fn with_front_face(mut self, front_face: FrontFace) -> Self {
        self.front_face = front_face;
        self
    }

    #[inline]
    pub fn with_depth_bias(mut self, depth_bias_info: DepthBiasInfo) -> Self {
        self.depth_bias_info = Some(depth_bias_info);
        self
    }

    #[inline]
    pub fn with_primitive_topology(
        mut self,
        topology: PrimitiveTopology,
        restart_enable: bool
    ) -> Self
    {
        self.primitive_topology = (topology, restart_enable);
        self
    }

    #[inline]
    pub fn with_sample_shading(mut self, sample_shading_info: SampleShadingInfo) -> Self {
        self.sample_shading_info = Some(sample_shading_info);
        self
    }

    /// Blend constants are used with color attachments that use 'ConstColor' or 'ConstAlpha' BlendFactors.
    /// The default constants are [0.0, 0.0, 0.0, 0.0]
    #[inline]
    pub fn with_blend_constants(mut self, blend_constants: BlendConstants) -> Self {
        self.color_blend_info.blend_constants = blend_constants;
        self
    }

    /// Sets the [`LogicOp`] for the framebuffer.
    ///
    /// If the [`logic_op`][1] feature is not enabled, `logic_op` *must* be [`None`].
    ///
    /// [1]: BaseDeviceFeatures::logic_op
    #[inline]
    pub fn with_logic_op(mut self, logic_op: Option<LogicOp>) -> Self {
        self.color_blend_info.logic_op = logic_op;
        self
    }

    /// Appends a color output to the pipeline.
    ///
    /// The number of color outputs of a pipeline must match exactly with the number of outputs in the fragment shader.
    #[inline]
    pub fn with_color_output(
        mut self,
        format: Format,
        write_mask: ColorComponents,
        blend_state: Option<ColorOutputBlendState>,
    ) -> Self
    {
        self.color_outputs.push((format, ColorOutputState(
            write_mask,
            blend_state,
        )));
        self
    } 

    /// Sets the depth output format of the pipeline.
    #[inline]
    pub fn with_depth_output(mut self, format: Format) -> Self {
        self.depth_output_format = format;
        self
    }

    /// Sets the stencil output format of the pipeline.
    #[inline]
    pub fn with_stencil_output(mut self, format: Format) -> Self {
        self.stencil_output_format = format;
        self
    }

    /// Sets the robustness info of the pipeline.
    ///
    /// # Valid usage
    /// - If [`pipeline_robustness`][1] is not enabled, each [`buffer behavior`][2] and
    ///   [`image behavior`][3] *must* be device default.
    /// - If [`robust_image_access`][4] is not supported, image behavior *must* not be
    ///   [`robust image access`][5].
    /// - If [`robustness2`][6] is not enabled or [`robust buffer access 2`][7] is not supported,
    ///   buffer behavior *must* not be [`robust buffer access 2`][8].
    /// - If [`robustness2`][6] is not enabled or [`robust image access 2`][9] is not supported,
    ///   image behavior *must* not be [`robust image access 2`][10]
    ///
    /// [1]: ext::pipeline_robustness
    /// [2]: PipelineRobustnessBufferBehavior
    /// [3]: PipelineRobustnessImageBehavior
    /// [4]: ext::robust_image_access::Attributes::IS_SUPPORTED
    /// [5]: PipelineRobustnessImageBehavior::RobustImageAccess
    /// [6]: ext::robustness2
    /// [7]: ext::robustness2::Attributes::IS_ROBUST_BUFFER_ACCESS_2_SUPPORTED
    /// [8]: PipelineRobustnessBufferBehavior::RobustBufferAccess2
    /// [9]: ext::robustness2::Attributes::IS_ROBUST_IMAGE_ACCESS_2_SUPPORTED
    /// [10]: PipelineRobustnessImageBehavior::RobustImageAccess2
    #[inline]
    pub fn with_robustness_info(mut self, info: PipelineRobustnessInfo) -> Self {
        self.robustness_info = info;
        self
    }

    pub(crate) async fn prepare<'a, Alloc>(
        &self,
        gpu: &Gpu,
        alloc: &'a Alloc,
    ) -> Result<(PreparedCreateInfos<'a, Alloc>, ShaderSet)>
        where 
            Alloc: LocalAlloc<Error = arena::Error> + Sync,
    {
        let shader_set = gpu
            .get_shader_set(self.shader_set_id)
            .await?;

        let shaders = shader_set.shaders();
        let mut shader_stage_infos = NonNullVec32
            ::with_capacity(shaders.len() as u32, alloc)
            .context("alloc failed")?;

        let mut fragment_shader_included = false;

        for module in shaders {
            match module.stage() {
                ShaderStage::Fragment => {
                    if fragment_shader_included {
                        return Err(Error::just_context("fragment shader included twice in pipeline"))
                    }
                    fragment_shader_included = true;
                },
                ShaderStage::Vertex | ShaderStage::MeshEXT |
                ShaderStage::TaskEXT | ShaderStage::TessellationControl |
                ShaderStage::TessellationEvaluation => {},
                x => {
                    return Err(Error::just_context(format!(
                        "unsupported shader stage for graphics pipeline {x}"
                    )))
                }
            }
            shader_stage_infos.push(vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                stage: module.stage().into(),
                p_name: module.entry_point().as_ptr(),
                module: module.handle(),
                ..Default::default()
            });
        }

        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            topology: self.primitive_topology.0.into(),
            primitive_restart_enable: self.primitive_topology.1.into(),
            ..Default::default()
        };

        let tesellation_state = vk::PipelineTessellationStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_TESSELLATION_STATE_CREATE_INFO,
            ..Default::default()
        };

        let depth_bias_info = self.depth_bias_info.unwrap_or_default();

        let rasterization_state = vk::PipelineRasterizationStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            depth_clamp_enable: self.depth_clamp.into(),
            rasterizer_discard_enable: self.rasterizer_discard.into(),
            polygon_mode: self.polygon_mode.into(),
            cull_mode: self.cull_mode.into(),
            front_face: self.front_face.into(),
            depth_bias_enable: self.depth_bias_info.is_some().into(),
            depth_bias_constant_factor: depth_bias_info.constant_factor,
            depth_bias_clamp: depth_bias_info.clamp,
            depth_bias_slope_factor: depth_bias_info.slope_factor,
            line_width: 1.0,
            ..Default::default()
        };

        let sample_shading_info = self.sample_shading_info.unwrap_or_default();

        let multisample_state = vk::PipelineMultisampleStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            rasterization_samples: sample_shading_info.samples.into(),
            sample_shading_enable: self.sample_shading_info.is_some().into(),
            min_sample_shading: sample_shading_info.min_shading,
            p_sample_mask: core::ptr::null(),
            alpha_to_coverage_enable: sample_shading_info.alpha_to_coverage.into(),
            alpha_to_one_enable: sample_shading_info.alpha_to_one.into(),
            ..Default::default()
        };

        let depth_stencil_info = self.depth_stencil_info.unwrap_or_default();

        let stencil_test_info = depth_stencil_info.stencil_test_info.unwrap_or_default();

        let depth_bounds = depth_stencil_info.depth_bounds.unwrap_or_default();

        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            depth_test_enable: self.depth_stencil_info.is_some().into(),
            depth_write_enable: depth_stencil_info.write_enable.into(),
            depth_compare_op: depth_stencil_info.compare_op.into(),
            depth_bounds_test_enable: depth_stencil_info.depth_bounds.is_some().into(),
            stencil_test_enable: depth_stencil_info.stencil_test_info.is_some().into(),
            front: stencil_test_info.front.into(),
            back: stencil_test_info.back.into(),
            min_depth_bounds: depth_bounds.min,
            max_depth_bounds: depth_bounds.max,
            ..Default::default()
        };

        let mut color_blend_attachment_states = NonNullVec32::with_capacity(
            self.color_outputs.len(), alloc
        ).context("alloc failed")?;

        color_blend_attachment_states.extend(self.color_outputs
            .iter().map(|(_, info)| (*info).into())
        );

        let color_blend_state = vk::PipelineColorBlendStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            logic_op_enable: self.color_blend_info.logic_op.is_some().into(),
            logic_op: self.color_blend_info.logic_op
                .map(|op| vk::LogicOp::from_raw(op.as_raw()))
                .unwrap_or_default(),
            attachment_count: color_blend_attachment_states.len(),
            p_attachments: color_blend_attachment_states.as_ptr(),
            blend_constants: self.color_blend_info.blend_constants.into(),
            ..Default::default()
        };

        let mut dynamic_states = NonNullVec32::with_capacity(
            2 + self.dynamic_states.len(),
            alloc
        ).context("alloc failed")?;

        dynamic_states.append(&[
            vk::DynamicState::VIEWPORT_WITH_COUNT,
            vk::DynamicState::SCISSOR_WITH_COUNT,
        ]);

        dynamic_states.extend(
            self.dynamic_states
            .iter()
            .map(|&state| state.into()),
        );

        let dynamic_state = vk::PipelineDynamicStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO,
            dynamic_state_count: dynamic_states.len(),
            p_dynamic_states: dynamic_states.as_ptr(),
            ..Default::default()
        };

        let mut color_output_formats = NonNullVec32::<vk::Format>::with_capacity(
            self.color_outputs.len(), alloc
        ).context("alloc failed")?;

        color_output_formats.extend(self.color_outputs
            .iter().map(|(format, _)| (*format).into())
        );

        let rendering_info = vk::PipelineRenderingCreateInfo {
            s_type: vk::StructureType::PIPELINE_RENDERING_CREATE_INFO,
            view_mask: 0,
            color_attachment_count: color_output_formats.len(),
            p_color_attachment_formats: color_output_formats.as_ptr(),
            depth_attachment_format: self.depth_output_format.into(),
            stencil_attachment_format: self.stencil_output_format.into(),
            ..Default::default()
        };

        let mut vertex_input_bindings = NonNullVec32 
            ::with_capacity(self.vertex_input_bindings.len(), alloc)
            .context("alloc failed")?;

        vertex_input_bindings.extend(
            self.vertex_input_bindings.iter().copied().map(Into::into)
        );

        let mut vertex_input_attributes = NonNullVec32
            ::with_capacity(self.vertex_input_attributes.len(), alloc)
            .context("alloc failed")?;

        vertex_input_attributes.extend(
            self.vertex_input_attributes.iter().copied().map(Into::into)
        );

        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            vertex_binding_description_count: vertex_input_bindings.len(),
            p_vertex_binding_descriptions: vertex_input_bindings.as_ptr(),
            vertex_attribute_description_count: vertex_input_attributes.len(),
            p_vertex_attribute_descriptions: vertex_input_attributes.as_ptr(),
            ..Default::default()
        };

        if !gpu
            .get_device_attribute(ext::pipeline_robustness::Attributes::IS_ENABLED)
            .bool().unwrap_or_default() &&
            self.robustness_info != PipelineRobustnessInfo::default()
        {
            return Err(Error::just_context(
                "pipeline robustness info must be the default value if pipeline robustness extension is not enabled"
            ))
        }

        match self.robustness_info.image_behavior {
            PipelineRobustnessImageBehavior::ROBUST_IMAGE_ACCESS 
                if !gpu
                    .get_device_attribute(ext::robust_image_access::Attributes::IS_SUPPORTED)
                    .bool().unwrap_or_default() =>
            {
                return Err(Error::just_context(format!("{}{}",
                    "pipeline robustness image behavior is ROBUST_IMAGE_ACCESS, ",
                    "when it is not supported",
                )))
            },
            PipelineRobustnessImageBehavior::ROBUST_IMAGE_ACCESS2
                if !gpu
                    .get_device_attribute(ext::robustness2::Attributes::IS_ROBUST_IMAGE_ACCESS_2_SUPPORTED)
                    .bool().unwrap_or_default() =>
            {
                return Err(Error::just_context(
                    "pipeline robustness image behavior is ROBUST_IMAGE_ACCESS2 when it is not supported"
                ))
            },
            _ => {}
        };

        for behavior in [
                self.robustness_info.storage_buffer_behavior,
                self.robustness_info.uniform_buffer_behavior,
                self.robustness_info.vertex_input_behavior,
            ]
        {
            if behavior == PipelineRobustnessBufferBehavior::ROBUST_BUFFER_ACCESS2 &&
                !gpu.get_device_attribute(ext::robustness2::Attributes::IS_ROBUST_BUFFER_ACCESS_2_SUPPORTED)
                    .bool().unwrap_or_default()
            {
                return Err(Error::just_context(
                    "pipeline robustness buffer behavior must not be robust buffer access 2 if its not supported"
                ))
            }
        }

        Ok((PreparedCreateInfos {
            shader_stage_infos,
            vertex_input_state,
            input_assembly_state,
            tesellation_state,
            rasterization_state,
            viewport_state: Default::default(),
            multisample_state,
            depth_stencil_state,
            color_blend_state,
            dynamic_state,
            rendering_info,
            layout: shader_set.pipeline_layout(),
            robustness_info: self.robustness_info.into(),
            _color_output_formats: color_output_formats,
            _color_blend_attachment_state: color_blend_attachment_states,
            _vertex_input_bindings: vertex_input_bindings,
            _vertex_input_attributes: vertex_input_attributes,
            _dynamic_states: dynamic_states,
            alloc,
        }, shader_set))
    }
}

pub(crate) struct PreparedCreateInfos<'a, Alloc>
    where Alloc: LocalAlloc,
{
    pub shader_stage_infos: NonNullVec32<'a, vk::PipelineShaderStageCreateInfo<'static>>,
    pub vertex_input_state: vk::PipelineVertexInputStateCreateInfo<'static>,
    pub input_assembly_state: vk::PipelineInputAssemblyStateCreateInfo<'static>,
    pub tesellation_state: vk::PipelineTessellationStateCreateInfo<'static>,
    pub viewport_state: vk::PipelineViewportStateCreateInfo<'static>,
    pub rasterization_state: vk::PipelineRasterizationStateCreateInfo<'static>,
    pub multisample_state: vk::PipelineMultisampleStateCreateInfo<'static>,
    pub depth_stencil_state: vk::PipelineDepthStencilStateCreateInfo<'static>,
    pub color_blend_state: vk::PipelineColorBlendStateCreateInfo<'static>,
    pub dynamic_state: vk::PipelineDynamicStateCreateInfo<'static>,
    pub rendering_info: vk::PipelineRenderingCreateInfo<'static>,
    pub layout: vk::PipelineLayout,
    pub robustness_info: vk::PipelineRobustnessCreateInfo<'static>,
    pub _color_blend_attachment_state: NonNullVec32<'a, vk::PipelineColorBlendAttachmentState>,
    pub _vertex_input_bindings: NonNullVec32<'a, vk::VertexInputBindingDescription>,
    pub _vertex_input_attributes: NonNullVec32<'a, vk::VertexInputAttributeDescription>,
    pub _dynamic_states: NonNullVec32<'a, vk::DynamicState>,
    pub _color_output_formats: NonNullVec32<'a, vk::Format>,
    pub alloc: &'a Alloc,
}

unsafe impl<Alloc> Send for PreparedCreateInfos<'_, Alloc>
    where Alloc: LocalAlloc,
{}

unsafe impl<Alloc> Sync for PreparedCreateInfos<'_, Alloc>
    where Alloc: LocalAlloc,
{}

impl<'a, Alloc> PreparedCreateInfos<'a, Alloc>
    where Alloc: LocalAlloc,
{

    #[inline]
    pub fn as_create_info(&self) -> vk::GraphicsPipelineCreateInfo<'static> {
        vk::GraphicsPipelineCreateInfo {
            s_type: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
            stage_count: self.shader_stage_infos.len(),
            p_stages: self.shader_stage_infos.as_ptr(),
            p_vertex_input_state: &self.vertex_input_state,
            p_input_assembly_state: &self.input_assembly_state,
            p_tessellation_state: &self.tesellation_state,
            p_viewport_state: &self.viewport_state,
            p_rasterization_state: &self.rasterization_state,
            p_multisample_state: &self.multisample_state,
            p_depth_stencil_state: &self.depth_stencil_state,
            p_color_blend_state: &self.color_blend_state,
            p_dynamic_state: &self.dynamic_state,
            layout: self.layout,
            ..Default::default()
        }
    }
}

impl<Alloc> Drop for PreparedCreateInfos<'_, Alloc>
    where Alloc: LocalAlloc,
{

    fn drop(&mut self) {
        unsafe {
            self.shader_stage_infos.drop_and_free(self.alloc);
            self._color_blend_attachment_state.drop_and_free(self.alloc);
            self._vertex_input_bindings.drop_and_free(self.alloc);
            self._vertex_input_attributes.drop_and_free(self.alloc);
            self._dynamic_states.drop_and_free(self.alloc);
            self._color_output_formats.drop_and_free(self.alloc);
        }
    }
}

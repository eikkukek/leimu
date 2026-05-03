use tuhka::vk;
use leimu_proc::BuildStructure;

use crate::gpu::prelude::*;

#[derive(Default, Clone, Copy, BuildStructure)]
pub struct DepthBiasInfo {
    pub constant_factor: f32,
    pub clamp: f32,
    pub slope_factor: f32,
}

#[derive(Default, Clone, Copy, BuildStructure)]
pub struct SampleShadingInfo {
    pub samples: MsaaSamples,
    pub min_shading: f32,
    pub alpha_to_coverage: bool,
    pub alpha_to_one: bool,
}

#[derive(Default, Clone, Copy, BuildStructure)]
pub struct DepthBounds {
    pub min: f32,
    pub max: f32,
}

#[derive(Clone, Copy)]
pub struct StencilOpState {
    /// Operation performed when stencil test fails
    pub fail_op: StencilOp,
    /// Operation performed when both stencil and depth test pass
    pub pass_op: StencilOp,
    /// Operation performed when stencil test passes but depth test fails
    pub depth_fail_op: StencilOp,
    /// Compare operation for the stencil test
    pub compare_op: CompareOp,
    /// Bitmask applied to stencil and reference before comparison
    pub compare_mask: u32,
    /// Bitmask controlling which bits can be written to stencil buffer
    pub write_mask: u32,
    /// The bits which are compared against the stencil buffer
    pub reference: u32,
}

impl Default for StencilOpState {

    fn default() -> Self {
        Self {
            fail_op: StencilOp::KEEP,
            pass_op: StencilOp::KEEP,
            depth_fail_op: StencilOp::KEEP,
            compare_op: CompareOp::ALWAYS,
            compare_mask: 0,
            write_mask: 0,
            reference: 0,
        }
    }
}

impl From<StencilOpState> for vk::StencilOpState {

    fn from(value: StencilOpState) -> Self {
        Self {
            fail_op: value.fail_op.into(),
            pass_op: value.pass_op.into(),
            depth_fail_op: value.depth_fail_op.into(),
            compare_op: value.compare_op.into(),
            compare_mask: value.compare_mask,
            write_mask: value.write_mask,
            reference: value.reference,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct StencilTestInfo {
    pub front: StencilOpState,
    pub back: StencilOpState,
}

#[derive(Default, Clone, Copy, BuildStructure)]
pub struct DepthStencilInfo {
    pub compare_op: CompareOp,
    pub depth_bounds: Option<DepthBounds>,
    pub stencil_test_info: Option<StencilTestInfo>,
    pub write_enable: bool,
}

#[derive(Clone, Copy)]
pub struct ColorOutputBlendState {
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
}

#[derive(Clone, Copy)]
pub(crate) struct ColorOutputState(pub ColorComponents, pub Option<ColorOutputBlendState>);

impl From<ColorOutputState> for vk::PipelineColorBlendAttachmentState {

    fn from(value: ColorOutputState) -> Self {
        match value.1 {
            None => {
                Self {
                    blend_enable: 0,
                    color_write_mask: value.0.into(),
                    ..Default::default()
                }
            },
            Some(b) => {
                Self {
                    blend_enable: 1,
                    src_color_blend_factor: b.src_color_blend_factor.into(),
                    dst_color_blend_factor: b.dst_color_blend_factor.into(),
                    color_blend_op: b.color_blend_op.into(),
                    src_alpha_blend_factor: b.src_alpha_blend_factor.into(),
                    dst_alpha_blend_factor: b.dst_alpha_blend_factor.into(),
                    alpha_blend_op: b.alpha_blend_op.into(),
                    color_write_mask: value.0.into(),
                }
            }
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct BlendConstants([f32; 4]);

impl From<BlendConstants> for [f32; 4] {

    fn from(value: BlendConstants) -> Self {
        value.0
    }
}

#[derive(Default, Clone)]
pub(crate) struct ColorBlendInfo {
    pub blend_constants: BlendConstants, // used in 'ConstColor' and 'ConstAlpha' BlendFactors
    pub logic_op: Option<LogicOp>, // only for integer frame buffers
}

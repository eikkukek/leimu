use tuhka::vk;

use crate::bitflags;
use crate::gpu::prelude::*;

bitflags! {
    /// These flags describes where [`command`][1] dependencies are waited on. Specifically, what the
    /// wait stage mask will be for the wait semaphore signaled by the dependency.
    ///
    /// [1]: Commands
    pub struct MemoryDependencyHint: Flags64 {
        /// Setting this flag means that the wait stage will be set to the earliest possible value.
        ///
        /// The default value of [`MemoryDependencyHint`].
        NONE = 0,
        /// The stage where vertex and index buffers are consumed.
        VERTEX_INPUT = vk::PipelineStageFlags2::VERTEX_INPUT.as_raw(),
        /// The stage where vertex shaders execute.
        VERTEX_SHADER = vk::PipelineStageFlags2::VERTEX_SHADER.as_raw(),
        /// The stage where task shaders execute.
        TASK_SHADER = vk::PipelineStageFlags2::TASK_SHADER_EXT.as_raw(),
        /// The stage where mesh shaders execute.
        MESH_SHADER = vk::PipelineStageFlags2::MESH_SHADER_EXT.as_raw(),
        /// The stage where fragment shaders execute.
        FRAGMENT_SHADER = vk::PipelineStageFlags2::FRAGMENT_SHADER.as_raw(),
        /// The stage where late fragment tests and depth/stencil store operations take place.
        DEPTH_STENCIL_OUTPUT = vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS.as_raw(),
        /// The stage where colors are output from a graphics pipeline.
        COLOR_OUTPUT = vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT.as_raw(),
        /// The stage where compute shaders execute.
        COMPUTE_SHADER = vk::PipelineStageFlags2::COMPUTE_SHADER.as_raw(),
        /// The stage where all transfer commands execute.
        TRANSFER = vk::PipelineStageFlags2::TRANSFER.as_raw()
    }
}

impl From<MemoryDependencyHint> for vk::PipelineStageFlags2 {

    #[inline(always)]
    fn from(value: MemoryDependencyHint) -> Self {
        Self::from_raw(value.as_raw())
    }
}

use core::{
    ops::{Deref, DerefMut},
    fmt::{self, Display},
    marker::PhantomData,
    num::NonZeroU64,
};

use tuhka::vk;
use leimu_proc::BuildStructure;
use leimu_mem::{
    alloc::LocalAlloc,
    arena::{self, Arena},
    conditional::True,
    slot_map::SlotIndex,
    vec::{FixedVec32, NonNullVec32, Vec32},
    int::NonZeroOption,
};

use crate::{
    core::slice,
    gpu::{
        prelude::*,
        command_cache::PipelineCommandCache,
    },
    error::*,
    executor::block_on,
};

#[derive(Clone, Copy, BuildStructure)]
pub struct DrawInfo {
    /// Specifies the first vertex.
    #[default(0)]
    pub first_vertex: u32,
    /// Specifies vertex count.
    ///
    /// The default is one.
    #[default(1)]
    pub vertex_count: u32,
    /// Specifies the first instance.
    #[default(0)]
    pub first_instance: u32,
    /// Specifies instance count.
    ///
    /// The default is one.
    #[default(1)]
    pub instance_count: u32,
}

#[derive(Clone, Copy, BuildStructure)]
pub struct IndexedDrawInfo {
    #[default(0)]
    pub first_index: u32,
    #[default(1)]
    pub index_count: u32,
    #[default(IndexType::U32)]
    pub index_type: IndexType,
    #[default(0)]
    pub first_instance: u32,
    #[default(1)]
    pub instance_count: u32,
    #[default(0)]
    pub vertex_offset: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct DrawBufferRange {
    pub id: BufferId,
    pub offset: DeviceSize,
    pub size: Option<NonZeroU64>,
}

impl DrawBufferRange {

    /// Set size to zero to specify the entire buffer from [`offset`][1].
    ///
    /// [1]: Self::offset
    #[inline(always)]
    pub fn new(id: BufferId, offset: DeviceSize, size: DeviceSize) -> Self {
        Self {
            id,
            offset,
            size: NonZeroU64::new(size),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct IndexBufferInfo {
    pub id: BufferId,
    pub offset: u64,
}

impl IndexBufferInfo {

    #[inline(always)]
    pub fn new(id: BufferId, offset: u64) -> Self {
        Self {
            id,
            offset,
        }
    }
}

pub(super) struct DrawCall {
    pub index_buffer: Option<DrawBufferRange>,
    pub vertex_buffers: NonNullVec32<'static, DrawBufferRange>,
}

#[derive(Default)]
pub(crate) struct DrawCommandStorage {
    pub(super) pipelines: Vec32<PipelineHandle>,
    pub(super) pipeline_cache: PipelineCommandCache,
    pub(super) draw_calls: Vec32<DrawCall>,
    pub command_buffer: vk::CommandBuffer,
    pub(super) wait_scope: vk::PipelineStageFlags2,
    pub(super) color_formats: NonNullVec32<'static, Format>,
    pub(super) depth_format: Format,
    pub(super) stencil_format: Format,
    pub(super) sample_count: MsaaSamples,
}

#[derive(Default, Clone, Copy, BuildStructure)]
pub struct DrawCommandInfo<'a> {
    pub color_formats: &'a [Format],
    pub depth_format: Format,
    pub stencil_format: Format,
    pub sample_count: MsaaSamples,
}

impl DrawCommandStorage {

    #[inline(always)]
    pub fn reinit<Alloc>(
        &mut self,
        command_buffer: vk::CommandBuffer,
        color_formats: &[Format],
        depth_format: Format,
        stencil_format: Format,
        sample_count: MsaaSamples,
        alloc: &Alloc,
    ) -> Result<()>
        where Alloc: LocalAlloc
    {
        self.command_buffer = command_buffer;
        self.wait_scope = vk::PipelineStageFlags2::NONE;
        self.color_formats = NonNullVec32
            ::with_capacity(color_formats.len() as u32, alloc)
            .context("alloc failed")?
            .into_static();
        self.color_formats.fast_append(color_formats);
        self.depth_format = depth_format;
        self.stencil_format = stencil_format;
        self.sample_count= sample_count;
        Ok(())
    }

    pub unsafe fn reset<Alloc>(
        &mut self,
        alloc: &Alloc
    ) where Alloc: LocalAlloc<Error = arena::Error>,
    {
        unsafe {
            self.pipeline_cache.reset(alloc);
            for call in &mut self.draw_calls {
                call.vertex_buffers.drop_and_free(alloc);
            }
            self.color_formats.drop_and_free(alloc);
            self.draw_calls.clear();
            self.pipelines.clear();
        }
    }
}

pub(crate) struct DrawCommandResource {
    pub queue: DeviceQueue,
    pub storage: DrawCommandStorage,
    pub alloc: Arena<True>,
    pub _pool_handle: CommandPoolHandle,
}

impl Drop for DrawCommandResource {

    fn drop(&mut self) {
        unsafe {
            self.storage.reset(&self.alloc);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DrawCommandId(pub(crate) SlotIndex<DrawCommandResource>);

impl Display for DrawCommandId {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

mod state {

    pub trait CanBeginDraw {}
    pub trait CanDraw {}
    pub trait CanDrawIndexed {}

    pub struct Base {}
    impl CanBeginDraw for Base {}

    pub struct Draw {}
    impl CanDraw for Draw {}

    pub struct DrawIndexed {}
    impl CanDrawIndexed for DrawIndexed {}
}

pub struct DrawPipelineCommands<'a, 'b, State = state::Base> {
    general: PipelineCommands<'a, 'b>,
    pipeline: &'a GraphicsPipeline,
    wait_scope: &'a mut vk::PipelineStageFlags2,
    draw_calls: &'a mut Vec32<DrawCall>,
    draw_info: Option<DrawInfo>,
    indexed_draw_info: Option<IndexedDrawInfo>,
    _marker: PhantomData<State>,
}

pub struct DrawCommands<'a> {
    gpu: Gpu,
    storage: &'a mut DrawCommandStorage,
    buffers: ResourceReadGuard<'a, BufferMeta, BufferId>,
    images: ResourceReadGuard<'a, ImageMeta, ImageIndex>,
    last_pipeline: Option<GraphicsPipeline>,
    alloc: &'a dyn LocalAlloc<Error = arena::Error>
}

impl<'a> DrawCommands<'a> {

    pub(crate) fn new(
        gpu: Gpu,
        storage: &'a mut DrawCommandStorage,
        alloc: &'a dyn LocalAlloc<Error = arena::Error>,
        buffers: ResourceReadGuard<'a, BufferMeta, BufferId>,
        images: ResourceReadGuard<'a, ImageMeta, ImageIndex>,
    ) -> Self {
        Self {
            gpu,
            storage,
            buffers,
            images,
            last_pipeline: None,
            alloc,
        }
    }

    /// Binds a graphics pipeline used for all subsequent draw commands.
    ///
    /// # Valid usage
    /// - `id` *must* be a valid [`GraphicsPipelineId`].
    /// - The bound pipeline's sample count *must* match the sample counts defined when creating
    ///   the [`draw commands`][1].
    /// - The bound pipeline's color output formats *must* match the color output formats defined
    ///   when creating the [`draw commands`][1].
    /// - The bound pipeline's depth and stencil formats *must* match the respective formats
    ///   defined when creating the [`draw commands`][1].
    /// - The valid usage section of [`DrawCommands::set_multi_viewport`] apply when the number of
    ///   viewports and scissors is not one.
    ///
    /// [1]: DrawCommands
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipeline.html>
    pub fn bind_pipeline(
        &mut self,
        id: GraphicsPipelineId,
        viewports: &[Viewport],
        scissors: &[Scissor],
    ) -> Result<DrawPipelineCommands<'_, 'a>> {
        let pipeline = block_on(self.gpu.get_graphics_pipeline(id))?;
        if pipeline.samples() != self.storage.sample_count {
            return Err(Error::just_context(format!(
                "pipeline sample count {} must match pass sample count {}",
                self.storage.sample_count, pipeline.samples(),
            )))
        }
        if *pipeline.color_output_formats() != **self.storage.color_formats {
            return Err(Error::just_context(
                "pipeline color output formats don't match with pass"
            ))
        }
        if pipeline.depth_output_format() != self.storage.depth_format {
            return Err(Error::just_context(format!(
                "pipeline depth output format {} doesn't match with pass depth format {}",
                pipeline.depth_output_format(), self.storage.depth_format,
            )))
        }
        if pipeline.stencil_output_format() != self.storage.stencil_format {
            return Err(Error::just_context(format!(
                "pipeline stencil output format {} doesn't match with pass stencil format {}",
                pipeline.stencil_output_format(), self.storage.stencil_format,
            )))
        }
        let handle = pipeline.handle().clone();
        unsafe {
            self.gpu.device().cmd_bind_pipeline(
                self.storage.command_buffer, vk::PipelineBindPoint::GRAPHICS, handle.handle()
            );
        }
        self.storage.pipelines.push(handle);
        self.last_pipeline = Some(pipeline.clone());
        let mut p = self.pipeline_commands().unwrap();
        if viewports.len() != 1 {
            p.set_multi_viewport(viewports, scissors)?;
        } else {
            p.set_viewport(viewports[0], scissors[0])?;
        }
        Ok(p)
    }

    /// Gets the [`pipeline commands`][1] for the last [`pipeline`][2] [`bound`][3].
    ///
    /// You can [`bind`][4] and [`push`][5] descriptor sets, [`push constants`][6] and set
    /// the [`dynamic state`][7] of the [`pipeline`][2]
    ///
    /// Returns an error if no pipeline is bound.
    ///
    /// [1]: DrawPipelineCommands
    /// [2]: GraphicsPipeline
    /// [3]: Self::bind_pipeline
    /// [4]: PipelineCommands::bind_descriptor_sets
    /// [5]: PipelineCommands::push_descriptor_bindings
    /// [6]: PipelineCommands::push_constants
    /// [7]: DynamicState
    #[inline(always)]
    pub fn pipeline_commands(&mut self) -> Result<DrawPipelineCommands<'_, 'a>> {
        let Some(pipeline) = &self.last_pipeline else {
            return Err(Error::just_context(
                "no pipeline bound"
            ))
        };
        unsafe {
            Ok(DrawPipelineCommands {
                general: PipelineCommands::new(
                    self.gpu.clone(),
                    self.storage.command_buffer,
                    pipeline.handle().clone(),
                    &mut self.storage.pipeline_cache,
                    self.alloc,
                    &self.buffers,
                    &self.images,
                ),
                pipeline,
                wait_scope: &mut self.storage.wait_scope,
                draw_calls: &mut self.storage.draw_calls,
                draw_info: None,
                indexed_draw_info: None,
                _marker: PhantomData,
            })
        }
    } 
}

impl<'a, 'b, T> Deref for DrawPipelineCommands<'a, 'b, T> {

    type Target = PipelineCommands<'a, 'b>;
    
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.general
    }
}

impl<'a, 'b, T> DerefMut for DrawPipelineCommands<'a, 'b, T> {

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.general
    }
}

impl<'a, 'b, State> DrawPipelineCommands<'a, 'b, State> {

    fn check_dynamic_state(&self, dynamic_state: DynamicState) -> Result<()> {
        if !self.pipeline.has_dynamic_state(dynamic_state) {
            return Err(Error::just_context(format!(
                "current pipeline's dynamic state doesn't include {dynamic_state}"
            )))
        }
        Ok(())
    }

    /// Dynamically sets the line width for subsequent drawing commands.
    ///
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][1] dynamic state 
    ///   includes [`line width`][2] and if subsequent drawing commands generate line primitives.
    /// - If the [`wide lines`][3] feature of [`enabled base features`][4] is set to `false`, `line_width`
    ///   *must* be 1.0.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineWidth.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::LINE_WIDTH
    /// [3]: BaseDeviceFeatures::wide_lines
    /// [4]: Gpu::enabled_base_features
    pub fn set_line_width(
        &mut self,
        line_width: f32,
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::LINE_WIDTH)?;
        if !self.gpu.enabled_base_features().wide_lines && line_width != 1.0 {
            return Err(Error::just_context(format!(
                "line width must be 1.0 if the wide lines base device feature is not enabled, given line width is {line_width}"
            )))
        }
        unsafe {
            self.gpu.device().cmd_set_line_width(
                self.command_buffer,
                line_width
            );
        }
        Ok(())
    }

    /// Dynamically sets the depth bias factors and clamp for subsequent drawing commands.
    ///
    /// # Parameters
    /// - `depth_bias_constant_factor`: controls the constant value added to each fragment
    /// - `depth_bias_clamp`: is the maximum (or minimum) depth bias of the fragment
    /// - `depth_bias_slope_factor`: a scalar applied to a fragment's slope in depth bias
    ///   calculations
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][1] dynamic state
    ///   includes [`depth bias`][2] and if subsequent drawing commands have depth bias enabled.
    /// - If the [`depth bias clamp`][3] feature of [`enabled base features`][4] is set to `false`,
    ///   `depth_bias_clamp` *must* be 0.0.
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::DEPTH_BIAS
    /// [3]: BaseDeviceFeatures::depth_bias_clamp
    /// [4]: Gpu::enabled_base_features
    pub fn set_depth_bias(
        &mut self,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::DEPTH_BIAS)?;
        if !self.gpu.enabled_base_features().depth_bias_clamp && depth_bias_clamp != 0.0 {
            return Err(Error::just_context(format!(
                "depth bias clamp must be 0.0 if depth bias clamp base device feature is not enabled, given depth bias clamp was {}",
                depth_bias_clamp,
            )))
        }
        unsafe {
            self.gpu.device().cmd_set_depth_bias(
                self.command_buffer,
                depth_bias_constant_factor, 
                depth_bias_clamp,
                depth_bias_slope_factor
            );
        }
        Ok(())
    }

    /// Dynamically sets the values of blend constants.
    ///
    /// # Parameters
    /// - `blend_constants`: specifies the `[R, G, B, A]` color components of the blend constant
    ///   color used in blending, depending on the [`blend factor`][1].
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][2] dynamic state
    ///   includes [`blend constants`][3] and if subsequent drawing commands have blending
    ///   enabled with blend functions using a blend constant.
    ///
    /// [1]: BlendFactor
    /// [2]: GraphicsPipeline
    /// [3]: DynamicState::BLEND_CONSTANTS
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetBlendConstants.html>
    pub fn set_blend_constants(
        &self,
        blend_constants: [f32; 4],
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::BLEND_CONSTANTS)?;
        unsafe {
            self.gpu.device().cmd_set_blend_constants(
                self.command_buffer,
                &blend_constants
            );
        }
        Ok(())
    }

    /// Dynamically sets the depth bounds range.
    ///
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][1] dynamic state
    ///   includes [`depth bounds`][2] and if subsequent drawing commands have depth
    ///   bounds test enabled.
    /// - `min_depth_bounds` must be less than or equal to `max_depth_bounds` and both bounds need to be
    ///   between `0.0` and `1.0` inclusively.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBounds.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::DEPTH_BOUNDS
    pub fn set_depth_bounds(
        &mut self,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::DEPTH_BOUNDS)?;
        if min_depth_bounds > max_depth_bounds {
            return Err(Error::just_context(format!(
                "min depth bounds {min_depth_bounds} must be less than or equal to max depth bounds {max_depth_bounds}"
            )))
        }
        if min_depth_bounds.clamp(0.0, 1.0) != min_depth_bounds {
            return Err(Error::just_context(format!(
                "min depth bounds {min_depth_bounds} must be inclusively between 0.0 and 1.0"
            )))
        }
        if max_depth_bounds.clamp(0.0, 1.0) != max_depth_bounds {
            return Err(Error::just_context(format!(
                "max depth bounds {max_depth_bounds} must be inclusively between 0.0 and 1.0"
            )))
        }
        unsafe {
            self.gpu.device().cmd_set_depth_bounds(
                self.command_buffer,
                min_depth_bounds,
                max_depth_bounds
            );
        }
        Ok(())
    }

    /// Dynamically sets the stencil compare mask.
    ///
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][1] dynamic
    ///   state inludes [`stencil compare mask`][2] and if subsequent drawing commands
    ///   have stencil test enabled.
    /// - Both [`front and back faces`][3] *must* be set (either together or separately) if
    ///   stencil test is enabled.
    /// - `face_mask` *must* not be empty and *must* be a valid [`stencil face`][3] bitmask.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilCompareMask.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::STENCIL_COMPARE_MASK
    /// [3]: StencilFaces
    pub fn set_stencil_compare_mask(
        &mut self,
        face_mask: StencilFaces,
        compare_mask: u32
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::STENCIL_COMPARE_MASK)?;
        if face_mask.is_empty() {
            return Err(Error::just_context(
                "stencil face mask must not be empty"
            ))
        }
        unsafe {
            self.gpu.device().cmd_set_stencil_compare_mask(
                self.command_buffer,
                face_mask.into(),
                compare_mask
            );
        }
        Ok(())
    }
    
    /// Dynamically sets stencil write mask.
    ///
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][1] dynamic
    ///   state includes [`stencil write mask`][2] and if subsequent drawing commands have
    ///   stencil test enabled.
    /// - Both [`front and back faces`][3] *must* be set (either together or separately) if
    ///   stencil test is enabled.
    /// - `face_mask` *must* not be empty and *must* be a valid [`stencil face`][3] bitmask.
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilWriteMask.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::STENCIL_WRITE_MASK
    /// [3]: StencilFaces
    pub fn set_stencil_write_mask(
        &mut self,
        face_mask: StencilFaces,
        write_mask: u32,
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::STENCIL_WRITE_MASK)?;
        if face_mask.is_empty() {
            return Err(Error::just_context(
                "stencil face mask must not be empty"
            ))
        }
        unsafe {
            self.gpu.device().cmd_set_stencil_write_mask(
                self.command_buffer,
                face_mask.into(),
                write_mask
            );
        }
        Ok(())
    }

    /// Dynamically sets stencil reference.
    ///
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][1] dynamic
    ///   state includes [`stencil reference`][3] and if subsequent drawing commands have
    ///   stencil test enabled.
    /// - Both [`front and back faces`][3] *must* be set (either together or separately) if
    ///   stencil test is enabled.
    /// - `face_mask` *must* not be empty and *must* be a valid [`stencil face`][3] bitmask.
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilReference.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::STENCIL_REFERENCE
    /// [3]: StencilFaces
    pub fn set_stencil_reference(
        &mut self,
        face_mask: StencilFaces,
        reference: u32,
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::STENCIL_REFERENCE)?;
        if face_mask.is_empty() {
            return Err(Error::just_context(
                "stencil face mask must not be empty"
            ))
        }
        unsafe {
            self.gpu.device().cmd_set_stencil_reference(
                self.command_buffer,
                face_mask.into(),
                reference
            );
        }
        Ok(())
    }

    /// Dynamically sets cull mode.
    ///
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][1] dynamic
    ///   state includes [`cull mode`][2] and if there are *any* subsequent drawing commands
    ///   using the pipeline.
    /// - `cull_mode` *must* be a valid [`cull mode`][3] bitmask.
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::CULL_MODE
    /// [3]: CullModes
    pub fn set_cull_mode(
        &mut self,
        cull_mode: CullModes,
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::CULL_MODE)?;
        unsafe {
            self.gpu.device().cmd_set_cull_mode(
                self.command_buffer,
                vk::CullModeFlags::from_raw(cull_mode.as_raw()),
            );
        }
        Ok(())
    }

    /// Dynamically sets front face orientation.
    ///
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][1] dynamic
    ///   state includes [`front face`][2] and if there are *any* subsequent drawing commands
    ///   using the pipeline.
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::FRONT_FACE
    pub fn set_front_face(
        &mut self,
        front_face: FrontFace,
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::FRONT_FACE)?;
        unsafe {
            self.gpu.device().cmd_set_front_face(
                self.command_buffer,
                front_face.into(),
            );
        }
        Ok(())
    }

    /// Dynamically sets primitive topology.
    ///
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][1] dynamic
    ///   state includes [`primitive topology`][2] and if there are *any* subsequent drawing
    ///   commands using the pipeline.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::PRIMITIVE_TOPOLOGY
    pub fn set_primitive_topology(
        &mut self,
        primitive_topology: PrimitiveTopology,
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::PRIMITIVE_TOPOLOGY)?;
        unsafe {
            self.gpu.device().cmd_set_primitive_topology(
                self.command_buffer,
                primitive_topology.into(),
            );
        }
        Ok(())
    }

    /// Dynamically sets viewport.
    ///
    /// This is equivalent to (and more efficient than) calling [`set_multi_viewport`][1] with just
    /// one viewport
    ///
    /// This is automatically called when binding a [`pipeline`][2] with a viewport count of one.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html>
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html>
    ///
    /// [1]: DrawCommands::set_multi_viewport
    /// [2]: GraphicsPipeline
    pub fn set_viewport(
        &mut self,
        viewport: Viewport,
        scissor: Scissor,
    ) -> Result<()> {
        let scissor = vk::Rect2D {
            offset: vk::Offset2D {
                x: scissor.x as i32,
                y: scissor.y as i32,
            },
            extent: vk::Extent2D {
                width: scissor.width,
                height: scissor.height,
            },
        };
        unsafe {
            self.gpu.device().cmd_set_viewport_with_count(
                self.command_buffer,
                &[viewport.into()],
            );
            self.gpu.device().cmd_set_scissor_with_count(
                self.command_buffer,
                &[scissor],
            );
        }
        Ok(())
    }

    /// Dynamically sets multi viewport.
    ///
    /// This combines [`vkCmdSetViewportWithCount`][1] and [`vkCmdSetScissorWithCount`][2].
    ///
    /// If you only have one viewport, consider using [`set_viewport`][3] for efficiency.
    ///
    /// This is automatically called when binding a [`pipeline`][4] with more than one viewports.
    ///
    /// # Valid usage
    /// - The number of viewports and scissors *must* match.
    /// - If [`multi viewport`][5] feature of [`enabled base features`][6] is set to `false`,
    ///   the number of viewports *must* be 1. Otherwise the number of viewports *must* be
    ///   between 1 and [`the maximum supported viewports`][7].
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html>
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html>
    ///
    /// [1]: <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html>
    /// [2]: <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html>
    /// [3]: DrawCommands::set_viewport
    /// [4]: GraphicsPipeline
    /// [5]: BaseDeviceFeatures::multi_viewport
    /// [6]: Gpu::enabled_base_features
    /// [7]: DeviceLimits::max_viewports
    /// [8]: DeviceLimits
    pub fn set_multi_viewport(
        &mut self,
        viewports: &[Viewport],
        scissors: &[Scissor],
    ) -> Result<()> {
        let n = viewports.len() as u32;
        if scissors.len() as u32 != n {
            return Err(Error::just_context(format!(
                "the number of viewports {n} and scissors {} must match",
                scissors.len(),
            )))
        }
        if !self.gpu.enabled_base_features().multi_viewport && n != 1 {
            return Err(Error::just_context(format!(
                "viewport count must be 1 if the multi viewport base device feature is not enabled, given view count is {n}"
            )))
        } else if n.clamp(1, self.gpu.device_limits().max_viewports()) != n {
            return Err(Error::just_context(format!(
                "viewport count must be between inclusively between 1 and DeviceLimits::max_viewports() ({}), given view count is {n}",
                self.gpu.device_limits().max_viewports(),
            )))
        }
        let tmp_alloc = self.gpu.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let mut vk_scissors = FixedVec32
            ::with_capacity(n, &tmp_alloc)
            .context("alloc failed")?;
        vk_scissors.extend(scissors.iter().map(|scissor|
            vk::Rect2D {
                offset: vk::Offset2D {
                    x: scissor.x as i32,
                    y: scissor.y as i32,
                },
                extent: vk::Extent2D {
                    width: scissor.width,
                    height: scissor.height,
                },
            }
        ));
        unsafe {
            self.gpu.device().cmd_set_viewport_with_count(
                self.command_buffer,
                slice::cast(viewports).unwrap_unchecked(),
            );
            self.gpu.device().cmd_set_scissor_with_count(
                self.command_buffer,
                &vk_scissors
            );
        }
        Ok(())
    }

    /// Dynamically enables depth test.
    ///
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][1] dynamic
    ///   state includes [`depth test enable`][2] and if there are *any* subsequent drawing
    ///   commands using the pipeline.
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::DEPTH_TEST_ENABLE
    pub fn set_depth_test_enable(
        &mut self,
        enabled: bool
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::DEPTH_TEST_ENABLE)?;
        unsafe {
            self.gpu.device().cmd_set_depth_test_enable(
                self.command_buffer,
                enabled
            );
        }
        Ok(())
    }

    /// Dynamically enables depth write.
    ///
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][1] dynamic
    ///   state includes [`depth write enable`][2] and if there are *any* subsequent drawing
    ///   commands using the pipeline.
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::DEPTH_WRITE_ENABLE
    pub fn set_depth_write_enable(
        &mut self,
        enabled: bool,
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::DEPTH_WRITE_ENABLE)?;
        unsafe {
            self.gpu.device().cmd_set_depth_write_enable(
                self.command_buffer,
                enabled
            );
        }
        Ok(())
    }

    /// Dynamically sets the depth compare operation.
    ///
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][1] dynamic
    ///   state includes [`depth compare op`][2] and if there are *any* subsequent drawing
    ///   commands using the pipeline.
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::DEPTH_COMPARE_OP
    pub fn set_depth_compare_op(
        &mut self,
        compare_op: CompareOp,
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::DEPTH_COMPARE_OP)?;
        unsafe {
            self.gpu.device().cmd_set_depth_compare_op(
                self.command_buffer,
                compare_op.into()
            );
        }
        Ok(())
    }

    /// Dynamically enables depth bounds test.
    ///
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][1] dynamic
    ///   state includes [`depth bounds test enable`][2] and if there are *any* subsequent drawing
    ///   commands using the pipeline.
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::DEPTH_BOUNDS_TEST_ENABLE
    pub fn set_depth_bounds_test_enable(
        &mut self,
        enabled: bool,
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::DEPTH_BOUNDS_TEST_ENABLE)?;
        unsafe {
            self.gpu.device().cmd_set_depth_bounds_test_enable(
                self.command_buffer,
                enabled
            );
        }
        Ok(())
    }

    /// Dynamically enables stencil test.
    ///
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][1] dynamic
    ///   state includes [`stencil test enable`][2] and if there are *any* subsequent drawing
    ///   commands using the pipeline.
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::STENCIL_TEST_ENABLE
    pub fn set_stencil_test_enable(
        &mut self,
        enabled: bool,
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::STENCIL_TEST_ENABLE)?;
        unsafe {
            self.gpu.device().cmd_set_stencil_test_enable(
                self.command_buffer,
                enabled
            );
        }
        Ok(())
    }

    /// Dynamically sets the stencil operation.
    /// # Valid usage
    /// - This *must* be set if and only if the currently bound [`pipeline's`][1] dynamic
    ///   state includes [`stencil op`][2] and if there are *any* subsequent drawing
    ///   commands using the pipeline with stencil test enabled.
    /// - Both [`front and back faces`][3] *must* be set (either together or separately) if
    ///   stencil test is enabled.
    /// - `face_mask` *must* not be empty and *must* be a valid [`stencil face`][3] bitmask.
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::STENCIL_OP
    /// [3]: StencilFaces
    pub fn set_stencil_op(
        &mut self,
        face_mask: StencilFaces,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) -> Result<()> {
        self.check_dynamic_state(DynamicState::STENCIL_OP)?;
        unsafe {
            self.gpu.device().cmd_set_stencil_op(
                self.command_buffer,
                face_mask.into(),
                fail_op.into(),
                pass_op.into(),
                depth_fail_op.into(),
                compare_op.into()
            );
        }
        Ok(())
    }

    /// Binds vertex buffers and allows performing draw calls within the closure.
    ///
    /// # Valid usage
    /// - Each vertex binding's buffer id *must* be a valid [`BufferId`].
    /// - Each vertex buffer *must* have been created with [`BufferUsages::VERTEX_BUFFER`] bit
    ///   set.
    /// - Each vertex binding's offset + size *must* be less than or equal to the buffer's size.
    /// - If and only if currently bound [`pipeline's`][1] dynamic state includes
    ///   [`vertex input binding stride`][2], `vertex_strides` needs to be [`Some`] and
    ///   the number of vertex strides *must* be equal to the number of vertex bindings.
    /// - If [`robust buffer access`][3] or [`robust buffer access 2`][4] are not enabled and the
    ///   [`pipeline`][1] was not created with [`vertex input behavior`][5] value other than
    ///   [`disabled`][6] then, for a given vertex buffer binding, any attribute data fetched *must*
    ///   be entirely contained within the corresponding vertex buffer binding.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: DynamicState::VERTEX_INPUT_BINDING_STRIDE
    /// [3]: BaseDeviceFeatures::robust_buffer_access
    /// [4]: ext::robustness2::Attributes::IS_ROBUST_BUFFER_ACCESS_2_ENABLED
    /// [5]: PipelineRobustnessInfo::vertex_input_behavior
    /// [6]: PipelineRobustnessBufferBehavior::Disabled
    pub fn begin_drawing<F>(
        &mut self,
        draw_info: DrawInfo,
        vertex_bindings: &[DrawBufferRange],
        vertex_strides: Option<&[DeviceSize]>,
        f: F,
    ) -> Result<()>
        where
            State: state::CanBeginDraw,
            F: FnOnce(&mut DrawPipelineCommands<state::Draw>) -> EventResult<()>,
    {
        *self.wait_scope |= vk::PipelineStageFlags2::VERTEX_INPUT;
        let n_bindings = vertex_bindings.len() as u32;
        let mut call = DrawCall {
            index_buffer: None,
            vertex_buffers: NonNullVec32::with_capacity(
                n_bindings,
                self.alloc,
            ).context("alloc failed")?.into_static(),
        };
        let tmp_alloc = self.gpu.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        unsafe {
            let mut vert = FixedVec32
                ::with_capacity(n_bindings, &tmp_alloc)
                .context("alloc failed")?;
            let mut vert_offsets = FixedVec32
                ::with_capacity(n_bindings, &tmp_alloc)
                .context("alloc failed")?;
            let mut vert_sizes = FixedVec32
                ::with_capacity(n_bindings, &tmp_alloc)
                .context("alloc failed")?;
            for (i, buf_info) in vertex_bindings.iter().copied().enumerate() {
                let buf = self.buffers.get(buf_info.id)?;
                let size = buf_info.size.unwrap_or_sentinel(
                    buf.properties().size.wrapping_sub(buf_info.offset)
                );
                if buf_info.offset + size > buf.properties().size {
                    return Err(Error::just_context(format!(
                        "{}{}",
                        format_args!("vertex buffer offset {} + size {} is out of",
                            buf_info.offset, size,
                        ),
                        format_args!("range of vertex buffer size {} at vertex binding index {i}",
                            buf.properties().size,
                        ),
                    )))
                }
                if let Some(err) = buf.validate_usage(BufferUsages::VERTEX_BUFFER) {
                    return Err(Error::new(err, "vertex buffer has incompatible usage"))
                }
                vert.push(buf.handle());
                vert_offsets.push(buf_info.offset);
                vert_sizes.push(size);
                call.vertex_buffers.push(DrawBufferRange {
                    id: buf_info.id,
                    offset: buf_info.offset,
                    size: NonZeroU64::new(size)
                });
            }
            if let Some(strides) = vertex_strides {
                self.check_dynamic_state(DynamicState::VERTEX_INPUT_BINDING_STRIDE)?;
                if strides.len() as u32 != n_bindings {
                    return Err(Error::just_context(format!(
                        "the number of vertex strides {} must match the number of vertex bindings {n_bindings}",
                        strides.len(),
                    )))
                }
            } else if self.check_dynamic_state(DynamicState::VERTEX_INPUT_BINDING_STRIDE).is_ok() {
                return Err(Error::just_context(format!(
                    "{}{}",
                    "the dynamic state of currently bound pipeline includes vertex input binding stride, ",
                    "but no strides were given",
                )))
            }
            let command_buffer = self.command_buffer;
            let device = self.gpu.device();
            device.cmd_bind_vertex_buffers2(
                command_buffer,
                0,
                &vert, &vert_offsets,
                Some(&vert_sizes), None
            );
        }
        self.draw_info = Some(draw_info);
        let cmd = unsafe {
            &mut *(self as *mut Self).cast::<DrawPipelineCommands<state::Draw>>()
        };
        f(cmd).context_from_tracked(|orig| format!(
            "failed to record draw commands at {}", orig.or_this(),
        ))?;
        self.draw_info = None;
        self.draw_calls.push(call);
        Ok(())
    }

    /// Performs a draw call.
    ///
    /// This is only usable in a closure passed to [`begin_drawing`][1].
    ///
    /// # Valid usage
    /// - All [`dynamic states`][2] of the currently bound [`pipeline`][3] *must* be defined before
    ///   performing any draw calls.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDraw.html>
    ///
    /// [1]: Self::begin_drawing
    /// [2]: DynamicState
    /// [3]: GraphicsPipeline
    pub fn draw(&self) -> Result<()>
        where State: state::CanDraw
    {
        let draw_info = self.draw_info.unwrap();
        unsafe {
            self.gpu.device().cmd_draw(
                self.command_buffer,
                draw_info.vertex_count,
                draw_info.instance_count,
                draw_info.first_vertex,
                draw_info.first_instance,
            );
        }
        Ok(())
    }

    /// Binds an index buffer and vertex buffers and allows performing draw calls within the
    /// closure.
    ///
    /// # Valid usage
    /// - If the [`index type uint8`][2] device extension is not enabled, [`index type`][3] *must*
    ///   not be [`IndexType::U8`].
    /// - Index buffer id *must* be a valid [`BufferId`].
    /// - The index buffer *must* have been created with [`BufferUsages::INDEX_BUFFER`] bit
    ///   set.
    /// - Index buffer offset + [`index size`][4] * ([`first index`][5] + [`index count`][6]) *must*
    ///   be less than or equal to the index buffer's size.
    /// - Each vertex binding's buffer id *must* be a valid [`BufferId`].
    /// - Each vertex buffer *must* have been created with [`BufferUsages::VERTEX_BUFFER`] bit
    ///   set.
    /// - Each vertex binding's offset + size *must* be less than or equal to the buffer's size.
    /// - If and only if currently bound [`pipeline's`][1] dynamic state includes
    ///   [`vertex input binding stride`][7], `vertex_strides` needs to be [`Some`] and
    ///   the number of vertex strides *must* be equal to the number of vertex bindings.
    /// - If [`robust buffer access`][8] or [`robust buffer access 2`][9] are not enabled and the
    ///   [`pipeline`][1] was not created with [`vertex input behavior`][10] value other than
    ///   [`disabled`][11] then, for a given vertex buffer binding, any attribute data fetched *must*
    ///   be entirely contained within the corresponding vertex buffer binding.
    /// 
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html>
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html>
    ///
    /// [1]: GraphicsPipeline
    /// [2]: ext::index_type_uint8
    /// [3]: IndexedDrawInfo::index_type
    /// [4]: IndexType::index_size
    /// [5]: IndexedDrawInfo::first_index
    /// [6]: IndexedDrawInfo::index_count
    /// [7]: DynamicState::VERTEX_INPUT_BINDING_STRIDE
    /// [8]: BaseDeviceFeatures::robust_buffer_access
    /// [9]: ext::robustness2::Attributes::IS_ROBUST_BUFFER_ACCESS_2_ENABLED
    /// [10]: PipelineRobustnessInfo::vertex_input_behavior
    /// [11]: PipelineRobustnessBufferBehavior::Disabled
    pub fn begin_drawing_indexed<F>(
        &mut self,
        draw_info: IndexedDrawInfo,
        index_buffer: IndexBufferInfo,
        vertex_bindings: &[DrawBufferRange],
        vertex_strides: Option<&[DeviceSize]>,
        f: F,
    ) -> Result<()>
        where
            State: state::CanBeginDraw,
            F: FnOnce(&mut DrawPipelineCommands<state::DrawIndexed>) -> EventResult<()>,
    {
        *self.wait_scope |= vk::PipelineStageFlags2::VERTEX_INPUT;
        let index_buf_size = (draw_info.first_index + draw_info.index_count) as vk::DeviceSize
            * draw_info.index_type.index_size();
        let n_bindings = vertex_bindings.len() as u32;
        let mut call = DrawCall {
            index_buffer: Some(DrawBufferRange::new(
                index_buffer.id,
                index_buffer.offset,
                index_buf_size,
            )),
            vertex_buffers: NonNullVec32::with_capacity(
                n_bindings,
                self.alloc,
            ).context("alloc failed")?.into_static(),
        };
        unsafe {
            let command_buffer = self.command_buffer;
            let index_buf = self.buffers.get(index_buffer.id)?;
            if index_buffer.offset + index_buf_size > index_buf.properties().size
            {
                return Err(Error::just_context(format!(
                    "given buffer offset {} + size {} is out of range of index buffer size {}",
                    index_buffer.offset, index_buf_size, index_buf.properties().size,
                )))
            }
            if let Some(err) = index_buf.validate_usage(BufferUsages::INDEX_BUFFER) {
                return Err(Error::new(err, "index buffer has incompatible usage"))
            }
            let tmp_alloc = self.gpu.tmp_alloc();
            let tmp_alloc = tmp_alloc.guard();
            let mut vert_bufs = NonNullVec32
                ::with_capacity(n_bindings, &tmp_alloc)
                .context("alloc failed")?;
            let mut vert_offsets = NonNullVec32
                ::with_capacity(n_bindings, &tmp_alloc)
                .context("alloc failed")?;
            let mut vert_sizes = NonNullVec32
                ::with_capacity(n_bindings, &tmp_alloc)
                .context("alloc failed")?;
            for (i, buf_info) in vertex_bindings.iter().copied().enumerate() {
                let buf = self.buffers.get(buf_info.id)?;
                let size = buf_info.size.unwrap_or_sentinel(
                    buf.properties().size.wrapping_sub(buf_info.offset)
                );
                if buf_info.offset + size > buf.properties().size {
                    return Err(Error::just_context(format!(
                        "vertex buffer offset {} + size {} is out of range of vertex buffer size {} at vertex binding index {i}",
                        buf_info.offset, size, buf.properties().size,
                    )))
                }
                if let Some(err) = buf.validate_usage(BufferUsages::VERTEX_BUFFER) {
                    return Err(Error::new(err, "vertex buffer has incompatible usage"))
                }
                vert_bufs.push(buf.handle());
                vert_offsets.push(buf_info.offset);
                vert_sizes.push(size);
                call.vertex_buffers.push(DrawBufferRange {
                    id: buf_info.id,
                    offset: buf_info.offset,
                    size: NonZeroU64::new(size),
                });
            }
            if let Some(strides) = vertex_strides {
                self.check_dynamic_state(DynamicState::VERTEX_INPUT_BINDING_STRIDE)?;
                if strides.len() as u32 != n_bindings {
                    return Err(Error::just_context(format!(
                        "the number of vertex strides {} must match the number of vertex bindings {n_bindings}",
                        strides.len(),
                    )))
                }
            } else if self.check_dynamic_state(DynamicState::VERTEX_INPUT_BINDING_STRIDE).is_ok() {
                return Err(Error::just_context(format!(
                    "{}{}",
                    "the dynamic state of currently bound pipeline includes vertex input binding stride, ",
                    "but no strides were given",
                )))
            }
            let device = self.gpu.device();
            device.cmd_bind_vertex_buffers2(
                command_buffer,
                0,
                &vert_bufs, &vert_offsets, Some(&vert_sizes),
                vertex_strides,
            );
            device.cmd_bind_index_buffer2(
                command_buffer, index_buf.handle(),
                index_buffer.offset, index_buf_size, draw_info.index_type.into()
            );
            
            vert_bufs.drop_and_free(&tmp_alloc);
            vert_offsets.drop_and_free(&tmp_alloc);
            vert_sizes.drop_and_free(&tmp_alloc);
        }
        self.indexed_draw_info = Some(draw_info);
        let cmd = unsafe {
            &mut *(self as *mut Self).cast::<DrawPipelineCommands<state::DrawIndexed>>()
        };
        f(cmd).context_from_tracked(|orig| format!(
            "failed to record draw commands at {}", orig.or_this(),
        ))?;
        self.indexed_draw_info = None;
        self.draw_calls.push(call);
        Ok(())
    }

    /// Performs an indexed draw call.
    ///
    /// This is only usable in a closure passed to [`begin_drawing_indexed`][1].
    ///
    /// # Valid usage
    /// - All [`dynamic states`][2] of the currently bound [`pipeline`][3] *must* be defined before
    ///   performing any draw calls.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexed.html>
    ///
    /// [1]: Self::begin_drawing_indexed
    /// [2]: DynamicState
    /// [3]: GraphicsPipeline
    #[inline(always)]
    pub fn draw_indexed(
        &mut self,
    ) -> Result<()>
        where State: state::CanDrawIndexed
    {
        let draw_info = self.indexed_draw_info.unwrap();
        unsafe {
            self.gpu.device().cmd_draw_indexed(
                self.command_buffer,
                draw_info.index_count, draw_info.instance_count,
                draw_info.first_index, draw_info.vertex_offset,
                draw_info.first_instance,
            );
        }
        Ok(())
    } 
}

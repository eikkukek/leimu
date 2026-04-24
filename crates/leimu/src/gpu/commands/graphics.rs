mod structs;
mod draw;

use tuhka::vk;
use leimu_proc::BuildStructure;
use leimu_mem::{
    alloc::LocalAlloc,
    vec::{NonNullVec32, FixedVec32, Vec32},
    arena::{self, ArenaGuard},
    conditional::True,
};

use crate::{
    error::*,
    core::OptionExt,
    gpu::prelude::*,
};

use super::prelude::CommandResult;

pub use structs::*;
pub use draw::*;

#[derive(Default)]
pub(crate) struct GraphicsCommandCache {
    draw_ids: Vec32<DrawCommandId>,
    draw_storages: Vec32<DrawCommandStorage>,
    next_draw_command_storage: u32,
}

/// Rendering parameters used in dynamic rendering.
#[derive(Clone, Copy, PartialEq, Eq, BuildStructure)]
pub struct RenderingInfo {
    #[skip]
    pub render_area: Option<RenderArea>,
    /// Specifies the number of layers rendered to.
    ///
    /// The default value is one.
    ///
    /// # Valid usage
    /// - `layer_count` *must* not be zero if [`RenderingInfo::view_mask`] is zero.
    /// - each attachment's image view *must* contain at least `layer_count` layers if
    ///   [`RenderingInfo::view_mask`] is zero.
    #[default(1)]
    pub layer_count: u32,
    /// If this is not zero, this specifies a bitfield of view indices describing which views are
    /// active during rendering.
    ///
    /// [`RenderingInfo::layer_count`] is ignored if this is not zero.
    ///
    /// The default value is zero.
    ///
    /// # Valid usage
    /// - If `view_mask` is not zero, each attachment's image view *must* contain at least a number
    ///   of layers greater than the index of the most significant bit of `view_mask`.
    pub view_mask: u32,
    /// Specifies how many multisample anti-aliasing samples attachments have.
    ///
    /// The default value is [`one sample`][1].
    ///
    /// # Valid usage
    /// - `msaa_samples` *must* have exactly one bit set.
    ///
    /// [1]: MsaaSamples::X1
    #[default(MsaaSamples::X1)]
    pub msaa_samples: MsaaSamples,
}

impl RenderingInfo {

    #[inline]
    pub fn render_area<T>(mut self, render_area: T) -> Self
        where T: Into<Option<RenderArea>>
    {
        self.render_area = render_area.into();
        self
    }
}

impl RenderingInfo {
}

pub struct GraphicsCommands<'a, 'b> {
    gpu: Gpu,
    queue: DeviceQueue,
    recorder: CommandRecorder<'a, 'b>,
    primary_command_buffer: vk::CommandBuffer,
    command_id: CommandId,
    wait_scope: vk::PipelineStageFlags2,
    signal_scope: vk::PipelineStageFlags2,
}

pub struct ActiveRenderPass<'a, 'b, 'c>
{
    cmd: &'a mut GraphicsCommands<'b, 'c>,
    sample_count: MsaaSamples,
    color_attachments: &'a [PassAttachment],
    depth_stencil_attachment: &'a DepthStencilAttachment,
    color_formats: NonNullVec32<'static, Format>,
    depth_format: Format,
    stencil_format: Format,
    alloc: ArenaGuard<'a, True>,
}

pub struct NewGraphicsCommands;

impl NewCommands for NewGraphicsCommands {

    const NAME: &'static str = "graphics commands";

    type Target<'a, 'b> = GraphicsCommands<'a, 'b>;

    fn new<'a, 'b>(
        mut recorder: CommandRecorder<'a, 'b>,
        command_id: CommandId,
        queue: DeviceQueue,
    ) -> Result<Self::Target<'a, 'b>>
        where Self::Target<'a, 'b>: Commands<'a, 'b>,
    {
        if !queue.queue_flags().contains(QueueFlags::GRAPHICS) {
            return Err(Error::just_context(format!(
                "queue {queue} doesn't support graphics operations",
            )))
        }
        let command_buffer = recorder
            .get_current_worker()
            .allocate_primaries(&queue, 1)?[0];
        let begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            ..Default::default()
        };
        let gpu = recorder.gpu().clone();
        unsafe {
            gpu.device()
                .begin_command_buffer(command_buffer, &begin_info)
                .context("failed to begin primary command buffer")?;
        }
        Ok(GraphicsCommands {
            gpu,
            queue,
            recorder,
            primary_command_buffer: command_buffer,
            command_id,
            wait_scope: vk::PipelineStageFlags2::NONE,
            signal_scope: vk::PipelineStageFlags2::NONE,
        })
    }
}

unsafe impl<'a, 'b> Commands<'a, 'b> for GraphicsCommands<'a, 'b> {

    fn add_signal_semaphore(
        &mut self, 
        semaphore_id: TimelineSemaphoreId,
        value: u64,
    ) {
        self.recorder.add_signal_semaphore(self.command_id, semaphore_id, value);
    }

    fn add_wait_semaphore(
        &mut self,
        semaphore_id: TimelineSemaphoreId,
        value: u64,
        dependency_hint: MemoryDependencyHint,
    ) {
        self.recorder.add_wait_semaphore(
            self.command_id,
            semaphore_id, value, dependency_hint
        );
    }

    fn finish<'c, Alloc>(self, alloc: &'c Alloc) -> Result<CommandResult<'c, Alloc>>
        where Alloc: ?Sized + LocalAlloc<Error = arena::Error> 
    {
        unsafe {
            self.gpu.device()
                .end_command_buffer(self.primary_command_buffer)
                .context("failed to end command buffer")?;
        }
        let mut primary_command_buffers = FixedVec32
            ::with_capacity(1, alloc)
            .context("alloc failed")?;
        primary_command_buffers.push(self.primary_command_buffer);
        Ok(CommandResult {
            primary_command_buffers,
            wait_scope: self.wait_scope,
            signal_scope: self.signal_scope,
            queue: self.queue,
        })
    }
}

impl<'a, 'b> GraphicsCommands<'a, 'b> { 

    pub fn swapchain_image_view(
        &self,
        surface_id: SurfaceId
    ) -> Result<(SwapchainImageViewId<'_>, Format)> {
        self.recorder.swapchain_image_view(surface_id)
    }

    pub fn render<F>(
        &mut self,
        rendering_info: RenderingInfo,
        color_attachments: &[PassAttachment],
        depth_stencil_attachment: &DepthStencilAttachment,
        f: F,
    ) -> Result<()>
        where F: FnOnce(&mut ActiveRenderPass) -> EventResult<()>
    {
        let command_id = self.command_id;
        let msaa_samples = rendering_info.msaa_samples;
        let stack = self.recorder.stack().clone();
        let mut vk_color_attachments = NonNullVec32
            ::with_capacity(color_attachments.len() as u32, &stack)
            .context("alloc failed")?;
        let mut min_extent = vk::Extent2D {
            width: u32::MAX,
            height: u32::MAX,
        };
        let stack = self.recorder.stack().clone();
        let mut process_attachment = |
            attachment: &PassAttachment,
            resolve_aspect: ResolveAspect,
            usage: ImageUsages,
        | -> Result<(vk::RenderingAttachmentInfo, Format)>
        {
            self.recorder.write_resources(|guard| {
                let image_view = attachment.image_view;
                let image_id = image_view.image_id();
                let image = guard.register_image(
                    image_id.slot_index(),
                    command_id.index(),
                )?;
                if let Some(err) = image.validate_usage(usage) {
                    return Err(Error::new(err, format!(
                        "image {} can't be used as a {} attachment",
                        image_id, resolve_aspect,
                    )))
                }
                let properties = image.properties();
                if msaa_samples != properties.samples {
                    return Err(Error::just_context(format!(
                    "image sample counts must match, image (id {}) had sample count {} while pass sample count was {}",
                        image_id, properties.samples, msaa_samples,
                    )))
                }
                let dimensions = properties.dimensions;
                min_extent.width = min_extent.width.min(dimensions.width);
                min_extent.height = min_extent.height.min(dimensions.height);
                let view = *image.get_view(image_view)?;
                let effective_range = view.subresource_range.effective(
                    properties.mip_levels,
                    properties.array_layers
                );
                let layer_count = effective_range.layer_count;
                let mip_levels = effective_range.layer_count;
                let format = view.component_info.format;
                if mip_levels != 1 {
                    return Err(Error::just_context(format!(
                        "image view {} mip levels {} must be 1 when used as a rendering attachment",
                        image_view, mip_levels,
                    )))
                }
                if layer_count < rendering_info.layer_count {
                    return Err(Error::just_context(format!(
                        "image view {} layer count {} was less than pass layer count {}",
                        image_view, layer_count, rendering_info.layer_count,
                    )))
                }
                let mut attachment_info = vk::RenderingAttachmentInfo {
                    s_type: vk::StructureType::RENDERING_ATTACHMENT_INFO,
                    image_view: view.handle,
                    image_layout: vk::ImageLayout::UNDEFINED,
                    load_op: attachment.load_op.into(),
                    store_op: attachment.store_op.into(),
                    clear_value: attachment.clear_value.into(),
                    ..Default::default()
                };
                if let Some(resolve) = attachment.resolve {
                    let image_view = resolve.image_view;
                    let image_id = image_view.image_id();
                    let image = guard.register_image(
                        image_id.slot_index(),
                        command_id.index(),
                    )?;
                    let properties = image.properties();
                    if properties.dimensions != dimensions {
                        return Err(Error::just_context(format!(
                            "resolve image {} dimensions {} must match source dimensions {}",
                            image_id, properties.dimensions, dimensions,
                        )))
                    }
                    if properties.samples != MsaaSamples::X1 {
                        return Err(Error::just_context(format!(
                            "resolve image {} sample count must be one but given sample count was {}",
                            image_id, properties.samples,
                        )))
                    }
                    let view = *image.get_view(image_view)?;
                    if !view.component_info.format.is_compatible_with(format) {
                        return Err(Error::just_context(format!(
                            "resolve image view {} format {} is not compatible from source image view {} format {}",
                            image_view, view.component_info.format, attachment.image_view, format,
                        )))
                    }
                    let mode_bit = resolve.mode.as_raw();
                    if mode_bit.count_ones() != 1 {
                        return Err(Error::just_context(format!(
                            "attempting to specify more than or less than one resolve mode with mask {}",
                            mode_bit
                        )))
                    }
                    if view.component_info.format.resolve_modes().by_aspect(resolve_aspect).as_raw() & mode_bit != mode_bit {
                        return Err(Error::just_context(format!(
                            "image format {} doesn't support resolve mode {} for resolve aspect {}",
                            format, resolve.mode, resolve_aspect,
                        )))
                    }
                    match resolve_aspect {
                        ResolveAspect::Depth if
                            self.gpu.device().supported_depth_resolve_modes().as_raw() &
                            mode_bit != mode_bit
                        => {
                            return Err(Error::just_context(format!(
                                "resolve mode {} is not supported for depth attachments", resolve.mode,
                            )))
                        },
                        ResolveAspect::Stencil if
                            self.gpu.device().supported_stencil_resolve_modes().as_raw() & mode_bit != mode_bit
                        => {
                            return Err(Error::just_context(format!(
                                "resolve mode {} is not supported for stencil attachments", resolve.mode,
                            )))
                        },
                        _ => {},
                    };
                    let effective_range = view.subresource_range.effective(
                        properties.mip_levels,
                        properties.array_layers
                    );
                    if effective_range.level_count != 1 {
                        return Err(Error::just_context(format!(
                            "resolve image view {} subresource range mip levels {} must be 1 when used as a rendering attachment",
                            image_view, effective_range.level_count,
                        )))
                    }
                    if effective_range.layer_count != layer_count {
                        return Err(Error::just_context(format!(
                            "resolve image view {} subresource layer count {} must match source subresource layer count {}",
                            image_view, effective_range.layer_count, layer_count,
                        )))
                    }
                    attachment_info.resolve_image_view = view.handle;
                    attachment_info.resolve_image_layout = vk::ImageLayout::UNDEFINED;
                    attachment_info.resolve_mode = resolve.mode.into();
                }
                Ok((attachment_info, view.component_info.format))
            })
        };
        let mut color_formats = NonNullVec32
            ::with_capacity(color_attachments.len() as u32, &*stack)
            .context("alloc failed")?
            .into_static();
        for attachment in color_attachments { 
            let (attachment, format) = process_attachment(
                attachment,
                ResolveAspect::Color,
                ImageUsages::COLOR_ATTACHMENT,
            )?;
            color_formats.push(format);
            vk_color_attachments.push(attachment);
        }
        let mut depth_attachment = None;
        let mut depth_format = Format::Undefined;
        let mut stencil_attachment = None;
        let mut stencil_format = Format::Undefined;
        match depth_stencil_attachment {
            DepthStencilAttachment::None => {},
            DepthStencilAttachment::Depth(attachment) => {
                let (depth, format) = process_attachment(
                    attachment,
                    ResolveAspect::Depth,
                    ImageUsages::DEPTH_STENCIL_ATTACHMENT,
                )?;
                depth_format = format;
                depth_attachment = Some(depth);
            },
            DepthStencilAttachment::Stencil(attachment) => {
                let (stencil, format) = process_attachment(
                    attachment,
                    ResolveAspect::Stencil,
                    ImageUsages::DEPTH_STENCIL_ATTACHMENT,
                )?;
                stencil_format = format;
                stencil_attachment = Some(stencil);
            },
            DepthStencilAttachment::DepthStencil { depth, stencil } => {
                if depth.image_view != stencil.image_view {
                    return Err(Error::just_context(
                        "depth and stencil attachment image views are not the same"
                    ))
                }
                let (depth, format) = process_attachment(
                    depth,
                    ResolveAspect::Depth,
                    ImageUsages::DEPTH_STENCIL_ATTACHMENT,
                )?;
                depth_format = format;
                depth_attachment = Some(depth);
                let (stencil, format) = process_attachment(
                    stencil,
                    ResolveAspect::Stencil,
                    ImageUsages::DEPTH_STENCIL_ATTACHMENT,
                )?;
                stencil_format = format;
                stencil_attachment = Some(stencil);
            },
        }
        if !color_attachments.is_empty() {
            self.signal_scope |= vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT;
        } else if
            !matches!(depth_stencil_attachment, DepthStencilAttachment::None)
        {
            self.signal_scope |= vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS;
        }
        self.wait_scope |= vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT;
        let mut pass = ActiveRenderPass {
            cmd: self,
            sample_count: msaa_samples,
            color_attachments,
            depth_stencil_attachment,
            color_formats,
            depth_format,
            stencil_format,
            alloc: stack.guard(),
        };
        f(&mut pass).context_from_tracked(|orig| {
            format!("error recording render pass at {}", orig.or_this())
        })?;
        pass.finish(
            &rendering_info,
            min_extent,
            &mut vk_color_attachments,
            &mut depth_attachment,
            &mut stencil_attachment,
        )?;
        unsafe {
            vk_color_attachments.drop_and_free(&stack);
        }
        Ok(())
    } 

    #[inline(always)]
    pub fn copy_commands(&mut self) -> CopyCommands<'_, 'b> {
        CopyCommands::new(
            self.recorder.forward(),
            self.command_id,
            self.queue.clone(),
            Some(self.primary_command_buffer)
        ).unwrap()
    }
}

impl<'a, 'b, 'c> ActiveRenderPass<'a, 'b, 'c>
{

    pub fn draw<F>(
        &mut self,
        draw_command_id: DrawCommandId,
    ) -> Result<()>
    {
        let cmd = self.cmd.gpu.get_draw_commands(draw_command_id)?;
        if cmd.queue.family_index() != self.cmd.queue.family_index() {
            return Err(Error::just_context(format!(
                "draw command queue {} family index {} is different from pass queue {} family index {}",
                cmd.queue, cmd.queue.family_index(), self.cmd.queue, self.cmd.queue.family_index(),
            )))
        }
        if cmd.storage.color_formats.as_ref() != self.color_formats.as_ref() {
            return Err(Error::just_context(format!(
                "draw command color formats {} don't match pass color formats {}",
                cmd.storage.color_formats, self.color_formats,
            )))
        }
        if cmd.storage.depth_format != self.depth_format {
            return Err(Error::just_context(format!(
                "draw command depth format {} doesn't match pass depth format {}",
                cmd.storage.depth_format, self.depth_format,
            )))
        }
        if cmd.storage.stencil_format != self.stencil_format {
            return Err(Error::just_context(format!(
                "draw command stencil format {} doesn't match pass stencil format {}",
                cmd.storage.stencil_format, self.stencil_format,
            )))
        }
        if cmd.storage.sample_count != self.sample_count {
            return Err(Error::just_context(format!(
                "draw command sample count {} doesn't match pass sample count {}",
                cmd.storage.sample_count, self.sample_count,
            )))
        }
        Ok(())
    }

    pub fn dynamic_draw<F>(
        &mut self,
        f: F,
    ) -> Result<()>
        where F: FnOnce(&mut DrawCommands) -> Result<()>
    {
        let command_buffer = self.cmd.recorder
            .get_current_worker()
            .allocate_secondaries(&self.cmd.queue, 1)?[0];
        let mut rendering_inheritance_info = vk::CommandBufferInheritanceRenderingInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO,
            color_attachment_count: self.color_formats.len(),
            p_color_attachment_formats: self.color_formats.as_ptr().cast(),
            depth_attachment_format: self.depth_format.into(),
            stencil_attachment_format: self.stencil_format.into(),
            rasterization_samples: self.sample_count.into(),
            ..Default::default()
        };
        let inheritance_info = vk::CommandBufferInheritanceInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_INHERITANCE_INFO,
            ..Default::default()
        }.push_next(&mut rendering_inheritance_info);
        let begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            flags:
                vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT |
                vk::CommandBufferUsageFlags::RENDER_PASS_CONTINUE,
            p_inheritance_info: &inheritance_info,
            ..Default::default()
        };
        unsafe {
            self.cmd.gpu.device()
                .begin_command_buffer(command_buffer, &begin_info)
                .context("failed to begin secondary command buffer")?;
        }
        let cache = unsafe { &mut *self.cmd.recorder.cache().get() };
        let cache = &mut cache.graphics_command_cache;
        if cache.next_draw_command_storage >= cache.draw_storages.len() {
            cache.draw_storages.resize_with(cache.next_draw_command_storage + 1, || {
                Default::default()
            });
        }
        let draw_storage = &mut cache.draw_storages[cache.next_draw_command_storage as usize];
        cache.next_draw_command_storage += 1;
        draw_storage.reinit(
            command_buffer,
            &self.color_formats,
            self.depth_format,
            self.stencil_format,
            self.sample_count,
            &self.alloc,
        )?;
        let mut draw_commands = DrawCommands::new(
            self.cmd.gpu.clone(),
            draw_storage,
            &self.alloc,
            self.cmd.recorder.buffers(),
            self.cmd.recorder.images(),
        );
        f(&mut draw_commands)?;
        unsafe {
            self.cmd.gpu.device()
                .end_command_buffer(command_buffer)
                .context("failed to end secondary command buffer")?;
        }
        Ok(())
    }

    fn finish(
        mut self,
        rendering_info: &RenderingInfo,
        min_extent: vk::Extent2D,
        color_attachments: &mut [vk::RenderingAttachmentInfo<'_>],
        depth_attachment: &mut Option<vk::RenderingAttachmentInfo<'_>>,
        stencil_attachment: &mut Option<vk::RenderingAttachmentInfo<'_>>,
    ) -> Result<()>
    {
        let cmd_cache = unsafe { &mut *self.cmd.recorder.cache().get() };
        let num_draw_id = cmd_cache.graphics_command_cache.draw_ids.len();
        let draw_storages = &mut cmd_cache.graphics_command_cache.draw_storages[
            0..cmd_cache.graphics_command_cache.next_draw_command_storage as usize
        ];
        let tmp_alloc = self.cmd.gpu.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let command_id = self.cmd.command_id;
        let primary_command_buffer = self.cmd.primary_command_buffer;
        let queue_family_index = self.cmd.queue.family_index();
        let mut command_buffers = FixedVec32::with_capacity(
            draw_storages.len() as u32 + num_draw_id,
            &tmp_alloc
        ).context("alloc failed")?;
        for attachment in self.color_attachments {
            let image_view = attachment.image_view;
            cmd_cache.shader_resource_cache.touch_image(
                image_view,
                ShaderImageLayout::Attachment(AttachmentImageLayout::Color),
                vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
                Some(ExplicitAccess::COLOR_ATTACHMENT),
            );
            if let Some(resolve) = attachment.resolve {
                cmd_cache.shader_resource_cache.touch_image(
                    resolve.image_view,
                    ShaderImageLayout::Attachment(AttachmentImageLayout::Color),
                    vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
                    Some(ExplicitAccess::COLOR_ATTACHMENT),
                );
            }
        }
        match self.depth_stencil_attachment {
            DepthStencilAttachment::None => {},
            DepthStencilAttachment::Depth(attachment) => {
                let image_view = attachment.image_view;
                cmd_cache.shader_resource_cache.touch_image(
                    image_view,
                    ShaderImageLayout::Attachment(AttachmentImageLayout::DepthStencil(
                        DepthStencilAttachmentType::Depth)
                    ),
                    vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS |
                    vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
                    Some(ExplicitAccess::DEPTH_STENCIL_ATTACHMENT),
                );
                if let Some(resolve) = attachment.resolve {
                    cmd_cache.shader_resource_cache.touch_image(
                        resolve.image_view,
                        ShaderImageLayout::Attachment(
                            AttachmentImageLayout::DepthStencil(DepthStencilAttachmentType::Depth)
                        ),
                        vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
                        Some(ExplicitAccess::DEPTH_STENCIL_ATTACHMENT),
                    );
                }
            },
            DepthStencilAttachment::Stencil(attachment) => {
                cmd_cache.shader_resource_cache.touch_image(
                    attachment.image_view,
                    ShaderImageLayout::Attachment(AttachmentImageLayout::DepthStencil(
                        DepthStencilAttachmentType::Stencil)
                    ),
                    vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS |
                    vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
                    Some(ExplicitAccess::DEPTH_STENCIL_ATTACHMENT),
                );
                if let Some(resolve) = attachment.resolve {
                    cmd_cache.shader_resource_cache.touch_image(
                        resolve.image_view,
                        ShaderImageLayout::Attachment(
                            AttachmentImageLayout::DepthStencil(DepthStencilAttachmentType::Depth)
                        ),
                        vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
                        Some(ExplicitAccess::DEPTH_STENCIL_ATTACHMENT),
                    );
                }
            },
            DepthStencilAttachment::DepthStencil { depth, stencil } => {
                let depth_image_view = depth.image_view;
                let stencil_image_view = stencil.image_view;
                let mut depth_layout = ShaderImageLayout::Attachment(
                    AttachmentImageLayout::DepthStencil(DepthStencilAttachmentType::Depth)
                );
                let mut stencil_layout = ShaderImageLayout::Attachment(
                    AttachmentImageLayout::DepthStencil(DepthStencilAttachmentType::Stencil)
                );
                if depth_image_view.image_id() == stencil_image_view.image_id() {
                    depth_layout = ShaderImageLayout::Attachment(AttachmentImageLayout::DepthStencil(
                        DepthStencilAttachmentType::DepthStencil
                    ));
                    stencil_layout = depth_layout;
                }
                cmd_cache.shader_resource_cache.touch_image(
                    depth_image_view,
                    depth_layout,
                    vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS |
                    vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
                    Some(ExplicitAccess::DEPTH_STENCIL_ATTACHMENT),
                );
                cmd_cache.shader_resource_cache.touch_image(
                    stencil_image_view,
                    stencil_layout,
                    vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS |
                    vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
                    Some(ExplicitAccess::DEPTH_STENCIL_ATTACHMENT),
                );
                if let Some(depth) = depth.resolve &&
                    let Some(stencil) = stencil.resolve
                {
                    let mut depth_layout = ShaderImageLayout::Attachment(
                        AttachmentImageLayout::DepthStencil(DepthStencilAttachmentType::Depth)
                    );
                    let mut stencil_layout = ShaderImageLayout::Attachment(
                        AttachmentImageLayout::DepthStencil(DepthStencilAttachmentType::Stencil)
                    );
                    if depth.image_view.image_id() == stencil.image_view.image_id() {
                        depth_layout = ShaderImageLayout::Attachment(AttachmentImageLayout::DepthStencil(
                            DepthStencilAttachmentType::DepthStencil
                        ));
                        stencil_layout = depth_layout;
                    }
                    cmd_cache.shader_resource_cache.touch_image(
                        depth.image_view,
                        depth_layout,
                        vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
                        Some(ExplicitAccess::DEPTH_STENCIL_ATTACHMENT),
                    );
                    cmd_cache.shader_resource_cache.touch_image(
                        stencil.image_view,
                        stencil_layout,
                        vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
                        Some(ExplicitAccess::DEPTH_STENCIL_ATTACHMENT),
                    );
                } else if let Some(depth) = depth.resolve {
                    cmd_cache.shader_resource_cache.touch_image(
                        depth.image_view,
                        ShaderImageLayout::Attachment(
                            AttachmentImageLayout::DepthStencil(DepthStencilAttachmentType::Depth)
                        ),
                        vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
                        Some(ExplicitAccess::DEPTH_STENCIL_ATTACHMENT),
                    );
                } else if let Some(stencil) = stencil.resolve {
                    cmd_cache.shader_resource_cache.touch_image(
                        stencil.image_view,
                        ShaderImageLayout::Attachment(
                            AttachmentImageLayout::DepthStencil(DepthStencilAttachmentType::Stencil)
                        ),
                        vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
                        Some(ExplicitAccess::DEPTH_STENCIL_ATTACHMENT),
                    );
                }
            },
        };
        for storage in draw_storages {
            self.cmd.wait_scope |= storage.wait_scope;
            let all_shader_stages = storage.pipeline_cache
                .prepare_shader_resource_cache(
                    &mut self.cmd.recorder,
                    &tmp_alloc,
                ).context("failed to process draw command pipeline cache")?;
            for call in &storage.draw_calls {
                if let Some(index_buffer) = call.index_buffer {
                    cmd_cache.shader_resource_cache.touch_buffer(
                        index_buffer.id,
                        index_buffer.offset,
                        unsafe { index_buffer.size.unwrap_unchecked().get() },
                        vk::PipelineStageFlags2::INDEX_INPUT,
                        ExplicitAccess::from_raw(vk::AccessFlags2::INDEX_READ.as_raw()),
                        CommandOrdering::Lenient
                    );
                }
                for vertex_buffer in &call.vertex_buffers {
                    cmd_cache.shader_resource_cache.touch_buffer(
                        vertex_buffer.id,
                        vertex_buffer.offset,
                        unsafe { vertex_buffer.size.unwrap_unchecked().get() },
                        vk::PipelineStageFlags2::VERTEX_ATTRIBUTE_INPUT,
                        ExplicitAccess::from_raw(vk::AccessFlags2::VERTEX_ATTRIBUTE_READ.as_raw()),
                        CommandOrdering::Lenient
                    );
                }
            }
            if !storage.draw_calls.is_empty() {
                self.cmd.wait_scope |= vk::PipelineStageFlags2::VERTEX_INPUT;
            }
            else {
                self.cmd.wait_scope |= all_shader_stages.pipeline_stage_mask();
            }
            let worker = self.cmd.recorder.get_current_worker();
            for pipeline in &storage.pipelines {
                worker.add_pipeline(pipeline.clone());
            }
            unsafe { storage.reset(&self.alloc) }
            command_buffers.push(storage.command_buffer);
        }
        for &id in &cmd_cache.graphics_command_cache.draw_ids {
            let storage = &self.cmd.gpu.get_draw_commands(id)?.storage;
            self.cmd.wait_scope |= storage.wait_scope;
            let all_shader_stages = storage.pipeline_cache.prepare_shader_resource_cache(
                &mut self.cmd.recorder,
                &tmp_alloc,
            ).context("failed to process draw command pipeline cache")?;
            for call in &storage.draw_calls {
                if let Some(index_buffer) = call.index_buffer {
                    cmd_cache.shader_resource_cache.touch_buffer(
                        index_buffer.id,
                        index_buffer.offset,
                        unsafe { index_buffer.size.unwrap_unchecked().get() },
                        vk::PipelineStageFlags2::INDEX_INPUT,
                        ExplicitAccess::from_raw(vk::AccessFlags2::INDEX_READ.as_raw()),
                        CommandOrdering::Lenient
                    );
                }
                for vertex_buffer in &call.vertex_buffers {
                    cmd_cache.shader_resource_cache.touch_buffer(
                        vertex_buffer.id,
                        vertex_buffer.offset,
                        unsafe { vertex_buffer.size.unwrap_unchecked().get() },
                        vk::PipelineStageFlags2::VERTEX_ATTRIBUTE_INPUT,
                        ExplicitAccess::from_raw(vk::AccessFlags2::VERTEX_ATTRIBUTE_READ.as_raw()),
                        CommandOrdering::Lenient
                    );
                }
            }
            if !storage.draw_calls.is_empty() {
                self.cmd.wait_scope |= vk::PipelineStageFlags2::VERTEX_INPUT;
            }
            else {
                self.cmd.wait_scope |= all_shader_stages.pipeline_stage_mask();
            }
            let worker = self.cmd.recorder.get_current_worker();
            for pipeline in &storage.pipelines {
                worker.add_pipeline(pipeline.clone());
            }
            command_buffers.push(storage.command_buffer);
        }
        unsafe {
            cmd_cache.shader_resource_cache.process(
                &mut self.cmd.recorder,
                self.cmd.primary_command_buffer,
                queue_family_index,
                command_id,
                &tmp_alloc
            ).context("failed to process shader resources")?;
        }
        for (i, attachment) in self.color_attachments.iter().enumerate() {
            let image_view = attachment.image_view;
            let images = self.cmd.recorder.images();
            let image = images.get(image_view.image_id()).unwrap();
            let state = image.view_state(
                image_view, ImageAspects::COLOR,
            )?;
            let vk = &mut color_attachments[i];
            vk.image_layout = state.layout;
            if let Some(resolve) = attachment.resolve {
                let image = images.get(resolve.image_view.image_id()).unwrap();
                let state = image.view_state(
                    resolve.image_view, ImageAspects::COLOR,
                )?;
                vk.resolve_image_layout = state.layout;
            }
        }
        match self.depth_stencil_attachment {
            DepthStencilAttachment::None => {},
            DepthStencilAttachment::Depth(depth) => {
                let image_view = depth.image_view;
                let images = self.cmd.recorder.images();
                let image = images.get(image_view.image_id()).unwrap();
                let state = image.view_state(
                    image_view, ImageAspects::DEPTH
                )?;
                let vk = depth_attachment.as_mut().unwrap();
                vk.image_layout = state.layout;
                if let Some(resolve) = depth.resolve {
                    let image = images.get(resolve.image_view.image_id()).unwrap();
                    let state = image.view_state(
                        resolve.image_view, ImageAspects::DEPTH,
                    )?;
                    vk.resolve_image_layout = state.layout;
                }
            },
            DepthStencilAttachment::Stencil(stencil) => {
                let image_view = stencil.image_view;
                let images = self.cmd.recorder.images();
                let image = images.get(image_view.image_id()).unwrap();
                let state = image.view_state(
                    image_view, ImageAspects::STENCIL
                )?;
                let vk = stencil_attachment.as_mut().unwrap();
                vk.image_layout = state.layout;
                if let Some(resolve) = stencil.resolve {
                    let image = images.get(resolve.image_view.image_id()).unwrap();
                    let state = image.view_state(
                        resolve.image_view, ImageAspects::STENCIL
                    )?;
                    vk.resolve_image_layout = state.layout;
                }
            },
            DepthStencilAttachment::DepthStencil { depth, stencil } => {
                let image_view = depth.image_view;
                let images = self.cmd.recorder.images();
                let image = images.get(image_view.image_id()).unwrap();
                let state = image.view_state(
                    image_view, ImageAspects::DEPTH
                )?;
                let vk_depth = depth_attachment.as_mut().unwrap();
                vk_depth.image_layout = state.layout;
                if let Some(resolve) = depth.resolve {
                    let image = images.get(resolve.image_view.image_id()).unwrap();
                    let state = image.view_state(
                        resolve.image_view, ImageAspects::DEPTH
                    )?;
                    vk_depth.resolve_image_layout = state.layout;
                }
                let image_view = stencil.image_view;
                let images = self.cmd.recorder.images();
                let image = images.get(image_view.image_id()).unwrap();
                let state = image.view_state(
                    image_view, ImageAspects::STENCIL
                )?;
                let vk_stencil = stencil_attachment.as_mut().unwrap();
                vk_stencil.image_layout = state.layout;
                if let Some(resolve) = stencil.resolve {
                    let image = images.get(resolve.image_view.image_id()).unwrap();
                    let state = image.view_state(
                        resolve.image_view, ImageAspects::STENCIL
                    )?;
                    vk_stencil.resolve_image_layout = state.layout;
                }
            },
        };
        let rendering_info = vk::RenderingInfo {
            s_type: vk::StructureType::RENDERING_INFO,
            flags: vk::RenderingFlags::CONTENTS_SECONDARY_COMMAND_BUFFERS,
            render_area: rendering_info.render_area
                .unwrap_or(RenderArea {
                    width: min_extent.width,
                    height: min_extent.height,
                    ..Default::default()
                }).into(),
            layer_count: rendering_info.layer_count,
            color_attachment_count: color_attachments.len() as u32,
            p_color_attachments: color_attachments.as_ptr(),
            p_depth_attachment: depth_attachment.as_ref().as_ptr(),
            p_stencil_attachment: stencil_attachment.as_ref().as_ptr(),
            ..Default::default()
        };
        unsafe {
            self.cmd.gpu.device().cmd_begin_rendering(
                primary_command_buffer,
                &rendering_info
            );
            if !command_buffers.is_empty() {
                self.cmd.gpu.device()
                    .cmd_execute_commands(primary_command_buffer, &command_buffers);
            }
            self.cmd.gpu.device()
                .cmd_end_rendering(primary_command_buffer);
            self.color_formats.drop_and_free(&self.alloc);
        }
        cmd_cache.graphics_command_cache.next_draw_command_storage = 0;
        cmd_cache.graphics_command_cache.draw_ids.clear();
        Ok(())
    }
}

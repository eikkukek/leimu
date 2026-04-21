use tuhka::vk;
use leimu_mem::{
    vec::{FixedVec32, Vec32},
    alloc::LocalAlloc,
    arena,
};

use crate::{
    gpu::prelude::*,
    error::*,
    threads::executor::block_on,
};

#[derive(Default)]
pub(crate) struct ComputeCommandCache {
    pipelines: Vec32<PipelineHandle>,
}

impl ComputeCommandCache {

    #[inline(always)]
    fn reset(&mut self) {
        self.pipelines.clear();
    }
}

pub struct ComputeCommands<'a, 'b> {
    recorder: CommandRecorder<'a, 'b>,
    gpu: Gpu,
    queue: DeviceQueue,
    current_pipeline: Option<ComputePipeline>,
    primary_command_buffer: vk::CommandBuffer,
    command_id: CommandId,
    wait_scope: vk::PipelineStageFlags2,
    signal_scope: vk::PipelineStageFlags2,
}

pub struct NewComputeCommands;

impl NewCommands for NewComputeCommands {

    const NAME: &'static str = "compute commands";

    type Target<'a, 'b> = ComputeCommands<'a, 'b>;

    fn new<'a, 'b>(
        mut recorder: CommandRecorder<'a, 'b>,
        command_id: CommandId,
        queue: DeviceQueue,
    ) -> Result<Self::Target<'a, 'b>>
        where Self::Target<'a, 'b>: Commands<'a, 'b>
    {
        if !queue.queue_flags().contains(QueueFlags::COMPUTE) {
            return Err(Error::just_context(format!(
                "queue {queue} can't be used as a compute queue"
            )))
        }
        let primary_command_buffer = recorder
            .get_current_worker()
            .allocate_primaries(&queue, 1)?[0];
        let begin_info = vk::CommandBufferBeginInfo {
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            ..Default::default()
        };
        let gpu = recorder.gpu().clone();
        unsafe {
            gpu.device().begin_command_buffer(
                primary_command_buffer,
                &begin_info
            ).context("failed to begin command buffer")?;
        }
        Ok(ComputeCommands {
            recorder,
            gpu,
            queue,
            current_pipeline: None,
            primary_command_buffer,
            command_id,
            wait_scope: vk::PipelineStageFlags2::empty(),
            signal_scope: vk::PipelineStageFlags2::empty(),
        })
    }
}

unsafe impl<'a, 'b> Commands<'a, 'b> for ComputeCommands<'a, 'b> {

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

impl<'a, 'b> ComputeCommands<'a, 'b> {

    pub fn bind_pipeline<F>(
        &mut self,
        id: ComputePipelineId,
        f: F,
    ) -> Result<()>
        where F: FnOnce(&mut PipelineCommands) -> EventResult<()>
    {
        let alloc = self.recorder.stack().clone();
        let alloc = alloc.guard();
        let pipeline = self.current_pipeline.insert(
            block_on(self.gpu.get_compute_pipeline(id))?.clone()
        );
        let cmd_cache = unsafe {
            &mut *self.recorder.cache().get()
        };
        cmd_cache.compute_command_cache.pipelines.push(pipeline.handle().clone());
        let command_buffer = self.primary_command_buffer;
        unsafe {
            self.gpu.device().cmd_bind_pipeline(
                command_buffer, vk::PipelineBindPoint::COMPUTE,
                pipeline.handle().handle(),
            );
        }
        cmd_cache.compute_command_cache.reset();
        self.recorder.get_current_worker().add_pipeline(pipeline.handle().clone());
        let buffers = self.gpu.read_buffers();
        let images = self.gpu.read_images();
        let mut commands = unsafe { PipelineCommands::new(
            self.gpu.clone(),
            command_buffer,
            pipeline.handle().clone(),
            &mut cmd_cache.pipeline_cache,
            &alloc,
            &buffers,
            &images,
        ) };
        f(&mut commands).context_from_tracked(|orig| format!(
            "failed to record pipeline commands at {}", orig.or_this(),
        ))?;
        unsafe {
            let tmp_alloc = self.gpu.tmp_alloc();
            let tmp_alloc = tmp_alloc.guard();
            cmd_cache.pipeline_cache.prepare_shader_resource_cache(
                &mut self.recorder,
                &tmp_alloc,
            ).context("failed to process pipeline commands")?;
            cmd_cache.shader_resource_cache.process(
                &mut self.recorder, command_buffer,
                self.queue.family_index(),
                self.command_id,
                &tmp_alloc,
            ).context("failed to process pipeline commands")?;
            cmd_cache.pipeline_cache.reset(&alloc);
        }
        Ok(())
    }

    #[inline(always)]
    pub fn dispatch(
        &mut self,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) -> Result<()>
    {
        if self.current_pipeline.is_none() {
            return Err(Error::just_context("attempting to dispatch with no pipeline bound"))
        }
        unsafe {
            self.gpu.device().cmd_dispatch(
                self.primary_command_buffer,
                group_count_x,
                group_count_y,
                group_count_z
            );
        }
        Ok(())
    }
}

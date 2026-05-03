use core::{
    cell::UnsafeCell,
    ops::DerefMut,
    num::NonZeroU64,
    marker::PhantomData,
};

use parking_lot::{RwLock, RwLockWriteGuard, RwLockReadGuard};
use ahash::{AHashMap, AHashSet};
use tuhka::vk;
use leimu_mem::{
    alloc::{LocalAlloc, LocalAllocExt, Layout},
    arena::{self, Arena},
    slot_map::*,
    vec::{Vec32, FixedVec32, NonNullVec32},
    vec32,
    conditional::True,
    int::NonZeroOption,
};


use crate::{
    caller, core::{OptionExt, TryExtend}, error::*, gpu::{prelude::{
        command_cache::*,
        *,
    }, semaphore_create_info}, sync::*
};

#[derive(Default)]
pub(super) struct FlushResources {
    registered_buffers: AHashSet<BufferId>,
    flush_buffers: Vec32<BufferId>,
    registered_images: AHashSet<ImageIndex>,
    flush_images: Vec32<ImageIndex>,
}

impl FlushResources {

    #[inline]
    fn reset(&mut self) {
        self.registered_buffers.clear();
        self.flush_buffers.clear();
        self.registered_images.clear();
        self.flush_images.clear();
    }
}

struct Inner {
    frame_semaphore: TimelineSemaphoreId,
    current_frame: u64,
    workers: Vec32<SchedulerWorker>,
    free_worker: u32,
    commands: SlotMap<CommandFrameResources>,
    stack: Arc<Arena<True>>,
    command_resources: Vec32<CommandResources>,
    flush_resources: FlushResources,
}

impl Inner {

    fn new(gpu: Gpu, num_workers: u32) -> Result<Self>
    {
        let mut frame_semaphore = Default::default();
        gpu.create_timeline_semaphores([semaphore_create_info(&mut frame_semaphore, 0)])?;
        let mut workers = vec32![];
        workers.try_extend((0..num_workers).map(|_| {
            SchedulerWorker::new(gpu.clone())
        }))?;
        Ok(Self {
            frame_semaphore,
            current_frame: 0,
            workers,
            free_worker: 0,
            commands: SlotMap::with_capacity(8),
            stack: Arc::new(Arena::with_fallback(gpu.memory_layout().tmp_arena_size())
                .context("failed to create arena alloc")?
            ),
            command_resources: vec32![],
            flush_resources: FlushResources::default(),
        })
    } 
}

pub struct CommandScheduler<'a> {
    inner: UnsafeCell<RwLockWriteGuard<'a, Inner>>,
    gpu: Gpu,
}

impl<'a> CommandScheduler<'a> {

    fn allocate_new_resources(&mut self) -> Result<()> {
        let n_new_resources = self.inner.get_mut().commands.capacity() -
            self.inner.get_mut().command_resources.len();
        let capacity = self.inner.get_mut().commands.capacity();
        self.inner.get_mut().command_resources.reserve(capacity);
        let mut new_ids = vec32![TimelineSemaphoreId::default(); n_new_resources];
        self.gpu.create_timeline_semaphores(
            new_ids.iter_mut().map(|id| semaphore_create_info(id, 0))
        )?;
        self.inner.get_mut().command_resources.extend(
            new_ids.iter().map(|&id| CommandResources::new(id))
        );
        Ok(())
    }

    #[track_caller]
    pub fn new_commands<Cmd>(
        &mut self,
        queue: DeviceQueue,
        f: impl for<'b, 'c> FnOnce(
            &mut Cmd::Target<'b, 'c>
        ) -> EventResult<()> + Send + Sync + 'static,
    ) -> Result<CommandBuilder<'_>>
        where
            Cmd: NewCommands,
            for<'b, 'c> Cmd::Target<'b, 'c>: Commands<'b, 'c>
    {
        let mut f = Some(f);
        let mut queue = Some(queue);
        let f = super::command::make_fn_record_command(move |rec, id, alloc| {
            let mut command = Cmd::new(rec, id, queue.take().unwrap()).context_with(|| format!(
                "failed to initialize {}", Cmd::NAME
            ))?;
            (f.take().unwrap())(&mut command)
            .context_from_tracked(|orig| format!(
                "failed to record {} at {}", Cmd::NAME, orig.or_this(),
            ))?;
            command.finish(alloc)
        });
        let fp = unsafe {
            let ptr = self.inner
                .get_mut().stack
                .alloc_uninit(1)
                .context("alloc failed")?;
            ptr.write(f);
            ptr
        };
        let idx = self.inner.get_mut().commands.insert(CommandFrameResources {
            fp,
            dep: Default::default(),
            loc: caller!(),
        });
        if idx.index() >= self.inner.get_mut().command_resources.len() {
            self.allocate_new_resources()
                .context("failed to allocate command resources")?;
        }
        unsafe {
            let frame_resources = (*self.inner.get()).deref_mut().commands.get_mut(idx).unwrap();
            let resources = &mut (*self.inner.get()).deref_mut().command_resources[idx.index() as usize];
            Ok(CommandBuilder { id: CommandId(idx), frame_resources, resources, })
        }
    }
}

#[derive(Clone)]
pub(crate) struct QueueScheduler {
    inner: Arc<RwLock<Inner>>,
}

pub struct QueueSchedulerReadGuard<'a> {
    inner: RwLockReadGuard<'a, Inner>,
}

impl<'a> QueueSchedulerReadGuard<'a> {

    #[inline]
    pub fn get_frame_semaphore_id(&self) -> TimelineSemaphoreId {
        self.inner.frame_semaphore
    }
}

impl QueueScheduler {

    pub fn new(
        gpu: Gpu,
        num_workers: u32,
    ) -> Result<Self>
    {
        Ok(Self {
            inner: Arc::new(RwLock::new(Inner::new(gpu, num_workers)?)),
        })
    }

    #[inline]
    pub fn read(&self) -> QueueSchedulerReadGuard<'_> {
        QueueSchedulerReadGuard { inner: self.inner.read(), }
    }

    #[inline]
    pub fn schedule(&self, gpu: Gpu) -> CommandScheduler<'_> {
        CommandScheduler {
            inner: UnsafeCell::new(self.inner.write()),
            gpu,
        }
    }

    pub fn record<'a, F, Alloc>(
        &self,
        gpu: &Gpu,
        cache: &mut UnsafeCell<CommandRecorderCache>,
        event_handler: &mut F,
        alloc: &'a Alloc,
    ) -> Result<Submits<'a, Alloc>>
        where
            F: FnMut(Event) -> EventResult<()>,
            Alloc: LocalAlloc<Error = arena::Error>,
    {
        let inner = self.inner.write();
        let mut recorder = CommandRecorderInner {
            surfaces: gpu.write_surfaces(),
            gpu: gpu.clone(),
            inner,
            cache,
        };
        recorder.compile(event_handler, alloc)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct PresentSwapchain {
    pub present_id2: Option<NonZeroU64>,
    pub swapchain: vk::SwapchainKHR,
    pub image_index: u32,
    pub wait_semaphore: vk::Semaphore,
}

#[derive(Default)]
pub(crate) struct PresentSubmits {
    submits: AHashMap<DeviceQueue, Vec32<PresentSwapchain>>,
}

impl PresentSubmits {

    pub unsafe fn add_swapchain(
        &mut self,
        queue: DeviceQueue,
        info: PresentSwapchain,
    ) {
        self.submits
            .entry(queue)
            .and_modify(|s| {
                s.push(info)
            }).or_insert_with(|| {
                vec32![info]
            });
    }

    fn clear(&mut self) {
        for p in self.submits.values_mut() {
            p.clear();
        }
    }
}

#[derive(Default)]
pub struct CommandRecorderCache {
    pub(crate) shader_resource_cache: ShaderResourceCache,
    pub(super) graphics_command_cache: GraphicsCommandCache,
    pub(super) compute_command_cache: ComputeCommandCache,
    pub pipeline_cache: PipelineCommandCache,
    pub(crate) present_submits: PresentSubmits,
}

struct CommandRecorderInner<'a> {
    gpu: Gpu,
    inner: RwLockWriteGuard<'a, Inner>,
    surfaces: ResourceWriteGuard<'a, Surface, SurfaceId>,
    cache: &'a mut UnsafeCell<CommandRecorderCache>,
}

impl CommandRecorderInner<'_> {

    fn compile<'a, F, Alloc>(
        &mut self,
        event_handler: &mut F,
        alloc: &'a Alloc,
    ) -> Result<Submits<'a, Alloc>>
        where
            F: FnMut(Event) -> EventResult<()>,
            Alloc: LocalAlloc<Error = arena::Error>,
    {
        self.cache.get_mut().present_submits.clear();
        let surfaces: *mut SlotMap<Surface> = self.surfaces.deref_mut();
        for (id, surface) in unsafe { &mut *surfaces }.iter_mut() {
            let data = surface
                .acquire_next_image(CommandRecorder::new(self))
                .context_with(|| format!("failed to acquire surface {id} image"))?;
            if let Some(image_count) = data.recreated_image_count {
                (event_handler)(Event::SwapchainCreated {
                    surface_id: SurfaceId(id),
                    new_format: data.image_format,
                    new_size: data.extent,
                    image_count: image_count.get(),
                }).context_from_tracked(|orig| format!(
                    "swapchain create event error at {}", orig.or_this(),
                ))?;
            }
        }
        let max_index = self.inner.commands.capacity();
        let mut in_degree = FixedVec32
            ::with_len(max_index, 0, alloc)
            .context("alloc error")?;
        let mut dependents = FixedVec32
            ::with_len_with(max_index, |_| None, alloc)
            .context("alloc error")?;
        let mut queue = FixedVec32
            ::with_capacity(self.inner.commands.len(), alloc)
            .context("alloc error")?;
        for (idx, _) in &self.inner.commands {
            let resources = &self.inner.command_resources[idx.index() as usize];
            let in_deg = resources.dependencies.len();
            if in_deg == 0 {
                queue.push(idx);
            }
            in_degree[idx.index() as usize] = in_deg;
            for &dep in &resources.dependencies {
                let dep = &mut dependents[dep.dependency.index() as usize];
                let dep = match dep {
                    Some(dep) => dep,
                    None => dep.insert(FixedVec32
                        ::with_capacity(queue.capacity(), alloc)
                        .context("alloc error")?
                    ),
                };
                dep.push(idx);
            }
        }
        let mut free_worker = None;
        let mut fallback_index = (0, u64::MAX);
        self.inner.current_frame += 1;
        let current_frame = self.inner.current_frame;
        for (i, worker) in self.inner.workers.iter_mut().enumerate() {
            let result = worker
                .reset(&self.gpu, current_frame)
                .context("unexpected worker reset error")?;
            match result {
                CommandPoolResetResult::Ready(id, value) => {
                    free_worker = Some((i, id, value));
                    break
                },
                CommandPoolResetResult::Pending(last_reset) => if last_reset < fallback_index.1 {
                    fallback_index = (i, last_reset)
                }
            }
        }
        let &mut free_worker = OptionExt::get_or_try_insert_with(
            &mut free_worker,
            || {
                let (id, value) = self.inner.workers[fallback_index.0]
                    .wait_and_reset(&self.gpu, current_frame)?;
                Ok((fallback_index.0, id, value))
            }
        )?;
        self.inner.free_worker = free_worker.0 as u32;
        let mut sorted = FixedVec32
            ::with_capacity(queue.capacity(), alloc)
            .context("alloc error")?;
        while let Some(node) = queue.pop() {
            sorted.push(node);
            for &dep in dependents[node.index() as usize]
                .as_deref()
                .unwrap_or_default()
            {
                let in_deg = &mut in_degree[dep.index() as usize];
                *in_deg -= 1;
                if *in_deg == 0 {
                    queue.push(dep);
                }
            }
        }
        if sorted.len() != self.inner.commands.len() {
            return Err(Error::just_context(
                "topological sort failed"
            ))
        }
        let mut submits = Submits {
            submits: FixedVec32
                ::with_capacity(sorted.len() + 2, alloc)
                .context("alloc error")?,
            present_submits: FixedVec32::new(alloc),
        };
        let mut global_wait_semaphore_infos = NonNullVec32
            ::with_capacity(sorted.len() + 1, alloc)
            .context("alloc failed")?;
        let mut swapchain_wait_semaphores = NonNullVec32
            ::with_capacity(sorted.len(), alloc)
            .context("alloc failed")?;
        for &idx in &sorted {
            let index = idx.index() as usize;
            let mut command = self.inner.commands.get(idx).unwrap().clone();
            let cmd_resources = &mut self.inner.command_resources[index];
            let timeline_value = cmd_resources.timeline_value + 1;
            for &dep in &command.dep {
                if let Err(err) = self.inner.commands.get(dep.dependency.0) {
                    return Err(Error::new(err, format!(
                        "command {} had a dependency with an invalid id {}",
                        command.loc, dep.dependency,
                    )))
                }
                let wait_for = &self.inner.command_resources[dep.dependency.0.index() as usize];
                let semaphore_id = wait_for.semaphore_id;
                let semaphore_value = wait_for.timeline_value;
                unsafe {
                    self.inner.command_resources.get_unchecked_mut(index)
                        .add_wait_for_semaphore(semaphore_id, semaphore_value, dep.hint);
                }
            }
            let command_result = unsafe {
                let (result, size, align) = {
                    let f = command.fp.as_mut();
                    (f(CommandRecorder::new(self), CommandId(idx), alloc)?,
                    size_of_val(f),
                    align_of_val(f)
                )
                };
                command.fp.drop_in_place();
                self.inner.stack.free_raw(
                    command.fp.cast(),
                    Layout::from_size_align_unchecked(size, align)
                );
                result
            };
            {
                let mut buffers = self.gpu.write_buffers();
                for &id in &self.inner.flush_resources.flush_buffers {
                    if let Ok(buffer) = buffers.get_mut(id.slot_index()) {
                        buffer.flush_state();
                    }
                }
            }
            {
                let mut images = self.gpu.write_images();
                for &id in &self.inner.flush_resources.flush_images {
                    if let Ok(image) = images.get_mut(id) {
                        image.flush_subresources();
                    }
                }
            }
            self.inner.flush_resources.reset();
            let command_resources = unsafe {
                self.inner.command_resources.get_unchecked(index)
            };
            let signal = unsafe { self.gpu
                .get_timeline_semaphore(command_resources.semaphore_id)
                .unwrap_unchecked()
            };
            let mut submit_info = SubmitInfo {
                device_queue_index: command_result.queue.device_queue_index(),
                wait_semaphore_infos: NonNullVec32::with_capacity(
                    command_resources.wait_semaphore_cache.len(),
                    alloc
                ).context("alloc failed")?,
                command_buffer_infos: NonNullVec32::with_capacity(
                    command_result.primary_command_buffers.len(),
                    alloc
                ).context("alloc failed")?,
                signal_semaphore_infos: NonNullVec32::with_capacity(
                    1 + command_resources.signal_semaphores.len(), alloc
                ).context("alloc failed")?,
                alloc,
            };
            submit_info.wait_semaphore_infos.try_extend(command_resources.wait_semaphore_cache
                .iter()
                .map(|&id| {
                    let &(value, dependency_hint) = command_resources.wait_semaphores
                        .get(&id).unwrap();
                    if let Some(signal) = command_resources.signal_semaphores
                        .iter()
                        .filter_map(|&signal| (signal.0 == id).then_some(signal.1))
                        .find(|&signal| signal <= value)
                    {
                        return Err(Error::just_context(format!(
                            "{}{}",
                            format_args!("command id {} semaphore {id} signal value {signal} was less than or ",
                                CommandId(idx)
                            ),
                            format_args!("equal to wait value {value}")
                        )))
                    }
                    Ok(vk::SemaphoreSubmitInfo {
                        semaphore: self.gpu.get_timeline_semaphore(id)?,
                        value,
                        stage_mask: if dependency_hint.is_empty() {
                            command_result.wait_scope
                        } else {
                            dependency_hint.into()
                        },
                        ..Default::default()
                    })
                })
            )?;
            submit_info.command_buffer_infos.extend(command_result.primary_command_buffers
                .iter().map(|&command_buffer| {
                    vk::CommandBufferSubmitInfo {
                        command_buffer,
                        ..Default::default()
                    }
                })
            );
            submit_info.signal_semaphore_infos.push(vk::SemaphoreSubmitInfo {
                semaphore: signal,
                value: timeline_value,
                stage_mask: command_result.signal_scope,
                ..Default::default()
            });
            submit_info.signal_semaphore_infos.try_extend(command_resources.signal_semaphores
                .iter()
                .map(|&(id, value)| {
                    let semaphore = self.gpu.get_timeline_semaphore(id)?;
                    Ok(vk::SemaphoreSubmitInfo {
                        semaphore,
                        value,
                        stage_mask: command_result.signal_scope,
                        ..Default::default()
                    })
                })
            )?;
            submits.submits.push(submit_info);
            if !command_resources.touched_swapchain_images.is_empty() {
                swapchain_wait_semaphores.push(vk::SemaphoreSubmitInfo {
                    semaphore: signal,
                    value: timeline_value,
                    stage_mask: vk::PipelineStageFlags2::NONE,
                    ..Default::default()
                });
            }
            unsafe { self.inner.command_resources.get_unchecked_mut(index) }.finish(timeline_value);
            global_wait_semaphore_infos.push(vk::SemaphoreSubmitInfo {
                semaphore: signal,
                value: timeline_value,
                stage_mask: vk::PipelineStageFlags2::NONE,
                ..Default::default()
            });
        }
        self.inner.commands.clear();
        let present_prep_semaphore = unsafe {
            self.gpu.get_timeline_semaphore(
                self.inner.workers[self.inner.free_worker as usize]
                .present_prep_semaphore
            ).unwrap_unchecked()
        };
        if !self.surfaces.is_empty() {
            let queue = self.gpu
                .any_device_queue(QueueFlags::empty())
                .expect("no queues created");
            let free_worker = self.inner.free_worker as usize;
            let command_buffer = self.inner.workers[free_worker]
                .allocate_primaries(&queue, 1)?[0];
            let begin_info = vk::CommandBufferBeginInfo {
                ..Default::default()
            };
            unsafe {
                self.gpu.device().begin_command_buffer(
                    command_buffer,
                    &begin_info
                ).context("failed to begin command buffer")?;
            }
            let surfaces: *mut _ = &mut *self.surfaces;
            let surfaces = unsafe {
                &mut *surfaces
            };
            for (id, surface) in surfaces.iter_mut() {
                surface.get_present_submit(CommandRecorder::new(self), command_buffer)
                    .context_with(|| format!(
                        "failed to get present submit from surface {id}"
                    ))?;
            }
            unsafe {
                self.gpu.device().end_command_buffer(
                    command_buffer
                ).context("failed to end command buffer")?;
            }
            let mut submit = SubmitInfo {
                device_queue_index: queue.device_queue_index(),
                wait_semaphore_infos: swapchain_wait_semaphores,
                command_buffer_infos: NonNullVec32
                    ::with_capacity(1, alloc)
                    .context("alloc failed")?,
                signal_semaphore_infos: Default::default(),
                alloc,
            };
            submit.command_buffer_infos.push(vk::CommandBufferSubmitInfo {
                command_buffer,
                ..Default::default()
            });
            let worker = &self.inner.workers[self.inner.free_worker as usize];
            let n_present_submits = self.cache.get_mut().present_submits.submits.len() as u32;
            let mut all_signal_semaphores = FixedVec32
                ::with_capacity(n_present_submits, alloc)
                .context("alloc failed")?;
            submits.present_submits = FixedVec32
                ::with_capacity(n_present_submits, alloc)
                .context("alloc failed")?;
            for (queue, swapchains) in &self.cache.get_mut().present_submits.submits {
                if swapchains.is_empty() {
                    continue
                }
                let n_swapchains = swapchains.len();
                let mut wait_semaphores = FixedVec32
                    ::with_capacity(n_swapchains, alloc)
                    .context("alloc failed")?;
                let mut handles = FixedVec32
                    ::with_capacity(n_swapchains, alloc)
                    .context("alloc failed")?;
                let mut indices = FixedVec32
                    ::with_capacity(n_swapchains, alloc)
                    .context("alloc failed")?;
                let mut present_id2 = FixedVec32
                    ::with_capacity(n_swapchains, alloc)
                    .context("alloc failed")?;
                let mut signal_semaphores = FixedVec32
                    ::with_capacity(n_swapchains, alloc)
                    .context("alloc failed")?;
                for s in swapchains.iter() {
                    wait_semaphores.push(s.wait_semaphore);
                    handles.push(s.swapchain);
                    indices.push(s.image_index);
                    present_id2.push(s.present_id2.unwrap_or_sentinel(0));
                    signal_semaphores.push(vk::SemaphoreSubmitInfo {
                        semaphore: s.wait_semaphore,
                        value: 0,
                        stage_mask: vk::PipelineStageFlags2::NONE,
                        ..Default::default()
                    });
                }
                all_signal_semaphores.push(signal_semaphores);
                submits.present_submits.push(PresentSubmit {
                    queue: queue.handle(),
                    wait_semaphores,
                    swapchains: handles,
                    image_indices: indices,
                    present_id2,
                });
            }
            let n: u32 = all_signal_semaphores.iter().map(|s| s.len()).sum();
            submit.signal_semaphore_infos = NonNullVec32::with_capacity(
                n + 1,
                alloc
            ).context("alloc failed")?;
            submit.signal_semaphore_infos.push(vk::SemaphoreSubmitInfo {
                semaphore: present_prep_semaphore,
                value: worker.present_prep_value + 1,
                stage_mask: vk::PipelineStageFlags2::NONE,
                ..Default::default()
            });
            for semaphores in all_signal_semaphores {
                submit.signal_semaphore_infos.fast_append(&semaphores);
            }
            submits.submits.push(submit);
            global_wait_semaphore_infos.push(vk::SemaphoreSubmitInfo {
                semaphore: present_prep_semaphore,
                value: worker.present_prep_value + 1,
                stage_mask: vk::PipelineStageFlags2::NONE,
                ..Default::default()
            });
        }
        let mut global_signal_semaphore_infos = NonNullVec32
            ::with_capacity(2, alloc)
            .context("alloc failed")?;
        let worker = &self.inner.workers[self.inner.free_worker as usize];
        global_signal_semaphore_infos.fast_append(&[
            vk::SemaphoreSubmitInfo {
                s_type: vk::StructureType::SEMAPHORE_SUBMIT_INFO,
                semaphore: unsafe { self.gpu
                    .get_timeline_semaphore(worker.semaphore_id) 
                    .unwrap_unchecked()
                },
                value: worker.timeline_value,
                stage_mask: vk::PipelineStageFlags2::BOTTOM_OF_PIPE,
                ..Default::default()
            },
            vk::SemaphoreSubmitInfo {
                s_type: vk::StructureType::SEMAPHORE_SUBMIT_INFO,
                semaphore: unsafe { self.gpu
                    .get_timeline_semaphore(self.inner.frame_semaphore) 
                    .unwrap_unchecked()
                },
                value: current_frame,
                stage_mask: vk::PipelineStageFlags2::BOTTOM_OF_PIPE,
                ..Default::default()
            },
        ]);
        submits.submits.push(SubmitInfo {
            device_queue_index: self.gpu
                .any_device_queue(QueueFlags::empty())
                .expect("no queues created")
                .device_queue_index(),
            wait_semaphore_infos: global_wait_semaphore_infos,
            command_buffer_infos: Default::default(),
            signal_semaphore_infos: global_signal_semaphore_infos,
            alloc
        });
        self.inner.workers[free_worker.0].present_prep_value += 1;
        unsafe {
            self.inner.stack.clear();
        }
        Ok(submits)
    }
}

pub struct CommandRecorder<'a, 'b> {
    inner: *mut CommandRecorderInner<'b>,
    _marker: PhantomData<& 'a ()>,
}

impl<'a, 'b> AsRef<CommandRecorderInner<'b>> for CommandRecorder<'a, 'b> {
    
    #[inline]
    fn as_ref(&self) -> &CommandRecorderInner<'b> {
        unsafe {
            &*self.inner
        }
    }
}

impl<'a, 'b> AsMut<CommandRecorderInner<'b>> for CommandRecorder<'a, 'b> {

    #[inline]
    fn as_mut(&mut self) -> &mut CommandRecorderInner<'b> {
        unsafe {
            &mut *self.inner
        }
    }
}

pub(crate) struct SubmitInfo<'a, Alloc>
    where Alloc: LocalAlloc,
{
    pub device_queue_index: u32,
    pub wait_semaphore_infos: NonNullVec32<'a, vk::SemaphoreSubmitInfo<'static>>,
    pub command_buffer_infos: NonNullVec32<'a, vk::CommandBufferSubmitInfo<'static>>,
    pub signal_semaphore_infos: NonNullVec32<'a, vk::SemaphoreSubmitInfo<'static>>,
    pub alloc: &'a Alloc,
}

impl<'a, Alloc> Drop for SubmitInfo<'a, Alloc>
    where Alloc: LocalAlloc,
{

    fn drop(&mut self) {
        unsafe {
            self.wait_semaphore_infos.drop_and_free(self.alloc);
            self.command_buffer_infos.drop_and_free(self.alloc);
            self.signal_semaphore_infos.drop_and_free(self.alloc);
        }
    }
}

pub(crate) struct PresentSubmit<'a, Alloc>
    where Alloc: LocalAlloc<Error = arena::Error>
{
    pub queue: vk::Queue,
    pub wait_semaphores: FixedVec32<'a, vk::Semaphore, Alloc>,
    pub swapchains: FixedVec32<'a, vk::SwapchainKHR, Alloc>,
    pub image_indices: FixedVec32<'a, u32, Alloc>,
    pub present_id2: FixedVec32<'a, u64, Alloc>,
}

pub struct ImageBufferWriteGuard<'a> {
    buffers: ResourceWriteGuard<'a, BufferMeta, BufferId>,
    images: ResourceWriteGuard<'a, ImageMeta, ImageIndex>,
    inner: &'a mut Inner,
}

impl ImageBufferWriteGuard<'_> {

    #[inline]
    pub fn register_buffer(&mut self, id: BufferId) -> Result<&mut BufferMeta> {
        if self.inner.flush_resources.registered_buffers.insert(id) {
            self.inner.flush_resources.flush_buffers.push(id);
        }
        self.buffers.get_mut(id)
    }

    #[inline]
    pub fn register_image(
        &mut self,
        id: ImageIndex,
        command_index: u32,
    ) -> Result<&mut ImageMeta>
    {
        if self.inner.flush_resources.registered_images.insert(id) {
            self.inner.flush_resources.flush_images.push(id);
        }
        let image = self.images
            .get_mut(id)?;
        if command_index != COMMAND_INDEX_IGNORED && image.is_swapchain() {
            self.inner.command_resources[command_index as usize]
                .touched_swapchain_images.insert(id);
        }
        Ok(image)
    }
}

pub(crate) struct Submits<'a, Alloc>
    where Alloc: LocalAlloc<Error = arena::Error>,
{
    pub submits: FixedVec32<'a, SubmitInfo<'a, Alloc>, Alloc>,
    pub present_submits: FixedVec32<'a, PresentSubmit<'a, Alloc>, Alloc>,
}

impl<'a, 'b> CommandRecorder<'a, 'b> {

    #[inline]
    fn new(inner: &'a mut CommandRecorderInner<'b>) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn forward(&mut self) -> CommandRecorder<'_, 'b> {
        CommandRecorder {
            inner: self.inner,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn gpu(&self) -> &Gpu {
        &self.as_ref().gpu
    }

    #[inline]
    pub fn stack(&self) -> &Arc<Arena<True>> {
        &self.as_ref().inner.stack
    }

    #[inline]
    pub fn cache(&mut self) -> &mut UnsafeCell<CommandRecorderCache> {
        self.as_mut().cache
    }

    #[inline]
    pub fn get_current_worker(&mut self) -> &mut SchedulerWorker {
        let free_worker = self.as_ref().inner.free_worker;
        &mut self.as_mut().inner.workers[free_worker as usize]
    } 

    #[inline]
    pub fn current_frame(&self) -> u64 {
        self.as_ref().inner.current_frame
    }

    #[inline]
    pub fn swapchain_image_view(
        &self,
        surface_id: SurfaceId,
    ) -> Result<(SwapchainImageViewId<'_>, Format)> {
        let id = self.as_ref().surfaces
            .get(surface_id)?
            .current_image_view();
        let format = self.images()
            .get(id.image_id().slot_index())
            .unwrap().properties().format;
        Ok((id, format))
    }

    #[inline]
    pub fn add_signal_semaphore(
        &self,
        command_id: CommandId,
        semaphore_id: TimelineSemaphoreId,
        value: u64,
    ) {
        unsafe { &mut *self.inner }
            .inner.command_resources[command_id.index() as usize]
            .signal_semaphores.push((semaphore_id, value));
    }
    
    #[inline]
    pub fn add_wait_semaphore(
        &self,
        command_id: CommandId,
        semaphore_id: TimelineSemaphoreId,
        value: u64,
        dependency_hint: MemoryDependencyHint,
    ) {
        unsafe { &mut *self.inner }
            .inner.command_resources[command_id.index() as usize]
            .add_wait_for_semaphore(semaphore_id, value, dependency_hint);
    }

    #[inline]
    pub(crate) fn buffers(&self) -> ResourceReadGuard<'_, BufferMeta, BufferId> {
        self.as_ref().gpu.read_buffers::<BufferId>()
    }

    #[inline]
    pub(crate) fn images(&self) -> ResourceReadGuard<'_, ImageMeta, ImageIndex> {
        self.as_ref().gpu.read_images()
    }

    #[inline]
    pub fn write_resources<F, T>(&mut self, f: F) -> Result<T>
        where F: FnOnce(&mut ImageBufferWriteGuard) -> Result<T>,
    {
        let mut guard = ImageBufferWriteGuard {
            buffers: unsafe { &*self.inner }.gpu.write_buffers(),
            images: unsafe { &*self.inner }.gpu.write_images(),
            inner: &mut unsafe { &mut *self.inner }.inner
        };
        f(&mut guard)
    }

    pub(crate) unsafe fn create_swapchain_images(
        &self,
        images: &swapchain::SwapchainImages<'_>,
        out: &mut [ImageViewId],
        alloc: &impl LocalAlloc<Error = arena::Error>,
    ) -> Result<()>
    {
        let dimensions = images.extent.into();
        let device = self.gpu().device().clone();
        let mut img = self.gpu().write_images::<ImageId>();
        for (i, &handle) in images.handles.iter().enumerate() {
            let index = img.insert(unsafe { ImageMeta::from_swapchain_image(
                device.clone(),
                handle,
                dimensions,
                images.format,
                images.usage,
                alloc,
            )?});
            out[i] = ImageViewId::new(
                ImageId::new(index),
                unsafe {
                    img.get_unchecked_mut(index)
                        .create_view(ImageRange::whole_range(ImageAspects::COLOR))?
                }
            );
        }
        Ok(())
    }

    pub(crate) fn destroy_swapchain_images(
        &self,
        views: &[ImageViewId],
    ) {
        let mut img = self.gpu().write_images::<ImageIndex>();
        for view in views {
            img.remove(view.image_id().slot_index()).ok();
        }
    } 
}

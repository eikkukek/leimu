use core::{
    ptr::NonNull,
    fmt::{self, Display},
};

use ahash::{AHashMap, AHashSet};

use leimu_mem::{
    alloc::LocalAlloc,
    arena,
    vec::{Vec32, NonNullVec32, FixedVec32},
    slot_map::*,
    conditional::True,
    vec32,
};
use tuhka::vk;

use crate::{
    bitflags,
    error::*,
    gpu::prelude::*,
    location,
};

/// An opaque handle to a [`command buffer`][1].
///
/// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBuffer.html
pub type CommandBuffer = vk::CommandBuffer;

pub struct CommandResult<'a, Alloc>
    where Alloc: LocalAlloc + ?Sized,
{
    pub primary_command_buffers: FixedVec32<'a, CommandBuffer, Alloc>,
    pub wait_scope: vk::PipelineStageFlags2,
    pub signal_scope: vk::PipelineStageFlags2,
    pub queue: DeviceQueue,
}

/// A builder trait for [`Commands`]
pub trait NewCommands {

    const NAME: &'static str;

    type Target<'a, 'b>;

    fn new<'a, 'b>(
        recorder: CommandRecorder<'a, 'b>,
        command_id: CommandId,
        queue: DeviceQueue,
    ) -> Result<Self::Target<'a, 'b>>
        where Self::Target<'a, 'b>: Commands<'a, 'b>;
}

/// A trait for [`recording`][1] commands with a [`command buffer`][2] and
/// [`submitting them to a queue`][3]
///
/// # Safety
/// There are a multitude of safety considerations when implementing this trait yourself.
///
/// Resources need to be handled with proper [`pipeline barriers`][4], command buffers need to be
/// recorded with proper validation and the implementation needs to fit in the Leimu [`command recording`][1]
/// scheme.
///
/// [1]: CommandRecorder
/// [2]: https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBuffer.html
/// [3]: https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html
/// [4]: Device::cmd_pipeline_barrier2
pub unsafe trait Commands<'a, 'b>: Sized
{
    fn add_signal_semaphore(
        &mut self, 
        semaphore_id: TimelineSemaphoreId,
        value: u64,
    );

    fn add_wait_semaphore(
        &mut self,
        semaphore_id: TimelineSemaphoreId,
        value: u64,
        dependency_hint: MemoryDependencyHint,
    );
    
    fn finish<'c, Alloc>(self, alloc: &'c Alloc) -> Result<CommandResult<'c, Alloc>>
        where Alloc: ?Sized + LocalAlloc<Error = arena::Error>;
}

pub(super) type FpRecordCommand = dyn for<'a, 'b> FnMut(
    CommandRecorder,
    CommandId,
    &'a (dyn LocalAlloc<Error = arena::Error> + 'b),
) -> Result<CommandResult<'a, dyn LocalAlloc<Error = arena::Error> + 'b>> + Send + Sync + 'static;

#[inline]
pub(super) fn make_fn_record_command<F>(
    mut f: F
) -> impl for <'a, 'b> FnMut(
    CommandRecorder,
    CommandId,
    &'a (dyn LocalAlloc<Error = arena::Error> + 'b)
) -> Result<CommandResult<'a, dyn LocalAlloc<Error = arena::Error> + 'b>>
    + 'static
    + Send + Sync
    where F: for<'a, 'b> FnMut(
        CommandRecorder,
        CommandId,
        &'a (dyn LocalAlloc<Error = arena::Error> + 'b)
    ) -> Result<CommandResult<'a, dyn LocalAlloc<Error = arena::Error> + 'b>> +
        'static +
        Send + Sync
{
    move |rec, id, alloc| {
        (f)(rec, id, alloc)
    }
}

/// Specifies how strict [`buffer memory barriers`][1] are when recording commands.
///
/// This is ignored for [`image memory barriers`][2] as they need to always be strict.
///
/// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferMemoryBarrier2.html
/// [2]: https://docs.vulkan.org/refpages/latest/refpages/source/VkImageMemoryBarrier2.html
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum CommandOrdering {
    #[default]
    /// Specifies that memory barriers are only created when necessary.
    Lenient,
    /// Specifies that memory barriers are created on every buffer access.
    Strict,
}

bitflags!(
    /// Specifies what kind of accesses to a resource will be made when recording commands.
    pub struct ExplicitAccess: Flags64 {
        /// Specifies no accesses to a resource.
        NONE = 0x0,
        /// Specifies that a resource will be read from in a shader.
        SHADER_READ = vk::AccessFlags2::SHADER_READ.as_raw(),
        /// Specifies that a resource will be written to in a shader.
        SHADER_WRITE = vk::AccessFlags2::SHADER_WRITE.as_raw(),
        /// Specifies that the resource will be read from and writen to in a shader.
        SHADER_READ_AND_WRITE =
            Self::SHADER_READ.as_raw() |
            Self::SHADER_WRITE.as_raw(),
        /// Specifies that the resource will be used for color attachment read operations.
        COLOR_ATTACHMENT_READ = vk::AccessFlags2::COLOR_ATTACHMENT_READ.as_raw(),
        /// Specifies that the resource will be used for color attachment write operations.
        COLOR_ATTACHMENT_WRITE = vk::AccessFlags2::COLOR_ATTACHMENT_WRITE.as_raw(),
        /// Specifies that the resource will be used as a color attachment.
        COLOR_ATTACHMENT =
            Self::COLOR_ATTACHMENT_READ.as_raw() |
            Self::COLOR_ATTACHMENT_WRITE.as_raw(),
        /// Specifies that resource will be used for depth stencil attachment read operations.
        DEPTH_STENCIL_ATTACHMENT_READ = vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_READ.as_raw(),
        /// Specifies that resource will be used for depth stencil attachment write operations.
        DEPTH_STENCIL_ATTACHMENT_WRITE = vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE.as_raw(),
        /// Specifies that the resource will be used as a depth/stencil attachment.
        DEPTH_STENCIL_ATTACHMENT =
            Self::DEPTH_STENCIL_ATTACHMENT_READ.as_raw() |
            Self::DEPTH_STENCIL_ATTACHMENT_WRITE.as_raw(),
    }
);

impl From<ExplicitAccess> for vk::AccessFlags2 {

    #[inline]
    fn from(value: ExplicitAccess) -> Self {
        Self::from_raw(value.as_raw())
    }
}

/// Specifies [`buffer`][1] and [`image`][2] memory barrier behavior when recording commands.
///
/// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferMemoryBarrier2.html
/// [2]: https://docs.vulkan.org/refpages/latest/refpages/source/VkImageMemoryBarrier2.html
#[derive(Clone, Copy)]
pub struct CommandBarrierInfo {
    pub ordering: CommandOrdering,
    pub access: ExplicitAccess,
}

impl CommandBarrierInfo {

    #[inline]
    pub fn new(
        ordering: CommandOrdering,
        access: ExplicitAccess
    ) -> Self {
        Self {
            ordering,
            access,
        }
    }
}

#[derive(Clone, Copy)]
pub struct BindingBarrierInfo {
    pub set: u32,
    pub binding: u32,
    pub barrier_info: CommandBarrierInfo,
}

impl BindingBarrierInfo {

    #[inline]
    pub fn new(
        set: u32,
        binding: u32,
        barrier_info: CommandBarrierInfo,
    ) -> Self {
        Self {
            set,
            binding,
            barrier_info,
        }
    }
}

/// An ID to a [`command`][1], which can be used as a dependency for other commands.
///
/// Note that this ID is ephemeral and becomes invalid once the [`Gpu ticks`][2].
///
/// [1]: Commands
/// [2]: Gpu::tick
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CommandId(pub(crate) SlotIndex<CommandFrameResources>);

impl Display for CommandId {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CommandId {

    /// Gets the index part of [`CommandId`].
    #[inline]
    pub fn index(self) -> u32 {
        self.0.index()
    }
}

#[derive(Clone, Copy)]
pub struct CommandDependency {
    pub dependency: CommandId,
    pub hint: MemoryDependencyHint,
}

impl CommandDependency {

    #[inline]
    pub fn new(
        dependency: CommandId,
        hint: MemoryDependencyHint,
    ) -> Self {
        Self {
            dependency,
            hint,
        }
    }
}

pub(crate) struct SchedulerCommandPool {
    pool: vk::CommandPool,
    primaries: Vec32<CommandBuffer>,
    next_primary: u32,
    secondaries: Vec32<CommandBuffer>,
    next_secondary: u32,
}

impl SchedulerCommandPool {

    #[inline]
    pub fn new(
        device: &Device,
        queue_family_index: u32,
    ) -> Result<Self>
    {
        let create_info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            flags: vk::CommandPoolCreateFlags::TRANSIENT,
            queue_family_index,
            ..Default::default()
        };
        let pool = unsafe {
            device
                .create_command_pool(&create_info, None)
                .context("failed to create command pool")?
                .value
        };
        Ok(Self {
            pool,
            primaries: vec32![],
            next_primary: 0,
            secondaries: vec32![],
            next_secondary: 0,
        })
    }

    pub fn allocate_primaries(
        &mut self,
        device: &Device,
        count: u32,
    ) -> Result<&[CommandBuffer]> {
        let new_next_primary = self.next_primary + count;
        if new_next_primary > self.primaries.len() {
            let old_n = self.primaries.len();
            let new_n = (old_n + count).next_power_of_two();
            self.primaries.resize(new_n, Default::default());
            let new_buffers = &mut self.primaries[old_n as usize..new_n as usize];
            let n_alloc = new_buffers.len() as u32;
            let alloc_info = vk::CommandBufferAllocateInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                command_pool: self.pool,
                level: vk::CommandBufferLevel::PRIMARY,
                command_buffer_count: n_alloc,
                ..Default::default()
            };
            unsafe {
                device.allocate_command_buffers(
                    &alloc_info,
                    new_buffers
                ).context("failed to allocate command buffers")?;
            }
        }
        let buffers = &self.primaries[self.next_primary as usize..new_next_primary as usize];
        self.next_primary = new_next_primary;
        Ok(buffers)
    }

    pub fn allocate_secondaries(
        &mut self,
        device: &Device,
        count: u32,
    ) -> Result<&[CommandBuffer]> {
        let new_next_secondary = self.next_secondary + count;
        if new_next_secondary > self.secondaries.len() {
            let old_n = self.secondaries.len();
            let new_n = (old_n + count).next_power_of_two();
            self.secondaries.resize(new_n, Default::default());
            let new_buffers = &mut self.secondaries[old_n as usize..new_n as usize];
            let n_alloc = new_buffers.len() as u32;
            let alloc_info = vk::CommandBufferAllocateInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                command_pool: self.pool,
                level: vk::CommandBufferLevel::SECONDARY,
                command_buffer_count: n_alloc,
                ..Default::default()
            };
            unsafe {
                device.allocate_command_buffers(
                    &alloc_info,
                    new_buffers
                ).context("failed to allocate command buffers")?;
            }
        }
        let buffers = &self.secondaries[self.next_secondary as usize..new_next_secondary as usize];
        self.next_secondary = new_next_secondary;
        Ok(buffers)
    }

    #[inline]
    pub unsafe fn reset(&mut self, device: &Device) -> Result<()> {
        unsafe {
            device.reset_command_pool(
                self.pool, vk::CommandPoolResetFlags::empty(),
            ).context("failed to reset command pool")?;
            self.next_primary = 0;
            self.next_secondary = 0;
        }
        Ok(())
    }

    #[inline]
    pub unsafe fn destroy(&mut self, device: &Device) {
        unsafe {
            device.destroy_command_pool(self.pool, None);
        }
    }
}

pub(crate) enum CommandPoolResetResult {
    Ready(TimelineSemaphoreId, u64),
    Pending(u64),
}

pub struct SchedulerWorker {
    pub(super) device: Device,
    command_pools: AHashMap<DeviceQueue, SchedulerCommandPool>,
    pub(super) semaphore_id: TimelineSemaphoreId,
    pub(super) timeline_value: u64,
    pub(super) last_reset: u64,
    pub(super) present_prep_semaphore: TimelineSemaphoreId,
    pub(super) present_prep_value: u64,
    pub(super) samplers: AHashSet<Sampler>,
    pub(super) pipelines: AHashSet<PipelineHandle>,
}

impl SchedulerWorker {

    pub(super) fn new(gpu: Gpu) -> Result<Self> {
        let device = gpu.device();
        let mut semaphore_id = Default::default();
        let mut present_prep_semaphore = Default::default();
        gpu.create_timeline_semaphores([
            (&mut semaphore_id, 0),
            (&mut present_prep_semaphore, 0)
        ])?;
        let mut command_pools = AHashMap::default();
        for queue in device.device_queues() {
            command_pools.entry(
                queue.clone()
            ).insert_entry(SchedulerCommandPool::new(device, queue.family_index())?);
        }
        Ok(Self {
            command_pools,
            device: gpu.device().clone(),
            semaphore_id,
            timeline_value: 0,
            last_reset: 0,
            present_prep_semaphore,
            present_prep_value: 0,
            samplers: AHashSet::default(),
            pipelines: AHashSet::default(),
        })
    }

    #[inline(always)]
    pub fn allocate_primaries(
        &mut self,
        queue: &DeviceQueue,
        count: u32,
    ) -> Result<&[CommandBuffer]> {
        self.command_pools
            .get_mut(queue)
            .ok_or_else(|| Error::just_context(format!("invalid device queue {queue}")))?
            .allocate_primaries(&self.device, count)
            .context_with(|| format!("failed to allocate primary command buffers for queue {queue}"))
    }

    #[inline(always)]
    pub fn allocate_secondaries(
        &mut self,
        queue: &DeviceQueue,
        count: u32,
    ) -> Result<&[CommandBuffer]> {
        self.command_pools
            .get_mut(queue)
            .ok_or_else(|| Error::just_context(format!("invalid device queue {queue}")))?
            .allocate_secondaries(&self.device, count)
            .context_with(|| format!("failed to allocate secondary command buffers for queue {queue}"))
    }

    #[inline(always)]
    pub fn add_sampler(&mut self, sampler: Sampler) {
        self.samplers.insert(sampler);
    }

    #[inline(always)]
    pub fn add_pipeline(&mut self, pipeline: PipelineHandle) {
        self.pipelines.insert(pipeline);
    }

    #[inline(always)]
    pub(crate) fn reset(
        &mut self,
        gpu: &Gpu,
        current_frame: u64,
    ) -> Result<CommandPoolResetResult> {
        if gpu.get_semaphore_counter_value(self.semaphore_id)? >= self.timeline_value {
            let device = &self.device;
            for pool in self.command_pools.values_mut() {
                unsafe {
                    pool.reset(device)?;
                }
            }
            self.timeline_value += 1;
            self.last_reset = current_frame;
            self.samplers.clear();
            self.pipelines.clear();
            Ok(CommandPoolResetResult::Ready(self.semaphore_id, self.timeline_value))
        } else {
            Ok(CommandPoolResetResult::Pending(self.last_reset))
        }
    }

    #[inline(always)]
    pub(crate) fn wait_and_reset(
        &mut self,
        gpu: &Gpu,
        current_frame: u64,
    ) -> Result<(TimelineSemaphoreId, u64)> {
        if gpu.wait_for_semaphores(
            &[(self.semaphore_id, self.timeline_value)],
            core::time::Duration::from_nanos(self.device.frame_timeout()),
        )? {
            self.timeline_value += 1;
            self.last_reset = current_frame;
            self.samplers.clear();
            self.pipelines.clear();
            Ok((self.semaphore_id, self.timeline_value))
        } else {
            Err(Error::just_context(format!(
                "frame timeout {} nanoseconds reached at {}", gpu.device().frame_timeout(), location!(),
            )))
        }
    }
}

impl Drop for SchedulerWorker {

    fn drop(&mut self) {
        unsafe {
            let device = &self.device;
            for pool in self.command_pools.values_mut() {
                pool.destroy(device);
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct CommandFrameResources {
    pub(super) fp: NonNull<FpRecordCommand>,
    pub(super) dep: NonNullVec32<'static, CommandDependency, True>,
    pub(super) loc: Location,
}

unsafe impl Send for CommandFrameResources {}
unsafe impl Sync for CommandFrameResources {}

pub(crate) struct CommandResources {
    pub(super) semaphore_id: TimelineSemaphoreId,
    pub(super) timeline_value: u64,
    pub(super) dependencies: Vec32<CommandDependency>,
    pub(super) signal_semaphores: Vec32<(TimelineSemaphoreId, u64)>,
    pub(super) wait_semaphores: AHashMap<TimelineSemaphoreId, (u64, MemoryDependencyHint)>,
    pub(super) wait_semaphore_cache: Vec32<TimelineSemaphoreId>,
    pub(super) touched_swapchain_images: AHashSet<ImageIndex>,
}

impl CommandResources {

    pub(super) fn new(semaphore_id: TimelineSemaphoreId) -> Self
    {
        Self {
            dependencies: vec32![],
            signal_semaphores: vec32![],
            semaphore_id,
            timeline_value: 0,
            wait_semaphores: AHashMap::default(),
            wait_semaphore_cache: vec32![],
            touched_swapchain_images: AHashSet::default(),
        }
    }

    #[inline(always)]
    pub fn add_wait_for_semaphore(
        &mut self,
        id: TimelineSemaphoreId,
        value: u64,
        dependency_hint: MemoryDependencyHint,
    ) {
        self.wait_semaphores
            .entry(id)
            .and_modify(|(v, hint)| {
                *v = (*v).max(value);
                *hint |= dependency_hint;
            }).or_insert_with(|| {
                self.wait_semaphore_cache.push(id);
                (value, dependency_hint)
            });
    }

    #[inline(always)]
    pub fn finish(&mut self, timeline_value: u64) {
        self.timeline_value = timeline_value;
        self.dependencies.clear();
        self.signal_semaphores.clear();
        self.wait_semaphores.clear();
        self.wait_semaphore_cache.clear();
        self.touched_swapchain_images.clear();
    }
}

/// A handle to a command, which can be used to add dependencies to said command.
///
/// To get the ID of a command use [`CommandBuilder::id`].
///
/// To create a command, you need to use the [`CommandScheduler`] provided by the event loop.
pub struct CommandBuilder<'a> {
    pub(super) id: CommandId,
    pub(super) frame_resources: &'a mut CommandFrameResources,
    pub(super) resources: &'a mut CommandResources,
}

impl<'a> CommandBuilder<'a> {

    /// Adds local [`dependencies`][1].
    ///
    /// Note that [`command ids`][2] become *invalid* after the [`Gpu ticks`][3].
    ///
    /// [1]: CommandDependency
    /// [2]: CommandId
    /// [3]: Gpu::tick
    #[inline(always)]
    pub fn with_dependencies(
        self,
        dependency: impl IntoIterator<Item = CommandDependency>,
    ) -> Self
    {
        self.resources.dependencies.extend(dependency);
        self
    }

    /// Adds a wait [`semaphore`][1].
    ///
    /// You can optionally specify where the dependency is [`waited on`][2].
    ///
    /// If you want the scope of the dependency to be inferred, set `dependency hint` to [`NONE`][3].
    ///
    /// # Valid usage
    /// - If the same [`semaphore`][1] is [`added as a signal semaphore`][4], the value of the wait
    ///   semaphore *must* be less than the signal value.
    ///
    /// # Vulkan docs
    /// <https://github.khronos.org/Vulkan-Site/spec/latest/chapters/synchronization.html>
    ///
    /// [1]: TimelineSemaphoreId
    /// [2]: MemoryDependencyHint
    /// [3]: MemoryDependencyHint::NONE
    /// [4]: Self::with_signal_semaphore
    #[inline(always)]
    pub fn with_wait_semaphore(
        self,
        id: TimelineSemaphoreId,
        value: u64,
        dependency_hint: MemoryDependencyHint,
    ) -> Self 
    {
        self.resources.add_wait_for_semaphore(id, value, dependency_hint);
        self
    }

    /// Adds a signal [`semaphore`][1].
    ///
    /// # Valid usage
    /// - If the same [`semaphore`][1] is [`added as a wait semapore semaphore`][2], the value of
    ///   signal semaphore *must* be greater than the wait value.
    ///
    /// [1]: TimelineSemaphoreId
    /// [2]: Self::with_wait_semaphore
    #[inline(always)]
    pub fn with_signal_semaphore(
        self,
        id: TimelineSemaphoreId,
        value: u64,
    ) -> Self {
        self.resources.signal_semaphores.push((id, value));
        self
    }

    #[inline(always)]
    pub fn id(self) -> CommandId {
        self.id
    }
}

impl<'a> Drop for CommandBuilder<'a> {

    fn drop(&mut self) {
        unsafe {
            self.frame_resources.dep =
            NonNullVec32::new(
                NonNull::new_unchecked(self.resources.dependencies.as_mut_ptr()),
                self.resources.dependencies.capacity()
            ).with_len(self.resources.dependencies.len()).into_clonable()
        }
    }
}

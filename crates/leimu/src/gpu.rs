//! Everything [`Gpu`] related.
//!
//! See [`Instance`], [`Device`] and [`Gpu`] to get started.

pub mod ext;
mod memory_binder;
mod device;

mod instance;
mod queue;
mod physical_device;
mod definitions;
mod version;
mod enums;
#[allow(missing_docs)]
mod format;
mod memory_layout;
mod handle;
mod shader;
mod shader_set;
mod descriptor;
mod attributes;
mod subresource_state;
mod pipeline;
mod surface;
mod sampler;
mod image;
mod buffer;
mod swapchain;
mod resources;
mod commands;
mod event;

use core::{
    ops::Deref,
    fmt::{self, Display},
    cell::UnsafeCell,
    num::NonZeroU32,
    time::Duration,
};

use ahash::AHashMap;

use leimu_mem::{
    vec::{FixedVec32, Vec32},
    slot_map::SlotMap,
    conditional::True,
    vec32,
    arena::Arena,
};
use tuhka::vk;

use crate::{
    error::*,
    sync::{atomic::AtomicU64, *},
    executor::{ThreadPool, block_on},
    core::collections::EntryExt,
};

/// Vulkan bindings used by leimu.
pub use tuhka;

pub(crate) mod prelude {

    use super::*;

    pub use {
        device::*,
        instance::*,
        super::Gpu,
        super::DeviceId,
        super::queue::*,
        attributes::*,
        definitions::*,
        version::*,
        enums::*,
        memory_layout::MemoryLayout,
        handle::*,
        sampler::*,
        image::*,
        buffer::*,
        physical_device::*,
        resources::*,
        pipeline::*,
        commands::prelude::*,
        leimu_proc::VertexInput,
        shader::*,
        super::shader_set::*,
        super::descriptor::*,
        pipeline::vertex_input::*,
        super::memory_binder::*,
        super::ext,
        surface::VulkanWindow,
        super::event::Event,
        super::format::*,
    };

    pub type DeviceName = ([u8; 256], usize);

    pub(crate) use surface::Surface;

    pub(super) use swapchain::Swapchain;
    pub(super) use super::swapchain;
    pub(super) use super::commands;
    pub(super) use super::subresource_state;

    pub(crate) const COMMAND_INDEX_IGNORED: u32 = u32::MAX;
}
use commands::scheduler::QueueScheduler;

pub use prelude::*;

pub(crate) struct MtContext {
    fallback_alloc: Arc<Arena<True>>,
    tmp_allocs: AHashMap<std::thread::ThreadId, Arc<Arena<True>>>,
}

impl MtContext {

    #[inline]
    pub fn tmp_alloc(
        &self
    ) -> Arc<Arena<True>>
    {
        self.tmp_allocs
            .get(&std::thread::current().id())
            .cloned()
            .unwrap_or(self.fallback_alloc.clone())
    }
}

/// Parameters used when creating [`Cache`].
#[derive(Clone, Copy)]
pub struct CacheAttributes {
    /// The local allocated arena size, in bytes.
    ///
    /// The default is 65536 bytes.
    pub arena_size: usize,
}

impl Default for CacheAttributes {

    #[inline]
    fn default() -> Self {
        Self {
            arena_size: 1 << 16,
        }
    }
}

/// An id assigned to a [`Device`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DeviceId(u64);

impl Display for DeviceId {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

static DEVICE_ID: AtomicU64 = AtomicU64::new(0);

/// A cache used by [`Gpu`] that is **not** thread-safe.
///
/// This type does **not** implement [`Send`] or [`Sync`] and must only be accessed from 
/// a single thread.
///
/// See [`create_cache`] for full explanation.
pub struct Cache {
    command_cache: UnsafeCell<CommandRecorderCache>,
    arena: Arena<True>,
    id: Option<DeviceId>,
    submit_cache: Box<[Vec32<vk::SubmitInfo2<'static>>]>,
}

impl Cache {

    fn init(
        &mut self,
        gpu: &Gpu,
    ) -> Result<()> {
        if let Some(id) = self.id {
            if id != gpu.device().id() {
                return Err(Error::just_context(
                    "gpu cache cannot be reused between different Gpu instances"
                ))
            }
        } else {
            self.id = Some(gpu.device().id());
            self.submit_cache = (0..gpu.device().device_queues().len())
                .map(|_| vec32![])
                .collect();
        }
        Ok(())
    }
}

/// Creates a [`Cache`], which is needed for recording commands with the [`Gpu`] via [`tick`][1].
///
/// [`Cache`] is *not* [`Send`] or [`Sync`] and should only be used from a single thread.
///
/// [1]: Gpu::tick
pub fn create_cache(
    attributes: CacheAttributes,
) -> Cache
{
    Cache {
        command_cache: Default::default(),
        arena: Arena
            ::with_fallback(attributes.arena_size)
            .expect("global alloc failed"),
        id: None,
        submit_cache: Default::default(),
    }
}

/// Specifies the parameters for [`creating`][1] timeline semaphores.
///
/// [1]: Gpu::create_timeline_semaphores
pub struct SemaphoreCreateInfo<'a> {
    /// Mutable reference to where the [`id`][1] will be returned to.
    ///
    /// [1]: TimelineSemaphoreId
    pub out_id: &'a mut TimelineSemaphoreId,
    /// The initial value of the timeline semaphore,
    pub initial_value: u64,
}

/// Creates [`SemaphoreCreateInfo`].
#[inline]
pub fn semaphore_create_info<'a>(
    out_id: &'a mut TimelineSemaphoreId,
    initial_value: u64
) -> SemaphoreCreateInfo<'a> {
    SemaphoreCreateInfo { out_id, initial_value }
}

/// Specifies the parameters for [`waiting`][1] on a semaphore on the host.
///
/// [1]: Gpu::wait_for_semaphores
#[derive(Clone, Copy)]
pub struct SemaphoreWaitInfo {
    /// The id of the semaphore to be waited upon.
    pub id: TimelineSemaphoreId,
    /// The value to wait upon.
    pub value: u64,
}

#[inline]
pub fn semaphore_wait_info(
    id: TimelineSemaphoreId,
    value: u64
) -> SemaphoreWaitInfo {
    SemaphoreWaitInfo { id, value }
}

struct GpuInner {
    thread_pool: ThreadPool,
    memory_layout: MemoryLayout,
    queue_scheduler: OnceLock<QueueScheduler>,
    shader_cache: RwLock<ShaderCache>,
    pipeline_batches: SwapLock<SlotMap<OnceLock<PipelineBatch>>>,
    descriptor_pools: SwapLock<SlotMap<DescriptorPool>>,
    surfaces: RwLock<SlotMap<Surface>>,
    buffers: RwLock<SlotMap<BufferMeta>>,
    images: RwLock<SlotMap<ImageMeta>>,
    timeline_semaphores: RwLock<SlotMap<vk::Semaphore>>,
    draw_commands: RwLock<SlotMap<DrawCommandResource>>,
    mt_ctx: Arc<MtContext>,
    desired_buffered_frames: u32,
    device: Device,
}

/// The GPU interface of leimu.
///
/// A [`Clone`] + [`Send`] + [`Sync`] handle.
#[derive(Clone)]
pub struct Gpu {
    inner: Arc<GpuInner>,
}

impl Gpu {

    /// Creates the [`Gpu`] in a standalone mode.
    ///
    /// This is useful when the application has its own event loop/windowing system, or when
    /// running the [`Gpu`] in [`headless`][1] mode.
    ///
    /// To advance the GPU loop, simply call [`tick`][2] once per frame or whenever you want to
    /// progress command submission.
    ///
    /// Surfaces can be created via [`create_surface`][3] and swapchain updates can be requested via
    /// [`request_swapchain_update`][4].
    ///
    /// Everything else is handled by [`tick`][2].
    ///
    /// [1]: crate::Entry::headless
    /// [2]: Self::tick
    /// [3]: Self::create_surface
    /// [4]: Self::request_swapchain_update
    pub fn standalone(
        device: Device,
        thread_pool: ThreadPool,
        memory_layout: MemoryLayout,
        desired_buffered_frames: NonZeroU32,
    ) -> Result<Self> {
        let main_tmp_alloc = Arena
            ::with_fallback(memory_layout.tmp_arena_size())
            .context("failed to create arena alloc")?;
        let mut tmp_allocs = AHashMap::default();
        tmp_allocs.insert(
            std::thread::current().id(),
            Arc::new(Arena::with_fallback(memory_layout.tmp_arena_size())
                .context("failed to create arena alloc")?)
        );
        for id in thread_pool.worker_threads() {
            tmp_allocs
                .entry(id)
                .or_try_insert_with(|| Ok(Arc::new(Arena::with_fallback(
                    memory_layout.tmp_arena_size()
                ).context("failed to create arena alloc")?)))?;
        }
        let command_workers = device.command_workers();
        let s = Self{inner:Arc::new(GpuInner {
            thread_pool,
            queue_scheduler: OnceLock::new(),
            shader_cache: RwLock::new(ShaderCache::new(device.clone())),
            surfaces: RwLock::new(SlotMap::new()),
            device,
            pipeline_batches: SwapLock::default(),
            descriptor_pools: SwapLock::new(SlotMap::new()),
            images: RwLock::new(SlotMap::new()),
            buffers: RwLock::new(SlotMap::new()),
            timeline_semaphores: RwLock::new(SlotMap::new()),
            draw_commands: RwLock::new(SlotMap::new()),
            mt_ctx: Arc::new(MtContext {
                fallback_alloc: Arc::new(main_tmp_alloc),
                tmp_allocs,
            }),
            desired_buffered_frames: desired_buffered_frames.get(),
            memory_layout,
        })};
        let queue_scheduler = QueueScheduler::new(s.clone(), command_workers)
            .context_with(|| "failed to create queue scheduler")?;
        s.inner.queue_scheduler.get_or_init(|| {
            queue_scheduler
        });
        Ok(s)
    }

    #[cfg(feature = "event-loop")]
    pub(crate) fn new(
        event_loop: &crate::event_loop::EventLoop,
        device: Device,
        attributes: crate::Attributes,
    ) -> Result<(Self, Cache)>
    {
        Ok((Self::standalone(
            device,
            event_loop.thread_pool(),
            attributes.gpu_memory_layout,
            NonZeroU32::new(attributes.desired_buffered_frames).unwrap(),
        )?, create_cache(attributes.gpu_cache_attributes)))
    }

    /// Gets the [`limits`][1] of the [`Device`] used to create this [`Gpu`].
    ///
    /// [1]: PhysicalDeviceLimits
    #[inline]
    pub fn device_limits(&self) -> &PhysicalDeviceLimits {
        self.inner.device.physical_device().limits()
    }

    /// Returns [`base device features`][1] enabled on the [`Device`] used to create this [`Gpu`].
    ///
    /// [1]: BaseDeviceFeatures
    #[inline]
    pub fn enabled_base_features(&self) -> &BaseDeviceFeatures {
        self.inner.device.base_device_features()
    }

    /// Gets an [`extension`][ext] device.
    #[inline]
    pub fn get_extension_device<T: ext::ExtensionDevice>(&self) -> Option<&T> {
        self.inner.device.get_extension_device()
    }

    /// Gets an [`extension`][ext] attribute.
    #[inline]
    pub fn get_device_attribute(&self, name: ext::ConstName) -> &ext::DeviceAttribute {
        self.inner.device.get_device_attribute(name)
    }

    #[inline]
    pub(crate) fn memory_layout(&self) -> MemoryLayout {
        self.inner.memory_layout
    }

    #[inline]
    pub(crate) fn thread_pool(&self) -> ThreadPool {
        self.inner.thread_pool.clone()
    }

    #[inline]
    pub(crate) fn tmp_alloc(
        &self,
    ) -> Arc<Arena<True>>
    {
        self.inner.mt_ctx.tmp_alloc()
    }

    /// Gets the [`Device`] used to create this [`Gpu`].
    #[inline]
    pub fn device(&self) -> &Device {
        &self.inner.device
    }

    #[inline]
    pub(crate) fn queue_scheduler(&self) -> &QueueScheduler {
        unsafe {
            self.inner.queue_scheduler.get().unwrap_unchecked()
        }
    }
    
    /// Returns the api version of this [`Gpu`].
    #[inline]
    pub fn api_version(&self) -> Version {
        self.inner.device.physical_device().api_version()
    }

    /// Returns the [`physical device`][1] of this [`Gpu`].
    ///
    /// [1]: PhysicalDevice
    #[inline]
    pub fn physical_device(&self) -> &PhysicalDevice {
        self.inner.device.physical_device()
    }

    /// Searches for any [`DeviceQueue`], which matches `flags`.
    #[inline]
    pub fn any_device_queue(&self, flags: QueueFlags) -> Option<DeviceQueue> {
        self.inner.device.any_device_queue(flags)
    } 
    
    /// Gets [`ImageFormatProperties`] for the given parameters.
    pub fn get_image_format_properties(
        &self,
        format: Format,
        usage: ImageUsages,
        is_3d: bool,
        has_mutable_format: bool,
        is_cube_map_compatible: bool,
    ) -> Result<ImageFormatProperties>
    {
        let vk_format: vk::Format = format.into();
        let mut flags = vk::ImageCreateFlags::empty();
        if has_mutable_format {
            flags |= vk::ImageCreateFlags::MUTABLE_FORMAT;
        }
        if is_cube_map_compatible {
            flags |= vk::ImageCreateFlags::CUBE_COMPATIBLE;
        }
        let format_info = vk::PhysicalDeviceImageFormatInfo2 {
            format: vk_format,
            ty: if is_3d {
                    vk::ImageType::TYPE_3D
                } else {
                    vk::ImageType::TYPE_2D
                },
            tiling: vk::ImageTiling::OPTIMAL,
            usage: usage.into(),
            flags,
            ..Default::default()
        };
        let mut image_format_prop = vk::ImageFormatProperties2::default();
        unsafe {
            self.inner.device.instance()
                .get_physical_device_image_format_properties2(
                    self.inner.device.physical_device().handle(),
                    &format_info, &mut image_format_prop
                ).context("failed to get image format properties")?;
        }
        let image_format_prop = image_format_prop.image_format_properties;
        let mut format_properties3 = vk::FormatProperties3::default();
        let mut format_properties = vk::FormatProperties2
            ::default().push_next(&mut format_properties3);
        unsafe {
            self.inner.device.instance().get_physical_device_format_properties2(
                self.inner.device.physical_device().handle(),
                vk_format, &mut format_properties,
            );
        }
        let format_features = FormatFeatures::from_raw(
            format_properties3.optimal_tiling_features.as_raw()
        );
        let mut max_dimensions: Dimensions = image_format_prop.max_extent.into();
        if usage.intersects(
            ImageUsages::COLOR_ATTACHMENT |
            ImageUsages::DEPTH_STENCIL_ATTACHMENT |
            ImageUsages::INPUT_ATTACHMENT
        ) {
            let limits = self.inner.device.physical_device().limits();
            max_dimensions.width = max_dimensions.width.min(limits.max_framebuffer_width());
            max_dimensions.height = max_dimensions.width.min(limits.max_framebuffer_height());
            max_dimensions.depth = 1;
        }
        Ok(ImageFormatProperties {
            max_dimensions,
            max_mip_levels: image_format_prop.max_mip_levels,
            max_array_layers: image_format_prop.max_array_layers,
            sample_counts: image_format_prop.sample_counts.into(),
            format_features,
        })
    }

    /// Returns the user-defined desired buffered frames used when creating swapchains.
    pub fn desired_buffered_frames(&self) -> u32 {
        self.inner.desired_buffered_frames
    }

    /// Creates a surface for `window`.
    pub fn create_surface<H: VulkanWindow>(
        &self,
        window: Arc<H>,
    ) -> Result<SurfaceId> {
        let mut surfaces = self.inner.surfaces.write();
        Ok(SurfaceId(surfaces.insert(Surface::new(
            window,
            self.clone(),
            self.inner.desired_buffered_frames,
        )?)))
    }

    /// Request swapchain update for a surface.
    pub fn request_swapchain_update(
        &self,
        surface_id: SurfaceId,
        framebuffer_size: (u32, u32)
    ) -> Result<()> {
        self.inner.surfaces
            .write()
            .get_mut(surface_id.slot_index())
            .context_with(|| format!(
                "invalid surface id {surface_id}"
            ))?.request_swapchain_update(
                self.inner.desired_buffered_frames, framebuffer_size
            );
        Ok(())
    }

    /// Destroys a previously [`created`][1] surface.
    ///
    /// [1]: Self::create_surface
    pub fn destroy_surface(
        &self,
        surface_id: SurfaceId,
    ) -> Result<()> {
        self.inner.surfaces
            .write()
            .remove(surface_id.slot_index())
            .context_with(|| format!(
                "invalid surface id {surface_id}"
            ))?;
        Ok(())
    }
   
    /// Schedules a batch of [`commands`][1].
    ///
    /// Commands created in the closure are guaranteed to be recorded in the same scope.
    ///
    /// [`CommandId`]s of commands created in the same scope **can** be used as [`dependencies`][2]
    /// of other commands in that scope.
    ///
    /// [1]: Commands
    /// [2]: CommandDependency
    #[inline]
    pub fn schedule_commands<F>(&self, f: F) -> Result<()>
        where F: FnOnce(&mut CommandScheduler<'_>) -> EventResult<()>
    {
        let mut scheduler = unsafe {
            self.inner.queue_scheduler
            .get()
            .unwrap_unchecked()
        }.schedule(self.clone());
        f(&mut scheduler).context_from_tracked(|orig| {
            format!("failed to schedule commands at {}", orig.or_this())
        })?;
        Ok(())
    }
    
    /// Creates static [`DrawCommands`], which can be used when drawing with [`GraphicsCommands`].
    pub fn create_draw_commands<F>(
        &self,
        command_pool: &mut CommandPool,
        info: DrawCommandInfo,
        arena_size: usize,
        f: F,
    ) -> Result<DrawCommandId>
        where F: FnOnce(&mut DrawCommands) -> EventResult<()>,
    {
        let alloc = Arena
            ::with_fallback(arena_size)
            .context("failed to create arena")?;
        let command_buffer = command_pool
            .allocate_primaries(1)?[0];
        let mut rendering_inheritance_info = vk::CommandBufferInheritanceRenderingInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO,
            color_attachment_count: info.color_formats.len() as u32,
            p_color_attachment_formats: info.color_formats.as_ptr().cast(),
            depth_attachment_format: info.depth_format.into(),
            stencil_attachment_format: info.stencil_format.into(),
            rasterization_samples: info.sample_count.into(),
            ..Default::default()
        };
        let inheritance_info = vk::CommandBufferInheritanceInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_INHERITANCE_INFO,
            ..Default::default()
        }.push_next(&mut rendering_inheritance_info);
        let begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            p_inheritance_info: &inheritance_info,
            ..Default::default()
        };
        unsafe {
            self.device()
                .begin_command_buffer(command_buffer, &begin_info)
                .context("failed to command buffer")?;
        }
        let mut storage = DrawCommandStorage::default();
        storage.reinit(
            command_buffer,
            info.color_formats,
            info.depth_format, info.stencil_format, info.sample_count,
            &alloc
        )?;
        {
            let mut commands = DrawCommands::new(
                self.clone(),
                &mut storage,
                &alloc,
                self.read_buffers(),
                self.read_images(),
            );
            f(&mut commands).context_from_tracked(|orig| format!(
                "failed to record draw commands {}", orig.or_this(),
            ))?;
        }
        unsafe {
            self.device()
                .end_command_buffer(command_buffer)
                .context("failed to end command buffer")?;
        }
        let resource = DrawCommandResource {
            queue: command_pool.queue().clone(),
            storage,
            alloc,
            _pool_handle: command_pool.handle().clone(),
        };
        let mut commands = self.inner.draw_commands.write();
        Ok(DrawCommandId(commands.insert(resource)))
    }

    /// Destroys previously [`created`][1] [`DrawCommands`] with `id`.
    ///
    /// [1]: Self::create_draw_commands
    #[inline]
    pub fn destroy_draw_commands(
        &self,
        id: DrawCommandId
    ) -> Result<CommandBuffer>
    {
        self.inner.draw_commands
            .write()
            .remove(id.0)
            .context_with(|| format!(
                "invalid draw commands id {id}"
            )).map(|d| d.storage.command_buffer)
    }

    #[inline]
    pub(crate) fn get_draw_commands(
        &self,
        id: DrawCommandId,
    ) -> Result<impl Deref<Target = DrawCommandResource>>
    {
        RwLockReadGuard::try_map(self.inner.draw_commands.read(), |d| {
            d.get(id.0).ok()
        }).map_err(|_| Error::just_context(format!(
            "invalid draw command id {id}"
        )))
    }
    
    /// Advances the [`Gpu`] frame.
    ///
    /// This records and submits scheduled commands and updates swapchains.
    pub fn tick<F>(
        &self,
        mut event_handler: F,
        cache: &mut Cache,
    ) -> Result<()>
        where F: FnMut(Event) -> EventResult<()>
    {
        cache.init(self).context("failed to init cache")?;
        let pools = self.inner.descriptor_pools.load();
        for pool in pools.values_mut() {
            pool.update(self);
        }
        unsafe {
            cache.arena.clear();
        }
        let submits = self.queue_scheduler().record(
            self, &mut cache.command_cache,
            &mut event_handler, &cache.arena,
        ).context("failed to record commands")?;
        for submit in &submits.submits {
            let submit_info = vk::SubmitInfo2 {
                wait_semaphore_info_count: submit.wait_semaphore_infos.len(),
                p_wait_semaphore_infos: submit.wait_semaphore_infos.as_ptr(),
                command_buffer_info_count: submit.command_buffer_infos.len(),
                p_command_buffer_infos: submit.command_buffer_infos.as_ptr(),
                signal_semaphore_info_count: submit.signal_semaphore_infos.len(),
                p_signal_semaphore_infos: submit.signal_semaphore_infos.as_ptr(),
                ..Default::default()
            };
            cache.submit_cache[submit.device_queue_index as usize]
                .push(submit_info);
        }
        for (i, submits) in cache.submit_cache.iter_mut().enumerate() {
            if submits.is_empty() {
                continue
            }
            let queue = &self.inner.device.device_queues()[i];
            unsafe {
                self.inner.device.queue_submit2(
                    queue.handle(),
                    submits,
                    vk::Fence::null(),
                )
            }.context_with(|| format!(
                "failed to submit to queue {queue:?}"
            ))?;
            submits.clear();
        }
        for present_submit in &submits.present_submits {
            let mut present_id2 = vk::PresentId2KHR {
                swapchain_count: present_submit.swapchains.len(),
                p_present_ids: present_submit.present_id2.as_ptr(),
                ..Default::default()
            };
            let present_info = vk::PresentInfoKHR {
                wait_semaphore_count: present_submit.wait_semaphores.len(),
                p_wait_semaphores: present_submit.wait_semaphores.as_ptr(),
                swapchain_count: present_submit.swapchains.len(),
                p_swapchains: present_submit.swapchains.as_ptr(),
                p_image_indices: present_submit.image_indices.as_ptr(),
                ..Default::default()
            }.push_next(&mut present_id2);
            unsafe {
                self.inner.device.queue_present(
                    present_submit.queue,
                    &present_info
                )
            }.context("queue present failed")?;
        }
        Ok(())
    } 

    #[inline]
    pub(crate) fn write_surfaces(&self) -> ResourceWriteGuard<'_, Surface, SurfaceId> {
        ResourceWriteGuard::new(self.inner.surfaces.write())
    }

    /// Creates a [`ShaderSet`].
    ///
    /// Shader sets are needed for [`creating pipelines`][1] and for [`allocating'][2] descriptor
    /// sets.
    ///
    /// [1]: Self::create_pipeline_batch
    /// [2]: Self::allocate_descriptor_sets
    pub fn create_shader_set<const N_SHADERS: usize>(
        &self,
        shaders: [Shader; N_SHADERS],
        attributes: ShaderSetAttributes,
    ) -> Result<ShaderSetId>
    {
        self.inner.shader_cache.write().create_shader_set(
            shaders,
            attributes,
            self.inner.thread_pool.clone(),
            self.inner.mt_ctx.clone(),
        )
    }

    /// Gets a future containing the result of a previous call to [`create_shader_set`][1].
    ///
    /// [1]: Self::create_shader_set
    #[inline]
    pub fn get_shader_set<'a>(
        &self,
        id: ShaderSetId
    ) -> impl Future<Output = Result<ShaderSet>> + Send + Sync + use<'a> 
    {
        self.inner.shader_cache
            .read()
            .get_shader_set(id)
    }
    
    /// Deletes a [`ShaderSet`] from the resource pool.
    #[inline]
    pub fn delete_shader_set(&self, id: ShaderSetId) { 
        self.inner.shader_cache.write().delete_shader_set(id)
    }

    /// Creates a descriptor pool, which can be used to [`allocate`][1] descriptor sets for use in
    /// shaders.
    ///
    /// [1]: Self::allocate_descriptor_sets
    pub fn create_descriptor_pool<I>(
        &self,
        pool_sizes: I,
        max_sets: u32,
        max_inline_uniform_block_bindings: u32,
    ) -> Result<DescriptorPoolId>
        where I: IntoIterator<Item = DescriptorPoolSize>
    {
        self.inner.descriptor_pools.modify(|pools| {
            Ok(DescriptorPoolId::new(pools.try_insert_with_index(|index| DescriptorPool::new(
                self.inner.device.clone(), DescriptorPoolId::new(index),
                pool_sizes, max_sets, max_inline_uniform_block_bindings,
            ).context("failed to create descriptor pool"))?))
        })
    }
    
    /// Delets a descriptor pool from the resource pool.
    #[inline]
    pub fn destroy_descriptor_pool(
        &self,
        id: DescriptorPoolId,
    ) {
        self.inner.descriptor_pools.modify(|pools| {
            pools.remove(id.slot_index()).ok();
        });
    }

    #[inline]
    pub(crate) fn get_descriptor_pools(
        &self
    ) -> impl Deref<Target = SlotMap<DescriptorPool>>
    {
        self.inner.descriptor_pools.load()
    }

    /// Allocates descriptor sets from a descriptor pool with `pool_id`.
    pub fn allocate_descriptor_sets<'a, I, >(
        &self,
        pool_id: DescriptorPoolId,
        set_infos: I,
    ) -> Result<()>
        where
            I: IntoIterator<Item = DescriptorSetInfo<'a>>,
            I::IntoIter: ExactSizeIterator

    {
        let pools = self.inner.descriptor_pools.load();
        let pool = pools
            .get(pool_id.slot_index())
            .context("failed find pool")
            .cloned();
        let tmp_alloc = self.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        pool?.allocate(set_infos, &tmp_alloc)
            .context("failed to allocate descriptor sets")
    }
    
    /// Frees descriptor sets from a descriptor pool with `pool_id`.
    ///
    /// # Valid usage
    /// - `pool_id` **must** be a valid [`DescriptorPoolId`].
    /// - Each id in `set_ids` **must** be a valid [`DescriptorSetId`] that was [`allocated`][1]
    ///   with the same [`DescriptorPoolId`] as `pool_id`.
    ///
    /// [1]: Self::allocate_descriptor_sets
    pub fn free_descriptor_sets(
        &self,
        pool_id: DescriptorPoolId,
        set_ids: &[DescriptorSetId],
    ) -> Result<()>
    {
        let queue_scheduler = self.queue_scheduler().read();
        let tmp_alloc = self.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let pools = self.inner.descriptor_pools.load();
        #[cfg(debug_assertions)]
        if let Some(id) = set_ids.iter().find(|id| id.pool_id() != pool_id) {
            return Err(Error::just_context(format!(
                "attempting to free descriptor sets {id} that was allocated from a different pool, expected pool {pool_id}",
            )))
        }
        let pool = pools
            .get(pool_id.slot_index())
            .context("failed to find pool")?;
        unsafe {
            pool.free(
                self,
                &queue_scheduler,
                set_ids,
                &tmp_alloc,
            )
        }
    }

    /// Updates descriptor sets [`allocated`][1] from the descriptor pool with `pool_id`.
    ///
    /// [1]: Self::allocate_descriptor_sets
    pub fn update_descriptor_sets<'a, W, C>(
        &self,
        pool_id: DescriptorPoolId,
        writes: W,
        copies: C,
    ) -> Result<()>
        where
            W: IntoIterator<Item = WriteDescriptorSet<'a>>,
            W::IntoIter: ExactSizeIterator,
            C: IntoIterator<Item = CopyDescriptorSet>,
            C::IntoIter: ExactSizeIterator,
    {
        let writes = writes.into_iter();
        let copies = copies.into_iter();
        let tmp_alloc = self.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let queue_scheduler = self.queue_scheduler().read();
        let pools = self.inner.descriptor_pools.load();
        let pool = pools
            .get(pool_id.slot_index())
            .context_with(|| format!(
                "invalid pool id {pool_id}"
            ))?;
        let mut pool = pool.write();
        let mut unpoison = FixedVec32::with_capacity(
            writes.len() as u32 + copies.len() as u32,
            &tmp_alloc,
        ).context("alloc error")?;
        let mut descriptor_infos = FixedVec32
            ::with_capacity(writes.len() as u32, &tmp_alloc)
            .context("alloc error")?;
        let mut vk_writes = FixedVec32
            ::with_capacity(writes.len() as u32, &tmp_alloc)
            .context("vec error")?; 
        let finished_frame = self.get_semaphore_counter_value(
            queue_scheduler.get_frame_semaphore_id()
        )?;
        for write in writes {
            if write.set_id.pool_id() != pool_id {
                return Err(Error::just_context(format!(
                    "buffer update descriptor set {} was allocated from a different pool, expected pool {pool_id}",
                    write.set_id,
                )))
            }
            let mut set = pool
                .get_descriptor_set_for_update(
                    write.set_id,
                    finished_frame,
                )
                .context_with(|| format!(
                    "failed to get descriptor set {}",
                    write.set_id,
                ))?;
            let (ty, infos) = set 
                .update(self, &write, &tmp_alloc)
                .context_with(|| format!(
                    "failed to update descriptor set {}",
                    write.set_id,
                ))?;
            descriptor_infos.push(infos);
            let last = unsafe {
                descriptor_infos.last_mut().unwrap_unchecked()
            };
            let mut write = vk::WriteDescriptorSet {
                s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
                dst_set: set.descriptor_set(),
                dst_binding: write.binding,
                dst_array_element: write.starting_index,
                descriptor_type: ty,
                ..Default::default()
            };
            match last {
                DescriptorUpdateInfos::Buffer(buffers) => {
                    write.descriptor_count = buffers.len();
                    write.p_buffer_info = buffers.as_ptr();
                },
                DescriptorUpdateInfos::Image(images) => {
                    write.descriptor_count = images.len();
                    write.p_image_info = images.as_ptr();
                },
                DescriptorUpdateInfos::InlineUniformBlock(info) => {
                    write.descriptor_count = info.data_size;
                    // Safe because FixedVec doesn't reallocate
                    write.p_next = info as *const _ as *const core::ffi::c_void;
                },
            }
            unpoison.push(set.into_inner());
            vk_writes.push(write);
        }
        let mut vk_copies = FixedVec32
            ::with_capacity(copies.len() as u32, &tmp_alloc)
            .context("vec error")?;
        for copy in copies {
            if copy.src_set_id.pool_id() != pool_id {
                return Err(Error::just_context(format!(
                    "descriptor set copy source {} was allocated from a different pool, expected pool {pool_id}",
                    copy.src_set_id,
                )))
            }
            if copy.dst_set_id.pool_id() != pool_id {
                return Err(Error::just_context(format!(
                    "descriptor set copy destination {} was allocated from a different pool, expected pool {pool_id}",
                    copy.dst_set_id,
                )))
            }
            let src = pool 
                .get_descriptor_set_handle(copy.src_set_id)
                .context_with(|| format!(
                    "failed to get source descriptor set {} for copy",
                    copy.src_set_id,
                ))?;
            let mut dst = pool
                .get_descriptor_set_for_update(
                    copy.dst_set_id,
                    finished_frame,
                ).context_with(|| format!(
                    "failed to get destination descriptor set {} for copy",
                    copy.dst_set_id,
                ))?;
            let vk_copy = unsafe { dst.copy_from(
                src,
                copy.src_binding,
                copy.src_starting_index,
                copy.dst_binding,
                copy.dst_starting_index,
                copy.array_count,
            ) }.context_with(|| format!(
                "failed to copy source descriptor set {} to destination descriptor set resource {}",
                copy.src_set_id, copy.dst_set_id,
            ))?; 
            unpoison.push(dst.into_inner());
            vk_copies.push(vk_copy);
        }
        unsafe {
            self.inner.device.update_descriptor_sets(&vk_writes, &vk_copies);
            for mut handle in unpoison {
                handle.unpoison();
            }
        }
        Ok(())
    } 

    pub fn write_sampler_descriptors_ext(
        &self,
    ) -> Result<()> { Ok(()) }

    pub fn write_resource_descriptors_ext(
        &self,
    ) -> Result<()> { Ok(()) }

    #[inline]
    pub(crate) fn reserve_pipeline_batch_slot(&self) -> PipelineBatchId {
        PipelineBatchId::new(self.inner.pipeline_batches.modify(|data| {
            data.insert(OnceLock::new())
        }))
    }

    #[inline]
    pub(crate) fn discard_pipeline_batch(&self, id: PipelineBatchId) {
        self.inner.pipeline_batches.modify(|data| {
            data.remove(id.slot_index())
        }).ok();
    }

    #[inline]
    pub(crate) fn init_pipeline_batch(
        &self,
        id: PipelineBatchId,
        batch: PipelineBatch,
    ) {
        if let Ok(b) = self.inner.pipeline_batches.load().get(id.slot_index()) {
            b.get_or_init(|| batch);
        }
    }

    /// Creates a new [`PipelineBatchBuilder`].
    ///
    /// # Valid usage
    /// - `cache` *must* either be [`None`] or a valid [`PipelineCache`] handle.
    /// - You should always call [`PipelineBatchBuilder::build`] when you are finished with the
    ///   batch and you should *not* rely on automatic builds.
    #[inline]
    pub fn create_pipeline_batch<Cache>(
        &self,
        cache: Cache,
    ) -> Result<PipelineBatchBuilder>
        where Cache: Into<Option<PipelineCache>>
    {
        let cache = cache.into();
        if let Some(cache) = &cache &&
            cache.device_id() != self.device().id()
        {
            return Err(Error::just_context(format!(
                "cache logical device id {} is different from this Gpu instance device id {}",
                cache.device_id(), self.device().id(),
            )))
        }
        Ok(PipelineBatchBuilder::new(
            self.clone(), cache
        ))
    }

    /// Destroys an entire pipeline batch.
    ///
    /// # Valid usage
    /// - `batch_id` *must* be a valid [`PipelineBatchId`].
    pub fn destroy_pipeline_batch(
        &self,
        batch_id: PipelineBatchId,
    ) -> Result<()> {
        self.inner.pipeline_batches.modify(|batches| {
            batches.remove(batch_id.slot_index())
            .context_with(|| format!(
                "invalid pipeline batch id {batch_id}"
            )).map(|_| ())
        })
    }

    /// Destroys pipelines from a given pipeline batch.
    ///
    /// If you want to destroy an entire pipeline batch, consider using
    /// [`Gpu::destroy_pipeline_batch`] for efficiency.
    ///
    /// # Valid usage
    /// - This *should* only be called from the main thread.
    /// - `batch_id` *must* be a valid [`PipelineBatchId`].
    /// - Each id in `graphics_pipeline_ids` *must* be a valid [`GraphicsPipelineId`] and *must*
    ///   have originated from the specified batch.
    /// - Each id in `compute_pipeline_ids` *must* be a valid [`ComputePipelineId`] and *must*
    ///   have originated from the specified batch.
    pub fn destroy_pipelines<I1, I2>(
        &self,
        batch_id: PipelineBatchId,
        graphics_pipeline_ids: I1,
        compute_pipeline_ids: I2,
    ) -> Result<()>
        where
            I1: IntoIterator<Item = GraphicsPipelineId>,
            I1::IntoIter: ExactSizeIterator,
            I2: IntoIterator<Item = ComputePipelineId>,
            I2::IntoIter: ExactSizeIterator,
    {
        self.inner.pipeline_batches.modify(|batches| {
            let batch = batches
                .get_mut(batch_id.slot_index())
                .context_with(|| format!(
                    "invalid pipeline batch id {batch_id}"
                ))?.get().unwrap();
            block_on(batch.destroy_graphics_pipelines(
                graphics_pipeline_ids.into_iter()
            ))?;
            block_on(batch.destroy_compute_pipelines(
                compute_pipeline_ids.into_iter()
            ))?;
            Ok(())
        })
    } 

    /// Gets a previously [`created`][1] [`PipelineBatch`].
    ///
    /// [1]: Self::create_pipeline_batch
    #[inline]
    pub fn get_pipeline_batch(
        &self,
        id: PipelineBatchId,
    ) -> Result<impl Deref<Target = PipelineBatch>>
    {
        self.inner.pipeline_batches
            .load()
           .try_map(|batches| {
                Ok(batches
                    .get(id.slot_index())
                    .context_with(|| format!(
                        "invalid pipeline batch id {id}"
                    ))?
                    .get().unwrap()
                )
            })
    }

    /// Gets a [`graphics pipeline`][1] handle with `id`.
    ///
    /// [1]: GraphicsPipeline
    pub async fn get_graphics_pipeline<'a>(
        &self,
        id: GraphicsPipelineId,
    ) -> Result<impl Deref<Target = GraphicsPipeline> + use<'a>>
    {
        self.inner.pipeline_batches
            .load()
            .try_map(|batches| {
                batches
                    .get(id.batch_id().slot_index())
                    .context_with(|| format!(
                        "invalid pipeline batch id {}", id.batch_id(),
                    ))
            })?.get().unwrap()
            .get_graphics_pipeline(id.pipeline_id()).await
            .context_with(|| format!(
                "invalid graphics pipeline id {id}"
            ))
    }

    /// Gets a [`compute pipeline`][1] handle with `id`.
    ///
    /// [1]: ComputePipeline
    pub async fn get_compute_pipeline<'a>(
        &self,
        id: ComputePipelineId,
    ) -> Result<impl Deref<Target = ComputePipeline> + use<'a>>
    {
        self.inner.pipeline_batches
            .load()
            .try_map(|batches| {
                batches
                    .get(id.batch_id().slot_index())
            }).context_with(|| format!(
                "invalid pipeline batch id {}", id.batch_id()
            ))?.get().unwrap()
            .get_compute_pipeline(id.pipeline_id()).await
            .context_with(|| format!(
                "invalid graphics pipeline id {id}"
            ))
    }

    /// Creates buffers and images in a batch.
    ///
    /// If one resource creation fails, no resources are returned.
    ///
    /// [`BufferId`]s and [`ImageId`]s are returned to their respective [`BufferCreateInfo`]s and
    /// [`ImageCreateInfo`]s.
    ///
    /// # Valid usage
    /// - The valid usage of buffer and image create infos are described in [`ImageCreateInfo`] and
    ///   [`BufferCreateInfo`] respectively.
    pub fn create_resources<'a, B, I>(
        &self,
        buffer_create_infos: B,
        image_create_infos: I,
    ) -> Result<()>
        where
            B: IntoIterator<Item = BufferCreateInfo<'a>>,
            B::IntoIter: ExactSizeIterator,
            I: IntoIterator<Item = ImageCreateInfo<'a>>,
            I::IntoIter: ExactSizeIterator,
    {
        let buffer_create_infos = buffer_create_infos.into_iter();
        let image_create_infos = image_create_infos.into_iter();
        let tmp_alloc = self.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let buffers = UnsafeCell::new(self.inner.buffers.write());
        let images = UnsafeCell::new(self.inner.images.write());
        let mut buffer_bind_infos = FixedVec32::with_capacity(
            buffer_create_infos.len() as u32, &tmp_alloc,
        ).context("alloc failed")?;
        let mut image_bind_infos = FixedVec32::with_capacity(
            image_create_infos.len() as u32, &tmp_alloc,
        ).context("alloc failed")?;
        let mut guard = RaiiHandle::new((
                FixedVec32::<BufferId, _>::with_capacity(
                    buffer_create_infos.len() as u32,
                    &tmp_alloc,
                ).context("alloc failed")?,
                FixedVec32::<ImageId, _>::with_capacity(
                    image_create_infos.len() as u32,
                    &tmp_alloc,
                ).context("alloc failed")?,
            ),
            |(bufs, imgs)| unsafe {
                for &id in &bufs {
                    (&mut *buffers.get()).remove(id.slot_index()).ok();
                }
                for &id in &imgs {
                    (&mut *images.get()).remove(id.slot_index()).ok();
                }
            }
        );
        for (i, create_info) in buffer_create_infos.enumerate() {
            let mut bind_info = Default::default();
            let buffer_meta = create_info.build(self.device().clone(), &mut bind_info)
                .context_with(|| format!("failed to create buffer at index {i}"))?;
            buffer_bind_infos.push(bind_info);
            let id = BufferId::new(
                unsafe { &mut *buffers.get() }.insert(buffer_meta)
            );
            *create_info.out = id;
            guard.0.push(id);
        }
        for (i, create_info) in image_create_infos.enumerate() {
            let mut bind_info = Default::default();
            let image_meta = create_info.build(self.device().clone(), &mut bind_info)
                .context_with(|| format!("failed to create image at index {i}"))?;
            image_bind_infos.push(bind_info);
            let id = ImageId::new(
                unsafe { &mut *images.get() }.insert(image_meta)
            );
            *create_info.out = id;
            guard.1.push(id);
        }
        unsafe {
            if !buffer_bind_infos.is_empty() {
                self.inner.device.bind_buffer_memory2(&buffer_bind_infos)
                    .context("failed to bind the memory of buffers")?;
            }
            if !image_bind_infos.is_empty() {
                self.inner.device.bind_image_memory2(&image_bind_infos)
                    .context("failed to bind the memory of images")?;
            }
            
        }
        guard.into_inner();
        Ok(())
    }

    pub(crate) fn write_buffers<Id>(&self) -> ResourceWriteGuard<'_, BufferMeta, Id>
        where Id: ResourceId<BufferMeta>
    {
        ResourceWriteGuard::new(self.inner.buffers.write())
    }

    pub(crate) fn write_images<Id>(&self) -> ResourceWriteGuard<'_, ImageMeta, Id>
        where Id: ResourceId<ImageMeta>
    {
        ResourceWriteGuard::new(self.inner.images.write())
    }

    pub fn destroy_resources<I, B>(
        &self,
        buffers: I,
        images: B,
    ) -> Result<()>
        where
            I: IntoIterator<Item = BufferId>,
            B: IntoIterator<Item = ImageId>,
    {
        let buffers = buffers.into_iter();
        let images = images.into_iter();
        let mut all_buffers = self.inner.buffers.write();
        let mut all_images = self.inner.images.write();
        let pools = self.inner.descriptor_pools.load();
        for id in buffers {
            all_buffers
                .remove(id.0)
                .context_with(|| format!(
                    "invalid buffer id {id}",
                ))?;
            for pool in pools.values() {
                pool.buffer_delete(id);
            }
        }
        for id in images {
            let image = all_images
                .remove(id.slot_index())
                .context_with(|| format!(
                    "invalid image id {id}"
                ))?;
            for pool in pools.values() {
                for id in image.view_index_iter(id) {
                    pool.image_view_delete(id);
                }
            }
        }
        Ok(())
    }

    /// Returns whether a [`BufferId`] is valid.
    #[inline]
    pub fn is_buffer_valid(&self, id: BufferId) -> bool {
        self.inner.buffers.read().contains(id.0)
    }

    #[inline]
    pub fn map_buffer(
        &self,
        id: BufferId
    ) -> Result<MemoryMap>
    {
        self.inner.buffers
            .write()
            .get_mut(id.0)
            .context_with(|| format!(
                "invalid buffer id {id}"
            ))?.memory_mut().map_memory()
            .context("failed to map memory")
    }

    pub fn flush_mapped_memory_ranges(
        &self,
        ranges: &[MappedBufferMemoryRange],
    ) -> Result<()>
    {
        let tmp_alloc = self.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let mut vk_ranges = FixedVec32::with_capacity(
            ranges.len() as u32,
            &tmp_alloc
        ).context("alloc failed")?;
        let buffers = self.read_buffers();
        let non_coherent_atom_size = self.device_limits().non_coherent_atom_size();
        for range in ranges {
            let buffer = buffers.get(range.buffer_id)?;
            let memory = buffer.memory();
            if !memory.is_mapped() {
                return Err(Error::just_context(format!(
                    "buffer {} memory is not mapped",
                    range.buffer_id,
                )))
            }
            let offset = memory.offset() + range.offset;
            if !offset.is_multiple_of(non_coherent_atom_size) {
                return Err(Error::just_context(format!(
                    "buffer {} range offset {offset} is not a multiple of non coherent atom size {}",
                    range.buffer_id, non_coherent_atom_size,
                )))
            }
            if !range.size.is_multiple_of(non_coherent_atom_size) &&
                offset + range.size != memory.memory_size()
            {
                return Err(Error::just_context(format!(
                    "buffer {} range size {} is not a multiple of non coherent atom size {}",
                    range.buffer_id, range.size, non_coherent_atom_size,
                )))
            }
            if range.offset + range.size > memory.size() {
                return Err(Error::just_context(format!(
                    "buffer {} range offset {} + size {} is greater than allocation size {}",
                    range.buffer_id, range.offset, range.size, memory.size(),
                )))
            }
            vk_ranges.push(vk::MappedMemoryRange {
                memory: vk::DeviceMemory::from_raw(memory.handle()),
                offset,
                size: range.size,
                ..Default::default()
            });
        } 
        unsafe {
            self.device().flush_mapped_memory_ranges(&vk_ranges)
        }.context("failed to flush mapped memory ranges").map(|_| ())
    }

    pub fn invalidate_mapped_memory_ranges(
        &self,
        ranges: &[MappedBufferMemoryRange],
    ) -> Result<()>
    {
        let tmp_alloc = self.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let mut vk_ranges = FixedVec32::with_capacity(
            ranges.len() as u32,
            &tmp_alloc
        ).context("alloc failed")?;
        let buffers = self.read_buffers();
        let non_coherent_atom_size = self.device_limits().non_coherent_atom_size();
        for range in ranges {
            let buffer = buffers.get(range.buffer_id)?;
            let memory = buffer.memory();
            if !memory.is_mapped() {
                return Err(Error::just_context(format!(
                    "buffer {} memory is not mapped",
                    range.buffer_id,
                )))
            }
            let offset = memory.offset() + range.offset;
            if !offset.is_multiple_of(non_coherent_atom_size) {
                return Err(Error::just_context(format!(
                    "buffer {} range offset {offset} is not a multiple of non coherent atom size {}",
                    range.buffer_id, non_coherent_atom_size,
                )))
            }
            if !range.size.is_multiple_of(non_coherent_atom_size) &&
                offset + range.size != memory.memory_size() {
                return Err(Error::just_context(format!(
                    "buffer {} range size {} is not a multiple of non coherent atom size {}",
                    range.buffer_id, range.size, non_coherent_atom_size,
                )))
            }
            if range.offset + range.size > memory.size() {
                return Err(Error::just_context(format!(
                    "buffer {} range offset {} + size {} is greater than allocation size {}",
                    range.buffer_id, range.offset, range.size, memory.size(),
                )))
            }
            vk_ranges.push(vk::MappedMemoryRange {
                memory: vk::DeviceMemory::from_raw(memory.handle()),
                offset,
                size: range.size,
                ..Default::default()
            });
        } 
        unsafe {
            self.device().invalidate_mapped_memory_ranges(&vk_ranges)
        }.context("failed to flush mapped memory ranges").map(|_| ())
    }

    /// Returns whether an [`ImageId`] is valid.
    #[inline]
    pub fn is_image_valid(&self, id: ImageId) -> bool {
        self.inner.images.read().contains(id.slot_index())
    }

    /// Creates an image view.
    pub fn create_image_view(
        &self,
        image_id: ImageId,
        range: ImageRange,
    ) -> Result<ImageViewId> {
        self.inner.images
            .write()
            .get_mut(image_id.slot_index())
            .context_with(|| format!(
                "invalid image id {image_id}"
            ))?
            .create_view(range)
            .map(|idx| ImageViewId::new(image_id, idx))
    }
    
    /// Returns whether an [`ImageViewId`] is valid.
    #[inline]
    pub fn is_image_view_valid(&self, id: ImageViewId) -> bool {
        if let Ok(img) = self.inner.images.read().get(id.image_id().slot_index()) {
            img.get_view(id).is_ok()
        } else {
            false
        }
    }

    #[inline]
    pub(crate) fn read_buffers<Id: ResourceId<BufferMeta>>(
        &self
    ) -> ResourceReadGuard<'_, BufferMeta, Id>
    {
        ResourceGuard::new(self.inner.buffers.read())
    } 

    #[inline]
    pub(crate) fn read_images<Id: ResourceId<ImageMeta>>(
        &self
    ) -> ResourceReadGuard<'_, ImageMeta, Id>
    {
        ResourceGuard::new(self.inner.images.read())
    } 

    /// Creates timeline semaphores from an iterator over their initial values.
    pub fn create_timeline_semaphores<'a, I>(
        &self,
        create_infos: I,
    ) -> Result<()>
        where
            I: IntoIterator<Item = SemaphoreCreateInfo<'a>>,
            I::IntoIter: ExactSizeIterator,
    {
        let create_infos = create_infos.into_iter();
        if create_infos.len() == 0 {
            return Ok(())
        }
        let tmp_alloc = self.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let mut indices = FixedVec32::with_capacity(
            create_infos.len() as u32,
            &tmp_alloc,
        ).context("alloc failed")?;
        let mut err = None;
        let mut semaphores = self.inner.timeline_semaphores.write();
        for SemaphoreCreateInfo { out_id, initial_value } in create_infos {
            if err.is_some() {
                break;
            }
            let mut type_info = vk::SemaphoreTypeCreateInfo {
                s_type: vk::StructureType::SEMAPHORE_TYPE_CREATE_INFO,
                semaphore_type: vk::SemaphoreType::TIMELINE,
                initial_value,
                ..Default::default()
            };
            let semaphore_info = vk::SemaphoreCreateInfo {
                s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
                ..Default::default()
            }.push_next(&mut type_info);
            match unsafe {
                self.inner.device
                    .create_semaphore(&semaphore_info, None)
            } {
                Ok(handle) => {
                    let index = semaphores.insert(handle.value);
                    indices.push(index);
                    *out_id = TimelineSemaphoreId(index);
                },
                Err(e) => { err = Some(e); }
            }
        }
        if let Some(err) = err {
            for index in indices {
                semaphores.remove(index).ok();
            }
            return Err(Error::new(err, "failed to create timeline semaphore"))
        }
        Ok(())
    }

    /// Gets the counter value of a timeline semaphore.
    #[inline]
    pub fn get_semaphore_counter_value(&self, id: TimelineSemaphoreId) -> Result<u64> {
        let &handle = self.inner.timeline_semaphores
            .read()
            .get(id.0)
            .context_with(|| format!("failed to find timeline semaphore {id}"))?;
        unsafe {
            self.inner.device
                .get_semaphore_counter_value(handle)
                .context("failed to get timeline semaphore value")
                .map(|res| res.value)
        }
    }

    /// Waits for previous semaphores until `timeout`.
    ///
    /// Returns Ok(true) on success, Ok(false) on timeout and Err(err) if there's another error.
    pub fn wait_for_semaphores<I>(
        &self,
        semaphores: I,
        timeout: Duration,
    ) -> Result<bool>
        where
            I: IntoIterator<Item = SemaphoreWaitInfo>,
            I::IntoIter: ExactSizeIterator,
    {
        let tmp_alloc = self.tmp_alloc();
        let tmp_alloc = tmp_alloc.guard();
        let semaphores = semaphores.into_iter();
        let n_semaphores = semaphores.len() as u32;
        let mut handles = FixedVec32
            ::with_capacity(n_semaphores, &tmp_alloc)
            .context("alloc failed")?;
        let mut values = FixedVec32
            ::with_capacity(n_semaphores, &tmp_alloc)
            .context("alloc failed")?;
        let read = self.inner.timeline_semaphores.read();
        for SemaphoreWaitInfo { id, value } in semaphores {
            let &semaphore = read
                .get(id.0)
                .context("failed to find timeline semaphore")?;
            handles.push(semaphore);
            values.push(value);
        }
        let wait_info = vk::SemaphoreWaitInfo {
            s_type: vk::StructureType::SEMAPHORE_WAIT_INFO,
            semaphore_count: n_semaphores,
            p_semaphores: handles.as_ptr(),
            p_values: values.as_ptr(),
            ..Default::default()
        };
        let res = unsafe {
            self.inner.device.wait_semaphores(
                &wait_info,
                timeout.as_nanos() as u64,
            )
        }.context("unexpected vulkan error")?;
        Ok(res.result == vk::Result::SUCCESS)
    }

    /// Destroys a batch of timeline semaphores.
    pub fn destroy_timeline_semaphores(&self, ids: &[TimelineSemaphoreId]) {
        let mut semaphores = self.inner.timeline_semaphores.write();
        for id in ids {
            if let Ok(handle) = semaphores.remove(id.0) {
                unsafe {
                    self.inner.device.destroy_semaphore(handle, None);
                }
            }
        }
    }

    #[inline]
    pub(crate) fn get_timeline_semaphore(&self, id: TimelineSemaphoreId) -> Result<vk::Semaphore> {
        self.inner.timeline_semaphores
            .read()
            .get(id.0).copied()
            .context("failed to find timeline semaphore")
    }
}

impl Drop for GpuInner {

    fn drop(&mut self) {
        unsafe {
            log::info!("cleaning up GPU");
            for &handle in self.timeline_semaphores.write().values() {
                self.device.destroy_semaphore(handle, None);
            }
        }
    }
}

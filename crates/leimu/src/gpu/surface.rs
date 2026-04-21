use std::sync::Arc;

use core::num::NonZeroU32;

use raw_window_handle::{
    HasDisplayHandle,
    HasWindowHandle,
};

use tuhka::{vk, window};
use leimu_mem::{
    vec::NonNullVec32,
    arena::Arena,
};

use crate::{
    error::*,
};

use super::{
    commands::scheduler::PresentSwapchain,
    prelude::*,
};

/// A trait for windows usable as Vulkan surfaces.
///
/// # Safety
/// Implementations of [`HasWindowHandle`] and [`HasDisplayHandle`] *must* be valid implementations.
pub unsafe trait VulkanWindow: HasWindowHandle + HasDisplayHandle + Send + Sync + 'static {

    fn inner_size(&self) -> (u32, u32);
}

#[derive(Clone, Copy)]
enum SwapchainState {
    Valid,
    OutOfDate {
        desired_image_count: NonZeroU32,
        size: (u32, u32),
    },
}

pub(crate) struct AcquireImageData {
    pub _image_view: ImageViewId,
    pub image_format: Format,
    pub extent: (u32, u32),
    pub recreated_image_count: Option<NonZeroU32>,
}

pub(crate) struct Surface {
    gpu: Gpu,
    window: Arc<dyn VulkanWindow>,
    handle: vk::SurfaceKHR,
    swapchain_state: SwapchainState,
    swapchain: Option<Swapchain>,
    image_view_ids: NonNullVec32<'static, ImageViewId>,
    wait_semaphores: NonNullVec32<'static, vk::Semaphore>,
    frame_index: u32,
    image_index: u32,
    alloc: Arena,
    present_queue: DeviceQueue,
    desired_image_count: u32,
}

impl ResourceMeta for Surface {

    const NAME: &str = "surface";
}

impl Surface {

    pub fn new(
        window: Arc<dyn VulkanWindow>,
        gpu: Gpu,
        desired_image_count: u32,
    ) -> Result<Self>
    {
        let alloc = Arena::new(1 << 10).context("alloc failed")?;
        let handle = unsafe {
            window::create_surface(
                gpu.device().instance(),
                window.display_handle()
                    .map_err(|err|
                        Error::just_context(format!("failed to get display handle: {err}"))
                    )?.as_raw(),
                window
                    .window_handle()
                    .map_err(|err|
                        Error::just_context(format!("failed to get window handle: {err}"))
                    )?.as_raw(),
                None
            ).context("failed to create vulkan surface")?
        };
        let present_queue = gpu
            .device()
            .get_present_queue(handle)
            .context_with(|| 
                "failed to find a suitable queue for presentation"
            )?;
        Ok(Self {
            gpu,
            handle,
            swapchain: None,
            swapchain_state: SwapchainState::OutOfDate {
                desired_image_count: NonZeroU32::new(desired_image_count).unwrap(),
                size: window.inner_size(),
            },
            desired_image_count,
            window,
            present_queue,
            image_view_ids: Default::default(),
            wait_semaphores: Default::default(),
            frame_index: 0,
            image_index: 0,
            alloc,
        })
    }
    
    #[inline(always)]
    pub fn request_swapchain_update(
        &mut self,
        desired_image_count: u32,
        size: (u32, u32),
    ) {
        self.desired_image_count = desired_image_count;
        self.swapchain_state = SwapchainState::OutOfDate {
            desired_image_count: NonZeroU32::new(desired_image_count).unwrap(),
            size,
        };
    }

    #[inline(always)]
    pub fn current_image_view(&self) -> SwapchainImageViewId<'_> {
        let id = self.image_view_ids[self.image_index as usize];
        SwapchainImageViewId::new(
            SwapchainImageId::new(id.image_id().slot_index()),
            id.view_id(),
        )
    }

    pub fn acquire_next_image(
        &mut self,
        recorder: CommandRecorder<'_, '_>,
    ) -> Result<AcquireImageData> {
        let mut recreated = None;
        let swapchain = match self.swapchain_state {
            SwapchainState::Valid => unsafe { self.swapchain.as_mut().unwrap_unchecked() },
            SwapchainState::OutOfDate { desired_image_count, size, } => {
                if let Some(mut swapchain) = self.swapchain.take() {
                    swapchain.destroy(
                        self.gpu.device(),
                        self.present_queue.handle(),
                    );
                }
                recorder.destroy_swapchain_images(&self.image_view_ids);
                unsafe {
                    self.alloc.clear();
                }
                let tmp_alloc = recorder.gpu().tmp_alloc();
                let tmp_alloc = tmp_alloc.guard();
                let swapchain = self.swapchain.insert(Swapchain::new(
                    self.gpu.device(),
                    self.handle,
                    vk::Extent2D { width: size.0, height: size.1 },
                    desired_image_count.get(),
                    &self.alloc,
                    &tmp_alloc,
                ).context("failed to create swapchain")?);
                self.swapchain_state = SwapchainState::Valid;
                let images = swapchain.images();
                let n_images = images.handles.len() as u32;
                self.image_view_ids = NonNullVec32::with_capacity(
                    n_images,
                    &self.alloc
                ).context("alloc failed")?.into_static();
                self.image_view_ids.resize(n_images, Default::default());
                unsafe {
                    recorder.create_swapchain_images(
                        &images,
                        &mut self.image_view_ids,
                        &self.alloc
                    ).context("failed to create swapchain images")?;
                }
                self.wait_semaphores = NonNullVec32::with_capacity(
                    n_images, &self.alloc
                ).context("alloc failed")?.into_static();
                self.wait_semaphores.try_resize_with(n_images, || unsafe {
                    self.gpu.device().create_semaphore(
                        &Default::default(), None
                    ).context("failed to create semaphore")
                    .map(|res| res.value)
                })?;
                recreated = NonZeroU32::new(n_images);
                swapchain
            },
        };
        unsafe {
            let Some(data) = swapchain.acquire_next_image(
                self.gpu.device(), self.frame_index,
                ).context("failed to acquire next swapchain image")? else
            {
                self.request_swapchain_update(
                    self.desired_image_count,
                    self.window.inner_size(),
                );
                return self.acquire_next_image(recorder)
            };
            if data.suboptimal {
                self.request_swapchain_update(
                    self.desired_image_count,
                    self.window.inner_size()
                );
            }
            self.frame_index = (self.frame_index + 1) % self.image_view_ids.len();
            self.image_index = data.image_index;
            Ok(AcquireImageData {
                _image_view: self.image_view_ids[data.image_index as usize],
                image_format: data.image_format,
                extent: data.extent,
                recreated_image_count: recreated,
            })
        }
    }

    pub fn get_present_submit(
        &mut self,
        mut recorder: CommandRecorder<'_, '_>,
        command_buffer: vk::CommandBuffer,
    ) -> Result<()> {
        let Some(swapchain) = &mut self.swapchain else {
            return Ok(())
        };
        let view = self.image_view_ids[self.image_index as usize];
        let cache = unsafe {
            &mut *recorder.cache().get()
        };
        recorder.write_resources(|guard| {
            let image = guard
                .register_image(
                    view.image_id().slot_index(),
                    COMMAND_INDEX_IGNORED,
                ).context("failed to get swapchain image")?;
            let range = image.view_memory_barrier(
                ImageSubresourceState {
                    stage_mask: vk::PipelineStageFlags2::NONE,
                    access_mask: vk::AccessFlags2::NONE,
                    layout: vk::ImageLayout::PRESENT_SRC_KHR,
                    queue_family_index: self.present_queue.family_index(),
                },
                view,
                true,
                &mut cache.shader_resource_cache.image_memory_barrier_cache,
            ).context("swapchain image memory barrier failed")?;
            if !range.is_empty() {
                let tmp_alloc = self.gpu.tmp_alloc();
                let tmp_alloc = tmp_alloc.guard();
                let mem_barriers = cache.shader_resource_cache.image_memory_barrier_cache.flush(
                    &[range], &tmp_alloc
                )?;
                let dependency_info = vk::DependencyInfo {
                    image_memory_barrier_count: mem_barriers.len(),
                    p_image_memory_barriers: mem_barriers.as_ptr(),
                    ..Default::default()
                };
                unsafe {
                    self.gpu.device().cmd_pipeline_barrier2(
                        command_buffer, &dependency_info
                    );
                }
            }
            unsafe {
                cache.present_submits.add_swapchain(
                    self.present_queue.clone(),
                    PresentSwapchain {
                        swapchain: swapchain.handle(),
                        image_index: self.image_index,
                        wait_semaphore: self.wait_semaphores[self.image_index as usize],
                        present_id2: swapchain.present_id2(),
                    }
                );
            }
            Ok(())
        }) 
    }
}

impl Drop for Surface {

    fn drop(&mut self) {
        if let Some(mut swapchain) = self.swapchain.take() {
            swapchain.destroy(
                self.gpu.device(),
                self.present_queue.handle(),
            );
        }
        self.gpu.destroy_resources([], self
            .image_view_ids
            .iter().map(|id| id.image_id())
        ).unwrap();
        unsafe {
            self.gpu.device().instance()
                .destroy_surface(self.handle, None);
        }
    }
}

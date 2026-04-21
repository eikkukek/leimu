use core::{
    ptr::NonNull,
    num::NonZeroU64,
};

use leimu_mem::{
    vec::{FixedVec32, NonNullVec32},
    alloc::LocalAlloc,
    arena::{self, Arena}
};
use tuhka::vk;

use crate::{
    error::*,
    gpu::prelude::*,
};

#[derive(Clone, Copy)]
pub struct SwapchainFrameData {
    pub image_index: u32,
    pub suboptimal: bool,
    pub image_format: Format,
    pub extent: (u32, u32),
}

pub struct Swapchain {
    image_count: u32,
    images: NonNull<vk::Image>,
    fences: NonNull<vk::Fence>,
    handle: vk::SwapchainKHR,
    image_index: u32,
    surface_format: vk::SurfaceFormatKHR,
    image_extent: vk::Extent2D,
    image_usage: vk::ImageUsageFlags,
    present_id: Option<NonZeroU64>,
}

pub struct SwapchainImages<'a> {
    pub handles: &'a [vk::Image],
    pub format: vk::Format,
    pub extent: vk::Extent2D,
    pub usage: vk::ImageUsageFlags,
}

unsafe impl Send for Swapchain {}
unsafe impl Sync for Swapchain {}

impl Swapchain { 

    pub fn new(
        device: &Device,
        surface: vk::SurfaceKHR,
        framebuffer_extent: vk::Extent2D,
        desired_image_count: u32,
        alloc: &Arena,
        tmp_alloc: &impl LocalAlloc<Error = arena::Error>,
    ) -> Result<Self>
    {
        if framebuffer_extent.width == 0 || framebuffer_extent.height == 0 {
            return Err(Error::just_context("frame buffer size was 0"))
        }
        let surface_format = find_surface_format(
            device, surface, tmp_alloc,
        )?;
        let present_mode = find_present_mode(
            device, surface, tmp_alloc,
        )?;
        let mut id2 = vk::SurfaceCapabilitiesPresentId2KHR::default();
        let mut wait2 = vk::SurfaceCapabilitiesPresentWait2KHR::default();
        let surface_info = vk::PhysicalDeviceSurfaceInfo2KHR {
            s_type: vk::StructureType::PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
            surface,
            ..Default::default()
        };
        let mut capabilities = vk::SurfaceCapabilities2KHR
            ::default()
            .push_next(&mut id2)
            .push_next(&mut wait2);
        unsafe {
            device.instance()
                .get_physical_device_surface_capabilities2(
                    device.physical_device().handle(), &surface_info, &mut capabilities,
                ).context("failed to get physical device surface capabilities")?;
        };
        let mut image_extent = capabilities.surface_capabilities.current_extent;
        if image_extent.width == u32::MAX {
            image_extent.width = framebuffer_extent.width.clamp(
                capabilities.surface_capabilities.min_image_extent.width,
                capabilities.surface_capabilities.max_image_extent.width,
            );
            image_extent.height = framebuffer_extent.height.clamp(
                capabilities.surface_capabilities.min_image_extent.height,
                capabilities.surface_capabilities.max_image_extent.height,
            );
        }
        if image_extent.width == 0 || image_extent.height == 0 {
            return Err(Error::just_context("swapchain extent was 0"));
        }
        let mut actual_image_count = capabilities.surface_capabilities.min_image_count + 1;
        actual_image_count = actual_image_count.max(desired_image_count);
        if capabilities.surface_capabilities.max_image_count > 0 &&
            actual_image_count > capabilities.surface_capabilities.max_image_count
        {
            actual_image_count = capabilities.surface_capabilities.max_image_count;
        }
        let mut pre_transform = capabilities.surface_capabilities.current_transform;
        if capabilities.surface_capabilities.supported_transforms.contains(
            vk::SurfaceTransformFlagsKHR::IDENTITY_KHR
        ) {
            pre_transform = vk::SurfaceTransformFlagsKHR::IDENTITY_KHR;
        }
        let mut composite_alpha = vk::CompositeAlphaFlagsKHR::OPAQUE_KHR;
        if capabilities.surface_capabilities.supported_composite_alpha.contains(
            vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED_KHR
        )
        {
            composite_alpha = vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED_KHR;
        }
        let image_usage = vk::ImageUsageFlags::COLOR_ATTACHMENT;
        if !capabilities.surface_capabilities.supported_usage_flags.contains(image_usage)
        {
            return Err(Error::just_context(format!(
                "unsupported swapchain, missing image usage {}",
                super::ImageUsages::from_raw(image_usage.as_raw()),
            )))
        }
        let mut create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            surface,
            min_image_count: actual_image_count,
            image_format: surface_format.format,
            image_color_space: surface_format.color_space,
            image_extent,
            image_array_layers: 1,
            image_usage,
            image_sharing_mode: vk::SharingMode::EXCLUSIVE,
            pre_transform,
            composite_alpha,
            present_mode,
            ..Default::default()
        };
        let mut present_id = None;
        if id2.present_id2_supported != 0 && wait2.present_wait2_supported != 0 {
            present_id = Some(unsafe { NonZeroU64::new_unchecked(1) });
            create_info.flags |=
                vk::SwapchainCreateFlagsKHR::PRESENT_WAIT_2_KHR |
                vk::SwapchainCreateFlagsKHR::PRESENT_ID_2_KHR;
        }
        let handle = unsafe {
            match device.create_swapchain(&create_info, None) {
                Ok(swapchain) => swapchain,
                Err(err) => return Err(Error::new(err, "failed to create swapchain")),
            }
        }.value;
        let image_count = unsafe {
            device.get_swapchain_images_len(
                handle,
            ).context("failed to get Vulkan swapchain images")?
        }.value;
        let mut images = NonNullVec32
            ::with_capacity(
                image_count,
                alloc,
            ).context("alloc failed")?;
        images.resize(image_count, Default::default());
        unsafe {
            device.get_swapchain_images(
                handle,
                &mut images,
            ).context("failed to get Vulkan swapchain images")?;
        };
        let mut fences = NonNullVec32
            ::with_capacity(
                image_count,
                alloc
            ).context("alloc failed")?.into_static();
        fences.try_resize_with(image_count, || unsafe {
            device.create_fence(
                &Default::default(),
                None
            ).map(|res| res.value)
        }).context("failed to create fences")?;
        Ok(Self {
            image_count: images.len(),
            images: images.into_inner(),
            fences: fences.into_inner(),
            handle,
            image_index: 0,
            surface_format,
            image_extent,
            image_usage,
            present_id,
        })
    }

    pub fn destroy(
        &mut self,
        device: &Device,
        present_queue: vk::Queue,
    )
    {
        unsafe {
            if device.queue_wait_idle(present_queue).is_err() &&
                device.device_wait_idle().is_err()
            {
                panic!("GPU hang")
            }
        }
        for &fence in unsafe { core::slice::from_raw_parts(
            self.fences.as_ptr(),
            self.image_count as usize,
        ) } {
            unsafe {
                device.destroy_fence(fence, None);
            }
        }
        unsafe { device.destroy_swapchain(self.handle, None); }
    }


    #[inline(always)]
    pub fn handle(&self) -> vk::SwapchainKHR {
        self.handle
    }

    #[inline(always)]
    pub fn images(&self) -> SwapchainImages<'_> {
        SwapchainImages {
            handles: unsafe {
                core::slice::from_raw_parts(
                    self.images.as_ptr(),
                    self.image_count as usize,
                )
            },
            format: self.surface_format.format,
            extent: self.image_extent,
            usage: self.image_usage
        }
    }

    pub fn present_id2(&self) -> Option<NonZeroU64> {
        self.present_id
    }

    pub unsafe fn acquire_next_image(
        &mut self,
        device: &Device,
        frame_index: u32,
    ) -> Result<Option<SwapchainFrameData>>
    {
        let fence = unsafe {
            self.fences.add(frame_index as usize).read()
        }; 
        let (image_index, suboptimal) = unsafe { device
            .acquire_next_image(
                self.handle,
                device.frame_timeout(),
                vk::Semaphore::null(),
                fence,
            ).context("failed to acquire swapchain image")
            .map(|res| match res.result {
                vk::Result::SUCCESS => {
                    (Some(res.value), false)
                },
                vk::Result::SUBOPTIMAL_KHR => {
                    (Some(res.value), true)
                },
                _ => (None, false)
            })?
        };
        unsafe {
            if device.wait_for_fences(
                &[fence],
                true, device.frame_timeout()
                ).context("unexpected fence wait error")?.result == vk::Result::TIMEOUT
            {
                return Err(Error::just_context(
                    format!(
                        "frame timeout {} nanoseconds reached at {}", device.frame_timeout(),
                        location!(),
                    )
                ))
            }
            device.reset_fences(&[fence])
                .context("failed to reset fence")?;
        }
        let Some(image_index) = image_index else {
            return Err(Error::just_context(
                format!(
                    "frame timeout {} nanoseconds reached at {}", device.frame_timeout(),
                    location!(),
                )
            ))
        };
        if image_index >= self.image_count {
            return Err(Error::just_context(format!(
                "aquired Vulkan image index {} was out of bounds with image count {}",
                image_index, self.image_count,
            )))
        }
        self.image_index = image_index;
        if let Some(id) = &mut self.present_id {
            let next = id.get() + 1;
            *id = NonZeroU64::new(next).unwrap();
        }
        Ok(Some(
            SwapchainFrameData {
                image_index,
                suboptimal,
                image_format: unsafe { ::core::mem::transmute::<
                    vk::Format,
                    Format
                >(self.surface_format.format)},
                extent: (self.image_extent.width, self.image_extent.height),
            }
        ))
    }
}

fn find_surface_format<Alloc: LocalAlloc<Error = arena::Error>>(
    device: &Device,
    surface_handle: vk::SurfaceKHR,
    tmp_alloc: &Alloc,
) -> Result<vk::SurfaceFormatKHR>
{
    unsafe {
        let physical_device = device.physical_device().handle();
        let count = device.instance().get_physical_device_surface_formats_len(
            physical_device,
            surface_handle,
        ).context("failed to get Vulkan surface formats")?.value;
        let mut formats = FixedVec32
            ::with_len(count, Default::default(), &tmp_alloc)
            .context("alloc failed")?;
        device.instance().get_physical_device_surface_formats(
            physical_device,
            surface_handle,
            &mut formats,
        ).context("failed to get Vulkan surface formats")?;
        for &format in &formats {
            if format.format == vk::Format::R8G8B8A8_SRGB &&
                format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR_KHR
            {
                return Ok(format);
            }
        }
        Ok(formats[0])
    }
}

fn find_present_mode<Alloc: LocalAlloc<Error = arena::Error>>(
    device: &Device,
    surface: vk::SurfaceKHR,
    tmp_alloc: &Alloc,
) -> Result<vk::PresentModeKHR>
{
    unsafe {
        let physical_device = device.physical_device().handle();
        let count = device.instance().get_physical_device_surface_present_modes_len(
            physical_device,
            surface,
        ).context("failed to get Vulkan surface present modes")?.value;
        let mut modes = FixedVec32
            ::with_len(count, Default::default(), &tmp_alloc)
            .context("alloc failed")?;
        device.instance().get_physical_device_surface_present_modes(
            physical_device,
            surface,
            &mut modes,
        ).context("failed to get Vulkan surface present modes")?;
        for mode in &modes {
            if *mode == vk::PresentModeKHR::MAILBOX_KHR {
                return Ok(vk::PresentModeKHR::MAILBOX_KHR); // low latency and no tearing
            }
        }
        Ok(vk::PresentModeKHR::FIFO_KHR) // always supported
    }
}

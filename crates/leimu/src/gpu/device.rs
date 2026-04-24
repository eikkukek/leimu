use core::ops::Deref;

use leimu_mem::{
    vec::Vec32, vec32,
};

use tuhka::{vk, khr};

use super::prelude::*;

use crate::{
    error::*,
    sync::*,
};

/// Alias for chained [`tuhka::Device`].
///
/// The chain includes [`swapchain`][1] and [`present_wait2`][2].
///
/// [1]: khr::swapchain
/// [2]: khr::present_wait2
pub type VkDevice = tuhka::Device<khr::swapchain::Device<khr::present_wait2::Device>>;

struct Inner {
    id: DeviceId,
    physical_device: PhysicalDevice,
    device: VkDevice,
    device_queues: Box<[DeviceQueue]>,
    max_memory_allocation_size: DeviceSize,
    frame_timeout: u64,
    enabled_device_extensions: ext::EnabledDeviceExtensions,
    base_device_features: BaseDeviceFeatures,
    supported_depth_resolve_modes: vk::ResolveModeFlags,
    supported_stencil_resolve_modes: vk::ResolveModeFlags,
    instance: Instance,
    command_workers: u32,
}

/// Represents [`Vulkan device`][1].
///
/// The backbone of [`Gpu`].
///
/// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkDevice.html
#[derive(Clone)]
pub struct Device {
    inner: Arc<Inner>,
}

impl Deref for Device {

    type Target = VkDevice;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner.device
    }
}

impl Device {

    pub(crate) fn new(
        suitable: &SuitablePhysicalDevices,
        index: u32,
        queue_create_infos: &[DeviceQueueCreateInfo],
    ) -> Result<Device> {
        if queue_create_infos.is_empty() {
            return Err(Error::just_context(
                "at least one device queue needs to be specified"
            ))
        }
        let physical_device = &suitable.devices[index as usize];
        let enabled_device_extension_names: Vec32<_> = suitable
            .device_extension_infos
            .iter()
            .map(|ext| ext.name.as_ptr())
            .collect();
        let instance = suitable.instance.clone();
        let mut vulkan_12_features = None;
        let mut vulkan_14_features = None;
        let mut enabled_device_extensions = ext::EnabledDeviceExtensions::new();
        let mut physical_device_context = ext::PhysicalDeviceContext::new(
            &instance,
            physical_device,
            &mut vulkan_12_features,
            &mut vulkan_14_features,
            Some(&mut enabled_device_extensions),
        );
        let mut enable_features = Vec32::<ext::ExtendsDeviceCreateInfoObj>::with_capacity(
            suitable.device_extensions.len()
        );
        for ext in &suitable.device_extensions {
            if let Some(feature) = ext.register(&mut physical_device_context) {
                let s_type = feature.s_type();
                if enable_features.iter().any(|f| f.s_type() == s_type) {
                    return Err(Error::just_context(format!(
                        "device feature with structure type {:?} included twice when resolving device extensions",
                        feature.s_type(),
                    )))
                }
                enable_features.push(feature);
            }
        }
        let features = suitable.attributes.required_features.into();
        let mut priorities = vec32![];
        let device_queue_create_infos = physical_device.queue_families()
            .get_create_infos(queue_create_infos, &mut priorities)
            .context("failed to get device queue create infos")?;
        let mut device_create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DEVICE_CREATE_INFO,
            queue_create_info_count: device_queue_create_infos.len(),
            p_queue_create_infos: device_queue_create_infos.as_ptr(),
            enabled_extension_count: enabled_device_extension_names.len() as u32,
            pp_enabled_extension_names: enabled_device_extension_names.as_ptr(),
            p_enabled_features: &features,
            ..Default::default()
        };
        for enable_feature in &mut enable_features {
            device_create_info = device_create_info.push_next(enable_feature.as_mut());
        }
        let device: VkDevice = unsafe {
            instance
                .create_device(physical_device.handle(), &device_create_info, None)
                .context_with(|| {
                    "failed to create logical device"
                })?.value
        };
        let mut properties_11 = vk::PhysicalDeviceVulkan11Properties::default();
        physical_device_context.get_properties(&mut properties_11);
        let mut depth_stencil_resolve_properties = vk::PhysicalDeviceDepthStencilResolveProperties::default();
        physical_device_context.get_properties(&mut depth_stencil_resolve_properties);
        let max_memory_allocation_size = properties_11.max_memory_allocation_size;
        let id = DeviceId(super::DEVICE_ID.fetch_add(1, atomic::Ordering::AcqRel));
        let device_queues: Box<[_]> = queue_create_infos
            .iter()
            .enumerate()
            .map(|(i, info)| {
                let properties = physical_device.queue_families()
                    .properties()[info.family_index as usize];
                unsafe {
                    DeviceQueue::new(
                        id, &device,
                        i as u32,
                        info,
                        properties,
                    )
                }
            }).collect();
        for queue in &device_queues {
            log::info!("selected queue {queue} with queue family index {} and queue index {}",
                queue.family_index(), queue.queue_index(),
            );
        }
        Ok(Self {
            inner: Arc::new(Inner {
                id,
                physical_device: physical_device.clone(),
                device,
                device_queues,
                frame_timeout: suitable.attributes.frame_timeout.as_nanos() as u64,
                max_memory_allocation_size,
                enabled_device_extensions,
                base_device_features: suitable.attributes.required_features,
                supported_depth_resolve_modes: depth_stencil_resolve_properties.supported_depth_resolve_modes,
                supported_stencil_resolve_modes: depth_stencil_resolve_properties.supported_stencil_resolve_modes,
                instance: instance.clone(),
                command_workers: suitable.attributes.command_workers,
            })
        })
    }

    #[inline(always)]
    pub fn id(&self) -> DeviceId {
        self.inner.id
    }

    #[inline(always)]
    pub fn instance(&self) -> &Instance {
        &self.inner.instance
    }

    #[inline(always)]
    pub fn api_version(&self) -> Version {
        self.inner.physical_device.api_version()
    }

    #[inline(always)]
    pub fn physical_device(&self) -> &PhysicalDevice {
        &self.inner.physical_device
    }

    #[inline(always)]
    pub fn handle(&self) -> vk::Device {
        self.inner.device.handle()
    }

    #[inline(always)]
    pub fn base_device_features(&self) -> &BaseDeviceFeatures {
        &self.inner.base_device_features
    }

    #[inline(always)]
    pub fn get_device_attribute(&self, name: ext::ConstName) -> &ext::DeviceAttribute {
        self.inner.enabled_device_extensions.get_attribute(name)
    }

    #[inline(always)]
    pub fn get_extension_device<T: ext::ExtensionDevice>(&self) -> Option<T> {
        self.inner.enabled_device_extensions.get_device(self)
    }

    #[inline(always)]
    pub fn max_memory_allocation_size(&self) -> vk::DeviceSize {
        self.inner.max_memory_allocation_size
    }

    #[inline(always)]
    pub fn queue_families(&self) -> &QueueFamilies {
        self.inner.physical_device.queue_families()
    }

    #[inline(always)]
    pub fn device_queues(&self) -> &[DeviceQueue] {
        &self.inner.device_queues
    }

    #[inline(always)]
    pub fn any_device_queue(
        &self,
        flags: QueueFlags,
    ) -> Option<DeviceQueue> {
        self.inner.device_queues
            .iter()
            .find(|q| {
                q.queue_flags().contains(flags)
            }).cloned()
    }

    #[inline(always)]
    pub fn get_present_queue(&self, surface: vk::SurfaceKHR) -> Result<DeviceQueue> {
        for queue in &self.inner.device_queues {
            let index = queue.family_index();
            let supported = unsafe {
                self.inner.instance.get_physical_device_surface_support(
                    self.inner.physical_device.handle(),
                    index, surface
                ).context("failed to get physical device surface support")?
            }.value;
            if supported {
                return Ok(queue.clone())
            }
        }
        Err(Error::just_context("no queue with presentation support"))
    }

    #[inline(always)]
    pub fn frame_timeout(&self) -> u64 {
        self.inner.frame_timeout
    }

    #[inline(always)]
    pub fn supported_depth_resolve_modes(&self) -> vk::ResolveModeFlags {
        self.inner.supported_depth_resolve_modes
    }

    #[inline(always)]
    pub fn supported_stencil_resolve_modes(&self) -> vk::ResolveModeFlags {
        self.inner.supported_stencil_resolve_modes
    }

    #[inline(always)]
    pub fn command_workers(&self) -> u32 {
        self.inner.command_workers
    }
}

impl Drop for Inner {

    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_device(
                &self.device,
                None,
            );
        }
    }
}

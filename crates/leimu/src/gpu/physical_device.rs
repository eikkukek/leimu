use core::ffi::CStr;

use tuhka::vk;
use leimu_core::TryExtend;
use leimu_mem::{
    vec::Vec32,
    vec32,
};

use crate::{
    gpu::prelude::*,
    error::*,
    log, 
};

#[derive(Clone)]
pub struct PhysicalDevice {
    handle: vk::PhysicalDevice,
    api_version: Version,
    driver_version: Version,
    device_type: PhysicalDeviceType,
    limits: vk::PhysicalDeviceLimits,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    queue_families: QueueFamilies,
    device_name: DeviceName,
}

impl PhysicalDevice {

    fn new(
        instance: &Instance,
        handle: vk::PhysicalDevice,
    ) -> Self
    {
        let queue_families = QueueFamilies::new(handle, instance);
        let mut properties = vk::PhysicalDeviceProperties2::default();
        unsafe { instance.get_physical_device_properties2(handle, &mut properties) };
        let properties = properties.properties;
        let mut device_name = ([0u8; 256], 0);
        for ch in properties.device_name {
            if ch <= 0 || device_name.1 == 256 { break }
            device_name.0[device_name.1] = ch as u8;
            device_name.1 += 1;
        }
        let mut memory_properties = vk::PhysicalDeviceMemoryProperties2::default();
        unsafe { instance
            .get_physical_device_memory_properties2(handle, &mut memory_properties)
        };
        let memory_properties = memory_properties.memory_properties;
        Self {
            handle,
            api_version: Version::from_u32(properties.api_version),
            driver_version: Version::from_u32(properties.driver_version),
            device_type: properties.device_type.into(),
            limits: properties.limits,
            memory_properties,
            queue_families,
            device_name,
        }
    }

    #[inline(always)]
    pub fn handle(&self) -> vk::PhysicalDevice {
        self.handle
    }

    #[inline(always)]
    pub fn api_version(&self) -> Version {
        self.api_version
    }

    #[inline(always)]
    pub fn driver_version(&self) -> Version {
        self.driver_version
    }

    #[inline(always)]
    pub fn device_type(&self) -> PhysicalDeviceType {
        self.device_type
    }

    #[inline(always)]
    pub fn device_name(&self) -> &str {
        str::from_utf8(&self.device_name.0[0..self.device_name.1])
            .unwrap_or("<utf8-error")
    }

    #[inline(always)]
    pub fn queue_families(&self) -> &QueueFamilies {
        &self.queue_families
    }

    #[inline(always)]
    pub fn limits(&self) -> &vk::PhysicalDeviceLimits {
        &self.limits
    }

    #[inline(always)]
    pub fn memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties {
        &self.memory_properties
    }
}

#[derive(Error, Debug)]
pub(super) enum DeviceSuitability<'a> {
    Ok,
    #[display("of missing features: {0}")]
    MissingFeatures(ext::MissingDeviceFeatureError),
    #[display("of missing extensions: {0:?}")]
    MissingExtensions(Vec32<&'a CStr>),
    #[display("Leimu requires at least Vulkan version 1.1 (device version was {0})")]
    OldVersion(Version),
}

pub(super) fn is_device_suitable<'a>(
    instance: &Instance,
    attributes: &DeviceAttributes,
    physical_device: &PhysicalDevice,
    device_extension_infos: &[ext::DeviceExtensionInfo],
) -> Result<DeviceSuitability<'a>>
{
    let mut vulkan12_features = None;
    let mut vulkan14_features = None;
    let mut features = vk::PhysicalDeviceFeatures2::default();
    unsafe {
        instance.get_physical_device_features2(physical_device.handle(), &mut features)
    };
    let features = features.features;
    if let Some(err) = attributes.required_features.find_missing_features(&features) {
        return Ok(DeviceSuitability::MissingFeatures(err))
    }
    let context = ext::PhysicalDeviceContext::new(
        instance,
        physical_device,
        &mut vulkan12_features,
        &mut vulkan14_features,
        None,
    );
    for info in device_extension_infos {
        if let Some(precondition) = &info.precondition &&
            let Some(err) = precondition.call(&context)
        {
            return Ok(DeviceSuitability::MissingFeatures(err))
        }
    }
    let api_version = physical_device.api_version;
    if api_version < vk::API_VERSION_1_1 {
        return Ok(DeviceSuitability::OldVersion(api_version))
    }
    let mut check_ext = vec32![];
    for info in device_extension_infos {
        if info.deprecation_version > api_version {
            check_ext.push(info.name);
        }
    }
    let available_extensions = unsafe {
        let mut av = vec![Default::default(); instance
            .enumerate_device_extension_properties_len(physical_device.handle(), None)
            .context("failed to enumerate vulkan device extensions")?.value as usize
        ];
        instance.enumerate_device_extension_properties(
            physical_device.handle(),
            None,
            &mut av,
        ).context("failed to enumerate vulkan device extensions")?;
        av
    };

    let missing_extensions: Vec32<_> = check_ext.iter().filter_map(|&ext| {
        (!available_extensions.iter().any(|a| {
            a.extension_name_as_cstr().unwrap_or_default() == ext
        })).then_some(ext)
    }).collect();

    if !missing_extensions.is_empty() {
        return Ok(DeviceSuitability::MissingExtensions(missing_extensions))
    }

    Ok(DeviceSuitability::Ok)
}

pub(crate) fn find_suitable_physical_devices(
    instance: &Instance,
    attributes: &DeviceAttributes,
    device_extension_infos: &[ext::DeviceExtensionInfo],
) -> Result<Vec32<PhysicalDevice>>
{
    let physical_devices = unsafe {
        let mut res = vec![Default::default(); instance
            .enumerate_physical_devices_len()
            .context("failed to enumerate vulkan devices")?.value as usize
        ];
        instance.enumerate_physical_devices(&mut res)
            .context("failed to enumerate vulkan devices")?;
        res
    };
    let mut suitable = vec32![];
    suitable.try_extend(physical_devices
        .iter()
        .filter_map(|&physical_device| {
            let physical_device = PhysicalDevice::new(instance, physical_device);
            match is_device_suitable(
                instance,
                attributes,
                &physical_device,
                device_extension_infos,
            ) {
                Ok(suitability) => match suitability {
                    DeviceSuitability::Ok => Some(Ok(physical_device)),
                    reason => {
                        log::warn!("Physical device {} was unsuitable because {}",
                            physical_device.device_name(), reason,
                        );
                        None
                    }
                },
                Err(err) => Some(Err(err)),
            }
        })
    )?;
    if suitable.is_empty() {
        return Err(Error::just_context(
            "no suitable physical device found"
        ))
    }
    Ok(suitable)
}

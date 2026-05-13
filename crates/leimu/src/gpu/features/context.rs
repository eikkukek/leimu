use core::{
    any::TypeId,
    ffi::CStr,
};

use ahash::{AHashSet, AHashMap};

use tuhka::vk;

use leimu_mem::{vec32, vec::Vec32};

use crate::{
    gpu::prelude::*,
    error::*,
    default,
};

use crate::gpu::features::{
    Attribute,
    ExtendsDeviceCreateInfoObj, ExtendsDeviceCreateInfoExt
};

pub struct DeviceFeatureContext {
    instance: Instance,
    physical_device: vk::PhysicalDevice,
    api_version: Version,
    extension_properties: Vec32<vk::ExtensionProperties>,
    vulkan_10: vk::PhysicalDeviceFeatures,
    vulkan_11: vk::PhysicalDeviceVulkan11Features<'static>,
    vulkan_12: vk::PhysicalDeviceVulkan12Features<'static>,
    pub(crate) out_vulkan_10_features: vk::PhysicalDeviceFeatures,
    pub(crate) required_extensions: AHashSet<&'static CStr>,
    pub(crate) attributes: AHashSet<Attribute>,
    device_features: AHashMap<TypeId, ExtendsDeviceCreateInfoObj>,
}

impl DeviceFeatureContext {

    pub(crate) unsafe fn new(
        instance: Instance,
        physical_device: vk::PhysicalDevice
    ) -> Result<Self> {
        let mut properties = default();
        unsafe {
            instance.get_physical_device_properties2(physical_device, &mut properties);
        }
        let mut vulkan_11 = vk::PhysicalDeviceVulkan11Features::default();
        let mut vulkan_12 = vk::PhysicalDeviceVulkan12Features::default();
        let mut features = vk::PhysicalDeviceFeatures2::default()
            .push_next(&mut vulkan_11)
            .push_next(&mut vulkan_12);
        unsafe {
            instance.get_physical_device_features2(physical_device, &mut features);
        }
        let extension_properties = unsafe {
            let mut properties = vec32![default(); instance.enumerate_device_extension_properties_len(
                physical_device, None
            ).context("failed to enumerate physical device extension properties")?.value];
            instance.enumerate_device_extension_properties(physical_device, None, &mut properties)
                .context("failed to enumerate physical device extension properties")?;
            properties
        };
        Ok(Self {
            api_version: Version(properties.properties.api_version),
            physical_device,
            instance,
            extension_properties,
            vulkan_10: features.features,
            vulkan_11,
            vulkan_12,
            out_vulkan_10_features: default(),
            required_extensions: AHashSet::new(),
            attributes: AHashSet::new(),
            device_features: AHashMap::default(),
        })
    }

    #[inline]
    pub fn api_version(&self) -> Version {
        self.api_version
    }

    #[inline]
    pub fn vulkan_10_features(&self) -> &vk::PhysicalDeviceFeatures {
        &self.vulkan_10
    }

    #[inline]
    pub fn vulkan_11_features(&self) -> &vk::PhysicalDeviceVulkan11Features<'static> {
        &self.vulkan_11
    }

    #[inline]
    pub fn vulkan_12_features(&self) -> &vk::PhysicalDeviceVulkan12Features<'static> {
        &self.vulkan_12
    }

    #[inline]
    pub fn add_required_extension(&mut self, name: &'static CStr) -> Result<()>
    {
        if self.required_extensions.insert(name) &&
            !self.extension_properties
                .iter()
                .any(|prop| prop.extension_name_as_cstr().unwrap_or_default() == name)
        {
            return Err(Error::just_context(format!(
                "extension {name:?} not present"
            )))
        }
        Ok(())
    }

    #[inline]
    pub fn get_physical_device_features<E>(
        &self,
        out: &mut E 
    ) where E: vk::ExtendsPhysicalDeviceFeatures2 {
        let mut features = vk::PhysicalDeviceFeatures2
            ::default()
            .push_next(out);
        unsafe {
            self.instance.get_physical_device_features2(
                self.physical_device,
                &mut features
            );
        }
    }

    #[inline]
    pub fn get_physical_device_properties<E>(
        &self,
        out: &mut E,
    ) where E: vk::ExtendsPhysicalDeviceProperties2 {
        let mut properties = vk::PhysicalDeviceProperties2
            ::default()
            .push_next(out);
        unsafe {
            self.instance.get_physical_device_properties2(
                self.physical_device,
                &mut properties,
            );
        }
    }

    #[inline]
    pub fn add_device_attribute(&mut self, attribute: Attribute) {
        self.attributes.insert(attribute);
    }

    #[inline]
    pub fn set_features<T, F>(&mut self, f: F)
        where
            T: ExtendsDeviceCreateInfoExt + Default + Copy,
            F: FnOnce(T) -> T
    {
        let default = T::default();
        let features = self.device_features
            .entry(default.type_id())
            .or_insert_with(|| ExtendsDeviceCreateInfoObj::new(default))
            .as_structure()
            .unwrap();
        *features = f(*features);
    }

    #[inline]
    pub fn device_features_iter(&self) -> impl Iterator<Item = ExtendsDeviceCreateInfoObj>
    {
        self.device_features.values().cloned()
    }
}

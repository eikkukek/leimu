//! Provided by [`VK_KHR_index_type_uint8`][1] or Vulkan 1.4.
//!
//! [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_index_type_uint8.html

use super::*;

use tuhka::khr;


pub struct Attributes;

impl Attributes {
    /// Attribute type `bool`.
    pub const IS_ENABLED: ConstName = ConstName::new("index_type_uint8");
}

/// The extension type.
#[derive(Clone, Copy)]
pub struct DeviceExtensionIndexTypeUint8;

unsafe impl DeviceExtension for DeviceExtensionIndexTypeUint8 {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::index_type_uint8::NAME,
            deprecation_version: Version::VULKAN_API_VERSION_1_4,
            precondition: Precondition::new(|ctx| {
                let mut features = vk::PhysicalDeviceIndexTypeUint8Features::default();
                ctx.get_features(&mut features);
                (features.index_type_uint8 == 0).then(|| MissingDeviceFeatureError::new(
                    "index type uint8"
                ))
            }),
        })
    }

    fn register(
        &self,
        ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        ctx.register_attribute(DeviceAttribute::new_bool(Attributes::IS_ENABLED, true));
        Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDeviceIndexTypeUint8Features
            ::default()
            .index_type_uint8(true)
        ))
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

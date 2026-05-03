//! Provided by VK_KHR_push_descriptor or Vulkan 1.4.

use {
    tuhka::{
        vk,
        khr,
    },
    super::*,
};

/// Defines attribute names.
pub struct Attributes;

impl Attributes {
    /// Attribute type [`bool`][1].
    ///
    /// [1]: DeviceAttribute::bool
    pub const IS_ENABLED: ConstName = ConstName::new("push_descriptor");
    /// Attribute type [`u32`][1].
    ///
    /// [1]: DeviceAttribute::u32
    pub const MAX_PUSH_DESCRIPTORS: ConstName = ConstName::new("max_push_descriptors");
}

/// The extension type.
#[derive(Clone, Copy)]
pub struct Extension;

unsafe impl DeviceExtension for Extension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::push_descriptor::NAME,
            deprecation_version: API_VERSION_1_4,
            precondition: Precondition::new(|ctx| {
                if ctx.api_version() >= API_VERSION_1_4 {
                    let mut features = vk::PhysicalDeviceVulkan14Features::default();
                    ctx.get_features(&mut features);
                    (features.push_descriptor == 0).then(|| MissingDeviceFeatureError::new(
                        "push descriptor"
                    ))
                } else {
                    None
                }
            }),
        })
    }

    fn register(
        &self,
        ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        let mut properties = vk::PhysicalDevicePushDescriptorProperties::default();
        ctx.get_properties(&mut properties);
        ctx.register_attribute(DeviceAttribute::new_u32(
            Attributes::MAX_PUSH_DESCRIPTORS,
            properties.max_push_descriptors,
        ));
        ctx.register_attribute(DeviceAttribute::new_bool(
            Attributes::IS_ENABLED,
            true,
        ));
        if ctx.api_version() >= API_VERSION_1_4 {
            ctx.vulkan_14_features().push_descriptor = vk::TRUE;
        }
        None
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

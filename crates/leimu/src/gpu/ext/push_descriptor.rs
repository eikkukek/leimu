//! Provided by VK_KHR_push_descriptor or Vulkan 1.4.

use {
    tuhka::{
        vk,
        khr::push_descriptor,
    },
    super::{device::*, *},
};

/// Push descriptor properties.
#[derive(Clone, Copy)]
pub struct Properties {
    /// The maximum number of descriptors that **can** be used in a [`shader set`][1] created with
    /// the [`push descriptor`][2] flag [`set`][3].
    ///
    /// [1]: Gpu::create_shader_set
    /// [2]: DescriptorSetLayoutFlags::PUSH_DESCRIPTOR
    /// [3]: ShaderSetAttributes::with_descriptor_set_layout_flags
    pub max_push_descriptors: u32,
}

/// The name of the extension.
pub const NAME: ConstName = ConstName::from_c_str(push_descriptor::NAME);

/// [`Properties`] [`device attribute`][1] name.
///
/// [1]: Gpu::get_device_attribute
pub const PROPERTIES: AttributeName<Properties> = AttributeName::new("push_descriptor properties");

/// Creates the [`extension type`][1].
///
/// [1]: DeviceAttributes::with_device_extension
pub fn extension() -> impl DeviceExtension {
    Extension
}

#[derive(Clone, Copy)]
struct Extension;

unsafe impl DeviceExtension for Extension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: push_descriptor::NAME,
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
    ) -> RegisteredExtension {
        let mut properties = vk::PhysicalDevicePushDescriptorProperties::default();
        ctx.get_properties(&mut properties);
        ctx.register_attribute(Attribute::new(PROPERTIES, Properties {
            max_push_descriptors: properties.max_push_descriptors,
        }));
        if ctx.api_version() >= API_VERSION_1_4 {
            ctx.vulkan_14_features().push_descriptor = vk::TRUE;
        }
        registered_extension(NAME, None)
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

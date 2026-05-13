//! Provided by [`VK_KHR_index_type_uint8`][1] or Vulkan 1.4.
//!
//! Allows the use of [`IndexType::U8`].
//!
//! [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_index_type_uint8.html

use super::{*, device::*};

use tuhka::khr::index_type_uint8;

/// The name of the extension.
pub const NAME: ConstName = ConstName::new(index_type_uint8::NAME.to_str().unwrap());

/// Creates the [`extension type`][1].
///
/// [1]: DeviceAttributes::with_device_extension
#[inline]
pub const fn extension() -> impl DeviceExtension {
    Extension
}

#[derive(Clone, Copy)]
struct Extension;

unsafe impl DeviceExtension for Extension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: index_type_uint8::NAME,
            deprecation_version: API_VERSION_1_4,
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
    ) -> RegisteredExtension {
        registered_extension(
            NAME,
            Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDeviceIndexTypeUint8Features
                ::default()
                .index_type_uint8(true)
            ))
        )
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

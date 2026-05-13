//! Provided by [`VK_KHR_robustness2`][1].
//!
//! [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_robustness2.html

use super::{*, device::*};

use tuhka::khr::robustness2;

pub struct Attributes;

/// Specifies which features are enabled.
pub struct Features {
    pub robust_buffer_access2: bool,
    pub robust_image_access2: bool,
    pub null_descriptor: bool,
}

/// Specifies which features are supported.
pub struct SupportedFeatures {
    pub robust_buffer_access2: bool,
    pub robust_image_access2: bool,
}

/// The name of the extension.
pub const NAME: ConstName = ConstName::new(robustness2::NAME.to_str().unwrap());

/// Enabled [`Features`] [`device attribute`][1] name.
///
/// [1]: Gpu::get_device_attribute
pub const ENABLED_FEATURES: AttributeName<Features> = AttributeName::new("robustness2 enabled features");

/// [`SupportedFeatures`] [`device attribute`][1] name.
///
/// [1]: Gpu::get_device_attribute
pub const SUPPORTED_FEATURES: AttributeName<SupportedFeatures> = AttributeName::new("robustness2 supported features");

/// Creates the [`extension type`][1].
///
/// [1]: DeviceAttributes::with_device_extension
#[inline]
pub const fn extension(
    robust_buffer_access2: FeatureRequirements,
    robust_image_access2: FeatureRequirements,
    enable_null_descriptor: bool,
) -> impl DeviceExtension {
    Extension {
        robust_buffer_access2,
        robust_image_access2,
        enable_null_descriptor,
    }
}

#[derive(Clone, Copy)]
struct Extension {
    robust_buffer_access2: FeatureRequirements,
    robust_image_access2: FeatureRequirements,
    enable_null_descriptor: bool,
}

unsafe impl DeviceExtension for Extension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        let s = *self;
        Some(DeviceExtensionInfo {
            name: c"VK_KHR_robustness2",
            deprecation_version: VERSION_MAX,
            precondition: Precondition::new(move |context| {
                let mut features = vk::PhysicalDeviceRobustness2FeaturesEXT::default();
                context.get_features(&mut features);
                if s.robust_buffer_access2.is_required() && features.robust_buffer_access2 == 0 {
                    Some(MissingDeviceFeatureError::new("robust buffer access2"))
                } else if s.robust_image_access2.is_required() && features.robust_image_access2 == 0 {
                    Some(MissingDeviceFeatureError::new("robust image access2"))
                } else if s.enable_null_descriptor && features.null_descriptor == 0 {
                    Some(MissingDeviceFeatureError::new("null descriptor"))
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
        let mut supported = vk::PhysicalDeviceRobustness2FeaturesKHR::default();
        ctx.get_features(&mut supported);
        let supported = SupportedFeatures {
            robust_buffer_access2: supported.robust_buffer_access2 != 0,
            robust_image_access2: supported.robust_image_access2 != 0,
        };
        let mut enabled = Features {
            robust_buffer_access2: false,
            robust_image_access2: false,
            null_descriptor: false,
        };
        let mut features = vk::PhysicalDeviceRobustness2FeaturesEXT::default();
        if matches!(self.robust_buffer_access2, FeatureRequirements::Enable) {
            enabled.robust_buffer_access2 = true;
            features.robust_buffer_access2 = vk::TRUE;
        }
        if matches!(self.robust_image_access2, FeatureRequirements::Enable) {
            enabled.robust_image_access2 = true;
            features.robust_image_access2 = vk::TRUE;
        }
        if self.enable_null_descriptor {
            enabled.null_descriptor = true;
            features.null_descriptor = vk::TRUE;
        }
        ctx.register_attribute(Attribute::new(SUPPORTED_FEATURES, supported));
        ctx.register_attribute(Attribute::new(ENABLED_FEATURES, enabled));
        registered_extension(
            NAME,
            Some(ExtendsDeviceCreateInfoObj::new(features)),
        )
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

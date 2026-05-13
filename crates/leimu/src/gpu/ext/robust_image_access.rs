//! Provided by Vulkan 1.3 or VK_EXT_image_robustness.

use super::{*, device::*};

use tuhka::ext;

/// [`Device attribute`][1] name of an attribute which specifies the [`requirements`][2] of
/// robust image access.
///
/// [1]: DeviceAttributes::with_device_extension
/// [2]: FeatureRequirements
pub const SUPPORT: AttributeName<FeatureRequirements>
    = AttributeName::new("robust_image_access supported");

/// Creates the [`extension type`][1].
///
/// [1]: DeviceAttributes::with_device_extension
pub const fn extension(
    robust_image_access: FeatureRequirements,
) -> impl DeviceExtension
{
    Extension {
        robust_image_access
    }
}

#[derive(Clone, Copy)]
struct Extension {
    robust_image_access: FeatureRequirements,
}

unsafe impl DeviceExtension for Extension {

    fn get_info(&self, _attributes: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        let s = *self;
        Some(DeviceExtensionInfo {
            name: ext::image_robustness::NAME,
            deprecation_version: API_VERSION_1_3,
            precondition: Precondition::new(move |context| {
                if s.robust_image_access.is_required() {
                    let mut features = vk::PhysicalDeviceImageRobustnessFeatures::default();
                    context.get_features(&mut features);
                    (features.robust_image_access == 0).then(|| {
                        MissingDeviceFeatureError::new("robust image access")
                    })
                } else {
                    None
                }
            })
        })
    }

    fn register(
        &self,
        ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        match self.robust_image_access {
            FeatureRequirements::DontCare => None,
            FeatureRequirements::Require => {
                ctx.register_attribute(Attribute::new(SUPPORT, FeatureRequirements::Require));
                None
            },
            FeatureRequirements::Enable => {
                ctx.register_attribute(Attribute::new(SUPPORT, FeatureRequirements::Enable));
                Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDeviceImageRobustnessFeatures
                    ::default()
                    .robust_image_access(true)
                ))
            },
        }
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

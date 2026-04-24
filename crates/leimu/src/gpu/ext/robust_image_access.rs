//! Provided by Vulkan 1.3 or VK_EXT_image_robustness.

use super::*;

use tuhka::ext;

pub struct Attributes;

impl Attributes {

    /// Attribute type `bool`.
    pub const IS_SUPPORTED: ConstName
        = ConstName::new("robust_image_access supported");

    /// Attribute type `bool`.
    pub const IS_ENABLED: ConstName
        = ConstName::new("robust_image_access enabled");
}

/// The extension type.
#[derive(Clone, Copy)]
pub struct Extension {
    pub robust_image_access: RobustAccessRequirements,
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
        context: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        let mut features = vk::PhysicalDeviceImageRobustnessFeatures::default();
        context.get_features(&mut features);
        if features.robust_image_access != 0 {
            context.register_attribute(DeviceAttribute::new_bool(
                Attributes::IS_SUPPORTED, true
            ));
        }
        matches!(self.robust_image_access, RobustAccessRequirements::Enabled).then(|| {
            context.register_attribute(DeviceAttribute::new_bool(
                Attributes::IS_ENABLED, true
            ));
            ExtendsDeviceCreateInfoObj::new(vk::PhysicalDeviceImageRobustnessFeatures
                ::default()
                .robust_image_access(true)
            )
        })
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

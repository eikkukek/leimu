use super::*;

use tuhka::{vk, khr, ext};

#[derive(Clone, Copy)]
struct TimelineSemaphoreExtension;

unsafe impl DeviceExtension for TimelineSemaphoreExtension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::timeline_semaphore::NAME,
            deprecation_version: Version::VULKAN_API_VERSION_1_2,
            precondition: Precondition::new(|ctx| {
                let mut features = vk::PhysicalDeviceTimelineSemaphoreFeatures::default();
                ctx.get_features(&mut features);
                (features.timeline_semaphore == 0).then(|| {
                    MissingDeviceFeatureError::new("timeline semaphore")
                })
            }),
        })
    }

    fn register(
        &self,
        _ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDeviceTimelineSemaphoreFeatures
            ::default()
            .timeline_semaphore(true)
        ))
    }

    fn boxed(&self)  -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
struct ShaderViewportIndexLayerExtension;

unsafe impl DeviceExtension for ShaderViewportIndexLayerExtension {

    fn get_info(&self, attributes: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        attributes.required_features.multi_viewport.then(|| {
            DeviceExtensionInfo {
                name: ext::shader_viewport_index_layer::NAME,
                deprecation_version: Version::VULKAN_API_VERSION_1_2,
                precondition: Precondition::new(|ctx| {
                    if ctx.api_version() >= Version::VULKAN_API_VERSION_1_2 {
                        let mut features = vk::PhysicalDeviceVulkan12Features::default();
                        ctx.get_features(&mut features);
                        (features.shader_output_viewport_index == 0).then(|| {
                            MissingDeviceFeatureError::new("shader output viewport index")
                        }).or_else(|| {
                            (features.shader_output_layer == 0).then(|| {
                                MissingDeviceFeatureError::new("shader output layer")
                            })
                        })
                    } else {
                        None
                    }
                })
            }
        })
    }

    fn register(
        &self,
        ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        if ctx.api_version() >= Version::VULKAN_API_VERSION_1_2 {
            let features = ctx.vulkan_12_features();
            features.shader_output_viewport_index = vk::TRUE;
            features.shader_output_layer = vk::TRUE;
        }
        None
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
struct CreateRenderPass2Extension;

unsafe impl DeviceExtension for CreateRenderPass2Extension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::create_renderpass2::NAME,
            deprecation_version: Version::VULKAN_API_VERSION_1_2,
            precondition: None,
        })
    }

    fn register(
        &self,
        _ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        None
    }

    fn boxed(&self)  -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
struct DepthStencilResolveExtension;

unsafe impl DeviceExtension for DepthStencilResolveExtension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::depth_stencil_resolve::NAME,
            deprecation_version: Version::VULKAN_API_VERSION_1_2,
            precondition: None,
        })
    }

    fn register(
        &self,
        _ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        None
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
struct DynamicRenderingExtension;

unsafe impl DeviceExtension for DynamicRenderingExtension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::dynamic_rendering::NAME,
            deprecation_version: Version::VULKAN_API_VERSION_1_3,
            precondition: Precondition::new(|ctx| {
                let mut features = vk::PhysicalDeviceDynamicRenderingFeatures::default();
                ctx.get_features(&mut features);
                (features.dynamic_rendering == 0).then(|| {
                    MissingDeviceFeatureError::new("dynamic rendering")
                })
            }),
        })
    }

    fn register(
        &self,
        _ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDeviceDynamicRenderingFeatures
            ::default()
            .dynamic_rendering(true)
        ))
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
struct FormatFeatureFlags2Extension;

unsafe impl DeviceExtension for FormatFeatureFlags2Extension {

    fn get_info(&self, _attributes: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::format_feature_flags2::NAME,
            deprecation_version: Version::VULKAN_API_VERSION_1_3,
            precondition: None,
        })
    }

    fn register(
        &self,
        _ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        None
    }
    
    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
struct ExtendedDynamicStateExtension;

unsafe impl DeviceExtension for ExtendedDynamicStateExtension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: ext::extended_dynamic_state::NAME,
            deprecation_version: Version::VULKAN_API_VERSION_1_3,
            precondition: Precondition::new(|ctx| {
                if ctx.api_version() < Version::VULKAN_API_VERSION_1_3 {
                    let mut features = vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT::default();
                    ctx.get_features(&mut features);
                    (features.extended_dynamic_state == 0).then(|| {
                        MissingDeviceFeatureError::new("extended dynamic state")
                    })
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
        (ctx.api_version() < Version::VULKAN_API_VERSION_1_3).then(||
            ExtendsDeviceCreateInfoObj::new(vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT
                ::default()
                .extended_dynamic_state(true)
            )
        )
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
struct CopyCommands2Extension;

unsafe impl DeviceExtension for CopyCommands2Extension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::copy_commands2::NAME,
            deprecation_version: Version::VULKAN_API_VERSION_1_3,
            precondition: None,
        })
    }

    fn register(
        &self,
        _ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        None
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
struct Synchronization2Extension;

unsafe impl DeviceExtension for Synchronization2Extension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::synchronization2::NAME,
            deprecation_version: Version::VULKAN_API_VERSION_1_3,
            precondition: Precondition::new(|ctx| {
                let mut features = vk::PhysicalDeviceSynchronization2Features::default();
                ctx.get_features(&mut features);
                (features.synchronization2 == 0).then(|| {
                    MissingDeviceFeatureError::new("synchronization2")
                })
            }),
        })
    }

    fn register(
        &self,
        _ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDeviceSynchronization2Features
            ::default().synchronization2(true)
        ))
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
struct Maintenance4Extension;

unsafe impl DeviceExtension for Maintenance4Extension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::maintenance4::NAME,
            deprecation_version: Version::VULKAN_API_VERSION_1_3,
            precondition: Precondition::new(|ctx| {
                let mut features = vk::PhysicalDeviceMaintenance4Features::default();
                ctx.get_features(&mut features);
                (features.maintenance4 == 0).then(|| {
                    MissingDeviceFeatureError::new("maintenance4")
                })
            }),
        })
    }

    fn register(
        &self,
        _ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDeviceMaintenance4Features
            ::default()
            .maintenance4(true)
        ))
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
struct DynamicRenderingLocalReadExtension;

unsafe impl DeviceExtension for DynamicRenderingLocalReadExtension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::dynamic_rendering_local_read::NAME,
            deprecation_version: Version::VULKAN_API_VERSION_1_4,
            precondition: Precondition::new(|ctx| {
                let mut features = vk::PhysicalDeviceDynamicRenderingLocalReadFeatures::default();
                ctx.get_features(&mut features);
                (features.dynamic_rendering_local_read == 0).then(|| {
                    MissingDeviceFeatureError::new("dynamic rendering local read")
                })
            }),
        })
    }

    fn register(
        &self,
        _ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDeviceDynamicRenderingLocalReadFeatures
            ::default().dynamic_rendering_local_read(true)
        ))
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
struct Maintenance5Extension;

unsafe impl DeviceExtension for Maintenance5Extension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::maintenance5::NAME,
            deprecation_version: Version::VULKAN_API_VERSION_1_4,
            precondition: Precondition::new(|ctx| {
                let mut features = vk::PhysicalDeviceMaintenance5Features::default();
                ctx.get_features(&mut features);
                (features.maintenance5 == 0).then(|| {
                    MissingDeviceFeatureError::new("maintenance5")
                })
            }),
        })
    }

    fn register(
        &self,
        _ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDeviceMaintenance5Features
            ::default()
            .maintenance5(true)
        ))
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
struct Maintenance6Extension;

unsafe impl DeviceExtension for Maintenance6Extension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::maintenance6::NAME,
            deprecation_version: Version::VULKAN_API_VERSION_1_4,
            precondition: Precondition::new(|ctx| {
                let mut features = vk::PhysicalDeviceMaintenance6Features::default();
                ctx.get_features(&mut features);
                (features.maintenance6 == 0).then(|| {
                    MissingDeviceFeatureError::new("maintenance6")
                })
            }),
        })
    }

    fn register(
        &self,
        _ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDeviceMaintenance6Features
            ::default()
            .maintenance6(true)
        ))
    }

    fn boxed(&self)  -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
struct SwapchainExtension;

unsafe impl DeviceExtension for SwapchainExtension {

    fn get_info(&self, _attributes: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::swapchain::NAME,
            deprecation_version: Version::MAX,
            precondition: None
        })
    }

    fn register(
        &self,
        _ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        None
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
struct PresentId2Extension;

unsafe impl DeviceExtension for PresentId2Extension {

    fn get_info(&self, _: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::present_id2::NAME,
            deprecation_version: Version::MAX,
            precondition: Precondition::new(|ctx| {
                let mut features = vk::PhysicalDevicePresentId2FeaturesKHR::default();
                ctx.get_features(&mut features);
                (features.present_id2 == 0).then(|| {
                    MissingDeviceFeatureError::new("present id2")
                })
            })
        })
    }

    fn register(
        &self,
        _ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDevicePresentId2FeaturesKHR
            ::default()
            .present_id2(true)
        ))
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy)]
struct PresentWait2Extension;

unsafe impl DeviceExtension for PresentWait2Extension {

    fn get_info(&self, _attributes: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: khr::present_wait2::NAME,
            deprecation_version: Version::MAX,
            precondition: Precondition::new(|ctx| {
                let mut features = vk::PhysicalDevicePresentWait2FeaturesKHR::default();
                ctx.get_features(&mut features);
                (features.present_wait2 == 0).then(|| {
                    MissingDeviceFeatureError::new("present wait2")
                })
            })
        })
    }

    fn register(
        &self,
        _ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDevicePresentWait2FeaturesKHR
            ::default()
            .present_wait2(true)
        ))
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

pub(crate) fn core_extensions() -> impl Iterator<Item = DeviceExtensionObj> {
    let extensions: [DeviceExtensionObj; 16] = [
        TimelineSemaphoreExtension.boxed().into(),
        ShaderViewportIndexLayerExtension.boxed().into(),
        DynamicRenderingExtension.boxed().into(),
        FormatFeatureFlags2Extension.boxed().into(),
        ExtendedDynamicStateExtension.boxed().into(),
        CopyCommands2Extension.boxed().into(),
        Synchronization2Extension.boxed().into(),
        Maintenance4Extension.boxed().into(),
        DynamicRenderingLocalReadExtension.boxed().into(),
        Maintenance5Extension.boxed().into(),
        Maintenance6Extension.boxed().into(),
        CreateRenderPass2Extension.boxed().into(),
        DepthStencilResolveExtension.boxed().into(),
        SwapchainExtension.boxed().into(),
        PresentId2Extension.boxed().into(),
        PresentWait2Extension.boxed().into(),
    ];
    extensions.into_iter()
}

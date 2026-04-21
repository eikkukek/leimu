//! Provided [`VK_EXT_pipeline_robustness`][1] or Vulkan 1.4.
//!
//! [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_robustness.html

use super::*;

use tuhka::ext;

pub struct Attributes;

impl Attributes {

    /// Attribute type `bool`.
    pub const IS_ENABLED: ConstName
        = ConstName::new("pipeline_robustness enabled");

    /// Attribute type `i32`, can be safely cast to [`vk::PipelineRobustnessBufferBehaviour`].
    pub const DEFAULT_ROBUSTNESS_STORAGE_BUFFERS: ConstName
        = ConstName::new("pipeline_robustness default_robustness_storage_buffers");

    /// Attribute type `i32`, can be safely cast to [`vk::PipelineRobustnessBufferBehaviour`].
    pub const DEFAULT_ROBUSTNESS_UNIFORM_BUFFERS: ConstName
        = ConstName::new("pipeline_robustness default_robustness_uniform_buffers");

    /// Attribute type `i32`, can be safely cast to [`vk::PipelineRobustnessBufferBehaviour`].
    pub const DEFAULT_ROBUSTNESS_VERTEX_INPUTS: ConstName
        = ConstName::new("pipeline_robustness default_robustness_vertex_inputs");

    /// Attribute type `i32`, can be safely cast to [`vk::PipelineRobustnessImageBehaviour`].
    pub const DEFAULT_ROBUSTNESS_IMAGE: ConstName
        = ConstName::new("pipeline_robustness default_robustness_image_attribute_name");
}

#[derive(Clone, Copy)]
pub struct Extension;

unsafe impl DeviceExtension for Extension {

    fn get_info(&self, _attributes: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: ext::pipeline_robustness::NAME,
            deprecation_version: Version::VULKAN_API_VERSION_1_4,
            precondition: Precondition::new(|ctx| {
                let mut features = vk::PhysicalDevicePipelineRobustnessFeatures::default();
                ctx.get_features(&mut features);
                (features.pipeline_robustness == 0).then(||
                    MissingDeviceFeatureError::new("pipeline robustness")
                )
            }),
        })
    }

    fn register(
        &self,
        ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        let mut properties = vk::PhysicalDevicePipelineRobustnessProperties::default();
        ctx.get_properties(&mut properties);
        ctx.register_attribute(DeviceAttribute::new_i32(
            Attributes::DEFAULT_ROBUSTNESS_STORAGE_BUFFERS,
            properties.default_robustness_storage_buffers.as_raw(),
        ));
        ctx.register_attribute(DeviceAttribute::new_i32(
            Attributes::DEFAULT_ROBUSTNESS_UNIFORM_BUFFERS,
            properties.default_robustness_uniform_buffers.as_raw(),
        ));
        ctx.register_attribute(DeviceAttribute::new_i32(
            Attributes::DEFAULT_ROBUSTNESS_VERTEX_INPUTS,
            properties.default_robustness_vertex_inputs.as_raw(),
        ));
        ctx.register_attribute(DeviceAttribute::new_i32(
            Attributes::DEFAULT_ROBUSTNESS_IMAGE,
            properties.default_robustness_images.as_raw(),
        ));
        ctx.register_attribute(DeviceAttribute::new_bool(
            Attributes::IS_ENABLED, true
        ));
        Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDevicePipelineRobustnessFeatures
            ::default().pipeline_robustness(true)
        ))
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

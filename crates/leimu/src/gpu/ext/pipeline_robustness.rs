//! Provided [`VK_EXT_pipeline_robustness`][1] or Vulkan 1.4.
//!
//! [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_robustness.html

use super::{*, device::*};

use tuhka::ext::pipeline_robustness;

pub struct Attributes;

/// Pipeline robustness properties
#[derive(Clone, Copy)]
pub struct Properties {
    /// Describes the behavior of out of bounds accesses made to storage buffers when no robustness
    /// features are enabled.
    pub default_robustness_storage_buffers: PipelineRobustnessBufferBehavior,
    /// Describes the behavior of out of bounds accesses made to uniform buffers when no robustness
    /// features are enabled.
    pub default_robustness_uniform_buffers: PipelineRobustnessBufferBehavior,
    /// Describes the behavior of out of bounds accesses made to vertex input attributes when no
    /// robustness features are enable.
    pub default_robustness_vertex_inputs: PipelineRobustnessBufferBehavior,
    /// Describes the behavior of out of bounds accesses made to images when no robustness features
    /// are enabled.
    pub default_robustness_image: PipelineRobustnessImageBehavior,
}

pub const NAME: ConstName = ConstName::new(pipeline_robustness::NAME.to_str().unwrap());

/// [`Properties`] [`device attribute`][1] name.
///
/// [1]: Gpu::get_device_attribute
pub const PROPERTIES: AttributeName<Properties> = AttributeName::new("pipeline_robustness properties");

/// Creates the [`extension type`][1].
///
/// [1]: DeviceAttributes::with_device_extension
pub const fn extension() -> impl DeviceExtension {
    Extension
}

#[derive(Clone, Copy)]
struct Extension;

unsafe impl DeviceExtension for Extension {

    fn get_info(&self, _attributes: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: pipeline_robustness::NAME,
            deprecation_version: API_VERSION_1_4,
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
    ) -> RegisteredExtension {
        let mut properties = vk::PhysicalDevicePipelineRobustnessProperties::default();
        ctx.get_properties(&mut properties);
        ctx.register_attribute(Attribute::new(
            PROPERTIES,
            Properties {
                default_robustness_storage_buffers: unsafe {
                    PipelineRobustnessBufferBehavior::from_raw(
                        properties.default_robustness_storage_buffers.as_raw()
                    )
                },
                default_robustness_uniform_buffers: unsafe {
                    PipelineRobustnessBufferBehavior::from_raw(
                        properties.default_robustness_uniform_buffers.as_raw()
                    )
                },
                default_robustness_vertex_inputs: unsafe {
                    PipelineRobustnessBufferBehavior::from_raw(
                        properties.default_robustness_vertex_inputs.as_raw()
                    )
                },
                default_robustness_image: unsafe {
                    PipelineRobustnessImageBehavior::from_raw(
                        properties.default_robustness_images.as_raw()
                    )
                },
            }
        ));
        registered_extension(
            NAME,
            Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDevicePipelineRobustnessFeatures
                ::default().pipeline_robustness(true)
            ))
        )
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

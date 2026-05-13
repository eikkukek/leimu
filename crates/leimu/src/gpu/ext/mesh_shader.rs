//! Provided by VK_EXT_mesh_shader.
//!
//! Allows the use of mesh shaders.

use tuhka::ext::mesh_shader;
use leimu_proc::BuildStructure;

use super::{*, device::*};

/// Mesh shader properties.
#[derive(Clone, Copy, Debug)]
pub struct Properties {
    pub max_task_work_group_total_count: u32,
    pub max_task_work_group_count: [u32; 3usize],
    pub max_task_work_group_invocations: u32,
    pub max_task_work_group_size: [u32; 3usize],
    pub max_task_payload_size: u32,
    pub max_task_shared_memory_size: u32,
    pub max_task_payload_and_shared_memory_size: u32,
    pub max_mesh_work_group_total_count: u32,
    pub max_mesh_work_group_count: [u32; 3usize],
    pub max_mesh_work_group_invocations: u32,
    pub max_mesh_work_group_size: [u32; 3usize],
    pub max_mesh_shared_memory_size: u32,
    pub max_mesh_payload_and_shared_memory_size: u32,
    pub max_mesh_output_memory_size: u32,
    pub max_mesh_payload_and_output_memory_size: u32,
    pub max_mesh_output_components: u32,
    pub max_mesh_output_vertices: u32,
    pub max_mesh_output_primitives: u32,
    pub max_mesh_output_layers: u32,
    pub max_mesh_multiview_view_count: u32,
    pub mesh_output_per_vertex_granularity: u32,
    pub mesh_output_per_primitive_granularity: u32,
    pub max_preferred_task_work_group_invocations: u32,
    pub max_preferred_mesh_work_group_invocations: u32,
    pub prefers_local_invocation_vertex_output: bool,
    pub prefers_local_invocation_primitive_output: bool,
    pub prefers_compact_vertex_output: bool,
    pub prefers_compact_primitive_output: bool,
}

/// Mesh shader features.
///
/// Specifies what features are enabled from VK_EXT_mesh_shader.
#[derive(Default, Clone, Copy, BuildStructure, Debug)]
pub struct Features {
    pub task_shader: bool,
    pub mesh_shader: bool,
    pub multiview_mesh_shader: bool,
    pub primitive_fragment_shading_rate_mesh_shader: bool,
    pub mesh_shader_queries: bool,
}

/// The name of the extension
pub const NAME: ConstName = ConstName::new(mesh_shader::NAME.to_str().unwrap());

/// [`Features`] [`device attribute`][1] name.
///
/// [1]: Gpu::get_device_attribute
pub const FEATURES: AttributeName<Features> = AttributeName::new("EXT_mesh_shader features");

/// [`Properties`] [`device attribute`][1] name.
///
/// [1]: Gpu::get_device_attribute
pub const PROPERTIES: AttributeName<Properties> = AttributeName::new("EXT_mesh_shader properties");

/// Creates the [`extension type`][1].
///
/// [1]: DeviceAttributes::with_device_extension
pub const fn extension(
    required_features: Features,
) -> impl DeviceExtension {
    Extension { required_features }
}

/// The extension type.
#[derive(Clone, Copy)]
struct Extension {
    required_features: Features,
}

unsafe impl DeviceExtension for Extension {
    
    fn get_info(&self, _attributes: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        let required = self.required_features;
        Some(DeviceExtensionInfo {
            name: mesh_shader::NAME,
            deprecation_version: VERSION_MAX,
            precondition: Precondition::new(move |ctx| {
                let mut features = vk::PhysicalDeviceMeshShaderFeaturesEXT::default();
                ctx.get_features(&mut features);
                if required.task_shader && features.task_shader == 0 {
                    Some(MissingDeviceFeatureError::new("task_shader"))
                } else if required.mesh_shader && features.mesh_shader == 0 {
                    Some(MissingDeviceFeatureError::new("mesh_shader"))
                } else if required.multiview_mesh_shader && features.multiview_mesh_shader == 0 {
                    Some(MissingDeviceFeatureError::new("multiview_mesh_shader"))
                } else if required.primitive_fragment_shading_rate_mesh_shader &&
                    features.primitive_fragment_shading_rate_mesh_shader == 0 {
                    Some(MissingDeviceFeatureError::new("primitive_fragment_shading_rate_mesh_shader"))
                } else if required.mesh_shader_queries && features.mesh_shader_queries == 0 {
                    Some(MissingDeviceFeatureError::new("mesh_shader_queries"))
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
        let mut properties = vk::PhysicalDeviceMeshShaderPropertiesEXT::default();
        ctx.get_properties(&mut properties);
        macro_rules! prop {
            ($prop:ident, $($name:ident $(!= $b:literal)?),+ $(,)?) => {Properties {$(
                $name: $prop.$name $(!= $b)?,
            )+}};
        }
        let properties = prop!(properties,
            max_task_work_group_total_count,
            max_task_work_group_count,
            max_task_work_group_invocations,
            max_task_work_group_size,
            max_task_payload_size,
            max_task_shared_memory_size,
            max_task_payload_and_shared_memory_size,
            max_mesh_work_group_total_count,
            max_mesh_work_group_count,
            max_mesh_work_group_invocations,
            max_mesh_work_group_size,
            max_mesh_shared_memory_size,
            max_mesh_payload_and_shared_memory_size,
            max_mesh_output_memory_size,
            max_mesh_payload_and_output_memory_size,
            max_mesh_output_components,
            max_mesh_output_vertices,
            max_mesh_output_primitives,
            max_mesh_output_layers,
            max_mesh_multiview_view_count,
            mesh_output_per_vertex_granularity,
            mesh_output_per_primitive_granularity,
            max_preferred_task_work_group_invocations,
            max_preferred_mesh_work_group_invocations,
            prefers_local_invocation_vertex_output != 0,
            prefers_local_invocation_primitive_output != 0,
            prefers_compact_vertex_output != 0,
            prefers_compact_primitive_output != 0,
        );
        ctx.register_attribute(Attribute::new(
            FEATURES, self.required_features
        ));
        ctx.register_attribute(Attribute::new(
            PROPERTIES, properties,
        ));
        registered_extension(
            NAME,
            Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDeviceMeshShaderFeaturesEXT::default()
                .task_shader(self.required_features.task_shader)
                .mesh_shader(self.required_features.mesh_shader)
                .multiview_mesh_shader(self.required_features.multiview_mesh_shader)
                .primitive_fragment_shading_rate_mesh_shader(
                    self.required_features.primitive_fragment_shading_rate_mesh_shader
                ).mesh_shader_queries(self.required_features.mesh_shader_queries)
            ))
        )
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

pub use mesh_shader::Device;

impl AnyExtensionDevice for Device {

    fn boxed(&self) -> Box<dyn AnyExtensionDevice> {
        Box::new(self.clone())
    }
}

impl ExtensionDevice for Device {

    const EXT_NAME: ConstName = NAME;

    fn new(device: &crate::gpu::Device) -> Box<Self> {
        Box::new(Device::new(device))
    }
}

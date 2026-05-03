//! Provided by VK_EXT_mesh_shader.

use tuhka::ext::mesh_shader;
use leimu_proc::BuildStructure;

use super::*;

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

/// Defines attribute names.
pub struct Attributes;

impl Attributes {
    /// Attribute type [`structure`][1].of type [`Features`].
    ///
    /// [1]: DeviceAttribute::structure
    pub const FEATURES: ConstName = ConstName::new("EXT_mesh_shader_features");
    /// Attribute type [`structure`][1] of type [`Properties`].
    ///
    /// [1]: DeviceAttribute::structure
    pub const PROPERTIES: ConstName = ConstName::new("EXT_mesh_shader_properties");
}

/// The extension type.
#[derive(Clone, Copy)]
pub struct Extension {
    pub required_features: Features,
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
    ) -> Option<ExtendsDeviceCreateInfoObj> {
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
        ctx.register_attribute(DeviceAttribute::new_structure(
            Attributes::FEATURES, self.required_features
        ));
        ctx.register_attribute(DeviceAttribute::new_structure(
            Attributes::PROPERTIES, properties,
        ));
        Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDeviceMeshShaderFeaturesEXT::default()
            .task_shader(self.required_features.task_shader)
            .mesh_shader(self.required_features.mesh_shader)
            .multiview_mesh_shader(self.required_features.multiview_mesh_shader)
            .primitive_fragment_shading_rate_mesh_shader(
                self.required_features.primitive_fragment_shading_rate_mesh_shader
            ).mesh_shader_queries(self.required_features.mesh_shader_queries)
        ))
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

    const NAME: ConstName = ConstName::new("EXT_mesh_shader");

    fn precondition<'a, F>(f: F) -> bool
        where F: Fn(&ConstName) -> Option<&'a DeviceAttribute>
    {
        f(&Attributes::FEATURES)
            .is_some_and(|attr| attr.structure::<Features>().is_some())
    }

    fn new(device: &crate::gpu::Device) -> Box<Self> {
        Box::new(Device::new(device))
    }
}

//! Provided by VK_KHR_descriptor_indexing or Vulkan 1.2.

use tuhka::{
    ext::descriptor_indexing,
    vk,
};
use leimu_proc::BuildStructure;

use super::*;

pub struct Attributes;

impl Attributes {
    /// Attribute type [`bool`][1].
    ///
    /// [1]: DeviceAttribute::bool
    pub const IS_ENABLED: ConstName = ConstName::new("dynamic_indexing");
    /// Attribute type [`structure`][1], where T = [`Features`].
    ///
    /// [1]: DeviceAttribute::structure
    pub const FEATURES: ConstName = ConstName::new("dynamic_indexing_features");
}

#[derive(Default, Clone, Copy, BuildStructure)]
pub struct Features {
    pub shader_input_attachment_array_dynamic_indexing: bool,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    pub shader_storage_texel_buffer_array_dynamic_indexing: bool,
    pub shader_uniform_buffer_array_non_uniform_indexing: bool,
    pub shader_sampled_image_array_non_uniform_indexing: bool,
    pub shader_storage_buffer_array_non_uniform_indexing: bool,
    pub shader_storage_image_array_non_uniform_indexing: bool,
    pub shader_input_attachment_array_non_uniform_indexing: bool,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    pub descriptor_binding_uniform_buffer_update_after_bind: bool,
    pub descriptor_binding_sampled_image_update_after_bind: bool,
    pub descriptor_binding_storage_image_update_after_bind: bool,
    pub descriptor_binding_storage_buffer_update_after_bind: bool,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    pub descriptor_binding_update_unused_while_pending: bool,
    pub descriptor_binding_partially_bound: bool,
    pub descriptor_binding_variable_descriptor_count: bool,
    pub runtime_descriptor_array: bool,
}

#[derive(Clone, Copy)]
pub struct Extension {
    pub required_features: Features,
}

unsafe impl DeviceExtension for Extension {

    fn get_info(&self, _attributes: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        let required = self.required_features;
        Some(DeviceExtensionInfo {
            name: descriptor_indexing::NAME,
            deprecation_version: API_VERSION_1_2,
            precondition: Precondition::new(move |ctx| {
                let mut features = vk::PhysicalDeviceDescriptorIndexingFeatures
                    ::default();
                ctx.get_features(&mut features);
                macro_rules! check {
                    ($(pub $field:ident: bool,)+) => {$(
                        if required.$field && features.$field == 0 {
                            return Some(MissingDeviceFeatureError::new(
                                stringify!($field)
                            ))
                        }
                    )+};
                }
                check! {
                    pub shader_input_attachment_array_dynamic_indexing: bool,
                    pub shader_uniform_texel_buffer_array_dynamic_indexing: bool,
                    pub shader_storage_texel_buffer_array_dynamic_indexing: bool,
                    pub shader_uniform_buffer_array_non_uniform_indexing: bool,
                    pub shader_sampled_image_array_non_uniform_indexing: bool,
                    pub shader_storage_buffer_array_non_uniform_indexing: bool,
                    pub shader_storage_image_array_non_uniform_indexing: bool,
                    pub shader_input_attachment_array_non_uniform_indexing: bool,
                    pub shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
                    pub shader_storage_texel_buffer_array_non_uniform_indexing: bool,
                    pub descriptor_binding_uniform_buffer_update_after_bind: bool,
                    pub descriptor_binding_sampled_image_update_after_bind: bool,
                    pub descriptor_binding_storage_image_update_after_bind: bool,
                    pub descriptor_binding_storage_buffer_update_after_bind: bool,
                    pub descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
                    pub descriptor_binding_storage_texel_buffer_update_after_bind: bool,
                    pub descriptor_binding_update_unused_while_pending: bool,
                    pub descriptor_binding_partially_bound: bool,
                    pub descriptor_binding_variable_descriptor_count: bool,
                    pub runtime_descriptor_array: bool,
                };
                None
            }),
        })
    }

    fn register(
        &self,
        ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        macro_rules! set {
            ($features:ident, $(pub $field:ident: bool,)+) => {$(
                $features = $features.$field(self.required_features.$field);
            )+};
        }
        ctx.register_attribute(DeviceAttribute::new_bool(
            Attributes::IS_ENABLED,
            true,
        ));
        ctx.register_attribute(DeviceAttribute::new_structure(
            Attributes::FEATURES,
            self.required_features
        ));
        let mut features = vk::PhysicalDeviceDescriptorIndexingFeatures::default();
        set! {
            features,
            pub shader_input_attachment_array_dynamic_indexing: bool,
            pub shader_uniform_texel_buffer_array_dynamic_indexing: bool,
            pub shader_storage_texel_buffer_array_dynamic_indexing: bool,
            pub shader_uniform_buffer_array_non_uniform_indexing: bool,
            pub shader_sampled_image_array_non_uniform_indexing: bool,
            pub shader_storage_buffer_array_non_uniform_indexing: bool,
            pub shader_storage_image_array_non_uniform_indexing: bool,
            pub shader_input_attachment_array_non_uniform_indexing: bool,
            pub shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
            pub shader_storage_texel_buffer_array_non_uniform_indexing: bool,
            pub descriptor_binding_uniform_buffer_update_after_bind: bool,
            pub descriptor_binding_sampled_image_update_after_bind: bool,
            pub descriptor_binding_storage_image_update_after_bind: bool,
            pub descriptor_binding_storage_buffer_update_after_bind: bool,
            pub descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
            pub descriptor_binding_storage_texel_buffer_update_after_bind: bool,
            pub descriptor_binding_update_unused_while_pending: bool,
            pub descriptor_binding_partially_bound: bool,
            pub descriptor_binding_variable_descriptor_count: bool,
            pub runtime_descriptor_array: bool,
        };
        Some(ExtendsDeviceCreateInfoObj::new(features))
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

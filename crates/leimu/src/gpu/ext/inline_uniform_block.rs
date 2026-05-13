//! Provided by VK_EXT_inline_uniform_block or Vulkan 1.3.

use {
    tuhka::{
        vk,
        ext::inline_uniform_block,
    },
    super::{*, device::*},
};

/// Specifies properties supported by the [`device`][1].
///
/// [1]: PhysicalDevice
#[derive(Clone, Copy)]
pub struct Properties {
    /// Specifies the maximum size in bytes of an [`inline uniform block`][1] binding.
    ///
    /// [1]: ShaderSetAttributes::with_inline_uniform_block
    pub max_inline_uniform_block_size: u32,
    /// Specifies the maximum number of inline uniform block bindings that can be accessible to a
    /// single shader stage in a [`shader set`][1].
    ///
    /// [1]: Gpu::create_shader_set
    pub max_per_stage_descriptor_inline_uniform_blocks: u32,
    /// Specifies the maximum number of inline uniform block bindings in a [`shader set`][1].
    ///
    /// [1]: Gpu::create_shader_set
    pub max_descriptor_set_inline_uniform_blocks: u32,
}

/// The name of the extension
pub const NAME: ConstName = ConstName::new(inline_uniform_block::NAME.to_str().unwrap());

/// [`Properties`] [`device attribute`][1] name.
pub const PROPERTIES: AttributeName<Properties> = AttributeName::new("inline_uniform_block");

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
            name: inline_uniform_block::NAME,
            deprecation_version: API_VERSION_1_3,
            precondition: Precondition::new(|ctx| {
                let mut features = vk::PhysicalDeviceInlineUniformBlockFeatures::default();
                ctx.get_features(&mut features);
                (features.inline_uniform_block == 0).then(||
                    MissingDeviceFeatureError::new("inline uniform block")
                )
            })
        })
    }

    fn register(
        &self,
        ctx: &mut PhysicalDeviceContext<'_>,
    ) -> RegisteredExtension {
        let mut properties = vk::PhysicalDeviceInlineUniformBlockProperties::default();
        ctx.get_properties(&mut properties);
        ctx.register_attribute(Attribute::new(
            PROPERTIES,
            Properties {
                max_inline_uniform_block_size:
                    properties.max_inline_uniform_block_size,
                max_per_stage_descriptor_inline_uniform_blocks:
                    properties.max_per_stage_descriptor_inline_uniform_blocks,
                max_descriptor_set_inline_uniform_blocks:
                    properties.max_descriptor_set_inline_uniform_blocks,
            }
        ));
        registered_extension(
            NAME,
            Some(ExtendsDeviceCreateInfoObj::new(
                vk::PhysicalDeviceInlineUniformBlockFeatures
                    ::default()
                    .inline_uniform_block(true)
            ))
        )
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

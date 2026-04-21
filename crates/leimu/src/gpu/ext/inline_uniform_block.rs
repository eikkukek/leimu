//! Provided by VK_EXT_inline_uniform_block or Vulkan 1.3.

use {
    tuhka::{
        vk,
        ext,
    },
    super::*,
};

/// Attribute names.
pub struct Attributes;

impl Attributes {
    /// Attribute type `bool`.
    pub const IS_ENABLED: ConstName = ConstName::new("inline_uniform_block");
    /// Attribute type `u32`.
    pub const MAX_INLINE_UNIFORM_BLOCK_SIZE: ConstName
        = ConstName::new("max_inline_uniform_block_size");
    /// Attribute type `u32`.
    pub const MAX_PER_STAGE_DESCRIPTOR_INLINE_UNIFORM_BLOCKS: ConstName
        = ConstName::new("max_per_stage_descriptor_inline_uniform_blocks");
    /// Attribute type `u32`.
    pub const MAX_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCKS: ConstName
        = ConstName::new("max_descriptor_set_inline_uniform_blocks");
}

/// The extension type.
#[derive(Clone, Copy)]
pub struct Extension;

unsafe impl DeviceExtension for Extension {

    fn get_info(&self, _attributes: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: ext::inline_uniform_block::NAME,
            deprecation_version: Version::VULKAN_API_VERSION_1_3,
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
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        let mut properties = vk::PhysicalDeviceInlineUniformBlockProperties::default();
        ctx.get_properties(&mut properties);
        ctx.register_attribute(DeviceAttribute::new_u32(
            Attributes::MAX_INLINE_UNIFORM_BLOCK_SIZE,
            properties.max_inline_uniform_block_size,
        ));
        ctx.register_attribute(DeviceAttribute::new_u32(
            Attributes::MAX_PER_STAGE_DESCRIPTOR_INLINE_UNIFORM_BLOCKS,
            properties.max_per_stage_descriptor_inline_uniform_blocks,
        ));
        ctx.register_attribute(DeviceAttribute::new_u32(
            Attributes::MAX_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCKS,
            properties.max_descriptor_set_inline_uniform_blocks,
        ));
        ctx.register_attribute(DeviceAttribute::new_bool(
            Attributes::IS_ENABLED, true,
        ));
        Some(ExtendsDeviceCreateInfoObj::new(
            vk::PhysicalDeviceInlineUniformBlockFeatures
                ::default()
                .inline_uniform_block(true)
        ))
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

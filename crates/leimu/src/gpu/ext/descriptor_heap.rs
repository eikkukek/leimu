//! Provided by VK_EXT_descriptor_heap.
//!
//! Due to missing driver support, this extension is not yet fully supported.

use tuhka::ext::descriptor_heap;
use super::*;

pub use descriptor_heap::Device;

#[derive(Clone, Copy, Debug)]
pub struct Properties {
    /// Specifies the required alignment of buffers used for [`binding`][1] sampler heaps.
    ///
    /// [1]: Device::cmd_bind_sampler_heap
    pub sampler_heap_alignment: DeviceSize,
    /// Specifies the required alignment of buffers used for [`binding`][1] resource heaps.
    ///
    /// [1]: Device::cmd_bind_resource_heap
    pub resource_heap_alignment: DeviceSize,
    /// Specifies the maximum size of the range used to [`bind`][1] sampler heaps.
    ///
    /// [1]: Device::cmd_bind_sampler_heap
    pub max_sampler_heap_size: DeviceSize,
    /// Specifies the maximum size of the range used to [`bind`][1] resource heaps.
    ///
    /// [1]: Device::cmd_bind_resource_heap
    pub max_resource_heap_size: DeviceSize,
    /// Specifies the minimum amount of data needed to be reserved within a bound sampler heap
    /// range when embedded samplers are not used.
    pub min_sampler_heap_reserved_range: DeviceSize,
    /// Specifies the minimum amount of data needed to be reserved within a bound sampler heap
    /// range when embedded samplers are used.
    pub min_sampler_heap_reserved_range_with_embedded: DeviceSize,
    /// Specifies the minimum amount of data needed to be reserved within a bound resource heap
    /// range.
    pub min_resource_heap_reserved_range: DeviceSize,
    /// Specifies the size of sampler descriptors written by [`write_sampler_descriptors`][1].
    ///
    /// [1]: Device::write_sampler_descriptors
    pub sampler_descriptor_size: DeviceSize,
    /// Specifies the maximum size of image and texel buffer descriptors written by
    /// [`write_resource_descriptors`][1].
    ///
    /// [1]: Device::write_resource_descriptors
    pub image_descriptor_size: DeviceSize,
    /// Specifies the maximum size of unformatted buffer descriptors or acceleration structures
    /// written by [`write_resource_descriptors`][1].
    ///
    /// [1]: Device::write_resource_descriptors
    pub buffer_descriptor_size: DeviceSize,
    /// Specifies the required alignment of sampler descriptors within a sampler heap.
    pub sampler_descriptor_alignment: DeviceSize,
    /// Specifies the required alignment of image descriptors within a resource heap.
    pub image_descriptor_alignment: DeviceSize,
    /// Specifies the required alignment of buffer descriptors within a buffer heap.
    pub buffer_descriptor_alignment: DeviceSize,
    /// Specifies the maximum total size of all push data.
    pub max_push_data_size: DeviceSize,
}

/// Defines attribute names.
pub struct Attributes;

impl Attributes {
    /// Attribute type [`bool`][1].
    ///
    /// [1]: DeviceAttribute::bool
    pub const IS_ENABLED: ConstName = ConstName::new("EXT_descriptor_heap_enabled");
    /// Attribute type [`structure`][1] of type [`Properties`].
    ///
    /// [1]: DeviceAttribute::structure
    pub const PROPERTIES: ConstName = ConstName::new("EXT_descriptor_heap_properties");
}

impl AnyExtensionDevice for Device {

    fn boxed(&self) -> Box<dyn AnyExtensionDevice> {
        Box::new(self.clone())
    }
}

impl ExtensionDevice for Device {

    const NAME: ConstName = ConstName::new("EXT_descriptor_heap");

    fn precondition<'a, F>(f: F) -> bool
        where F: Fn(&ConstName) -> Option<&'a DeviceAttribute>
    {
        f(&Attributes::IS_ENABLED)
            .is_some_and(|attr| attr.bool().is_some_and(|b| b))
    }

    fn new(device: &crate::gpu::Device) -> Box<Self> {
        Box::new(Device::new(device))
    }
}

/// The extension type.
#[derive(Clone, Copy)]
pub struct Extension;

unsafe impl DeviceExtension for Extension {

    fn get_info(&self, _attributes: &DeviceAttributes) -> Option<DeviceExtensionInfo> {
        Some(DeviceExtensionInfo {
            name: descriptor_heap::NAME,
            deprecation_version: VERSION_MAX,
            precondition: Precondition::new(|ctx| {
                let mut features = vk::PhysicalDeviceDescriptorHeapFeaturesEXT::default();
                ctx.get_features(&mut features);
                (features.descriptor_heap == 0)
                    .then(|| MissingDeviceFeatureError::new("descriptor_heap"))
            }),
        })
    }

    fn register(
        &self,
        ctx: &mut PhysicalDeviceContext<'_>,
    ) -> Option<ExtendsDeviceCreateInfoObj> {
        let mut properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default();
        ctx.get_properties(&mut properties);
        let properties = Properties {
            sampler_heap_alignment: properties.sampler_heap_alignment,
            resource_heap_alignment: properties.resource_heap_alignment,
            max_sampler_heap_size: properties.max_sampler_heap_size,
            max_resource_heap_size: properties.max_resource_heap_size,
            min_sampler_heap_reserved_range: properties.min_sampler_heap_reserved_range,
            min_sampler_heap_reserved_range_with_embedded:
                properties.min_sampler_heap_reserved_range_with_embedded,
            min_resource_heap_reserved_range: properties.min_resource_heap_reserved_range,
            sampler_descriptor_size: properties.sampler_descriptor_size,
            image_descriptor_size: properties.image_descriptor_size,
            buffer_descriptor_size: properties.buffer_descriptor_size,
            sampler_descriptor_alignment: properties.sampler_descriptor_alignment,
            image_descriptor_alignment: properties.image_descriptor_alignment,
            buffer_descriptor_alignment: properties.buffer_descriptor_alignment,
            max_push_data_size: properties.max_push_data_size,
        };
        ctx.register_attribute(DeviceAttribute::new_bool(Attributes::IS_ENABLED, true));
        ctx.register_attribute(DeviceAttribute::new_structure(
            Attributes::PROPERTIES, properties,
        ));
        Some(ExtendsDeviceCreateInfoObj::new(vk::PhysicalDeviceDescriptorHeapFeaturesEXT
            ::default()
            .descriptor_heap(true)
        ))
    }

    fn boxed(&self) -> Box<dyn DeviceExtension> {
        Box::new(*self)
    }
}

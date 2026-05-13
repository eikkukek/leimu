//! Provided by VK_EXT_descriptor_heap.
//!
//! Due to missing driver support, this extension is not yet fully supported.

use tuhka::ext::descriptor_heap;
use super::{*, device::*};

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

/// [`Properties`] [`device attribute`][1] name.
///
/// [1]: Gpu::get_device_attribute
pub const PROPERTIES: AttributeName<Properties> = AttributeName::new("EXT_descriptor_heap_properties");

impl AnyExtensionDevice for Device {

    fn boxed(&self) -> Box<dyn AnyExtensionDevice> {
        Box::new(self.clone())
    }
}

impl ExtensionDevice for Device {

    const NAME: ConstName = ConstName::new("EXT_descriptor_heap");

    fn precondition<'a, F>(f: F) -> bool
        where F: Fn(&ConstName) -> Option<&'a Attribute>
    {
        f(PROPERTIES.name())
            .is_some_and(|attr|
                attr.get::<Properties>().is_some()
            )
    }

    fn new(device: &crate::gpu::Device) -> Box<Self> {
        Box::new(Device::new(device))
    }
}
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
        ctx.register_attribute(Attribute::new(
            PROPERTIES, properties,
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

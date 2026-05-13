//! Provides an interface for enabling and using Vulkan device extensions.
//!
//! # Core extensions
//! The following device extensions are required by Leimu and are always enabled:
//! - [`VK_KHR_timeline_semaphore`][1]
//! - If [`multi_viewport`][2] is enabled, [`VK_EXT_shader_viewport_index_layer`][3]
//! - [`VK_KHR_create_renderpass2`][4]
//! - [`VK_KHR_depth_stencil_resolve`][5]
//! - [`VK_KHR_dynamic_rendering`][6]
//! - [`VK_KHR_format_feature_flags2`][7]
//! - [`VK_EXT_extended_dynamic_state`][8]
//! - [`VK_KHR_copy_commands2`][9]
//! - [`Vk_KHR_synchronization2`][10]
//! - [`VK_KHR_maintenance4`][11]
//! - [`VK_KHR_dynamic_rendering_local_read`][12]
//! - [`VK_KHR_maintenance5`][13]
//! - [`VK_KHR_maintenance6`][14]
//! - [`VK_KHR_present_id2`][15]
//! - [`VK_KHR_separate_depth_stencil_layouts`][19]
//! - [`VK_EXT_separate_stencil_usage`][20]
//!
//! # Provided extensions
//! The following device extensions have been implemented for Leimu and *can* be enabled by
//! applications:
//! - [`VK_KHR_push_descriptor`][push_descriptor]
//! - [`VK_EXT_inline_uniform_block`][inline_uniform_block]
//! - [`VK_KHR_index_type_uint8`][index_type_uint8]
//! - [`VK_KHR_robustness2`][robustness2]
//! - [`VK_EXT_pipeline_robustness`][pipeline_robustness]
//! - [`VK_EXT_descriptor_indexing`][descriptor_indexing]
//! - [`VK_EXT_mesh_shader`][mesh_shader]
//! - [`VK_EXT_extended_dynamic_state2`][extended_dynamic_state2]
//! - [`VK_EXT_extended_dynamic_state3`][extended_dynamic_state3]
//!
//! # Future extensions
//! These extensions will be added once driver support is more wide spread:
//! - [`VK_EXT_present_timing`][16]: gives the ability to schedule present operations to happen at a
//!   specific time.
//! - [`VK_EXT_descriptor_heap`][17]: replaces [`descriptor sets`][17] with new descriptor heaps.
//!
//! [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_timeline_semaphore.html
//! [2]: BaseDeviceFeatures::multi_viewport
//! [3]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_viewport_index_layer.html
//! [4]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_create_renderpass2.html
//! [5]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_depth_stencil_resolve.html
//! [6]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_dynamic_rendering.html
//! [7]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_format_feature_flags2.html
//! [8]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_extended_dynamic_state.html
//! [9]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_copy_commands2.html
//! [10]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_synchronization2.html
//! [11]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance4.html
//! [12]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_dynamic_rendering_local_read.html
//! [13]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance5.html
//! [14]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance6.html
//! [15]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_id2.html
//! [17]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_descriptor_heap.html
//! [18]: Gpu::allocate_descriptor_sets
//! [19]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_separate_depth_stencil_layouts.html
//! [20]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_separate_stencil_usage.html

mod core;
pub mod push_descriptor;
pub mod inline_uniform_block;
pub mod index_type_uint8;
pub mod robust_image_access;
pub mod robustness2;
pub mod pipeline_robustness;
pub mod descriptor_indexing;
pub mod mesh_shader;
pub mod extended_dynamic_state2;
pub mod extended_dynamic_state3;
//pub mod descriptor_heap;

pub(crate) use core::core_extensions;

use {
    ::core::{
        any::Any,
        ffi::CStr,
        hash::{self, Hash},
        borrow::Borrow,
        ops::{Deref, DerefMut},
        mem,
        ptr::NonNull,
        fmt::{self, Display, Debug},
        marker::PhantomData,
    },
    ahash::{AHashSet, AHashMap},
    tuhka::{
        vk,
    },
    crate::{
        gpu::prelude::*,
        core::{
            OptionExt,
            EntryExt,
        },
        sync::Mutex,
    },
};

/// Specifies requirements of a particular feature.
#[derive(Clone, Copy)]
pub enum FeatureRequirements {
    /// Support for the feature isn't checked.
    DontCare,
    /// The feature is required.
    Require,
    /// The feature is required and enabled.
    Enable,
}

impl FeatureRequirements {

    #[inline]
    pub fn is_required(self) -> bool {
        matches!(self, Self::Require | Self::Enable)
    }
}

pub mod prelude {

    use super::*; 

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AttributeName<T>
        where T: Any + Send + Sync
    {
        name: ConstName,
        _marker: PhantomData<T>,
    }

    impl<T> AttributeName<T> 
        where T: Any + Send + Sync
    {

        #[inline]
        pub const fn new(name: &'static str) -> Self {
            Self {
                name: ConstName::new(name),
                _marker: PhantomData,
            }
        }

        #[inline]
        pub const fn name(&self) -> &ConstName {
            &self.name
        }
    }

    impl<T> Debug for AttributeName<T>
        where T: Any + Send + Sync
    {

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            Debug::fmt(&self.name, f)
        }
    }

    impl<T> Display for AttributeName<T>
        where T: Any + Send + Sync
    {

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            Display::fmt(&self.name, f)
        }
    }

    pub struct Attribute {
        name: ConstName,
        inner: Box<dyn Any + Send + Sync>,
    }

    impl Attribute {

        #[inline]
        pub fn new<T>(
            name: AttributeName<T>,
            value: T,
        ) -> Self
            where T: Any + Send + Sync
        {
            Self {
                name: *name.name(),
                inner: Box::new(value),
            }
        }

        #[inline]
        pub fn get<T: Any>(&self) -> Option<&T> {
            let value = self.inner.as_ref();
            if value.is::<T>() {
                let ptr: *const dyn Any = value;
                Some(unsafe {
                    &*ptr.cast()
                })
            } else {
                None
            }
        }
    }

    impl PartialEq for Attribute {
        
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
        }
    }

    impl Eq for Attribute {}

    impl Hash for Attribute {

        #[inline]
        fn hash<H: hash::Hasher>(&self, state: &mut H) {
            self.name.hash(state);
        }
    }

    impl Borrow<ConstName> for Attribute {
        
        #[inline]
        fn borrow(&self) -> &ConstName {
            &self.name
        }
    }
}

use prelude::*;

/// Prelude for implementing new device extensions.
pub mod device {

    use leimu_mem::vec::ArrayVec;

    use crate::default;

    use super::*;

    pub trait ExtendsDeviceCreateInfoExt {

        fn s_type(&self) -> vk::StructureType;
        fn as_mut(&mut self) -> &mut dyn vk::ExtendsDeviceCreateInfo;
        fn to_obj(&self) -> ExtendsDeviceCreateInfoObj;
    }

    impl<T> ExtendsDeviceCreateInfoExt for T
        where T:
            vk::ExtendsDeviceCreateInfo +
            Clone + Copy + 'static
    {
        fn s_type(&self) -> vk::StructureType {
            self.base_in()
                .s_type
        }

        fn as_mut(&mut self) -> &mut dyn vk::ExtendsDeviceCreateInfo {
            self
        }

        fn to_obj(&self) -> ExtendsDeviceCreateInfoObj {
            ExtendsDeviceCreateInfoObj::new(*self)
        }
    }

    pub struct ExtendsDeviceCreateInfoObj(Box<dyn ExtendsDeviceCreateInfoExt>);

    impl ExtendsDeviceCreateInfoObj {

        #[inline]
        pub fn new<T>(x: T) -> Self
            where T: ExtendsDeviceCreateInfoExt + 'static
        {
            Self(Box::new(x))
        }
    }

    impl Deref for ExtendsDeviceCreateInfoObj {

        type Target = dyn ExtendsDeviceCreateInfoExt;

        #[inline]
        fn deref(&self) -> &Self::Target {
            &*self.0
        }
    }

    impl DerefMut for ExtendsDeviceCreateInfoObj {

        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.0
        }
    } 

    #[derive(Debug)]
    pub struct MissingDeviceFeatureError {
        missing: String,
    }

    impl Display for MissingDeviceFeatureError {

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.missing)
        }
    }

    impl MissingDeviceFeatureError {

        #[inline]
        pub fn new(missing_features: &str) -> Self
        {
            Self {
                missing: missing_features.to_string(),
            }
        }
    }

    /// A trait for cloning [`ExtensionDevice`] trait objects.
    pub trait AnyExtensionDevice: Any + Send + Sync {

        /// Clones self to a [`Box`].
        fn boxed(&self) -> Box<dyn AnyExtensionDevice>;
    }

    pub struct ExtensionDeviceObj(Box<dyn AnyExtensionDevice>);

    impl Deref for ExtensionDeviceObj {

        type Target = dyn Any;
        
        #[inline]
        fn deref(&self) -> &Self::Target {
            &*self.0
        }
    }

    impl Clone for ExtensionDeviceObj {

        #[inline]
        fn clone(&self) -> Self {
            Self(self.0.boxed())
        }
    }

    /// A trait for a extension's device-level functions.
    pub trait ExtensionDevice: AnyExtensionDevice + Clone {

        /// The name hash of the extension this device comes from.
        const EXT_NAME: ConstName;

        /// Creates a new Device from [`Device`].
        fn new(device: &Device) -> Box<Self>;
    }

    pub struct EnabledDeviceExtensions {
        attributes: AHashSet<Attribute>,
        enabled_extensions: AHashSet<ConstName>,
        supported_dynamic_state: ArrayVec<DynamicState, {DynamicState::VARIANT_COUNT}>,
        extension_devices: Mutex<AHashMap<ConstName, ExtensionDeviceObj>>
    }

    impl EnabledDeviceExtensions {

        #[inline]
        pub fn new() -> Self {
            Self {
                attributes: default(),
                enabled_extensions: default(),
                supported_dynamic_state: [
                    DynamicState::LineWidth,
                    DynamicState::DepthBias,
                    DynamicState::BlendConstants,
                    DynamicState::DepthBounds,
                    DynamicState::StencilCompareMask,
                    DynamicState::StencilWriteMask,
                    DynamicState::StencilReference,
                ].into_iter().collect(),
                extension_devices: default(),
            }
        }

        #[inline]
        pub(crate) fn add_extension(
            &mut self,
            name: ConstName
        ) -> bool {
            self.enabled_extensions.insert(name)
        }

        #[inline]
        pub fn contains_extension(
            &self,
            name: &ConstName
        ) -> bool {
            self.enabled_extensions.contains(&name)
        }
        
        #[inline]
        pub fn add_attribute(&mut self, property: Attribute) {
            self.attributes.insert(property);
        }

        #[inline]
        pub fn get_attribute(&self, name: &ConstName) -> &Attribute {
            self.attributes
                .get(&name)
                .unwrap_or_default()
        }

        #[inline]
        pub fn add_supported_dynamic_state(&mut self, state: DynamicState) {
            if !self.supported_dynamic_state.contains(&state) {
                self.supported_dynamic_state.push(state);
            }
        }

        #[inline]
        pub fn contains_dynamic_state(&self, state: DynamicState) -> bool {
            self.supported_dynamic_state.contains(&state)
        }

        #[inline]
        pub(crate) fn get_device<T: ExtensionDevice + 'static>(
            &self,
            device: &Device,
        ) -> Option<&T>
        {
            let mut devices = self.extension_devices.lock();
            let obj = devices.entry(T::EXT_NAME)
                .or_try_insert_with(|| {
                    if !self.enabled_extensions.contains(&T::EXT_NAME) {
                        return Err(format!("extension {} not enabled", T::EXT_NAME))
                    }
                    Ok(ExtensionDeviceObj(T::new(device)))
                }).inspect_err(|err| {
                    log::debug!("{}", err);
                }).ok()?;
            obj.is::<T>().then(|| unsafe {
                mem::transmute::<
                    &dyn AnyExtensionDevice,
                    (NonNull<T>, *const ())
                >(obj.0.deref()).0
                .as_ref()
            })
        }
    }

    pub struct PhysicalDeviceContext<'a> {
        instance: &'a Instance,
        physical_device: &'a PhysicalDevice,
        vulkan_12_features: &'a mut Option<vk::PhysicalDeviceVulkan12Features<'static>>,
        vulkan_13_features: &'a mut Option<vk::PhysicalDeviceVulkan13Features<'static>>,
        vulkan_14_features: &'a mut Option<vk::PhysicalDeviceVulkan14Features<'static>>,
        enabled_extensions: Option<&'a mut EnabledDeviceExtensions>,
    }

    impl<'a> PhysicalDeviceContext<'a> {

        #[inline]
        pub fn new(
            instance: &'a Instance,
            physical_device: &'a PhysicalDevice,
            vulkan_12_features: &'a mut Option<vk::PhysicalDeviceVulkan12Features<'static>>,
            vulkan_13_features: &'a mut Option<vk::PhysicalDeviceVulkan13Features<'static>>,
            vulkan_14_features: &'a mut Option<vk::PhysicalDeviceVulkan14Features<'static>>,
            enabled_extensions: Option<&'a mut EnabledDeviceExtensions>,
        ) -> Self {
            Self {
                instance,
                physical_device,
                vulkan_12_features,
                vulkan_13_features,
                vulkan_14_features,
                enabled_extensions,
            }
        }

        #[inline]
        pub fn api_version(&self) -> Version {
            self.physical_device.api_version()
        }

        #[inline]
        pub fn get_features<T>(
            &self,
            out: &mut T
        ) where T: vk::ExtendsPhysicalDeviceFeatures2,
        {
            let mut features = vk::PhysicalDeviceFeatures2
                ::default()
                .push_next(out);
            unsafe {
                self.instance.get_physical_device_features2(
                    self.physical_device.handle(), &mut features,
                );
            }
        }

        #[inline]
        pub fn get_properties<T>(
            &self,
            out: &mut T,
        ) where T: vk::ExtendsPhysicalDeviceProperties2,
        {
            let mut properties = vk::PhysicalDeviceProperties2
                ::default()
                .push_next(out);
            unsafe {
                self.instance.get_physical_device_properties2(
                    self.physical_device.handle(),
                    &mut properties
                );
            }
        }

        #[inline]
        pub fn vulkan_12_features(&mut self) -> &mut vk::PhysicalDeviceVulkan12Features<'static> {
            self.vulkan_12_features.get_or_insert_default()
        }

        #[inline]
        pub fn vulkan_13_features(&mut self) -> &mut vk::PhysicalDeviceVulkan13Features<'static> {
            self.vulkan_13_features.get_or_insert_default()
        }

        #[inline]
        pub fn vulkan_14_features(&mut self) -> &mut vk::PhysicalDeviceVulkan14Features<'static> {
            self.vulkan_14_features.get_or_insert_default()
        }

        #[inline]
        pub fn register_attribute(&mut self, attribute: Attribute) {
            self.enabled_extensions.edit(|extensions| {
                extensions.add_attribute(attribute);
            });
        }

        #[inline]
        pub fn add_supported_dynamic_state<I>(&mut self, states: I)
            where I: IntoIterator<Item = DynamicState>
        {
            self.enabled_extensions.edit(|extensions| {
                for state in states {
                    extensions.add_supported_dynamic_state(state);
                }
            });
        }
    }

    type FnPrecondition = dyn Fn(&PhysicalDeviceContext<'_>) -> Option<MissingDeviceFeatureError>;

    pub struct Precondition(Box<FnPrecondition>);

    impl Precondition {

        #[inline]
        pub fn new<F>(f: F) -> Option<Self>
            where F: Fn(&PhysicalDeviceContext<'_>) -> Option<MissingDeviceFeatureError> + 'static
        {
            Some(Self(Box::new(f)))
        }

        #[inline]
        pub fn call(&self, ctx: &PhysicalDeviceContext<'_>) -> Option<MissingDeviceFeatureError> {
            (self.0)(ctx)
        }
    }

    #[derive(Default)]
    pub struct DeviceExtensionInfo {
        pub name: &'static CStr,
        pub deprecation_version: Version,
        pub precondition: Option<Precondition>,
    }

    pub struct RegisteredExtension {
        pub name: ConstName,
        pub extends_create_info: Option<ExtendsDeviceCreateInfoObj>,
    }

    #[inline]
    pub fn registered_extension(
        name: ConstName,
        extends_create_info: Option<ExtendsDeviceCreateInfoObj>,
    ) -> RegisteredExtension {
        RegisteredExtension { name, extends_create_info }
    }

    /// # Safety
    /// You should only implement this trait if you know what you are doing.
    pub unsafe trait DeviceExtension: 'static + Send + Sync {

        /// Conditionally gets info about the extension.
        fn get_info(&self, attributes: &DeviceAttributes) -> Option<DeviceExtensionInfo>;

        /// Registers the extension and optionally returns a structure extending
        /// [`vk::DeviceCreateInfo`].
        fn register(
            &self,
            ctx: &mut PhysicalDeviceContext<'_>,
        ) -> RegisteredExtension;

        /// Clones self to a box.
        fn boxed(&self) -> Box<dyn DeviceExtension>;
    }

    pub struct DeviceExtensionObj(Box<dyn DeviceExtension>);

    impl From<Box<dyn DeviceExtension>> for DeviceExtensionObj {

        #[inline]
        fn from(value: Box<dyn DeviceExtension>) -> Self {
            Self(value)
        }
    }

    impl Clone for DeviceExtensionObj {

        #[inline]
        fn clone(&self) -> Self {
            Self(self.0.boxed())
        }
    }

    impl Deref for DeviceExtensionObj {

        type Target = dyn DeviceExtension;

        #[inline]
        fn deref(&self) -> &Self::Target {
            &*self.0
        }
    }
}

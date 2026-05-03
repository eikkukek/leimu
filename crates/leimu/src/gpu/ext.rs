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
//! - [`VK_KHR_present_wait2`][16]
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
//!
//! # Future extensions
//!  *can* be enabled, but doesn't yet have a high level
//! interface for usage in commands.
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
//! [16]: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_wait2.html

mod core;
pub mod push_descriptor;
pub mod inline_uniform_block;
pub mod index_type_uint8;
pub mod robust_image_access;
pub mod robustness2;
pub mod pipeline_robustness;
pub mod descriptor_indexing;
pub mod mesh_shader;
pub mod descriptor_heap;

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
        fmt::{self, Display},
    },
    ahash::{AHashSet, AHashMap},
    tuhka::{
        vk,
    },
    crate::{
        gpu::prelude::*,
        core::{
            OptionExt,
            collections::EntryExt,
        },
        sync::Mutex,
    },
};

pub trait ExtendsDeviceCreateInfoExt {

    fn s_type(&self) -> vk::StructureType;
    fn as_mut(&mut self) -> &mut dyn vk::ExtendsDeviceCreateInfo;
    fn to_obj(&self) -> ExtendsDeviceCreateInfoObj;
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

#[derive(Clone, Copy)]
pub enum RobustAccessRequirements {
    NotRequired,
    Required,
    Enabled,
}

impl RobustAccessRequirements {

    #[inline]
    pub fn is_required(self) -> bool {
        matches!(self, Self::Required | Self::Enabled)
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

#[derive(Clone, Copy, Debug)]
pub struct ConstName {
    name: &'static str,
    hash: u64,
}

impl Display for ConstName {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ConstName {

    /// Uses 64-bit FNV-1 hash.
    pub const fn new(name: &'static str) -> Self {
        let mut hash = 0xcbf29ce484222325u64;
        let len = name.len();
        let bytes = name.as_bytes();
        let mut i = 0;
        while i < len {
            hash ^= bytes[i] as u64;
            hash = hash.wrapping_mul(0x00000100000001b3u64);
            i += 1;
        }
        Self {
            name,
            hash,
        }
    }
}

impl Hash for ConstName {

    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl PartialEq for ConstName {

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for ConstName {}

enum AttributeType {
    None,
    Bool(bool),
    U32(u32),
    I32(i32),
    DeviceSize(vk::DeviceSize),
    Structure(Box<dyn Any + Send + Sync>),
}

pub struct DeviceAttribute {
    name: ConstName,
    ty: AttributeType,
}

impl DeviceAttribute {

    #[inline]
    const fn empty() -> Self {
        Self {
            name: ConstName::new(""),
            ty: AttributeType::None,
        }
    }

    #[inline]
    pub fn new_bool(name: ConstName, value: bool) -> Self {
        Self {
            name,
            ty: AttributeType::Bool(value),
        }
    }

    #[inline]
    pub fn new_u32(name: ConstName, value: u32) -> Self {
        Self {
            name,
            ty: AttributeType::U32(value),
        }
    }

    #[inline]
    pub fn new_i32(name: ConstName, value: i32) -> Self {
        Self {
            name,
            ty: AttributeType::I32(value),
        }
    }

    #[inline]
    pub fn new_device_size(name: ConstName, value: vk::DeviceSize) -> Self {
        Self {
            name,
            ty: AttributeType::DeviceSize(value),
        }
    }

    #[inline]
    pub fn new_structure<
        T: Any + Send + Sync
    >(name: ConstName, value: T) -> Self {
        Self {
            name,
            ty: AttributeType::Structure(Box::new(value)),
        }
    }

    #[inline]
    pub fn bool(&self) -> Option<bool> {
        match self.ty {
            AttributeType::Bool(value) => Some(value),
            _ => None,
        }
    }

    #[inline]
    pub fn u32(&self) -> Option<u32> {
        match self.ty {
            AttributeType::U32(value) => Some(value),
            _ => None,
        }
    }

    #[inline]
    pub fn i32(&self) -> Option<i32> {
        match self.ty {
            AttributeType::I32(value) => Some(value),
            _ => None,
        }
    }

    #[inline]
    pub fn device_size(&self) -> Option<vk::DeviceSize> {
        match self.ty {
            AttributeType::DeviceSize(value) => Some(value),
            _ => None,
        }
    }

    #[inline]
    pub fn structure<T: Any>(&self) -> Option<&T> {
        match &self.ty {
            AttributeType::Structure(value) => {
                if value.is::<T>() {
                    let ptr: *const dyn Any = value.as_ref();
                    Some(unsafe {
                        &*ptr.cast()
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Default for &DeviceAttribute {

    #[inline]
    fn default() -> Self {
        static DEFAULT: DeviceAttribute = DeviceAttribute::empty();
        &DEFAULT
    }
}

impl PartialEq for DeviceAttribute {
    
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for DeviceAttribute {}

impl Hash for DeviceAttribute {

    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Borrow<ConstName> for DeviceAttribute {
    
    #[inline]
    fn borrow(&self) -> &ConstName {
        &self.name
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

    /// The name hash for the device.
    const NAME: ConstName;

    /// Precondition for using the device.
    ///
    /// Should return `true` when the precondition is met.
    fn precondition<'a, F>(f: F) -> bool
        where F: Fn(&ConstName) -> Option<&'a DeviceAttribute>;

    /// Creates a new Device from [`Device`].
    fn new(device: &Device) -> Box<Self>;
}

#[derive(Default)]
pub struct EnabledDeviceExtensions {
    attributes: AHashSet<DeviceAttribute>,
    extension_devices: Mutex<AHashMap<ConstName, ExtensionDeviceObj>>
}

impl EnabledDeviceExtensions {

    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
    
    #[inline]
    pub fn add_attribute(&mut self, property: DeviceAttribute) {
        self.attributes.insert(property);
    }

    #[inline]
    pub fn get_attribute(&self, name: ConstName) -> &DeviceAttribute {
        self.attributes
            .get(&name)
            .unwrap_or_default()
    }

    #[inline]
    pub(crate) fn get_device<T: ExtensionDevice + 'static>(
        &self,
        device: &Device,
    ) -> Option<&T>
    {
        let mut devices = self.extension_devices.lock();
        let obj = devices.entry(T::NAME)
            .or_try_insert_with(|| {
                if !T::precondition(|name| {
                    self.attributes.get(name)
                }) {
                    return Err(format!(
                        "precondition for {} not met", T::NAME.name
                    ))
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
    vulkan_14_features: &'a mut Option<vk::PhysicalDeviceVulkan14Features<'static>>,
    enabled_extensions: Option<&'a mut EnabledDeviceExtensions>,
}

impl<'a> PhysicalDeviceContext<'a> {

    #[inline]
    pub fn new(
        instance: &'a Instance,
        physical_device: &'a PhysicalDevice,
        vulkan_12_features: &'a mut Option<vk::PhysicalDeviceVulkan12Features<'static>>,
        vulkan_14_features: &'a mut Option<vk::PhysicalDeviceVulkan14Features<'static>>,
        enabled_extensions: Option<&'a mut EnabledDeviceExtensions>,
    ) -> Self {
        Self {
            instance,
            physical_device,
            vulkan_12_features,
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
    pub fn vulkan_14_features(&mut self) -> &mut vk::PhysicalDeviceVulkan14Features<'static> {
        self.vulkan_14_features.get_or_insert_default()
    }

    #[inline]
    pub fn register_attribute(&mut self, attribute: DeviceAttribute) {
        self.enabled_extensions.edit(|extensions| {
            extensions.add_attribute(attribute);
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
    ) -> Option<ExtendsDeviceCreateInfoObj>;

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

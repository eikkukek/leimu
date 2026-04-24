//! New up-to-date Vulkan bindings.
//!
//! # Usage
//! ```
//! use tuhka::{vk, Library, Instance};
//! let library = unsafe {
//!      Library::load()
//! }.unwrap();
//! let app_name = c"Test";
//! let engine_name = c"Engine";
//! let application_info = vk::ApplicationInfo {
//!     p_application_name: app_name.as_ptr(),
//!     application_version: vk::make_api_version(0, 1, 0, 0),
//!     p_engine_name: engine_name.as_ptr(),
//!     engine_version: vk::make_api_version(0, 1, 0, 0),
//!     api_version: vk::API_VERSION_1_4,
//!     ..Default::default()
//! };
//! let create_info = vk::InstanceCreateInfo {
//! p_application_info: &application_info,
//!     ..Default::default()
//! };
//! let instance: Instance = unsafe {
//!     library.create_instance(&create_info, None)
//! }.unwrap().value;
//! unsafe {
//!     library.destroy_instance(&instance, None);
//! }

#![no_std]

pub mod vk;
mod option;
mod library;
mod instance;
mod device;
mod result;
mod core_gen;
mod extension_gen;
#[cfg(feature = "window")]
pub mod window;

pub(crate) use option::PtrOption;
pub use result::*;
pub use core_gen::*;

pub use library::*;
pub use instance::*;
pub use device::*;
pub use extension_gen::*;
mod macros;

/// A trait for loading extension instances/devices generically.
pub trait LoadWith {

    /// The handle type, which is either [`vk::Device`] or [`vk::Instance`].
    type Handle;

    /// Loads the device.
    ///
    /// # Safety
    /// `handle` *must* be a valid handle and `f` *must* yield valid function pointers for
    /// `handle`.
    unsafe fn load_with(
        f: &mut dyn FnMut(&core::ffi::CStr) -> *const core::ffi::c_void,
        handle: Self::Handle,
    ) -> Self;
}

pub mod nop {
    use super::*;
    pub struct Device;
    impl LoadWith for Device {

        type Handle = vk::Device;

        unsafe fn load_with(
            _f: &mut dyn FnMut(&core::ffi::CStr) -> *const core::ffi::c_void,
            _handle: Self::Handle,
        ) -> Self {
            Self
        }
    }
    pub struct Instance;
    impl LoadWith for Instance {

        type Handle = vk::Instance;

        unsafe fn load_with(
            _f: &mut dyn FnMut(&core::ffi::CStr) -> *const core::ffi::c_void,
            _handle: Self::Handle,
        ) -> Self {
            Self
        }
    }
}

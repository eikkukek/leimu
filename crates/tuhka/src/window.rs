use core::{ffi, fmt::{self, Display}, error::Error};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use crate::*;

/// An error associated with [`creating surfaces`][1].
///
/// [1]: create_surface
#[derive(Debug)]
pub enum CreateSurfaceError {
    VkErr(vk::Result),
    MissingHandle(&'static str),
    UnsupportedPlatform,
}

impl Display for CreateSurfaceError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VkErr(err) => write!(f, "{err}"),
            Self::MissingHandle(h) => write!(f, "missing handle {h}"),
            Self::UnsupportedPlatform => write!(f, "unsupported platform"),
        }
    }
}

impl Error for CreateSurfaceError {}

/// Gets required instance extensions for [`creating surfaces`][1] for the given display.
///
/// Returns [`None`] if the platform is not supported.
///
/// See [`create_surface`] for supported platforms.
///
/// [1]: create_surface
pub fn required_instance_extensions(
    display_handle: RawDisplayHandle,
) -> Option<&'static [&'static ffi::CStr]>
{
    match display_handle {
        RawDisplayHandle::Windows(_) => {
            Some(&[khr::surface::NAME, khr::win32_surface::NAME])
        },
        RawDisplayHandle::Wayland(_) => {
            Some(&[khr::surface::NAME, khr::wayland_surface::NAME])
        },
        RawDisplayHandle::Xlib(_) => {
            Some(&[khr::surface::NAME, khr::xlib_surface::NAME])
        },
        RawDisplayHandle::Xcb(_) => {
            Some(&[khr::surface::NAME, khr::xcb_surface::NAME])
        },
        RawDisplayHandle::Android(_) => {
            Some(&[khr::surface::NAME, khr::android_surface::NAME])
        },
        RawDisplayHandle::AppKit(_) | RawDisplayHandle::UiKit(_) => {
            Some(&[khr::surface::NAME, ext::metal_surface::NAME])
        },
        RawDisplayHandle::Ohos(_) => {
            Some(&[khr::surface::NAME, ohos::surface::NAME])
        },
        _ => None,
    }
}

/// Creates a surface for the given [`RawDisplayHandle`], [`RawWindowHandle`] pair.
///
/// # Supported platforms
/// * Win32
/// * Wayland
/// * Xlib
/// * Xcb
/// * Android
/// * macos (AppKit/metal)
/// * ios (UiKit/metal)
/// * Ohos
///
/// # Safety
/// All raw Vulkan calls are unsafe as there is no validation of input or usage.
pub unsafe fn create_surface<Ext>(
    instance: &Instance<Ext>,
    display_handle: RawDisplayHandle,
    window_handle: RawWindowHandle,
    allocator: Option<&vk::AllocationCallbacks>
) -> Result<vk::SurfaceKHR, CreateSurfaceError>
{
    match (display_handle, window_handle) {
        (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(window)) => {
            let create_info = vk::Win32SurfaceCreateInfoKHR::default()
                .hwnd(window.hwnd.get())
                .hinstance(
                    window 
                        .hinstance
                        .ok_or(CreateSurfaceError::MissingHandle("HINSTANCE"))?
                        .get(),
                );
            let instance = khr::win32_surface::Instance
                ::new(instance);
            unsafe {
                instance.create_win32_surface(
                    &create_info,
                    allocator
                ).map(|res| res.value)
                .map_err(CreateSurfaceError::VkErr)
            }
        },
        (RawDisplayHandle::Wayland(display), RawWindowHandle::Wayland(window)) => {
            let create_info = vk::WaylandSurfaceCreateInfoKHR::default()
                .display(display.display.as_ptr())
                .surface(window.surface.as_ptr());
            let instance = khr::wayland_surface::Instance
                ::new(instance);
            unsafe {
                instance.create_wayland_surface(
                    &create_info,
                    allocator
                ).map(|res| res.value)
                .map_err(CreateSurfaceError::VkErr)
            }
        },
        (RawDisplayHandle::Xlib(display), RawWindowHandle::Xlib(window)) => {
            let create_info = vk::XlibSurfaceCreateInfoKHR::default()
                .dpy(display.display
                    .ok_or(CreateSurfaceError::MissingHandle("xlib Display"))?
                    .as_ptr()
                ).window(window.window);
            let instance = khr::xlib_surface::Instance
                ::new(instance);
            unsafe {
                instance.create_xlib_surface(
                    &create_info,
                    allocator
                ).map(|res| res.value)
                .map_err(CreateSurfaceError::VkErr)
            }
        },
        (RawDisplayHandle::Xcb(display), RawWindowHandle::Xcb(window)) => {
            let create_info = vk::XcbSurfaceCreateInfoKHR::default()
                .connection(
                    display.connection
                        .ok_or(CreateSurfaceError::MissingHandle("xcb_connection_t"))?
                        .as_ptr()
                ).window(window.window.get());
            let instance = khr::xcb_surface::Instance
                ::new(instance);
            unsafe {
                instance.create_xcb_surface(
                    &create_info,
                    allocator
                ).map(|res| res.value)
                .map_err(CreateSurfaceError::VkErr)
            }
        },
        (RawDisplayHandle::Android(_), RawWindowHandle::AndroidNdk(window)) => {
            let create_info = vk::AndroidSurfaceCreateInfoKHR::default()
                .window(window.a_native_window.as_ptr());
            let instance = khr::android_surface::Instance::new(instance);
            unsafe {
                instance.create_android_surface(
                    &create_info,
                    allocator
                ).map(|res| res.value)
                .map_err(CreateSurfaceError::VkErr)
            }
        },
        #[cfg(target_os = "macos")]
        (RawDisplayHandle::AppKit(_), RawWindowHandle::AppKit(window)) => {
            use raw_window_metal::Layer;
            let layer = unsafe {
                Layer::from_ns_view(window.ns_view).into_raw()
            };
            let create_info = vk::MetalSurfaceCreateInfoEXT::default()
                .p_layer(layer.as_ptr());
            let instance = ext::metal_surface::Instance::new(instance);
            unsafe {
                instance.create_metal_surface(
                    &create_info,
                    allocator
                ).map(|res| res.value)
                .map_err(CreateSurfaceError::VkErr)
            }
        },
        #[cfg(target_os = "ios")]
        (RawDisplayHandle::UiKit(_), RawWindowHandle::UiKit(window)) => {
            use raw_window_metal::Layer;
            let layer = unsafe {
                Layer::from_ui_view(window.ui_view).into_raw()
            };
            let create_info = vk::MetalSurfaceCreateInfoEXT::default()
                .p_layer(layer.as_ptr());
            let instance = ext::metal_surface::Instance::new(instance);
            unsafe {
                instance.create_metal_surface(
                    &create_info,
                    allocator
                ).map(|res| res.value)
                .map_err(CreateSurfaceError::VkErr)
            }
        },
        (RawDisplayHandle::Ohos(_), RawWindowHandle::OhosNdk(window)) => {
            let create_info = vk::SurfaceCreateInfoOHOS::default()
                .window(window.native_window.as_ptr());
            let instance = ohos::surface::Instance::new(instance);
            unsafe {
                instance.create_surface(
                    &create_info,
                    allocator
                ).map(|res| res.value)
                .map_err(CreateSurfaceError::VkErr)
            }
        },
        _ => Err(CreateSurfaceError::UnsupportedPlatform)
    }
}

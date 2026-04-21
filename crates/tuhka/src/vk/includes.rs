#![allow(non_camel_case_types)]

use super::*;

pub type Display = ffi::c_void;
pub type VisualID = ffi::c_uint;
pub type Window = ffi::c_ulong;
pub type RROutput = ffi::c_ulong;
pub type wl_display = ffi::c_void;
pub type wl_surface = ffi::c_void;
pub type ubm_device = ffi::c_void;
pub type ubm_surface = ffi::c_void;
pub type HANDLE = isize;
pub type HINSTANCE = HANDLE;
pub type HWND = HANDLE;
pub type HMONITOR = HANDLE;
pub type DWORD = ffi::c_ulong;
pub type SECURITY_ATTRIBUTES = ffi::c_void;
pub type LPCWSTR = *const u16;
pub type xcb_connection_t = ffi::c_void;
pub type xcb_visualid_t = u32;
pub type xcb_window_t = u32;
pub type IDirectFB = ffi::c_void;
pub type IDirectFBSurface = ffi::c_void;
pub type zx_handle_t = u32;
pub type GgpStreamDescriptor = u32;
pub type GgpFrameToken = u64;
pub type _screen_context = ffi::c_void;
pub type _screen_window = ffi::c_void;
pub type _screen_buffer = ffi::c_void;
pub type __IOSurface = ffi::c_void;

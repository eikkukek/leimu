use core::ffi;
use crate::{
    vk,
    nop,
    LoadWith,
    DeviceFpV10,
    DeviceFpV11,
    DeviceFpV12,
    DeviceFpV13,
    DeviceFpV14,
    InstanceFpV10,
};

#[cfg(feature = "ext-load-with")]
use core::ops::Deref;

/// # Vulkan docs
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDevice.html>
#[derive(Clone)]
pub struct Device<Ext = nop::Device>
{
    pub(crate) handle: vk::Device,
    pub(crate) fp_v10: DeviceFpV10,
    pub(crate) fp_v11: DeviceFpV11,
    pub(crate) fp_v12: DeviceFpV12,
    pub(crate) fp_v13: DeviceFpV13,
    pub(crate) fp_v14: DeviceFpV14,
    pub(crate) _ext: Ext,
}

impl<Ext> Device<Ext>
{

    /// Loads the [`Device`] from [`InstanceFpV10`].
    ///
    /// # Safety
    /// It has to be ensured that [`get_device_proc_addr`][1] yields valid Vulkan function pointers
    /// and that `handle` is a valid [`vk::Device`].
    ///
    /// [1]: InstanceFpV10::get_device_proc_addr
    pub unsafe fn load(
        version: u32,
        instance_fp: &InstanceFpV10,
        handle: vk::Device
    ) -> Self
        where Ext: LoadWith<Handle = vk::Device>
    {
        unsafe {
            Self::load_with(
                version,
                &mut move |cname| {
                    (instance_fp.get_device_proc_addr)(handle, cname.as_ptr())
                        as *const ffi::c_void
                },
                handle,
            )
        }
    }

    /// Loads the [`Device`] with a function.
    ///
    /// # Safety
    /// It has to be ensured that `handle` is a valid [`vk::Device`] and that `f` returns valid
    /// Vulkan function pointers for `handle`.
    pub unsafe fn load_with(
        version: u32,
        f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void,
        handle: vk::Device,
    ) -> Self 
        where Ext: LoadWith<Handle = vk::Device>
    {
        Self {
            handle,
            fp_v10: DeviceFpV10::load(version, f),
            fp_v11: DeviceFpV11::load(version, f),
            fp_v12: DeviceFpV12::load(version, f),
            fp_v13: DeviceFpV13::load(version, f),
            fp_v14: DeviceFpV14::load(version, f),
            _ext: unsafe {
                Ext::load_with(f, handle)
            },
        }
    }

    #[inline]
    pub fn handle(&self) -> vk::Device {
        self.handle
    }

    #[inline]
    pub fn fp_v10(&self) -> &DeviceFpV10 {
        &self.fp_v10
    }

    #[inline]
    pub fn fp_v11(&self) -> &DeviceFpV11 {
        &self.fp_v11
    }

    #[inline]
    pub fn fp_v12(&self) -> &DeviceFpV12 {
        &self.fp_v12
    }

    #[inline]
    pub fn fp_v13(&self) -> &DeviceFpV13 {
        &self.fp_v13
    }

    #[inline]
    pub fn fp_v14(&self) -> &DeviceFpV14 {
        &self.fp_v14
    }
}

#[cfg(feature = "ext-load-with")]
impl<Ext> Deref for Device<Ext> {
    type Target = Ext;
    #[inline]
    fn deref(&self) -> &Ext {
        &self._ext
    }
}

use core::ffi;
use crate::{
    vk,
    nop,
    CoreFp,
    InstanceFpV10,
    InstanceFpV11,
    InstanceFpV13,
    VkResult,
    PtrOption,
    LoadWith,
};

#[cfg(feature = "ext-load-with")]
use core::ops::Deref;

/// # Vulkan docs
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkInstance.html>
#[derive(Clone)]
pub struct Instance<Ext = nop::Instance> 
{
    pub(crate) handle: vk::Instance,
    pub(crate) fp_v10: InstanceFpV10,
    pub(crate) fp_v11: InstanceFpV11,
    pub(crate) fp_v13: InstanceFpV13,
    pub(crate) _ext: Ext,
}

impl<Ext> Instance<Ext> {

    /// Loads the [`Instance`] from [`CoreFp`].
    ///
    /// # Safety
    /// It has to be ensured that [`get_instance_proc_addr`][1] yields valid Vulkan function
    /// pointers and that `handle` is a valid [`vk::Instance`].
    ///
    /// [1]: CoreFp::get_instance_proc_addr
    pub unsafe fn load(
        version: u32,
        core_fp: &CoreFp,
        handle: vk::Instance
    ) -> Self
        where Ext: LoadWith<Handle = vk::Instance>,
    {
        unsafe {
            Self::load_with(
                version,
                &mut move |cname| {
                    (core_fp.get_instance_proc_addr)(
                        handle,
                        cname.as_ptr(),
                    ) as *const ffi::c_void
                },
                handle
            )
        }
    }

    /// Loads the [`Instance`] with a function.
    ///
    /// # Safety
    /// It has to be ensured that `handle` is a valid [`vk::Instance`] and that `f` returns valid
    /// Vulkan function pointers for `handle`.
    pub unsafe fn load_with(
        version: u32,
        f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void,
        handle: vk::Instance,
    ) -> Self
        where Ext: LoadWith<Handle = vk::Instance>,
    {
        Self {
            handle,
            fp_v10: InstanceFpV10::load(version, f),
            fp_v11: InstanceFpV11::load(version, f),
            fp_v13: InstanceFpV13::load(version, f),
            _ext: unsafe {
                Ext::load_with(f, handle)
            },
        }
    }

    #[inline]
    pub fn handle(&self) -> vk::Instance {
        self.handle
    }

    #[inline]
    pub fn fp_v10(&self) -> &InstanceFpV10 {
        &self.fp_v10
    }

    #[inline]
    pub fn fp_v11(&self) -> &InstanceFpV11 {
        &self.fp_v11
    }

    #[inline]
    pub fn fp_v13(&self) -> &InstanceFpV13 {
        &self.fp_v13
    }

    /// Creates a [`Device'][1].
    ///
    /// # Safety
    /// All raw Vulkan calls are unsafe as there is no validation of input or usage.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDevice.html>
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties.html>
    ///
    /// [1]: crate::Device
    pub unsafe fn create_device<E>(
        &self,
        physical_device: vk::PhysicalDevice,
        create_info: &vk::DeviceCreateInfo<'_>,
        allocator: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<crate::Device<E>>
        where E: LoadWith<Handle = vk::Device>
    {
        unsafe {
            let mut handle = ::core::mem::MaybeUninit::uninit();
            let handle = (self.fp_v10.create_device)(
                physical_device,
                create_info,
                allocator.as_ptr(),
                handle.as_mut_ptr(),
            ).result_with_assume_init(&[vk::Result::SUCCESS], handle)?;
            #[allow(deprecated)]
            let properties = self.get_physical_device_properties(
                physical_device,
            );
            let version = properties.api_version;
            Ok(handle.with_value(crate::Device::load(
                version,
                &self.fp_v10,
                handle.value
            )))
        }
    }

    /// Destroys a [`Device'][1].
    ///
    /// # Safety
    /// All raw Vulkan calls are unsafe as there is no validation of input or usage.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDevice.html>
    ///
    /// [1]: crate::Device
    pub unsafe fn destroy_device<E>(
        &self,
        device: &crate::Device<E>,
        allocator: Option<&vk::AllocationCallbacks>,
    ) {
        unsafe {
            (device.fp_v10().destroy_device)(
                device.handle(),
                allocator.as_ptr(),
            )
        }
    }
}

#[cfg(feature = "ext-load-with")]
impl<Ext> Deref for Instance<Ext> {
    type Target = Ext;
    #[inline]
    fn deref(&self) -> &Ext {
        &self._ext
    }
}

use crate::PtrOption;
use crate::vk::*;
use core::ffi;
#[derive(Clone, Copy)]
pub struct LibraryFpV10 {
    pub create_instance: PFN_vkCreateInstance,
    pub destroy_instance: PFN_vkDestroyInstance,
    pub enumerate_instance_layer_properties: PFN_vkEnumerateInstanceLayerProperties,
    pub enumerate_instance_extension_properties: PFN_vkEnumerateInstanceExtensionProperties,
}
unsafe impl Send for LibraryFpV10 {}
unsafe impl Sync for LibraryFpV10 {}
impl LibraryFpV10 {
    #[allow(unused_variables)]
    pub fn load(version: u32, f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void) -> Self {
        Self {
            create_instance: unsafe {
                unsafe extern "system" fn create_instance(
                    _create_info: *const InstanceCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _instance: *mut crate::vk::Instance,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(create_instance),))
                }
                let f = f(c"vkCreateInstance");
                if f.is_null() {
                    create_instance
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateInstance>(f)
                }
            },
            destroy_instance: unsafe {
                unsafe extern "system" fn destroy_instance(
                    _instance: crate::vk::Instance,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_instance),))
                }
                let f = f(c"vkDestroyInstance");
                if f.is_null() {
                    destroy_instance
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyInstance>(f)
                }
            },
            enumerate_instance_layer_properties: unsafe {
                unsafe extern "system" fn enumerate_instance_layer_properties(
                    _property_count: *mut u32,
                    _properties: *mut LayerProperties,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(enumerate_instance_layer_properties),
                    ))
                }
                let f = f(c"vkEnumerateInstanceLayerProperties");
                if f.is_null() {
                    enumerate_instance_layer_properties
                } else {
                    ::core::mem::transmute::<
                        *const ffi::c_void,
                        PFN_vkEnumerateInstanceLayerProperties,
                    >(f)
                }
            },
            enumerate_instance_extension_properties: unsafe {
                unsafe extern "system" fn enumerate_instance_extension_properties(
                    _layer_name: *const ffi::c_char,
                    _property_count: *mut u32,
                    _properties: *mut ExtensionProperties,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(enumerate_instance_extension_properties),
                    ))
                }
                let f = f(c"vkEnumerateInstanceExtensionProperties");
                if f.is_null() {
                    enumerate_instance_extension_properties
                } else {
                    ::core::mem::transmute::<
                        *const ffi::c_void,
                        PFN_vkEnumerateInstanceExtensionProperties,
                    >(f)
                }
            },
        }
    }
}
impl crate::Library {
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceLayerProperties.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    pub unsafe fn enumerate_instance_layer_properties_len(&self) -> crate::VkResult<u32> {
        unsafe {
            let mut property_count = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.enumerate_instance_layer_properties)(
                property_count.as_mut_ptr(),
                core::ptr::null_mut(),
            )
            .result_with_assume_init(
                &[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE],
                property_count,
            )
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceLayerProperties.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    pub unsafe fn enumerate_instance_layer_properties(
        &self,
        out: &mut [LayerProperties],
    ) -> crate::VkResult<()> {
        unsafe {
            let mut len = out.len() as _;
            (self.fp_v10.enumerate_instance_layer_properties)(&mut len, out.as_mut_ptr())
                .result(&[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    pub unsafe fn enumerate_instance_extension_properties_len(
        &self,
        layer_name: Option<&ffi::CStr>,
    ) -> crate::VkResult<u32> {
        unsafe {
            let mut property_count = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.enumerate_instance_extension_properties)(
                layer_name
                    .map(|layer_name| layer_name.as_ptr())
                    .unwrap_or_default(),
                property_count.as_mut_ptr(),
                core::ptr::null_mut(),
            )
            .result_with_assume_init(
                &[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE],
                property_count,
            )
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    pub unsafe fn enumerate_instance_extension_properties(
        &self,
        layer_name: Option<&ffi::CStr>,
        out: &mut [ExtensionProperties],
    ) -> crate::VkResult<()> {
        unsafe {
            let mut len = out.len() as _;
            (self.fp_v10.enumerate_instance_extension_properties)(
                layer_name
                    .map(|layer_name| layer_name.as_ptr())
                    .unwrap_or_default(),
                &mut len,
                out.as_mut_ptr(),
            )
            .result(&[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE])
        }
    }
}
#[derive(Clone, Copy)]
pub struct LibraryFpV11 {
    pub enumerate_instance_version: PFN_vkEnumerateInstanceVersion,
}
unsafe impl Send for LibraryFpV11 {}
unsafe impl Sync for LibraryFpV11 {}
impl LibraryFpV11 {
    #[allow(unused_variables)]
    pub fn load(version: u32, f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void) -> Self {
        Self {
            enumerate_instance_version: unsafe {
                unsafe extern "system" fn enumerate_instance_version(
                    _api_version: *mut u32,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(enumerate_instance_version),
                    ))
                }
                let f = f(c"vkEnumerateInstanceVersion");
                if f.is_null() {
                    enumerate_instance_version
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkEnumerateInstanceVersion>(f)
                }
            },
        }
    }
}
impl crate::Library {}
#[derive(Clone, Copy)]
pub struct InstanceFpV10 {
    pub destroy_instance: PFN_vkDestroyInstance,
    pub enumerate_physical_devices: PFN_vkEnumeratePhysicalDevices,
    pub get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    pub get_physical_device_properties: PFN_vkGetPhysicalDeviceProperties,
    pub get_physical_device_queue_family_properties: PFN_vkGetPhysicalDeviceQueueFamilyProperties,
    pub get_physical_device_memory_properties: PFN_vkGetPhysicalDeviceMemoryProperties,
    pub get_physical_device_features: PFN_vkGetPhysicalDeviceFeatures,
    pub get_physical_device_format_properties: PFN_vkGetPhysicalDeviceFormatProperties,
    pub get_physical_device_image_format_properties: PFN_vkGetPhysicalDeviceImageFormatProperties,
    pub create_device: PFN_vkCreateDevice,
    pub enumerate_device_layer_properties: PFN_vkEnumerateDeviceLayerProperties,
    pub enumerate_device_extension_properties: PFN_vkEnumerateDeviceExtensionProperties,
    pub get_physical_device_sparse_image_format_properties:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties,
    pub get_device_proc_addr: PFN_vkGetDeviceProcAddr,
}
unsafe impl Send for InstanceFpV10 {}
unsafe impl Sync for InstanceFpV10 {}
impl InstanceFpV10 {
    #[allow(unused_variables)]
    pub fn load(version: u32, f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void) -> Self {
        Self {
            destroy_instance: unsafe {
                unsafe extern "system" fn destroy_instance(
                    _instance: crate::vk::Instance,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_instance),))
                }
                let f = f(c"vkDestroyInstance");
                if f.is_null() {
                    destroy_instance
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyInstance>(f)
                }
            },
            enumerate_physical_devices: unsafe {
                unsafe extern "system" fn enumerate_physical_devices(
                    _instance: crate::vk::Instance,
                    _physical_device_count: *mut u32,
                    _physical_devices: *mut PhysicalDevice,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(enumerate_physical_devices),
                    ))
                }
                let f = f(c"vkEnumeratePhysicalDevices");
                if f.is_null() {
                    enumerate_physical_devices
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkEnumeratePhysicalDevices>(f)
                }
            },
            get_instance_proc_addr: unsafe {
                unsafe extern "system" fn get_instance_proc_addr(
                    _instance: crate::vk::Instance,
                    _name: *const ffi::c_char,
                ) -> PFN_vkVoidFunction {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_instance_proc_addr),
                    ))
                }
                let f = f(c"vkGetInstanceProcAddr");
                if f.is_null() {
                    get_instance_proc_addr
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetInstanceProcAddr>(f)
                }
            },
            get_physical_device_properties: unsafe {
                unsafe extern "system" fn get_physical_device_properties(
                    _physical_device: PhysicalDevice,
                    _properties: *mut PhysicalDeviceProperties,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_physical_device_properties),
                    ))
                }
                let f = f(c"vkGetPhysicalDeviceProperties");
                if f.is_null() {
                    get_physical_device_properties
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetPhysicalDeviceProperties>(
                        f,
                    )
                }
            },
            get_physical_device_queue_family_properties: unsafe {
                unsafe extern "system" fn get_physical_device_queue_family_properties(
                    _physical_device: PhysicalDevice,
                    _queue_family_property_count: *mut u32,
                    _queue_family_properties: *mut QueueFamilyProperties,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_physical_device_queue_family_properties),
                    ))
                }
                let f = f(c"vkGetPhysicalDeviceQueueFamilyProperties");
                if f.is_null() {
                    get_physical_device_queue_family_properties
                } else {
                    ::core::mem::transmute::<
                        *const ffi::c_void,
                        PFN_vkGetPhysicalDeviceQueueFamilyProperties,
                    >(f)
                }
            },
            get_physical_device_memory_properties: unsafe {
                unsafe extern "system" fn get_physical_device_memory_properties(
                    _physical_device: PhysicalDevice,
                    _memory_properties: *mut PhysicalDeviceMemoryProperties,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_physical_device_memory_properties),
                    ))
                }
                let f = f(c"vkGetPhysicalDeviceMemoryProperties");
                if f.is_null() {
                    get_physical_device_memory_properties
                } else {
                    ::core::mem::transmute::<
                        *const ffi::c_void,
                        PFN_vkGetPhysicalDeviceMemoryProperties,
                    >(f)
                }
            },
            get_physical_device_features: unsafe {
                unsafe extern "system" fn get_physical_device_features(
                    _physical_device: PhysicalDevice,
                    _features: *mut PhysicalDeviceFeatures,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_physical_device_features),
                    ))
                }
                let f = f(c"vkGetPhysicalDeviceFeatures");
                if f.is_null() {
                    get_physical_device_features
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetPhysicalDeviceFeatures>(f)
                }
            },
            get_physical_device_format_properties: unsafe {
                unsafe extern "system" fn get_physical_device_format_properties(
                    _physical_device: PhysicalDevice,
                    _format: Format,
                    _format_properties: *mut FormatProperties,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_physical_device_format_properties),
                    ))
                }
                let f = f(c"vkGetPhysicalDeviceFormatProperties");
                if f.is_null() {
                    get_physical_device_format_properties
                } else {
                    ::core::mem::transmute::<
                        *const ffi::c_void,
                        PFN_vkGetPhysicalDeviceFormatProperties,
                    >(f)
                }
            },
            get_physical_device_image_format_properties: unsafe {
                unsafe extern "system" fn get_physical_device_image_format_properties(
                    _physical_device: PhysicalDevice,
                    _format: Format,
                    _ty: ImageType,
                    _tiling: ImageTiling,
                    _usage: ImageUsageFlags,
                    _flags: ImageCreateFlags,
                    _image_format_properties: *mut ImageFormatProperties,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_physical_device_image_format_properties),
                    ))
                }
                let f = f(c"vkGetPhysicalDeviceImageFormatProperties");
                if f.is_null() {
                    get_physical_device_image_format_properties
                } else {
                    ::core::mem::transmute::<
                        *const ffi::c_void,
                        PFN_vkGetPhysicalDeviceImageFormatProperties,
                    >(f)
                }
            },
            create_device: unsafe {
                unsafe extern "system" fn create_device(
                    _physical_device: PhysicalDevice,
                    _create_info: *const DeviceCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _device: *mut crate::vk::Device,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(create_device),))
                }
                let f = f(c"vkCreateDevice");
                if f.is_null() {
                    create_device
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateDevice>(f)
                }
            },
            enumerate_device_layer_properties: unsafe {
                unsafe extern "system" fn enumerate_device_layer_properties(
                    _physical_device: PhysicalDevice,
                    _property_count: *mut u32,
                    _properties: *mut LayerProperties,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(enumerate_device_layer_properties),
                    ))
                }
                let f = f(c"vkEnumerateDeviceLayerProperties");
                if f.is_null() {
                    enumerate_device_layer_properties
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkEnumerateDeviceLayerProperties>(
                        f,
                    )
                }
            },
            enumerate_device_extension_properties: unsafe {
                unsafe extern "system" fn enumerate_device_extension_properties(
                    _physical_device: PhysicalDevice,
                    _layer_name: *const ffi::c_char,
                    _property_count: *mut u32,
                    _properties: *mut ExtensionProperties,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(enumerate_device_extension_properties),
                    ))
                }
                let f = f(c"vkEnumerateDeviceExtensionProperties");
                if f.is_null() {
                    enumerate_device_extension_properties
                } else {
                    ::core::mem::transmute::<
                        *const ffi::c_void,
                        PFN_vkEnumerateDeviceExtensionProperties,
                    >(f)
                }
            },
            get_physical_device_sparse_image_format_properties: unsafe {
                unsafe extern "system" fn get_physical_device_sparse_image_format_properties(
                    _physical_device: PhysicalDevice,
                    _format: Format,
                    _ty: ImageType,
                    _samples: SampleCountFlags,
                    _usage: ImageUsageFlags,
                    _tiling: ImageTiling,
                    _property_count: *mut u32,
                    _properties: *mut SparseImageFormatProperties,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_physical_device_sparse_image_format_properties),
                    ))
                }
                let f = f(c"vkGetPhysicalDeviceSparseImageFormatProperties");
                if f.is_null() {
                    get_physical_device_sparse_image_format_properties
                } else {
                    ::core::mem::transmute::<
                        *const ffi::c_void,
                        PFN_vkGetPhysicalDeviceSparseImageFormatProperties,
                    >(f)
                }
            },
            get_device_proc_addr: unsafe {
                unsafe extern "system" fn get_device_proc_addr(
                    _device: crate::vk::Device,
                    _name: *const ffi::c_char,
                ) -> PFN_vkVoidFunction {
                    panic!(concat!("failed to load ", stringify!(get_device_proc_addr),))
                }
                let f = f(c"vkGetDeviceProcAddr");
                if f.is_null() {
                    get_device_proc_addr
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetDeviceProcAddr>(f)
                }
            },
        }
    }
}
impl<Ext> crate::Instance<Ext> {
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDevices.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    pub unsafe fn enumerate_physical_devices_len(&self) -> crate::VkResult<u32> {
        unsafe {
            let mut physical_device_count = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.enumerate_physical_devices)(
                self.handle,
                physical_device_count.as_mut_ptr(),
                core::ptr::null_mut(),
            )
            .result_with_assume_init(
                &[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE],
                physical_device_count,
            )
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDevices.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    pub unsafe fn enumerate_physical_devices(
        &self,
        out: &mut [PhysicalDevice],
    ) -> crate::VkResult<()> {
        unsafe {
            let mut len = out.len() as _;
            (self.fp_v10.enumerate_physical_devices)(self.handle, &mut len, out.as_mut_ptr())
                .result(&[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE])
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html>"]
    #[doc = r""]
    pub unsafe fn get_instance_proc_addr(&self, name: &ffi::CStr) -> PFN_vkVoidFunction {
        unsafe { (self.fp_v10.get_instance_proc_addr)(self.handle, name.as_ptr()) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkGetPhysicalDeviceProperties2"]
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceProperties {
        unsafe {
            let mut properties = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.get_physical_device_properties)(physical_device, properties.as_mut_ptr());
            properties.assume_init()
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkGetPhysicalDeviceQueueFamilyProperties2"]
    pub unsafe fn get_physical_device_queue_family_properties_len(
        &self,
        physical_device: PhysicalDevice,
    ) -> u32 {
        unsafe {
            let mut queue_family_property_count = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.get_physical_device_queue_family_properties)(
                physical_device,
                queue_family_property_count.as_mut_ptr(),
                core::ptr::null_mut(),
            );
            queue_family_property_count.assume_init()
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkGetPhysicalDeviceQueueFamilyProperties2"]
    pub unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: PhysicalDevice,
        out: &mut [QueueFamilyProperties],
    ) {
        unsafe {
            let mut len = out.len() as _;
            (self.fp_v10.get_physical_device_queue_family_properties)(
                physical_device,
                &mut len,
                out.as_mut_ptr(),
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkGetPhysicalDeviceMemoryProperties2"]
    pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceMemoryProperties {
        unsafe {
            let mut memory_properties = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.get_physical_device_memory_properties)(
                physical_device,
                memory_properties.as_mut_ptr(),
            );
            memory_properties.assume_init()
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkGetPhysicalDeviceFeatures2"]
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceFeatures {
        unsafe {
            let mut features = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.get_physical_device_features)(physical_device, features.as_mut_ptr());
            features.assume_init()
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkGetPhysicalDeviceFormatProperties2"]
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
    ) -> FormatProperties {
        unsafe {
            let mut format_properties = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.get_physical_device_format_properties)(
                physical_device,
                format,
                format_properties.as_mut_ptr(),
            );
            format_properties.assume_init()
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[deprecated = "superseded by vkGetPhysicalDeviceImageFormatProperties2"]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn get_physical_device_image_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
    ) -> crate::VkResult<ImageFormatProperties> {
        unsafe {
            let mut image_format_properties = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.get_physical_device_image_format_properties)(
                physical_device,
                format,
                ty,
                tiling,
                usage,
                flags,
                image_format_properties.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], image_format_properties)
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceLayerProperties.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    #[deprecated]
    pub unsafe fn enumerate_device_layer_properties_len(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::VkResult<u32> {
        unsafe {
            let mut property_count = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.enumerate_device_layer_properties)(
                physical_device,
                property_count.as_mut_ptr(),
                core::ptr::null_mut(),
            )
            .result_with_assume_init(
                &[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE],
                property_count,
            )
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceLayerProperties.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    #[deprecated]
    pub unsafe fn enumerate_device_layer_properties(
        &self,
        physical_device: PhysicalDevice,
        out: &mut [LayerProperties],
    ) -> crate::VkResult<()> {
        unsafe {
            let mut len = out.len() as _;
            (self.fp_v10.enumerate_device_layer_properties)(
                physical_device,
                &mut len,
                out.as_mut_ptr(),
            )
            .result(&[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceExtensionProperties.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    pub unsafe fn enumerate_device_extension_properties_len(
        &self,
        physical_device: PhysicalDevice,
        layer_name: Option<&ffi::CStr>,
    ) -> crate::VkResult<u32> {
        unsafe {
            let mut property_count = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.enumerate_device_extension_properties)(
                physical_device,
                layer_name
                    .map(|layer_name| layer_name.as_ptr())
                    .unwrap_or_default(),
                property_count.as_mut_ptr(),
                core::ptr::null_mut(),
            )
            .result_with_assume_init(
                &[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE],
                property_count,
            )
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceExtensionProperties.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    pub unsafe fn enumerate_device_extension_properties(
        &self,
        physical_device: PhysicalDevice,
        layer_name: Option<&ffi::CStr>,
        out: &mut [ExtensionProperties],
    ) -> crate::VkResult<()> {
        unsafe {
            let mut len = out.len() as _;
            (self.fp_v10.enumerate_device_extension_properties)(
                physical_device,
                layer_name
                    .map(|layer_name| layer_name.as_ptr())
                    .unwrap_or_default(),
                &mut len,
                out.as_mut_ptr(),
            )
            .result(&[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE])
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkGetPhysicalDeviceSparseImageFormatProperties2"]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn get_physical_device_sparse_image_format_properties_len(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
    ) -> u32 {
        unsafe {
            let mut property_count = ::core::mem::MaybeUninit::uninit();
            (self
                .fp_v10
                .get_physical_device_sparse_image_format_properties)(
                physical_device,
                format,
                ty,
                samples,
                usage,
                tiling,
                property_count.as_mut_ptr(),
                core::ptr::null_mut(),
            );
            property_count.assume_init()
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkGetPhysicalDeviceSparseImageFormatProperties2"]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn get_physical_device_sparse_image_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        out: &mut [SparseImageFormatProperties],
    ) {
        unsafe {
            let mut len = out.len() as _;
            (self
                .fp_v10
                .get_physical_device_sparse_image_format_properties)(
                physical_device,
                format,
                ty,
                samples,
                usage,
                tiling,
                &mut len,
                out.as_mut_ptr(),
            )
        }
    }
}
#[derive(Clone, Copy)]
pub struct InstanceFpV11 {
    pub get_physical_device_features2: PFN_vkGetPhysicalDeviceFeatures2,
    pub get_physical_device_properties2: PFN_vkGetPhysicalDeviceProperties2,
    pub get_physical_device_format_properties2: PFN_vkGetPhysicalDeviceFormatProperties2,
    pub get_physical_device_image_format_properties2: PFN_vkGetPhysicalDeviceImageFormatProperties2,
    pub get_physical_device_queue_family_properties2: PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
    pub get_physical_device_memory_properties2: PFN_vkGetPhysicalDeviceMemoryProperties2,
    pub get_physical_device_sparse_image_format_properties2:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
    pub get_physical_device_external_buffer_properties:
        PFN_vkGetPhysicalDeviceExternalBufferProperties,
    pub get_physical_device_external_semaphore_properties:
        PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
    pub get_physical_device_external_fence_properties:
        PFN_vkGetPhysicalDeviceExternalFenceProperties,
    pub enumerate_physical_device_groups: PFN_vkEnumeratePhysicalDeviceGroups,
}
unsafe impl Send for InstanceFpV11 {}
unsafe impl Sync for InstanceFpV11 {}
impl InstanceFpV11 {
    #[allow(unused_variables)]
    pub fn load(version: u32, f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void) -> Self {
        if version >= API_VERSION_1_1 {
            Self {
                get_physical_device_features2: unsafe {
                    unsafe extern "system" fn get_physical_device_features2(
                        _physical_device: PhysicalDevice,
                        _features: *mut PhysicalDeviceFeatures2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_features2),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceFeatures2");
                    if f.is_null() {
                        get_physical_device_features2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetPhysicalDeviceFeatures2>(
                            f,
                        )
                    }
                },
                get_physical_device_properties2: unsafe {
                    unsafe extern "system" fn get_physical_device_properties2(
                        _physical_device: PhysicalDevice,
                        _properties: *mut PhysicalDeviceProperties2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_properties2),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceProperties2");
                    if f.is_null() {
                        get_physical_device_properties2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceProperties2,
                        >(f)
                    }
                },
                get_physical_device_format_properties2: unsafe {
                    unsafe extern "system" fn get_physical_device_format_properties2(
                        _physical_device: PhysicalDevice,
                        _format: Format,
                        _format_properties: *mut FormatProperties2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_format_properties2),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceFormatProperties2");
                    if f.is_null() {
                        get_physical_device_format_properties2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceFormatProperties2,
                        >(f)
                    }
                },
                get_physical_device_image_format_properties2: unsafe {
                    unsafe extern "system" fn get_physical_device_image_format_properties2(
                        _physical_device: PhysicalDevice,
                        _image_format_info: *const PhysicalDeviceImageFormatInfo2,
                        _image_format_properties: *mut ImageFormatProperties2,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_image_format_properties2),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceImageFormatProperties2");
                    if f.is_null() {
                        get_physical_device_image_format_properties2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceImageFormatProperties2,
                        >(f)
                    }
                },
                get_physical_device_queue_family_properties2: unsafe {
                    unsafe extern "system" fn get_physical_device_queue_family_properties2(
                        _physical_device: PhysicalDevice,
                        _queue_family_property_count: *mut u32,
                        _queue_family_properties: *mut QueueFamilyProperties2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_queue_family_properties2),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceQueueFamilyProperties2");
                    if f.is_null() {
                        get_physical_device_queue_family_properties2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
                        >(f)
                    }
                },
                get_physical_device_memory_properties2: unsafe {
                    unsafe extern "system" fn get_physical_device_memory_properties2(
                        _physical_device: PhysicalDevice,
                        _memory_properties: *mut PhysicalDeviceMemoryProperties2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_memory_properties2),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceMemoryProperties2");
                    if f.is_null() {
                        get_physical_device_memory_properties2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceMemoryProperties2,
                        >(f)
                    }
                },
                get_physical_device_sparse_image_format_properties2: unsafe {
                    unsafe extern "system" fn get_physical_device_sparse_image_format_properties2(
                        _physical_device: PhysicalDevice,
                        _format_info: *const PhysicalDeviceSparseImageFormatInfo2,
                        _property_count: *mut u32,
                        _properties: *mut SparseImageFormatProperties2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_sparse_image_format_properties2),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceSparseImageFormatProperties2");
                    if f.is_null() {
                        get_physical_device_sparse_image_format_properties2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
                        >(f)
                    }
                },
                get_physical_device_external_buffer_properties: unsafe {
                    unsafe extern "system" fn get_physical_device_external_buffer_properties(
                        _physical_device: PhysicalDevice,
                        _external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
                        _external_buffer_properties: *mut ExternalBufferProperties,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_external_buffer_properties),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceExternalBufferProperties");
                    if f.is_null() {
                        get_physical_device_external_buffer_properties
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceExternalBufferProperties,
                        >(f)
                    }
                },
                get_physical_device_external_semaphore_properties: unsafe {
                    unsafe extern "system" fn get_physical_device_external_semaphore_properties(
                        _physical_device: PhysicalDevice,
                        _external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
                        _external_semaphore_properties: *mut ExternalSemaphoreProperties,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_external_semaphore_properties),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceExternalSemaphoreProperties");
                    if f.is_null() {
                        get_physical_device_external_semaphore_properties
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
                        >(f)
                    }
                },
                get_physical_device_external_fence_properties: unsafe {
                    unsafe extern "system" fn get_physical_device_external_fence_properties(
                        _physical_device: PhysicalDevice,
                        _external_fence_info: *const PhysicalDeviceExternalFenceInfo,
                        _external_fence_properties: *mut ExternalFenceProperties,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_external_fence_properties),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceExternalFenceProperties");
                    if f.is_null() {
                        get_physical_device_external_fence_properties
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceExternalFenceProperties,
                        >(f)
                    }
                },
                enumerate_physical_device_groups: unsafe {
                    unsafe extern "system" fn enumerate_physical_device_groups(
                        _instance: crate::vk::Instance,
                        _physical_device_group_count: *mut u32,
                        _physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(enumerate_physical_device_groups),
                        ))
                    }
                    let f = f(c"vkEnumeratePhysicalDeviceGroups");
                    if f.is_null() {
                        enumerate_physical_device_groups
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkEnumeratePhysicalDeviceGroups,
                        >(f)
                    }
                },
            }
        } else {
            Self {
                get_physical_device_features2: unsafe {
                    unsafe extern "system" fn get_physical_device_features2(
                        _physical_device: PhysicalDevice,
                        _features: *mut PhysicalDeviceFeatures2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_features2),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceFeatures2KHR");
                    if f.is_null() {
                        get_physical_device_features2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceFeatures2KHR,
                        >(f)
                    }
                },
                get_physical_device_properties2: unsafe {
                    unsafe extern "system" fn get_physical_device_properties2(
                        _physical_device: PhysicalDevice,
                        _properties: *mut PhysicalDeviceProperties2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_properties2),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceProperties2KHR");
                    if f.is_null() {
                        get_physical_device_properties2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceProperties2KHR,
                        >(f)
                    }
                },
                get_physical_device_format_properties2: unsafe {
                    unsafe extern "system" fn get_physical_device_format_properties2(
                        _physical_device: PhysicalDevice,
                        _format: Format,
                        _format_properties: *mut FormatProperties2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_format_properties2),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceFormatProperties2KHR");
                    if f.is_null() {
                        get_physical_device_format_properties2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceFormatProperties2KHR,
                        >(f)
                    }
                },
                get_physical_device_image_format_properties2: unsafe {
                    unsafe extern "system" fn get_physical_device_image_format_properties2(
                        _physical_device: PhysicalDevice,
                        _image_format_info: *const PhysicalDeviceImageFormatInfo2,
                        _image_format_properties: *mut ImageFormatProperties2,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_image_format_properties2),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceImageFormatProperties2KHR");
                    if f.is_null() {
                        get_physical_device_image_format_properties2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceImageFormatProperties2KHR,
                        >(f)
                    }
                },
                get_physical_device_queue_family_properties2: unsafe {
                    unsafe extern "system" fn get_physical_device_queue_family_properties2(
                        _physical_device: PhysicalDevice,
                        _queue_family_property_count: *mut u32,
                        _queue_family_properties: *mut QueueFamilyProperties2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_queue_family_properties2),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceQueueFamilyProperties2KHR");
                    if f.is_null() {
                        get_physical_device_queue_family_properties2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR,
                        >(f)
                    }
                },
                get_physical_device_memory_properties2: unsafe {
                    unsafe extern "system" fn get_physical_device_memory_properties2(
                        _physical_device: PhysicalDevice,
                        _memory_properties: *mut PhysicalDeviceMemoryProperties2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_memory_properties2),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceMemoryProperties2KHR");
                    if f.is_null() {
                        get_physical_device_memory_properties2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceMemoryProperties2KHR,
                        >(f)
                    }
                },
                get_physical_device_sparse_image_format_properties2: unsafe {
                    unsafe extern "system" fn get_physical_device_sparse_image_format_properties2(
                        _physical_device: PhysicalDevice,
                        _format_info: *const PhysicalDeviceSparseImageFormatInfo2,
                        _property_count: *mut u32,
                        _properties: *mut SparseImageFormatProperties2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_sparse_image_format_properties2),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceSparseImageFormatProperties2KHR");
                    if f.is_null() {
                        get_physical_device_sparse_image_format_properties2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR,
                        >(f)
                    }
                },
                get_physical_device_external_buffer_properties: unsafe {
                    unsafe extern "system" fn get_physical_device_external_buffer_properties(
                        _physical_device: PhysicalDevice,
                        _external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
                        _external_buffer_properties: *mut ExternalBufferProperties,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_external_buffer_properties),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceExternalBufferPropertiesKHR");
                    if f.is_null() {
                        get_physical_device_external_buffer_properties
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR,
                        >(f)
                    }
                },
                get_physical_device_external_semaphore_properties: unsafe {
                    unsafe extern "system" fn get_physical_device_external_semaphore_properties(
                        _physical_device: PhysicalDevice,
                        _external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
                        _external_semaphore_properties: *mut ExternalSemaphoreProperties,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_external_semaphore_properties),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR");
                    if f.is_null() {
                        get_physical_device_external_semaphore_properties
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR,
                        >(f)
                    }
                },
                get_physical_device_external_fence_properties: unsafe {
                    unsafe extern "system" fn get_physical_device_external_fence_properties(
                        _physical_device: PhysicalDevice,
                        _external_fence_info: *const PhysicalDeviceExternalFenceInfo,
                        _external_fence_properties: *mut ExternalFenceProperties,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_external_fence_properties),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceExternalFencePropertiesKHR");
                    if f.is_null() {
                        get_physical_device_external_fence_properties
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR,
                        >(f)
                    }
                },
                enumerate_physical_device_groups: unsafe {
                    unsafe extern "system" fn enumerate_physical_device_groups(
                        _instance: crate::vk::Instance,
                        _physical_device_group_count: *mut u32,
                        _physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(enumerate_physical_device_groups),
                        ))
                    }
                    let f = f(c"vkEnumeratePhysicalDeviceGroupsKHR");
                    if f.is_null() {
                        enumerate_physical_device_groups
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkEnumeratePhysicalDeviceGroupsKHR,
                        >(f)
                    }
                },
            }
        }
    }
}
impl<Ext> crate::Instance<Ext> {
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_get_physical_device_properties2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2.html>"]
    #[doc = r""]
    pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: PhysicalDevice,
        features: &mut PhysicalDeviceFeatures2<'_>,
    ) {
        unsafe { (self.fp_v11.get_physical_device_features2)(physical_device, features) }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_get_physical_device_properties2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2.html>"]
    #[doc = r""]
    pub unsafe fn get_physical_device_properties2(
        &self,
        physical_device: PhysicalDevice,
        properties: &mut PhysicalDeviceProperties2<'_>,
    ) {
        unsafe { (self.fp_v11.get_physical_device_properties2)(physical_device, properties) }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_get_physical_device_properties2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2.html>"]
    #[doc = r""]
    pub unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        format_properties: &mut FormatProperties2<'_>,
    ) {
        unsafe {
            (self.fp_v11.get_physical_device_format_properties2)(
                physical_device,
                format,
                format_properties,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_get_physical_device_properties2.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        image_format_info: &PhysicalDeviceImageFormatInfo2<'_>,
        image_format_properties: &mut ImageFormatProperties2<'_>,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v11.get_physical_device_image_format_properties2)(
                physical_device,
                image_format_info,
                image_format_properties,
            )
            .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_get_physical_device_properties2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html>"]
    #[doc = r""]
    pub unsafe fn get_physical_device_queue_family_properties2_len(
        &self,
        physical_device: PhysicalDevice,
    ) -> u32 {
        unsafe {
            let mut queue_family_property_count = ::core::mem::MaybeUninit::uninit();
            (self.fp_v11.get_physical_device_queue_family_properties2)(
                physical_device,
                queue_family_property_count.as_mut_ptr(),
                core::ptr::null_mut(),
            );
            queue_family_property_count.assume_init()
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_get_physical_device_properties2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html>"]
    #[doc = r""]
    pub unsafe fn get_physical_device_queue_family_properties2(
        &self,
        physical_device: PhysicalDevice,
        out: &mut [QueueFamilyProperties2<'_>],
    ) {
        unsafe {
            let mut len = out.len() as _;
            (self.fp_v11.get_physical_device_queue_family_properties2)(
                physical_device,
                &mut len,
                out.as_mut_ptr(),
            )
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_get_physical_device_properties2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2.html>"]
    #[doc = r""]
    pub unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: PhysicalDevice,
        memory_properties: &mut PhysicalDeviceMemoryProperties2<'_>,
    ) {
        unsafe {
            (self.fp_v11.get_physical_device_memory_properties2)(physical_device, memory_properties)
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_get_physical_device_properties2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html>"]
    #[doc = r""]
    pub unsafe fn get_physical_device_sparse_image_format_properties2_len(
        &self,
        physical_device: PhysicalDevice,
        format_info: &PhysicalDeviceSparseImageFormatInfo2<'_>,
    ) -> u32 {
        unsafe {
            let mut property_count = ::core::mem::MaybeUninit::uninit();
            (self
                .fp_v11
                .get_physical_device_sparse_image_format_properties2)(
                physical_device,
                format_info,
                property_count.as_mut_ptr(),
                core::ptr::null_mut(),
            );
            property_count.assume_init()
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_get_physical_device_properties2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html>"]
    #[doc = r""]
    pub unsafe fn get_physical_device_sparse_image_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        format_info: &PhysicalDeviceSparseImageFormatInfo2<'_>,
        out: &mut [SparseImageFormatProperties2<'_>],
    ) {
        unsafe {
            let mut len = out.len() as _;
            (self
                .fp_v11
                .get_physical_device_sparse_image_format_properties2)(
                physical_device,
                format_info,
                &mut len,
                out.as_mut_ptr(),
            )
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_external_memory_capabilities.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferProperties.html>"]
    #[doc = r""]
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: PhysicalDevice,
        external_buffer_info: &PhysicalDeviceExternalBufferInfo<'_>,
        external_buffer_properties: &mut ExternalBufferProperties<'_>,
    ) {
        unsafe {
            (self.fp_v11.get_physical_device_external_buffer_properties)(
                physical_device,
                external_buffer_info,
                external_buffer_properties,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_external_semaphore_capabilities.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphoreProperties.html>"]
    #[doc = r""]
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: PhysicalDevice,
        external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo<'_>,
        external_semaphore_properties: &mut ExternalSemaphoreProperties<'_>,
    ) {
        unsafe {
            (self
                .fp_v11
                .get_physical_device_external_semaphore_properties)(
                physical_device,
                external_semaphore_info,
                external_semaphore_properties,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_external_fence_capabilities.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFenceProperties.html>"]
    #[doc = r""]
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: PhysicalDevice,
        external_fence_info: &PhysicalDeviceExternalFenceInfo<'_>,
        external_fence_properties: &mut ExternalFenceProperties<'_>,
    ) {
        unsafe {
            (self.fp_v11.get_physical_device_external_fence_properties)(
                physical_device,
                external_fence_info,
                external_fence_properties,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_device_group_creation.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    pub unsafe fn enumerate_physical_device_groups_len(&self) -> crate::VkResult<u32> {
        unsafe {
            let mut physical_device_group_count = ::core::mem::MaybeUninit::uninit();
            (self.fp_v11.enumerate_physical_device_groups)(
                self.handle,
                physical_device_group_count.as_mut_ptr(),
                core::ptr::null_mut(),
            )
            .result_with_assume_init(
                &[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE],
                physical_device_group_count,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_device_group_creation.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        out: &mut [PhysicalDeviceGroupProperties<'_>],
    ) -> crate::VkResult<()> {
        unsafe {
            let mut len = out.len() as _;
            (self.fp_v11.enumerate_physical_device_groups)(self.handle, &mut len, out.as_mut_ptr())
                .result(&[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE])
        }
    }
}
#[derive(Clone, Copy)]
pub struct InstanceFpV13 {
    pub get_physical_device_tool_properties: PFN_vkGetPhysicalDeviceToolProperties,
}
unsafe impl Send for InstanceFpV13 {}
unsafe impl Sync for InstanceFpV13 {}
impl InstanceFpV13 {
    #[allow(unused_variables)]
    pub fn load(version: u32, f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void) -> Self {
        if version >= API_VERSION_1_3 {
            Self {
                get_physical_device_tool_properties: unsafe {
                    unsafe extern "system" fn get_physical_device_tool_properties(
                        _physical_device: PhysicalDevice,
                        _tool_count: *mut u32,
                        _tool_properties: *mut PhysicalDeviceToolProperties,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_tool_properties),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceToolProperties");
                    if f.is_null() {
                        get_physical_device_tool_properties
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceToolProperties,
                        >(f)
                    }
                },
            }
        } else {
            Self {
                get_physical_device_tool_properties: unsafe {
                    unsafe extern "system" fn get_physical_device_tool_properties(
                        _physical_device: PhysicalDevice,
                        _tool_count: *mut u32,
                        _tool_properties: *mut PhysicalDeviceToolProperties,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_physical_device_tool_properties),
                        ))
                    }
                    let f = f(c"vkGetPhysicalDeviceToolPropertiesEXT");
                    if f.is_null() {
                        get_physical_device_tool_properties
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetPhysicalDeviceToolPropertiesEXT,
                        >(f)
                    }
                },
            }
        }
    }
}
impl<Ext> crate::Instance<Ext> {
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_tooling_info.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    pub unsafe fn get_physical_device_tool_properties_len(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::VkResult<u32> {
        unsafe {
            let mut tool_count = ::core::mem::MaybeUninit::uninit();
            (self.fp_v13.get_physical_device_tool_properties)(
                physical_device,
                tool_count.as_mut_ptr(),
                core::ptr::null_mut(),
            )
            .result_with_assume_init(
                &[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE],
                tool_count,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_tooling_info.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    pub unsafe fn get_physical_device_tool_properties(
        &self,
        physical_device: PhysicalDevice,
        out: &mut [PhysicalDeviceToolProperties<'_>],
    ) -> crate::VkResult<()> {
        unsafe {
            let mut len = out.len() as _;
            (self.fp_v13.get_physical_device_tool_properties)(
                physical_device,
                &mut len,
                out.as_mut_ptr(),
            )
            .result(&[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE])
        }
    }
}
#[derive(Clone, Copy)]
pub struct DeviceFpV10 {
    pub get_device_proc_addr: PFN_vkGetDeviceProcAddr,
    pub destroy_device: PFN_vkDestroyDevice,
    pub get_device_queue: PFN_vkGetDeviceQueue,
    pub queue_submit: PFN_vkQueueSubmit,
    pub queue_wait_idle: PFN_vkQueueWaitIdle,
    pub device_wait_idle: PFN_vkDeviceWaitIdle,
    pub allocate_memory: PFN_vkAllocateMemory,
    pub free_memory: PFN_vkFreeMemory,
    pub map_memory: PFN_vkMapMemory,
    pub unmap_memory: PFN_vkUnmapMemory,
    pub flush_mapped_memory_ranges: PFN_vkFlushMappedMemoryRanges,
    pub invalidate_mapped_memory_ranges: PFN_vkInvalidateMappedMemoryRanges,
    pub get_device_memory_commitment: PFN_vkGetDeviceMemoryCommitment,
    pub get_buffer_memory_requirements: PFN_vkGetBufferMemoryRequirements,
    pub bind_buffer_memory: PFN_vkBindBufferMemory,
    pub get_image_memory_requirements: PFN_vkGetImageMemoryRequirements,
    pub bind_image_memory: PFN_vkBindImageMemory,
    pub get_image_sparse_memory_requirements: PFN_vkGetImageSparseMemoryRequirements,
    pub queue_bind_sparse: PFN_vkQueueBindSparse,
    pub create_fence: PFN_vkCreateFence,
    pub destroy_fence: PFN_vkDestroyFence,
    pub reset_fences: PFN_vkResetFences,
    pub get_fence_status: PFN_vkGetFenceStatus,
    pub wait_for_fences: PFN_vkWaitForFences,
    pub create_semaphore: PFN_vkCreateSemaphore,
    pub destroy_semaphore: PFN_vkDestroySemaphore,
    pub create_event: PFN_vkCreateEvent,
    pub destroy_event: PFN_vkDestroyEvent,
    pub get_event_status: PFN_vkGetEventStatus,
    pub set_event: PFN_vkSetEvent,
    pub reset_event: PFN_vkResetEvent,
    pub create_query_pool: PFN_vkCreateQueryPool,
    pub destroy_query_pool: PFN_vkDestroyQueryPool,
    pub get_query_pool_results: PFN_vkGetQueryPoolResults,
    pub create_buffer: PFN_vkCreateBuffer,
    pub destroy_buffer: PFN_vkDestroyBuffer,
    pub create_buffer_view: PFN_vkCreateBufferView,
    pub destroy_buffer_view: PFN_vkDestroyBufferView,
    pub create_image: PFN_vkCreateImage,
    pub destroy_image: PFN_vkDestroyImage,
    pub get_image_subresource_layout: PFN_vkGetImageSubresourceLayout,
    pub create_image_view: PFN_vkCreateImageView,
    pub destroy_image_view: PFN_vkDestroyImageView,
    pub create_shader_module: PFN_vkCreateShaderModule,
    pub destroy_shader_module: PFN_vkDestroyShaderModule,
    pub create_pipeline_cache: PFN_vkCreatePipelineCache,
    pub destroy_pipeline_cache: PFN_vkDestroyPipelineCache,
    pub get_pipeline_cache_data: PFN_vkGetPipelineCacheData,
    pub merge_pipeline_caches: PFN_vkMergePipelineCaches,
    pub create_graphics_pipelines: PFN_vkCreateGraphicsPipelines,
    pub create_compute_pipelines: PFN_vkCreateComputePipelines,
    pub destroy_pipeline: PFN_vkDestroyPipeline,
    pub create_pipeline_layout: PFN_vkCreatePipelineLayout,
    pub destroy_pipeline_layout: PFN_vkDestroyPipelineLayout,
    pub create_sampler: PFN_vkCreateSampler,
    pub destroy_sampler: PFN_vkDestroySampler,
    pub create_descriptor_set_layout: PFN_vkCreateDescriptorSetLayout,
    pub destroy_descriptor_set_layout: PFN_vkDestroyDescriptorSetLayout,
    pub create_descriptor_pool: PFN_vkCreateDescriptorPool,
    pub destroy_descriptor_pool: PFN_vkDestroyDescriptorPool,
    pub reset_descriptor_pool: PFN_vkResetDescriptorPool,
    pub allocate_descriptor_sets: PFN_vkAllocateDescriptorSets,
    pub free_descriptor_sets: PFN_vkFreeDescriptorSets,
    pub update_descriptor_sets: PFN_vkUpdateDescriptorSets,
    pub create_framebuffer: PFN_vkCreateFramebuffer,
    pub destroy_framebuffer: PFN_vkDestroyFramebuffer,
    pub create_render_pass: PFN_vkCreateRenderPass,
    pub destroy_render_pass: PFN_vkDestroyRenderPass,
    pub get_render_area_granularity: PFN_vkGetRenderAreaGranularity,
    pub create_command_pool: PFN_vkCreateCommandPool,
    pub destroy_command_pool: PFN_vkDestroyCommandPool,
    pub reset_command_pool: PFN_vkResetCommandPool,
    pub allocate_command_buffers: PFN_vkAllocateCommandBuffers,
    pub free_command_buffers: PFN_vkFreeCommandBuffers,
    pub begin_command_buffer: PFN_vkBeginCommandBuffer,
    pub end_command_buffer: PFN_vkEndCommandBuffer,
    pub reset_command_buffer: PFN_vkResetCommandBuffer,
    pub cmd_bind_pipeline: PFN_vkCmdBindPipeline,
    pub cmd_set_viewport: PFN_vkCmdSetViewport,
    pub cmd_set_scissor: PFN_vkCmdSetScissor,
    pub cmd_set_line_width: PFN_vkCmdSetLineWidth,
    pub cmd_set_depth_bias: PFN_vkCmdSetDepthBias,
    pub cmd_set_blend_constants: PFN_vkCmdSetBlendConstants,
    pub cmd_set_depth_bounds: PFN_vkCmdSetDepthBounds,
    pub cmd_set_stencil_compare_mask: PFN_vkCmdSetStencilCompareMask,
    pub cmd_set_stencil_write_mask: PFN_vkCmdSetStencilWriteMask,
    pub cmd_set_stencil_reference: PFN_vkCmdSetStencilReference,
    pub cmd_bind_descriptor_sets: PFN_vkCmdBindDescriptorSets,
    pub cmd_bind_index_buffer: PFN_vkCmdBindIndexBuffer,
    pub cmd_bind_vertex_buffers: PFN_vkCmdBindVertexBuffers,
    pub cmd_draw: PFN_vkCmdDraw,
    pub cmd_draw_indexed: PFN_vkCmdDrawIndexed,
    pub cmd_draw_indirect: PFN_vkCmdDrawIndirect,
    pub cmd_draw_indexed_indirect: PFN_vkCmdDrawIndexedIndirect,
    pub cmd_dispatch: PFN_vkCmdDispatch,
    pub cmd_dispatch_indirect: PFN_vkCmdDispatchIndirect,
    pub cmd_copy_buffer: PFN_vkCmdCopyBuffer,
    pub cmd_copy_image: PFN_vkCmdCopyImage,
    pub cmd_blit_image: PFN_vkCmdBlitImage,
    pub cmd_copy_buffer_to_image: PFN_vkCmdCopyBufferToImage,
    pub cmd_copy_image_to_buffer: PFN_vkCmdCopyImageToBuffer,
    pub cmd_update_buffer: PFN_vkCmdUpdateBuffer,
    pub cmd_fill_buffer: PFN_vkCmdFillBuffer,
    pub cmd_clear_color_image: PFN_vkCmdClearColorImage,
    pub cmd_clear_depth_stencil_image: PFN_vkCmdClearDepthStencilImage,
    pub cmd_clear_attachments: PFN_vkCmdClearAttachments,
    pub cmd_resolve_image: PFN_vkCmdResolveImage,
    pub cmd_set_event: PFN_vkCmdSetEvent,
    pub cmd_reset_event: PFN_vkCmdResetEvent,
    pub cmd_wait_events: PFN_vkCmdWaitEvents,
    pub cmd_pipeline_barrier: PFN_vkCmdPipelineBarrier,
    pub cmd_begin_query: PFN_vkCmdBeginQuery,
    pub cmd_end_query: PFN_vkCmdEndQuery,
    pub cmd_reset_query_pool: PFN_vkCmdResetQueryPool,
    pub cmd_write_timestamp: PFN_vkCmdWriteTimestamp,
    pub cmd_copy_query_pool_results: PFN_vkCmdCopyQueryPoolResults,
    pub cmd_push_constants: PFN_vkCmdPushConstants,
    pub cmd_begin_render_pass: PFN_vkCmdBeginRenderPass,
    pub cmd_next_subpass: PFN_vkCmdNextSubpass,
    pub cmd_end_render_pass: PFN_vkCmdEndRenderPass,
    pub cmd_execute_commands: PFN_vkCmdExecuteCommands,
}
unsafe impl Send for DeviceFpV10 {}
unsafe impl Sync for DeviceFpV10 {}
impl DeviceFpV10 {
    #[allow(unused_variables)]
    pub fn load(version: u32, f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void) -> Self {
        Self {
            get_device_proc_addr: unsafe {
                unsafe extern "system" fn get_device_proc_addr(
                    _device: crate::vk::Device,
                    _name: *const ffi::c_char,
                ) -> PFN_vkVoidFunction {
                    panic!(concat!("failed to load ", stringify!(get_device_proc_addr),))
                }
                let f = f(c"vkGetDeviceProcAddr");
                if f.is_null() {
                    get_device_proc_addr
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetDeviceProcAddr>(f)
                }
            },
            destroy_device: unsafe {
                unsafe extern "system" fn destroy_device(
                    _device: crate::vk::Device,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_device),))
                }
                let f = f(c"vkDestroyDevice");
                if f.is_null() {
                    destroy_device
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyDevice>(f)
                }
            },
            get_device_queue: unsafe {
                unsafe extern "system" fn get_device_queue(
                    _device: crate::vk::Device,
                    _queue_family_index: u32,
                    _queue_index: u32,
                    _queue: *mut Queue,
                ) {
                    panic!(concat!("failed to load ", stringify!(get_device_queue),))
                }
                let f = f(c"vkGetDeviceQueue");
                if f.is_null() {
                    get_device_queue
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetDeviceQueue>(f)
                }
            },
            queue_submit: unsafe {
                unsafe extern "system" fn queue_submit(
                    _queue: Queue,
                    _submit_count: u32,
                    _submits: *const SubmitInfo,
                    _fence: Fence,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(queue_submit),))
                }
                let f = f(c"vkQueueSubmit");
                if f.is_null() {
                    queue_submit
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkQueueSubmit>(f)
                }
            },
            queue_wait_idle: unsafe {
                unsafe extern "system" fn queue_wait_idle(_queue: Queue) -> Result {
                    panic!(concat!("failed to load ", stringify!(queue_wait_idle),))
                }
                let f = f(c"vkQueueWaitIdle");
                if f.is_null() {
                    queue_wait_idle
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkQueueWaitIdle>(f)
                }
            },
            device_wait_idle: unsafe {
                unsafe extern "system" fn device_wait_idle(_device: crate::vk::Device) -> Result {
                    panic!(concat!("failed to load ", stringify!(device_wait_idle),))
                }
                let f = f(c"vkDeviceWaitIdle");
                if f.is_null() {
                    device_wait_idle
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDeviceWaitIdle>(f)
                }
            },
            allocate_memory: unsafe {
                unsafe extern "system" fn allocate_memory(
                    _device: crate::vk::Device,
                    _allocate_info: *const MemoryAllocateInfo,
                    _allocator: *const AllocationCallbacks,
                    _memory: *mut DeviceMemory,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(allocate_memory),))
                }
                let f = f(c"vkAllocateMemory");
                if f.is_null() {
                    allocate_memory
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkAllocateMemory>(f)
                }
            },
            free_memory: unsafe {
                unsafe extern "system" fn free_memory(
                    _device: crate::vk::Device,
                    _memory: DeviceMemory,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(free_memory),))
                }
                let f = f(c"vkFreeMemory");
                if f.is_null() {
                    free_memory
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkFreeMemory>(f)
                }
            },
            map_memory: unsafe {
                unsafe extern "system" fn map_memory(
                    _device: crate::vk::Device,
                    _memory: DeviceMemory,
                    _offset: DeviceSize,
                    _size: DeviceSize,
                    _flags: MemoryMapFlags,
                    _pp_data: *mut *mut ffi::c_void,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(map_memory),))
                }
                let f = f(c"vkMapMemory");
                if f.is_null() {
                    map_memory
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkMapMemory>(f)
                }
            },
            unmap_memory: unsafe {
                unsafe extern "system" fn unmap_memory(
                    _device: crate::vk::Device,
                    _memory: DeviceMemory,
                ) {
                    panic!(concat!("failed to load ", stringify!(unmap_memory),))
                }
                let f = f(c"vkUnmapMemory");
                if f.is_null() {
                    unmap_memory
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkUnmapMemory>(f)
                }
            },
            flush_mapped_memory_ranges: unsafe {
                unsafe extern "system" fn flush_mapped_memory_ranges(
                    _device: crate::vk::Device,
                    _memory_range_count: u32,
                    _memory_ranges: *const MappedMemoryRange,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(flush_mapped_memory_ranges),
                    ))
                }
                let f = f(c"vkFlushMappedMemoryRanges");
                if f.is_null() {
                    flush_mapped_memory_ranges
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkFlushMappedMemoryRanges>(f)
                }
            },
            invalidate_mapped_memory_ranges: unsafe {
                unsafe extern "system" fn invalidate_mapped_memory_ranges(
                    _device: crate::vk::Device,
                    _memory_range_count: u32,
                    _memory_ranges: *const MappedMemoryRange,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(invalidate_mapped_memory_ranges),
                    ))
                }
                let f = f(c"vkInvalidateMappedMemoryRanges");
                if f.is_null() {
                    invalidate_mapped_memory_ranges
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkInvalidateMappedMemoryRanges>(
                        f,
                    )
                }
            },
            get_device_memory_commitment: unsafe {
                unsafe extern "system" fn get_device_memory_commitment(
                    _device: crate::vk::Device,
                    _memory: DeviceMemory,
                    _committed_memory_in_bytes: *mut DeviceSize,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_device_memory_commitment),
                    ))
                }
                let f = f(c"vkGetDeviceMemoryCommitment");
                if f.is_null() {
                    get_device_memory_commitment
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetDeviceMemoryCommitment>(f)
                }
            },
            get_buffer_memory_requirements: unsafe {
                unsafe extern "system" fn get_buffer_memory_requirements(
                    _device: crate::vk::Device,
                    _buffer: Buffer,
                    _memory_requirements: *mut MemoryRequirements,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_buffer_memory_requirements),
                    ))
                }
                let f = f(c"vkGetBufferMemoryRequirements");
                if f.is_null() {
                    get_buffer_memory_requirements
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetBufferMemoryRequirements>(
                        f,
                    )
                }
            },
            bind_buffer_memory: unsafe {
                unsafe extern "system" fn bind_buffer_memory(
                    _device: crate::vk::Device,
                    _buffer: Buffer,
                    _memory: DeviceMemory,
                    _memory_offset: DeviceSize,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(bind_buffer_memory),))
                }
                let f = f(c"vkBindBufferMemory");
                if f.is_null() {
                    bind_buffer_memory
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkBindBufferMemory>(f)
                }
            },
            get_image_memory_requirements: unsafe {
                unsafe extern "system" fn get_image_memory_requirements(
                    _device: crate::vk::Device,
                    _image: Image,
                    _memory_requirements: *mut MemoryRequirements,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_image_memory_requirements),
                    ))
                }
                let f = f(c"vkGetImageMemoryRequirements");
                if f.is_null() {
                    get_image_memory_requirements
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetImageMemoryRequirements>(
                        f,
                    )
                }
            },
            bind_image_memory: unsafe {
                unsafe extern "system" fn bind_image_memory(
                    _device: crate::vk::Device,
                    _image: Image,
                    _memory: DeviceMemory,
                    _memory_offset: DeviceSize,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(bind_image_memory),))
                }
                let f = f(c"vkBindImageMemory");
                if f.is_null() {
                    bind_image_memory
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkBindImageMemory>(f)
                }
            },
            get_image_sparse_memory_requirements: unsafe {
                unsafe extern "system" fn get_image_sparse_memory_requirements(
                    _device: crate::vk::Device,
                    _image: Image,
                    _sparse_memory_requirement_count: *mut u32,
                    _sparse_memory_requirements: *mut SparseImageMemoryRequirements,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_image_sparse_memory_requirements),
                    ))
                }
                let f = f(c"vkGetImageSparseMemoryRequirements");
                if f.is_null() {
                    get_image_sparse_memory_requirements
                } else {
                    ::core::mem::transmute::<
                        *const ffi::c_void,
                        PFN_vkGetImageSparseMemoryRequirements,
                    >(f)
                }
            },
            queue_bind_sparse: unsafe {
                unsafe extern "system" fn queue_bind_sparse(
                    _queue: Queue,
                    _bind_info_count: u32,
                    _bind_info: *const BindSparseInfo,
                    _fence: Fence,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(queue_bind_sparse),))
                }
                let f = f(c"vkQueueBindSparse");
                if f.is_null() {
                    queue_bind_sparse
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkQueueBindSparse>(f)
                }
            },
            create_fence: unsafe {
                unsafe extern "system" fn create_fence(
                    _device: crate::vk::Device,
                    _create_info: *const FenceCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _fence: *mut Fence,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(create_fence),))
                }
                let f = f(c"vkCreateFence");
                if f.is_null() {
                    create_fence
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateFence>(f)
                }
            },
            destroy_fence: unsafe {
                unsafe extern "system" fn destroy_fence(
                    _device: crate::vk::Device,
                    _fence: Fence,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_fence),))
                }
                let f = f(c"vkDestroyFence");
                if f.is_null() {
                    destroy_fence
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyFence>(f)
                }
            },
            reset_fences: unsafe {
                unsafe extern "system" fn reset_fences(
                    _device: crate::vk::Device,
                    _fence_count: u32,
                    _fences: *const Fence,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(reset_fences),))
                }
                let f = f(c"vkResetFences");
                if f.is_null() {
                    reset_fences
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkResetFences>(f)
                }
            },
            get_fence_status: unsafe {
                unsafe extern "system" fn get_fence_status(
                    _device: crate::vk::Device,
                    _fence: Fence,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(get_fence_status),))
                }
                let f = f(c"vkGetFenceStatus");
                if f.is_null() {
                    get_fence_status
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetFenceStatus>(f)
                }
            },
            wait_for_fences: unsafe {
                unsafe extern "system" fn wait_for_fences(
                    _device: crate::vk::Device,
                    _fence_count: u32,
                    _fences: *const Fence,
                    _wait_all: Bool32,
                    _timeout: u64,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(wait_for_fences),))
                }
                let f = f(c"vkWaitForFences");
                if f.is_null() {
                    wait_for_fences
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkWaitForFences>(f)
                }
            },
            create_semaphore: unsafe {
                unsafe extern "system" fn create_semaphore(
                    _device: crate::vk::Device,
                    _create_info: *const SemaphoreCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _semaphore: *mut Semaphore,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(create_semaphore),))
                }
                let f = f(c"vkCreateSemaphore");
                if f.is_null() {
                    create_semaphore
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateSemaphore>(f)
                }
            },
            destroy_semaphore: unsafe {
                unsafe extern "system" fn destroy_semaphore(
                    _device: crate::vk::Device,
                    _semaphore: Semaphore,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_semaphore),))
                }
                let f = f(c"vkDestroySemaphore");
                if f.is_null() {
                    destroy_semaphore
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroySemaphore>(f)
                }
            },
            create_event: unsafe {
                unsafe extern "system" fn create_event(
                    _device: crate::vk::Device,
                    _create_info: *const EventCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _event: *mut Event,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(create_event),))
                }
                let f = f(c"vkCreateEvent");
                if f.is_null() {
                    create_event
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateEvent>(f)
                }
            },
            destroy_event: unsafe {
                unsafe extern "system" fn destroy_event(
                    _device: crate::vk::Device,
                    _event: Event,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_event),))
                }
                let f = f(c"vkDestroyEvent");
                if f.is_null() {
                    destroy_event
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyEvent>(f)
                }
            },
            get_event_status: unsafe {
                unsafe extern "system" fn get_event_status(
                    _device: crate::vk::Device,
                    _event: Event,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(get_event_status),))
                }
                let f = f(c"vkGetEventStatus");
                if f.is_null() {
                    get_event_status
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetEventStatus>(f)
                }
            },
            set_event: unsafe {
                unsafe extern "system" fn set_event(
                    _device: crate::vk::Device,
                    _event: Event,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(set_event),))
                }
                let f = f(c"vkSetEvent");
                if f.is_null() {
                    set_event
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkSetEvent>(f)
                }
            },
            reset_event: unsafe {
                unsafe extern "system" fn reset_event(
                    _device: crate::vk::Device,
                    _event: Event,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(reset_event),))
                }
                let f = f(c"vkResetEvent");
                if f.is_null() {
                    reset_event
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkResetEvent>(f)
                }
            },
            create_query_pool: unsafe {
                unsafe extern "system" fn create_query_pool(
                    _device: crate::vk::Device,
                    _create_info: *const QueryPoolCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _query_pool: *mut QueryPool,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(create_query_pool),))
                }
                let f = f(c"vkCreateQueryPool");
                if f.is_null() {
                    create_query_pool
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateQueryPool>(f)
                }
            },
            destroy_query_pool: unsafe {
                unsafe extern "system" fn destroy_query_pool(
                    _device: crate::vk::Device,
                    _query_pool: QueryPool,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_query_pool),))
                }
                let f = f(c"vkDestroyQueryPool");
                if f.is_null() {
                    destroy_query_pool
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyQueryPool>(f)
                }
            },
            get_query_pool_results: unsafe {
                unsafe extern "system" fn get_query_pool_results(
                    _device: crate::vk::Device,
                    _query_pool: QueryPool,
                    _first_query: u32,
                    _query_count: u32,
                    _data_size: usize,
                    _data: *mut ffi::c_void,
                    _stride: DeviceSize,
                    _flags: QueryResultFlags,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_query_pool_results),
                    ))
                }
                let f = f(c"vkGetQueryPoolResults");
                if f.is_null() {
                    get_query_pool_results
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetQueryPoolResults>(f)
                }
            },
            create_buffer: unsafe {
                unsafe extern "system" fn create_buffer(
                    _device: crate::vk::Device,
                    _create_info: *const BufferCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _buffer: *mut Buffer,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(create_buffer),))
                }
                let f = f(c"vkCreateBuffer");
                if f.is_null() {
                    create_buffer
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateBuffer>(f)
                }
            },
            destroy_buffer: unsafe {
                unsafe extern "system" fn destroy_buffer(
                    _device: crate::vk::Device,
                    _buffer: Buffer,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_buffer),))
                }
                let f = f(c"vkDestroyBuffer");
                if f.is_null() {
                    destroy_buffer
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyBuffer>(f)
                }
            },
            create_buffer_view: unsafe {
                unsafe extern "system" fn create_buffer_view(
                    _device: crate::vk::Device,
                    _create_info: *const BufferViewCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _view: *mut BufferView,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(create_buffer_view),))
                }
                let f = f(c"vkCreateBufferView");
                if f.is_null() {
                    create_buffer_view
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateBufferView>(f)
                }
            },
            destroy_buffer_view: unsafe {
                unsafe extern "system" fn destroy_buffer_view(
                    _device: crate::vk::Device,
                    _buffer_view: BufferView,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_buffer_view),))
                }
                let f = f(c"vkDestroyBufferView");
                if f.is_null() {
                    destroy_buffer_view
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyBufferView>(f)
                }
            },
            create_image: unsafe {
                unsafe extern "system" fn create_image(
                    _device: crate::vk::Device,
                    _create_info: *const ImageCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _image: *mut Image,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(create_image),))
                }
                let f = f(c"vkCreateImage");
                if f.is_null() {
                    create_image
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateImage>(f)
                }
            },
            destroy_image: unsafe {
                unsafe extern "system" fn destroy_image(
                    _device: crate::vk::Device,
                    _image: Image,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_image),))
                }
                let f = f(c"vkDestroyImage");
                if f.is_null() {
                    destroy_image
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyImage>(f)
                }
            },
            get_image_subresource_layout: unsafe {
                unsafe extern "system" fn get_image_subresource_layout(
                    _device: crate::vk::Device,
                    _image: Image,
                    _subresource: *const ImageSubresource,
                    _layout: *mut SubresourceLayout,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_image_subresource_layout),
                    ))
                }
                let f = f(c"vkGetImageSubresourceLayout");
                if f.is_null() {
                    get_image_subresource_layout
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetImageSubresourceLayout>(f)
                }
            },
            create_image_view: unsafe {
                unsafe extern "system" fn create_image_view(
                    _device: crate::vk::Device,
                    _create_info: *const ImageViewCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _view: *mut ImageView,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(create_image_view),))
                }
                let f = f(c"vkCreateImageView");
                if f.is_null() {
                    create_image_view
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateImageView>(f)
                }
            },
            destroy_image_view: unsafe {
                unsafe extern "system" fn destroy_image_view(
                    _device: crate::vk::Device,
                    _image_view: ImageView,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_image_view),))
                }
                let f = f(c"vkDestroyImageView");
                if f.is_null() {
                    destroy_image_view
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyImageView>(f)
                }
            },
            create_shader_module: unsafe {
                unsafe extern "system" fn create_shader_module(
                    _device: crate::vk::Device,
                    _create_info: *const ShaderModuleCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _shader_module: *mut ShaderModule,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(create_shader_module),))
                }
                let f = f(c"vkCreateShaderModule");
                if f.is_null() {
                    create_shader_module
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateShaderModule>(f)
                }
            },
            destroy_shader_module: unsafe {
                unsafe extern "system" fn destroy_shader_module(
                    _device: crate::vk::Device,
                    _shader_module: ShaderModule,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(destroy_shader_module),
                    ))
                }
                let f = f(c"vkDestroyShaderModule");
                if f.is_null() {
                    destroy_shader_module
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyShaderModule>(f)
                }
            },
            create_pipeline_cache: unsafe {
                unsafe extern "system" fn create_pipeline_cache(
                    _device: crate::vk::Device,
                    _create_info: *const PipelineCacheCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _pipeline_cache: *mut PipelineCache,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(create_pipeline_cache),
                    ))
                }
                let f = f(c"vkCreatePipelineCache");
                if f.is_null() {
                    create_pipeline_cache
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreatePipelineCache>(f)
                }
            },
            destroy_pipeline_cache: unsafe {
                unsafe extern "system" fn destroy_pipeline_cache(
                    _device: crate::vk::Device,
                    _pipeline_cache: PipelineCache,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(destroy_pipeline_cache),
                    ))
                }
                let f = f(c"vkDestroyPipelineCache");
                if f.is_null() {
                    destroy_pipeline_cache
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyPipelineCache>(f)
                }
            },
            get_pipeline_cache_data: unsafe {
                unsafe extern "system" fn get_pipeline_cache_data(
                    _device: crate::vk::Device,
                    _pipeline_cache: PipelineCache,
                    _data_size: *mut usize,
                    _data: *mut ffi::c_void,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_pipeline_cache_data),
                    ))
                }
                let f = f(c"vkGetPipelineCacheData");
                if f.is_null() {
                    get_pipeline_cache_data
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetPipelineCacheData>(f)
                }
            },
            merge_pipeline_caches: unsafe {
                unsafe extern "system" fn merge_pipeline_caches(
                    _device: crate::vk::Device,
                    _dst_cache: PipelineCache,
                    _src_cache_count: u32,
                    _src_caches: *const PipelineCache,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(merge_pipeline_caches),
                    ))
                }
                let f = f(c"vkMergePipelineCaches");
                if f.is_null() {
                    merge_pipeline_caches
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkMergePipelineCaches>(f)
                }
            },
            create_graphics_pipelines: unsafe {
                unsafe extern "system" fn create_graphics_pipelines(
                    _device: crate::vk::Device,
                    _pipeline_cache: PipelineCache,
                    _create_info_count: u32,
                    _create_infos: *const GraphicsPipelineCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _pipelines: *mut Pipeline,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(create_graphics_pipelines),
                    ))
                }
                let f = f(c"vkCreateGraphicsPipelines");
                if f.is_null() {
                    create_graphics_pipelines
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateGraphicsPipelines>(f)
                }
            },
            create_compute_pipelines: unsafe {
                unsafe extern "system" fn create_compute_pipelines(
                    _device: crate::vk::Device,
                    _pipeline_cache: PipelineCache,
                    _create_info_count: u32,
                    _create_infos: *const ComputePipelineCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _pipelines: *mut Pipeline,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(create_compute_pipelines),
                    ))
                }
                let f = f(c"vkCreateComputePipelines");
                if f.is_null() {
                    create_compute_pipelines
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateComputePipelines>(f)
                }
            },
            destroy_pipeline: unsafe {
                unsafe extern "system" fn destroy_pipeline(
                    _device: crate::vk::Device,
                    _pipeline: Pipeline,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_pipeline),))
                }
                let f = f(c"vkDestroyPipeline");
                if f.is_null() {
                    destroy_pipeline
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyPipeline>(f)
                }
            },
            create_pipeline_layout: unsafe {
                unsafe extern "system" fn create_pipeline_layout(
                    _device: crate::vk::Device,
                    _create_info: *const PipelineLayoutCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _pipeline_layout: *mut PipelineLayout,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(create_pipeline_layout),
                    ))
                }
                let f = f(c"vkCreatePipelineLayout");
                if f.is_null() {
                    create_pipeline_layout
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreatePipelineLayout>(f)
                }
            },
            destroy_pipeline_layout: unsafe {
                unsafe extern "system" fn destroy_pipeline_layout(
                    _device: crate::vk::Device,
                    _pipeline_layout: PipelineLayout,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(destroy_pipeline_layout),
                    ))
                }
                let f = f(c"vkDestroyPipelineLayout");
                if f.is_null() {
                    destroy_pipeline_layout
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyPipelineLayout>(f)
                }
            },
            create_sampler: unsafe {
                unsafe extern "system" fn create_sampler(
                    _device: crate::vk::Device,
                    _create_info: *const SamplerCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _sampler: *mut Sampler,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(create_sampler),))
                }
                let f = f(c"vkCreateSampler");
                if f.is_null() {
                    create_sampler
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateSampler>(f)
                }
            },
            destroy_sampler: unsafe {
                unsafe extern "system" fn destroy_sampler(
                    _device: crate::vk::Device,
                    _sampler: Sampler,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_sampler),))
                }
                let f = f(c"vkDestroySampler");
                if f.is_null() {
                    destroy_sampler
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroySampler>(f)
                }
            },
            create_descriptor_set_layout: unsafe {
                unsafe extern "system" fn create_descriptor_set_layout(
                    _device: crate::vk::Device,
                    _create_info: *const DescriptorSetLayoutCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _set_layout: *mut DescriptorSetLayout,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(create_descriptor_set_layout),
                    ))
                }
                let f = f(c"vkCreateDescriptorSetLayout");
                if f.is_null() {
                    create_descriptor_set_layout
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateDescriptorSetLayout>(f)
                }
            },
            destroy_descriptor_set_layout: unsafe {
                unsafe extern "system" fn destroy_descriptor_set_layout(
                    _device: crate::vk::Device,
                    _descriptor_set_layout: DescriptorSetLayout,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(destroy_descriptor_set_layout),
                    ))
                }
                let f = f(c"vkDestroyDescriptorSetLayout");
                if f.is_null() {
                    destroy_descriptor_set_layout
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyDescriptorSetLayout>(
                        f,
                    )
                }
            },
            create_descriptor_pool: unsafe {
                unsafe extern "system" fn create_descriptor_pool(
                    _device: crate::vk::Device,
                    _create_info: *const DescriptorPoolCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _descriptor_pool: *mut DescriptorPool,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(create_descriptor_pool),
                    ))
                }
                let f = f(c"vkCreateDescriptorPool");
                if f.is_null() {
                    create_descriptor_pool
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateDescriptorPool>(f)
                }
            },
            destroy_descriptor_pool: unsafe {
                unsafe extern "system" fn destroy_descriptor_pool(
                    _device: crate::vk::Device,
                    _descriptor_pool: DescriptorPool,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(destroy_descriptor_pool),
                    ))
                }
                let f = f(c"vkDestroyDescriptorPool");
                if f.is_null() {
                    destroy_descriptor_pool
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyDescriptorPool>(f)
                }
            },
            reset_descriptor_pool: unsafe {
                unsafe extern "system" fn reset_descriptor_pool(
                    _device: crate::vk::Device,
                    _descriptor_pool: DescriptorPool,
                    _flags: DescriptorPoolResetFlags,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(reset_descriptor_pool),
                    ))
                }
                let f = f(c"vkResetDescriptorPool");
                if f.is_null() {
                    reset_descriptor_pool
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkResetDescriptorPool>(f)
                }
            },
            allocate_descriptor_sets: unsafe {
                unsafe extern "system" fn allocate_descriptor_sets(
                    _device: crate::vk::Device,
                    _allocate_info: *const DescriptorSetAllocateInfo,
                    _descriptor_sets: *mut DescriptorSet,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(allocate_descriptor_sets),
                    ))
                }
                let f = f(c"vkAllocateDescriptorSets");
                if f.is_null() {
                    allocate_descriptor_sets
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkAllocateDescriptorSets>(f)
                }
            },
            free_descriptor_sets: unsafe {
                unsafe extern "system" fn free_descriptor_sets(
                    _device: crate::vk::Device,
                    _descriptor_pool: DescriptorPool,
                    _descriptor_set_count: u32,
                    _descriptor_sets: *const DescriptorSet,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(free_descriptor_sets),))
                }
                let f = f(c"vkFreeDescriptorSets");
                if f.is_null() {
                    free_descriptor_sets
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkFreeDescriptorSets>(f)
                }
            },
            update_descriptor_sets: unsafe {
                unsafe extern "system" fn update_descriptor_sets(
                    _device: crate::vk::Device,
                    _descriptor_write_count: u32,
                    _descriptor_writes: *const WriteDescriptorSet,
                    _descriptor_copy_count: u32,
                    _descriptor_copies: *const CopyDescriptorSet,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(update_descriptor_sets),
                    ))
                }
                let f = f(c"vkUpdateDescriptorSets");
                if f.is_null() {
                    update_descriptor_sets
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkUpdateDescriptorSets>(f)
                }
            },
            create_framebuffer: unsafe {
                unsafe extern "system" fn create_framebuffer(
                    _device: crate::vk::Device,
                    _create_info: *const FramebufferCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _framebuffer: *mut Framebuffer,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(create_framebuffer),))
                }
                let f = f(c"vkCreateFramebuffer");
                if f.is_null() {
                    create_framebuffer
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateFramebuffer>(f)
                }
            },
            destroy_framebuffer: unsafe {
                unsafe extern "system" fn destroy_framebuffer(
                    _device: crate::vk::Device,
                    _framebuffer: Framebuffer,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_framebuffer),))
                }
                let f = f(c"vkDestroyFramebuffer");
                if f.is_null() {
                    destroy_framebuffer
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyFramebuffer>(f)
                }
            },
            create_render_pass: unsafe {
                unsafe extern "system" fn create_render_pass(
                    _device: crate::vk::Device,
                    _create_info: *const RenderPassCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _render_pass: *mut RenderPass,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(create_render_pass),))
                }
                let f = f(c"vkCreateRenderPass");
                if f.is_null() {
                    create_render_pass
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateRenderPass>(f)
                }
            },
            destroy_render_pass: unsafe {
                unsafe extern "system" fn destroy_render_pass(
                    _device: crate::vk::Device,
                    _render_pass: RenderPass,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_render_pass),))
                }
                let f = f(c"vkDestroyRenderPass");
                if f.is_null() {
                    destroy_render_pass
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyRenderPass>(f)
                }
            },
            get_render_area_granularity: unsafe {
                unsafe extern "system" fn get_render_area_granularity(
                    _device: crate::vk::Device,
                    _render_pass: RenderPass,
                    _granularity: *mut Extent2D,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(get_render_area_granularity),
                    ))
                }
                let f = f(c"vkGetRenderAreaGranularity");
                if f.is_null() {
                    get_render_area_granularity
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetRenderAreaGranularity>(f)
                }
            },
            create_command_pool: unsafe {
                unsafe extern "system" fn create_command_pool(
                    _device: crate::vk::Device,
                    _create_info: *const CommandPoolCreateInfo,
                    _allocator: *const AllocationCallbacks,
                    _command_pool: *mut CommandPool,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(create_command_pool),))
                }
                let f = f(c"vkCreateCommandPool");
                if f.is_null() {
                    create_command_pool
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateCommandPool>(f)
                }
            },
            destroy_command_pool: unsafe {
                unsafe extern "system" fn destroy_command_pool(
                    _device: crate::vk::Device,
                    _command_pool: CommandPool,
                    _allocator: *const AllocationCallbacks,
                ) {
                    panic!(concat!("failed to load ", stringify!(destroy_command_pool),))
                }
                let f = f(c"vkDestroyCommandPool");
                if f.is_null() {
                    destroy_command_pool
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyCommandPool>(f)
                }
            },
            reset_command_pool: unsafe {
                unsafe extern "system" fn reset_command_pool(
                    _device: crate::vk::Device,
                    _command_pool: CommandPool,
                    _flags: CommandPoolResetFlags,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(reset_command_pool),))
                }
                let f = f(c"vkResetCommandPool");
                if f.is_null() {
                    reset_command_pool
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkResetCommandPool>(f)
                }
            },
            allocate_command_buffers: unsafe {
                unsafe extern "system" fn allocate_command_buffers(
                    _device: crate::vk::Device,
                    _allocate_info: *const CommandBufferAllocateInfo,
                    _command_buffers: *mut CommandBuffer,
                ) -> Result {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(allocate_command_buffers),
                    ))
                }
                let f = f(c"vkAllocateCommandBuffers");
                if f.is_null() {
                    allocate_command_buffers
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkAllocateCommandBuffers>(f)
                }
            },
            free_command_buffers: unsafe {
                unsafe extern "system" fn free_command_buffers(
                    _device: crate::vk::Device,
                    _command_pool: CommandPool,
                    _command_buffer_count: u32,
                    _command_buffers: *const CommandBuffer,
                ) {
                    panic!(concat!("failed to load ", stringify!(free_command_buffers),))
                }
                let f = f(c"vkFreeCommandBuffers");
                if f.is_null() {
                    free_command_buffers
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkFreeCommandBuffers>(f)
                }
            },
            begin_command_buffer: unsafe {
                unsafe extern "system" fn begin_command_buffer(
                    _command_buffer: CommandBuffer,
                    _begin_info: *const CommandBufferBeginInfo,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(begin_command_buffer),))
                }
                let f = f(c"vkBeginCommandBuffer");
                if f.is_null() {
                    begin_command_buffer
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkBeginCommandBuffer>(f)
                }
            },
            end_command_buffer: unsafe {
                unsafe extern "system" fn end_command_buffer(
                    _command_buffer: CommandBuffer,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(end_command_buffer),))
                }
                let f = f(c"vkEndCommandBuffer");
                if f.is_null() {
                    end_command_buffer
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkEndCommandBuffer>(f)
                }
            },
            reset_command_buffer: unsafe {
                unsafe extern "system" fn reset_command_buffer(
                    _command_buffer: CommandBuffer,
                    _flags: CommandBufferResetFlags,
                ) -> Result {
                    panic!(concat!("failed to load ", stringify!(reset_command_buffer),))
                }
                let f = f(c"vkResetCommandBuffer");
                if f.is_null() {
                    reset_command_buffer
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkResetCommandBuffer>(f)
                }
            },
            cmd_bind_pipeline: unsafe {
                unsafe extern "system" fn cmd_bind_pipeline(
                    _command_buffer: CommandBuffer,
                    _pipeline_bind_point: PipelineBindPoint,
                    _pipeline: Pipeline,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_bind_pipeline),))
                }
                let f = f(c"vkCmdBindPipeline");
                if f.is_null() {
                    cmd_bind_pipeline
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBindPipeline>(f)
                }
            },
            cmd_set_viewport: unsafe {
                unsafe extern "system" fn cmd_set_viewport(
                    _command_buffer: CommandBuffer,
                    _first_viewport: u32,
                    _viewport_count: u32,
                    _viewports: *const Viewport,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_set_viewport),))
                }
                let f = f(c"vkCmdSetViewport");
                if f.is_null() {
                    cmd_set_viewport
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetViewport>(f)
                }
            },
            cmd_set_scissor: unsafe {
                unsafe extern "system" fn cmd_set_scissor(
                    _command_buffer: CommandBuffer,
                    _first_scissor: u32,
                    _scissor_count: u32,
                    _scissors: *const Rect2D,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_set_scissor),))
                }
                let f = f(c"vkCmdSetScissor");
                if f.is_null() {
                    cmd_set_scissor
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetScissor>(f)
                }
            },
            cmd_set_line_width: unsafe {
                unsafe extern "system" fn cmd_set_line_width(
                    _command_buffer: CommandBuffer,
                    _line_width: f32,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_set_line_width),))
                }
                let f = f(c"vkCmdSetLineWidth");
                if f.is_null() {
                    cmd_set_line_width
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetLineWidth>(f)
                }
            },
            cmd_set_depth_bias: unsafe {
                unsafe extern "system" fn cmd_set_depth_bias(
                    _command_buffer: CommandBuffer,
                    _depth_bias_constant_factor: f32,
                    _depth_bias_clamp: f32,
                    _depth_bias_slope_factor: f32,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_set_depth_bias),))
                }
                let f = f(c"vkCmdSetDepthBias");
                if f.is_null() {
                    cmd_set_depth_bias
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetDepthBias>(f)
                }
            },
            cmd_set_blend_constants: unsafe {
                unsafe extern "system" fn cmd_set_blend_constants(
                    _command_buffer: CommandBuffer,
                    _blend_constants: *const [f32; 4usize],
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_set_blend_constants),
                    ))
                }
                let f = f(c"vkCmdSetBlendConstants");
                if f.is_null() {
                    cmd_set_blend_constants
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetBlendConstants>(f)
                }
            },
            cmd_set_depth_bounds: unsafe {
                unsafe extern "system" fn cmd_set_depth_bounds(
                    _command_buffer: CommandBuffer,
                    _min_depth_bounds: f32,
                    _max_depth_bounds: f32,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_set_depth_bounds),))
                }
                let f = f(c"vkCmdSetDepthBounds");
                if f.is_null() {
                    cmd_set_depth_bounds
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetDepthBounds>(f)
                }
            },
            cmd_set_stencil_compare_mask: unsafe {
                unsafe extern "system" fn cmd_set_stencil_compare_mask(
                    _command_buffer: CommandBuffer,
                    _face_mask: StencilFaceFlags,
                    _compare_mask: u32,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_set_stencil_compare_mask),
                    ))
                }
                let f = f(c"vkCmdSetStencilCompareMask");
                if f.is_null() {
                    cmd_set_stencil_compare_mask
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetStencilCompareMask>(f)
                }
            },
            cmd_set_stencil_write_mask: unsafe {
                unsafe extern "system" fn cmd_set_stencil_write_mask(
                    _command_buffer: CommandBuffer,
                    _face_mask: StencilFaceFlags,
                    _write_mask: u32,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_set_stencil_write_mask),
                    ))
                }
                let f = f(c"vkCmdSetStencilWriteMask");
                if f.is_null() {
                    cmd_set_stencil_write_mask
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetStencilWriteMask>(f)
                }
            },
            cmd_set_stencil_reference: unsafe {
                unsafe extern "system" fn cmd_set_stencil_reference(
                    _command_buffer: CommandBuffer,
                    _face_mask: StencilFaceFlags,
                    _reference: u32,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_set_stencil_reference),
                    ))
                }
                let f = f(c"vkCmdSetStencilReference");
                if f.is_null() {
                    cmd_set_stencil_reference
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetStencilReference>(f)
                }
            },
            cmd_bind_descriptor_sets: unsafe {
                unsafe extern "system" fn cmd_bind_descriptor_sets(
                    _command_buffer: CommandBuffer,
                    _pipeline_bind_point: PipelineBindPoint,
                    _layout: PipelineLayout,
                    _first_set: u32,
                    _descriptor_set_count: u32,
                    _descriptor_sets: *const DescriptorSet,
                    _dynamic_offset_count: u32,
                    _dynamic_offsets: *const u32,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_bind_descriptor_sets),
                    ))
                }
                let f = f(c"vkCmdBindDescriptorSets");
                if f.is_null() {
                    cmd_bind_descriptor_sets
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBindDescriptorSets>(f)
                }
            },
            cmd_bind_index_buffer: unsafe {
                unsafe extern "system" fn cmd_bind_index_buffer(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _index_type: IndexType,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_bind_index_buffer),
                    ))
                }
                let f = f(c"vkCmdBindIndexBuffer");
                if f.is_null() {
                    cmd_bind_index_buffer
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBindIndexBuffer>(f)
                }
            },
            cmd_bind_vertex_buffers: unsafe {
                unsafe extern "system" fn cmd_bind_vertex_buffers(
                    _command_buffer: CommandBuffer,
                    _first_binding: u32,
                    _binding_count: u32,
                    _buffers: *const Buffer,
                    _offsets: *const DeviceSize,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_bind_vertex_buffers),
                    ))
                }
                let f = f(c"vkCmdBindVertexBuffers");
                if f.is_null() {
                    cmd_bind_vertex_buffers
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBindVertexBuffers>(f)
                }
            },
            cmd_draw: unsafe {
                unsafe extern "system" fn cmd_draw(
                    _command_buffer: CommandBuffer,
                    _vertex_count: u32,
                    _instance_count: u32,
                    _first_vertex: u32,
                    _first_instance: u32,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_draw),))
                }
                let f = f(c"vkCmdDraw");
                if f.is_null() {
                    cmd_draw
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdDraw>(f)
                }
            },
            cmd_draw_indexed: unsafe {
                unsafe extern "system" fn cmd_draw_indexed(
                    _command_buffer: CommandBuffer,
                    _index_count: u32,
                    _instance_count: u32,
                    _first_index: u32,
                    _vertex_offset: i32,
                    _first_instance: u32,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_draw_indexed),))
                }
                let f = f(c"vkCmdDrawIndexed");
                if f.is_null() {
                    cmd_draw_indexed
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdDrawIndexed>(f)
                }
            },
            cmd_draw_indirect: unsafe {
                unsafe extern "system" fn cmd_draw_indirect(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_draw_indirect),))
                }
                let f = f(c"vkCmdDrawIndirect");
                if f.is_null() {
                    cmd_draw_indirect
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdDrawIndirect>(f)
                }
            },
            cmd_draw_indexed_indirect: unsafe {
                unsafe extern "system" fn cmd_draw_indexed_indirect(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_draw_indexed_indirect),
                    ))
                }
                let f = f(c"vkCmdDrawIndexedIndirect");
                if f.is_null() {
                    cmd_draw_indexed_indirect
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdDrawIndexedIndirect>(f)
                }
            },
            cmd_dispatch: unsafe {
                unsafe extern "system" fn cmd_dispatch(
                    _command_buffer: CommandBuffer,
                    _group_count_x: u32,
                    _group_count_y: u32,
                    _group_count_z: u32,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_dispatch),))
                }
                let f = f(c"vkCmdDispatch");
                if f.is_null() {
                    cmd_dispatch
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdDispatch>(f)
                }
            },
            cmd_dispatch_indirect: unsafe {
                unsafe extern "system" fn cmd_dispatch_indirect(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_dispatch_indirect),
                    ))
                }
                let f = f(c"vkCmdDispatchIndirect");
                if f.is_null() {
                    cmd_dispatch_indirect
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdDispatchIndirect>(f)
                }
            },
            cmd_copy_buffer: unsafe {
                unsafe extern "system" fn cmd_copy_buffer(
                    _command_buffer: CommandBuffer,
                    _src_buffer: Buffer,
                    _dst_buffer: Buffer,
                    _region_count: u32,
                    _regions: *const BufferCopy,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_copy_buffer),))
                }
                let f = f(c"vkCmdCopyBuffer");
                if f.is_null() {
                    cmd_copy_buffer
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdCopyBuffer>(f)
                }
            },
            cmd_copy_image: unsafe {
                unsafe extern "system" fn cmd_copy_image(
                    _command_buffer: CommandBuffer,
                    _src_image: Image,
                    _src_image_layout: ImageLayout,
                    _dst_image: Image,
                    _dst_image_layout: ImageLayout,
                    _region_count: u32,
                    _regions: *const ImageCopy,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_copy_image),))
                }
                let f = f(c"vkCmdCopyImage");
                if f.is_null() {
                    cmd_copy_image
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdCopyImage>(f)
                }
            },
            cmd_blit_image: unsafe {
                unsafe extern "system" fn cmd_blit_image(
                    _command_buffer: CommandBuffer,
                    _src_image: Image,
                    _src_image_layout: ImageLayout,
                    _dst_image: Image,
                    _dst_image_layout: ImageLayout,
                    _region_count: u32,
                    _regions: *const ImageBlit,
                    _filter: Filter,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_blit_image),))
                }
                let f = f(c"vkCmdBlitImage");
                if f.is_null() {
                    cmd_blit_image
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBlitImage>(f)
                }
            },
            cmd_copy_buffer_to_image: unsafe {
                unsafe extern "system" fn cmd_copy_buffer_to_image(
                    _command_buffer: CommandBuffer,
                    _src_buffer: Buffer,
                    _dst_image: Image,
                    _dst_image_layout: ImageLayout,
                    _region_count: u32,
                    _regions: *const BufferImageCopy,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_copy_buffer_to_image),
                    ))
                }
                let f = f(c"vkCmdCopyBufferToImage");
                if f.is_null() {
                    cmd_copy_buffer_to_image
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdCopyBufferToImage>(f)
                }
            },
            cmd_copy_image_to_buffer: unsafe {
                unsafe extern "system" fn cmd_copy_image_to_buffer(
                    _command_buffer: CommandBuffer,
                    _src_image: Image,
                    _src_image_layout: ImageLayout,
                    _dst_buffer: Buffer,
                    _region_count: u32,
                    _regions: *const BufferImageCopy,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_copy_image_to_buffer),
                    ))
                }
                let f = f(c"vkCmdCopyImageToBuffer");
                if f.is_null() {
                    cmd_copy_image_to_buffer
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdCopyImageToBuffer>(f)
                }
            },
            cmd_update_buffer: unsafe {
                unsafe extern "system" fn cmd_update_buffer(
                    _command_buffer: CommandBuffer,
                    _dst_buffer: Buffer,
                    _dst_offset: DeviceSize,
                    _data_size: DeviceSize,
                    _data: *const ffi::c_void,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_update_buffer),))
                }
                let f = f(c"vkCmdUpdateBuffer");
                if f.is_null() {
                    cmd_update_buffer
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdUpdateBuffer>(f)
                }
            },
            cmd_fill_buffer: unsafe {
                unsafe extern "system" fn cmd_fill_buffer(
                    _command_buffer: CommandBuffer,
                    _dst_buffer: Buffer,
                    _dst_offset: DeviceSize,
                    _size: DeviceSize,
                    _data: u32,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_fill_buffer),))
                }
                let f = f(c"vkCmdFillBuffer");
                if f.is_null() {
                    cmd_fill_buffer
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdFillBuffer>(f)
                }
            },
            cmd_clear_color_image: unsafe {
                unsafe extern "system" fn cmd_clear_color_image(
                    _command_buffer: CommandBuffer,
                    _image: Image,
                    _image_layout: ImageLayout,
                    _color: *const ClearColorValue,
                    _range_count: u32,
                    _ranges: *const ImageSubresourceRange,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_clear_color_image),
                    ))
                }
                let f = f(c"vkCmdClearColorImage");
                if f.is_null() {
                    cmd_clear_color_image
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdClearColorImage>(f)
                }
            },
            cmd_clear_depth_stencil_image: unsafe {
                unsafe extern "system" fn cmd_clear_depth_stencil_image(
                    _command_buffer: CommandBuffer,
                    _image: Image,
                    _image_layout: ImageLayout,
                    _depth_stencil: *const ClearDepthStencilValue,
                    _range_count: u32,
                    _ranges: *const ImageSubresourceRange,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_clear_depth_stencil_image),
                    ))
                }
                let f = f(c"vkCmdClearDepthStencilImage");
                if f.is_null() {
                    cmd_clear_depth_stencil_image
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdClearDepthStencilImage>(f)
                }
            },
            cmd_clear_attachments: unsafe {
                unsafe extern "system" fn cmd_clear_attachments(
                    _command_buffer: CommandBuffer,
                    _attachment_count: u32,
                    _attachments: *const ClearAttachment,
                    _rect_count: u32,
                    _rects: *const ClearRect,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_clear_attachments),
                    ))
                }
                let f = f(c"vkCmdClearAttachments");
                if f.is_null() {
                    cmd_clear_attachments
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdClearAttachments>(f)
                }
            },
            cmd_resolve_image: unsafe {
                unsafe extern "system" fn cmd_resolve_image(
                    _command_buffer: CommandBuffer,
                    _src_image: Image,
                    _src_image_layout: ImageLayout,
                    _dst_image: Image,
                    _dst_image_layout: ImageLayout,
                    _region_count: u32,
                    _regions: *const ImageResolve,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_resolve_image),))
                }
                let f = f(c"vkCmdResolveImage");
                if f.is_null() {
                    cmd_resolve_image
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdResolveImage>(f)
                }
            },
            cmd_set_event: unsafe {
                unsafe extern "system" fn cmd_set_event(
                    _command_buffer: CommandBuffer,
                    _event: Event,
                    _stage_mask: PipelineStageFlags,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_set_event),))
                }
                let f = f(c"vkCmdSetEvent");
                if f.is_null() {
                    cmd_set_event
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetEvent>(f)
                }
            },
            cmd_reset_event: unsafe {
                unsafe extern "system" fn cmd_reset_event(
                    _command_buffer: CommandBuffer,
                    _event: Event,
                    _stage_mask: PipelineStageFlags,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_reset_event),))
                }
                let f = f(c"vkCmdResetEvent");
                if f.is_null() {
                    cmd_reset_event
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdResetEvent>(f)
                }
            },
            cmd_wait_events: unsafe {
                unsafe extern "system" fn cmd_wait_events(
                    _command_buffer: CommandBuffer,
                    _event_count: u32,
                    _events: *const Event,
                    _src_stage_mask: PipelineStageFlags,
                    _dst_stage_mask: PipelineStageFlags,
                    _memory_barrier_count: u32,
                    _memory_barriers: *const MemoryBarrier,
                    _buffer_memory_barrier_count: u32,
                    _buffer_memory_barriers: *const BufferMemoryBarrier,
                    _image_memory_barrier_count: u32,
                    _image_memory_barriers: *const ImageMemoryBarrier,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_wait_events),))
                }
                let f = f(c"vkCmdWaitEvents");
                if f.is_null() {
                    cmd_wait_events
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdWaitEvents>(f)
                }
            },
            cmd_pipeline_barrier: unsafe {
                unsafe extern "system" fn cmd_pipeline_barrier(
                    _command_buffer: CommandBuffer,
                    _src_stage_mask: PipelineStageFlags,
                    _dst_stage_mask: PipelineStageFlags,
                    _dependency_flags: DependencyFlags,
                    _memory_barrier_count: u32,
                    _memory_barriers: *const MemoryBarrier,
                    _buffer_memory_barrier_count: u32,
                    _buffer_memory_barriers: *const BufferMemoryBarrier,
                    _image_memory_barrier_count: u32,
                    _image_memory_barriers: *const ImageMemoryBarrier,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_pipeline_barrier),))
                }
                let f = f(c"vkCmdPipelineBarrier");
                if f.is_null() {
                    cmd_pipeline_barrier
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdPipelineBarrier>(f)
                }
            },
            cmd_begin_query: unsafe {
                unsafe extern "system" fn cmd_begin_query(
                    _command_buffer: CommandBuffer,
                    _query_pool: QueryPool,
                    _query: u32,
                    _flags: QueryControlFlags,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_begin_query),))
                }
                let f = f(c"vkCmdBeginQuery");
                if f.is_null() {
                    cmd_begin_query
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBeginQuery>(f)
                }
            },
            cmd_end_query: unsafe {
                unsafe extern "system" fn cmd_end_query(
                    _command_buffer: CommandBuffer,
                    _query_pool: QueryPool,
                    _query: u32,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_end_query),))
                }
                let f = f(c"vkCmdEndQuery");
                if f.is_null() {
                    cmd_end_query
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdEndQuery>(f)
                }
            },
            cmd_reset_query_pool: unsafe {
                unsafe extern "system" fn cmd_reset_query_pool(
                    _command_buffer: CommandBuffer,
                    _query_pool: QueryPool,
                    _first_query: u32,
                    _query_count: u32,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_reset_query_pool),))
                }
                let f = f(c"vkCmdResetQueryPool");
                if f.is_null() {
                    cmd_reset_query_pool
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdResetQueryPool>(f)
                }
            },
            cmd_write_timestamp: unsafe {
                unsafe extern "system" fn cmd_write_timestamp(
                    _command_buffer: CommandBuffer,
                    _pipeline_stage: PipelineStageFlags,
                    _query_pool: QueryPool,
                    _query: u32,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_write_timestamp),))
                }
                let f = f(c"vkCmdWriteTimestamp");
                if f.is_null() {
                    cmd_write_timestamp
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdWriteTimestamp>(f)
                }
            },
            cmd_copy_query_pool_results: unsafe {
                unsafe extern "system" fn cmd_copy_query_pool_results(
                    _command_buffer: CommandBuffer,
                    _query_pool: QueryPool,
                    _first_query: u32,
                    _query_count: u32,
                    _dst_buffer: Buffer,
                    _dst_offset: DeviceSize,
                    _stride: DeviceSize,
                    _flags: QueryResultFlags,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_copy_query_pool_results),
                    ))
                }
                let f = f(c"vkCmdCopyQueryPoolResults");
                if f.is_null() {
                    cmd_copy_query_pool_results
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdCopyQueryPoolResults>(f)
                }
            },
            cmd_push_constants: unsafe {
                unsafe extern "system" fn cmd_push_constants(
                    _command_buffer: CommandBuffer,
                    _layout: PipelineLayout,
                    _stage_flags: ShaderStageFlags,
                    _offset: u32,
                    _size: u32,
                    _values: *const ffi::c_void,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_push_constants),))
                }
                let f = f(c"vkCmdPushConstants");
                if f.is_null() {
                    cmd_push_constants
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdPushConstants>(f)
                }
            },
            cmd_begin_render_pass: unsafe {
                unsafe extern "system" fn cmd_begin_render_pass(
                    _command_buffer: CommandBuffer,
                    _render_pass_begin: *const RenderPassBeginInfo,
                    _contents: SubpassContents,
                ) {
                    panic!(concat!(
                        "failed to load ",
                        stringify!(cmd_begin_render_pass),
                    ))
                }
                let f = f(c"vkCmdBeginRenderPass");
                if f.is_null() {
                    cmd_begin_render_pass
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBeginRenderPass>(f)
                }
            },
            cmd_next_subpass: unsafe {
                unsafe extern "system" fn cmd_next_subpass(
                    _command_buffer: CommandBuffer,
                    _contents: SubpassContents,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_next_subpass),))
                }
                let f = f(c"vkCmdNextSubpass");
                if f.is_null() {
                    cmd_next_subpass
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdNextSubpass>(f)
                }
            },
            cmd_end_render_pass: unsafe {
                unsafe extern "system" fn cmd_end_render_pass(_command_buffer: CommandBuffer) {
                    panic!(concat!("failed to load ", stringify!(cmd_end_render_pass),))
                }
                let f = f(c"vkCmdEndRenderPass");
                if f.is_null() {
                    cmd_end_render_pass
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdEndRenderPass>(f)
                }
            },
            cmd_execute_commands: unsafe {
                unsafe extern "system" fn cmd_execute_commands(
                    _command_buffer: CommandBuffer,
                    _command_buffer_count: u32,
                    _command_buffers: *const CommandBuffer,
                ) {
                    panic!(concat!("failed to load ", stringify!(cmd_execute_commands),))
                }
                let f = f(c"vkCmdExecuteCommands");
                if f.is_null() {
                    cmd_execute_commands
                } else {
                    ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdExecuteCommands>(f)
                }
            },
        }
    }
}
impl<Ext> crate::Device<Ext> {
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceProcAddr.html>"]
    #[doc = r""]
    pub unsafe fn get_device_proc_addr(&self, name: &ffi::CStr) -> PFN_vkVoidFunction {
        unsafe { (self.fp_v10.get_device_proc_addr)(self.handle, name.as_ptr()) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue.html>"]
    #[doc = r""]
    pub unsafe fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> Queue {
        unsafe {
            let mut queue = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.get_device_queue)(
                self.handle,
                queue_family_index,
                queue_index,
                queue.as_mut_ptr(),
            );
            queue.assume_init()
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[deprecated = "superseded by vkQueueSubmit2"]
    pub unsafe fn queue_submit(
        &self,
        queue: Queue,
        submits: &[SubmitInfo<'_>],
        fence: Fence,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.queue_submit)(
                queue,
                submits.len() as _,
                if !submits.is_empty() {
                    submits.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                fence,
            )
            .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueWaitIdle.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn queue_wait_idle(&self, queue: Queue) -> crate::VkResult<()> {
        unsafe { (self.fp_v10.queue_wait_idle)(queue).result(&[crate::vk::Result::SUCCESS]) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDeviceWaitIdle.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn device_wait_idle(&self) -> crate::VkResult<()> {
        unsafe { (self.fp_v10.device_wait_idle)(self.handle).result(&[crate::vk::Result::SUCCESS]) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateMemory.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn allocate_memory(
        &self,
        allocate_info: &MemoryAllocateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<DeviceMemory> {
        unsafe {
            let mut memory = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.allocate_memory)(
                self.handle,
                allocate_info,
                allocator.as_ptr(),
                memory.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], memory)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeMemory.html>"]
    #[doc = r""]
    pub unsafe fn free_memory(
        &self,
        memory: DeviceMemory,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.fp_v10.free_memory)(self.handle, memory, allocator.as_ptr()) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn map_memory(
        &self,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
    ) -> crate::VkResult<*mut ffi::c_void> {
        unsafe {
            let mut pp_data = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.map_memory)(
                self.handle,
                memory,
                offset,
                size,
                flags,
                pp_data.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], pp_data)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory.html>"]
    #[doc = r""]
    pub unsafe fn unmap_memory(&self, memory: DeviceMemory) {
        unsafe { (self.fp_v10.unmap_memory)(self.handle, memory) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn flush_mapped_memory_ranges(
        &self,
        memory_ranges: &[MappedMemoryRange<'_>],
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.flush_mapped_memory_ranges)(
                self.handle,
                memory_ranges.len() as _,
                if !memory_ranges.is_empty() {
                    memory_ranges.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
            .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkInvalidateMappedMemoryRanges.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn invalidate_mapped_memory_ranges(
        &self,
        memory_ranges: &[MappedMemoryRange<'_>],
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.invalidate_mapped_memory_ranges)(
                self.handle,
                memory_ranges.len() as _,
                if !memory_ranges.is_empty() {
                    memory_ranges.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
            .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryCommitment.html>"]
    #[doc = r""]
    pub unsafe fn get_device_memory_commitment(&self, memory: DeviceMemory) -> DeviceSize {
        unsafe {
            let mut committed_memory_in_bytes = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.get_device_memory_commitment)(
                self.handle,
                memory,
                committed_memory_in_bytes.as_mut_ptr(),
            );
            committed_memory_in_bytes.assume_init()
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements.html>"]
    #[doc = r""]
    pub unsafe fn get_buffer_memory_requirements(&self, buffer: Buffer) -> MemoryRequirements {
        unsafe {
            let mut memory_requirements = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.get_buffer_memory_requirements)(
                self.handle,
                buffer,
                memory_requirements.as_mut_ptr(),
            );
            memory_requirements.assume_init()
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn bind_buffer_memory(
        &self,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.bind_buffer_memory)(self.handle, buffer, memory, memory_offset)
                .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements.html>"]
    #[doc = r""]
    pub unsafe fn get_image_memory_requirements(&self, image: Image) -> MemoryRequirements {
        unsafe {
            let mut memory_requirements = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.get_image_memory_requirements)(
                self.handle,
                image,
                memory_requirements.as_mut_ptr(),
            );
            memory_requirements.assume_init()
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn bind_image_memory(
        &self,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.bind_image_memory)(self.handle, image, memory, memory_offset)
                .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements.html>"]
    #[doc = r""]
    pub unsafe fn get_image_sparse_memory_requirements_len(&self, image: Image) -> u32 {
        unsafe {
            let mut sparse_memory_requirement_count = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.get_image_sparse_memory_requirements)(
                self.handle,
                image,
                sparse_memory_requirement_count.as_mut_ptr(),
                core::ptr::null_mut(),
            );
            sparse_memory_requirement_count.assume_init()
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements.html>"]
    #[doc = r""]
    pub unsafe fn get_image_sparse_memory_requirements(
        &self,
        image: Image,
        out: &mut [SparseImageMemoryRequirements],
    ) {
        unsafe {
            let mut len = out.len() as _;
            (self.fp_v10.get_image_sparse_memory_requirements)(
                self.handle,
                image,
                &mut len,
                out.as_mut_ptr(),
            )
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBindSparse.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn queue_bind_sparse(
        &self,
        queue: Queue,
        bind_info: &[BindSparseInfo<'_>],
        fence: Fence,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.queue_bind_sparse)(
                queue,
                bind_info.len() as _,
                if !bind_info.is_empty() {
                    bind_info.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                fence,
            )
            .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFence.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_fence(
        &self,
        create_info: &FenceCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<Fence> {
        unsafe {
            let mut fence = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_fence)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                fence.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], fence)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFence.html>"]
    #[doc = r""]
    pub unsafe fn destroy_fence(&self, fence: Fence, allocator: Option<&AllocationCallbacks>) {
        unsafe { (self.fp_v10.destroy_fence)(self.handle, fence, allocator.as_ptr()) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetFences.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn reset_fences(&self, fences: &[Fence]) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.reset_fences)(
                self.handle,
                fences.len() as _,
                if !fences.is_empty() {
                    fences.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
            .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`NOT_READY`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceStatus.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::NOT_READY"]
    pub unsafe fn get_fence_status(&self, fence: Fence) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.get_fence_status)(self.handle, fence)
                .result(&[crate::vk::Result::SUCCESS, crate::vk::Result::NOT_READY])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`TIMEOUT`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForFences.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::TIMEOUT"]
    pub unsafe fn wait_for_fences(
        &self,
        fences: &[Fence],
        wait_all: bool,
        timeout: u64,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.wait_for_fences)(
                self.handle,
                fences.len() as _,
                if !fences.is_empty() {
                    fences.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                wait_all as Bool32,
                timeout,
            )
            .result(&[crate::vk::Result::SUCCESS, crate::vk::Result::TIMEOUT])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSemaphore.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_semaphore(
        &self,
        create_info: &SemaphoreCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<Semaphore> {
        unsafe {
            let mut semaphore = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_semaphore)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                semaphore.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], semaphore)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphore.html>"]
    #[doc = r""]
    pub unsafe fn destroy_semaphore(
        &self,
        semaphore: Semaphore,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.fp_v10.destroy_semaphore)(self.handle, semaphore, allocator.as_ptr()) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateEvent.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_event(
        &self,
        create_info: &EventCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<Event> {
        unsafe {
            let mut event = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_event)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                event.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], event)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyEvent.html>"]
    #[doc = r""]
    pub unsafe fn destroy_event(&self, event: Event, allocator: Option<&AllocationCallbacks>) {
        unsafe { (self.fp_v10.destroy_event)(self.handle, event, allocator.as_ptr()) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`EVENT_SET`][0]"]
    #[doc = "* [`EVENT_RESET`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEventStatus.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::EVENT_SET"]
    #[doc = "[1]: Result::EVENT_RESET"]
    pub unsafe fn get_event_status(&self, event: Event) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.get_event_status)(self.handle, event)
                .result(&[crate::vk::Result::EVENT_SET, crate::vk::Result::EVENT_RESET])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetEvent.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn set_event(&self, event: Event) -> crate::VkResult<()> {
        unsafe { (self.fp_v10.set_event)(self.handle, event).result(&[crate::vk::Result::SUCCESS]) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetEvent.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn reset_event(&self, event: Event) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.reset_event)(self.handle, event).result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateQueryPool.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_query_pool(
        &self,
        create_info: &QueryPoolCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<QueryPool> {
        unsafe {
            let mut query_pool = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_query_pool)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                query_pool.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], query_pool)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyQueryPool.html>"]
    #[doc = r""]
    pub unsafe fn destroy_query_pool(
        &self,
        query_pool: QueryPool,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.fp_v10.destroy_query_pool)(self.handle, query_pool, allocator.as_ptr()) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`NOT_READY`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueryPoolResults.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::NOT_READY"]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn get_query_pool_results(
        &self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        data: &mut [ffi::c_void],
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.get_query_pool_results)(
                self.handle,
                query_pool,
                first_query,
                query_count,
                data_size,
                data.as_mut_ptr(),
                stride,
                flags,
            )
            .result(&[crate::vk::Result::SUCCESS, crate::vk::Result::NOT_READY])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBuffer.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_buffer(
        &self,
        create_info: &BufferCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<Buffer> {
        unsafe {
            let mut buffer = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_buffer)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                buffer.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], buffer)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBuffer.html>"]
    #[doc = r""]
    pub unsafe fn destroy_buffer(&self, buffer: Buffer, allocator: Option<&AllocationCallbacks>) {
        unsafe { (self.fp_v10.destroy_buffer)(self.handle, buffer, allocator.as_ptr()) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferView.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_buffer_view(
        &self,
        create_info: &BufferViewCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<BufferView> {
        unsafe {
            let mut view = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_buffer_view)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                view.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], view)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferView.html>"]
    #[doc = r""]
    pub unsafe fn destroy_buffer_view(
        &self,
        buffer_view: BufferView,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.fp_v10.destroy_buffer_view)(self.handle, buffer_view, allocator.as_ptr()) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImage.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_image(
        &self,
        create_info: &ImageCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<Image> {
        unsafe {
            let mut image = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_image)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                image.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], image)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImage.html>"]
    #[doc = r""]
    pub unsafe fn destroy_image(&self, image: Image, allocator: Option<&AllocationCallbacks>) {
        unsafe { (self.fp_v10.destroy_image)(self.handle, image, allocator.as_ptr()) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout.html>"]
    #[doc = r""]
    pub unsafe fn get_image_subresource_layout(
        &self,
        image: Image,
        subresource: &ImageSubresource,
    ) -> SubresourceLayout {
        unsafe {
            let mut layout = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.get_image_subresource_layout)(
                self.handle,
                image,
                subresource,
                layout.as_mut_ptr(),
            );
            layout.assume_init()
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImageView.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_image_view(
        &self,
        create_info: &ImageViewCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<ImageView> {
        unsafe {
            let mut view = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_image_view)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                view.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], view)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImageView.html>"]
    #[doc = r""]
    pub unsafe fn destroy_image_view(
        &self,
        image_view: ImageView,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.fp_v10.destroy_image_view)(self.handle, image_view, allocator.as_ptr()) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderModule.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_shader_module(
        &self,
        create_info: &ShaderModuleCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<ShaderModule> {
        unsafe {
            let mut shader_module = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_shader_module)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                shader_module.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], shader_module)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderModule.html>"]
    #[doc = r""]
    pub unsafe fn destroy_shader_module(
        &self,
        shader_module: ShaderModule,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fp_v10.destroy_shader_module)(self.handle, shader_module, allocator.as_ptr())
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineCache.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_pipeline_cache(
        &self,
        create_info: &PipelineCacheCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<PipelineCache> {
        unsafe {
            let mut pipeline_cache = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_pipeline_cache)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                pipeline_cache.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], pipeline_cache)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineCache.html>"]
    #[doc = r""]
    pub unsafe fn destroy_pipeline_cache(
        &self,
        pipeline_cache: PipelineCache,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fp_v10.destroy_pipeline_cache)(self.handle, pipeline_cache, allocator.as_ptr())
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineCacheData.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    pub unsafe fn get_pipeline_cache_data_len(
        &self,
        pipeline_cache: PipelineCache,
    ) -> crate::VkResult<usize> {
        unsafe {
            let mut data_size = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.get_pipeline_cache_data)(
                self.handle,
                pipeline_cache,
                data_size.as_mut_ptr(),
                core::ptr::null_mut(),
            )
            .result_with_assume_init(
                &[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE],
                data_size,
            )
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`INCOMPLETE`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineCacheData.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::INCOMPLETE"]
    pub unsafe fn get_pipeline_cache_data(
        &self,
        pipeline_cache: PipelineCache,
        out: &mut [u8],
    ) -> crate::VkResult<()> {
        unsafe {
            let mut len = out.len() as _;
            (self.fp_v10.get_pipeline_cache_data)(
                self.handle,
                pipeline_cache,
                &mut len,
                out.as_mut_ptr().cast(),
            )
            .result(&[crate::vk::Result::SUCCESS, crate::vk::Result::INCOMPLETE])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkMergePipelineCaches.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn merge_pipeline_caches(
        &self,
        dst_cache: PipelineCache,
        src_caches: &[PipelineCache],
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.merge_pipeline_caches)(
                self.handle,
                dst_cache,
                src_caches.len() as _,
                if !src_caches.is_empty() {
                    src_caches.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
            .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`PIPELINE_COMPILE_REQUIRED_EXT`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGraphicsPipelines.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::PIPELINE_COMPILE_REQUIRED_EXT"]
    pub unsafe fn create_graphics_pipelines(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[GraphicsPipelineCreateInfo<'_>],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.create_graphics_pipelines)(
                self.handle,
                pipeline_cache,
                create_infos.len() as _,
                if !create_infos.is_empty() {
                    create_infos.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                allocator.as_ptr(),
                pipelines.as_mut_ptr(),
            )
            .result(&[
                crate::vk::Result::SUCCESS,
                crate::vk::Result::PIPELINE_COMPILE_REQUIRED_EXT,
            ])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`PIPELINE_COMPILE_REQUIRED_EXT`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateComputePipelines.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::PIPELINE_COMPILE_REQUIRED_EXT"]
    pub unsafe fn create_compute_pipelines(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[ComputePipelineCreateInfo<'_>],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.create_compute_pipelines)(
                self.handle,
                pipeline_cache,
                create_infos.len() as _,
                if !create_infos.is_empty() {
                    create_infos.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                allocator.as_ptr(),
                pipelines.as_mut_ptr(),
            )
            .result(&[
                crate::vk::Result::SUCCESS,
                crate::vk::Result::PIPELINE_COMPILE_REQUIRED_EXT,
            ])
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipeline.html>"]
    #[doc = r""]
    pub unsafe fn destroy_pipeline(
        &self,
        pipeline: Pipeline,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.fp_v10.destroy_pipeline)(self.handle, pipeline, allocator.as_ptr()) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineLayout.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_pipeline_layout(
        &self,
        create_info: &PipelineLayoutCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<PipelineLayout> {
        unsafe {
            let mut pipeline_layout = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_pipeline_layout)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                pipeline_layout.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], pipeline_layout)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineLayout.html>"]
    #[doc = r""]
    pub unsafe fn destroy_pipeline_layout(
        &self,
        pipeline_layout: PipelineLayout,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fp_v10.destroy_pipeline_layout)(self.handle, pipeline_layout, allocator.as_ptr())
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSampler.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_sampler(
        &self,
        create_info: &SamplerCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<Sampler> {
        unsafe {
            let mut sampler = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_sampler)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                sampler.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], sampler)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySampler.html>"]
    #[doc = r""]
    pub unsafe fn destroy_sampler(
        &self,
        sampler: Sampler,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.fp_v10.destroy_sampler)(self.handle, sampler, allocator.as_ptr()) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorSetLayout.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_descriptor_set_layout(
        &self,
        create_info: &DescriptorSetLayoutCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<DescriptorSetLayout> {
        unsafe {
            let mut set_layout = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_descriptor_set_layout)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                set_layout.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], set_layout)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorSetLayout.html>"]
    #[doc = r""]
    pub unsafe fn destroy_descriptor_set_layout(
        &self,
        descriptor_set_layout: DescriptorSetLayout,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fp_v10.destroy_descriptor_set_layout)(
                self.handle,
                descriptor_set_layout,
                allocator.as_ptr(),
            )
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorPool.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_descriptor_pool(
        &self,
        create_info: &DescriptorPoolCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<DescriptorPool> {
        unsafe {
            let mut descriptor_pool = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_descriptor_pool)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                descriptor_pool.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], descriptor_pool)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorPool.html>"]
    #[doc = r""]
    pub unsafe fn destroy_descriptor_pool(
        &self,
        descriptor_pool: DescriptorPool,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fp_v10.destroy_descriptor_pool)(self.handle, descriptor_pool, allocator.as_ptr())
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetDescriptorPool.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn reset_descriptor_pool(
        &self,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.reset_descriptor_pool)(self.handle, descriptor_pool, flags)
                .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateDescriptorSets.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn allocate_descriptor_sets(
        &self,
        allocate_info: &DescriptorSetAllocateInfo<'_>,
        descriptor_sets: &mut [DescriptorSet],
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.allocate_descriptor_sets)(
                self.handle,
                allocate_info,
                descriptor_sets.as_mut_ptr(),
            )
            .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeDescriptorSets.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn free_descriptor_sets(
        &self,
        descriptor_pool: DescriptorPool,
        descriptor_sets: &[DescriptorSet],
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.free_descriptor_sets)(
                self.handle,
                descriptor_pool,
                descriptor_sets.len() as _,
                if !descriptor_sets.is_empty() {
                    descriptor_sets.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
            .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSets.html>"]
    #[doc = r""]
    pub unsafe fn update_descriptor_sets(
        &self,
        descriptor_writes: &[WriteDescriptorSet<'_>],
        descriptor_copies: &[CopyDescriptorSet<'_>],
    ) {
        unsafe {
            (self.fp_v10.update_descriptor_sets)(
                self.handle,
                descriptor_writes.len() as _,
                if !descriptor_writes.is_empty() {
                    descriptor_writes.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                descriptor_copies.len() as _,
                if !descriptor_copies.is_empty() {
                    descriptor_copies.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFramebuffer.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[deprecated]
    pub unsafe fn create_framebuffer(
        &self,
        create_info: &FramebufferCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<Framebuffer> {
        unsafe {
            let mut framebuffer = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_framebuffer)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                framebuffer.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], framebuffer)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFramebuffer.html>"]
    #[doc = r""]
    #[deprecated]
    pub unsafe fn destroy_framebuffer(
        &self,
        framebuffer: Framebuffer,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.fp_v10.destroy_framebuffer)(self.handle, framebuffer, allocator.as_ptr()) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[deprecated = "superseded by vkCreateRenderPass2"]
    pub unsafe fn create_render_pass(
        &self,
        create_info: &RenderPassCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<RenderPass> {
        unsafe {
            let mut render_pass = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_render_pass)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                render_pass.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], render_pass)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyRenderPass.html>"]
    #[doc = r""]
    #[deprecated]
    pub unsafe fn destroy_render_pass(
        &self,
        render_pass: RenderPass,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.fp_v10.destroy_render_pass)(self.handle, render_pass, allocator.as_ptr()) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderAreaGranularity.html>"]
    #[doc = r""]
    #[deprecated]
    pub unsafe fn get_render_area_granularity(&self, render_pass: RenderPass) -> Extent2D {
        unsafe {
            let mut granularity = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.get_render_area_granularity)(
                self.handle,
                render_pass,
                granularity.as_mut_ptr(),
            );
            granularity.assume_init()
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCommandPool.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_command_pool(
        &self,
        create_info: &CommandPoolCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<CommandPool> {
        unsafe {
            let mut command_pool = ::core::mem::MaybeUninit::uninit();
            (self.fp_v10.create_command_pool)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                command_pool.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], command_pool)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCommandPool.html>"]
    #[doc = r""]
    pub unsafe fn destroy_command_pool(
        &self,
        command_pool: CommandPool,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.fp_v10.destroy_command_pool)(self.handle, command_pool, allocator.as_ptr()) }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandPool.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn reset_command_pool(
        &self,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.reset_command_pool)(self.handle, command_pool, flags)
                .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateCommandBuffers.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn allocate_command_buffers(
        &self,
        allocate_info: &CommandBufferAllocateInfo<'_>,
        command_buffers: &mut [CommandBuffer],
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.allocate_command_buffers)(
                self.handle,
                allocate_info,
                command_buffers.as_mut_ptr(),
            )
            .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeCommandBuffers.html>"]
    #[doc = r""]
    pub unsafe fn free_command_buffers(
        &self,
        command_pool: CommandPool,
        command_buffers: &[CommandBuffer],
    ) {
        unsafe {
            (self.fp_v10.free_command_buffers)(
                self.handle,
                command_pool,
                command_buffers.len() as _,
                if !command_buffers.is_empty() {
                    command_buffers.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBeginCommandBuffer.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn begin_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        begin_info: &CommandBufferBeginInfo<'_>,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.begin_command_buffer)(command_buffer, begin_info)
                .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkEndCommandBuffer.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn end_command_buffer(&self, command_buffer: CommandBuffer) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.end_command_buffer)(command_buffer).result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandBuffer.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn reset_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        flags: CommandBufferResetFlags,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v10.reset_command_buffer)(command_buffer, flags)
                .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipeline.html>"]
    #[doc = r""]
    pub unsafe fn cmd_bind_pipeline(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        unsafe { (self.fp_v10.cmd_bind_pipeline)(command_buffer, pipeline_bind_point, pipeline) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewport.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_viewport(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewports: &[Viewport],
    ) {
        unsafe {
            (self.fp_v10.cmd_set_viewport)(
                command_buffer,
                first_viewport,
                viewports.len() as _,
                if !viewports.is_empty() {
                    viewports.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissor.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_scissor(
        &self,
        command_buffer: CommandBuffer,
        first_scissor: u32,
        scissors: &[Rect2D],
    ) {
        unsafe {
            (self.fp_v10.cmd_set_scissor)(
                command_buffer,
                first_scissor,
                scissors.len() as _,
                if !scissors.is_empty() {
                    scissors.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineWidth.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_line_width(&self, command_buffer: CommandBuffer, line_width: f32) {
        unsafe { (self.fp_v10.cmd_set_line_width)(command_buffer, line_width) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_depth_bias(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        unsafe {
            (self.fp_v10.cmd_set_depth_bias)(
                command_buffer,
                depth_bias_constant_factor,
                depth_bias_clamp,
                depth_bias_slope_factor,
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetBlendConstants.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_blend_constants(
        &self,
        command_buffer: CommandBuffer,
        blend_constants: &[f32; 4usize],
    ) {
        unsafe { (self.fp_v10.cmd_set_blend_constants)(command_buffer, blend_constants) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBounds.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_depth_bounds(
        &self,
        command_buffer: CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) {
        unsafe {
            (self.fp_v10.cmd_set_depth_bounds)(command_buffer, min_depth_bounds, max_depth_bounds)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilCompareMask.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_stencil_compare_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: u32,
    ) {
        unsafe {
            (self.fp_v10.cmd_set_stencil_compare_mask)(command_buffer, face_mask, compare_mask)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilWriteMask.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_stencil_write_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: u32,
    ) {
        unsafe { (self.fp_v10.cmd_set_stencil_write_mask)(command_buffer, face_mask, write_mask) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilReference.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_stencil_reference(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: u32,
    ) {
        unsafe { (self.fp_v10.cmd_set_stencil_reference)(command_buffer, face_mask, reference) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets.html>"]
    #[doc = r""]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn cmd_bind_descriptor_sets(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_sets: &[DescriptorSet],
        dynamic_offsets: &[u32],
    ) {
        unsafe {
            (self.fp_v10.cmd_bind_descriptor_sets)(
                command_buffer,
                pipeline_bind_point,
                layout,
                first_set,
                descriptor_sets.len() as _,
                if !descriptor_sets.is_empty() {
                    descriptor_sets.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                dynamic_offsets.len() as _,
                if !dynamic_offsets.is_empty() {
                    dynamic_offsets.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer.html>"]
    #[doc = r""]
    pub unsafe fn cmd_bind_index_buffer(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType,
    ) {
        unsafe { (self.fp_v10.cmd_bind_index_buffer)(command_buffer, buffer, offset, index_type) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers.html>"]
    #[doc = r""]
    pub unsafe fn cmd_bind_vertex_buffers(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
    ) {
        unsafe {
            (self.fp_v10.cmd_bind_vertex_buffers)(
                command_buffer,
                first_binding,
                buffers.len() as _,
                if !buffers.is_empty() {
                    buffers.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                if !offsets.is_empty() {
                    offsets.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDraw.html>"]
    #[doc = r""]
    pub unsafe fn cmd_draw(
        &self,
        command_buffer: CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            (self.fp_v10.cmd_draw)(
                command_buffer,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexed.html>"]
    #[doc = r""]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn cmd_draw_indexed(
        &self,
        command_buffer: CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        unsafe {
            (self.fp_v10.cmd_draw_indexed)(
                command_buffer,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect.html>"]
    #[doc = r""]
    pub unsafe fn cmd_draw_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.fp_v10.cmd_draw_indirect)(command_buffer, buffer, offset, draw_count, stride)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect.html>"]
    #[doc = r""]
    pub unsafe fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.fp_v10.cmd_draw_indexed_indirect)(
                command_buffer,
                buffer,
                offset,
                draw_count,
                stride,
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatch.html>"]
    #[doc = r""]
    pub unsafe fn cmd_dispatch(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe {
            (self.fp_v10.cmd_dispatch)(command_buffer, group_count_x, group_count_y, group_count_z)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect.html>"]
    #[doc = r""]
    pub unsafe fn cmd_dispatch_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
    ) {
        unsafe { (self.fp_v10.cmd_dispatch_indirect)(command_buffer, buffer, offset) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer.html>"]
    #[doc = r""]
    pub unsafe fn cmd_copy_buffer(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        regions: &[BufferCopy],
    ) {
        unsafe {
            (self.fp_v10.cmd_copy_buffer)(
                command_buffer,
                src_buffer,
                dst_buffer,
                regions.len() as _,
                if !regions.is_empty() {
                    regions.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage.html>"]
    #[doc = r""]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn cmd_copy_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageCopy],
    ) {
        unsafe {
            (self.fp_v10.cmd_copy_image)(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions.len() as _,
                if !regions.is_empty() {
                    regions.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage.html>"]
    #[doc = r""]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn cmd_blit_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageBlit],
        filter: Filter,
    ) {
        unsafe {
            (self.fp_v10.cmd_blit_image)(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions.len() as _,
                if !regions.is_empty() {
                    regions.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                filter,
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage.html>"]
    #[doc = r""]
    pub unsafe fn cmd_copy_buffer_to_image(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[BufferImageCopy],
    ) {
        unsafe {
            (self.fp_v10.cmd_copy_buffer_to_image)(
                command_buffer,
                src_buffer,
                dst_image,
                dst_image_layout,
                regions.len() as _,
                if !regions.is_empty() {
                    regions.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer.html>"]
    #[doc = r""]
    pub unsafe fn cmd_copy_image_to_buffer(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        regions: &[BufferImageCopy],
    ) {
        unsafe {
            (self.fp_v10.cmd_copy_image_to_buffer)(
                command_buffer,
                src_image,
                src_image_layout,
                dst_buffer,
                regions.len() as _,
                if !regions.is_empty() {
                    regions.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateBuffer.html>"]
    #[doc = r""]
    pub unsafe fn cmd_update_buffer(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        data: &[u8],
    ) {
        unsafe {
            (self.fp_v10.cmd_update_buffer)(
                command_buffer,
                dst_buffer,
                dst_offset,
                data.len() as _,
                if !data.is_empty() {
                    data.as_ptr().cast()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillBuffer.html>"]
    #[doc = r""]
    pub unsafe fn cmd_fill_buffer(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: u32,
    ) {
        unsafe { (self.fp_v10.cmd_fill_buffer)(command_buffer, dst_buffer, dst_offset, size, data) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearColorImage.html>"]
    #[doc = r""]
    pub unsafe fn cmd_clear_color_image(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        color: &ClearColorValue,
        ranges: &[ImageSubresourceRange],
    ) {
        unsafe {
            (self.fp_v10.cmd_clear_color_image)(
                command_buffer,
                image,
                image_layout,
                color,
                ranges.len() as _,
                if !ranges.is_empty() {
                    ranges.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearDepthStencilImage.html>"]
    #[doc = r""]
    pub unsafe fn cmd_clear_depth_stencil_image(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        depth_stencil: &ClearDepthStencilValue,
        ranges: &[ImageSubresourceRange],
    ) {
        unsafe {
            (self.fp_v10.cmd_clear_depth_stencil_image)(
                command_buffer,
                image,
                image_layout,
                depth_stencil,
                ranges.len() as _,
                if !ranges.is_empty() {
                    ranges.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearAttachments.html>"]
    #[doc = r""]
    pub unsafe fn cmd_clear_attachments(
        &self,
        command_buffer: CommandBuffer,
        attachments: &[ClearAttachment],
        rects: &[ClearRect],
    ) {
        unsafe {
            (self.fp_v10.cmd_clear_attachments)(
                command_buffer,
                attachments.len() as _,
                if !attachments.is_empty() {
                    attachments.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                rects.len() as _,
                if !rects.is_empty() {
                    rects.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage.html>"]
    #[doc = r""]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn cmd_resolve_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageResolve],
    ) {
        unsafe {
            (self.fp_v10.cmd_resolve_image)(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions.len() as _,
                if !regions.is_empty() {
                    regions.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkCmdSetEvent2"]
    pub unsafe fn cmd_set_event(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) {
        unsafe { (self.fp_v10.cmd_set_event)(command_buffer, event, stage_mask) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkCmdResetEvent2"]
    pub unsafe fn cmd_reset_event(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) {
        unsafe { (self.fp_v10.cmd_reset_event)(command_buffer, event, stage_mask) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkCmdWaitEvents2"]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn cmd_wait_events(
        &self,
        command_buffer: CommandBuffer,
        events: &[Event],
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barriers: &[MemoryBarrier<'_>],
        buffer_memory_barriers: &[BufferMemoryBarrier<'_>],
        image_memory_barriers: &[ImageMemoryBarrier<'_>],
    ) {
        unsafe {
            (self.fp_v10.cmd_wait_events)(
                command_buffer,
                events.len() as _,
                if !events.is_empty() {
                    events.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                src_stage_mask,
                dst_stage_mask,
                memory_barriers.len() as _,
                if !memory_barriers.is_empty() {
                    memory_barriers.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                buffer_memory_barriers.len() as _,
                if !buffer_memory_barriers.is_empty() {
                    buffer_memory_barriers.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                image_memory_barriers.len() as _,
                if !image_memory_barriers.is_empty() {
                    image_memory_barriers.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkCmdPipelineBarrier2"]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn cmd_pipeline_barrier(
        &self,
        command_buffer: CommandBuffer,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barriers: &[MemoryBarrier<'_>],
        buffer_memory_barriers: &[BufferMemoryBarrier<'_>],
        image_memory_barriers: &[ImageMemoryBarrier<'_>],
    ) {
        unsafe {
            (self.fp_v10.cmd_pipeline_barrier)(
                command_buffer,
                src_stage_mask,
                dst_stage_mask,
                dependency_flags,
                memory_barriers.len() as _,
                if !memory_barriers.is_empty() {
                    memory_barriers.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                buffer_memory_barriers.len() as _,
                if !buffer_memory_barriers.is_empty() {
                    buffer_memory_barriers.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                image_memory_barriers.len() as _,
                if !image_memory_barriers.is_empty() {
                    image_memory_barriers.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQuery.html>"]
    #[doc = r""]
    pub unsafe fn cmd_begin_query(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
    ) {
        unsafe { (self.fp_v10.cmd_begin_query)(command_buffer, query_pool, query, flags) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQuery.html>"]
    #[doc = r""]
    pub unsafe fn cmd_end_query(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
    ) {
        unsafe { (self.fp_v10.cmd_end_query)(command_buffer, query_pool, query) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetQueryPool.html>"]
    #[doc = r""]
    pub unsafe fn cmd_reset_query_pool(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        unsafe {
            (self.fp_v10.cmd_reset_query_pool)(command_buffer, query_pool, first_query, query_count)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkCmdWriteTimestamp2"]
    pub unsafe fn cmd_write_timestamp(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlags,
        query_pool: QueryPool,
        query: u32,
    ) {
        unsafe {
            (self.fp_v10.cmd_write_timestamp)(command_buffer, pipeline_stage, query_pool, query)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResults.html>"]
    #[doc = r""]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn cmd_copy_query_pool_results(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) {
        unsafe {
            (self.fp_v10.cmd_copy_query_pool_results)(
                command_buffer,
                query_pool,
                first_query,
                query_count,
                dst_buffer,
                dst_offset,
                stride,
                flags,
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants.html>"]
    #[doc = r""]
    pub unsafe fn cmd_push_constants(
        &self,
        command_buffer: CommandBuffer,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        values: &[u8],
    ) {
        unsafe {
            (self.fp_v10.cmd_push_constants)(
                command_buffer,
                layout,
                stage_flags,
                offset,
                values.len() as _,
                if !values.is_empty() {
                    values.as_ptr().cast()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkCmdBeginRenderPass2"]
    pub unsafe fn cmd_begin_render_pass(
        &self,
        command_buffer: CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo<'_>,
        contents: SubpassContents,
    ) {
        unsafe { (self.fp_v10.cmd_begin_render_pass)(command_buffer, render_pass_begin, contents) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkCmdNextSubpass2"]
    pub unsafe fn cmd_next_subpass(
        &self,
        command_buffer: CommandBuffer,
        contents: SubpassContents,
    ) {
        unsafe { (self.fp_v10.cmd_next_subpass)(command_buffer, contents) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass.html>"]
    #[doc = r""]
    #[deprecated = "superseded by vkCmdEndRenderPass2"]
    pub unsafe fn cmd_end_render_pass(&self, command_buffer: CommandBuffer) {
        unsafe { (self.fp_v10.cmd_end_render_pass)(command_buffer) }
    }
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteCommands.html>"]
    #[doc = r""]
    pub unsafe fn cmd_execute_commands(
        &self,
        command_buffer: CommandBuffer,
        command_buffers: &[CommandBuffer],
    ) {
        unsafe {
            (self.fp_v10.cmd_execute_commands)(
                command_buffer,
                command_buffers.len() as _,
                if !command_buffers.is_empty() {
                    command_buffers.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
}
#[derive(Clone, Copy)]
pub struct DeviceFpV11 {
    pub trim_command_pool: PFN_vkTrimCommandPool,
    pub get_device_group_peer_memory_features: PFN_vkGetDeviceGroupPeerMemoryFeatures,
    pub bind_buffer_memory2: PFN_vkBindBufferMemory2,
    pub bind_image_memory2: PFN_vkBindImageMemory2,
    pub cmd_set_device_mask: PFN_vkCmdSetDeviceMask,
    pub cmd_dispatch_base: PFN_vkCmdDispatchBase,
    pub create_descriptor_update_template: PFN_vkCreateDescriptorUpdateTemplate,
    pub destroy_descriptor_update_template: PFN_vkDestroyDescriptorUpdateTemplate,
    pub update_descriptor_set_with_template: PFN_vkUpdateDescriptorSetWithTemplate,
    pub get_buffer_memory_requirements2: PFN_vkGetBufferMemoryRequirements2,
    pub get_image_memory_requirements2: PFN_vkGetImageMemoryRequirements2,
    pub get_image_sparse_memory_requirements2: PFN_vkGetImageSparseMemoryRequirements2,
    pub create_sampler_ycbcr_conversion: PFN_vkCreateSamplerYcbcrConversion,
    pub destroy_sampler_ycbcr_conversion: PFN_vkDestroySamplerYcbcrConversion,
    pub get_device_queue2: PFN_vkGetDeviceQueue2,
    pub get_descriptor_set_layout_support: PFN_vkGetDescriptorSetLayoutSupport,
}
unsafe impl Send for DeviceFpV11 {}
unsafe impl Sync for DeviceFpV11 {}
impl DeviceFpV11 {
    #[allow(unused_variables)]
    pub fn load(version: u32, f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void) -> Self {
        if version >= API_VERSION_1_1 {
            Self {
                trim_command_pool: unsafe {
                    unsafe extern "system" fn trim_command_pool(
                        _device: crate::vk::Device,
                        _command_pool: CommandPool,
                        _flags: CommandPoolTrimFlags,
                    ) {
                        panic!(concat!("failed to load ", stringify!(trim_command_pool),))
                    }
                    let f = f(c"vkTrimCommandPool");
                    if f.is_null() {
                        trim_command_pool
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkTrimCommandPool>(f)
                    }
                },
                get_device_group_peer_memory_features: unsafe {
                    unsafe extern "system" fn get_device_group_peer_memory_features(
                        _device: crate::vk::Device,
                        _heap_index: u32,
                        _local_device_index: u32,
                        _remote_device_index: u32,
                        _peer_memory_features: *mut PeerMemoryFeatureFlags,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_device_group_peer_memory_features),
                        ))
                    }
                    let f = f(c"vkGetDeviceGroupPeerMemoryFeatures");
                    if f.is_null() {
                        get_device_group_peer_memory_features
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetDeviceGroupPeerMemoryFeatures,
                        >(f)
                    }
                },
                bind_buffer_memory2: unsafe {
                    unsafe extern "system" fn bind_buffer_memory2(
                        _device: crate::vk::Device,
                        _bind_info_count: u32,
                        _bind_infos: *const BindBufferMemoryInfo,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(bind_buffer_memory2),))
                    }
                    let f = f(c"vkBindBufferMemory2");
                    if f.is_null() {
                        bind_buffer_memory2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkBindBufferMemory2>(f)
                    }
                },
                bind_image_memory2: unsafe {
                    unsafe extern "system" fn bind_image_memory2(
                        _device: crate::vk::Device,
                        _bind_info_count: u32,
                        _bind_infos: *const BindImageMemoryInfo,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(bind_image_memory2),))
                    }
                    let f = f(c"vkBindImageMemory2");
                    if f.is_null() {
                        bind_image_memory2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkBindImageMemory2>(f)
                    }
                },
                cmd_set_device_mask: unsafe {
                    unsafe extern "system" fn cmd_set_device_mask(
                        _command_buffer: CommandBuffer,
                        _device_mask: u32,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_set_device_mask),))
                    }
                    let f = f(c"vkCmdSetDeviceMask");
                    if f.is_null() {
                        cmd_set_device_mask
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetDeviceMask>(f)
                    }
                },
                cmd_dispatch_base: unsafe {
                    unsafe extern "system" fn cmd_dispatch_base(
                        _command_buffer: CommandBuffer,
                        _base_group_x: u32,
                        _base_group_y: u32,
                        _base_group_z: u32,
                        _group_count_x: u32,
                        _group_count_y: u32,
                        _group_count_z: u32,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_dispatch_base),))
                    }
                    let f = f(c"vkCmdDispatchBase");
                    if f.is_null() {
                        cmd_dispatch_base
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdDispatchBase>(f)
                    }
                },
                create_descriptor_update_template: unsafe {
                    unsafe extern "system" fn create_descriptor_update_template(
                        _device: crate::vk::Device,
                        _create_info: *const DescriptorUpdateTemplateCreateInfo,
                        _allocator: *const AllocationCallbacks,
                        _descriptor_update_template: *mut DescriptorUpdateTemplate,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(create_descriptor_update_template),
                        ))
                    }
                    let f = f(c"vkCreateDescriptorUpdateTemplate");
                    if f.is_null() {
                        create_descriptor_update_template
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCreateDescriptorUpdateTemplate,
                        >(f)
                    }
                },
                destroy_descriptor_update_template: unsafe {
                    unsafe extern "system" fn destroy_descriptor_update_template(
                        _device: crate::vk::Device,
                        _descriptor_update_template: DescriptorUpdateTemplate,
                        _allocator: *const AllocationCallbacks,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(destroy_descriptor_update_template),
                        ))
                    }
                    let f = f(c"vkDestroyDescriptorUpdateTemplate");
                    if f.is_null() {
                        destroy_descriptor_update_template
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkDestroyDescriptorUpdateTemplate,
                        >(f)
                    }
                },
                update_descriptor_set_with_template: unsafe {
                    unsafe extern "system" fn update_descriptor_set_with_template(
                        _device: crate::vk::Device,
                        _descriptor_set: DescriptorSet,
                        _descriptor_update_template: DescriptorUpdateTemplate,
                        _data: *const ffi::c_void,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(update_descriptor_set_with_template),
                        ))
                    }
                    let f = f(c"vkUpdateDescriptorSetWithTemplate");
                    if f.is_null() {
                        update_descriptor_set_with_template
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkUpdateDescriptorSetWithTemplate,
                        >(f)
                    }
                },
                get_buffer_memory_requirements2: unsafe {
                    unsafe extern "system" fn get_buffer_memory_requirements2(
                        _device: crate::vk::Device,
                        _info: *const BufferMemoryRequirementsInfo2,
                        _memory_requirements: *mut MemoryRequirements2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_buffer_memory_requirements2),
                        ))
                    }
                    let f = f(c"vkGetBufferMemoryRequirements2");
                    if f.is_null() {
                        get_buffer_memory_requirements2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetBufferMemoryRequirements2,
                        >(f)
                    }
                },
                get_image_memory_requirements2: unsafe {
                    unsafe extern "system" fn get_image_memory_requirements2(
                        _device: crate::vk::Device,
                        _info: *const ImageMemoryRequirementsInfo2,
                        _memory_requirements: *mut MemoryRequirements2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_image_memory_requirements2),
                        ))
                    }
                    let f = f(c"vkGetImageMemoryRequirements2");
                    if f.is_null() {
                        get_image_memory_requirements2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetImageMemoryRequirements2,
                        >(f)
                    }
                },
                get_image_sparse_memory_requirements2: unsafe {
                    unsafe extern "system" fn get_image_sparse_memory_requirements2(
                        _device: crate::vk::Device,
                        _info: *const ImageSparseMemoryRequirementsInfo2,
                        _sparse_memory_requirement_count: *mut u32,
                        _sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_image_sparse_memory_requirements2),
                        ))
                    }
                    let f = f(c"vkGetImageSparseMemoryRequirements2");
                    if f.is_null() {
                        get_image_sparse_memory_requirements2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetImageSparseMemoryRequirements2,
                        >(f)
                    }
                },
                create_sampler_ycbcr_conversion: unsafe {
                    unsafe extern "system" fn create_sampler_ycbcr_conversion(
                        _device: crate::vk::Device,
                        _create_info: *const SamplerYcbcrConversionCreateInfo,
                        _allocator: *const AllocationCallbacks,
                        _ycbcr_conversion: *mut SamplerYcbcrConversion,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(create_sampler_ycbcr_conversion),
                        ))
                    }
                    let f = f(c"vkCreateSamplerYcbcrConversion");
                    if f.is_null() {
                        create_sampler_ycbcr_conversion
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCreateSamplerYcbcrConversion,
                        >(f)
                    }
                },
                destroy_sampler_ycbcr_conversion: unsafe {
                    unsafe extern "system" fn destroy_sampler_ycbcr_conversion(
                        _device: crate::vk::Device,
                        _ycbcr_conversion: SamplerYcbcrConversion,
                        _allocator: *const AllocationCallbacks,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(destroy_sampler_ycbcr_conversion),
                        ))
                    }
                    let f = f(c"vkDestroySamplerYcbcrConversion");
                    if f.is_null() {
                        destroy_sampler_ycbcr_conversion
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkDestroySamplerYcbcrConversion,
                        >(f)
                    }
                },
                get_device_queue2: unsafe {
                    unsafe extern "system" fn get_device_queue2(
                        _device: crate::vk::Device,
                        _queue_info: *const DeviceQueueInfo2,
                        _queue: *mut Queue,
                    ) {
                        panic!(concat!("failed to load ", stringify!(get_device_queue2),))
                    }
                    let f = f(c"vkGetDeviceQueue2");
                    if f.is_null() {
                        get_device_queue2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetDeviceQueue2>(f)
                    }
                },
                get_descriptor_set_layout_support: unsafe {
                    unsafe extern "system" fn get_descriptor_set_layout_support(
                        _device: crate::vk::Device,
                        _create_info: *const DescriptorSetLayoutCreateInfo,
                        _support: *mut DescriptorSetLayoutSupport,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_descriptor_set_layout_support),
                        ))
                    }
                    let f = f(c"vkGetDescriptorSetLayoutSupport");
                    if f.is_null() {
                        get_descriptor_set_layout_support
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetDescriptorSetLayoutSupport,
                        >(f)
                    }
                },
            }
        } else {
            Self {
                trim_command_pool: unsafe {
                    unsafe extern "system" fn trim_command_pool(
                        _device: crate::vk::Device,
                        _command_pool: CommandPool,
                        _flags: CommandPoolTrimFlags,
                    ) {
                        panic!(concat!("failed to load ", stringify!(trim_command_pool),))
                    }
                    let f = f(c"vkTrimCommandPoolKHR");
                    if f.is_null() {
                        trim_command_pool
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkTrimCommandPoolKHR>(f)
                    }
                },
                get_device_group_peer_memory_features: unsafe {
                    unsafe extern "system" fn get_device_group_peer_memory_features(
                        _device: crate::vk::Device,
                        _heap_index: u32,
                        _local_device_index: u32,
                        _remote_device_index: u32,
                        _peer_memory_features: *mut PeerMemoryFeatureFlags,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_device_group_peer_memory_features),
                        ))
                    }
                    let f = f(c"vkGetDeviceGroupPeerMemoryFeaturesKHR");
                    if f.is_null() {
                        get_device_group_peer_memory_features
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR,
                        >(f)
                    }
                },
                bind_buffer_memory2: unsafe {
                    unsafe extern "system" fn bind_buffer_memory2(
                        _device: crate::vk::Device,
                        _bind_info_count: u32,
                        _bind_infos: *const BindBufferMemoryInfo,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(bind_buffer_memory2),))
                    }
                    let f = f(c"vkBindBufferMemory2KHR");
                    if f.is_null() {
                        bind_buffer_memory2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkBindBufferMemory2KHR>(f)
                    }
                },
                bind_image_memory2: unsafe {
                    unsafe extern "system" fn bind_image_memory2(
                        _device: crate::vk::Device,
                        _bind_info_count: u32,
                        _bind_infos: *const BindImageMemoryInfo,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(bind_image_memory2),))
                    }
                    let f = f(c"vkBindImageMemory2KHR");
                    if f.is_null() {
                        bind_image_memory2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkBindImageMemory2KHR>(f)
                    }
                },
                cmd_set_device_mask: unsafe {
                    unsafe extern "system" fn cmd_set_device_mask(
                        _command_buffer: CommandBuffer,
                        _device_mask: u32,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_set_device_mask),))
                    }
                    let f = f(c"vkCmdSetDeviceMaskKHR");
                    if f.is_null() {
                        cmd_set_device_mask
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetDeviceMaskKHR>(f)
                    }
                },
                cmd_dispatch_base: unsafe {
                    unsafe extern "system" fn cmd_dispatch_base(
                        _command_buffer: CommandBuffer,
                        _base_group_x: u32,
                        _base_group_y: u32,
                        _base_group_z: u32,
                        _group_count_x: u32,
                        _group_count_y: u32,
                        _group_count_z: u32,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_dispatch_base),))
                    }
                    let f = f(c"vkCmdDispatchBaseKHR");
                    if f.is_null() {
                        cmd_dispatch_base
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdDispatchBaseKHR>(f)
                    }
                },
                create_descriptor_update_template: unsafe {
                    unsafe extern "system" fn create_descriptor_update_template(
                        _device: crate::vk::Device,
                        _create_info: *const DescriptorUpdateTemplateCreateInfo,
                        _allocator: *const AllocationCallbacks,
                        _descriptor_update_template: *mut DescriptorUpdateTemplate,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(create_descriptor_update_template),
                        ))
                    }
                    let f = f(c"vkCreateDescriptorUpdateTemplateKHR");
                    if f.is_null() {
                        create_descriptor_update_template
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCreateDescriptorUpdateTemplateKHR,
                        >(f)
                    }
                },
                destroy_descriptor_update_template: unsafe {
                    unsafe extern "system" fn destroy_descriptor_update_template(
                        _device: crate::vk::Device,
                        _descriptor_update_template: DescriptorUpdateTemplate,
                        _allocator: *const AllocationCallbacks,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(destroy_descriptor_update_template),
                        ))
                    }
                    let f = f(c"vkDestroyDescriptorUpdateTemplateKHR");
                    if f.is_null() {
                        destroy_descriptor_update_template
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkDestroyDescriptorUpdateTemplateKHR,
                        >(f)
                    }
                },
                update_descriptor_set_with_template: unsafe {
                    unsafe extern "system" fn update_descriptor_set_with_template(
                        _device: crate::vk::Device,
                        _descriptor_set: DescriptorSet,
                        _descriptor_update_template: DescriptorUpdateTemplate,
                        _data: *const ffi::c_void,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(update_descriptor_set_with_template),
                        ))
                    }
                    let f = f(c"vkUpdateDescriptorSetWithTemplateKHR");
                    if f.is_null() {
                        update_descriptor_set_with_template
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkUpdateDescriptorSetWithTemplateKHR,
                        >(f)
                    }
                },
                get_buffer_memory_requirements2: unsafe {
                    unsafe extern "system" fn get_buffer_memory_requirements2(
                        _device: crate::vk::Device,
                        _info: *const BufferMemoryRequirementsInfo2,
                        _memory_requirements: *mut MemoryRequirements2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_buffer_memory_requirements2),
                        ))
                    }
                    let f = f(c"vkGetBufferMemoryRequirements2KHR");
                    if f.is_null() {
                        get_buffer_memory_requirements2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetBufferMemoryRequirements2KHR,
                        >(f)
                    }
                },
                get_image_memory_requirements2: unsafe {
                    unsafe extern "system" fn get_image_memory_requirements2(
                        _device: crate::vk::Device,
                        _info: *const ImageMemoryRequirementsInfo2,
                        _memory_requirements: *mut MemoryRequirements2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_image_memory_requirements2),
                        ))
                    }
                    let f = f(c"vkGetImageMemoryRequirements2KHR");
                    if f.is_null() {
                        get_image_memory_requirements2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetImageMemoryRequirements2KHR,
                        >(f)
                    }
                },
                get_image_sparse_memory_requirements2: unsafe {
                    unsafe extern "system" fn get_image_sparse_memory_requirements2(
                        _device: crate::vk::Device,
                        _info: *const ImageSparseMemoryRequirementsInfo2,
                        _sparse_memory_requirement_count: *mut u32,
                        _sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_image_sparse_memory_requirements2),
                        ))
                    }
                    let f = f(c"vkGetImageSparseMemoryRequirements2KHR");
                    if f.is_null() {
                        get_image_sparse_memory_requirements2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetImageSparseMemoryRequirements2KHR,
                        >(f)
                    }
                },
                create_sampler_ycbcr_conversion: unsafe {
                    unsafe extern "system" fn create_sampler_ycbcr_conversion(
                        _device: crate::vk::Device,
                        _create_info: *const SamplerYcbcrConversionCreateInfo,
                        _allocator: *const AllocationCallbacks,
                        _ycbcr_conversion: *mut SamplerYcbcrConversion,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(create_sampler_ycbcr_conversion),
                        ))
                    }
                    let f = f(c"vkCreateSamplerYcbcrConversionKHR");
                    if f.is_null() {
                        create_sampler_ycbcr_conversion
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCreateSamplerYcbcrConversionKHR,
                        >(f)
                    }
                },
                destroy_sampler_ycbcr_conversion: unsafe {
                    unsafe extern "system" fn destroy_sampler_ycbcr_conversion(
                        _device: crate::vk::Device,
                        _ycbcr_conversion: SamplerYcbcrConversion,
                        _allocator: *const AllocationCallbacks,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(destroy_sampler_ycbcr_conversion),
                        ))
                    }
                    let f = f(c"vkDestroySamplerYcbcrConversionKHR");
                    if f.is_null() {
                        destroy_sampler_ycbcr_conversion
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkDestroySamplerYcbcrConversionKHR,
                        >(f)
                    }
                },
                get_device_queue2: unsafe {
                    unsafe extern "system" fn get_device_queue2(
                        _device: crate::vk::Device,
                        _queue_info: *const DeviceQueueInfo2,
                        _queue: *mut Queue,
                    ) {
                        panic!(concat!("failed to load ", stringify!(get_device_queue2),))
                    }
                    let f = f(c"vkGetDeviceQueue2");
                    if f.is_null() {
                        get_device_queue2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetDeviceQueue2>(f)
                    }
                },
                get_descriptor_set_layout_support: unsafe {
                    unsafe extern "system" fn get_descriptor_set_layout_support(
                        _device: crate::vk::Device,
                        _create_info: *const DescriptorSetLayoutCreateInfo,
                        _support: *mut DescriptorSetLayoutSupport,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_descriptor_set_layout_support),
                        ))
                    }
                    let f = f(c"vkGetDescriptorSetLayoutSupportKHR");
                    if f.is_null() {
                        get_descriptor_set_layout_support
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetDescriptorSetLayoutSupportKHR,
                        >(f)
                    }
                },
            }
        }
    }
}
impl<Ext> crate::Device<Ext> {
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_maintenance1.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html>"]
    #[doc = r""]
    pub unsafe fn trim_command_pool(&self, command_pool: CommandPool, flags: CommandPoolTrimFlags) {
        unsafe { (self.fp_v11.trim_command_pool)(self.handle, command_pool, flags) }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_device_group.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeatures.html>"]
    #[doc = r""]
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> PeerMemoryFeatureFlags {
        unsafe {
            let mut peer_memory_features = ::core::mem::MaybeUninit::uninit();
            (self.fp_v11.get_device_group_peer_memory_features)(
                self.handle,
                heap_index,
                local_device_index,
                remote_device_index,
                peer_memory_features.as_mut_ptr(),
            );
            peer_memory_features.assume_init()
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_bind_memory2.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn bind_buffer_memory2(
        &self,
        bind_infos: &[BindBufferMemoryInfo<'_>],
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v11.bind_buffer_memory2)(
                self.handle,
                bind_infos.len() as _,
                if !bind_infos.is_empty() {
                    bind_infos.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
            .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_bind_memory2.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn bind_image_memory2(
        &self,
        bind_infos: &[BindImageMemoryInfo<'_>],
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v11.bind_image_memory2)(
                self.handle,
                bind_infos.len() as _,
                if !bind_infos.is_empty() {
                    bind_infos.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
            .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_device_group.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMask.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_device_mask(&self, command_buffer: CommandBuffer, device_mask: u32) {
        unsafe { (self.fp_v11.cmd_set_device_mask)(command_buffer, device_mask) }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_device_group.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBase.html>"]
    #[doc = r""]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe {
            (self.fp_v11.cmd_dispatch_base)(
                command_buffer,
                base_group_x,
                base_group_y,
                base_group_z,
                group_count_x,
                group_count_y,
                group_count_z,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_descriptor_update_template.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplate.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_descriptor_update_template(
        &self,
        create_info: &DescriptorUpdateTemplateCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<DescriptorUpdateTemplate> {
        unsafe {
            let mut descriptor_update_template = ::core::mem::MaybeUninit::uninit();
            (self.fp_v11.create_descriptor_update_template)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                descriptor_update_template.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], descriptor_update_template)
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_descriptor_update_template.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html>"]
    #[doc = r""]
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        descriptor_update_template: DescriptorUpdateTemplate,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fp_v11.destroy_descriptor_update_template)(
                self.handle,
                descriptor_update_template,
                allocator.as_ptr(),
            )
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_descriptor_update_template.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html>"]
    #[doc = r""]
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        data: &ffi::c_void,
    ) {
        unsafe {
            (self.fp_v11.update_descriptor_set_with_template)(
                self.handle,
                descriptor_set,
                descriptor_update_template,
                data,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_get_memory_requirements2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2.html>"]
    #[doc = r""]
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        info: &BufferMemoryRequirementsInfo2<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe {
            (self.fp_v11.get_buffer_memory_requirements2)(self.handle, info, memory_requirements)
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_get_memory_requirements2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2.html>"]
    #[doc = r""]
    pub unsafe fn get_image_memory_requirements2(
        &self,
        info: &ImageMemoryRequirementsInfo2<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe {
            (self.fp_v11.get_image_memory_requirements2)(self.handle, info, memory_requirements)
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_get_memory_requirements2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html>"]
    #[doc = r""]
    pub unsafe fn get_image_sparse_memory_requirements2_len(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2<'_>,
    ) -> u32 {
        unsafe {
            let mut sparse_memory_requirement_count = ::core::mem::MaybeUninit::uninit();
            (self.fp_v11.get_image_sparse_memory_requirements2)(
                self.handle,
                info,
                sparse_memory_requirement_count.as_mut_ptr(),
                core::ptr::null_mut(),
            );
            sparse_memory_requirement_count.assume_init()
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_get_memory_requirements2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html>"]
    #[doc = r""]
    pub unsafe fn get_image_sparse_memory_requirements2(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2<'_>,
        out: &mut [SparseImageMemoryRequirements2<'_>],
    ) {
        unsafe {
            let mut len = out.len() as _;
            (self.fp_v11.get_image_sparse_memory_requirements2)(
                self.handle,
                info,
                &mut len,
                out.as_mut_ptr(),
            )
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_sampler_ycbcr_conversion.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversion.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        create_info: &SamplerYcbcrConversionCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<SamplerYcbcrConversion> {
        unsafe {
            let mut ycbcr_conversion = ::core::mem::MaybeUninit::uninit();
            (self.fp_v11.create_sampler_ycbcr_conversion)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                ycbcr_conversion.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], ycbcr_conversion)
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_sampler_ycbcr_conversion.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html>"]
    #[doc = r""]
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        ycbcr_conversion: SamplerYcbcrConversion,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fp_v11.destroy_sampler_ycbcr_conversion)(
                self.handle,
                ycbcr_conversion,
                allocator.as_ptr(),
            )
        }
    }
    #[doc = "Requires Vulkan version 1.1"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue2.html>"]
    #[doc = r""]
    pub unsafe fn get_device_queue2(&self, queue_info: &DeviceQueueInfo2<'_>) -> Queue {
        unsafe {
            let mut queue = ::core::mem::MaybeUninit::uninit();
            (self.fp_v11.get_device_queue2)(self.handle, queue_info, queue.as_mut_ptr());
            queue.assume_init()
        }
    }
    #[doc = "Requires Vulkan version 1.1, otherwise provided by VK_KHR_maintenance3.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupport.html>"]
    #[doc = r""]
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        create_info: &DescriptorSetLayoutCreateInfo<'_>,
        support: &mut DescriptorSetLayoutSupport<'_>,
    ) {
        unsafe {
            (self.fp_v11.get_descriptor_set_layout_support)(self.handle, create_info, support)
        }
    }
}
#[derive(Clone, Copy)]
pub struct DeviceFpV12 {
    pub reset_query_pool: PFN_vkResetQueryPool,
    pub create_render_pass2: PFN_vkCreateRenderPass2,
    pub cmd_begin_render_pass2: PFN_vkCmdBeginRenderPass2,
    pub cmd_next_subpass2: PFN_vkCmdNextSubpass2,
    pub cmd_end_render_pass2: PFN_vkCmdEndRenderPass2,
    pub get_semaphore_counter_value: PFN_vkGetSemaphoreCounterValue,
    pub wait_semaphores: PFN_vkWaitSemaphores,
    pub signal_semaphore: PFN_vkSignalSemaphore,
    pub cmd_draw_indirect_count: PFN_vkCmdDrawIndirectCount,
    pub cmd_draw_indexed_indirect_count: PFN_vkCmdDrawIndexedIndirectCount,
    pub get_buffer_opaque_capture_address: PFN_vkGetBufferOpaqueCaptureAddress,
    pub get_buffer_device_address: PFN_vkGetBufferDeviceAddress,
    pub get_device_memory_opaque_capture_address: PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
}
unsafe impl Send for DeviceFpV12 {}
unsafe impl Sync for DeviceFpV12 {}
impl DeviceFpV12 {
    #[allow(unused_variables)]
    pub fn load(version: u32, f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void) -> Self {
        if version >= API_VERSION_1_2 {
            Self {
                reset_query_pool: unsafe {
                    unsafe extern "system" fn reset_query_pool(
                        _device: crate::vk::Device,
                        _query_pool: QueryPool,
                        _first_query: u32,
                        _query_count: u32,
                    ) {
                        panic!(concat!("failed to load ", stringify!(reset_query_pool),))
                    }
                    let f = f(c"vkResetQueryPool");
                    if f.is_null() {
                        reset_query_pool
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkResetQueryPool>(f)
                    }
                },
                create_render_pass2: unsafe {
                    unsafe extern "system" fn create_render_pass2(
                        _device: crate::vk::Device,
                        _create_info: *const RenderPassCreateInfo2,
                        _allocator: *const AllocationCallbacks,
                        _render_pass: *mut RenderPass,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(create_render_pass2),))
                    }
                    let f = f(c"vkCreateRenderPass2");
                    if f.is_null() {
                        create_render_pass2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateRenderPass2>(f)
                    }
                },
                cmd_begin_render_pass2: unsafe {
                    unsafe extern "system" fn cmd_begin_render_pass2(
                        _command_buffer: CommandBuffer,
                        _render_pass_begin: *const RenderPassBeginInfo,
                        _subpass_begin_info: *const SubpassBeginInfo,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_begin_render_pass2),
                        ))
                    }
                    let f = f(c"vkCmdBeginRenderPass2");
                    if f.is_null() {
                        cmd_begin_render_pass2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBeginRenderPass2>(f)
                    }
                },
                cmd_next_subpass2: unsafe {
                    unsafe extern "system" fn cmd_next_subpass2(
                        _command_buffer: CommandBuffer,
                        _subpass_begin_info: *const SubpassBeginInfo,
                        _subpass_end_info: *const SubpassEndInfo,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_next_subpass2),))
                    }
                    let f = f(c"vkCmdNextSubpass2");
                    if f.is_null() {
                        cmd_next_subpass2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdNextSubpass2>(f)
                    }
                },
                cmd_end_render_pass2: unsafe {
                    unsafe extern "system" fn cmd_end_render_pass2(
                        _command_buffer: CommandBuffer,
                        _subpass_end_info: *const SubpassEndInfo,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_end_render_pass2),))
                    }
                    let f = f(c"vkCmdEndRenderPass2");
                    if f.is_null() {
                        cmd_end_render_pass2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdEndRenderPass2>(f)
                    }
                },
                get_semaphore_counter_value: unsafe {
                    unsafe extern "system" fn get_semaphore_counter_value(
                        _device: crate::vk::Device,
                        _semaphore: Semaphore,
                        _value: *mut u64,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_semaphore_counter_value),
                        ))
                    }
                    let f = f(c"vkGetSemaphoreCounterValue");
                    if f.is_null() {
                        get_semaphore_counter_value
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetSemaphoreCounterValue>(
                            f,
                        )
                    }
                },
                wait_semaphores: unsafe {
                    unsafe extern "system" fn wait_semaphores(
                        _device: crate::vk::Device,
                        _wait_info: *const SemaphoreWaitInfo,
                        _timeout: u64,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(wait_semaphores),))
                    }
                    let f = f(c"vkWaitSemaphores");
                    if f.is_null() {
                        wait_semaphores
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkWaitSemaphores>(f)
                    }
                },
                signal_semaphore: unsafe {
                    unsafe extern "system" fn signal_semaphore(
                        _device: crate::vk::Device,
                        _signal_info: *const SemaphoreSignalInfo,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(signal_semaphore),))
                    }
                    let f = f(c"vkSignalSemaphore");
                    if f.is_null() {
                        signal_semaphore
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkSignalSemaphore>(f)
                    }
                },
                cmd_draw_indirect_count: unsafe {
                    unsafe extern "system" fn cmd_draw_indirect_count(
                        _command_buffer: CommandBuffer,
                        _buffer: Buffer,
                        _offset: DeviceSize,
                        _count_buffer: Buffer,
                        _count_buffer_offset: DeviceSize,
                        _max_draw_count: u32,
                        _stride: u32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_draw_indirect_count),
                        ))
                    }
                    let f = f(c"vkCmdDrawIndirectCount");
                    if f.is_null() {
                        cmd_draw_indirect_count
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdDrawIndirectCount>(f)
                    }
                },
                cmd_draw_indexed_indirect_count: unsafe {
                    unsafe extern "system" fn cmd_draw_indexed_indirect_count(
                        _command_buffer: CommandBuffer,
                        _buffer: Buffer,
                        _offset: DeviceSize,
                        _count_buffer: Buffer,
                        _count_buffer_offset: DeviceSize,
                        _max_draw_count: u32,
                        _stride: u32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_draw_indexed_indirect_count),
                        ))
                    }
                    let f = f(c"vkCmdDrawIndexedIndirectCount");
                    if f.is_null() {
                        cmd_draw_indexed_indirect_count
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdDrawIndexedIndirectCount,
                        >(f)
                    }
                },
                get_buffer_opaque_capture_address: unsafe {
                    unsafe extern "system" fn get_buffer_opaque_capture_address(
                        _device: crate::vk::Device,
                        _info: *const BufferDeviceAddressInfo,
                    ) -> u64 {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_buffer_opaque_capture_address),
                        ))
                    }
                    let f = f(c"vkGetBufferOpaqueCaptureAddress");
                    if f.is_null() {
                        get_buffer_opaque_capture_address
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetBufferOpaqueCaptureAddress,
                        >(f)
                    }
                },
                get_buffer_device_address: unsafe {
                    unsafe extern "system" fn get_buffer_device_address(
                        _device: crate::vk::Device,
                        _info: *const BufferDeviceAddressInfo,
                    ) -> DeviceAddress {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_buffer_device_address),
                        ))
                    }
                    let f = f(c"vkGetBufferDeviceAddress");
                    if f.is_null() {
                        get_buffer_device_address
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetBufferDeviceAddress>(
                            f,
                        )
                    }
                },
                get_device_memory_opaque_capture_address: unsafe {
                    unsafe extern "system" fn get_device_memory_opaque_capture_address(
                        _device: crate::vk::Device,
                        _info: *const DeviceMemoryOpaqueCaptureAddressInfo,
                    ) -> u64 {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_device_memory_opaque_capture_address),
                        ))
                    }
                    let f = f(c"vkGetDeviceMemoryOpaqueCaptureAddress");
                    if f.is_null() {
                        get_device_memory_opaque_capture_address
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
                        >(f)
                    }
                },
            }
        } else {
            Self {
                reset_query_pool: unsafe {
                    unsafe extern "system" fn reset_query_pool(
                        _device: crate::vk::Device,
                        _query_pool: QueryPool,
                        _first_query: u32,
                        _query_count: u32,
                    ) {
                        panic!(concat!("failed to load ", stringify!(reset_query_pool),))
                    }
                    let f = f(c"vkResetQueryPoolEXT");
                    if f.is_null() {
                        reset_query_pool
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkResetQueryPoolEXT>(f)
                    }
                },
                create_render_pass2: unsafe {
                    unsafe extern "system" fn create_render_pass2(
                        _device: crate::vk::Device,
                        _create_info: *const RenderPassCreateInfo2,
                        _allocator: *const AllocationCallbacks,
                        _render_pass: *mut RenderPass,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(create_render_pass2),))
                    }
                    let f = f(c"vkCreateRenderPass2KHR");
                    if f.is_null() {
                        create_render_pass2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreateRenderPass2KHR>(f)
                    }
                },
                cmd_begin_render_pass2: unsafe {
                    unsafe extern "system" fn cmd_begin_render_pass2(
                        _command_buffer: CommandBuffer,
                        _render_pass_begin: *const RenderPassBeginInfo,
                        _subpass_begin_info: *const SubpassBeginInfo,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_begin_render_pass2),
                        ))
                    }
                    let f = f(c"vkCmdBeginRenderPass2KHR");
                    if f.is_null() {
                        cmd_begin_render_pass2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBeginRenderPass2KHR>(
                            f,
                        )
                    }
                },
                cmd_next_subpass2: unsafe {
                    unsafe extern "system" fn cmd_next_subpass2(
                        _command_buffer: CommandBuffer,
                        _subpass_begin_info: *const SubpassBeginInfo,
                        _subpass_end_info: *const SubpassEndInfo,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_next_subpass2),))
                    }
                    let f = f(c"vkCmdNextSubpass2KHR");
                    if f.is_null() {
                        cmd_next_subpass2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdNextSubpass2KHR>(f)
                    }
                },
                cmd_end_render_pass2: unsafe {
                    unsafe extern "system" fn cmd_end_render_pass2(
                        _command_buffer: CommandBuffer,
                        _subpass_end_info: *const SubpassEndInfo,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_end_render_pass2),))
                    }
                    let f = f(c"vkCmdEndRenderPass2KHR");
                    if f.is_null() {
                        cmd_end_render_pass2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdEndRenderPass2KHR>(f)
                    }
                },
                get_semaphore_counter_value: unsafe {
                    unsafe extern "system" fn get_semaphore_counter_value(
                        _device: crate::vk::Device,
                        _semaphore: Semaphore,
                        _value: *mut u64,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_semaphore_counter_value),
                        ))
                    }
                    let f = f(c"vkGetSemaphoreCounterValueKHR");
                    if f.is_null() {
                        get_semaphore_counter_value
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetSemaphoreCounterValueKHR,
                        >(f)
                    }
                },
                wait_semaphores: unsafe {
                    unsafe extern "system" fn wait_semaphores(
                        _device: crate::vk::Device,
                        _wait_info: *const SemaphoreWaitInfo,
                        _timeout: u64,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(wait_semaphores),))
                    }
                    let f = f(c"vkWaitSemaphoresKHR");
                    if f.is_null() {
                        wait_semaphores
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkWaitSemaphoresKHR>(f)
                    }
                },
                signal_semaphore: unsafe {
                    unsafe extern "system" fn signal_semaphore(
                        _device: crate::vk::Device,
                        _signal_info: *const SemaphoreSignalInfo,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(signal_semaphore),))
                    }
                    let f = f(c"vkSignalSemaphoreKHR");
                    if f.is_null() {
                        signal_semaphore
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkSignalSemaphoreKHR>(f)
                    }
                },
                cmd_draw_indirect_count: unsafe {
                    unsafe extern "system" fn cmd_draw_indirect_count(
                        _command_buffer: CommandBuffer,
                        _buffer: Buffer,
                        _offset: DeviceSize,
                        _count_buffer: Buffer,
                        _count_buffer_offset: DeviceSize,
                        _max_draw_count: u32,
                        _stride: u32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_draw_indirect_count),
                        ))
                    }
                    let f = f(c"vkCmdDrawIndirectCountKHR");
                    if f.is_null() {
                        cmd_draw_indirect_count
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdDrawIndirectCountKHR>(
                            f,
                        )
                    }
                },
                cmd_draw_indexed_indirect_count: unsafe {
                    unsafe extern "system" fn cmd_draw_indexed_indirect_count(
                        _command_buffer: CommandBuffer,
                        _buffer: Buffer,
                        _offset: DeviceSize,
                        _count_buffer: Buffer,
                        _count_buffer_offset: DeviceSize,
                        _max_draw_count: u32,
                        _stride: u32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_draw_indexed_indirect_count),
                        ))
                    }
                    let f = f(c"vkCmdDrawIndexedIndirectCountKHR");
                    if f.is_null() {
                        cmd_draw_indexed_indirect_count
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdDrawIndexedIndirectCountKHR,
                        >(f)
                    }
                },
                get_buffer_opaque_capture_address: unsafe {
                    unsafe extern "system" fn get_buffer_opaque_capture_address(
                        _device: crate::vk::Device,
                        _info: *const BufferDeviceAddressInfo,
                    ) -> u64 {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_buffer_opaque_capture_address),
                        ))
                    }
                    let f = f(c"vkGetBufferOpaqueCaptureAddressKHR");
                    if f.is_null() {
                        get_buffer_opaque_capture_address
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetBufferOpaqueCaptureAddressKHR,
                        >(f)
                    }
                },
                get_buffer_device_address: unsafe {
                    unsafe extern "system" fn get_buffer_device_address(
                        _device: crate::vk::Device,
                        _info: *const BufferDeviceAddressInfo,
                    ) -> DeviceAddress {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_buffer_device_address),
                        ))
                    }
                    let f = f(c"vkGetBufferDeviceAddressKHR");
                    if f.is_null() {
                        get_buffer_device_address
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetBufferDeviceAddressKHR>(
                            f,
                        )
                    }
                },
                get_device_memory_opaque_capture_address: unsafe {
                    unsafe extern "system" fn get_device_memory_opaque_capture_address(
                        _device: crate::vk::Device,
                        _info: *const DeviceMemoryOpaqueCaptureAddressInfo,
                    ) -> u64 {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_device_memory_opaque_capture_address),
                        ))
                    }
                    let f = f(c"vkGetDeviceMemoryOpaqueCaptureAddressKHR");
                    if f.is_null() {
                        get_device_memory_opaque_capture_address
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR,
                        >(f)
                    }
                },
            }
        }
    }
}
impl<Ext> crate::Device<Ext> {
    #[doc = "Requires Vulkan version 1.2, otherwise provided by VK_EXT_host_query_reset.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html>"]
    #[doc = r""]
    pub unsafe fn reset_query_pool(
        &self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        unsafe { (self.fp_v12.reset_query_pool)(self.handle, query_pool, first_query, query_count) }
    }
    #[doc = "Requires Vulkan version 1.2, otherwise provided by VK_KHR_create_renderpass2.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[deprecated]
    pub unsafe fn create_render_pass2(
        &self,
        create_info: &RenderPassCreateInfo2<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<RenderPass> {
        unsafe {
            let mut render_pass = ::core::mem::MaybeUninit::uninit();
            (self.fp_v12.create_render_pass2)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                render_pass.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], render_pass)
        }
    }
    #[doc = "Requires Vulkan version 1.2, otherwise provided by VK_KHR_create_renderpass2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2.html>"]
    #[doc = r""]
    #[deprecated]
    pub unsafe fn cmd_begin_render_pass2(
        &self,
        command_buffer: CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo<'_>,
        subpass_begin_info: &SubpassBeginInfo<'_>,
    ) {
        unsafe {
            (self.fp_v12.cmd_begin_render_pass2)(
                command_buffer,
                render_pass_begin,
                subpass_begin_info,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.2, otherwise provided by VK_KHR_create_renderpass2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2.html>"]
    #[doc = r""]
    #[deprecated]
    pub unsafe fn cmd_next_subpass2(
        &self,
        command_buffer: CommandBuffer,
        subpass_begin_info: &SubpassBeginInfo<'_>,
        subpass_end_info: &SubpassEndInfo<'_>,
    ) {
        unsafe {
            (self.fp_v12.cmd_next_subpass2)(command_buffer, subpass_begin_info, subpass_end_info)
        }
    }
    #[doc = "Requires Vulkan version 1.2, otherwise provided by VK_KHR_create_renderpass2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2.html>"]
    #[doc = r""]
    #[deprecated]
    pub unsafe fn cmd_end_render_pass2(
        &self,
        command_buffer: CommandBuffer,
        subpass_end_info: &SubpassEndInfo<'_>,
    ) {
        unsafe { (self.fp_v12.cmd_end_render_pass2)(command_buffer, subpass_end_info) }
    }
    #[doc = "Requires Vulkan version 1.2, otherwise provided by VK_KHR_timeline_semaphore.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn get_semaphore_counter_value(&self, semaphore: Semaphore) -> crate::VkResult<u64> {
        unsafe {
            let mut value = ::core::mem::MaybeUninit::uninit();
            (self.fp_v12.get_semaphore_counter_value)(self.handle, semaphore, value.as_mut_ptr())
                .result_with_assume_init(&[crate::vk::Result::SUCCESS], value)
        }
    }
    #[doc = "Requires Vulkan version 1.2, otherwise provided by VK_KHR_timeline_semaphore.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = "* [`TIMEOUT`][1]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphores.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    #[doc = "[1]: Result::TIMEOUT"]
    pub unsafe fn wait_semaphores(
        &self,
        wait_info: &SemaphoreWaitInfo<'_>,
        timeout: u64,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v12.wait_semaphores)(self.handle, wait_info, timeout)
                .result(&[crate::vk::Result::SUCCESS, crate::vk::Result::TIMEOUT])
        }
    }
    #[doc = "Requires Vulkan version 1.2, otherwise provided by VK_KHR_timeline_semaphore.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphore.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn signal_semaphore(
        &self,
        signal_info: &SemaphoreSignalInfo<'_>,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v12.signal_semaphore)(self.handle, signal_info)
                .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "Requires Vulkan version 1.2, otherwise provided by VK_KHR_draw_indirect_count.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html>"]
    #[doc = r""]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn cmd_draw_indirect_count(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.fp_v12.cmd_draw_indirect_count)(
                command_buffer,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.2, otherwise provided by VK_KHR_draw_indirect_count.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html>"]
    #[doc = r""]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn cmd_draw_indexed_indirect_count(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.fp_v12.cmd_draw_indexed_indirect_count)(
                command_buffer,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.2, otherwise provided by VK_KHR_buffer_device_address.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddress.html>"]
    #[doc = r""]
    pub unsafe fn get_buffer_opaque_capture_address(
        &self,
        info: &BufferDeviceAddressInfo<'_>,
    ) -> u64 {
        unsafe { (self.fp_v12.get_buffer_opaque_capture_address)(self.handle, info) }
    }
    #[doc = "Requires Vulkan version 1.2, otherwise provided by VK_KHR_buffer_device_address.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html>"]
    #[doc = r""]
    pub unsafe fn get_buffer_device_address(
        &self,
        info: &BufferDeviceAddressInfo<'_>,
    ) -> DeviceAddress {
        unsafe { (self.fp_v12.get_buffer_device_address)(self.handle, info) }
    }
    #[doc = "Requires Vulkan version 1.2, otherwise provided by VK_KHR_buffer_device_address.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddress.html>"]
    #[doc = r""]
    pub unsafe fn get_device_memory_opaque_capture_address(
        &self,
        info: &DeviceMemoryOpaqueCaptureAddressInfo<'_>,
    ) -> u64 {
        unsafe { (self.fp_v12.get_device_memory_opaque_capture_address)(self.handle, info) }
    }
}
#[derive(Clone, Copy)]
pub struct DeviceFpV13 {
    pub get_device_buffer_memory_requirements: PFN_vkGetDeviceBufferMemoryRequirements,
    pub get_device_image_memory_requirements: PFN_vkGetDeviceImageMemoryRequirements,
    pub get_device_image_sparse_memory_requirements: PFN_vkGetDeviceImageSparseMemoryRequirements,
    pub cmd_set_cull_mode: PFN_vkCmdSetCullMode,
    pub cmd_set_front_face: PFN_vkCmdSetFrontFace,
    pub cmd_set_primitive_topology: PFN_vkCmdSetPrimitiveTopology,
    pub cmd_set_viewport_with_count: PFN_vkCmdSetViewportWithCount,
    pub cmd_set_scissor_with_count: PFN_vkCmdSetScissorWithCount,
    pub cmd_bind_vertex_buffers2: PFN_vkCmdBindVertexBuffers2,
    pub cmd_set_depth_test_enable: PFN_vkCmdSetDepthTestEnable,
    pub cmd_set_depth_write_enable: PFN_vkCmdSetDepthWriteEnable,
    pub cmd_set_depth_compare_op: PFN_vkCmdSetDepthCompareOp,
    pub cmd_set_depth_bounds_test_enable: PFN_vkCmdSetDepthBoundsTestEnable,
    pub cmd_set_stencil_test_enable: PFN_vkCmdSetStencilTestEnable,
    pub cmd_set_stencil_op: PFN_vkCmdSetStencilOp,
    pub cmd_set_rasterizer_discard_enable: PFN_vkCmdSetRasterizerDiscardEnable,
    pub cmd_set_depth_bias_enable: PFN_vkCmdSetDepthBiasEnable,
    pub cmd_set_primitive_restart_enable: PFN_vkCmdSetPrimitiveRestartEnable,
    pub create_private_data_slot: PFN_vkCreatePrivateDataSlot,
    pub destroy_private_data_slot: PFN_vkDestroyPrivateDataSlot,
    pub set_private_data: PFN_vkSetPrivateData,
    pub get_private_data: PFN_vkGetPrivateData,
    pub cmd_copy_buffer2: PFN_vkCmdCopyBuffer2,
    pub cmd_copy_image2: PFN_vkCmdCopyImage2,
    pub cmd_blit_image2: PFN_vkCmdBlitImage2,
    pub cmd_copy_buffer_to_image2: PFN_vkCmdCopyBufferToImage2,
    pub cmd_copy_image_to_buffer2: PFN_vkCmdCopyImageToBuffer2,
    pub cmd_resolve_image2: PFN_vkCmdResolveImage2,
    pub cmd_set_event2: PFN_vkCmdSetEvent2,
    pub cmd_reset_event2: PFN_vkCmdResetEvent2,
    pub cmd_wait_events2: PFN_vkCmdWaitEvents2,
    pub cmd_pipeline_barrier2: PFN_vkCmdPipelineBarrier2,
    pub queue_submit2: PFN_vkQueueSubmit2,
    pub cmd_write_timestamp2: PFN_vkCmdWriteTimestamp2,
    pub cmd_begin_rendering: PFN_vkCmdBeginRendering,
    pub cmd_end_rendering: PFN_vkCmdEndRendering,
}
unsafe impl Send for DeviceFpV13 {}
unsafe impl Sync for DeviceFpV13 {}
impl DeviceFpV13 {
    #[allow(unused_variables)]
    pub fn load(version: u32, f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void) -> Self {
        if version >= API_VERSION_1_3 {
            Self {
                get_device_buffer_memory_requirements: unsafe {
                    unsafe extern "system" fn get_device_buffer_memory_requirements(
                        _device: crate::vk::Device,
                        _info: *const DeviceBufferMemoryRequirements,
                        _memory_requirements: *mut MemoryRequirements2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_device_buffer_memory_requirements),
                        ))
                    }
                    let f = f(c"vkGetDeviceBufferMemoryRequirements");
                    if f.is_null() {
                        get_device_buffer_memory_requirements
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetDeviceBufferMemoryRequirements,
                        >(f)
                    }
                },
                get_device_image_memory_requirements: unsafe {
                    unsafe extern "system" fn get_device_image_memory_requirements(
                        _device: crate::vk::Device,
                        _info: *const DeviceImageMemoryRequirements,
                        _memory_requirements: *mut MemoryRequirements2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_device_image_memory_requirements),
                        ))
                    }
                    let f = f(c"vkGetDeviceImageMemoryRequirements");
                    if f.is_null() {
                        get_device_image_memory_requirements
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetDeviceImageMemoryRequirements,
                        >(f)
                    }
                },
                get_device_image_sparse_memory_requirements: unsafe {
                    unsafe extern "system" fn get_device_image_sparse_memory_requirements(
                        _device: crate::vk::Device,
                        _info: *const DeviceImageMemoryRequirements,
                        _sparse_memory_requirement_count: *mut u32,
                        _sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_device_image_sparse_memory_requirements),
                        ))
                    }
                    let f = f(c"vkGetDeviceImageSparseMemoryRequirements");
                    if f.is_null() {
                        get_device_image_sparse_memory_requirements
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetDeviceImageSparseMemoryRequirements,
                        >(f)
                    }
                },
                cmd_set_cull_mode: unsafe {
                    unsafe extern "system" fn cmd_set_cull_mode(
                        _command_buffer: CommandBuffer,
                        _cull_mode: CullModeFlags,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_set_cull_mode),))
                    }
                    let f = f(c"vkCmdSetCullMode");
                    if f.is_null() {
                        cmd_set_cull_mode
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetCullMode>(f)
                    }
                },
                cmd_set_front_face: unsafe {
                    unsafe extern "system" fn cmd_set_front_face(
                        _command_buffer: CommandBuffer,
                        _front_face: FrontFace,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_set_front_face),))
                    }
                    let f = f(c"vkCmdSetFrontFace");
                    if f.is_null() {
                        cmd_set_front_face
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetFrontFace>(f)
                    }
                },
                cmd_set_primitive_topology: unsafe {
                    unsafe extern "system" fn cmd_set_primitive_topology(
                        _command_buffer: CommandBuffer,
                        _primitive_topology: PrimitiveTopology,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_primitive_topology),
                        ))
                    }
                    let f = f(c"vkCmdSetPrimitiveTopology");
                    if f.is_null() {
                        cmd_set_primitive_topology
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetPrimitiveTopology>(
                            f,
                        )
                    }
                },
                cmd_set_viewport_with_count: unsafe {
                    unsafe extern "system" fn cmd_set_viewport_with_count(
                        _command_buffer: CommandBuffer,
                        _viewport_count: u32,
                        _viewports: *const Viewport,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_viewport_with_count),
                        ))
                    }
                    let f = f(c"vkCmdSetViewportWithCount");
                    if f.is_null() {
                        cmd_set_viewport_with_count
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetViewportWithCount>(
                            f,
                        )
                    }
                },
                cmd_set_scissor_with_count: unsafe {
                    unsafe extern "system" fn cmd_set_scissor_with_count(
                        _command_buffer: CommandBuffer,
                        _scissor_count: u32,
                        _scissors: *const Rect2D,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_scissor_with_count),
                        ))
                    }
                    let f = f(c"vkCmdSetScissorWithCount");
                    if f.is_null() {
                        cmd_set_scissor_with_count
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetScissorWithCount>(
                            f,
                        )
                    }
                },
                cmd_bind_vertex_buffers2: unsafe {
                    unsafe extern "system" fn cmd_bind_vertex_buffers2(
                        _command_buffer: CommandBuffer,
                        _first_binding: u32,
                        _binding_count: u32,
                        _buffers: *const Buffer,
                        _offsets: *const DeviceSize,
                        _sizes: *const DeviceSize,
                        _strides: *const DeviceSize,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_bind_vertex_buffers2),
                        ))
                    }
                    let f = f(c"vkCmdBindVertexBuffers2");
                    if f.is_null() {
                        cmd_bind_vertex_buffers2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBindVertexBuffers2>(f)
                    }
                },
                cmd_set_depth_test_enable: unsafe {
                    unsafe extern "system" fn cmd_set_depth_test_enable(
                        _command_buffer: CommandBuffer,
                        _depth_test_enable: Bool32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_depth_test_enable),
                        ))
                    }
                    let f = f(c"vkCmdSetDepthTestEnable");
                    if f.is_null() {
                        cmd_set_depth_test_enable
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetDepthTestEnable>(f)
                    }
                },
                cmd_set_depth_write_enable: unsafe {
                    unsafe extern "system" fn cmd_set_depth_write_enable(
                        _command_buffer: CommandBuffer,
                        _depth_write_enable: Bool32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_depth_write_enable),
                        ))
                    }
                    let f = f(c"vkCmdSetDepthWriteEnable");
                    if f.is_null() {
                        cmd_set_depth_write_enable
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetDepthWriteEnable>(
                            f,
                        )
                    }
                },
                cmd_set_depth_compare_op: unsafe {
                    unsafe extern "system" fn cmd_set_depth_compare_op(
                        _command_buffer: CommandBuffer,
                        _depth_compare_op: CompareOp,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_depth_compare_op),
                        ))
                    }
                    let f = f(c"vkCmdSetDepthCompareOp");
                    if f.is_null() {
                        cmd_set_depth_compare_op
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetDepthCompareOp>(f)
                    }
                },
                cmd_set_depth_bounds_test_enable: unsafe {
                    unsafe extern "system" fn cmd_set_depth_bounds_test_enable(
                        _command_buffer: CommandBuffer,
                        _depth_bounds_test_enable: Bool32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_depth_bounds_test_enable),
                        ))
                    }
                    let f = f(c"vkCmdSetDepthBoundsTestEnable");
                    if f.is_null() {
                        cmd_set_depth_bounds_test_enable
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdSetDepthBoundsTestEnable,
                        >(f)
                    }
                },
                cmd_set_stencil_test_enable: unsafe {
                    unsafe extern "system" fn cmd_set_stencil_test_enable(
                        _command_buffer: CommandBuffer,
                        _stencil_test_enable: Bool32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_stencil_test_enable),
                        ))
                    }
                    let f = f(c"vkCmdSetStencilTestEnable");
                    if f.is_null() {
                        cmd_set_stencil_test_enable
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetStencilTestEnable>(
                            f,
                        )
                    }
                },
                cmd_set_stencil_op: unsafe {
                    unsafe extern "system" fn cmd_set_stencil_op(
                        _command_buffer: CommandBuffer,
                        _face_mask: StencilFaceFlags,
                        _fail_op: StencilOp,
                        _pass_op: StencilOp,
                        _depth_fail_op: StencilOp,
                        _compare_op: CompareOp,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_set_stencil_op),))
                    }
                    let f = f(c"vkCmdSetStencilOp");
                    if f.is_null() {
                        cmd_set_stencil_op
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetStencilOp>(f)
                    }
                },
                cmd_set_rasterizer_discard_enable: unsafe {
                    unsafe extern "system" fn cmd_set_rasterizer_discard_enable(
                        _command_buffer: CommandBuffer,
                        _rasterizer_discard_enable: Bool32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_rasterizer_discard_enable),
                        ))
                    }
                    let f = f(c"vkCmdSetRasterizerDiscardEnable");
                    if f.is_null() {
                        cmd_set_rasterizer_discard_enable
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdSetRasterizerDiscardEnable,
                        >(f)
                    }
                },
                cmd_set_depth_bias_enable: unsafe {
                    unsafe extern "system" fn cmd_set_depth_bias_enable(
                        _command_buffer: CommandBuffer,
                        _depth_bias_enable: Bool32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_depth_bias_enable),
                        ))
                    }
                    let f = f(c"vkCmdSetDepthBiasEnable");
                    if f.is_null() {
                        cmd_set_depth_bias_enable
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetDepthBiasEnable>(f)
                    }
                },
                cmd_set_primitive_restart_enable: unsafe {
                    unsafe extern "system" fn cmd_set_primitive_restart_enable(
                        _command_buffer: CommandBuffer,
                        _primitive_restart_enable: Bool32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_primitive_restart_enable),
                        ))
                    }
                    let f = f(c"vkCmdSetPrimitiveRestartEnable");
                    if f.is_null() {
                        cmd_set_primitive_restart_enable
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdSetPrimitiveRestartEnable,
                        >(f)
                    }
                },
                create_private_data_slot: unsafe {
                    unsafe extern "system" fn create_private_data_slot(
                        _device: crate::vk::Device,
                        _create_info: *const PrivateDataSlotCreateInfo,
                        _allocator: *const AllocationCallbacks,
                        _private_data_slot: *mut PrivateDataSlot,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(create_private_data_slot),
                        ))
                    }
                    let f = f(c"vkCreatePrivateDataSlot");
                    if f.is_null() {
                        create_private_data_slot
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreatePrivateDataSlot>(f)
                    }
                },
                destroy_private_data_slot: unsafe {
                    unsafe extern "system" fn destroy_private_data_slot(
                        _device: crate::vk::Device,
                        _private_data_slot: PrivateDataSlot,
                        _allocator: *const AllocationCallbacks,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(destroy_private_data_slot),
                        ))
                    }
                    let f = f(c"vkDestroyPrivateDataSlot");
                    if f.is_null() {
                        destroy_private_data_slot
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyPrivateDataSlot>(
                            f,
                        )
                    }
                },
                set_private_data: unsafe {
                    unsafe extern "system" fn set_private_data(
                        _device: crate::vk::Device,
                        _object_type: ObjectType,
                        _object_handle: u64,
                        _private_data_slot: PrivateDataSlot,
                        _data: u64,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(set_private_data),))
                    }
                    let f = f(c"vkSetPrivateData");
                    if f.is_null() {
                        set_private_data
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkSetPrivateData>(f)
                    }
                },
                get_private_data: unsafe {
                    unsafe extern "system" fn get_private_data(
                        _device: crate::vk::Device,
                        _object_type: ObjectType,
                        _object_handle: u64,
                        _private_data_slot: PrivateDataSlot,
                        _data: *mut u64,
                    ) {
                        panic!(concat!("failed to load ", stringify!(get_private_data),))
                    }
                    let f = f(c"vkGetPrivateData");
                    if f.is_null() {
                        get_private_data
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetPrivateData>(f)
                    }
                },
                cmd_copy_buffer2: unsafe {
                    unsafe extern "system" fn cmd_copy_buffer2(
                        _command_buffer: CommandBuffer,
                        _copy_buffer_info: *const CopyBufferInfo2,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_copy_buffer2),))
                    }
                    let f = f(c"vkCmdCopyBuffer2");
                    if f.is_null() {
                        cmd_copy_buffer2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdCopyBuffer2>(f)
                    }
                },
                cmd_copy_image2: unsafe {
                    unsafe extern "system" fn cmd_copy_image2(
                        _command_buffer: CommandBuffer,
                        _copy_image_info: *const CopyImageInfo2,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_copy_image2),))
                    }
                    let f = f(c"vkCmdCopyImage2");
                    if f.is_null() {
                        cmd_copy_image2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdCopyImage2>(f)
                    }
                },
                cmd_blit_image2: unsafe {
                    unsafe extern "system" fn cmd_blit_image2(
                        _command_buffer: CommandBuffer,
                        _blit_image_info: *const BlitImageInfo2,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_blit_image2),))
                    }
                    let f = f(c"vkCmdBlitImage2");
                    if f.is_null() {
                        cmd_blit_image2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBlitImage2>(f)
                    }
                },
                cmd_copy_buffer_to_image2: unsafe {
                    unsafe extern "system" fn cmd_copy_buffer_to_image2(
                        _command_buffer: CommandBuffer,
                        _copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_copy_buffer_to_image2),
                        ))
                    }
                    let f = f(c"vkCmdCopyBufferToImage2");
                    if f.is_null() {
                        cmd_copy_buffer_to_image2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdCopyBufferToImage2>(f)
                    }
                },
                cmd_copy_image_to_buffer2: unsafe {
                    unsafe extern "system" fn cmd_copy_image_to_buffer2(
                        _command_buffer: CommandBuffer,
                        _copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_copy_image_to_buffer2),
                        ))
                    }
                    let f = f(c"vkCmdCopyImageToBuffer2");
                    if f.is_null() {
                        cmd_copy_image_to_buffer2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdCopyImageToBuffer2>(f)
                    }
                },
                cmd_resolve_image2: unsafe {
                    unsafe extern "system" fn cmd_resolve_image2(
                        _command_buffer: CommandBuffer,
                        _resolve_image_info: *const ResolveImageInfo2,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_resolve_image2),))
                    }
                    let f = f(c"vkCmdResolveImage2");
                    if f.is_null() {
                        cmd_resolve_image2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdResolveImage2>(f)
                    }
                },
                cmd_set_event2: unsafe {
                    unsafe extern "system" fn cmd_set_event2(
                        _command_buffer: CommandBuffer,
                        _event: Event,
                        _dependency_info: *const DependencyInfo,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_set_event2),))
                    }
                    let f = f(c"vkCmdSetEvent2");
                    if f.is_null() {
                        cmd_set_event2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetEvent2>(f)
                    }
                },
                cmd_reset_event2: unsafe {
                    unsafe extern "system" fn cmd_reset_event2(
                        _command_buffer: CommandBuffer,
                        _event: Event,
                        _stage_mask: PipelineStageFlags2,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_reset_event2),))
                    }
                    let f = f(c"vkCmdResetEvent2");
                    if f.is_null() {
                        cmd_reset_event2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdResetEvent2>(f)
                    }
                },
                cmd_wait_events2: unsafe {
                    unsafe extern "system" fn cmd_wait_events2(
                        _command_buffer: CommandBuffer,
                        _event_count: u32,
                        _events: *const Event,
                        _dependency_infos: *const DependencyInfo,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_wait_events2),))
                    }
                    let f = f(c"vkCmdWaitEvents2");
                    if f.is_null() {
                        cmd_wait_events2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdWaitEvents2>(f)
                    }
                },
                cmd_pipeline_barrier2: unsafe {
                    unsafe extern "system" fn cmd_pipeline_barrier2(
                        _command_buffer: CommandBuffer,
                        _dependency_info: *const DependencyInfo,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_pipeline_barrier2),
                        ))
                    }
                    let f = f(c"vkCmdPipelineBarrier2");
                    if f.is_null() {
                        cmd_pipeline_barrier2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdPipelineBarrier2>(f)
                    }
                },
                queue_submit2: unsafe {
                    unsafe extern "system" fn queue_submit2(
                        _queue: Queue,
                        _submit_count: u32,
                        _submits: *const SubmitInfo2,
                        _fence: Fence,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(queue_submit2),))
                    }
                    let f = f(c"vkQueueSubmit2");
                    if f.is_null() {
                        queue_submit2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkQueueSubmit2>(f)
                    }
                },
                cmd_write_timestamp2: unsafe {
                    unsafe extern "system" fn cmd_write_timestamp2(
                        _command_buffer: CommandBuffer,
                        _stage: PipelineStageFlags2,
                        _query_pool: QueryPool,
                        _query: u32,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_write_timestamp2),))
                    }
                    let f = f(c"vkCmdWriteTimestamp2");
                    if f.is_null() {
                        cmd_write_timestamp2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdWriteTimestamp2>(f)
                    }
                },
                cmd_begin_rendering: unsafe {
                    unsafe extern "system" fn cmd_begin_rendering(
                        _command_buffer: CommandBuffer,
                        _rendering_info: *const RenderingInfo,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_begin_rendering),))
                    }
                    let f = f(c"vkCmdBeginRendering");
                    if f.is_null() {
                        cmd_begin_rendering
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBeginRendering>(f)
                    }
                },
                cmd_end_rendering: unsafe {
                    unsafe extern "system" fn cmd_end_rendering(_command_buffer: CommandBuffer) {
                        panic!(concat!("failed to load ", stringify!(cmd_end_rendering),))
                    }
                    let f = f(c"vkCmdEndRendering");
                    if f.is_null() {
                        cmd_end_rendering
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdEndRendering>(f)
                    }
                },
            }
        } else {
            Self {
                get_device_buffer_memory_requirements: unsafe {
                    unsafe extern "system" fn get_device_buffer_memory_requirements(
                        _device: crate::vk::Device,
                        _info: *const DeviceBufferMemoryRequirements,
                        _memory_requirements: *mut MemoryRequirements2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_device_buffer_memory_requirements),
                        ))
                    }
                    let f = f(c"vkGetDeviceBufferMemoryRequirementsKHR");
                    if f.is_null() {
                        get_device_buffer_memory_requirements
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetDeviceBufferMemoryRequirementsKHR,
                        >(f)
                    }
                },
                get_device_image_memory_requirements: unsafe {
                    unsafe extern "system" fn get_device_image_memory_requirements(
                        _device: crate::vk::Device,
                        _info: *const DeviceImageMemoryRequirements,
                        _memory_requirements: *mut MemoryRequirements2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_device_image_memory_requirements),
                        ))
                    }
                    let f = f(c"vkGetDeviceImageMemoryRequirementsKHR");
                    if f.is_null() {
                        get_device_image_memory_requirements
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetDeviceImageMemoryRequirementsKHR,
                        >(f)
                    }
                },
                get_device_image_sparse_memory_requirements: unsafe {
                    unsafe extern "system" fn get_device_image_sparse_memory_requirements(
                        _device: crate::vk::Device,
                        _info: *const DeviceImageMemoryRequirements,
                        _sparse_memory_requirement_count: *mut u32,
                        _sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_device_image_sparse_memory_requirements),
                        ))
                    }
                    let f = f(c"vkGetDeviceImageSparseMemoryRequirementsKHR");
                    if f.is_null() {
                        get_device_image_sparse_memory_requirements
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetDeviceImageSparseMemoryRequirementsKHR,
                        >(f)
                    }
                },
                cmd_set_cull_mode: unsafe {
                    unsafe extern "system" fn cmd_set_cull_mode(
                        _command_buffer: CommandBuffer,
                        _cull_mode: CullModeFlags,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_set_cull_mode),))
                    }
                    let f = f(c"vkCmdSetCullModeEXT");
                    if f.is_null() {
                        cmd_set_cull_mode
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetCullModeEXT>(f)
                    }
                },
                cmd_set_front_face: unsafe {
                    unsafe extern "system" fn cmd_set_front_face(
                        _command_buffer: CommandBuffer,
                        _front_face: FrontFace,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_set_front_face),))
                    }
                    let f = f(c"vkCmdSetFrontFaceEXT");
                    if f.is_null() {
                        cmd_set_front_face
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetFrontFaceEXT>(f)
                    }
                },
                cmd_set_primitive_topology: unsafe {
                    unsafe extern "system" fn cmd_set_primitive_topology(
                        _command_buffer: CommandBuffer,
                        _primitive_topology: PrimitiveTopology,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_primitive_topology),
                        ))
                    }
                    let f = f(c"vkCmdSetPrimitiveTopologyEXT");
                    if f.is_null() {
                        cmd_set_primitive_topology
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetPrimitiveTopologyEXT>(
                            f,
                        )
                    }
                },
                cmd_set_viewport_with_count: unsafe {
                    unsafe extern "system" fn cmd_set_viewport_with_count(
                        _command_buffer: CommandBuffer,
                        _viewport_count: u32,
                        _viewports: *const Viewport,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_viewport_with_count),
                        ))
                    }
                    let f = f(c"vkCmdSetViewportWithCountEXT");
                    if f.is_null() {
                        cmd_set_viewport_with_count
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetViewportWithCountEXT>(
                            f,
                        )
                    }
                },
                cmd_set_scissor_with_count: unsafe {
                    unsafe extern "system" fn cmd_set_scissor_with_count(
                        _command_buffer: CommandBuffer,
                        _scissor_count: u32,
                        _scissors: *const Rect2D,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_scissor_with_count),
                        ))
                    }
                    let f = f(c"vkCmdSetScissorWithCountEXT");
                    if f.is_null() {
                        cmd_set_scissor_with_count
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetScissorWithCountEXT>(
                            f,
                        )
                    }
                },
                cmd_bind_vertex_buffers2: unsafe {
                    unsafe extern "system" fn cmd_bind_vertex_buffers2(
                        _command_buffer: CommandBuffer,
                        _first_binding: u32,
                        _binding_count: u32,
                        _buffers: *const Buffer,
                        _offsets: *const DeviceSize,
                        _sizes: *const DeviceSize,
                        _strides: *const DeviceSize,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_bind_vertex_buffers2),
                        ))
                    }
                    let f = f(c"vkCmdBindVertexBuffers2EXT");
                    if f.is_null() {
                        cmd_bind_vertex_buffers2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBindVertexBuffers2EXT>(
                            f,
                        )
                    }
                },
                cmd_set_depth_test_enable: unsafe {
                    unsafe extern "system" fn cmd_set_depth_test_enable(
                        _command_buffer: CommandBuffer,
                        _depth_test_enable: Bool32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_depth_test_enable),
                        ))
                    }
                    let f = f(c"vkCmdSetDepthTestEnableEXT");
                    if f.is_null() {
                        cmd_set_depth_test_enable
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetDepthTestEnableEXT>(
                            f,
                        )
                    }
                },
                cmd_set_depth_write_enable: unsafe {
                    unsafe extern "system" fn cmd_set_depth_write_enable(
                        _command_buffer: CommandBuffer,
                        _depth_write_enable: Bool32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_depth_write_enable),
                        ))
                    }
                    let f = f(c"vkCmdSetDepthWriteEnableEXT");
                    if f.is_null() {
                        cmd_set_depth_write_enable
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetDepthWriteEnableEXT>(
                            f,
                        )
                    }
                },
                cmd_set_depth_compare_op: unsafe {
                    unsafe extern "system" fn cmd_set_depth_compare_op(
                        _command_buffer: CommandBuffer,
                        _depth_compare_op: CompareOp,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_depth_compare_op),
                        ))
                    }
                    let f = f(c"vkCmdSetDepthCompareOpEXT");
                    if f.is_null() {
                        cmd_set_depth_compare_op
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetDepthCompareOpEXT>(
                            f,
                        )
                    }
                },
                cmd_set_depth_bounds_test_enable: unsafe {
                    unsafe extern "system" fn cmd_set_depth_bounds_test_enable(
                        _command_buffer: CommandBuffer,
                        _depth_bounds_test_enable: Bool32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_depth_bounds_test_enable),
                        ))
                    }
                    let f = f(c"vkCmdSetDepthBoundsTestEnableEXT");
                    if f.is_null() {
                        cmd_set_depth_bounds_test_enable
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdSetDepthBoundsTestEnableEXT,
                        >(f)
                    }
                },
                cmd_set_stencil_test_enable: unsafe {
                    unsafe extern "system" fn cmd_set_stencil_test_enable(
                        _command_buffer: CommandBuffer,
                        _stencil_test_enable: Bool32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_stencil_test_enable),
                        ))
                    }
                    let f = f(c"vkCmdSetStencilTestEnableEXT");
                    if f.is_null() {
                        cmd_set_stencil_test_enable
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetStencilTestEnableEXT>(
                            f,
                        )
                    }
                },
                cmd_set_stencil_op: unsafe {
                    unsafe extern "system" fn cmd_set_stencil_op(
                        _command_buffer: CommandBuffer,
                        _face_mask: StencilFaceFlags,
                        _fail_op: StencilOp,
                        _pass_op: StencilOp,
                        _depth_fail_op: StencilOp,
                        _compare_op: CompareOp,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_set_stencil_op),))
                    }
                    let f = f(c"vkCmdSetStencilOpEXT");
                    if f.is_null() {
                        cmd_set_stencil_op
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetStencilOpEXT>(f)
                    }
                },
                cmd_set_rasterizer_discard_enable: unsafe {
                    unsafe extern "system" fn cmd_set_rasterizer_discard_enable(
                        _command_buffer: CommandBuffer,
                        _rasterizer_discard_enable: Bool32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_rasterizer_discard_enable),
                        ))
                    }
                    let f = f(c"vkCmdSetRasterizerDiscardEnableEXT");
                    if f.is_null() {
                        cmd_set_rasterizer_discard_enable
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdSetRasterizerDiscardEnableEXT,
                        >(f)
                    }
                },
                cmd_set_depth_bias_enable: unsafe {
                    unsafe extern "system" fn cmd_set_depth_bias_enable(
                        _command_buffer: CommandBuffer,
                        _depth_bias_enable: Bool32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_depth_bias_enable),
                        ))
                    }
                    let f = f(c"vkCmdSetDepthBiasEnableEXT");
                    if f.is_null() {
                        cmd_set_depth_bias_enable
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetDepthBiasEnableEXT>(
                            f,
                        )
                    }
                },
                cmd_set_primitive_restart_enable: unsafe {
                    unsafe extern "system" fn cmd_set_primitive_restart_enable(
                        _command_buffer: CommandBuffer,
                        _primitive_restart_enable: Bool32,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_primitive_restart_enable),
                        ))
                    }
                    let f = f(c"vkCmdSetPrimitiveRestartEnableEXT");
                    if f.is_null() {
                        cmd_set_primitive_restart_enable
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdSetPrimitiveRestartEnableEXT,
                        >(f)
                    }
                },
                create_private_data_slot: unsafe {
                    unsafe extern "system" fn create_private_data_slot(
                        _device: crate::vk::Device,
                        _create_info: *const PrivateDataSlotCreateInfo,
                        _allocator: *const AllocationCallbacks,
                        _private_data_slot: *mut PrivateDataSlot,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(create_private_data_slot),
                        ))
                    }
                    let f = f(c"vkCreatePrivateDataSlotEXT");
                    if f.is_null() {
                        create_private_data_slot
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCreatePrivateDataSlotEXT>(
                            f,
                        )
                    }
                },
                destroy_private_data_slot: unsafe {
                    unsafe extern "system" fn destroy_private_data_slot(
                        _device: crate::vk::Device,
                        _private_data_slot: PrivateDataSlot,
                        _allocator: *const AllocationCallbacks,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(destroy_private_data_slot),
                        ))
                    }
                    let f = f(c"vkDestroyPrivateDataSlotEXT");
                    if f.is_null() {
                        destroy_private_data_slot
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkDestroyPrivateDataSlotEXT>(
                            f,
                        )
                    }
                },
                set_private_data: unsafe {
                    unsafe extern "system" fn set_private_data(
                        _device: crate::vk::Device,
                        _object_type: ObjectType,
                        _object_handle: u64,
                        _private_data_slot: PrivateDataSlot,
                        _data: u64,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(set_private_data),))
                    }
                    let f = f(c"vkSetPrivateDataEXT");
                    if f.is_null() {
                        set_private_data
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkSetPrivateDataEXT>(f)
                    }
                },
                get_private_data: unsafe {
                    unsafe extern "system" fn get_private_data(
                        _device: crate::vk::Device,
                        _object_type: ObjectType,
                        _object_handle: u64,
                        _private_data_slot: PrivateDataSlot,
                        _data: *mut u64,
                    ) {
                        panic!(concat!("failed to load ", stringify!(get_private_data),))
                    }
                    let f = f(c"vkGetPrivateDataEXT");
                    if f.is_null() {
                        get_private_data
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetPrivateDataEXT>(f)
                    }
                },
                cmd_copy_buffer2: unsafe {
                    unsafe extern "system" fn cmd_copy_buffer2(
                        _command_buffer: CommandBuffer,
                        _copy_buffer_info: *const CopyBufferInfo2,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_copy_buffer2),))
                    }
                    let f = f(c"vkCmdCopyBuffer2KHR");
                    if f.is_null() {
                        cmd_copy_buffer2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdCopyBuffer2KHR>(f)
                    }
                },
                cmd_copy_image2: unsafe {
                    unsafe extern "system" fn cmd_copy_image2(
                        _command_buffer: CommandBuffer,
                        _copy_image_info: *const CopyImageInfo2,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_copy_image2),))
                    }
                    let f = f(c"vkCmdCopyImage2KHR");
                    if f.is_null() {
                        cmd_copy_image2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdCopyImage2KHR>(f)
                    }
                },
                cmd_blit_image2: unsafe {
                    unsafe extern "system" fn cmd_blit_image2(
                        _command_buffer: CommandBuffer,
                        _blit_image_info: *const BlitImageInfo2,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_blit_image2),))
                    }
                    let f = f(c"vkCmdBlitImage2KHR");
                    if f.is_null() {
                        cmd_blit_image2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBlitImage2KHR>(f)
                    }
                },
                cmd_copy_buffer_to_image2: unsafe {
                    unsafe extern "system" fn cmd_copy_buffer_to_image2(
                        _command_buffer: CommandBuffer,
                        _copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_copy_buffer_to_image2),
                        ))
                    }
                    let f = f(c"vkCmdCopyBufferToImage2KHR");
                    if f.is_null() {
                        cmd_copy_buffer_to_image2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdCopyBufferToImage2KHR>(
                            f,
                        )
                    }
                },
                cmd_copy_image_to_buffer2: unsafe {
                    unsafe extern "system" fn cmd_copy_image_to_buffer2(
                        _command_buffer: CommandBuffer,
                        _copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_copy_image_to_buffer2),
                        ))
                    }
                    let f = f(c"vkCmdCopyImageToBuffer2KHR");
                    if f.is_null() {
                        cmd_copy_image_to_buffer2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdCopyImageToBuffer2KHR>(
                            f,
                        )
                    }
                },
                cmd_resolve_image2: unsafe {
                    unsafe extern "system" fn cmd_resolve_image2(
                        _command_buffer: CommandBuffer,
                        _resolve_image_info: *const ResolveImageInfo2,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_resolve_image2),))
                    }
                    let f = f(c"vkCmdResolveImage2KHR");
                    if f.is_null() {
                        cmd_resolve_image2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdResolveImage2KHR>(f)
                    }
                },
                cmd_set_event2: unsafe {
                    unsafe extern "system" fn cmd_set_event2(
                        _command_buffer: CommandBuffer,
                        _event: Event,
                        _dependency_info: *const DependencyInfo,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_set_event2),))
                    }
                    let f = f(c"vkCmdSetEvent2KHR");
                    if f.is_null() {
                        cmd_set_event2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetEvent2KHR>(f)
                    }
                },
                cmd_reset_event2: unsafe {
                    unsafe extern "system" fn cmd_reset_event2(
                        _command_buffer: CommandBuffer,
                        _event: Event,
                        _stage_mask: PipelineStageFlags2,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_reset_event2),))
                    }
                    let f = f(c"vkCmdResetEvent2KHR");
                    if f.is_null() {
                        cmd_reset_event2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdResetEvent2KHR>(f)
                    }
                },
                cmd_wait_events2: unsafe {
                    unsafe extern "system" fn cmd_wait_events2(
                        _command_buffer: CommandBuffer,
                        _event_count: u32,
                        _events: *const Event,
                        _dependency_infos: *const DependencyInfo,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_wait_events2),))
                    }
                    let f = f(c"vkCmdWaitEvents2KHR");
                    if f.is_null() {
                        cmd_wait_events2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdWaitEvents2KHR>(f)
                    }
                },
                cmd_pipeline_barrier2: unsafe {
                    unsafe extern "system" fn cmd_pipeline_barrier2(
                        _command_buffer: CommandBuffer,
                        _dependency_info: *const DependencyInfo,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_pipeline_barrier2),
                        ))
                    }
                    let f = f(c"vkCmdPipelineBarrier2KHR");
                    if f.is_null() {
                        cmd_pipeline_barrier2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdPipelineBarrier2KHR>(
                            f,
                        )
                    }
                },
                queue_submit2: unsafe {
                    unsafe extern "system" fn queue_submit2(
                        _queue: Queue,
                        _submit_count: u32,
                        _submits: *const SubmitInfo2,
                        _fence: Fence,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(queue_submit2),))
                    }
                    let f = f(c"vkQueueSubmit2KHR");
                    if f.is_null() {
                        queue_submit2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkQueueSubmit2KHR>(f)
                    }
                },
                cmd_write_timestamp2: unsafe {
                    unsafe extern "system" fn cmd_write_timestamp2(
                        _command_buffer: CommandBuffer,
                        _stage: PipelineStageFlags2,
                        _query_pool: QueryPool,
                        _query: u32,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_write_timestamp2),))
                    }
                    let f = f(c"vkCmdWriteTimestamp2KHR");
                    if f.is_null() {
                        cmd_write_timestamp2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdWriteTimestamp2KHR>(f)
                    }
                },
                cmd_begin_rendering: unsafe {
                    unsafe extern "system" fn cmd_begin_rendering(
                        _command_buffer: CommandBuffer,
                        _rendering_info: *const RenderingInfo,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_begin_rendering),))
                    }
                    let f = f(c"vkCmdBeginRenderingKHR");
                    if f.is_null() {
                        cmd_begin_rendering
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBeginRenderingKHR>(f)
                    }
                },
                cmd_end_rendering: unsafe {
                    unsafe extern "system" fn cmd_end_rendering(_command_buffer: CommandBuffer) {
                        panic!(concat!("failed to load ", stringify!(cmd_end_rendering),))
                    }
                    let f = f(c"vkCmdEndRenderingKHR");
                    if f.is_null() {
                        cmd_end_rendering
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdEndRenderingKHR>(f)
                    }
                },
            }
        }
    }
}
impl<Ext> crate::Device<Ext> {
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_maintenance4.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirements.html>"]
    #[doc = r""]
    pub unsafe fn get_device_buffer_memory_requirements(
        &self,
        info: &DeviceBufferMemoryRequirements<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe {
            (self.fp_v13.get_device_buffer_memory_requirements)(
                self.handle,
                info,
                memory_requirements,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_maintenance4.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirements.html>"]
    #[doc = r""]
    pub unsafe fn get_device_image_memory_requirements(
        &self,
        info: &DeviceImageMemoryRequirements<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe {
            (self.fp_v13.get_device_image_memory_requirements)(
                self.handle,
                info,
                memory_requirements,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_maintenance4.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html>"]
    #[doc = r""]
    pub unsafe fn get_device_image_sparse_memory_requirements_len(
        &self,
        info: &DeviceImageMemoryRequirements<'_>,
    ) -> u32 {
        unsafe {
            let mut sparse_memory_requirement_count = ::core::mem::MaybeUninit::uninit();
            (self.fp_v13.get_device_image_sparse_memory_requirements)(
                self.handle,
                info,
                sparse_memory_requirement_count.as_mut_ptr(),
                core::ptr::null_mut(),
            );
            sparse_memory_requirement_count.assume_init()
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_maintenance4.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html>"]
    #[doc = r""]
    pub unsafe fn get_device_image_sparse_memory_requirements(
        &self,
        info: &DeviceImageMemoryRequirements<'_>,
        out: &mut [SparseImageMemoryRequirements2<'_>],
    ) {
        unsafe {
            let mut len = out.len() as _;
            (self.fp_v13.get_device_image_sparse_memory_requirements)(
                self.handle,
                info,
                &mut len,
                out.as_mut_ptr(),
            )
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_extended_dynamic_state.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_cull_mode(
        &self,
        command_buffer: CommandBuffer,
        cull_mode: CullModeFlags,
    ) {
        unsafe { (self.fp_v13.cmd_set_cull_mode)(command_buffer, cull_mode) }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_extended_dynamic_state.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_front_face(&self, command_buffer: CommandBuffer, front_face: FrontFace) {
        unsafe { (self.fp_v13.cmd_set_front_face)(command_buffer, front_face) }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_extended_dynamic_state.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_primitive_topology(
        &self,
        command_buffer: CommandBuffer,
        primitive_topology: PrimitiveTopology,
    ) {
        unsafe { (self.fp_v13.cmd_set_primitive_topology)(command_buffer, primitive_topology) }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_extended_dynamic_state.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_viewport_with_count(
        &self,
        command_buffer: CommandBuffer,
        viewports: &[Viewport],
    ) {
        unsafe {
            (self.fp_v13.cmd_set_viewport_with_count)(
                command_buffer,
                viewports.len() as _,
                if !viewports.is_empty() {
                    viewports.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_extended_dynamic_state.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_scissor_with_count(
        &self,
        command_buffer: CommandBuffer,
        scissors: &[Rect2D],
    ) {
        unsafe {
            (self.fp_v13.cmd_set_scissor_with_count)(
                command_buffer,
                scissors.len() as _,
                if !scissors.is_empty() {
                    scissors.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_extended_dynamic_state.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html>"]
    #[doc = r""]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn cmd_bind_vertex_buffers2(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
        sizes: Option<&[DeviceSize]>,
        strides: Option<&[DeviceSize]>,
    ) {
        unsafe {
            (self.fp_v13.cmd_bind_vertex_buffers2)(
                command_buffer,
                first_binding,
                buffers.len() as _,
                if !buffers.is_empty() {
                    buffers.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                if !offsets.is_empty() {
                    offsets.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                sizes
                    .and_then(|s| (!s.is_empty()).then_some(s.as_ptr()))
                    .unwrap_or_default(),
                strides
                    .and_then(|s| (!s.is_empty()).then_some(s.as_ptr()))
                    .unwrap_or_default(),
            )
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_extended_dynamic_state.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_depth_test_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_test_enable: bool,
    ) {
        unsafe {
            (self.fp_v13.cmd_set_depth_test_enable)(command_buffer, depth_test_enable as Bool32)
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_extended_dynamic_state.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_depth_write_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_write_enable: bool,
    ) {
        unsafe {
            (self.fp_v13.cmd_set_depth_write_enable)(command_buffer, depth_write_enable as Bool32)
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_extended_dynamic_state.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_depth_compare_op(
        &self,
        command_buffer: CommandBuffer,
        depth_compare_op: CompareOp,
    ) {
        unsafe { (self.fp_v13.cmd_set_depth_compare_op)(command_buffer, depth_compare_op) }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_extended_dynamic_state.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_depth_bounds_test_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_bounds_test_enable: bool,
    ) {
        unsafe {
            (self.fp_v13.cmd_set_depth_bounds_test_enable)(
                command_buffer,
                depth_bounds_test_enable as Bool32,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_extended_dynamic_state.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_stencil_test_enable(
        &self,
        command_buffer: CommandBuffer,
        stencil_test_enable: bool,
    ) {
        unsafe {
            (self.fp_v13.cmd_set_stencil_test_enable)(command_buffer, stencil_test_enable as Bool32)
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_extended_dynamic_state.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html>"]
    #[doc = r""]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn cmd_set_stencil_op(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) {
        unsafe {
            (self.fp_v13.cmd_set_stencil_op)(
                command_buffer,
                face_mask,
                fail_op,
                pass_op,
                depth_fail_op,
                compare_op,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_extended_dynamic_state2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnable.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_rasterizer_discard_enable(
        &self,
        command_buffer: CommandBuffer,
        rasterizer_discard_enable: bool,
    ) {
        unsafe {
            (self.fp_v13.cmd_set_rasterizer_discard_enable)(
                command_buffer,
                rasterizer_discard_enable as Bool32,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_extended_dynamic_state2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnable.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_depth_bias_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_enable: bool,
    ) {
        unsafe {
            (self.fp_v13.cmd_set_depth_bias_enable)(command_buffer, depth_bias_enable as Bool32)
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_extended_dynamic_state2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnable.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_primitive_restart_enable(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_enable: bool,
    ) {
        unsafe {
            (self.fp_v13.cmd_set_primitive_restart_enable)(
                command_buffer,
                primitive_restart_enable as Bool32,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_private_data.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlot.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn create_private_data_slot(
        &self,
        create_info: &PrivateDataSlotCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::VkResult<PrivateDataSlot> {
        unsafe {
            let mut private_data_slot = ::core::mem::MaybeUninit::uninit();
            (self.fp_v13.create_private_data_slot)(
                self.handle,
                create_info,
                allocator.as_ptr(),
                private_data_slot.as_mut_ptr(),
            )
            .result_with_assume_init(&[crate::vk::Result::SUCCESS], private_data_slot)
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_private_data.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html>"]
    #[doc = r""]
    pub unsafe fn destroy_private_data_slot(
        &self,
        private_data_slot: PrivateDataSlot,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fp_v13.destroy_private_data_slot)(
                self.handle,
                private_data_slot,
                allocator.as_ptr(),
            )
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_private_data.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateData.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn set_private_data(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v13.set_private_data)(
                self.handle,
                object_type,
                object_handle,
                private_data_slot,
                data,
            )
            .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_EXT_private_data.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateData.html>"]
    #[doc = r""]
    pub unsafe fn get_private_data(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
    ) -> u64 {
        unsafe {
            let mut data = ::core::mem::MaybeUninit::uninit();
            (self.fp_v13.get_private_data)(
                self.handle,
                object_type,
                object_handle,
                private_data_slot,
                data.as_mut_ptr(),
            );
            data.assume_init()
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_copy_commands2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_copy_buffer2(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_info: &CopyBufferInfo2<'_>,
    ) {
        unsafe { (self.fp_v13.cmd_copy_buffer2)(command_buffer, copy_buffer_info) }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_copy_commands2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_copy_image2(
        &self,
        command_buffer: CommandBuffer,
        copy_image_info: &CopyImageInfo2<'_>,
    ) {
        unsafe { (self.fp_v13.cmd_copy_image2)(command_buffer, copy_image_info) }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_copy_commands2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_blit_image2(
        &self,
        command_buffer: CommandBuffer,
        blit_image_info: &BlitImageInfo2<'_>,
    ) {
        unsafe { (self.fp_v13.cmd_blit_image2)(command_buffer, blit_image_info) }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_copy_commands2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_copy_buffer_to_image2(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_to_image_info: &CopyBufferToImageInfo2<'_>,
    ) {
        unsafe {
            (self.fp_v13.cmd_copy_buffer_to_image2)(command_buffer, copy_buffer_to_image_info)
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_copy_commands2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_copy_image_to_buffer2(
        &self,
        command_buffer: CommandBuffer,
        copy_image_to_buffer_info: &CopyImageToBufferInfo2<'_>,
    ) {
        unsafe {
            (self.fp_v13.cmd_copy_image_to_buffer2)(command_buffer, copy_image_to_buffer_info)
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_copy_commands2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_resolve_image2(
        &self,
        command_buffer: CommandBuffer,
        resolve_image_info: &ResolveImageInfo2<'_>,
    ) {
        unsafe { (self.fp_v13.cmd_resolve_image2)(command_buffer, resolve_image_info) }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_synchronization2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_event2(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        dependency_info: &DependencyInfo<'_>,
    ) {
        unsafe { (self.fp_v13.cmd_set_event2)(command_buffer, event, dependency_info) }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_synchronization2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_reset_event2(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ) {
        unsafe { (self.fp_v13.cmd_reset_event2)(command_buffer, event, stage_mask) }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_synchronization2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_wait_events2(
        &self,
        command_buffer: CommandBuffer,
        events: &[Event],
        dependency_infos: &[DependencyInfo<'_>],
    ) {
        unsafe {
            (self.fp_v13.cmd_wait_events2)(
                command_buffer,
                events.len() as _,
                if !events.is_empty() {
                    events.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                if !dependency_infos.is_empty() {
                    dependency_infos.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_synchronization2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_pipeline_barrier2(
        &self,
        command_buffer: CommandBuffer,
        dependency_info: &DependencyInfo<'_>,
    ) {
        unsafe { (self.fp_v13.cmd_pipeline_barrier2)(command_buffer, dependency_info) }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_synchronization2.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn queue_submit2(
        &self,
        queue: Queue,
        submits: &[SubmitInfo2<'_>],
        fence: Fence,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v13.queue_submit2)(
                queue,
                submits.len() as _,
                if !submits.is_empty() {
                    submits.as_ptr()
                } else {
                    ::core::ptr::null()
                },
                fence,
            )
            .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_synchronization2.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_write_timestamp2(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        unsafe { (self.fp_v13.cmd_write_timestamp2)(command_buffer, stage, query_pool, query) }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_dynamic_rendering.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRendering.html>"]
    #[doc = r""]
    pub unsafe fn cmd_begin_rendering(
        &self,
        command_buffer: CommandBuffer,
        rendering_info: &RenderingInfo<'_>,
    ) {
        unsafe { (self.fp_v13.cmd_begin_rendering)(command_buffer, rendering_info) }
    }
    #[doc = "Requires Vulkan version 1.3, otherwise provided by VK_KHR_dynamic_rendering.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering.html>"]
    #[doc = r""]
    pub unsafe fn cmd_end_rendering(&self, command_buffer: CommandBuffer) {
        unsafe { (self.fp_v13.cmd_end_rendering)(command_buffer) }
    }
}
#[derive(Clone, Copy)]
pub struct DeviceFpV14 {
    pub get_rendering_area_granularity: PFN_vkGetRenderingAreaGranularity,
    pub cmd_push_descriptor_set: PFN_vkCmdPushDescriptorSet,
    pub cmd_push_descriptor_set_with_template: PFN_vkCmdPushDescriptorSetWithTemplate,
    pub cmd_set_line_stipple: PFN_vkCmdSetLineStipple,
    pub cmd_bind_index_buffer2: PFN_vkCmdBindIndexBuffer2,
    pub copy_memory_to_image: PFN_vkCopyMemoryToImage,
    pub copy_image_to_memory: PFN_vkCopyImageToMemory,
    pub copy_image_to_image: PFN_vkCopyImageToImage,
    pub transition_image_layout: PFN_vkTransitionImageLayout,
    pub get_image_subresource_layout2: PFN_vkGetImageSubresourceLayout2,
    pub get_device_image_subresource_layout: PFN_vkGetDeviceImageSubresourceLayout,
    pub map_memory2: PFN_vkMapMemory2,
    pub unmap_memory2: PFN_vkUnmapMemory2,
    pub cmd_bind_descriptor_sets2: PFN_vkCmdBindDescriptorSets2,
    pub cmd_push_constants2: PFN_vkCmdPushConstants2,
    pub cmd_push_descriptor_set2: PFN_vkCmdPushDescriptorSet2,
    pub cmd_push_descriptor_set_with_template2: PFN_vkCmdPushDescriptorSetWithTemplate2,
    pub cmd_set_rendering_attachment_locations: PFN_vkCmdSetRenderingAttachmentLocations,
    pub cmd_set_rendering_input_attachment_indices: PFN_vkCmdSetRenderingInputAttachmentIndices,
}
unsafe impl Send for DeviceFpV14 {}
unsafe impl Sync for DeviceFpV14 {}
impl DeviceFpV14 {
    #[allow(unused_variables)]
    pub fn load(version: u32, f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void) -> Self {
        if version >= API_VERSION_1_4 {
            Self {
                get_rendering_area_granularity: unsafe {
                    unsafe extern "system" fn get_rendering_area_granularity(
                        _device: crate::vk::Device,
                        _rendering_area_info: *const RenderingAreaInfo,
                        _granularity: *mut Extent2D,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_rendering_area_granularity),
                        ))
                    }
                    let f = f(c"vkGetRenderingAreaGranularity");
                    if f.is_null() {
                        get_rendering_area_granularity
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetRenderingAreaGranularity,
                        >(f)
                    }
                },
                cmd_push_descriptor_set: unsafe {
                    unsafe extern "system" fn cmd_push_descriptor_set(
                        _command_buffer: CommandBuffer,
                        _pipeline_bind_point: PipelineBindPoint,
                        _layout: PipelineLayout,
                        _set: u32,
                        _descriptor_write_count: u32,
                        _descriptor_writes: *const WriteDescriptorSet,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_push_descriptor_set),
                        ))
                    }
                    let f = f(c"vkCmdPushDescriptorSet");
                    if f.is_null() {
                        cmd_push_descriptor_set
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdPushDescriptorSet>(f)
                    }
                },
                cmd_push_descriptor_set_with_template: unsafe {
                    unsafe extern "system" fn cmd_push_descriptor_set_with_template(
                        _command_buffer: CommandBuffer,
                        _descriptor_update_template: DescriptorUpdateTemplate,
                        _layout: PipelineLayout,
                        _set: u32,
                        _data: *const ffi::c_void,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_push_descriptor_set_with_template),
                        ))
                    }
                    let f = f(c"vkCmdPushDescriptorSetWithTemplate");
                    if f.is_null() {
                        cmd_push_descriptor_set_with_template
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdPushDescriptorSetWithTemplate,
                        >(f)
                    }
                },
                cmd_set_line_stipple: unsafe {
                    unsafe extern "system" fn cmd_set_line_stipple(
                        _command_buffer: CommandBuffer,
                        _line_stipple_factor: u32,
                        _line_stipple_pattern: u16,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_set_line_stipple),))
                    }
                    let f = f(c"vkCmdSetLineStipple");
                    if f.is_null() {
                        cmd_set_line_stipple
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetLineStipple>(f)
                    }
                },
                cmd_bind_index_buffer2: unsafe {
                    unsafe extern "system" fn cmd_bind_index_buffer2(
                        _command_buffer: CommandBuffer,
                        _buffer: Buffer,
                        _offset: DeviceSize,
                        _size: DeviceSize,
                        _index_type: IndexType,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_bind_index_buffer2),
                        ))
                    }
                    let f = f(c"vkCmdBindIndexBuffer2");
                    if f.is_null() {
                        cmd_bind_index_buffer2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBindIndexBuffer2>(f)
                    }
                },
                copy_memory_to_image: unsafe {
                    unsafe extern "system" fn copy_memory_to_image(
                        _device: crate::vk::Device,
                        _copy_memory_to_image_info: *const CopyMemoryToImageInfo,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(copy_memory_to_image),))
                    }
                    let f = f(c"vkCopyMemoryToImage");
                    if f.is_null() {
                        copy_memory_to_image
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCopyMemoryToImage>(f)
                    }
                },
                copy_image_to_memory: unsafe {
                    unsafe extern "system" fn copy_image_to_memory(
                        _device: crate::vk::Device,
                        _copy_image_to_memory_info: *const CopyImageToMemoryInfo,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(copy_image_to_memory),))
                    }
                    let f = f(c"vkCopyImageToMemory");
                    if f.is_null() {
                        copy_image_to_memory
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCopyImageToMemory>(f)
                    }
                },
                copy_image_to_image: unsafe {
                    unsafe extern "system" fn copy_image_to_image(
                        _device: crate::vk::Device,
                        _copy_image_to_image_info: *const CopyImageToImageInfo,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(copy_image_to_image),))
                    }
                    let f = f(c"vkCopyImageToImage");
                    if f.is_null() {
                        copy_image_to_image
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCopyImageToImage>(f)
                    }
                },
                transition_image_layout: unsafe {
                    unsafe extern "system" fn transition_image_layout(
                        _device: crate::vk::Device,
                        _transition_count: u32,
                        _transitions: *const HostImageLayoutTransitionInfo,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(transition_image_layout),
                        ))
                    }
                    let f = f(c"vkTransitionImageLayout");
                    if f.is_null() {
                        transition_image_layout
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkTransitionImageLayout>(f)
                    }
                },
                get_image_subresource_layout2: unsafe {
                    unsafe extern "system" fn get_image_subresource_layout2(
                        _device: crate::vk::Device,
                        _image: Image,
                        _subresource: *const ImageSubresource2,
                        _layout: *mut SubresourceLayout2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_image_subresource_layout2),
                        ))
                    }
                    let f = f(c"vkGetImageSubresourceLayout2");
                    if f.is_null() {
                        get_image_subresource_layout2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkGetImageSubresourceLayout2>(
                            f,
                        )
                    }
                },
                get_device_image_subresource_layout: unsafe {
                    unsafe extern "system" fn get_device_image_subresource_layout(
                        _device: crate::vk::Device,
                        _info: *const DeviceImageSubresourceInfo,
                        _layout: *mut SubresourceLayout2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_device_image_subresource_layout),
                        ))
                    }
                    let f = f(c"vkGetDeviceImageSubresourceLayout");
                    if f.is_null() {
                        get_device_image_subresource_layout
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetDeviceImageSubresourceLayout,
                        >(f)
                    }
                },
                map_memory2: unsafe {
                    unsafe extern "system" fn map_memory2(
                        _device: crate::vk::Device,
                        _memory_map_info: *const MemoryMapInfo,
                        _pp_data: *mut *mut ffi::c_void,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(map_memory2),))
                    }
                    let f = f(c"vkMapMemory2");
                    if f.is_null() {
                        map_memory2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkMapMemory2>(f)
                    }
                },
                unmap_memory2: unsafe {
                    unsafe extern "system" fn unmap_memory2(
                        _device: crate::vk::Device,
                        _memory_unmap_info: *const MemoryUnmapInfo,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(unmap_memory2),))
                    }
                    let f = f(c"vkUnmapMemory2");
                    if f.is_null() {
                        unmap_memory2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkUnmapMemory2>(f)
                    }
                },
                cmd_bind_descriptor_sets2: unsafe {
                    unsafe extern "system" fn cmd_bind_descriptor_sets2(
                        _command_buffer: CommandBuffer,
                        _bind_descriptor_sets_info: *const BindDescriptorSetsInfo,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_bind_descriptor_sets2),
                        ))
                    }
                    let f = f(c"vkCmdBindDescriptorSets2");
                    if f.is_null() {
                        cmd_bind_descriptor_sets2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBindDescriptorSets2>(
                            f,
                        )
                    }
                },
                cmd_push_constants2: unsafe {
                    unsafe extern "system" fn cmd_push_constants2(
                        _command_buffer: CommandBuffer,
                        _push_constants_info: *const PushConstantsInfo,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_push_constants2),))
                    }
                    let f = f(c"vkCmdPushConstants2");
                    if f.is_null() {
                        cmd_push_constants2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdPushConstants2>(f)
                    }
                },
                cmd_push_descriptor_set2: unsafe {
                    unsafe extern "system" fn cmd_push_descriptor_set2(
                        _command_buffer: CommandBuffer,
                        _push_descriptor_set_info: *const PushDescriptorSetInfo,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_push_descriptor_set2),
                        ))
                    }
                    let f = f(c"vkCmdPushDescriptorSet2");
                    if f.is_null() {
                        cmd_push_descriptor_set2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdPushDescriptorSet2>(f)
                    }
                },
                cmd_push_descriptor_set_with_template2: unsafe {
                    unsafe extern "system" fn cmd_push_descriptor_set_with_template2(
                        _command_buffer: CommandBuffer,
                        _push_descriptor_set_with_template_info : * const PushDescriptorSetWithTemplateInfo,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_push_descriptor_set_with_template2),
                        ))
                    }
                    let f = f(c"vkCmdPushDescriptorSetWithTemplate2");
                    if f.is_null() {
                        cmd_push_descriptor_set_with_template2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdPushDescriptorSetWithTemplate2,
                        >(f)
                    }
                },
                cmd_set_rendering_attachment_locations: unsafe {
                    unsafe extern "system" fn cmd_set_rendering_attachment_locations(
                        _command_buffer: CommandBuffer,
                        _location_info: *const RenderingAttachmentLocationInfo,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_rendering_attachment_locations),
                        ))
                    }
                    let f = f(c"vkCmdSetRenderingAttachmentLocations");
                    if f.is_null() {
                        cmd_set_rendering_attachment_locations
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdSetRenderingAttachmentLocations,
                        >(f)
                    }
                },
                cmd_set_rendering_input_attachment_indices: unsafe {
                    unsafe extern "system" fn cmd_set_rendering_input_attachment_indices(
                        _command_buffer: CommandBuffer,
                        _input_attachment_index_info: *const RenderingInputAttachmentIndexInfo,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_rendering_input_attachment_indices),
                        ))
                    }
                    let f = f(c"vkCmdSetRenderingInputAttachmentIndices");
                    if f.is_null() {
                        cmd_set_rendering_input_attachment_indices
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdSetRenderingInputAttachmentIndices,
                        >(f)
                    }
                },
            }
        } else {
            Self {
                get_rendering_area_granularity: unsafe {
                    unsafe extern "system" fn get_rendering_area_granularity(
                        _device: crate::vk::Device,
                        _rendering_area_info: *const RenderingAreaInfo,
                        _granularity: *mut Extent2D,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_rendering_area_granularity),
                        ))
                    }
                    let f = f(c"vkGetRenderingAreaGranularityKHR");
                    if f.is_null() {
                        get_rendering_area_granularity
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetRenderingAreaGranularityKHR,
                        >(f)
                    }
                },
                cmd_push_descriptor_set: unsafe {
                    unsafe extern "system" fn cmd_push_descriptor_set(
                        _command_buffer: CommandBuffer,
                        _pipeline_bind_point: PipelineBindPoint,
                        _layout: PipelineLayout,
                        _set: u32,
                        _descriptor_write_count: u32,
                        _descriptor_writes: *const WriteDescriptorSet,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_push_descriptor_set),
                        ))
                    }
                    let f = f(c"vkCmdPushDescriptorSetKHR");
                    if f.is_null() {
                        cmd_push_descriptor_set
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdPushDescriptorSetKHR>(
                            f,
                        )
                    }
                },
                cmd_push_descriptor_set_with_template: unsafe {
                    unsafe extern "system" fn cmd_push_descriptor_set_with_template(
                        _command_buffer: CommandBuffer,
                        _descriptor_update_template: DescriptorUpdateTemplate,
                        _layout: PipelineLayout,
                        _set: u32,
                        _data: *const ffi::c_void,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_push_descriptor_set_with_template),
                        ))
                    }
                    let f = f(c"vkCmdPushDescriptorSetWithTemplateKHR");
                    if f.is_null() {
                        cmd_push_descriptor_set_with_template
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdPushDescriptorSetWithTemplateKHR,
                        >(f)
                    }
                },
                cmd_set_line_stipple: unsafe {
                    unsafe extern "system" fn cmd_set_line_stipple(
                        _command_buffer: CommandBuffer,
                        _line_stipple_factor: u32,
                        _line_stipple_pattern: u16,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_set_line_stipple),))
                    }
                    let f = f(c"vkCmdSetLineStippleKHR");
                    if f.is_null() {
                        cmd_set_line_stipple
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdSetLineStippleKHR>(f)
                    }
                },
                cmd_bind_index_buffer2: unsafe {
                    unsafe extern "system" fn cmd_bind_index_buffer2(
                        _command_buffer: CommandBuffer,
                        _buffer: Buffer,
                        _offset: DeviceSize,
                        _size: DeviceSize,
                        _index_type: IndexType,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_bind_index_buffer2),
                        ))
                    }
                    let f = f(c"vkCmdBindIndexBuffer2KHR");
                    if f.is_null() {
                        cmd_bind_index_buffer2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBindIndexBuffer2KHR>(
                            f,
                        )
                    }
                },
                copy_memory_to_image: unsafe {
                    unsafe extern "system" fn copy_memory_to_image(
                        _device: crate::vk::Device,
                        _copy_memory_to_image_info: *const CopyMemoryToImageInfo,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(copy_memory_to_image),))
                    }
                    let f = f(c"vkCopyMemoryToImageEXT");
                    if f.is_null() {
                        copy_memory_to_image
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCopyMemoryToImageEXT>(f)
                    }
                },
                copy_image_to_memory: unsafe {
                    unsafe extern "system" fn copy_image_to_memory(
                        _device: crate::vk::Device,
                        _copy_image_to_memory_info: *const CopyImageToMemoryInfo,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(copy_image_to_memory),))
                    }
                    let f = f(c"vkCopyImageToMemoryEXT");
                    if f.is_null() {
                        copy_image_to_memory
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCopyImageToMemoryEXT>(f)
                    }
                },
                copy_image_to_image: unsafe {
                    unsafe extern "system" fn copy_image_to_image(
                        _device: crate::vk::Device,
                        _copy_image_to_image_info: *const CopyImageToImageInfo,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(copy_image_to_image),))
                    }
                    let f = f(c"vkCopyImageToImageEXT");
                    if f.is_null() {
                        copy_image_to_image
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCopyImageToImageEXT>(f)
                    }
                },
                transition_image_layout: unsafe {
                    unsafe extern "system" fn transition_image_layout(
                        _device: crate::vk::Device,
                        _transition_count: u32,
                        _transitions: *const HostImageLayoutTransitionInfo,
                    ) -> Result {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(transition_image_layout),
                        ))
                    }
                    let f = f(c"vkTransitionImageLayoutEXT");
                    if f.is_null() {
                        transition_image_layout
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkTransitionImageLayoutEXT>(
                            f,
                        )
                    }
                },
                get_image_subresource_layout2: unsafe {
                    unsafe extern "system" fn get_image_subresource_layout2(
                        _device: crate::vk::Device,
                        _image: Image,
                        _subresource: *const ImageSubresource2,
                        _layout: *mut SubresourceLayout2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_image_subresource_layout2),
                        ))
                    }
                    let f = f(c"vkGetImageSubresourceLayout2EXT");
                    if f.is_null() {
                        get_image_subresource_layout2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetImageSubresourceLayout2EXT,
                        >(f)
                    }
                },
                get_device_image_subresource_layout: unsafe {
                    unsafe extern "system" fn get_device_image_subresource_layout(
                        _device: crate::vk::Device,
                        _info: *const DeviceImageSubresourceInfo,
                        _layout: *mut SubresourceLayout2,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(get_device_image_subresource_layout),
                        ))
                    }
                    let f = f(c"vkGetDeviceImageSubresourceLayoutKHR");
                    if f.is_null() {
                        get_device_image_subresource_layout
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkGetDeviceImageSubresourceLayoutKHR,
                        >(f)
                    }
                },
                map_memory2: unsafe {
                    unsafe extern "system" fn map_memory2(
                        _device: crate::vk::Device,
                        _memory_map_info: *const MemoryMapInfo,
                        _pp_data: *mut *mut ffi::c_void,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(map_memory2),))
                    }
                    let f = f(c"vkMapMemory2KHR");
                    if f.is_null() {
                        map_memory2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkMapMemory2KHR>(f)
                    }
                },
                unmap_memory2: unsafe {
                    unsafe extern "system" fn unmap_memory2(
                        _device: crate::vk::Device,
                        _memory_unmap_info: *const MemoryUnmapInfo,
                    ) -> Result {
                        panic!(concat!("failed to load ", stringify!(unmap_memory2),))
                    }
                    let f = f(c"vkUnmapMemory2KHR");
                    if f.is_null() {
                        unmap_memory2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkUnmapMemory2KHR>(f)
                    }
                },
                cmd_bind_descriptor_sets2: unsafe {
                    unsafe extern "system" fn cmd_bind_descriptor_sets2(
                        _command_buffer: CommandBuffer,
                        _bind_descriptor_sets_info: *const BindDescriptorSetsInfo,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_bind_descriptor_sets2),
                        ))
                    }
                    let f = f(c"vkCmdBindDescriptorSets2KHR");
                    if f.is_null() {
                        cmd_bind_descriptor_sets2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdBindDescriptorSets2KHR>(
                            f,
                        )
                    }
                },
                cmd_push_constants2: unsafe {
                    unsafe extern "system" fn cmd_push_constants2(
                        _command_buffer: CommandBuffer,
                        _push_constants_info: *const PushConstantsInfo,
                    ) {
                        panic!(concat!("failed to load ", stringify!(cmd_push_constants2),))
                    }
                    let f = f(c"vkCmdPushConstants2KHR");
                    if f.is_null() {
                        cmd_push_constants2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdPushConstants2KHR>(f)
                    }
                },
                cmd_push_descriptor_set2: unsafe {
                    unsafe extern "system" fn cmd_push_descriptor_set2(
                        _command_buffer: CommandBuffer,
                        _push_descriptor_set_info: *const PushDescriptorSetInfo,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_push_descriptor_set2),
                        ))
                    }
                    let f = f(c"vkCmdPushDescriptorSet2KHR");
                    if f.is_null() {
                        cmd_push_descriptor_set2
                    } else {
                        ::core::mem::transmute::<*const ffi::c_void, PFN_vkCmdPushDescriptorSet2KHR>(
                            f,
                        )
                    }
                },
                cmd_push_descriptor_set_with_template2: unsafe {
                    unsafe extern "system" fn cmd_push_descriptor_set_with_template2(
                        _command_buffer: CommandBuffer,
                        _push_descriptor_set_with_template_info : * const PushDescriptorSetWithTemplateInfo,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_push_descriptor_set_with_template2),
                        ))
                    }
                    let f = f(c"vkCmdPushDescriptorSetWithTemplate2KHR");
                    if f.is_null() {
                        cmd_push_descriptor_set_with_template2
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdPushDescriptorSetWithTemplate2KHR,
                        >(f)
                    }
                },
                cmd_set_rendering_attachment_locations: unsafe {
                    unsafe extern "system" fn cmd_set_rendering_attachment_locations(
                        _command_buffer: CommandBuffer,
                        _location_info: *const RenderingAttachmentLocationInfo,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_rendering_attachment_locations),
                        ))
                    }
                    let f = f(c"vkCmdSetRenderingAttachmentLocationsKHR");
                    if f.is_null() {
                        cmd_set_rendering_attachment_locations
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdSetRenderingAttachmentLocationsKHR,
                        >(f)
                    }
                },
                cmd_set_rendering_input_attachment_indices: unsafe {
                    unsafe extern "system" fn cmd_set_rendering_input_attachment_indices(
                        _command_buffer: CommandBuffer,
                        _input_attachment_index_info: *const RenderingInputAttachmentIndexInfo,
                    ) {
                        panic!(concat!(
                            "failed to load ",
                            stringify!(cmd_set_rendering_input_attachment_indices),
                        ))
                    }
                    let f = f(c"vkCmdSetRenderingInputAttachmentIndicesKHR");
                    if f.is_null() {
                        cmd_set_rendering_input_attachment_indices
                    } else {
                        ::core::mem::transmute::<
                            *const ffi::c_void,
                            PFN_vkCmdSetRenderingInputAttachmentIndicesKHR,
                        >(f)
                    }
                },
            }
        }
    }
}
impl<Ext> crate::Device<Ext> {
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_KHR_maintenance5.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularity.html>"]
    #[doc = r""]
    pub unsafe fn get_rendering_area_granularity(
        &self,
        rendering_area_info: &RenderingAreaInfo<'_>,
    ) -> Extent2D {
        unsafe {
            let mut granularity = ::core::mem::MaybeUninit::uninit();
            (self.fp_v14.get_rendering_area_granularity)(
                self.handle,
                rendering_area_info,
                granularity.as_mut_ptr(),
            );
            granularity.assume_init()
        }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_KHR_push_descriptor.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html>"]
    #[doc = r""]
    pub unsafe fn cmd_push_descriptor_set(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_writes: &[WriteDescriptorSet<'_>],
    ) {
        unsafe {
            (self.fp_v14.cmd_push_descriptor_set)(
                command_buffer,
                pipeline_bind_point,
                layout,
                set,
                descriptor_writes.len() as _,
                if !descriptor_writes.is_empty() {
                    descriptor_writes.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
        }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_KHR_push_descriptor.\nVK_KHR_push_descriptor depends on either:* VK_VERSION_1_1 or* VK_KHR_descriptor_update_template"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate.html>"]
    #[doc = r""]
    pub unsafe fn cmd_push_descriptor_set_with_template(
        &self,
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        data: &ffi::c_void,
    ) {
        unsafe {
            (self.fp_v14.cmd_push_descriptor_set_with_template)(
                command_buffer,
                descriptor_update_template,
                layout,
                set,
                data,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_KHR_line_rasterization.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_line_stipple(
        &self,
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        unsafe {
            (self.fp_v14.cmd_set_line_stipple)(
                command_buffer,
                line_stipple_factor,
                line_stipple_pattern,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_KHR_maintenance5.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_bind_index_buffer2(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ) {
        unsafe {
            (self.fp_v14.cmd_bind_index_buffer2)(command_buffer, buffer, offset, size, index_type)
        }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_EXT_host_image_copy.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImage.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn copy_memory_to_image(
        &self,
        copy_memory_to_image_info: &CopyMemoryToImageInfo<'_>,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v14.copy_memory_to_image)(self.handle, copy_memory_to_image_info)
                .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_EXT_host_image_copy.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemory.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn copy_image_to_memory(
        &self,
        copy_image_to_memory_info: &CopyImageToMemoryInfo<'_>,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v14.copy_image_to_memory)(self.handle, copy_image_to_memory_info)
                .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_EXT_host_image_copy.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImage.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn copy_image_to_image(
        &self,
        copy_image_to_image_info: &CopyImageToImageInfo<'_>,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v14.copy_image_to_image)(self.handle, copy_image_to_image_info)
                .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_EXT_host_image_copy.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayout.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn transition_image_layout(
        &self,
        transitions: &[HostImageLayoutTransitionInfo<'_>],
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v14.transition_image_layout)(
                self.handle,
                transitions.len() as _,
                if !transitions.is_empty() {
                    transitions.as_ptr()
                } else {
                    ::core::ptr::null()
                },
            )
            .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_EXT_host_image_copy.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html>"]
    #[doc = r""]
    pub unsafe fn get_image_subresource_layout2(
        &self,
        image: Image,
        subresource: &ImageSubresource2<'_>,
        layout: &mut SubresourceLayout2<'_>,
    ) {
        unsafe {
            (self.fp_v14.get_image_subresource_layout2)(self.handle, image, subresource, layout)
        }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_KHR_maintenance5.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayout.html>"]
    #[doc = r""]
    pub unsafe fn get_device_image_subresource_layout(
        &self,
        info: &DeviceImageSubresourceInfo<'_>,
        layout: &mut SubresourceLayout2<'_>,
    ) {
        unsafe { (self.fp_v14.get_device_image_subresource_layout)(self.handle, info, layout) }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_KHR_map_memory2.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn map_memory2(
        &self,
        memory_map_info: &MemoryMapInfo<'_>,
    ) -> crate::VkResult<*mut ffi::c_void> {
        unsafe {
            let mut pp_data = ::core::mem::MaybeUninit::uninit();
            (self.fp_v14.map_memory2)(self.handle, memory_map_info, pp_data.as_mut_ptr())
                .result_with_assume_init(&[crate::vk::Result::SUCCESS], pp_data)
        }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_KHR_map_memory2.\n"]
    #[doc = r""]
    #[doc = "# Success codes"]
    #[doc = "* [`SUCCESS`][0]"]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2.html>"]
    #[doc = r""]
    #[doc = "[0]: Result::SUCCESS"]
    pub unsafe fn unmap_memory2(
        &self,
        memory_unmap_info: &MemoryUnmapInfo<'_>,
    ) -> crate::VkResult<()> {
        unsafe {
            (self.fp_v14.unmap_memory2)(self.handle, memory_unmap_info)
                .result(&[crate::vk::Result::SUCCESS])
        }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_KHR_maintenance6.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_bind_descriptor_sets2(
        &self,
        command_buffer: CommandBuffer,
        bind_descriptor_sets_info: &BindDescriptorSetsInfo<'_>,
    ) {
        unsafe {
            (self.fp_v14.cmd_bind_descriptor_sets2)(command_buffer, bind_descriptor_sets_info)
        }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_KHR_maintenance6.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_push_constants2(
        &self,
        command_buffer: CommandBuffer,
        push_constants_info: &PushConstantsInfo<'_>,
    ) {
        unsafe { (self.fp_v14.cmd_push_constants2)(command_buffer, push_constants_info) }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_KHR_maintenance6.\nVK_KHR_maintenance6 depends on either:* VK_KHR_push_descriptor"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_push_descriptor_set2(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_info: &PushDescriptorSetInfo<'_>,
    ) {
        unsafe { (self.fp_v14.cmd_push_descriptor_set2)(command_buffer, push_descriptor_set_info) }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_KHR_maintenance6.\nVK_KHR_maintenance6 depends on either:* VK_KHR_push_descriptor"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2.html>"]
    #[doc = r""]
    pub unsafe fn cmd_push_descriptor_set_with_template2(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo<'_>,
    ) {
        unsafe {
            (self.fp_v14.cmd_push_descriptor_set_with_template2)(
                command_buffer,
                push_descriptor_set_with_template_info,
            )
        }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_KHR_dynamic_rendering_local_read.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_rendering_attachment_locations(
        &self,
        command_buffer: CommandBuffer,
        location_info: &RenderingAttachmentLocationInfo<'_>,
    ) {
        unsafe {
            (self.fp_v14.cmd_set_rendering_attachment_locations)(command_buffer, location_info)
        }
    }
    #[doc = "Requires Vulkan version 1.4, otherwise provided by VK_KHR_dynamic_rendering_local_read.\n"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" All raw Vulkan calls are unsafe as there is no validation of input or usage."]
    #[doc = r" # Vulkan docs"]
    #[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html>"]
    #[doc = r""]
    pub unsafe fn cmd_set_rendering_input_attachment_indices(
        &self,
        command_buffer: CommandBuffer,
        input_attachment_index_info: &RenderingInputAttachmentIndexInfo<'_>,
    ) {
        unsafe {
            (self.fp_v14.cmd_set_rendering_input_attachment_indices)(
                command_buffer,
                input_attachment_index_info,
            )
        }
    }
}

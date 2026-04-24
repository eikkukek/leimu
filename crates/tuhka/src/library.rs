use crate::vk::*;
use crate::{
    LibraryFpV10, LibraryFpV11,
    VkResult, PtrOption,
    LoadWith,
};
use core::fmt::{self, Display};
use core::ffi;

/// Raw Vulkan core functions needed to initialize [`Library`].
#[derive(Clone, Copy)]
pub struct CoreFp {
    pub get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
}
impl CoreFp {
    pub fn load(
        f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void
    ) -> Option<Self>
    {
        Some(Self {
            get_instance_proc_addr: unsafe {
                let f = f(c"vkGetInstanceProcAddr");
                if f.is_null() {
                    return None
                } else {
                    ::core::mem::transmute::<
                        *const ffi::c_void,
                        PFN_vkGetInstanceProcAddr,
                    >(f)
                }
            },
        })
    }
}
unsafe impl Send for CoreFp {}
unsafe impl Sync for CoreFp {}

#[derive(Debug)]
pub enum LoadingError {
    NoInstanceProcAddr,
    LibloadingFailed(libloading::Error),
}

impl Display for LoadingError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoInstanceProcAddr => write!(f, "failed to load vkGetInstanceProcAddr"),
            Self::LibloadingFailed(err) => write!(f, "{err}"),
        }
    }
}

impl core::error::Error for LoadingError {}

pub struct Library {
    pub(crate) core_fp: CoreFp,
    pub(crate) fp_v10: LibraryFpV10,
    pub(crate) fp_v11: LibraryFpV11,
    _lib: Option<libloading::Library>,
}

impl Library {

    /// Loads the [`Library`] with a default path for the current platform.
    ///
    /// See [`load_from_path`][Self::load_from_path] to load the [`Library`] from another path.
    ///
    /// # Safety
    /// Loading native libraries is inherently unsafe.
    ///
    /// The safety section for [`libloading`] [`Library::new`][1] and [`Library::get`][2] holds
    /// here.
    ///
    /// No Vulkan functions originating from this [`Library`] may be called after it is
    /// [`dropped`][drop].
    ///
    /// [1]: libloading::Library::new
    /// [2]: libloading::Library::get
    pub unsafe fn load() -> ::core::result::Result<Self, LoadingError> {
        #[cfg(windows)]
        const PATH: &str = "vulkan-1.dll";
        #[cfg(all(
            unix,
            not(any(
                target_os = "macos",
                target_os = "ios",
                target_os = "android",
                target_os = "fuchsia"
            ))
        ))]
        const PATH: &str = "libvulkan.so.1";
        #[cfg(any(target_os = "android", target_os = "android"))]
        const PATH: &str = "libvulkan.so";
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        const PATH: &str = "libvulkan.dylib";
        unsafe {
            Self::load_from_path(PATH)
        }
    }

    /// Loads the library from `filename`.
    ///
    /// # Safety
    /// Loading native libraries is inherently unsafe.
    ///
    /// The safety section for [`libloading`] [`Library::new`][1] and [`Library::get`][2] holds
    /// here.
    ///
    /// No Vulkan functions originating from this [`Library`] may be called after it is
    /// [`dropped`][drop].
    ///
    /// [1]: libloading::Library::new
    /// [2]: libloading::Library::get
    pub unsafe fn load_from_path<P>(filename: P) -> core::result::Result<Self, LoadingError>
        where P: libloading::AsFilename
    {
        let lib = unsafe {
            libloading::Library::new(filename)
        }.map_err(LoadingError::LibloadingFailed)?;
        let core_fp = CoreFp::load(&mut |name| unsafe {
            lib.get(name)
                .map(|sym| *sym)
                .unwrap_or(core::ptr::null())
        }).ok_or(LoadingError::NoInstanceProcAddr)?;
        unsafe {
            Ok(Self {
                _lib: Some(lib),
                ..Self::from_core_fp(core_fp)
            })
        } 
    }

    /// Loads the [`Library`] from [`CoreFp`].
    ///
    /// # Safety
    /// It has to be ensured that [`get_instance_proc_addr`][1] yields valid function pointers for
    /// use in Vulkan calls.
    ///
    /// [1]: CoreFp::get_instance_proc_addr
    pub unsafe fn from_core_fp(core_fp: CoreFp) -> Self {
        let mut load_fp = move |name: &ffi::CStr| unsafe {
            (core_fp.get_instance_proc_addr)(
                crate::vk::Instance::null(),
                name.as_ptr(),
            ) as *const ffi::c_void
        };
        Self {
            core_fp,
            fp_v10: LibraryFpV10::load(API_VERSION_1_4, &mut load_fp),
            fp_v11: LibraryFpV11::load(API_VERSION_1_4, &mut load_fp),
            _lib: None,
        }
    }
    
    #[inline]
    pub fn fp_v10(&self) -> &LibraryFpV10 {
        &self.fp_v10
    }

    #[inline]
    pub fn fp_v11(&self) -> &LibraryFpV11 {
        &self.fp_v11
    }

    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html>
    ///
    /// # Safety
    /// All raw Vulkan calls are unsafe as there is no validation of input or usage.
    #[inline]
    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: crate::vk::Instance,
        p_name: &ffi::CStr,
    ) -> PFN_vkVoidFunction {
        unsafe {
            (self.core_fp.get_instance_proc_addr)(
                instance,
                p_name.as_ptr(),
            )
        }
    }

    /// Tries to enumerate instance-level version before instance creation.
    ///
    /// If Vulkan version is not [`API_VERSION_1_1`] or loading the function otherwise fails,
    /// this returns [`None`].
    ///
    /// # Safety
    /// All raw Vulkan calls are unsafe as there is no validation of input or usage.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceVersion.html>
    pub unsafe fn try_enumerate_instance_version(
        &self,
    ) -> Option<VkResult<u32>> {
        unsafe {
            let f = self.get_instance_proc_addr(
                crate::vk::Instance::null(),
                c"vkEnumerateInstanceVersion"
            ) as *const ffi::c_void;
            if f.is_null() {
                None
            } else {
                let f = ::core::mem::transmute::<
                    *const ffi::c_void,
                    PFN_vkEnumerateInstanceVersion,
                >(f);
                let mut res = ::core::mem::MaybeUninit::uninit();
                Some((f)(res.as_mut_ptr()).result_with_assume_init(
                    &[Result::SUCCESS],
                    res,
                ))
            }
        }
    }
    
    /// Creates an [`Instance'][1].
    ///
    /// # Safety
    /// All raw Vulkan calls are unsafe as there is no validation of input or usage.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html>
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceVersion.html>
    ///
    /// [1]: crate::Instance
    pub unsafe fn create_instance<Ext>(
        &self,
        create_info: &InstanceCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks>,
    ) -> VkResult<crate::Instance<Ext>>
        where Ext: LoadWith<Handle = crate::vk::Instance>
    {
        unsafe {
            let mut handle = ::core::mem::MaybeUninit::uninit();
            let handle = (self.fp_v10().create_instance)(
                create_info,
                allocator.as_ptr(),
                handle.as_mut_ptr(),
            ).result_with_assume_init(&[Result::SUCCESS], handle)?;
            let version = self.try_enumerate_instance_version()
                .unwrap_or(Ok(crate::Success::new(
                    API_VERSION_1_0, handle.result,
                )))?;
            Ok(handle.with_value(crate::Instance::load(
                version.value, &self.core_fp,
                handle.value
            )))
        }
    }

    /// Destroys an [`Instance'][1].
    ///
    /// # Safety
    /// All raw Vulkan calls are unsafe as there is no validation of input or usage.
    ///
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyInstance.html>
    ///
    /// [1]: crate::Instance
    pub unsafe fn destroy_instance<Ext>(
        &self,
        instance: &crate::Instance<Ext>,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (instance.fp_v10().destroy_instance)(
                instance.handle(),
                allocator.as_ptr(),
            )
        }
    }
}

use core::{
    ops::{Deref, Index},
    fmt::{self, Display},
    ffi::CStr,
};

use raw_window_handle::RawDisplayHandle;

use ahash::AHashSet;

use leimu_mem::{
    vec::Vec32,
    vec32,
};
use tuhka::{
    vk,
    khr::{
        surface,
        get_surface_capabilities2,
    },
};

use crate::{
    gpu::prelude::*,
    error::*,
    sync::*,
};

#[derive(Clone, Copy)]
pub struct InstanceLayer<'a> {
    name: &'a CStr,
    is_required: bool,
}

#[inline]
pub fn instance_layer<'a>(
    name: &'a CStr,
    is_required: bool,
) -> InstanceLayer<'a> {
    InstanceLayer { name, is_required }
}

/// Khronos validation layer.
///
/// For this to be used, the Vulkan SDK needs to be installed.
pub const LAYER_KHRONOS_VALIDATION: &CStr = c"VK_LAYER_KHRONOS_validation";

/// Khronos validation layer.
///
/// For this to be used, the Vulkan SDK needs to be installed.
#[inline(always)]
pub const fn layer_khronos_validation(
    is_required: bool,
) -> InstanceLayer<'static> {
    InstanceLayer {
        name: LAYER_KHRONOS_VALIDATION,
        is_required,
    }
}

impl Display for InstanceLayer<'_> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

/// Alias for chained [`tuhka::Instance`].
///
/// The chain includes [`surface`] and [`get_surface_capabilities2`].
pub type VkInstance = tuhka::Instance<surface::Instance<get_surface_capabilities2::Instance>>;

struct Inner {
    library: Arc<tuhka::Library>,
    instance: VkInstance,
}

/// Represents a [`Vulkan instance`][1].
///
/// Provides methods for [`enumerating`][2] physical devices and [`creating logical devices`][3].
///
/// Also exposes raw Vulkan instance-level functions up to Vulkan 1.4.
///
/// # Example
/// ``` rust
/// use leimu::{Entry, gpu};
/// let entry = Entry::headless(None)?;
/// let instance = entry.create_instance(
///     "My app",
///     gpu::make_api_version(0, 1, 0, 0),
///     &[],
/// )?;
/// let suitable = instance.enumerate_suitable_physical_devices(
///     gpu:::default_device_attributes()
/// )?;
/// let idx = pick_device(&suitable);
/// let device = suitable[idx];
/// let queue_family_index = pick_queue_family(device.enumerate_queue_families());
/// let queue_info = gpu::DeviceQueueCreateInfo::new(
///     "queue",
///     idx,
///     0,
/// );
/// let device = suitable.create_device(idx, &[queue_info])
/// ```
///
/// [1]: tuhka::Instance
/// [2]: Self::enumerate_suitable_physical_devices
/// [3]: SuitablePhysicalDevices::create_device
#[derive(Clone)]
pub struct Instance {
    inner: Arc<Inner>,
}

impl Instance {

    pub(crate) fn new<'a, L>(
        entry: &crate::Entry,
        app_name: &CStr, 
        app_version: Version,
        layers: L,
    ) -> Result<Self>
        where
            L: IntoIterator<Item = InstanceLayer<'a>>,
    {
        let engine_name = c"leimu";
        let application_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_application_name: app_name.as_ptr(),
            application_version: app_version.as_u32(),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk::make_api_version(0, 1, 0, 0),
            api_version: vk::API_VERSION_1_4,
            ..Default::default()
        };
        let mut extensions = Vec32::<(&CStr, bool)>
            ::with_capacity(8);
        let mut found_extensions = Vec32::<*const i8>
            ::with_capacity(8);
        let mut found_extensions_hashed = AHashSet::default();
        get_required_instance_extensions(
            entry.raw_display_handle()?,
            &mut extensions
        )?;
        let library = entry.vulkan.clone();
        let mut found_layers = Vec32::<*const i8>
            ::with_capacity(8);
        let mut found_layers_hashed = AHashSet::default();
        verify_instance_extensions(
            &library,
            &extensions,
            &mut found_extensions,
            &mut found_extensions_hashed
        )?;
        verify_instance_layers(
            &library,
            layers,
            &mut found_layers,
            &mut found_layers_hashed,
        )?;
        let version = unsafe {
            let Some(Ok(version)) = library
                .try_enumerate_instance_version()
            else {
                return Err(Error::just_context(
                    "Leimu requires at least Vulkan version 1.1, enumerated version was 1.0"
                ))
            };
            version.value
        };
        if version < vk::API_VERSION_1_1 {
            return Err(Error::just_context(format!(
                "Leimu requires at least Vulkan version 1.1, enumerated version was {}",
                Version(version),
            )))
        }
        let instance_create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_application_info: &application_info,
            enabled_extension_count: found_extensions.len(),
            pp_enabled_extension_names: found_extensions.as_ptr() as _,
            enabled_layer_count: found_layers.len(),
            pp_enabled_layer_names: found_layers.as_ptr() as _,
            ..Default::default()
        };
        let instance = unsafe {
            library
                .create_instance(&instance_create_info, None)
                .context("failed to create vulkan instance")?
        }.value;
        Ok(Self {
            inner: Arc::new(Inner {
                library,
                instance,
            }),
        })
    }

    /// Enumerates all [`physical devices`][1] that are suitable for the given [`device attributes`][2].
    ///
    /// The [`resultant object`][3] provides methods for picking and creating a [`logical device`][4].
    ///
    /// See the [`struct-level`][Instance] docs for examples.
    ///
    /// [1]: PhysicalDevice
    /// [2]: DeviceAttributes
    /// [3]: SuitablePhysicalDevices
    /// [4]: Device
    pub fn enumerate_suitable_physical_devices(
        &self,
        device_attributes: DeviceAttributes,
    ) -> Result<SuitablePhysicalDevices> {
        let mut device_extensions = Vec32::with_capacity(device_attributes.device_extensions.len());
        device_extensions.extend(ext::core_extensions());
        device_extensions.extend(device_attributes.device_extensions.iter().cloned());
        let mut device_extension_infos = vec32![];
        device_extension_infos.extend(device_extensions
            .iter().filter_map(|ext| ext.get_info(&device_attributes))
        );
        let devices = find_suitable_physical_devices(
            self,
            &device_attributes,
            &device_extension_infos,
        )?;
        Ok(SuitablePhysicalDevices {
            instance: self.clone(),
            devices,
            attributes: device_attributes,
            device_extensions,
            device_extension_infos,
        })
    }
}

impl Deref for Instance {

    type Target = VkInstance;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner.instance
    }
}

/// The structure returned from [`enumerating suitable physical devices`][1].
///
/// See the struct-level docs of [`Instance`] for examples.
///
/// [1]: Instance::enumerate_suitable_physical_devices
pub struct SuitablePhysicalDevices {
    pub(super) instance: Instance,
    pub(super) devices: Vec32<PhysicalDevice>,
    pub(super) attributes: DeviceAttributes,
    pub(super) device_extensions: Vec32<ext::DeviceExtensionObj>,
    pub(super) device_extension_infos: Vec32<ext::DeviceExtensionInfo>,
}

impl SuitablePhysicalDevices {

    /// Returns an iterator over all suitable physical devices.
    #[inline]
    pub fn enumerate(&self) -> impl Iterator<Item = (u32, &PhysicalDevice)> {
        self.devices
        .iter().enumerate()
        .map(|(idx, d)| (idx as u32, d))
    }

    /// Gets the [`physical device`][1] at `idx`.
    ///
    /// Returns [`None`] if the index is out of bounds.
    ///
    /// [1]: PhysicalDevice
    #[inline]
    pub fn get(&self, idx: u32) -> Option<&PhysicalDevice> {
        self.devices.get(idx as usize)
    }

    /// Creates a [`logical device`][1] needed for creating [`Gpu`].
    ///
    /// # Parameters
    /// - `device_idx`: The index a physical device.
    /// - `queue_create_infos`: Specifies which which queues to create.
    ///
    /// # Panics
    /// Panics if `device_idx` is greater than or equal to the number of enumerated physical devices.
    ///
    /// # Valid usage
    /// - `queue_create_infos` *must* not be empty.
    /// - Each [`create info`][2] in `queue_create_infos` *must* have a valid queue family index for
    ///   the specified device and the queue index *must* be less than the queue count of that queue
    ///   family.
    ///
    /// [1]: Device
    /// [2]: DeviceQueueCreateInfo
    #[inline]
    pub fn create_device<'a, Q>(
        &self,
        device_idx: u32,
        queue_create_infos: Q,
    ) -> Result<Device>
        where
            Q: IntoIterator<Item = DeviceQueueCreateInfo<'a>>,
            Q::IntoIter: ExactSizeIterator,
    {
        Device::new(self, device_idx, queue_create_infos)
    }
}

impl Index<u32> for SuitablePhysicalDevices {

    type Output = PhysicalDevice;

    #[inline]
    fn index(&self, index: u32) -> &Self::Output {
        &self.devices[index as usize]
    }
}

impl Drop for Inner {

    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.library.destroy_instance(
                &self.instance,
                None,
            );
        }
    }
}

fn get_required_instance_extensions(
    handle: Option<RawDisplayHandle>,
    out: &mut Vec32::<(&CStr, bool)>,
) -> Result<()>
{
    if let Some(handle) = handle {
        out.push((get_surface_capabilities2::NAME, true));
        out.push((surface::NAME, true));
        let ext = tuhka::window::required_instance_extensions(
            handle
        ).ok_or_else(|| Error::just_context("unsupported platform"))?;
        out.extend(ext
            .iter()
            .map(|&name| (name, true))
        );
    }
    Ok(())
}

fn verify_instance_layers<'a, L>(
    library: &tuhka::Library,
    layers: L,
    found: &mut Vec32<*const i8>,
    found_hash: &mut AHashSet<&'a CStr>,
) -> Result<()>
    where L: IntoIterator<Item = InstanceLayer<'a>>
{
    let mut available = unsafe {
        vec![Default::default(); library.enumerate_instance_layer_properties_len()
            .context("failed to enumerate instance layers")?.value as usize
        ]
    };
    unsafe { library
        .enumerate_instance_layer_properties(&mut available)
        .context("failed to enumerate instance layers")?
    };
    for layer in layers {
        if !available
            .iter()
            .any(|a| layer.name == a.layer_name_as_cstr().unwrap())
        {
            if layer.is_required {
                return Err(Error::just_context(format!("instance layer {layer} not present")))
            } else {
                log::warn!("optional instance layer {layer} not present");
            }
        } else {
            found.push(layer.name.as_ptr());
            found_hash.insert(layer.name);
        }
    }
    Ok(())
}

fn verify_instance_extensions<'a>(
    library: &tuhka::Library,
    extensions: &[(&'a CStr, bool)],
    found: &mut Vec32<*const i8>,
    found_hash: &mut AHashSet<&'a CStr>,
) -> Result<()>
{
    let available = unsafe {
        let mut av = vec![Default::default(); library.enumerate_instance_extension_properties_len(
            None
        ).context("failed to enumerate instance layers")?.value as usize];
        library.enumerate_instance_extension_properties(None, &mut av)
        .context("failed to enumerate instance layers")?;
        av
    };
    for &(extension, required) in extensions {
        if !available
            .iter()
            .any(|a| extension == a.extension_name_as_cstr().unwrap())
        {
            if required {
                return Err(Error::just_context(format!(
                    "instance extension {extension:?} not present"
                )))
            } else {
                log::warn!("optional instance extension {extension:?} not present");
            }
        } else {
            found.push(extension.as_ptr());
            found_hash.insert(extension);
        }
    }
    Ok(())
}

use core::{
    hash::{self, Hash},
    fmt::{self, Display, Debug},
};

use tuhka::vk;

use leimu_mem::{
    vec::Vec32,
    vec32,
};

use crate::{
    gpu::prelude::*,
    sync::{atomic::AtomicU64, *},
    error::*,
};

#[derive(Clone, Copy)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub min_image_transfer_granularity: Dimensions,
}

pub struct UninitDeviceQueue {
    queue: Option<DeviceQueue>,
}

impl UninitDeviceQueue {

    pub fn get(self) -> Result<DeviceQueue> {
        self.queue
            .ok_or_else(|| Error::just_context("device queue not initialized"))
    }
}

/// Parameters used for creating a [`DeviceQueue`].
pub struct DeviceQueueCreateInfo<'a> {
    pub(super) out: &'a mut UninitDeviceQueue,
    pub(super) name: Arc<str>,
    pub(super) family_index: u32,
    pub(super) queue_index: u32,
}

impl<'a> DeviceQueueCreateInfo<'a> {

    /// Creates the parameters.
    ///
    /// # Parameters
    /// - `out`: A mutable reference to an [`uninit`][1] [`DeviceQueue`] where the resultant handle
    ///   will be stored.
    /// - `name`: The debug-name assigned to the queue.
    /// - `family_index`: The [`queue family`][2] index of the queue.
    /// - `queue_index`: The queue index within the [`queue family`][2] of this queue.
    ///
    /// # Valid usage
    /// - `family_index` *must* be a valid queue family index for the [`Device`].
    /// - `queue_index` *must* be less than the [`number of queues`][3] in the queue family.
    ///
    /// [1]: DeviceQueue::uninit
    /// [2]: QueueFamilyProperties
    /// [3]: QueueFamilyProperties::queue_count
    pub fn new(
        out: &'a mut UninitDeviceQueue,
        name: &str,
        family_index: u32,
        queue_index: u32,
    ) -> Self {
        Self {
            out,
            name: name.into(),
            family_index,
            queue_index,
        }
    }
}

struct DeviceQueueInner {
    handle: vk::Queue,
    device_id: DeviceId,
    device_queue_index: u32,
    family_index: u32,
    family_properties: QueueFamilyProperties,
    queue_index: u32,
}

/// Represents a [`device queue`][1].
///
/// Contains metadata and handle of a queue.
///
/// [1]: vk::Queue
#[derive(Clone)]
pub struct DeviceQueue {
    id: u64,
    name: Arc<str>,
    inner: Arc<DeviceQueueInner>,
}

impl Debug for DeviceQueue {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeviceQueue")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

impl Display for DeviceQueue {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.name)
    }
}

static DEVICE_QUEUE_ID: AtomicU64 = AtomicU64::new(0);

impl DeviceQueue {

    /// Creates an unitialized [`DeviceQueue`], which can be used as the `out` parameter of
    /// [`DeviceQueueCreateInfo`].
    ///
    /// # Example
    /// ``` rust
    /// use leimu::gpu;
    ///
    /// let mut devices: gpu::SuitablePhysicalDevices = ...;
    /// let mut queue = gpu::DeviceQueue::uninit();
    /// let create_info = gpu::DeviceQueueCreateInfo::new(
    ///     &mut queue,
    ///     "queue",
    ///     0, 0,
    /// );
    /// let device = devices.create_device(
    ///     0, [create_info],
    /// )?;
    /// let queue = queue.get()?;
    /// ```
    #[inline]
    pub fn uninit() -> UninitDeviceQueue {
        UninitDeviceQueue { queue: None }
    }

    pub(super) unsafe fn new<'a>(
        device_id: DeviceId,
        device: &VkDevice,
        device_queue_index: u32,
        create_info: DeviceQueueCreateInfo<'a>,
        family_properties: QueueFamilyProperties,
    ) -> Self {
        let id = DEVICE_QUEUE_ID.fetch_add(1, atomic::Ordering::AcqRel);
        let s = Self {
            id,
            name: create_info.name.clone(),
            inner: Arc::new(DeviceQueueInner {
                handle: unsafe {
                    device.get_device_queue(
                        create_info.family_index,
                        create_info.queue_index,
                    )
                },
                device_id,
                device_queue_index,
                family_index: create_info.family_index,
                family_properties,
                queue_index: create_info.queue_index,
            })
        };
        create_info.out.queue = Some(s.clone());
        s
    }

    #[inline]
    pub(super) fn handle(&self) -> vk::Queue {
        self.inner.handle
    }

    /// Returns the [`DeviceId`] of the [`device`][1] this queue belongs to.
    ///
    /// [1]: Device
    #[inline]
    pub fn device_id(&self) -> DeviceId {
        self.inner.device_id
    }

    /// Returns the index of this queue in the [`device`][1] this queue belongs to.
    ///
    /// Note that this is different from the [`queue family index`][2] and [`queue index`][3] of
    /// this queue.
    ///
    /// [1]: Device
    /// [2]: Self::family_index
    /// [3]: Self::queue_index
    #[inline]
    pub fn device_queue_index(&self) -> u32 {
        self.inner.device_queue_index
    }

    /// Returns the queue family index of this queue.
    #[inline]
    pub fn family_index(&self) -> u32 {
        self.inner.family_index
    }

    /// Returns the [`queue family properties`][1] of this queue.
    ///
    /// [1]: QueueFamilyProperties
    #[inline]
    pub fn queue_family_properties(&self) -> &QueueFamilyProperties {
        &self.inner.family_properties
    }

    /// Returns the [`QueueFlags`] of the queue family of this queue.
    #[inline]
    pub fn queue_flags(&self) -> QueueFlags {
        self.inner.family_properties.queue_flags
    }

    /// Returns the queue index within the [`queue family`][1] of this queue.
    ///
    /// [1]: Self::family_index
    #[inline]
    pub fn queue_index(&self) -> u32 {
        self.inner.queue_index
    }
}

impl PartialEq for DeviceQueue {

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DeviceQueue {}

impl Hash for DeviceQueue {

    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Clone)]
pub(crate) struct QueueFamilies {
    queue_family_properties: Arc<[QueueFamilyProperties]>,
}

mod enumerated {

    use super::*;

    pub struct Iter<'a> {
        pub(super) iter: ::core::iter::Enumerate<::core::slice::Iter<'a, QueueFamilyProperties>>,
    }
    
    impl<'a> Iterator for Iter<'a> {

        type Item = (u32, &'a QueueFamilyProperties);

        fn next(&mut self) -> Option<Self::Item> {
            let (idx, prop) = self.iter.next()?;
            Some((idx as u32, prop))
        }
    }
}

/// Enumerated [`queue families`][1].
///
/// [1]: QueueFamilyProperties
#[derive(Clone, Copy)]
pub struct EnumeratedQueueFamilies<'a> {
    pub(crate) properties: &'a [QueueFamilyProperties],
}

impl<'a> EnumeratedQueueFamilies<'a> {

    #[inline]
    pub fn as_slice(&self) -> &[QueueFamilyProperties] {
        self.properties
    }

    pub fn iter(&self) -> enumerated::Iter<'a> {
        enumerated::Iter {
            iter: self.properties.iter().enumerate()
        }
    }
}

impl<'a> IntoIterator for EnumeratedQueueFamilies<'a> {

    type IntoIter = enumerated::Iter<'a>;
    type Item = (u32, &'a QueueFamilyProperties);

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl QueueFamilies {

    pub fn new(
        physical_device: vk::PhysicalDevice,
        instance: &Instance,
    ) -> Self
    {
        let n_properties = unsafe { instance
            .get_physical_device_queue_family_properties2_len(physical_device)
        };
        let mut properties = vec32![Default::default(); n_properties];
        unsafe { instance
            .get_physical_device_queue_family_properties2(physical_device, &mut properties)
        }
        let properties: Arc<[_]> = properties
            .into_iter()
            .map(|p| {
                let p1 = p.queue_family_properties;
                QueueFamilyProperties {
                    queue_flags: QueueFlags::from_raw(p1.queue_flags.as_raw()),
                    queue_count: p1.queue_count,
                    min_image_transfer_granularity: p1.min_image_transfer_granularity.into(),
                }
            }).collect();
        Self {
            queue_family_properties: properties,
        }
    }

    #[inline]
    pub fn properties(&self) -> &[QueueFamilyProperties] {
        &self.queue_family_properties
    }

    pub fn get_create_infos<'a>(
        &self,
        create_infos: &[DeviceQueueCreateInfo],
        priorities: &'a mut Vec32<Vec32<f32>>,
    ) -> Result<Vec32<vk::DeviceQueueCreateInfo<'a>>> {
        let mut unique: Vec32<_> =
            create_infos.iter().map(|s| vk::DeviceQueueCreateInfo {
                queue_family_index: s.family_index,
                ..Default::default()
            }).collect();
        unique.sort_unstable_by_key(|a| a.queue_family_index);
        unique.dedup_by_key(|a| a.queue_family_index);
        priorities.resize(unique.len(), vec32![]);
        for (i, unique) in unique.iter_mut().enumerate() {
            let idx = unique.queue_family_index;
            unique.queue_count = create_infos
                .iter()
                .filter_map(|s| (s.family_index == idx).then_some(s.queue_index))
                .max()
                .unwrap() + 1;
            let priorities = &mut priorities[i];
            *priorities = vec32![1.0; unique.queue_count];
            unique.p_queue_priorities = priorities.as_ptr();
        }
        for unique in &unique {
            let properties = &self.queue_family_properties
                .get(unique.queue_family_index as usize)
                .ok_or_else(|| Error::just_context(format!(
                    "invalid queue family index {}", unique.queue_family_index,
                )))?;
            if unique.queue_count > properties.queue_count {
                return Err(Error::just_context(format!(
                    "{} queues requested, when queue family {} only has {} queues",
                    unique.queue_count, unique.queue_family_index, properties.queue_count,
                )))
            }
        }
        Ok(unique)
    }
}

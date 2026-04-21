use {
    core::{
        time::Duration,
        num::NonZeroU32,
    },
    leimu_mem::{
        vec::Vec32,
        vec32,
    },
};


use super::{
    ext,
    BaseDeviceFeatures,
};

#[derive(Clone)]
pub struct DeviceAttributes {
    pub(super) device_extensions: Vec32<ext::DeviceExtensionObj>,
    pub(super) command_workers: u32,
    pub(super) frame_timeout: Duration,
    pub(super) required_features: BaseDeviceFeatures,
}

impl DeviceAttributes {

    /// Adds a [`device extension`][1].
    ///
    /// [1]: ext::DeviceExtension
    #[inline(always)]
    pub fn with_device_extension<Ext>(mut self, extension: Ext) -> Self
        where Ext: ext::DeviceExtension,
    {
        self.device_extensions.push(extension.boxed().into());
        self
    }

    /// Sets the number of command pools used by the queue scheduler.
    ///
    /// The default is 4.
    #[inline(always)]
    pub fn with_command_workers(mut self, n: NonZeroU32) -> Self {
        self.command_workers = n.get();
        self
    }

    /// Sets the timeout used when waiting for work to finish per frame.
    ///
    /// The default is 2 seconds.
    #[inline(always)]
    pub fn with_frame_timeout(mut self, timeout: Duration) -> Self {
        self.frame_timeout = timeout;
        self
    }

    #[inline(always)]
    pub fn with_required_device_features(mut self, features: BaseDeviceFeatures) -> Self {
        self.required_features = features;
        self
    }
}

/// Creates default [`Attributes`] with no layers or extensions.
///
/// [`Queue requirements`][1] don't have a default, so you need to pass that here.
///
/// The [`requirements`][1] are used when selecting a [`physical device`][2] and when creating
/// [`device queues`][3].
///
/// [1]: QueueRequirements
/// [2]: PhysicalDevice
/// [3]: DeviceQueue
pub fn default_device_attributes() -> DeviceAttributes
{
    DeviceAttributes {
        device_extensions: vec32![],
        command_workers: 8,
        frame_timeout: Duration::from_secs(2),
        required_features: BaseDeviceFeatures::default(),
    }
}

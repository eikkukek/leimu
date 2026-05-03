use ::core::ffi::CStr;

#[cfg(feature = "event-loop")]
use {
    winit::event_loop::EventLoop,
    raw_window_handle::HasDisplayHandle,
};
use raw_window_handle::RawDisplayHandle;

use crate::{
    error::Context,
    sync::Arc,
};

use super::*;


enum Platform {
    #[cfg(feature = "event-loop")]
    EventLoop(Box<EventLoop<RunEvent>>),
    Headless(Option<RawDisplayHandle>),
}

/// The entry point of leimu.
///
/// Initializes the [`Vulkan library`][1] and sets up platform-dependent objects.
///
/// # Example
/// ``` rust
/// use leimu::{Entry, gpu};
///
/// #[cfg(feature = "event-loop")]
/// let entry = Entry::new()?;
///
/// #[cfg(not(feature = "event-loop"))]
/// let entry = Entry::headless(get_display_handle())?;
///
/// let instance = entry.create_instance(
///     "My app",
///     gpu::make_api_version(0, 1, 0, 0),
///     &[]
/// )?;
/// ```
///
/// [1]: tuhka::Library
/// [2]: crate::event_loop::EventLoop
pub struct Entry {
    platform: Platform,
    pub(super) vulkan: Arc<tuhka::Library>,
}

impl Entry {

    /// Creates the entry with an [`event loop`][1].
    ///
    /// [1]: crate::event_loop::EventLoop
    #[cfg(feature = "event-loop")]
    #[cfg_attr(docsrs, doc(cfg(feature = "event-loop")))]
    pub fn new() -> Result<Self> { 
        Ok(Self {
            platform: Platform::EventLoop(Box::new(EventLoop
                ::with_user_event()
                .build().context("failed to create event loop")?)),
            vulkan: Arc::new(unsafe {
                tuhka::Library::load()
            }.context("failed initialize Vulkan library")?),
        })
    }

    /// Creates the entry in headless mode without an event loop.
    pub fn headless(display: Option<RawDisplayHandle>) -> Result<Self> {
        Ok(Self {
            platform: Platform::Headless(display),
            vulkan: Arc::new(unsafe {
                tuhka::Library::load()
            }.context("failed initialize Vulkan library")?),
        })
    }

    pub(crate) fn raw_display_handle(&self) -> Result<Option<RawDisplayHandle>> {
        {
            match &self.platform {
                Platform::Headless(handle) => Ok(*handle),
                #[cfg(feature = "event-loop")]
                Platform::EventLoop(event_loop) => Ok(Some(event_loop
                    .display_handle()
                    .context("failed to get display handle")?
                    .as_raw()))
            }
        }
    }

    #[cfg(feature = "event-loop")]
    pub(crate) fn event_loop(&self) -> Option<&EventLoop<RunEvent>> {
        if let Platform::EventLoop(event_loop) = &self.platform {
            Some(event_loop)
        } else { None }
    }

    #[cfg(feature = "event-loop")]
    pub(crate) fn take_event_loop(&mut self) -> Option<EventLoop<RunEvent>> {
        let mut platform = Platform::Headless(None);
        ::core::mem::swap(&mut self.platform, &mut platform);
        if let Platform::EventLoop(event_loop) = platform {
            Some(*event_loop)
        } else {
            None
        }
    }

    /// Creates a [`Vulkan instance`][1], which is needed to create a [`Vulkan device`][2].
    ///
    /// Parameters:
    /// - `app_name`: The [`name`][3] of the application.
    /// - `app_version`: The [`version`][4] of the application.
    /// - `layers`: A reference to a slice containing [`instance layers`][5] to enable.
    ///
    /// There is currently no direct way to enable extra instance extensions.
    ///
    /// Only [`surface`][6] related instance extensions are enabled when not in pure headless mode.
    ///
    /// [1]: gpu::Instance
    /// [2]: gpu::Device
    /// [3]: tuhka::vk::ApplicationInfo::p_application_name
    /// [4]: tuhka::vk::ApplicationInfo::application_version
    /// [5]: gpu::InstanceLayer
    /// [6]: tuhka::vk::SurfaceKHR
    pub fn create_instance<'a, L>(
        &self,
        app_name: &CStr, 
        app_version: gpu::Version,
        layers: L,
    ) -> Result<gpu::Instance>
        where
            L: IntoIterator<Item = gpu::InstanceLayer<'a>>,
    {
        gpu::Instance::new(self, app_name, app_version, layers)
    }
}

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

pub struct Library {
    #[cfg(feature = "event-loop")]
    pub(super) event_loop: EventLoop<RunEvent>,
    #[cfg(not(feature = "event-loop"))]
    pub(super) display_handle: Option<RawDisplayHandle>,
    pub(super) vk_lib: Arc<tuhka::Library>,
}

impl Library {

    #[cfg(feature = "event-loop")]
    #[inline(always)]
    pub fn new() -> Result<Self> { 
        Ok(Self {
            event_loop: EventLoop
                ::with_user_event()
                .build().context("failed to create event loop")?,
            vk_lib: Arc::new(unsafe {
                tuhka::Library::load()
            }.context("failed initialize Vulkan")?),
        })
    }

    #[cfg(not(feature = "event-loop"))]
    pub fn new(display: Option<RawDisplayHandle>) -> Result<Self> {
        Ok(Self {
            display_handle: display,
            vk_lib: Arc::new(unsafe {
                tuhka::Library::load()
            }.context("failed initialize Vulkan")?),
        })
    }

    pub fn raw_display_handle(&self) -> Result<Option<RawDisplayHandle>> {
        #[cfg(feature = "event-loop")]
        {
            self.event_loop
                .display_handle()
                .context("failed to get display handle")
                .map(|h| Some(h.as_raw()))
        }

        #[cfg(not(feature = "event-loop"))]
        {
            Ok(self.display_handle)
        }
    }

    pub fn create_instance(
        &self,
        app_name: &str, 
        app_version: gpu::Version,
        layers: &[gpu::InstanceLayer<'_>],
    ) -> Result<gpu::Instance> {
        gpu::Instance::new(self, app_name, app_version, layers)
    }
}

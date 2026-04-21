#[cfg(feature = "event-loop")]
use {
    winit::event_loop::EventLoop,
    raw_window_handle::HasDisplayHandle,
};
use raw_window_handle::RawDisplayHandle;

use crate::{
    error::{Context, expand},
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
        log::init();
        log::info_fmt(|fmt| {
            fmt.text("INFO:  ", |spec| spec.with_color_spec(|spec| {
                spec.set_fg(Some(log::Color::Green)).set_bold(true);
            })).message(|spec| spec);
        });
        log::warn_fmt(|fmt| {
            fmt.text("WARN:  ", |spec| spec.with_color_spec(|spec| {
                spec.set_fg(Some(log::Color::Yellow)).set_bold(true);
            })).message(|spec| spec);
        });
        log::error_fmt(|fmt| {
            fmt.text("ERROR: ", |spec| spec.with_color_spec(|spec| {
                spec.set_fg(Some(log::Color::Red)).set_bold(true);
            })).message(|spec| spec);
        });
        log::debug_fmt(|fmt| {
            fmt.text("DEBUG: ", |spec| spec.with_color_spec(|spec| {
                spec.set_fg(Some(log::Color::Blue)).set_bold(true);
            })).message(|spec| spec);
        });
        log::trace_fmt(|fmt| {
            fmt.text("TRACE: ", |spec| spec.with_color_spec(|spec| {
                spec.set_fg(Some(log::Color::Rgb(130, 130, 130))).set_bold(true);
            })).message(|spec| spec);
        }); 
        if expand::ERROR_CAUSE_FMT.get().is_none() {
            let mut error_cause_fmt = log::LogFmt::default();
            log::LogFmtBuilder::new(&mut error_cause_fmt)
                .text("       caused by: ", |spec| spec.with_color_spec(|spec| {
                    spec.set_fg(Some(log::Color::Magenta)).set_bold(true);
                })).message(|spec| spec);
            expand::ERROR_CAUSE_FMT.set(log::custom_fmt(error_cause_fmt)).ok();
        }
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
        log::init();
        log::info_fmt(|fmt| {
            fmt.text("INFO:  ", |spec| spec.with_color_spec(|spec| {
                spec.set_fg(Some(log::Color::Green)).set_bold(true);
            })).message(|spec| spec);
        });
        log::warn_fmt(|fmt| {
            fmt.text("WARN:  ", |spec| spec.with_color_spec(|spec| {
                spec.set_fg(Some(log::Color::Yellow)).set_bold(true);
            })).message(|spec| spec);
        });
        log::error_fmt(|fmt| {
            fmt.text("ERROR: ", |spec| spec.with_color_spec(|spec| {
                spec.set_fg(Some(log::Color::Red)).set_bold(true);
            })).message(|spec| spec);
        });
        log::debug_fmt(|fmt| {
            fmt.text("DEBUG: ", |spec| spec.with_color_spec(|spec| {
                spec.set_fg(Some(log::Color::Blue)).set_bold(true);
            })).message(|spec| spec);
        });
        log::trace_fmt(|fmt| {
            fmt.text("TRACE: ", |spec| spec.with_color_spec(|spec| {
                spec.set_fg(Some(log::Color::Rgb(130, 130, 130))).set_bold(true);
            })).message(|spec| spec);
        }); 
        if expand::ERROR_CAUSE_FMT.get().is_none() {
            let mut error_cause_fmt = log::LogFmt::default();
            log::LogFmtBuilder::new(&mut error_cause_fmt)
                .text("       caused by: ", |spec| spec.with_color_spec(|spec| {
                    spec.set_fg(Some(log::Color::Magenta)).set_bold(true);
                })).message(|spec| spec);
            expand::ERROR_CAUSE_FMT.set(log::custom_fmt(error_cause_fmt)).ok();
        }
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
}

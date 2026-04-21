//! The error prelude of LeimLeimuu.
//!
//! # Includes 
//! - [`Error`] and [`EventError`] error types.
//! - [`Result`] and [`EventResult`] [`Result`][core::result::Result] types.
//! - [`Context`] and [`Tracked`] traits for error handling.

use core::{
    error,
    fmt::{self, Display, Debug, Formatter},
};

pub use leimu_error::*;

use crate::{
    sync::OnceLock,
};

pub mod expand {

    use super::*;

    use crate::log::{self, Result, error, warn};

    pub static ERROR_CAUSE_FMT: OnceLock<log::CustomFmt> = OnceLock::new();

    pub fn fn_expand_error(target: &str, err: Error) -> Result<bool> {
        if let Some(&error_cause_fmt) = ERROR_CAUSE_FMT.get() &&
            error!("{}", err)
        {
            let mut err: &dyn error::Error = &err;
            while let Some(source) = err.source() {
                err = source;
                log::log(
                    target,
                    log::LevelFmt::Other(error_cause_fmt, log::Level::Error),
                    format_args!("{}", err)
                )?;
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn fn_expand_warn(target: &str, err: Error) -> Result<bool> {
        if let Some(&error_cause_fmt) = ERROR_CAUSE_FMT.get() &&
            warn!("{}", err)
        {
            let mut err: &dyn error::Error = &err;
            while let Some(source) = err.source() {
                err = source;
                log::log(
                    target,
                    log::LevelFmt::Other(error_cause_fmt, log::Level::Warn),
                    format_args!("{}", err)
                )?;
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[macro_export]
    macro_rules! expand_error {
        ($err:expr) => {
            $crate::error::expand::fn_expand_error(module_path!(), $err)
                .unwrap_or(false)
        };
    }

    #[macro_export]
    macro_rules! expand_warn {
        ($err:expr) => {
            $crate::error::expand::fn_expand_warn(module_path!(), $err)
                .unwrap_or(false)
        };
    }
}

/// The event error type of Leimu.
#[derive(Error)]
#[display("{0}")]
pub struct EventError(
    #[source(self.0.source())]
    Error
);

impl EventError {

    #[track_caller]
    pub fn new<C>(err: impl error::Error + Send + Sync + 'static, ctx: C) -> Self
        where C: Display + Send + Sync + 'static
    {
        Self(build_error::new(err, ctx, Some(caller!())))
    }

    #[track_caller]
    pub fn just_context<C>(ctx: C) -> Self
        where C: Display + Send + Sync + 'static,
    {
        Self(build_error::just_context(ctx, Some(caller!())))
    }
}

impl From<Error> for EventError {

    #[track_caller]
    fn from(value: Error) -> Self {
        Self(value.with_location(caller!()))
    }
}

impl Debug for EventError {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Error as Debug>::fmt(&self.0, f)
    }
}

impl Tracked for EventError {

    fn location(&self) -> Option<Location> {
        self.0.location()
    }
}

/// The [`Result`][core::result::Result] type for event handlers.
pub type EventResult<T> = core::result::Result<T, EventError>;

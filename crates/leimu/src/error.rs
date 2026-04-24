//! The error prelude of Leimu.
//!
//! # Includes 
//! - [`Error`] and [`EventError`] [`error`][1] types.
//! - [`Result`] and [`EventResult`] [`result`][2] types.
//! - [`Context`] and [`Tracked`] traits for error handling.
//!
//! [1]: core::error::Error
//! [2]: core::result::Result

#![warn(missing_docs)]

pub mod location;
mod out;
mod tracked;
mod context;
mod event;

use core::error;

pub use leimu_proc::Error;

pub use location::Location;
pub use tracked::*;
pub use out::*;
pub use event::*;
pub use context::*;

/// [`Logs`][1] an entire error [`source`][2] chain.
///
/// [1]: log::error
/// [2]: error::Error::source
pub fn expand_error(err: Error) {
    log::error!("{}", err);
    let mut err: &dyn error::Error = &err;
    while let Some(source) = err.source() {
        err = source;
        log::error!("caused by: {err}");
    }
}

/// The [`Result`][1] type for event handlers.
///
/// [1]: core::result::Result
pub type EventResult<T> = core::result::Result<T, EventError>;

/// The [`Result`][1] type returned by Leimu functions.
///
/// [1]: core::result::Result
pub type Result<T> = core::result::Result<T, Error>;

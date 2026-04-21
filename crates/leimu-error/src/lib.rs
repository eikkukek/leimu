#[macro_use]
pub mod location;
mod tracked;
mod error;
mod context;

pub use location::Location;
pub use tracked::{Tracked};
pub use error::{Error, build_error};
pub use leimu_proc::Error;

pub use context::Context;

/// Type definition of [`Result`][core::result::Result] with [`Error`] as the [`Err`] type.
pub type Result<T> = core::result::Result<T, Error>;

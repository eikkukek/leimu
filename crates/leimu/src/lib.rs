//#![warn(missing_docs)]

pub mod core;
pub mod sync;
pub mod error;
pub mod executor;
pub mod gpu;

mod macros;

/// Base collections and allocators used internally by leimu.
///
/// A re-export of [`leimu_mem`].
pub mod mem {
    pub use leimu_mem::*;
}

mod entry;

pub use entry::Entry;

#[cfg(feature = "event-loop")]
mod leimu;

#[cfg(feature = "event-loop")]
pub use leimu::*;

pub use error::{Error, Result, EventError, EventResult};

#[inline]
pub fn default<T: Default>() -> T {
    T::default()
}

#[inline]
pub fn duration_secs(secs: f32) -> ::core::time::Duration {
    ::core::time::Duration::from_secs_f32(secs)
}

pub use executor::block_on;

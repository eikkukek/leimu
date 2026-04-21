//#![warn(missing_docs)]

pub mod error;
pub mod sync;
pub mod gpu;

mod macros;

pub use leimu_core as core;
pub use leimu_mem as mem;
pub use leimu_log as log;
pub use leimu_threads as threads;
pub use tuhka;

mod library;

pub use library::Library;

#[cfg(feature = "event-loop")]
mod leimu;

#[cfg(feature = "event-loop")]
pub use leimu::*;


pub use error::{Error, Result, EventError, EventResult};

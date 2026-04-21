//! Provides [`executor`] and [`sync`].
//!
//! Re-exports the [`futures`] crate and [`leimu_error`] as [`error`].

pub mod executor;
pub mod sync;

pub use futures;
pub use leimu_error as error;

//! The sync prelude of Leimu.

mod future_lock;
mod swap_lock;

pub(crate) use std::sync::{
    Arc, OnceLock, atomic,
};

pub(crate) use parking_lot::{
    RwLock, RwLockWriteGuard, RwLockReadGuard, Mutex,
};

pub use future_lock::FutureLock;
pub use swap_lock::*;

//! The sync prelude of Leimu.

mod future_lock;
mod swap_lock;

pub use std::sync::{
    Arc, OnceLock, LazyLock, atomic,
};

pub use parking_lot::{
    RwLock, RwLockWriteGuard, RwLockReadGuard,
    RwLockUpgradableReadGuard, MappedRwLockReadGuard, MappedRwLockWriteGuard,
    Mutex, MutexGuard, FairMutex, FairMutexGuard,
    MappedMutexGuard, MappedFairMutexGuard,
    ReentrantMutex, ReentrantMutexGuard, MappedReentrantMutexGuard,
    Condvar,
};

pub use future_lock::FutureLock;
pub use swap_lock::*;

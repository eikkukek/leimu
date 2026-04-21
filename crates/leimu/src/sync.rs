//! The sync prelude of Leimu.

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

pub use leimu_threads::sync::{FutureLock, SwapLock};

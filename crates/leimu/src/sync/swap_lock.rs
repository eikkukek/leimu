use std::sync::Arc;

use core::ops::Deref;

use parking_lot::Mutex;
use arc_swap::ArcSwap;

/// A simple lock that uses [`arc_swap::ArcSwap`] internally.
///
/// Loads are lock free, stores lock and clone the inner data.
#[derive(Default)]
pub struct SwapLock<T>
    where T: Clone
{
    mtx: Mutex<()>,
    data: ArcSwap<T>,
}

pub struct SwapLockGuard<T> {
    inner: arc_swap::Guard<Arc<T>>,
}

impl<T> SwapLock<T>
    where T: Clone
{

    #[inline]
    pub fn new(val: T) -> Self {
        Self {
            mtx: Mutex::new(()),
            data: ArcSwap::new(Arc::new(val)),
        }
    }

    #[inline]
    pub fn load(&self) -> SwapLockGuard<T> {
        SwapLockGuard { inner: self.data.load(), }
    }

    #[inline]
    pub fn swap(&self, value: T) -> T {
        let _lock = self.mtx.lock();
        let data = self.data.load().as_ref().clone();
        self.data.store(Arc::new(value));
        data
    }

    #[inline]
    pub fn modify<F, U>(&self, f: F) -> U
        where F: FnOnce(&mut T) -> U
    {
        let _lock = self.mtx.lock();
        let mut data = self.data.load().as_ref().clone();
        let u = f(&mut data);
        self.data.store(Arc::new(data));
        u
    }
}

impl<T> Deref for SwapLockGuard<T> {

    type Target = T;
    
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<T> SwapLockGuard<T> {

    #[inline]
    pub fn map<U, F>(self, f: F) -> MappedSwapLockGuard<T, U>
        where
            U: ?Sized,
            F: FnOnce(&T) -> &U,
    {
        let u = f(&self);
        MappedSwapLockGuard {
            u,
            _inner: self,
        }
    }

    #[inline]
    pub fn try_map<U, F, E>(self, f: F) -> Result<MappedSwapLockGuard<T, U>, E>
        where
            U: ?Sized,
            F: FnOnce(&T) -> Result<&U, E>
    {
        let u = f(&self)?;
        Ok(MappedSwapLockGuard {
            u,
            _inner: self,
        })
    }
}

pub struct MappedSwapLockGuard<T, U>
    where U: ?Sized
{
    u: *const U,
    _inner: SwapLockGuard<T>,
}

impl<T, U> Deref for MappedSwapLockGuard<T, U>
    where U: ?Sized
{

    type Target = U;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.u
        }
    }
}

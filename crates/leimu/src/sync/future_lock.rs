use std::{
    pin::Pin,
};

use core::{
    mem::{MaybeUninit},
    sync::atomic::{self, AtomicBool},
    result,
    error,
};

use futures::future::Future;

use crate::{
    error::{Result, Error},
};

use parking_lot::Mutex;

struct Inner<T, F, E = Error>
    where 
        T: 'static,
        F: Future<Output = result::Result<T, E>> + 'static,
        E: error::Error + Send + Sync + 'static,
{
    pending: MaybeUninit<F>,
    ready: MaybeUninit<T>,
}

/// A lock that waits for a future when it's first accessed.
pub struct FutureLock<T, F, E = Error>
    where
        F: Future<Output = result::Result<T, E>> + Send + Sync + 'static,
        T: 'static + Send + Sync,
        E: error::Error + Send + Sync + 'static,
{
    inner: Inner<T, F, E>,
    is_ready: AtomicBool,
    mtx: Mutex<i8>,
}

impl<T, F, E> FutureLock<T, F, E>
    where
        F: Future<Output = result::Result<T, E>> + Send + Sync,
        T: 'static + Send + Sync,
        E: error::Error + Send + Sync + 'static,
{
    
    #[inline(always)]
    pub fn new(f: F) -> Self {
        Self {
            inner: Inner {
                pending: MaybeUninit::new(f),
                ready: MaybeUninit::uninit(),
            },
            is_ready: AtomicBool::new(false),
            mtx: Mutex::new(0),
        }
    }
    
    /// Loads the inner value.
    ///
    /// No locking takes place if the future has finished.
    #[allow(clippy::await_holding_lock)]
    pub fn load(&self) -> Pin<Box<dyn Future<Output = Result<&T>> + '_ + Send + Sync>> {
        Box::pin(async move {
            if self.is_ready.load(atomic::Ordering::Relaxed) {
                unsafe {
                    Ok(self.inner.ready.assume_init_ref())
                }
            } else {
                let value = *self.mtx.lock();
                match value {
                    -1 => {
                        Result::<&T>::Err(Error::just_context("lock poisoned"))
                    },
                    1 => unsafe {
                        Result::<&T>::Ok(self.inner.ready.assume_init_ref())
                    },
                    2 => {
                        loop {
                            let value = *self.mtx.lock();
                            if value == 1 {
                                unsafe {
                                    return Ok(self.inner.ready.assume_init_ref())
                                }
                            } else if value == -1 {
                                return Result::<&T>::Err(Error::just_context("lock poisoned"))
                            }
                            std::hint::spin_loop();
                        }
                    },
                    0 => {
                        let ready = {
                            let mut guard = self.mtx.lock();
                            (*guard == 0).then(|| {
                                *guard = 2;
                                unsafe { self.inner.pending.assume_init_read() }
                            })
                        };
                        let Some(ready) = ready else {
                            return self.load().await
                        };
                        match ready.await {
                            Ok(ready) => unsafe {
                                let mut lock = self.mtx.lock();
                                self.inner.ready
                                    .as_ptr().cast_mut()
                                    .write(ready);
                                *lock = 1;
                                self.is_ready.store(true, atomic::Ordering::Relaxed);
                                Ok(self.inner.ready.assume_init_ref())
                            },
                            Err(err) => {
                                let mut lock = self.mtx.lock();
                                *lock = -1;
                                Err(Error::new(err, "future failed"))
                            }
                        }
                    },
                    _ => unreachable!()
                }
            }
        }) 
    }
}

impl<T, F, E> Drop for FutureLock<T, F, E>
    where
        F: Future<Output = result::Result<T, E>> + Send + Sync,
        T: 'static + Send + Sync,
        E: error::Error + Send + Sync + 'static,
{

    fn drop(&mut self) {
        let mut lock = self.mtx.lock();
        match *lock {
            1 => unsafe {
                self.inner.ready.assume_init_read();
            }
            0 => unsafe {
                let p = self.inner.pending.assume_init_read();
                std::thread::spawn(async move || {
                    p.await
                }).join().ok();
            }
            _ => {}
        }
        *lock = -1;
    }
}

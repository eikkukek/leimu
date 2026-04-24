//! Provides [`ThreadPool`], a simple job-stealing thread pool.

use std::{
    thread,
};
use std::sync::Arc;
use parking_lot::RwLock;
use core::pin::Pin;
use core::sync::atomic::{self, AtomicBool};

use futures::{FutureExt, task::{self, FutureObj}};
use crossbeam::{deque, channel};

pub use futures::task::{Spawn, SpawnExt};
pub use futures::executor::*;

use ahash::AHashMap;

use crate::error::{Error, Result, Context};

type Fut = Pin<Box<dyn Future<Output = ()> + Send>>;

struct Scheduler {
    injector: deque::Injector<Fut>,
    maybe_parked: RwLock<Vec<thread::Thread>>,
    shutdown: AtomicBool,
}

impl Scheduler {

    pub fn new(n_threads: usize) -> Self {
        Self {
            injector: deque::Injector::new(),
            maybe_parked: RwLock::new(Vec::with_capacity(n_threads)),
            shutdown: AtomicBool::new(false),
        }
    }
}

async fn worker(
    main_rx: channel::Receiver<Fut>,
    scheduler: Arc<Scheduler>,
    idx: usize,
    worker: deque::Worker<Fut>,
    stealers: Arc<[deque::Stealer<Fut>]>,
) {
    loop {
        if let Some(mut task) = worker.pop()
            .or_else(|| {
                if stealers
                    .iter()
                    .enumerate()
                    .filter(|(i, s)| *i == idx && s.len() > 4)
                    .any(|(_, s)| s.steal_batch(&worker).is_success()) ||
                    scheduler.injector.steal_batch(&worker).is_success()
                {
                    worker.pop()
                } else {
                    None
                }
            })
        {
            let mut pin = Pin::new(&mut task);
            let mut cx = task::Context::from_waker(task::noop_waker_ref());
            match pin.poll_unpin(&mut cx) {
                task::Poll::Ready(_) => {},
                task::Poll::Pending => {
                    worker.push(task)
                },
            }
        } else {
            let thread = thread::current();
            if !scheduler.maybe_parked.read()
                .iter()
                .any(|t| t.id() == thread.id())
            {
                scheduler.maybe_parked.write().push(thread);
            }
            thread::park();
            if scheduler.shutdown.load(atomic::Ordering::Acquire) {
                while let Some(mut task) = worker.pop()
                    .or_else(|| {
                        if stealers
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| *i == idx)
                            .any(|(_, s)| s.steal_batch(&worker).is_success()) ||
                            scheduler.injector.steal_batch(&worker).is_success()
                        {
                            worker.pop()
                        } else {
                            None
                        }
                    })
                {
                    let mut pin = Pin::new(&mut task);
                    let mut cx = task::Context::from_waker(task::noop_waker_ref());
                    match pin.poll_unpin(&mut cx) {
                        task::Poll::Ready(_) => {},
                        task::Poll::Pending => {
                            worker.push(task)
                        },
                    }
                }
                break;
            }
            if let Ok(task) = main_rx.recv()
            {
                worker.push(task);
            }
        }
    } 
}

struct Inner {
    senders: AHashMap<std::thread::ThreadId, channel::Sender<Fut>>,
    scheduler: Arc<Scheduler>,
    threads: Option<Vec<std::thread::JoinHandle<()>>>,
}

impl Inner {

    #[inline(always)]
    fn new(
        senders: AHashMap<std::thread::ThreadId, channel::Sender<Fut>>,
        scheduler: Arc<Scheduler>,
        threads: Vec<std::thread::JoinHandle<()>>,
    ) -> Self
    {
        Self {
            senders,
            scheduler,
            threads: Some(threads),
        }
    }
}

impl Drop for Inner {

    fn drop(&mut self) {
        self.scheduler.shutdown.store(true, atomic::Ordering::Release);
        if let Some(threads) = self.threads.take() {
            for handle in &threads {
                handle.thread().unpark();
            }
            for handle in threads {
                handle.join().unwrap();
            }
        } 
    }
}

/// A simple job-stealing thread pool.
///
/// Implements [`futures::task::Spawn`], which is re-exported in this module along with
/// [`futures::task::SpawnExt`].
///
/// The thread pool returned by [`ThreadPool::new`] returns a clonable handle to the thread pool
/// itself. Cloning the handle will create a new reference to the inner thread pool.
/// ```
#[derive(Clone)]
pub struct ThreadPool {
    inner: Arc<RwLock<Inner>>,
}

impl Spawn for ThreadPool {

    fn spawn_obj(&self,
        future: FutureObj<'static, ()>
    ) -> core::result::Result<(), task::SpawnError>
    {
        let inner = self.inner.write();
        if let Some(unpark) = inner.scheduler.maybe_parked.write().pop() &&
            let Some(tx) = inner.senders.get(&unpark.id())
        {
            unpark.unpark();
            tx.send(future.boxed()).expect("failed to send");
        } else {
            println!("here");
            inner.scheduler.injector.push(future.boxed());
        }
        Ok(())
    }
}

impl ThreadPool {
    
    /// Creates a new [`ThreadPool`].
    pub fn new() -> Result<Self> {
        let n_threads = thread::available_parallelism()
            .map(|n| n.get().saturating_sub(1).max(1))
            .unwrap_or(1);
        let sheduler = Arc::new(Scheduler::new(n_threads));
        let workers: Vec<_> = (0..n_threads)
            .map(|i| {
                (i, deque::Worker::new_fifo())
            }).collect();
        let stealers: Arc<[_]> = workers
            .iter()
            .map(|w| w.1.stealer())
            .collect();
        let mut senders = AHashMap::default();
        let mut threads = Vec::new();
        for (idx, w) in workers.into_iter() {
            let stealers = stealers.clone();
            let scheduler = sheduler.clone();
            let (tx, rx) = channel::bounded(0);
            let thread = thread::Builder::new()
                .name(format!("leimu_worker_{}", idx))
                .spawn(move || {
                    block_on(worker(
                        rx,
                        scheduler,
                        idx,
                        w,
                        stealers,
                    ));
                }).context("failed to create worker")?;
            if senders.insert(
                thread.thread().id(),
                tx
            ).is_some() {
                return Err(Error::just_context(
                    "duplicate thread ids"
                ))
            }
            threads.push(thread);
        }
        Ok(ThreadPool {
            inner: Arc::new(RwLock::new(Inner::new(
                senders,
                sheduler,
                threads.into_iter().collect()
            )))
        })
    }

    /// Returns the number of parked (inactive) threads.
    pub fn parked_threads(&self) -> usize {
        self.inner
            .read().scheduler.maybe_parked
            .read().len()
    }

    #[inline(always)]
    pub fn worker_threads(&self) -> Vec<std::thread::ThreadId> {
        self.inner.read().senders
            .keys().copied()
            .collect()
    }

    pub fn shutdown(&self) {
        let mut inner = self.inner.write();
        inner.scheduler.shutdown.store(true, atomic::Ordering::Release);
        if let Some(threads) = inner.threads.take() {
            for handle in &threads {
                handle.thread().unpark();
            }
            for handle in threads {
                handle.join().unwrap();
            }
        }
    }
}

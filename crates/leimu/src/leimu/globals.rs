mod data;
mod r#ref;

use core::cell::UnsafeCell;

use crate::event_loop;

use crate::error;

use data::Data;
pub use r#ref::Ref;

pub struct Globals<'a> {
    data: UnsafeCell<Vec<Data<'a>>>,
    initialized: UnsafeCell<bool>,
}

impl<'a> Globals<'a> {

    pub(crate) fn new() -> Self {
        Self {
            data: Default::default(),
            initialized: UnsafeCell::new(false),
        }
    }

    #[must_use]
    pub fn add<T, F>(
        &self,
        f: F,
    ) -> Ref<'_, T>
        where 
            T: 'a,
            F: FnOnce(&event_loop::ActiveEventLoop) -> error::EventResult<T> + 'a,
    {
        unsafe {
            assert!(!*self.initialized.get(), "globals can't be reused");
        }
        let data = unsafe {
            &mut *self.data.get()
        };
        data.push(Data::new(f));
        let t = data.last().unwrap().get_t();
        Ref {
            initialized: unsafe {
                &*self.initialized.get()
            },
            t,
        }
    }

    pub(crate) fn init(
        &self,
        win: &event_loop::ActiveEventLoop,
    ) -> error::Result<()> {
        unsafe {
            if *self.initialized.get() {
                return Err(error::Error
                    ::just_context("globals can't be reused")
                )
            }
        }
        for data in unsafe { &mut *self.data.get() } {
            data.init(win)?;
        }
        unsafe {
            *self.initialized.get() = true;
        }
        Ok(())
    }
}

impl<'a> Drop for Globals<'a> {

    fn drop(&mut self) {
        unsafe {
            if *self.initialized.get() {
                for data in &mut *self.data.get() {
                    data.drop_t();
                }
            }
        }
    }
}

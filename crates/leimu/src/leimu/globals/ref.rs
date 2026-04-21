use core::ops::{Deref, DerefMut};

pub struct Ref<'a, T> {
    pub(super) initialized: &'a bool,
    pub(super) t: *mut T,
}

unsafe impl<T> Sync for Ref<'_, T> where T: Sync {}

impl<'a, T> Deref for Ref<'a, T> {

    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        if !self.initialized {
            panic!("value not initialized, OnInit needs to be passed to Leimu before using Ref")
        }
        unsafe {
            &*self.t
        }
    }
}

impl<'a, T> DerefMut for Ref<'a, T> {
    
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        if !self.initialized {
            panic!("value not initialized, OnInit needs to be passed to Leimu before using Ref")
        }
        unsafe {
            &mut *self.t
        }
    }
}

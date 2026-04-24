use core::{
    ops::Deref,
    ptr,
};

pub trait OptionExt<T> {

    /// [`get_or_insert_with`][1] with a closure that may fail.
    ///
    /// This will get removed when the [`core version`][2] gets stabilized.
    ///
    /// [1]: Option::get_or_insert_with
    /// [2]: Option::get_or_try_insert_with
    fn get_or_try_insert_with<E, F>(&mut self, f: F) -> Result<&mut T, E>
        where F: FnOnce() -> Result<T, E>;

    /// [`unwrap_or_else`][1] with a closure that may fail.
    ///
    /// [1]: Option::unwrap_or_else
    fn unwrap_or_try_else<F, E>(self, f: F) -> Result<T, E>
        where F: FnOnce() -> Result<T, E>;

    /// Edits the value contained if [`Some`].
    fn edit<F>(&mut self, f: F)
        where F: FnOnce(&mut T);

    /// Gets a pointer to the value contained if [`Some`] and a null pointer otherwise.
    /// # Example
    /// ``` rust
    /// use nox_mem::option::OptionExt;
    ///
    /// let value = 10;
    /// let opt = Some(&value);
    /// assert!(!opt.as_ptr().is_null());
    /// assert!(None::<&i32>.as_ptr().is_null());
    /// ```
    fn as_ptr(&self) -> *const <T as Deref>::Target
        where
            T: Deref,
            <T as Deref>::Target: Sized;
}

impl<T> OptionExt<T> for Option<T> {

    #[inline]
    fn get_or_try_insert_with<E, F>(&mut self, f: F) -> Result<&mut T, E>
        where F: FnOnce() -> Result<T, E>
    {
        match self {
            Self::Some(t) => Ok(t),
            None => {
                let t = f()?;
                Ok(self.insert(t))
            }
        }
    }

    #[inline]
    fn unwrap_or_try_else<F, E>(self, f: F) -> Result<T, E>
        where F: FnOnce() -> Result<T, E>
    {
        match self {
            Self::Some(t) => Ok(t),
            None => {
                let t = f()?;
                Ok(t)
            }
        }
    }

    #[inline]
    fn edit<F>(&mut self, f: F)
        where F: FnOnce(&mut T) 
    {
        if let Some(t) = self {
            f(t)
        }
    }

    #[inline]
    fn as_ptr(&self) -> *const <T as Deref>::Target
        where
            T: Deref,
            <T as Deref>::Target: Sized
    {
        match self.as_ref() {
            Some(value) => value.deref(),
            None => ptr::null(),
        }
    }
}

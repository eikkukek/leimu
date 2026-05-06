mod option;
mod iter;
mod slice;
mod collections;

pub use option::OptionExt;
pub use iter::TryExtend;
pub use slice::SliceCast;
pub use collections::EntryExt;

/// A trait for converting data to bytes.
pub trait AsBytes {

    /// Converts self to a slice over [`u8`].
    fn as_bytes(&self) -> &[u8];

    /// Converts a mutable reference of self to a mutable slice over [`u8`].
    ///
    /// # Safety
    /// All writes to the slice are unsafe and it is up to the user to ensure those writes don't result
    /// in invalid values of self.
    unsafe fn as_bytes_mut(&mut self) -> &mut [u8];
}

impl<T> AsBytes for [T] {

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            ::core::slice::from_raw_parts(
                self.as_ptr() as *const u8,
                size_of_val(self)
            )
        }
    }

    #[inline]
    unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            ::core::slice::from_raw_parts_mut(
                self.as_mut_ptr() as *mut u8,
                size_of_val(self)
            )
        }
    }
}

impl<T> AsBytes for T
    where T: Sized
{

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            ::core::slice::from_raw_parts(
                <*const Self>::cast(self),
                size_of::<Self>()
            )
        }
    }

    #[inline]
    unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
         unsafe {
            ::core::slice::from_raw_parts_mut(
                <*mut Self>::cast::<>(self),
                size_of::<Self>()
            )
        }       
    }
}


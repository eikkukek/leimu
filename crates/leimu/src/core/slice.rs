//! An extension of [`slice`][1].
//!
//! # New methods
//! - [`as_bytes`] and [`as_bytes_mut`]
//! - [`value_as_bytes`] and [`value_as_bytes_mut`]
//! - [`cast`]
//!
//! [1]: core::slice

pub use core::slice::*;

/// Converts a slice over [`T`] to a slice over [`u8`].
#[inline]
pub fn as_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe {
        from_raw_parts(slice.as_ptr() as *const u8, size_of_val(slice))
    }
}

/// Converts a mutable slice over [`T`] to a mutable slice over [`u8`].
/// # Safety
/// All writes to the slice are unsafe and it is up to the user to ensure those writes don't result
/// in invalid values of [`T`].
#[inline]
pub unsafe fn as_bytes_mut<T>(slice: &mut [T]) -> &mut [u8] {
    unsafe {
        from_raw_parts_mut(slice.as_ptr() as *mut u8, size_of_val(slice))
    }
}

/// Converts a value of [`T`] to a slice of [`u8`].
#[inline]
pub fn value_as_bytes<T>(value: &T) -> &[u8] {
    unsafe {
        from_raw_parts(value as *const T as *const u8, size_of::<T>())
    }
}

/// Converts a value of [`T`] to a slice over [`u8`].
/// # Safety
/// All writes to the slice are unsafe and it is up to the programmer to ensure those writes don't
/// result in an invalid value of [`T`].
#[inline]
pub unsafe fn value_as_bytes_mut<T>(value: &mut T) -> &mut [u8] {
    unsafe {
        from_raw_parts_mut(value as *mut T as *mut u8, size_of::<T>())
    }
}

/// Converts a slice over `T` to a slice over `U`.
///
/// Alignments and sizes be compatible, so that `U` won't be misaligned and that the size of
/// the resultant slice will be the same as that of the original slice in bytes.
///
/// # Safety
/// This is unsafe because there's no guarantee that the values of `U` in the resultant slice will
/// be valid values.
#[inline]
pub unsafe fn cast<T: Copy, U: Copy>(slice: &[T]) -> Option<&[U]> {
    if !size_of::<T>().is_multiple_of(size_of::<U>()) || align_of::<T>() < align_of::<U>() {
        None
    }
    else {
        unsafe {
            Some(from_raw_parts(slice.as_ptr().cast(), size_of_val(slice) / size_of::<U>()))
        }
    }
}

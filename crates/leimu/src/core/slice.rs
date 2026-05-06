use core::slice::*;

pub trait SliceCast<T>
    where T: Copy
{
    /// Converts a slice over `T` to a slice over `U`.
    ///
    /// Alignments and sizes be compatible, so that `U` won't be misaligned and that the size of
    /// the resultant slice will be the same as that of the original slice in bytes.
    ///
    /// # Safety
    /// This is unsafe because there's no guarantee that the values of `U` in the resultant slice will
    /// be valid values.
    unsafe fn cast<U: Copy>(&self) -> Option<&[U]>;
}

impl<T> SliceCast<T> for [T]
    where T: Copy
{

    #[inline]
    unsafe fn cast<U: Copy>(&self) -> Option<&[U]> {
        if !size_of::<T>().is_multiple_of(size_of::<U>()) || align_of::<T>() < align_of::<U>() {
            None
        }
        else {
            unsafe {
                Some(from_raw_parts(
                    self.as_ptr().cast(),
                    size_of_val(self) / size_of::<U>()
                ))
            }
        }
    }
}

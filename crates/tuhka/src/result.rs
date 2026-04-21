use crate::vk;

#[derive(Clone, Copy)]
pub struct Success<T> {
    pub value: T,
    pub result: vk::Result,
}

impl<T> Success<T> {

    #[inline]
    pub fn new(value: T, result: vk::Result) -> Self {
        Self {
            value,
            result,
        }
    }

    #[inline]
    pub fn with_value<U>(self, value: U) -> Success<U> {
        Success { value, result: self.result }
    }
}

pub type VkResult<T> = Result<Success<T>, vk::Result>;

impl vk::Result {

    #[inline]
    pub fn result(
        self,
        success: &[vk::Result],
    ) -> VkResult<()>
    {
        if success.contains(&self) {
            Ok(Success {
                value: (),
                result: self,
            })
        } else { Err(self) }
    }

    #[inline]
    pub fn result_with<T>(
        self,
        success: &[vk::Result],
        f: impl FnOnce() -> T
    ) -> VkResult<T> {
        if success.contains(&self) {
            Ok(Success {
                value: f(),
                result: self,
            })
        } else { Err(self) }
    }

    #[inline]
    pub fn result_with_value<T>(
        self,
        success: &[vk::Result],
        value: T,
    ) -> VkResult<T> {
        if success.contains(&self) {
            Ok(Success {
                value,
                result: self,
            })
        } else { Err(self) }
    }

    /// # Safety
    /// `value` *must* be initialized
    #[inline]
    pub unsafe fn result_with_assume_init<T>(
        self,
        success: &[vk::Result],
        value: core::mem::MaybeUninit<T>
    ) -> VkResult<T> {
        unsafe {
            if success.contains(&self) {
                Ok(Success {
                    value: value.assume_init(),
                    result: self,
                })
            } else { Err(self) }
        }
    }
}

impl core::error::Error for vk::Result {}

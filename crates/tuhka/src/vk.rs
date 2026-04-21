use core::{ffi, fmt, marker::PhantomData};

#[allow(
    non_camel_case_types, non_snake_case, non_upper_case_globals,
    clippy::all,
)]
pub mod vk_video;
mod includes;
mod base_types;
mod defines;
mod handles;
mod enums;
mod chainable;
mod unions;
mod structs;
mod type_defs;

pub use includes::*;
pub use base_types::*;
pub use defines::*;
pub use handles::*;
pub use enums::*;
pub use chainable::*;
pub use unions::*;
pub use structs::*;
pub use type_defs::*;

fn flag_display(
    bits: u64,
    known: &[(u64, &str)],
    f: &mut fmt::Formatter<'_>
) -> fmt::Result
{
    let mut first = false;
    let mut remaining = bits;
    for &(bit, name) in known {
        if bits & bit == bit {
            if first {
                write!(f, "{name}")?;
                first = false;
            } else {
                write!(f, "|{name}")?;
            }
        }
        remaining &= !bit;
    }
    if remaining != 0 {
        if first {
            write!(f, "0x{:x}", remaining)?;
        } else {
            write!(f, "|0x{:x}", remaining)?;
        }
    }
    Ok(())
}

#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct packed_u24_u8(u32);

impl packed_u24_u8 {

    #[inline]
    pub fn new(high: u32, low: u8) -> Self {
        Self((high & 0xff_ffff) | ((low as u32) << 24))
    }

    #[inline]
    pub fn high(self) -> u32 {
        self.0 & 0xff_ffff
    }

    #[inline]
    pub fn low(self) -> u8 {
        (self.0 >> 24) as u8
    }
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct packed_u9_u9_u6_u4_u4(u32);

impl packed_u9_u9_u6_u4_u4 {

    #[inline]
    pub fn new(a: u16, b: u16, c: u8, d: u8, e: u8) -> Self {
        Self(
            (a as u32 & 0x1ff) |
            ((b as u32 & 0x1ff) << 9) |
            ((c as u32 & 0x3f) << 18) |
            ((d as u32 & 0xf) << 24) |
            ((e as u32 & 0xf) << 28)
        )
    }

    #[inline]
    pub fn a(self) -> u16 {
        (self.0 & 0x1ff) as u16
    }

    #[inline]
    pub fn b(self) -> u16 {
        ((self.0 >> 9) & 0x1ff) as u16
    }

    #[inline]
    pub fn c(self) -> u8 {
        ((self.0 >> 18) & 0x3f) as u8
    }

    #[inline]
    pub fn d(self) -> u8 {
        ((self.0 >> 24) & 0xf) as u8
    }

    #[inline]
    pub fn e(self) -> u8 {
        ((self.0 >> 28) & 0xf) as u8
    }
}

#[allow(non_camel_case_types)]
trait PFN_Default {

    fn default() -> Self;
}

#[inline]
pub(crate) fn c_char_slice_until_nul(
    str: &[ffi::c_char],
) -> core::result::Result<&ffi::CStr, ffi::FromBytesUntilNulError>
{
    let bytes = unsafe {
        core::slice::from_raw_parts(str.as_ptr().cast(), str.len())
    };
    ffi::CStr::from_bytes_until_nul(bytes)
}

#[derive(Debug)]
pub struct WriteCStrToArrayError {
    pub c_str_len: usize,
    pub array_len: usize,
}

impl fmt::Display for WriteCStrToArrayError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "`CStr` len with nul-byte {} is greater than array len {}",
            self.c_str_len, self.array_len,
        )
    }
}
impl core::error::Error for WriteCStrToArrayError {}

pub(crate) fn write_c_str_to_c_char_slice(
    src: &ffi::CStr,
    dst: &mut [ffi::c_char]
) -> core::result::Result<(), WriteCStrToArrayError>
{
    let bytes = src.to_bytes_with_nul();
    let bytes = unsafe {
        core::slice::from_raw_parts(
            bytes.as_ptr().cast(),
            bytes.len()
        )
    };
    let array_len = dst.len();
    dst.get_mut(..bytes.len())
        .ok_or(WriteCStrToArrayError {
            c_str_len: bytes.len(),
            array_len, 
        })?.copy_from_slice(bytes);
    Ok(())
}

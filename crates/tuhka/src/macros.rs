#[macro_export]
macro_rules! define_non_dispatchable_handle {
    ($name:ident, $ty:ident, $doc_link:literal $(,)?) => {
        #[repr(transparent)]
        #[derive(Default, Clone, Copy)]
        #[doc = $doc_link]
        pub struct $name(u64);
        impl $name {
            pub const TYPE: ObjectType = ObjectType::$ty;
            #[inline]
            pub const fn null() -> Self {
                Self(0)
            }
            #[inline]
            pub fn as_raw(self) -> u64 {
                self.0
            }
            #[inline]
            pub fn from_raw(x: u64) -> Self {
                Self(x)
            }
        }
        impl fmt::Pointer for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }
    };
}
#[macro_export]
macro_rules! define_handle {
    ($name:ident, $ty:ident, $doc_link:literal $(,)?) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[doc = $doc_link]
        pub struct $name(*mut u8);
        impl $name {
            pub const TYPE: ObjectType = ObjectType::$ty;
            #[inline]
            pub const fn null() -> Self {
                Self(::core::ptr::null_mut())
            }
            #[inline]
            pub fn as_raw(self) -> u64 {
                self.0 as u64
            }
            #[inline]
            pub fn from_raw(x: u64) -> Self {
                Self(x as *mut u8)
            }
        }
        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}
        impl Default for $name {

            #[inline]
            fn default() -> Self {
                Self::null()
            }
        }
        impl fmt::Pointer for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Pointer::fmt(&self.0, f)
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(&self.0, f)
            }
        }
    };
}
#[macro_export]
macro_rules! bitflags {
    ($name:ident, $flag_type:ty) => {
        impl $name {
            #[inline]
            pub const fn empty() -> Self {
                Self(0)
            }
            #[inline]
            pub const fn from_raw(x: $flag_type) -> Self {
                Self(x)
            }
            #[inline]
            pub const fn as_raw(self) -> $flag_type {
                self.0
            }
            #[inline]
            pub const fn is_empty(self) -> bool {
                self.0 == 0
            }
            #[inline]
            pub const fn intersects(self, other: Self) -> bool {
                self.0 & other.0 != 0
            }
            #[inline]
            pub const fn contains(self, other: Self) -> bool {
                self.0 & other.0 == other.0
            }
        }
        impl ::core::fmt::Debug for $name {

            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }
        impl ::core::ops::BitOr for $name {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }
        impl ::core::ops::BitOrAssign for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0
            }
        }
        impl ::core::ops::BitAnd for $name {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }
        impl ::core::ops::BitAndAssign for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0
            }
        }
        impl ::core::ops::BitXor for $name {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }
        impl ::core::ops::BitXorAssign for $name {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0
            }
        }
        impl ::core::ops::Not for $name {
            type Output = Self;
            #[inline]
            fn not(self) -> Self {
                Self(!self.0)
            }
        }
    };
}

/// ``` rust
/// x |= y & (z as u32) << y.trailing_zeros()
/// ```
#[macro_export]
macro_rules! or_flag {
    ($flags:expr, $flag:expr, $value:expr $(,)?) => {
        $flags |= $flag & ($value as u32) << $flag.trailing_zeros();
    };
}

#[macro_export]
macro_rules! bitflags {
    ($(
        $(#[doc = $doc:literal])*
        $(#[default = $default:expr])?
        $vis:vis struct $name:ident: $repr:ty {$(
            $(#[doc = $var_doc:literal])*
            $var:ident = $value:expr
        ),+ $(,)?}
    )+) => {$(
        $(#[doc = $doc])*
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        $vis struct $name($repr);

        impl $name {

            /// Creates an empty bitmask.
            #[inline]
            pub const fn empty() -> Self {
                Self(0)
            }

            /// Sets the bitmask to zero.
            #[inline]
            pub const fn clear(&mut self) {
                self.0 = 0;
            }

            /// Returns the raw inner value.
            #[inline]
            pub const fn as_raw(self) -> $repr {
                self.0
            }

            /// Creates [`Self`] from a raw value.
            #[inline]
            pub const fn from_raw(x: $repr) -> Self {
                Self(x)
            }

            /// Returns whether the bitmask is empty.
            #[inline]
            pub const fn is_empty(self) -> bool {
                self.0 == 0
            }

            /// Returns whether `self` intersects with `other`.
            #[inline]
            pub const fn intersects(self, other: Self) -> bool {
                self.0 & other.0 != 0
            }

            /// Returns whether `self` is a superset of `other`.
            #[inline]
            pub const fn contains(self, other: Self) -> bool {
                self.0 & other.0 == other.0
            }

            const DISPLAY_LEN: usize = {
                let mut res = 0;
                $(
                    if ($value as $repr).count_ones() == 1 {
                        res += 1;
                    }
                )*
                res
            };

            #[allow(unused)]
            const DISPLAY_KNOWN: [($repr, &'static str); Self::DISPLAY_LEN] = {
                let mut res = [(0, ""); Self::DISPLAY_LEN];
                let mut idx = 0;
                $(
                    if ($value as $repr).count_ones() == 1 {
                        res[idx] = ($value, stringify!($var));
                        idx += 1;
                    }
                )*
                res
            };

            $(
                $(#[doc = $var_doc])*
                pub const $var: Self = Self($value);
            )+
        }
        $(
        impl Default for $name {
            
            #[inline]
            fn default() -> Self {
                $default
            }
        })?
        impl ::core::ops::Deref for $name {

            type Target = $repr;
    
            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
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
        impl ::core::fmt::Debug for $name {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>
            ) -> ::core::fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }
        impl ::core::fmt::Display for $name {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>
            ) -> ::core::fmt::Result {
                let mut first = true;
                let mut remaining = self.0;
                for &(bit, name) in &Self::DISPLAY_KNOWN {
                    if self.0 & bit == bit {
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
        }
    )+};
}

#[macro_export]
macro_rules! c_enum {
    ($(
        $(#[doc = $doc:literal])*
        $(#[default = $default:expr])?
        $vis:vis struct $name:ident: $repr:ty {$(
            $(#[doc = $var_doc:literal])*
            #[display($var_display:literal)]
            $var:ident = $value:expr
        ),+ $(,)?}
    )+) => {$(
        $(#[doc = $doc])*
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        $vis struct $name($repr);

        impl $name {

            /// Returns the the raw inner value
            #[inline]
            pub const fn as_raw(self) -> $repr {
                self.0
            }

            /// Creates [`Self`] from a raw value.
            ///
            /// # Safety
            /// This *must* not result in an invalid Vulkan enum.
            #[inline]
            pub const unsafe fn from_raw(x: $repr) -> Self {
                Self(x)
            }

            $(
                $(#[doc = $var_doc])*
                pub const $var: Self = Self($value);
            )+
        }
        $(impl Default for $name {
            #[inline]
            fn default() -> Self {
                $default
            }
        })?
        impl ::core::fmt::Debug for $name {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>
            ) -> ::core::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl ::core::fmt::Display for $name {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>
            ) -> ::core::fmt::Result {
                match *self {
                    $(Self::$var => write!(f, $var_display),)+
                    x => write!(f, "unknown({x})"),
                }
            }
        }
    )+};
}

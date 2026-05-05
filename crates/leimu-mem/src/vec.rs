//! New allocator-aware vector types.
//!
//! # New types
//! - [`AllocVecBase`] and its derivatives: A new vector type, which uses [`LocalAlloc`][1] for its
//!   allocations.
//! - [`NonNullVec`] and [`NonNullVec32`]: Two vector types, which are just wrappers around a
//!   pointer.
//! - [`Vec32`]: A new vector type, which stores its capacity and length as [`u32`]. Requires the
//!   "std" feature.
//! - [`ArrayVec`]: A wrapper around an array, interpreted as a vector.
//!
//! [1]: crate::alloc::LocalAlloc

mod pointer;
mod alloc_vec;
mod array_vec;
mod non_null;

pub use pointer::Pointer;
pub use alloc_vec::{
    AllocVecBase,
    DynVec, DynVec32,
    FixedVec, FixedVec32,
};
pub use non_null::{NonNullVec, NonNullVec32};
pub use array_vec::ArrayVec;

pub(crate) use alloc_vec::FixedPolicy32;

#[cfg(feature = "std")]
mod std_features {

    /// Macro for creating a new [`Vec32`].
    ///
    /// # Examples
    /// ``` rust
    /// use leimu_mem::vec32;
    ///
    /// let vec1 = vec32!["foo"; 10];
    /// let vec2 = vec32![0, 1, 2, 3, 4];
    /// ```
    #[macro_export]
    macro_rules! vec32 {
        [$v:expr; $n:expr] => {
            $crate::vec::Vec32::with_len_reserve_exact($n, $v)
        };
        [$($elem:expr),* $(,)?] => {
            $crate::vec::Vec32::from([$($elem),*])
        };
    }

    pub use super::alloc_vec::{Vec32, StdVecBase};
}

#[cfg(feature = "std")]
pub use std_features::*;

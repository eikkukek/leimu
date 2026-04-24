//! An extension of [`collections`][1].
//!
//! [1]: std::collections

pub use std::collections::*;

use hash_map::Entry;

/// An extension trait for [`Entry`].
pub trait EntryExt<'a, K, V> {

    /// [`or_insert_with`][1] with a closure that may fail.
    ///
    /// [1]: Entry::or_insert_with
    fn or_try_insert_with<F, E>(self, f: F) -> Result<&'a mut V, E>
        where
            F: FnOnce() -> Result<V, E>;

    /// [`or_insert_with_key`][1] with a closure that may fail.
    ///
    /// [1]: Entry::or_insert_with_key
    fn or_try_insert_with_key<F, E>(self, f: F) -> Result<&'a mut V, E>
        where
            F: FnOnce(&K) -> Result<V, E>;
}

impl<'a, K, V> EntryExt<'a, K, V> for Entry<'a, K, V> {

    #[inline]
    fn or_try_insert_with<F, E>(self, f: F) -> Result<&'a mut V, E>
        where
            F: FnOnce() -> Result<V, E>
    {
        match self {
            Self::Occupied(occ) => {
                Ok(occ.into_mut())
            },
            Self::Vacant(vac) => {
                let value = f()?;
                Ok(vac.insert(value))
            }
        }
    }

    #[inline]
    fn or_try_insert_with_key<F, E>(self, f: F) -> Result<&'a mut V, E>
        where
            F: FnOnce(&K) -> Result<V, E>
    {
        match self {
            Self::Occupied(occ) => {
                Ok(occ.into_mut())
            },
            Self::Vacant(vac) => {
                let value = f(vac.key())?;
                Ok(vac.insert(value))
            }
        }
    }
}

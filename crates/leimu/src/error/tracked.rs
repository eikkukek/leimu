use crate::caller;

use super::Location;

/// Trait for types that track a [`Location`].
pub trait Tracked {
    
    /// Gets the tracked [`Location`] of self, if any.
    fn location(&self) -> Option<Location>;

    /// Gets the tracked [`Location`] of self, or the caller of this function.
    #[track_caller]
    #[inline(always)]
    fn or_this(&self) -> Location
    {
        self.location()
            .unwrap_or_else(|| caller!())
    }
}

impl Tracked for Location {

    #[inline(always)]
    fn location(&self) -> Option<Location> {
        Some(*self)
    }
}

impl Tracked for Option<Location> {

    #[inline(always)]
    fn location(&self) -> Option<Location> {
        *self
    }

    #[track_caller]
    #[inline(always)]
    fn or_this(&self) -> Location {
        self.unwrap_or_else(|| caller!())
    }
}

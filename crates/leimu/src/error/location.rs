//! A module used by the [`caller`][1] and [`location`][2] macros.
//!
//! [1]: crate::caller
//! [2]: crate::location

use core::fmt::{self, Debug, Display};

/// A wrapper around [`Location`][1].
///
/// [1]: core::panic::Location
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location(&'static core::panic::Location<'static>);

impl Display for Location {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Location {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <core::panic::Location<'_> as Debug>::fmt(self.0, f)
    }
}

/// Used by the [`location`][1] macro.
///
/// [1]: crate::location
#[inline]
#[track_caller]
pub fn location() -> Location {
    Location(core::panic::Location::caller())
}

/// Used by the [`caller`][1] macro.
///
/// [1]: crate::caller
#[inline]
pub fn new(loc: &'static core::panic::Location<'static>) -> Location {
    Location(loc)
}

/// Gets the current [`Location`][1].
///
/// [1]: core::panic::Location
#[macro_export]
macro_rules! location {
    () => {
        $crate::error::location::location()
    };
}

/// Gets the [`location`][1] of the caller of the current function that has the `track_caller`
/// attribute.
///
/// [1]: core::panic::Location
#[macro_export]
macro_rules! caller {
    () => {
        $crate::error::location::new(core::panic::Location::caller())
    };
}

use core::fmt::{self, Display, Debug, Formatter};

use crate::caller;
use super::*;

/// The event error type of Leimu.
///
/// A wrapper around [`Error`], that tracks the caller's location when it's converted from
/// an [`Error`].
///
/// [1]: From::from
#[derive(Error)]
#[display("{0}")]
pub struct EventError(
    #[source(self.0.source())]
    Error
);

impl EventError {

    /// Creates a new [`EventError`] from an error and a context.
    #[track_caller]
    pub fn new<C>(err: impl error::Error + Send + Sync + 'static, ctx: C) -> Self
        where C: Display + Send + Sync + 'static
    {
        Self(build_error::new(err, ctx, Some(caller!())))
    }

    /// Creates a new [`EventError`] with just a context.
    #[track_caller]
    pub fn just_context<C>(ctx: C) -> Self
        where C: Display + Send + Sync + 'static,
    {
        Self(build_error::just_context(ctx, Some(caller!())))
    }
}

impl From<Error> for EventError {

    #[track_caller]
    fn from(value: Error) -> Self {
        Self(value.with_location(caller!()))
    }
}

impl Debug for EventError {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Error as Debug>::fmt(&self.0, f)
    }
}

impl Tracked for EventError {

    fn location(&self) -> Option<Location> {
        self.0.location()
    }
}

use core::{
    error,
    result,
    fmt::Display,
};

use super::{Tracked, build_error, Result, Location};

/// A trait for adding context to a [`Result`][1], if it contains an [`Err`].
///
/// [1]: core::result::Result
pub trait Context<T, E>
    where
        E: error::Error + Send + Sync + 'static,
{
    /// Adds context to an error.
    fn context<C>(self, ctx: C) -> Result<T>
        where C: Display + Send + Sync + 'static;

    /// Adds context to an error with a closure that only gets called on an [`Err`] value.
    fn context_with<C>(self, f: impl FnOnce() -> C) -> Result<T>
        where C: Display + Send + Sync + 'static;

    /// Add context from an error with possibly tracked location.
    fn context_from_tracked<C>(self, f: impl FnOnce(Option<Location>) -> C) -> Result<T>
        where
            C: Display + Send + Sync + 'static,
            E: Tracked;
}

impl<T, E: error::Error + Send + Sync + 'static> Context<T, E> for result::Result<T, E> {
    
    fn context<C>(self, ctx: C) -> Result<T>
        where C: Display + Send + Sync + 'static,
    {
        self.map_err(|err| {
            build_error::new(err, ctx, None)
        })
    }

    fn context_with<C>(self, f: impl FnOnce() -> C) -> Result<T>
        where C: Display + Send + Sync + 'static,
    {
        self.map_err(|err| {
            build_error::new(err, f(), None)
        })
    }

    fn context_from_tracked<C>(self, f: impl FnOnce(Option<Location>) -> C) -> Result<T>
        where
            C: Display + Send + Sync + 'static,
            E: Tracked, 
    {
        self.map_err(|err| {
            let loc = err.location();
            build_error::new(err, f(loc), None)
        })
    }
}

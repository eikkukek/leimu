use crate::Location;

pub trait ImplTry {}

/// Trait for types that track [`Location`]s.
///
/// # Example
/// ```rust
/// use leimu_error::Context;
/// use leimu_error::Tracked;
///
/// fallible().context_from_tracked(|orig| {
///     format!("error at {}", orig.or_this())
/// })?;
/// ```
pub trait Tracked {
    
    fn location(&self) -> Option<Location>;

    #[track_caller]
    #[inline(always)]
    fn location_or_this(&self) -> Location {
        self.location()
            .unwrap_or_else(|| caller!())
    }

    #[track_caller]
    #[inline(always)]
    fn or_this(&self) -> Location
        where Self: ImplTry
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

impl ImplTry for Option<Location> {}

impl Tracked for Option<Location> {

    #[inline(always)]
    fn location(&self) -> Option<Location> {
        *self
    }

    #[track_caller]
    #[inline(always)]
    fn or_this(&self) -> Location
        where Self: ImplTry
    {
        self.unwrap_or_else(|| caller!())
    }
}

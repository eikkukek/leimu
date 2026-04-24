/// An auto-trait for types that implement [`Extend`].
///
/// Works like [`Extend`], but the iterator yields [`Result`]s.
///
/// The extension is cut short if an item is [`Err`].
///
/// # Example
/// ``` rust
/// use nox_mem::iter::TryExtend;
///
/// let mut vec = vec![];
/// let iter = [Ok(1), Ok(2), Ok(3), Err(4), Ok(5)];
/// assert!(matches!(vec.try_extend(iter), Err(4)));
/// assert_eq!(vec, [1, 2, 3]);
/// ```
pub trait TryExtend<A>: Extend<A> {

    /// Tries to extend a collection with contents of an iterator.
    ///
    /// The [`trait-level`][1] docs contain more information.
    ///
    /// [1]: TryExtend
    fn try_extend<E, I>(&mut self, iter: I) -> Result<(), E>
        where I: IntoIterator<Item = Result<A, E>>;
}

impl<A, T> TryExtend<A> for T
    where T: Extend<A>
{

    fn try_extend<E, I>(&mut self, iter: I) -> Result<(), E>
        where I: IntoIterator<Item = Result<A, E>>
    {
        let mut err = None;
        self.extend(
            iter.into_iter()
                .map(|item| {
                    match item {
                        Ok(x) => Some(x),
                        Err(e) => {
                            err = Some(e);
                            None
                        }
                    }
                }).take_while(|item| item.is_some())
                .map(|item| unsafe { item.unwrap_unchecked() })
        );
        if let Some(err) = err {
            Err(err)
        } else { Ok(()) }
    }
}

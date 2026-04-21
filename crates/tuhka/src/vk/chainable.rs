use super::*;

/// A trait for structures, which *can* be a part of `p_next` chains.
///
/// # Safety
/// A structure implementing this trait *must* adhere to the memory layout of [`BaseOutStructure`].
pub unsafe trait Chainable {

    /// Casts self to [`BaseOutStructure`].
    fn base_in(&self) -> &BaseInStructure<'_>;

    fn base_out(&mut self) -> &mut BaseOutStructure<'_> {
        unsafe {
            &mut *<*const BaseInStructure>::cast::<BaseOutStructure>(
                self.base_in()
            ).cast_mut()
        }
    }
}

/// Creates a [`BaseOutStructure`] iterator over a `p_next` chain.
pub fn chain_out_iter<'a, T>(
    first: &'a mut T,
) -> impl Iterator<Item = &'a mut BaseOutStructure<'a>>
    where T: ?Sized + Chainable
{
    let out: *mut BaseOutStructure<'a> = first.base_out();
    (0..).scan(out, |out, _| unsafe {
        let this = *out;
        if out.is_null() { return None };
        let next = (*this).p_next;
        *out = next;
        Some(&mut *this)
    })
}

/// Creates a [`BaseInStructure`] iterator over a `p_next` chain.
pub fn chain_in_iter<'a, T>(
    first: &'a T
) ->  impl Iterator<Item = &'a BaseInStructure<'a>>
    where T: ?Sized + Chainable
{
    let _in: *const BaseInStructure<'a> = first.base_in();
    (0..).scan(_in, |_in, _| unsafe {
        let this = *_in;
        if _in.is_null() { return None }
        let next = (*this).p_next;
        *_in = next;
        Some(&*this)
    })
}

use core::{
    any::Any,
    ptr::NonNull,
    mem,
    marker::PhantomData,
};

use leimu_mem::conditional::*;

/// A trait for structures that extend other structures.
///
/// # Safety
/// If you are implementing this trait, you should only implement the `p_next` and `p_next_mut`
/// functions. Both of these functions need to return [`None`] or valid pointers to dyn
/// [`ExtendsStructure`].
pub unsafe trait ExtendsStructure: Any
{

    /// Gets underlying raw p_next pointer.
    fn p_next(&self) -> Option<NonNull<dyn ExtendsStructure>>;

    /// Gets a mutable reference to the underlying raw p_next pointer.
    fn p_next_mut(&mut self) -> &mut Option<NonNull<dyn ExtendsStructure>>;

    fn clear_p_next(&mut self) {
        *self.p_next_mut() = None
    }

    /// Gets a reference to the next value of the chain, if [`Some`].
    fn next(&self) -> Option<&dyn ExtendsStructure> {
        unsafe {
            self.p_next().map(|next|
                next.as_ref()
            )
        }
    }

    /// Gets a mutable reference to the next value of the chain, if [`Some`]
    fn next_mut(&mut self) -> Option<&mut dyn ExtendsStructure> {
        unsafe {
            self.p_next().map(|mut next|
                next.as_mut()
            )
        }
    }
   
    /// This can cause cycles in p_next chains if used incorrectly.
    fn push_next<'b>(&'b mut self, next: &'b mut dyn ExtendsStructure) {
        unsafe {
            if let Some(head) = self.p_next() {
                let mut tail = head;
                while let Some(n) = tail.as_mut().next_mut() {
                    tail = NonNull::from_mut(n);
                }
                *tail.as_mut().p_next_mut() = Some(NonNull::from_mut(next))
            } else {
                *self.p_next_mut() = Some(NonNull::from_mut(next))
            }
        }
    }
}

/// An auto-trait for types that implement [`ExtendsStructure`].
pub trait ExtendsStructureExt: Sized + ExtendsStructure {
    
    #[inline(always)]
    fn get_ref(value: &dyn ExtendsStructure) -> Option<&Self> {
        (value as &dyn Any).is::<Self>().then(|| {
            unsafe {
                mem::transmute::<
                    &dyn ExtendsStructure,
                    (&Self, *const ())
                >(value).0
            }
        })
    }
    
    #[inline(always)]
    fn get_mut(value: &mut dyn ExtendsStructure) -> Option<&mut Self> {
        (value as &dyn Any).is::<Self>().then(|| {
            unsafe {
                mem::transmute::<
                    &mut dyn ExtendsStructure,
                    (&mut Self, *const ())
                >(value).0
            }
        })
    }
}

impl<T> ExtendsStructureExt for T
    where T: Sized + ExtendsStructure
{}

/// A trait for structures that can be extended.
///
/// # Safety
/// [`ExtendableHead::head`] must return [`None`] or a valid pointer to dyn [`ExtendsStructure`].
pub unsafe trait ExtendableHead: Sized {

    #[allow(clippy::mut_from_ref)]
    fn head(&self) -> Option<NonNull<dyn ExtendsStructure>>;
}

pub struct ExtendableIterator<'a, T, U, IsMut: Conditional>
    where
        T: 'a,
        U: Sized + ExtendsStructure,
{
    p_head: Option<NonNull<dyn ExtendsStructure>>,
    _marker: PhantomData<&'a (T, U, IsMut)>,
}

impl<'a, T, U> Iterator for ExtendableIterator<'a, T, U, False>
    where U: Sized + ExtendsStructure
{
    type Item = &'a U;

    fn next(&mut self) -> Option<Self::Item> {
        let this = unsafe {
            self.p_head?.as_ref()
        };
        self.p_head = this.p_next();
        let Some(next) = U::get_ref(this) else {
            return self.next()
        };
        Some(next)
    }
}

impl<'a, T, U> Iterator for ExtendableIterator<'a, T, U, True>
    where U: Sized + ExtendsStructure
{
    type Item = &'a mut U;

    fn next(&mut self) -> Option<Self::Item> {
        let this = unsafe {
            self.p_head?.as_mut()
        };
        self.p_head = this.p_next();
        let Some(next) = U::get_mut(this) else {
            return self.next()
        };
        Some(next)
    }
}

/// An auto trait for types that implement [`ExtendableHead`].
pub trait Extendable: ExtendableHead {

    #[inline(always)]
    fn next_iter<T>(
        &self,
    ) -> ExtendableIterator<'_, Self, T, False>
        where T: Sized + ExtendsStructure
    {
        ExtendableIterator {
            p_head: self
                .head(),
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    fn next_iter_mut<T: Sized + ExtendsStructure>(
        &mut self
    ) -> ExtendableIterator<'_, Self, T, True> {
        ExtendableIterator {
            p_head: self
                .head(),
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    fn clear_chain(&mut self) {
        (0..).scan(self.head(), |state, _| {
            let next = unsafe {
                (*state)?.as_mut()
            };
            *state = next.p_next();
            next.clear_p_next();
            Some(())
        }).count();
    }
}

impl<T: ExtendableHead> Extendable for T {}

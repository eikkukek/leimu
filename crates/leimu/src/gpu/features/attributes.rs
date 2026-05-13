use core::{
    any::Any,
    fmt::{self, Display, Debug},
    hash::{self, Hash},
    borrow::Borrow,
    marker::PhantomData,
};

use crate::gpu::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AttributeName<T>
    where T: Any + Send + Sync
{
    name: ConstName,
    _marker: PhantomData<T>,
}

impl<T> AttributeName<T> 
    where T: Any + Send + Sync
{

    #[inline]
    pub const fn new(name: &'static str) -> Self {
        Self {
            name: ConstName::new(name),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub const fn name(&self) -> &ConstName {
        &self.name
    }
}

impl<T> Debug for AttributeName<T>
    where T: Any + Send + Sync
{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.name, f)
    }
}

impl<T> Display for AttributeName<T>
    where T: Any + Send + Sync
{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.name, f)
    }
}

pub struct Attribute {
    name: ConstName,
    inner: Box<dyn Any + Send + Sync>,
}

impl Attribute {

    #[inline]
    pub fn new<T>(
        name: AttributeName<T>,
        value: T,
    ) -> Self
        where T: Any + Send + Sync
    {
        Self {
            name: *name.name(),
            inner: Box::new(value),
        }
    }

    #[inline]
    pub fn get<T: Any>(&self) -> Option<&T> {
        let value = self.inner.as_ref();
        if value.is::<T>() {
            let ptr: *const dyn Any = value;
            Some(unsafe {
                &*ptr.cast()
            })
        } else {
            None
        }
    }
}

impl PartialEq for Attribute {
    
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Attribute {}

impl Hash for Attribute {

    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Borrow<ConstName> for Attribute {
    
    #[inline]
    fn borrow(&self) -> &ConstName {
        &self.name
    }
}

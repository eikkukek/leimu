use core::{
    ops::{Deref, DerefMut},
    any::Any,
};

use tuhka::vk;

pub trait ExtendsDeviceCreateInfoExt: Any {

    fn s_type(&self) -> vk::StructureType;
    fn as_mut(&mut self) -> &mut dyn vk::ExtendsDeviceCreateInfo;
    fn to_obj(&self) -> ExtendsDeviceCreateInfoObj;
}

impl<T> ExtendsDeviceCreateInfoExt for T
    where T:
        vk::ExtendsDeviceCreateInfo +
        Clone + Copy + 'static
{
    fn s_type(&self) -> vk::StructureType {
        self.base_in()
            .s_type
    }

    fn as_mut(&mut self) -> &mut dyn vk::ExtendsDeviceCreateInfo {
        self
    }

    fn to_obj(&self) -> ExtendsDeviceCreateInfoObj {
        ExtendsDeviceCreateInfoObj::new(*self)
    }
}

pub struct ExtendsDeviceCreateInfoObj(Box<dyn ExtendsDeviceCreateInfoExt>);

impl ExtendsDeviceCreateInfoObj {

    #[inline]
    pub fn new<T>(x: T) -> Self
        where T: ExtendsDeviceCreateInfoExt + 'static
    {
        Self(Box::new(x))
    }

    #[inline]
    pub fn as_structure<T>(&mut self) -> Option<&mut T>
        where T: ExtendsDeviceCreateInfoExt
    {
        let s: &mut dyn Any = &mut *self.0;
        s.is::<T>().then(|| unsafe {
            &mut *<*mut dyn Any>::cast::<T>(s)
        })
    }
}

impl Deref for ExtendsDeviceCreateInfoObj {

    type Target = dyn ExtendsDeviceCreateInfoExt;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl DerefMut for ExtendsDeviceCreateInfoObj {

    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl Clone for ExtendsDeviceCreateInfoObj {

    #[inline]
    fn clone(&self) -> Self {
        self.to_obj()
    }
}

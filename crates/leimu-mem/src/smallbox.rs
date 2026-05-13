/// Creates a "SmallBox" for a given trait object.
///
/// Only allocates if the size of the data exceeds `N_BUF`.
///
/// # Example
/// ``` rust
/// use std::borrow::Borrow;
/// leimu_mem::smallbox!(
///     /// Supports doc comments
///     pub struct MySmallBox: Borrow<str>
/// );
/// // No extra allocation
/// let hello = MySmallBox::<8>::new("hello");
/// // Heap allocation since size of String > 8
/// let world = MySmallBox::<8>::new("world".to_string());
/// assert_eq!(
///     format!("{}, {}", (*hello).borrow(), (*world).borrow()),
///     "hello, world"
/// );
#[macro_export]
macro_rules! smallbox {
    (
        $(#[doc = $doc:literal])*
        $vis:vis struct $name:ident:
            $first_bound:ident $([($($fn_args:ty),*) $(-> $fn_ret:ty)?])? $(<$first_gen:ty>)? $(+ $bounds:ident)*
    ) => {

        $(#[doc = $doc])*
        $vis struct $name<const N_BUF: usize> {
            data: [u8; N_BUF],
            ptr: (*const (), *const ()),
            _marker: core::marker::PhantomData<dyn
                $first_bound $(($($fn_args),*) $(-> $fn_ret)?)? $(<$first_gen>)? $(+ $bounds )*
            >,
        }

        impl<const N_BUF: usize> $name<N_BUF> {

            #[allow(unused_parens)]
            pub fn new<T>(x: T) -> Self
                where T: $first_bound $(($($fn_args),*) $(-> $fn_ret)?)? $(<$first_gen>)? $(+ $bounds)* + 'static
            {
                if size_of_val(&x) <= N_BUF {
                    let mut data = [0u8; N_BUF];
                    let ptr = data
                        .as_mut_ptr()
                        .cast::<T>();
                    unsafe {
                        ptr.write(x);
                    }
                    let mut ptr = unsafe {
                        core::mem::transmute::<
                            *mut (dyn $first_bound $(($($fn_args),*) $(-> $fn_ret)?)? $(<$first_gen>)? $(+ $bounds)*),
                            (*const (), *const ())>
                        (
                            ptr
                        )
                    };
                    ptr.0 = core::ptr::null();
                    Self {
                        data,
                        ptr,
                        _marker: core::marker::PhantomData,
                    }
                } else {
                    let ptr = unsafe {
                        std::alloc::alloc(std::alloc::Layout::new::<T>())
                        .cast::<T>()
                    };
                    unsafe {
                        ptr.write(x);
                    }
                    let ptr = unsafe {
                        core::mem::transmute::<
                            *mut (dyn $first_bound $(($($fn_args),*) $(-> $fn_ret)?)? $(<$first_gen>)? $(+ $bounds)*),
                            (*const (), *const ())>(
                            ptr
                        )
                    };
                    Self {
                        data: [0u8; N_BUF],
                        ptr,
                        _marker: core::marker::PhantomData,
                    }
                }
            }
        }

        impl<const N_BUF: usize> core::ops::Deref for $name<N_BUF> {

            type Target = dyn $first_bound $(($($fn_args),*) $(-> $fn_ret)?)? $(<$first_gen>)? $(+ $bounds)*;

            #[allow(unused_parens)]
            fn deref(&self) -> &Self::Target {
                let mut ptr = self.ptr;
                if ptr.0.is_null() {
                    ptr.0 = self.data
                        .as_ptr()
                        .cast();
                    unsafe {
                        core::mem::transmute::<
                            (*const (), *const ()),
                            &(dyn $first_bound $(($($fn_args),*) $(-> $fn_ret)?)? $(<$first_gen>)? $(+ $bounds)*)
                        >(
                            ptr
                        )
                    }
                } else {
                    unsafe {
                        core::mem::transmute::<
                            (*const (), *const()),
                            &(dyn $first_bound $(($($fn_args),*) $(-> $fn_ret)?)? $(<$first_gen>)? $(+ $bounds)*)
                        >(
                            ptr
                        )
                    }
                }
            }
        }

        impl<const N_BUF: usize> core::ops::DerefMut for $name<N_BUF> {

            #[allow(unused_parens)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                let mut ptr = self.ptr;
                if ptr.0.is_null() {
                    ptr.0 = self.data
                        .as_ptr()
                        .cast();
                    unsafe {
                        core::mem::transmute::<
                            (*const (), *const ()),
                            &mut (dyn $first_bound $(($($fn_args),*) $(-> $fn_ret)?)? $(<$first_gen>)? $(+ $bounds)*)
                        >(
                            ptr
                        )
                    }
                } else {
                    unsafe {
                        core::mem::transmute::<
                            (*const (), *const()),
                            &mut (dyn $first_bound $(($($fn_args),*) $(-> $fn_ret)?)? $(<$first_gen>)? $(+ $bounds)*)
                        >(
                            ptr
                        )
                    }
                }
            }
        }

        impl<const N_BUF: usize> Drop for $name<N_BUF> {

            #[allow(unused_parens)]
            fn drop(&mut self) {
                let mut ptr = self.ptr; 
                if ptr.0.is_null() {
                    ptr.0 = self.data
                        .as_ptr()
                        .cast();
                    unsafe {
                        let ptr = core::mem::transmute::<
                            (*const (), *const ()),
                            *mut (dyn $first_bound $(($($fn_args),*) $(-> $fn_ret)?)? $(<$first_gen>)? $(+ $bounds)*)
                        >(ptr);
                        ptr.drop_in_place();
                    }
               } else {
                    unsafe {
                        let ptr = core::mem::transmute::<
                            (*const (), *const ()),
                            *mut (dyn $first_bound $(($($fn_args),*) $(-> $fn_ret)?)? $(<$first_gen>)? $(+ $bounds)*)
                        >(ptr);
                        let (size, align) = {
                            let x = ptr.as_ref().unwrap_unchecked();
                            (size_of_val(x), align_of_val(x))
                        };
                        ptr.drop_in_place();
                        std::alloc::dealloc(ptr.cast(), std::alloc::Layout::from_size_align(
                            size, align,
                        ).unwrap());
                    }
               }
            }
        }
    };
}

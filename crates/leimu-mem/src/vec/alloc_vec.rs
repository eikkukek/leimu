use core::{
    marker::PhantomData,
    slice::{self, Iter, IterMut},
    ops::{Deref, DerefMut},
    hash::{Hash, Hasher},
    fmt::{Debug, Display},
    mem::ManuallyDrop,
    borrow::{Borrow, BorrowMut},
    result,
};

use crate::{
    reserve::*,
    int::{FromUsize, IntoUsize},
    alloc::{LocalAlloc, LocalAllocExt, LocalAllocWrap},
    conditional::{Conditional, False},
};

use super::{
    Pointer,
    non_null::NonNullVecBase,
};

pub struct DynPolicy;

unsafe impl ReservePolicy for DynPolicy {

    #[inline]
    fn can_grow() -> bool {
        true
    }

    #[inline]
    fn grow(current: usize, required: usize) -> Result<usize, ReserveError<()>> {
        let power_of_2 = required.next_power_of_two().max(2);
        if power_of_2 < current { Ok(current) }
        else { Ok(power_of_2) }
    }

    #[inline]
    fn grow_infallible(current: usize, required: usize) -> usize {
        let power_of_2 = required.next_power_of_two().max(2);
        if power_of_2 < current { current }
        else { power_of_2 }
    }
}

pub struct FixedPolicy;

unsafe impl ReservePolicy for FixedPolicy {

    #[inline]
    fn can_grow() -> bool {
        false
    }

    #[inline]
    fn grow(current: usize, requested: usize) -> Result<usize, ReserveError<()>> {
        if requested <= current {
            Ok(current)
        } else {
            Err(ReserveError::max_capacity_exceeded(current, requested, ()))
        }
    }

    #[inline]
    fn grow_infallible(current: usize, required: usize) -> usize {
        if required > current.into_usize() { panic!("maximum capacity of {current} exceeded") }
        current
    }
}

pub struct DynPolicy32;

pub struct FixedPolicy32;

unsafe impl ReservePolicy<u32> for DynPolicy32 {

    fn can_grow() -> bool {
        true
    }

    #[inline]
    fn grow(current: u32, required: usize) -> core::result::Result<u32, ReserveError<()>> {
        let power_of_2 = required.next_power_of_two().max(2);
        if power_of_2 > u32::MAX as usize {
            Err(ReserveError::max_capacity_exceeded(u32::MAX as usize, power_of_2, ()))
        } else if power_of_2 <= current as usize {
            Ok(current)
        } else { Ok(power_of_2.max(2) as u32) }
    }

    #[inline]
    fn grow_infallible(current: u32, required: usize) -> u32 {
        let power_of_2 = required.next_power_of_two().max(2);
        if power_of_2 > u32::MAX as usize || power_of_2 <= current as usize {
            current
        } else {
            power_of_2.max(2) as u32
        }
    }
}

unsafe impl ReservePolicy<u32> for FixedPolicy32 {

    #[inline]
    fn can_grow() -> bool {
        false
    }

    #[inline]
    fn grow(current: u32, requested: usize) -> Result<u32, ReserveError<()>> {
        if requested <= current as usize {
            Ok(current)
        } else {
            Err(ReserveError::max_capacity_exceeded(current, requested, ()))
        }
    }

    #[inline]
    fn grow_infallible(current: u32, required: usize) -> u32 {
        if required > current.into_usize() { panic!("maximum capacity of {current} exceeded") }
        current
    }
}

pub struct AllocVec<T, Alloc, ReservePol, IsStd, SizeType = usize>
    where
        T: Sized,
        Alloc: LocalAlloc,
        ReservePol: ReservePolicy<SizeType>,
        IsStd: Conditional,
        SizeType: IntoUsize + FromUsize,
{
    data: Pointer<T, SizeType>,
    capacity: SizeType,
    len: SizeType,
    alloc: Alloc,
    _markers: PhantomData<(ReservePol, IsStd)>,
}

/// A vector type, which uses a [`local, owned allocator`][1] for its allocations.
///
/// It is recommended to use one of its derivatives ([`DynVec`], [`FixedVec`], [`DynVec32`] and
/// [`FixedVec32`]) when using it with a local allocator.
///
/// # Examples
/// ``` rust
/// use leimu_mem::arena::Arena;
/// use leimu_mem::vec::FixedVec;
///
/// let arena = Arena::new(64).unwrap();
/// let mut vec = FixedVec::with_capacity(5, &arena).unwrap();
/// vec.push(1);
/// vec.append(&[2, 3]);
/// vec.extend(4..6);
/// assert_eq!(vec, [1, 2, 3, 4, 5]);
/// ```
///
/// [1]: LocalAlloc
pub type AllocVecBase<T, SizeType, Alloc, ReservePol> =
    AllocVec<T, Alloc, ReservePol, False, SizeType>;

/// A vector using [`LocalAlloc`], which can reallocate.
///
/// See [`AllocVecBase`] for full description and examples.
pub type DynVec<'a, T, Alloc> = AllocVecBase<T, usize, LocalAllocWrap<Alloc, &'a Alloc>, DynPolicy>;
/// A vector using [`LocalAlloc`], which can reallocate.
///
/// Stores its capacity and length as [`u32`].
///
/// See [`AllocVecBase`] for full description and examples.
pub type DynVec32<'a, T, Alloc> = AllocVecBase<T, u32, LocalAllocWrap<Alloc, &'a Alloc>, DynPolicy32>;

/// A vector using [`LocalAlloc`], which won't reallocate.
///
/// See [`AllocVecBase`] for full description and examples.
pub type FixedVec<'a, T, Alloc> = AllocVecBase<T, usize, LocalAllocWrap<Alloc, &'a Alloc>, FixedPolicy>;
/// A vector using [`LocalAlloc`], which won't reallocate.
///
/// Stores its capacity and length as [`u32`].
///
/// See [`AllocVecBase`] for full description and examples.
pub type FixedVec32<'a, T, Alloc> = AllocVecBase<T, u32, LocalAllocWrap<Alloc, &'a Alloc>, FixedPolicy32>;

impl<T, Alloc, Wrap, ReservePol, SizeType> AllocVec<T, LocalAllocWrap<Alloc, Wrap>, ReservePol, False, SizeType>
    where
        T: Sized,
        Alloc: LocalAlloc + ?Sized,
        Wrap: Deref<Target = Alloc>,
        ReservePol: ReservePolicy<SizeType>,
        SizeType: IntoUsize + FromUsize
{

    pub fn new(alloc: Wrap) -> Self {
        Self {
            data: Pointer::dangling(),
            capacity: SizeType::ZERO,
            len: SizeType::ZERO,
            alloc: LocalAllocWrap::new(alloc),
            _markers: PhantomData,
        }
    }

    pub fn with_capacity(
        capacity: SizeType,
        alloc: Wrap,
    ) -> Result<Self, Alloc::Error> {
        if capacity == SizeType::ZERO {
            return Ok(Self::new(alloc))
        }
        let capacity = ReservePol::grow_infallible(capacity, capacity.into_usize());
        let data = unsafe { alloc
            .alloc_uninit(capacity.into_usize())?
        }.into();
        Ok(Self {
            data,
            capacity,
            len: SizeType::ZERO,
            alloc: LocalAllocWrap::new(alloc),
            _markers: PhantomData,
        })
    }

    pub fn with_len(
        len: SizeType,
        value: T,
        alloc: Wrap,
    ) -> Result<Self, Alloc::Error>
        where
            T: Clone
    {
        if len == SizeType::ZERO {
            return Ok(Self::new(alloc))
        }
        let capacity = ReservePol::grow_infallible(len, len.into_usize());
        let data: Pointer<T, SizeType> = unsafe { alloc
            .alloc_uninit(capacity.into_usize())?
        }.into();
        for i in 0..len.into_usize() {
            unsafe { data.add(i).write(value.clone()) };
        }
        Ok(Self {
            data,
            capacity,
            len,
            alloc: LocalAllocWrap::new(alloc),
            _markers: PhantomData,
        })
    }

    pub fn with_len_with<F>(
        len: SizeType,
        mut f: F,
        alloc: Wrap,
    ) -> Result<Self, Alloc::Error>
        where F: FnMut(SizeType) -> T
    {
        if len == SizeType::ZERO {
            return Ok(Self::new(alloc))
        }
        let capacity = ReservePol::grow_infallible(len, len.into_usize());
        let data: Pointer<T, SizeType> = unsafe { alloc
            .alloc_uninit(capacity.into_usize())?
            .into()
        };
        for i in 0..len.into_usize() {
            unsafe { data.add(i).write(f(SizeType::from_usize(i))) };
        }
        Ok(Self {
            data,
            capacity,
            len,
            alloc: LocalAllocWrap::new(alloc),
            _markers: PhantomData,
        })
    }

    pub fn flattened<U>(
        slices: &[U],
        alloc: Wrap,
    ) -> Result<Self, ReserveError<()>>
        where
            U: AsRef<[T]>,
            T: Clone,
    {
        let capacity: usize = slices
            .iter()
            .map(|s| s.as_ref().len())
            .sum();
        let capacity = ReservePol::grow(
            SizeType::from_usize(capacity),
            capacity,
        )?;
        let data: Pointer<T, SizeType> = unsafe { alloc
            .alloc_uninit(capacity.into_usize())
            .map_err(|err| ReserveError::alloc_error(err, ()))?
            .into()
        };
        let mut res = Self {
            data,
            capacity,
            len: SizeType::ZERO,
            alloc: LocalAllocWrap::new(alloc),
            _markers: PhantomData,
        };
        for slice in slices {
            res.append(slice.as_ref());
        }
        Ok(res)
    }
}

impl<T, Alloc, ReservePol, IsStd, SizeType> AllocVec<T, Alloc, ReservePol, IsStd, SizeType>
    where
        Alloc: LocalAlloc,
        ReservePol: ReservePolicy<SizeType>,
        IsStd: Conditional,
        SizeType: IntoUsize + FromUsize,
{

    /// Creates a new [`AllocVec`] from raw parts.
    /// 
    /// This function should only be used when implementing a custom [`AllocVec`] type.
    ///
    /// # Safety
    /// There are a multitude of reasons why this is unsafe. For example, the pointer needs to be valid up to
    /// capacity, the pointer needs to be a valid pointer for the given allocator and length needs to be
    /// less than capacity.
    pub unsafe fn from_raw_parts(
        data: Pointer<T, SizeType>,
        len: SizeType,
        capacity: SizeType,
        alloc: Alloc,
    ) -> Self {
        Self {
            data,
            len,
            capacity,
            alloc,
            _markers: PhantomData,
        }
    }
    
    /// Changes the [`LocalAlloc`] of the [`AllocVec`].
    ///
    /// This function should only be used when implementing a custom [`AllocVec`] type.
    ///
    /// # Safety
    /// The inner pointer of the source vector must be valid for the new allocator.
    pub unsafe fn with_alloc<A: LocalAlloc>(self, alloc: A) -> AllocVecBase<T, SizeType, A, ReservePol> {
        let vec = ManuallyDrop::new(self);
        AllocVec {
            data: vec.data,
            capacity: vec.capacity,
            len: vec.len,
            alloc,
            _markers: PhantomData
        }
    }

    pub unsafe fn into_non_null(self) -> NonNullVecBase<'static, T, SizeType> {
        let vec = ManuallyDrop::new(self);
        unsafe {
            NonNullVecBase::new(*vec.data, vec.capacity)
            .with_len(vec.len)
        }
    }
    
    /// Sets the capacity of [`AllocVec`].
    ///
    /// This function should only be used when implementing a custom [`AllocVec`] type.
    ///
    /// # Safety
    /// The inner pointer needs to be valid up to the `capacity` and `capacity` needs to be greater
    /// than or equal to the vector's length (which can be set via [`AllocVec::set_len`]).
    pub unsafe fn set_capacity(&mut self, capacity: SizeType) {
        self.capacity = capacity;
    }

    /// Consumes self and returns the inner pointer.
    pub fn into_inner(self) -> Pointer<T, SizeType> {
        let vec = ManuallyDrop::new(self);
        vec.data
    } 
}

pub struct IterAllocVec<T, SizeType: IntoUsize + FromUsize, Alloc: LocalAlloc> {
    data: Pointer<T, SizeType>,
    capacity: SizeType,
    start: SizeType,
    end: SizeType,
    alloc: Alloc,
}

impl<T, SizeType, Alloc> Iterator for IterAllocVec<T, SizeType, Alloc>
    where 
        SizeType: IntoUsize + FromUsize,
        Alloc: LocalAlloc,
{

    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start != self.end {
            let t = unsafe {
                self.data.add(self.start.into_usize()).read()
            };
            self.start += SizeType::ONE;
            Some(t)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.end - self.start).into_usize();
        (size, Some(size))
    }
}

impl<T, SizeType, Alloc> DoubleEndedIterator for IterAllocVec<T, SizeType, Alloc>
    where
        SizeType: IntoUsize + FromUsize,
        Alloc: LocalAlloc,
{

    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start != self.end {
            self.end -= SizeType::ONE;
            let t = unsafe {
                self.data.add(self.end.into_usize()).read()
            };
            Some(t)
        } else {
            None
        }
    }
}

impl<T, SizeType, Alloc> ExactSizeIterator for IterAllocVec<T, SizeType, Alloc>
    where
        SizeType: IntoUsize + FromUsize,
        Alloc: LocalAlloc,
{}

impl<T, SizeType, Alloc> Drop for IterAllocVec<T, SizeType, Alloc>
    where
        SizeType: IntoUsize + FromUsize,
        Alloc: LocalAlloc,
{

    fn drop(&mut self) {
        unsafe {
            if self.capacity == SizeType::ZERO { return }
            self.data
                .add(self.start.into_usize())
                .drop_in_place(self.end - self.start);
            self.alloc
                .free_uninit(*self.data, self.capacity.into_usize());
        }
    }
}

impl<T, Alloc, ReservePol, IsStd, SizeType> AllocVec<T, Alloc, ReservePol, IsStd, SizeType>
    where
        Alloc: LocalAlloc,
        ReservePol: ReservePolicy<SizeType>,
        IsStd: Conditional,
        SizeType: IntoUsize + FromUsize,
{

    /// Returns the length of the vector.
    #[inline]
    pub fn len(&self) -> SizeType {
        self.len
    }

    /// Returns whether the vector is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == SizeType::ZERO
    }

    /// Returns the allocated capacity of the vector.
    #[inline]
    pub fn capacity(&self) -> SizeType {
        self.capacity
    }

    /// Gets the pointer to the vector's data.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    /// Gets a mutable pointer to the vector's data.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_ptr()
    }

    /// Returns a slice over the contents of the vector.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data.as_ptr(), self.len.into_usize()) }
    }

    /// Returns a mutable slice over the contents of the vector.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.data.as_ptr(), self.len.into_usize()) }
    }

    /// A low-level operation that forces the length of the vector to `new_len`.
    ///
    /// Panics if `new_len` exceeds `capacity` on debug builds.
    ///
    /// # Safety
    /// If `new_len` > current length, elements with elements at indices greater than or equal to
    /// the current length are left uninitialized and if `new_len` < current length, elements at
    /// indices less than or equal to the current length are not dropped when the vector is
    /// [`cleared`][1] or [`dropped`][Drop].
    ///
    /// [1]: Self::clear
    #[inline]
    pub unsafe fn set_len(&mut self, new_len: SizeType) {
        #[cfg(debug_assertions)]
        if new_len > self.capacity { panic!("len was larger than capacity") }
        self.len = new_len;
    }

    /// Reserves space for the vector.
    ///
    /// This may speculatively reserve more space than `capacity` to avoid frequent reallocations.
    ///
    /// Use [`reserve_exact`][1] to allocate an exact capacity.
    ///
    /// This may panic if the vector has a fixed capacity or if an allocation fails.
    ///
    /// Use [`try_reserve`][2] if allocation failure can be handled.
    ///
    /// [1]: Self::reserve_exact
    /// [2]: Self::try_reserve
    pub fn reserve(&mut self, capacity: SizeType)
    {
        let capacity = ReservePol::grow_infallible(self.capacity, capacity.into_usize());
        self.reserve_exact(capacity);
    }

    /// Reserves space for the vector exactly up to `capacity`.
    ///
    /// This may panic if the vector has a fixed capacity or if an allocation fails.
    ///
    /// Use [`try_reserve_exact`][2] if allocation failure can be handled.
    ///
    /// [1]: Self::reserve_exact
    /// [2]: Self::try_reserve
    pub fn reserve_exact(&mut self, capacity: SizeType) {
        if capacity <= self.capacity { return }
        if !ReservePol::can_grow() {
            panic!(
                "maximum capacity of {} exceeded, requested capacity was {}",
                self.capacity, capacity,
            )
        }
        let tmp = unsafe {
            self.alloc.alloc_uninit(capacity.into_usize())
        }.unwrap().into();
        debug_assert!(self.len <= self.capacity);
        unsafe {
            self.data.move_elements(tmp, self.len);
        }
        if self.capacity != SizeType::ZERO {
            unsafe { self.alloc.free_uninit(*self.data, self.capacity.into_usize()); }
        }
        self.data = tmp;
        self.capacity = capacity
    }
    
    /// Tries to reserve space for the vector, returning an error if the vector has a fixed capacity
    /// or if an allocation fails.
    ///
    /// This may speculatively reserve more space than `capacity` to avoid frequent reallocations.
    ///
    /// Use [`try_reserve_exact`][1] to allocate an exact capacity.
    ///
    /// [1]: Self::try_reserve_exact
    pub fn try_reserve(&mut self, capacity: SizeType) -> Result<(), ReserveError<()>> {
        let capacity = ReservePol::grow(
            self.capacity, capacity.into_usize()
        )?;
        self.try_reserve_exact(capacity)
    }

    /// Tries to reserve space for the vector exactly up to `capacity`, returning an error if the
    /// vector has a fixed capacity or if an allocation fails.
    pub fn try_reserve_exact(&mut self, capacity: SizeType) -> Result<(), ReserveError<()>> {
        if capacity <= self.capacity { return Ok(()) }
        if !ReservePol::can_grow() {
            return Err(ReserveError::max_capacity_exceeded(
                self.capacity, capacity.into_usize(), (),
            ))
        }
        let tmp = unsafe {
            self.alloc.alloc_uninit(capacity.into_usize())
        }.map_err(|err| ReserveError::alloc_error(err, ()))?.into();
        debug_assert!(self.len <= self.capacity);
        unsafe {
            self.data.move_elements(tmp, self.len);
        }
        if self.capacity != SizeType::ZERO {
            unsafe { self.alloc.free_uninit(*self.data, self.capacity.into_usize()); }
        }
        self.data = tmp;
        self.capacity = capacity;
        Ok(())
    }

    /// Resizes the vector with a clonable value.
    ///
    /// If `len` is less than the current length, this shrinks the vector.
    ///
    /// This may panic if [`reserve`][1] fails.
    ///
    /// [1]: Self::reserve
    pub fn resize(&mut self, len: SizeType, value: T)
        where
            T: Clone
    {
        if len > self.capacity {
            self.reserve(len);
        }
        if len > self.len {
            for i in self.len.into_usize()..len.into_usize() {
                unsafe { self.data.add(i).write(value.clone()) }
            }
        }
        else if len < self.len {
            unsafe {
                self.data.add(len.into_usize()).drop_in_place(self.len - len);
            }
        }
        self.len = len;
    }

    /// Resizes the vector with the given closure.
    ///
    /// If `len` is less than the current length, this shrinks the vector.
    ///
    /// This may panic if [`reserve`][1] fails.
    ///
    /// [1]: Self::reserve
    pub fn resize_with<F>(&mut self, len: SizeType, mut f: F)
        where
            F: FnMut() -> T
    {
        if len > self.capacity {
            self.reserve(len);
        }
        if len > self.len {
            for i in self.len.into_usize()..len.into_usize() {
                unsafe { self.data.add(i).write(f()) }
            }
        }
        else if len < self.len {
            unsafe {
                self.data.add(len.into_usize()).drop_in_place(self.len - len);
            }
        }
        self.len = len;
    }

    /// Tries to resize the vector with the given closure that may return an error.
    ///
    /// If `len` is less than the current length, this shrinks the vector.
    ///
    /// This may panic if [`reserve`][1] fails.
    ///
    /// [1]: Self::reserve
    pub fn try_resize_with<F, E>(
        &mut self,
        len: SizeType,
        mut f: F,
    ) -> Result<(), E>
        where
            F: FnMut() -> result::Result<T, E>,
    {
        if len > self.capacity {
            self.reserve(len)
        }
        if len > self.len {
            for i in self.len.iter(len) {
                unsafe { self.data
                    .add(i.into_usize())
                    .write(f().inspect_err(|_| {
                        self.len = i;
                    })?)
                }
            }
        }
        else if len < self.len {
            unsafe {
                self.data.add(len.into_usize()).drop_in_place(self.len - len);
            }
        }
        self.len = len;
        Ok(())
    }

    /// Appends an element to the end of the vector.
    ///
    /// This may panic if [`reserve`][1] fails.
    ///
    /// [1]: Self::reserve
    pub fn push(&mut self, value: T) {
        if self.len >= self.capacity {
            if self.capacity == SizeType::ZERO {
                self.reserve(SizeType::from_usize(2));
            }
            else {
                let capacity = ReservePol
                    ::grow(
                        self.capacity,
                        self.capacity.into_usize() * 2
                    ).unwrap();
                self.reserve_exact(capacity);
            }
        }
        unsafe { self.data.add(self.len.into_usize()).write(value) };
        self.len += SizeType::ONE;
    }

    /// Inserts an element to the specified index.
    ///
    /// Panics if `index` is greater than the length of the vector.
    ///
    /// This may panic if [`reserve`][1] fails.
    ///
    /// [1]: Self::reserve
    pub fn insert(&mut self, index: SizeType, value: T) {
        if index > self.len {
            panic!("index {} was out of bounds with len {} when inserting", index, self.len)
        }
        if self.len >= self.capacity {
            let capacity = ReservePol
                ::grow(
                    self.capacity,
                    self.capacity.into_usize() * 2
                ).unwrap();
            self.reserve_exact(capacity);
        }
        unsafe {
            self.data.insert_element(value, index, self.len);
            self.len += SizeType::ONE;
        }
    }

    /// Appends a slice to the end of the vector.
    ///
    /// If the type implements [`Copy`], consider using [`fast_append`][1]
    ///
    /// This may panic if [`reserve`][2] fails.
    ///
    /// [1]: Self::fast_append
    /// [2]: Self::reserve
    pub fn append(&mut self, slice: &[T])
        where
            T: Clone
    {
        let len = self.len.into_usize() + slice.len();
        let capacity = ReservePol
            ::grow(self.capacity, len)
            .unwrap();
        self.reserve_exact(capacity);
        unsafe {
            Pointer
                ::new(slice.as_ptr().cast_mut())
                .unwrap()
                .clone_elements(
                    self.data.add(self.len.into_usize()),
                    FromUsize::from_usize(slice.len()),
                );
        }
        self.len = SizeType::from_usize(len);
    }

    /// Appends a slice to the end of the vector by [`copying`][1] the values.
    ///
    /// This may be faster than [`append`][2] for types implementing [`Copy`].
    ///
    /// This may panic if [`reserve`][3] fails.
    ///
    /// [1]: Copy
    /// [2]: Self::append
    /// [3]: Self::reserve
    pub fn fast_append(&mut self, slice: &[T])
        where T: Copy
    {
        let len = self.len.into_usize() + slice.len();
        let capacity = ReservePol
            ::grow(self.capacity, len)
            .unwrap();
        self.reserve_exact(capacity);
        unsafe {
            Pointer
                ::new(slice.as_ptr().cast_mut())
                .unwrap()
                .fast_clone_elements(
                    self.data.add(self.len.into_usize()),
                    FromUsize::from_usize(slice.len()),
                );
        }
        self.len = SizeType::from_usize(len);
    } 

    /// Returns a reference to the last element of the vector, or [`None`] if the vector is empty.
    #[inline]
    pub fn last(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            unsafe {Some(
                self.data.add(self.len.into_usize() - 1).as_ref()
            )}
        }
    }

    /// Returns a mutable reference to the last element of the vector, or [`None`] if the vector is
    /// empty.
    #[inline]
    pub fn last_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        }
        else {
            unsafe {Some(
                self.data.add(self.len.into_usize() - 1).as_mut()
            )}
        }
    }

    /// Removes an element from the specified index.
    ///
    /// Panics if `index` is greater than or equal to the length of the vector.
    pub fn remove(&mut self, index: SizeType) -> T {
        if index >= self.len {
            panic!("index {} was out of bounds with len {} when attempting to remove an element",
                index, self.len
            );
        }
        let removed = unsafe { self.data.add(index.into_usize()).read() };
        for i in index.into_usize() ..self.len.into_usize() - 1 {
            unsafe { self.data.add(i).write(
                self.data.add(i + 1).read()
            )}
        }
        self.len -= SizeType::ONE;
        removed
    }
    
    /// Removes an element from the specified index and replaces the value of that index with the
    /// last element of the vector if `index` is not the last index.
    ///
    /// This may be faster than [`remove`][1] but this doesn't preserve the order of elements.
    ///
    /// Panics if `index` is greater than or equal to the length of the vector.
    ///
    /// [1]: Self::remove
    pub fn swap_remove(&mut self, index: SizeType) -> T {
        if index >= self.len {
            panic!("index {} was out of bounds with len {} when attempting to remove an element",
                index, self.len
            )
        }
        let removed = unsafe { self.data.add(index.into_usize()).read() };
        self.len -= SizeType::ONE;
        if index != self.len {
            unsafe { self.data.add(index.into_usize()).write(
                self.data.add(self.len.into_usize()).read()
            )}
        }
        removed
    }

    /// Removes the last element of the vector and returns it, or [`None`] if the vector is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == SizeType::ZERO { return None }
        self.len -= SizeType::ONE;
        let ptr = unsafe { self.data.add(self.len.into_usize()) };
        Some(unsafe { ptr.read() })
    } 

    /// Removes all elements from the vector preserving its allocated capacity.
    pub fn clear(&mut self) {
        debug_assert!(self.len <= self.capacity);
        if self.capacity == SizeType::ZERO { return }
        unsafe {
            self.data.drop_in_place(self.len);
        }
        self.len = SizeType::ZERO;
    }

    /// Retains all elements that satisfy the predicate, preserving the order of elements.
    ///
    /// See [`retain_unordered`][1] for an unordered version.
    ///
    /// [1]: Self::retain_unordered
    pub fn retain<F>(&mut self, mut p: F)
        where F: FnMut(&T) -> bool
    {
        for i in SizeType::ZERO.iter(self.len).rev() {
            if !p(&self[i.into_usize()]) {
                self.remove(i);
            }
        }
    }

    /// Retains all elements that satisfy the predicate that takes a mutable reference, preserving
    /// the order of elements
    ///
    /// See [`retain_unordered_mut`][1] for an unordered version.
    ///
    /// [1]: Self::retain_unordered_mut
    pub fn retain_mut<F>(&mut self, mut p: F)
        where F: FnMut(&mut T) -> bool
    {
        for i in SizeType::ZERO.iter(self.len).rev() {
            if !p(&mut self[i.into_usize()]) {
                self.remove(i);
            }
        }
    }

    /// Retains all elements that satisfy the predicate without preserving the order of elements.
    pub fn retain_unordered<F>(&mut self, mut p: F)
        where F: FnMut(&T) -> bool
    {
        for i in SizeType::ZERO.iter(self.len).rev() {
            if !p(&self[i.into_usize()]) {
                self.swap_remove(i);
            }
        }
    }

    /// Retains all elements that satisfy the predicate that takes a mutable reference without
    /// preserving the order of elements
    pub fn retain_unordered_mut<F>(&mut self, mut p: F)
        where F: FnMut(&mut T) -> bool
    {
        for i in SizeType::ZERO.iter(self.len).rev() {
            if !p(&mut self[i.into_usize()]) {
                self.swap_remove(i);
            }
        }
    }

    /// Removes consecutive repeated elements from the vector according to [`PartialEq`]
    pub fn dedup(&mut self)
        where T: PartialEq
    {
        for i in (0..self.len.into_usize().saturating_sub(1)).rev() {
            if self[i] == self[i + 1] {
                self.remove(SizeType::from_usize(i + 1));
            }
        }
    }

    /// Removes consecutive repeated elements from the vector according to a closure defined
    /// equality.
    pub fn dedup_by<F>(&mut self, mut p: F)
        where F: FnMut(&T, &T) -> bool
    {
        for i in (0..self.len.into_usize().saturating_sub(1)).rev() {
            if p(&self[i], &self[i + 1]) {
                self.remove(SizeType::from_usize(i + 1));
            }
        }
    }

    /// Removes consecutive repeated elements that resolve to a key implementing [`PartialEq`]. 
    pub fn dedup_by_key<F, K>(&mut self, mut key: F)
        where
            F: FnMut(&T) -> K,
            K: PartialEq,
    {
        for i in (0..self.len.into_usize().saturating_sub(1)).rev() {
            if key(&self[i]) == key(&self[i + 1]) {
                self.remove(SizeType::from_usize(i + 1));
            }
        }
    }

    /// Returns an iterator over the elements of the vector.
    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    /// Returns a mutable iterator over the elements of the vector.
    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }
}

crate::macros::impl_traits!{
    for AllocVec<
        T: [Sized],
        Alloc: [LocalAlloc],
        ReservePol: [ReservePolicy<SizeType>],
        IsStd: [Conditional],
        SizeType: [IntoUsize + FromUsize]
    >
    Drop =>

        fn drop(&mut self) {
            self.clear();
            if self.capacity != SizeType::ZERO {
                unsafe { self.alloc.free_uninit(
                    *self.data,
                    self.capacity.into_usize()
                )}
            }
            self.capacity = SizeType::ZERO;
            self.data = Pointer::dangling();
        }
    ,
    AsRef<[T]> =>

        #[inline]
        fn as_ref(&self) -> &[T] {
            self.as_slice()
        }
    ,
    AsMut<[T]> =>

        #[inline]
        fn as_mut(&mut self) -> &mut [T] {
            self.as_mut_slice()
        }
    ,    
    Deref =>

        type Target = [T];

        #[inline]
        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    ,
    DerefMut =>

        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.as_mut()
        }
    ,
    Borrow<[T]> =>

        #[inline]
        fn borrow(&self) -> &[T] {
            self.as_slice()
        }
    ,
    BorrowMut<[T]> =>

        #[inline]
        fn borrow_mut(&mut self) -> &mut [T] {
            self.as_mut_slice()
        }
    ,
    IntoIterator for &'vec =>

        type Item = &'vec T;
        type IntoIter = Iter<'vec, T>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    ,
    IntoIterator for mut &'vec =>

        type Item = &'vec mut T;
        type IntoIter = IterMut<'vec, T>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    ,
    IntoIterator => 

        type Item = T;
        type IntoIter = IterAllocVec<T, SizeType, Alloc>;

        fn into_iter(self) -> Self::IntoIter {
            unsafe {
                let s = ManuallyDrop::new(self);
                let alloc_ptr: *const Alloc = &s
                    .alloc;
                IterAllocVec {
                    data: s.data,
                    capacity: s.capacity,
                    start: SizeType::ZERO,
                    end: s.len,
                    alloc: alloc_ptr.read(),
                }
            }
        }
    ,
    Eq where T: Eq =>
    ,
    Hash where T: Hash =>

        fn hash<H: Hasher>(&self, state: &mut H) {
            self.as_slice().hash(state)
        }
    ,
    Debug where T: Debug =>

        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            self.as_slice().fmt(f)
        }
    ,
    Display where T: Display =>

        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            if self.len == SizeType::ZERO {
                <str as Display>::fmt("[]", f)?;
                return Ok(())
            }
            <char as Display>::fmt(&'[', f)?;
            for value in &self[0..self.len.into_usize() - 1] {
                value.fmt(f)?;
                <str as Display>::fmt(", ", f)?;
            }
            self.last().unwrap().fmt(f)?;
            <char as Display>::fmt(&']', f)
        }
    ,
}

impl<A, Alloc, ReservePol, IsStd, SizeType> Extend<A>
    for AllocVec<A, Alloc, ReservePol, IsStd, SizeType>
    where
        Alloc: LocalAlloc,
        ReservePol: ReservePolicy<SizeType>,
        IsStd: Conditional,
        SizeType: IntoUsize + FromUsize,
{

    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let iter = iter.into_iter();
        if let (_, Some(len)) = iter.size_hint() {
            self.reserve_exact(ReservePol::grow_infallible(
                self.capacity,
                self.len.into_usize() + len
            ));
        }
        for item in iter {
            self.push(item);
        }
    }
}

impl<T, A, Alloc, ReservePol, IsStd, SizeType> PartialEq<T> 
    for AllocVec<A, Alloc, ReservePol, IsStd, SizeType>
    where
        T: AsRef<[A]>,
        A: PartialEq,
        Alloc: LocalAlloc,
        ReservePol: ReservePolicy<SizeType>,
        IsStd: Conditional,
        SizeType: IntoUsize + FromUsize,
{
    fn eq(&self, other: &T) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T, Alloc, ReservePol, IsStd, SizeType> PartialEq<
        AllocVec<T, Alloc, ReservePol, IsStd, SizeType>
    > for [T]
    where
        T: PartialEq,
        Alloc: LocalAlloc,
        ReservePol: ReservePolicy<SizeType>,
        IsStd: Conditional,
        SizeType: IntoUsize + FromUsize,
{
    fn eq(&self, other: &AllocVec<T, Alloc, ReservePol, IsStd, SizeType>) -> bool {
        self == other.as_ref()
    }
}

impl<T, Alloc, ReservePol, IsStd, SizeType> PartialEq<
        AllocVec<T, Alloc, ReservePol, IsStd, SizeType>
    > for &[T]
    where
        T: PartialEq,
        Alloc: LocalAlloc,
        ReservePol: ReservePolicy<SizeType>,
        IsStd: Conditional,
        SizeType: IntoUsize + FromUsize,
{
    fn eq(&self, other: &AllocVec<T, Alloc, ReservePol, IsStd, SizeType>) -> bool {
        *self == other.as_ref()
    }
}

unsafe impl<
    Alloc: LocalAlloc + Send,
    T: Send,
    ReservePol: ReservePolicy<SizeType>,
    IsStd: Conditional,
    SizeType: IntoUsize + FromUsize,
> Send for AllocVec<T, Alloc, ReservePol, IsStd, SizeType> {}

unsafe impl<
    Alloc: LocalAlloc + Sync,
    T: Sync,
    ReservePol: ReservePolicy<SizeType>,
    IsStd: Conditional,
    SizeType: IntoUsize + FromUsize,
> Sync for AllocVec<T, Alloc, ReservePol, IsStd, SizeType> {}

#[cfg(feature = "std")]
mod std_features {

    use super::*;

    use crate::{
        alloc::StdAlloc,
        conditional::True,
    };
    
    /// A base for vectors using [`GlobalAlloc`][1].
    ///
    /// To use this, you must specify a size type ([`usize`] or [`u32`]) and a [`policy`][2] used to
    /// grow the vector.
    ///
    /// # Example
    /// ``` rust
    /// use leimu_mem::vec::StdVecBase;
    /// use leimu_mem::reserve::{ReservePolicy, ReserveError};
    ///
    /// pub struct Policy;
    ///
    /// unsafe impl ReservePolicy<u32> for Policy {
    ///
    ///     fn can_grow() -> bool {
    ///         true
    ///     }
    ///
    ///     fn grow(current: u32, required: usize) -> Result<u32, ReserveError<()>> {
    ///         if required > current as usize {
    ///             Err(ReserveError::max_capacity_exceeded(current, required, ()))
    ///         } else {
    ///             Ok(current)
    ///         }
    ///     }
    ///
    ///     fn grow_infallible(current: u32, required: usize) -> u32 {
    ///         if required > current as usize {
    ///             panic!("maximum capacity of {current} exceeded")
    ///         }
    ///         current
    ///     }
    /// }
    ///
    /// pub type FixedBuffer<T> = StdVecBase<T, u32, Policy>;
    /// ```
    ///
    /// [1]: std::alloc::alloc
    /// [2]: ReservePolicy
    pub type StdVecBase<T, SizeType, ReservePol> = AllocVec<T, StdAlloc, ReservePol, True, SizeType>;

    /// A vector that stores its capacity and length as [`u32`] instead of [`usize`] resulting in the
    /// struct taking only 16 bytes on the stack on 64-bit systems.
    ///
    /// On 64-bit systems the size of [`Vec32<T>`] is equal to the size of [`Box<[T]>`][2].
    ///
    /// This was mainly made for Vulkan usage, since Vulkan often uses u32 for counts.
    ///
    /// Maximum capacity is restricted to be equal to [`u32::MAX`].
    ///
    /// [1]: std::alloc::alloc
    /// [2]: std::boxed::Box
    pub type Vec32<T> = StdVecBase<T, u32, DynPolicy32>;

    crate::macros::impl_traits!{
        for StdVecBase<T, SizeType: [IntoUsize + FromUsize], ReservePol: [ReservePolicy<SizeType>]>
        Default =>
            #[inline]
            fn default() -> Self {
                StdVecBase::new()
            }
        ,
        Clone where T: Clone =>

            fn clone(&self) -> Self {
                let mut clone = StdVecBase::with_capacity(self.capacity);
                unsafe {
                    self.data.clone_elements(clone.data, self.len);
                }
                clone.len = self.len;
                clone
            }
        ,
        From<&[T]> where T: Clone =>
            
            fn from(value: &[T]) -> Self {
                let len =  SizeType::from_usize(value.len());
                let mut vec = StdVecBase::with_capacity(len);
                unsafe {
                    Pointer
                        ::new(value.as_ptr().cast_mut())
                        .unwrap()
                        .clone_elements(vec.data, len);
                }
                vec.len = len;
                vec
            }
        ,
    }

    impl<const N: usize, T, SizeType, ReservePol> From<[T; N]>
        for StdVecBase<T, SizeType, ReservePol>
        where
            SizeType: IntoUsize + FromUsize,
            ReservePol: ReservePolicy<SizeType>,
    {
        
        fn from(value: [T; N]) -> Self {
            let mut vec = StdVecBase::with_capacity(SizeType::from_usize(N));
            for v in value {
                vec.push(v);
            }
            vec
        }
    }

    impl<A, SizeType, ReservePol> FromIterator<A> for StdVecBase<A, SizeType, ReservePol>
        where 
            SizeType: IntoUsize + FromUsize,
            ReservePol: ReservePolicy<SizeType>,
    {

        fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
            let mut vec = Self::new();
            vec.extend(iter);
            vec
        }
    }

    impl<T, SizeType, ReservePol> StdVecBase<T, SizeType, ReservePol>
        where 
            SizeType: IntoUsize + FromUsize,
            ReservePol: ReservePolicy<SizeType>,
    {
        
        /// Creates an empty vector with zero capacity.
        #[inline]
        pub fn new() -> Self {
            Self {
                data: Pointer::dangling(),
                capacity: SizeType::ZERO,
                len: SizeType::ZERO,
                alloc: StdAlloc,
                _markers: PhantomData,
            }
        }

        /// Creates an empty vector with at least the given capacity.
        ///
        /// The capacity of the vector might be more than what is given. If you want a vector with an
        /// exacit capacity, use the `with_exact_capacity` instead.
        pub fn with_capacity(
            capacity: SizeType,
        ) -> Self {
            if capacity == SizeType::ZERO {
                return Default::default()
            }
            let capacity = ReservePol::grow_infallible(capacity, capacity.into_usize());
            let data = unsafe { StdAlloc
                .alloc_uninit(capacity.into_usize())
                .expect("global alloc failed").into()
            };
            Self {
                data,
                capacity,
                len: SizeType::ZERO,
                alloc: StdAlloc,
                _markers: PhantomData,
            }
        }

        /// Creates an empty vector with at the exact capacity specified.
        pub fn with_exact_capacity(
            capacity: SizeType,
        ) -> Self {
            if capacity == SizeType::ZERO {
                return Default::default()
            }
            let data = unsafe { StdAlloc
                .alloc_uninit(capacity.into_usize())
                .expect("global alloc failed").into()
            };
            Self {
                data,
                capacity,
                len: SizeType::ZERO,
                alloc: StdAlloc,
                _markers: PhantomData,
            }
        }

        pub fn with_len(
            len: SizeType,
            value: T,
        ) -> Self
            where
                T: Clone
        {
            if len == SizeType::ZERO {
                return Default::default()
            }
            let capacity = ReservePol::grow_infallible(SizeType::ZERO, len.into_usize());
            let data: Pointer<T, SizeType> = unsafe { StdAlloc 
                .alloc_uninit(capacity.into_usize())
                .expect("global alloc failed").into()
            };
            for i in 0..len.into_usize() {
                unsafe { data.add(i).write(value.clone()) };
            }
            Self {
                data,
                capacity,
                len,
                alloc: StdAlloc,
                _markers: PhantomData,
            }
        }

        pub fn with_len_reserve_exact(
            len: SizeType,
            value: T,
        ) -> Self
            where
                T: Clone
        {
            if len == SizeType::ZERO {
                return Default::default()
            }
            let capacity = len;
            let data: Pointer<T, SizeType> = unsafe { StdAlloc 
                .alloc_uninit(capacity.into_usize())
                .expect("global alloc failed").into()
            };
            for i in 0..len.into_usize() {
                unsafe { data.add(i).write(value.clone()) };
            }
            Self {
                data,
                capacity,
                len,
                alloc: StdAlloc,
                _markers: PhantomData,
            }
        }

        pub fn with_len_with<F>(
            len: SizeType,
            mut f: F,
        ) -> Self
            where
                F: FnMut(SizeType) -> T,
        {
            if len == SizeType::ZERO {
                return Default::default()
            }
            let capacity = ReservePol::grow_infallible(SizeType::ZERO, len.into_usize());
            let data: Pointer<T, SizeType> = unsafe { StdAlloc
                .alloc_uninit(capacity.into_usize())
                .expect("global alloc failed").into()
            };
            for i in 0..len.into_usize() {
                unsafe { data.add(i).write(f(SizeType::from_usize(i))) };
            }
            Self {
                data,
                capacity,
                len,
                alloc: StdAlloc,
                _markers: PhantomData,
            }
        }

        pub fn flattened<U>(
            slices: &[U],
        ) -> Self
            where
                U: AsRef<[T]>,
                T: Clone,
        {
            let capacity: usize = slices
                .iter()
                .map(|s| s.as_ref().len())
                .sum();
            let capacity = ReservePol::grow_infallible(
                SizeType::ZERO,
                capacity,
            );
            let mut res = Self::with_capacity(capacity);
            for slice in slices {
                res.append(slice.as_ref());
            }
            res
        }
    }


    impl<T, Alloc, ReservePol, IsStd, SizeType> From<&AllocVec<T, Alloc, ReservePol, IsStd, SizeType>>
        for std::vec::Vec<T>
        where
            T: Clone,
            Alloc: LocalAlloc,
            ReservePol: ReservePolicy<SizeType>,
            IsStd: Conditional,
            SizeType: IntoUsize + FromUsize,
    {

        #[inline]
        fn from(value: &AllocVec<T, Alloc, ReservePol, IsStd, SizeType>) -> Self {
            value.to_vec()
        }
    }
}

#[cfg(feature = "std")]
pub use std_features::*;

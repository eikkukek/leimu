use core::{
    borrow::{Borrow, BorrowMut}, fmt::{self, Debug, Display, Formatter}, hash::{Hash, Hasher}, mem::{ManuallyDrop, MaybeUninit}, ops::{Deref, DerefMut}, ptr::{self}, slice
};

use crate::reserve::ReserveError;

use super::Pointer;

/// A wrapper around an array, which is interpreted as a vector.
pub struct ArrayVec<T, const N: usize>
{
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> ArrayVec<T, N>
{

    /// Creates a new empty vector.
    pub fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    /// Creates a new vector with a length and value.
    pub fn with_len(len: usize, value: T) -> Result<Self, ReserveError<()>>
        where
            T: Clone,
    {
        if len > N {
            return Err(ReserveError::max_capacity_exceeded(N, len, ()))
        }
        let mut data: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for t in data.iter_mut().take(len) {
            t.write(value.clone());
        }
        Ok(Self {
            data,
            len,
        })
    }

    /// Creates a new vector with a length and a closure which returns values.
    pub fn with_len_with<F>(len: usize, mut f: F) -> Result<Self, ReserveError<()>>
        where
            F: FnMut() -> T
    {
        if len > N {
            return Err(ReserveError::max_capacity_exceeded(N, len, ()))
        }
        let mut data: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for t in data.iter_mut().take(len) {
            t.write(f());
        }
        Ok(Self {
            data,
            len,
        })
    }

    /// Maps the vector to another [`ArrayVec`] with different element type.
    pub fn map<U>(&self, f: impl FnMut(&T) -> U) -> ArrayVec<U, N>
    {
        let mut vec = ArrayVec::new();
        vec.extend(self.into_iter().map(f));
        vec
    }

    /// Returns the underlying array, assuming all elements are initialzed.
    ///
    /// # Safety
    /// Values may be unitialized if the array is not full.
    pub unsafe fn assume_init_array(self) -> [T; N] {
        let s = ManuallyDrop::new(self);
        unsafe {
            s.as_ptr()
                .cast::<[T; N]>()
                .read()
        }
    }
}

impl<T, const N: usize> ArrayVec<T, N>
{

    /// Returns the length of the vector.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the vector is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the pointer to the vector's data.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr() as *const T
    }

    /// Gets a mutable pointer to the vector's data.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr() as *mut T
    }

    /// Returns a slice over the contents of the vector.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data.as_ptr() as *const T, self.len) }
    }

    /// Returns a mutable slice over the contents of the vector.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.data.as_ptr() as *mut T, self.len) }
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
    pub unsafe fn set_len(&mut self, len: usize) {
        #[cfg(debug_assertions)]
        if len > N { panic!("len was larger than capacity") }
        self.len = len;
    } 

    /// Resizes the vector with a clonable value.
    ///
    /// If `len` is less than the current length, this shrinks the vector.
    ///
    /// This panics if `len` is larger than `N`.
    pub fn resize(&mut self, len: usize, value: T)
        where
            T: Clone
    {
        if len > N {
            panic!("maximum capacity {N} exceeded with len {len}")
        };
        let ptr = self.data.as_mut_ptr();
        if len > self.len {
            for i in self.len..len {
                unsafe { (*ptr.add(i)).write(value.clone()) };
            }
        }
        else if len < self.len {
            for i in len..self.len {
                unsafe { ptr::drop_in_place((*ptr.add(i)).as_mut_ptr()); }
            }
        }
        self.len = len;
    }

    /// Resizes the vector with the given closure.
    ///
    /// If `len` is less than the current length, this shrinks the vector.
    ///
    /// This panics if `len` is larger than `N`.
    pub fn resize_with<F>(&mut self, len: usize, mut f: F)
        where
            F: FnMut() -> T
    {
        if len > N {
            panic!("maximum capacity {N} exceeded with len {len}")
        }
        let ptr = unsafe {
            Pointer::new(self.as_mut_ptr()).unwrap_unchecked()
        };
        if len > self.len {
            for i in self.len..len {
                unsafe { (*ptr.add(i)).write(f()) };
            }
        }
        else if len < self.len {
            unsafe {
                ptr.add(len).drop_in_place(self.len - len);
            }
        }
        self.len = len
    }

    /// Tries to resize the vector with the given closure that may return an error.
    ///
    /// If `len` is less than the current length, this shrinks the vector.
    ///
    /// This panics if `len` is larger than `N`.
    pub fn try_resize_with<F, E>(
        &mut self,
        len: usize,
        mut f: F,
    ) -> Result<(), E>
        where
            F: FnMut() -> Result<T, E>,
    {
        if len > N {
            panic!("max capacity {N} exceeded, with len {len}")
        }
        let ptr = unsafe {
            Pointer::new(self.as_mut_ptr()).unwrap_unchecked()
        };
        if len > self.len {
            for i in self.len..len {
                unsafe { (*ptr.add(i))
                    .write(f().inspect_err(|_| {
                        self.len = i;
                    })?)
                };
            }
        }
        else if len < self.len {
            unsafe {
                ptr.add(len).drop_in_place(self.len - len);
            }
        }
        self.len = len;
        Ok(())
    }

    /// Appends an element to the end of the vector.
    ///
    /// This panics if the length of the vector exceeds `N`.
    pub fn push(&mut self, value: T) {
        if self.len >= N {
            panic!("maximum capacity {N} exceeded")
        }
        let ptr = unsafe { self.as_mut_ptr().add(self.len) };
        unsafe { ptr::write(ptr, value) };
        self.len += 1;
    }

    /// Inserts an element to the specified index.
    ///
    /// Panics if `index` is greater than the length of the vector or if the length of the vector
    /// exceeds `N`.
    pub fn insert(&mut self, index: usize, value: T) {
        if index >= self.len {
            panic!("index {} was out of bounds with len {} when inserting", index, self.len)
        }
        if self.len >= N {
            panic!("maximum capacity {N} exceeded")
        }
        unsafe {
            Pointer
                ::new(self.as_mut_ptr())
                .unwrap_unchecked()
                .insert_element(value, index, self.len);
            self.len += 1;
        }
    }

    /// Appends a slice to the end of the vector.
    ///
    /// If the type implements [`Copy`], consider using [`fast_append`][1]
    ///
    /// This panics if the length of the vector exceeds `N`.
    ///
    /// [1]: Self::fast_append
    pub fn append(&mut self, slice: &[T])
        where
            T: Clone
    {
        let len = self.len + slice.len();
        if len > N {
            panic!("maximum capacity {N} exceeded with len {len}")
        }
        unsafe {
            Pointer
                ::new(slice.as_ptr().cast_mut())
                .unwrap()
                .clone_elements(
                    Pointer::new(self.as_mut_ptr()).unwrap_unchecked().add(self.len),
                    slice.len()
                );
        }
        self.len = len;
    }

    /// Appends a slice to the end of the vector by [`copying`][1] the values.
    ///
    /// This may be faster than [`append`][2] for types implementing [`Copy`].
    ///
    /// This panics if the length of the vector exceeds `N`.
    ///
    /// [1]: Copy
    /// [2]: Self::append
    pub fn fast_append(&mut self, slice: &[T])
        where T: Copy
    {
        let len = self.len + slice.len();
        if len > N {
            panic!("maximum capacity {N} exceeded with len {len}")
        }
        unsafe {
            Pointer
                ::new(slice.as_ptr().cast_mut())
                .unwrap()
                .fast_clone_elements(
                    Pointer::new(self.as_mut_ptr()).unwrap_unchecked().add(self.len),
                    slice.len()
                );
        }
        self.len = len;
    } 

    /// Returns a reference to the last element of the vector, or [`None`] if the vector is empty.
    #[inline]
    pub fn last(&self) -> Option<&T> {
        if self.len == 0 {
            None
        }
        else {
            unsafe { Some(self.data[self.len - 1].assume_init_ref()) }
        }
    }

    /// Returns a mutable reference to the last element of the vector, or [`None`] if the vector is
    /// empty.
    #[inline]
    pub fn last_mut(&mut self) -> Option<&mut T> {
         if self.len == 0 {
            None
        }
        else {
            unsafe { Some(self.data[self.len - 1].assume_init_mut()) }
        }       
    }

    /// Removes an element from the specified index.
    ///
    /// Panics if `index` is greater than or equal to the length of the vector.
    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!("index {} was out of bounds with len {} when removing", index, self.len())
        }
        let ptr = self.as_mut_ptr();
        let removed = unsafe { ptr::read(ptr.add(index)) };
        for i in index..self.len - 1 {
            unsafe { ptr::write(ptr.add(i), ptr::read(ptr.add(i + 1))) }
        }
        self.len -= 1;
        removed
    }

    /// Removes an element at `index` and swaps it with the last element, if any.
    ///
    /// Panics if the index is out of bounds.
    pub fn swap_remove(&mut self, index: usize) -> T {
        if index >= self.len { panic!("index {} was out of bounds with len {} when removing", index, self.len()) }
        let ptr = self.as_mut_ptr();
        let removed = unsafe { ptr::read(ptr.add(index)) };
        self.len -= 1;
        if index != self.len {
            unsafe { ptr::write(ptr.add(index), ptr::read(ptr.add(self.len))) }
        }
        removed
    }

    /// Removes the last element of the vector and returns it, or [`None`] if the vector is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 { return None }
        self.len -= 1;
        let ptr = unsafe { self.as_mut_ptr().add(self.len).cast::<T>() };
        Some(unsafe { ptr.read() })
    }

    /// Removes all elements from the vector.
    pub fn clear(&mut self) {
        debug_assert!(self.len <= N);
        unsafe { Pointer::new(self.as_mut_ptr()).unwrap_unchecked().drop_in_place(self.len); }
        self.len = 0;
    }

    /// Retains all elements that satisfy the predicate, preserving the order of elements.
    ///
    /// See [`retain_unordered`][1] for an unordered version.
    ///
    /// [1]: Self::retain_unordered
    pub fn retain<F>(&mut self, mut p: F)
        where F: FnMut(&T) -> bool
    {
        for i in (0..self.len).rev() {
            if !p(&self[i]) {
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
        for i in (0..self.len).rev() {
            if !p(&mut self[i]) {
                self.remove(i);
            }
        }
    }

    /// Retains all elements that satisfy the predicate without preserving the order of elements.
    pub fn retain_unordered<F>(&mut self, mut p: F)
        where F: FnMut(&T) -> bool
    {
        for i in (0..self.len).rev() {
            if !p(&self[i]) {
                self.swap_remove(i);
            }
        }
    }

    /// Retains all elements that satisfy the predicate that takes a mutable reference without
    /// preserving the order of elements
    pub fn retain_unordered_mut<F>(&mut self, mut p: F)
        where F: FnMut(&mut T) -> bool
    {
        for i in (0..self.len).rev() {
            if !p(&mut self[i]) {
                self.swap_remove(i);
            }
        }
    }

    /// Removes consecutive repeated elements from the vector according to [`PartialEq`]
    pub fn dedup(&mut self)
        where T: PartialEq
    {
        for i in (0..self.len.saturating_sub(1)).rev() {
            if self[i] == self[i + 1] {
                self.remove(i + 1);
            }
        }
    }

    /// Removes consecutive repeated elements from the vector according to a closure defined
    /// equality.
    pub fn dedup_by<F>(&mut self, mut p: F)
        where F: FnMut(&T, &T) -> bool
    {
        for i in (0..self.len.saturating_sub(1)).rev() {
            if p(&self[i], &self[i + 1]) {
                self.remove(i + 1);
            }
        }
    }

    /// Removes consecutive repeated elements that resolve to a key implementing [`PartialEq`]. 
    pub fn dedup_by_key<F, K>(&mut self, mut key: F)
        where
            F: FnMut(&T) -> K,
            K: PartialEq,
    {
        for i in (0..self.len.saturating_sub(1)).rev() {
            if key(&self[i]) == key(&self[i + 1]) {
                self.remove(i + 1);
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

pub struct IterArrayVec<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    start: usize,
    end: usize,
}

impl<T, const N: usize> Iterator for IterArrayVec<T, N> {

    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start != self.end {
            let t = unsafe {
                self.data
                    .as_ptr()
                    .add(self.start)
                    .read()
                    .assume_init_read()
            };
            self.start += 1;
            Some(t)
        } else {
            None
        }
    } 

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.end - self.start;
        (size, Some(size))
    }
}

impl<T, const N: usize> DoubleEndedIterator for IterArrayVec<T, N> {

    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start != self.end {
            self.end -= 1;
            let t = unsafe {
                self.data
                    .as_ptr()
                    .add(self.end)
                    .read()
                    .assume_init_read()
            };
            Some(t)
        } else {
            None
        }
    }
}

impl<T, const N: usize> ExactSizeIterator for IterArrayVec<T, N> {}

impl<T, const N: usize> Drop for IterArrayVec<T, N> {

    fn drop(&mut self) {
        unsafe {
            Pointer::new(self.data.as_mut_ptr())
                .unwrap()
                .add(self.start)
                .cast::<T>()
                .drop_in_place(self.end - self.start);
        }
    }
}

crate::macros::impl_traits! {
    for ArrayVec<T, N: [usize] [const]>
    Drop =>

        fn drop(&mut self) -> () {
            debug_assert!(self.len <= N);
            unsafe { Pointer::new(self.as_mut_ptr()).unwrap_unchecked().drop_in_place(self.len); }
            self.len = 0;
        }
    ,
    Default =>
        
        fn default() -> Self {
            Self::new()
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
    From<&[T]> where T: Clone =>
        
        fn from(slice: &[T]) -> Self {
            let mut vec = Self::new();
            vec.append(slice);
            vec
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
    IntoIterator for &'vec =>

        type Item = &'vec T;
        type IntoIter = slice::Iter<'vec, T>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    ,
    IntoIterator for mut &'vec =>

        type Item = &'vec mut T;
        type IntoIter = slice::IterMut<'vec, T>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    ,
    IntoIterator =>

        type Item = T;
        type IntoIter = IterArrayVec<T, N>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            unsafe {
                let s = MaybeUninit::new(self);
                let data: *const [_; N] = &s.assume_init_ref().data;
                IterArrayVec {
                    data: data.read(),
                    start: 0,
                    end: s.assume_init_ref().len,
                }
            }
        }
    ,
    Clone where T: Clone =>
    
        #[inline]
        fn clone(&self) -> Self {
            let mut vec = ArrayVec::new();
            vec.append(self);
            vec
        }
    ,
    Eq where T: Eq =>,
    Debug where T: Debug =>

        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            self.as_slice().fmt(f)
        }
    ,
    Display where T: Display =>

        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            if self.len == 0 {
                <str as Display>::fmt("[]", f)?;
                return Ok(())
            }
            <char as Display>::fmt(&'[', f)?;
            for value in &self.as_slice()[0..self.len - 1] {
                value.fmt(f)?;
                <str as Display>::fmt(", ", f)?;
            }
            self.last().unwrap().fmt(f)?;
            <char as Display>::fmt(&']', f)
        }
    ,
    Hash where T: Hash =>

        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.as_slice().hash(state)
        }
    ,
}

impl<T, A, const N: usize> PartialEq<T> for ArrayVec<A, N>
    where
        T: AsRef<[A]>,
        A: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<A, const N: usize> Extend<A> for ArrayVec<A, N> {

    #[inline]
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T, const N: usize> PartialEq<ArrayVec<T, N>> for [T]
    where
        T: PartialEq
{

    fn eq(&self, other: &ArrayVec<T, N>) -> bool {
        self == other.as_ref()
    }
}

impl<T, const N: usize> PartialEq<ArrayVec<T, N>> for &[T]
    where
        T: PartialEq
{

    fn eq(&self, other: &ArrayVec<T, N>) -> bool {
        *self == other.as_ref()
    }
}

impl<const N: usize, T> From<[T; N]> for ArrayVec<T, N> {
    
    fn from(value: [T; N]) -> Self {
        let mut vec = ArrayVec::new();
        vec.extend(value);
        vec
    }
}

impl<const N: usize, A> FromIterator<A> for ArrayVec<A, N> {

    #[inline]
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut vec = ArrayVec::new();
        vec.extend(iter);
        vec
    }
} 

//! A compact slot map implementation with support for custom allocators.
//!
//! Slot map is a data structure that associates values with *opaque, stable handles* (indices).
//!
//! Unlike `Vec`, removal doesn't shift elements, and re-insertion may reuse free slots.
//!
//! # New Types
//! - [`SlotMap`]: A slot map using [`GlobalAlloc`][1]. Requires the "std feature".
//! - [`DynSlotMap`]: A slot map using a local allocator.
//! - [`FixedSlotMap`]: [`DynSlotMap`] with a fixed-capacity.
//!
//! # Features
//!
//! - Constant-time insertion, removal and lookup
//! - Stable handles
//! - Custom allocators
//!
//! [1]: std::alloc::alloc
//!
//! # Examples
//! ```rust
//! use leimu_mem::slot_map::SlotMap;
//!
//! let mut map = SlotMap::new();
//! let key1 = map.insert("hello");
//! let key2 = map.insert("world");
//! assert_eq!(map.get(key1).ok(), Some(&"hello"));
//! assert_eq!(map.remove(key1).ok(), Some("hello"));
//! assert_eq!(map.get(key1).ok(), None);
//! assert_eq!(map.get(key2).ok(), Some(&"world"));
//! ```

use core::{
    marker::PhantomData,
    mem::MaybeUninit,
    ops::{Index, IndexMut, Deref},
    fmt::{self, Debug, Display},
    error::Error,
};

use crate::{
    conditional::{Conditional, False, True},
    alloc::{LocalAlloc, LocalAllocExt, LocalAllocWrap},
    vec::Pointer,
    reserve::*,
    int::{UInteger, NonZeroInteger},
};

/// An error indicating that a [`SlotIndex`] is invalid.
#[derive(Debug)]
pub enum IndexError<IndexType: UInteger = u32> {
    /// Indicates that the index is stale.
    StaleIndex {
        /// The index that is stale.
        index: SlotIndex<(), IndexType>,
        /// The version of the slot.
        slot_version: IndexType,
    },
    /// Indicates that the index is out of bounds.
    IndexOutOfBounds {
        /// The index that is out of bounds.
        index: SlotIndex<(), IndexType>,
        /// The capacity of the slot map.
        capacity: u32,
    },
}

impl<IndexType: UInteger> Display for IndexError<IndexType> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StaleIndex { index, slot_version } =>
                write!(f, "stale slot map index {index}, slot version is {slot_version}"),
            Self::IndexOutOfBounds { index, capacity } =>
                write!(f, "index {index} is out of bounds with capacity {capacity}")
        }
    }
}

impl<IndexType: UInteger> Error for IndexError<IndexType> {}

use IndexError::{StaleIndex, IndexOutOfBounds};

mod policy {

    use super::*;

    pub struct Dyn;

    unsafe impl ReservePolicy<u32> for Dyn {

        fn can_grow() -> bool {
            true
        }

        fn grow(current: u32, required: usize) -> core::result::Result<u32, ReserveError<()>> {
            let power_of_2 = required.next_power_of_two().max(2);
            if power_of_2 > u32::MAX as usize {
                Err(ReserveError::max_capacity_exceeded(u32::MAX as usize, power_of_2, ()))
            } else if power_of_2 <= current as usize {
                Ok(current)
            } else { Ok(power_of_2.max(2) as u32) }
        }

        fn grow_infallible(current: u32, required: usize) -> u32 {
            let power_of_2 = required.next_power_of_two().max(2);
            if power_of_2 <= current as usize {
                current
            } else if power_of_2 > u32::MAX as usize {
                panic!("maximum capacity {} reached", u32::MAX)
            } else {
                power_of_2.max(2) as u32
            }
        }
    }

    pub type Fixed = crate::vec::FixedPolicy32;
}

struct Slot<T, IndexType: UInteger> {
    value: MaybeUninit<T>,
    version: IndexType::NonZero,
    next_free_index: Option<Option<u32>>,
}

impl<T: Clone, IndexType: UInteger> Clone for Slot<T, IndexType> {

    fn clone(&self) -> Self {
        Self {
            value: if self.next_free_index.is_none() {
                unsafe {
                    MaybeUninit::new(self.value.assume_init_ref().clone())
                }
            } else {
                MaybeUninit::uninit()
            },
            version: self.version,
            next_free_index: self.next_free_index,
        }
    }
}

impl<T, IndexType: UInteger> Slot<T, IndexType> {

    fn empty(next_free_index: Option<u32>) -> Self {
        Self {
            value: MaybeUninit::uninit(),
            version: unsafe {
                IndexType::NonZero::new_unchecked(IndexType::ONE)
            },
            next_free_index: Some(next_free_index),
        }
    }
}

/// A slot index returned by a slot map.
///
/// Contains a version and an index.
#[must_use]
pub struct SlotIndex<T, IndexType = u32>
    where IndexType: UInteger,
{
    version: IndexType::NonZero,
    index: u32,
    _marker: PhantomData<T>,
}

impl<T, IndexType> Display for SlotIndex<T, IndexType>
    where IndexType: UInteger
{

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.version, self.index)
    }
}

impl<T, IndexType> SlotIndex<T, IndexType>
    where
        IndexType: UInteger
{

    /// Gets the index part of [`SlotIndex`].
    #[inline]
    pub fn index(&self) -> u32 {
        self.index
    }
    
    /// Gets the version part of [`SlotIndex`].
    #[inline]
    pub fn version(&self) -> IndexType {
        self.version.get()
    }
   
    #[inline]
    fn unit_index(self) -> SlotIndex<(), IndexType> {
        SlotIndex {
            version: self.version,
            index: self.index,
            _marker: PhantomData,
        }
    }
}

unsafe impl<T, IndexType: UInteger> Send for SlotIndex<T, IndexType> {}
unsafe impl<T, IndexType: UInteger> Sync for SlotIndex<T, IndexType> {}

impl<T, IndexType: UInteger> Debug for SlotIndex<T, IndexType> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SlotIndex")
            .field("version", &self.version)
            .field("index", &self.index)
            .finish()
    }
}

impl<T, IndexType: UInteger> Default for SlotIndex<T, IndexType>
{

    #[inline]
    fn default() -> Self {
        Self {
            version: unsafe {
                IndexType::NonZero::new_unchecked(IndexType::ONE)
            },
            index: u32::MAX,
            _marker: PhantomData,
        }
    }
}

impl<T, IndexType: UInteger> Clone for SlotIndex<T, IndexType> {

    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, IndexType: UInteger> Copy for SlotIndex<T, IndexType> {}

impl<T, IndexType: UInteger> PartialEq for SlotIndex<T, IndexType> {

    fn eq(&self, other: &Self) -> bool {
        self.version == other.version &&
        self.index == other.index
    }
}

impl<T, IndexType: UInteger> Eq for SlotIndex<T, IndexType> {}

impl<T, IndexType: UInteger> core::hash::Hash for SlotIndex<T, IndexType> {

    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.version.hash(state);
        self.index.hash(state);
    }
}

mod base {

    use super::*;

    pub struct Map<T, Alloc, ReservePol, IsStd, IndexType = u32>
        where
            Alloc: LocalAlloc,
            ReservePol: ReservePolicy<u32>,
            IsStd: Conditional,
            IndexType: UInteger,
    {
        pub(super) data: Pointer<Slot<T, IndexType>, u32>,
        pub(super) capacity: u32,
        pub(super) len: u32,
        pub(super) free_head: Option<u32>,
        pub(super) alloc: Alloc,
        pub(super) _marker: PhantomData<(IsStd, ReservePol)>,
    }
}

/// A dynamic slot map storing values of type `T`, backed by allocator 'Alloc'.
///
/// Provides stable, opaque handles for accessing values. Removal leaves versioned
/// empty slots and insertions reuse free slots.
///
/// See also [`SlotMap`] for a version using [`GlobalAlloc`][1].
///
/// # Type parameters
///
/// - `T`: value type
/// - `Alloc`: allocator implementing [`LocalAlloc`]
///
/// # Errors
///
/// Insertion and reservations return `Result` and may fail due to:
///
/// - Allocation failure
/// - Capacity reaching over u32::MAX
///
/// # Safety
///
/// The allocator must return valid pointers that are aligned to the requested alignment
/// and that are not freed or overwritten as long as the slot map uses them.
///
/// [1]: std::alloc::alloc
///
/// # Examples
/// ```rust
/// let allocator = MyLocalAlloc::default();
/// let mut map = DynSlotMap::new(&allocator);
/// let key = map.insert("value").unwrap();
/// map.remove(key);
pub type DynSlotMap<T, Alloc, Wrap> = base::Map<T, LocalAllocWrap<Alloc, Wrap>, policy::Dyn, False>;

/// A fixed-capacity slot map storing values of type `T`, backed by allocator `Alloc`.
///
/// Provides stable, opaque handles for accessing values. Removal leaves versioned
/// empty slots and insertions reuse free slots.
///
/// See also [`DynSlotMap`] for a version with dynamic capacity.
///
/// # Type parameters
///
/// - `T`: value type
/// - `Alloc`: allocator implementing [`LocalAlloc`]
///
/// # Errors
///
/// Insertion and reservations return `Result` and may fail due to:
///
/// - Allocation failure
/// - Capacity reaching over u32::MAX
/// - Reaching fixed capacity
///
/// # Safety
///
/// The allocator must return valid pointers that are aligned to the requested alignment
/// and that are not freed or overwritten as long as the slot map uses them.
///
/// # Example
///
/// ```rust
/// let allocator = MyLocalAlloc::default();
/// let mut map = FixedSlotMap::with_capacity(8, &allocator).unwrap();
/// let key = map.insert("value").unwrap();
/// map.remove(key);
/// ```
pub type FixedSlotMap<T, Alloc, Wrap> = base::Map<T, LocalAllocWrap<Alloc, Wrap>, policy::Fixed, False>;

impl<T, Alloc, Wrap, ReservePol, IndexType>
    base::Map<T, LocalAllocWrap<Alloc, Wrap>, ReservePol, False, IndexType>
    where
        Alloc: LocalAlloc,
        Wrap: Deref<Target = Alloc>,
        IndexType: UInteger,
        ReservePol: ReservePolicy<u32>,
{

    /// Creates an empty map with an [`allocator`][1].
    ///
    /// [1]: LocalAlloc
    pub fn new(alloc: Wrap) -> Self {
        Self {
            data: Pointer::dangling(),
            capacity: 0,
            len: 0,
            free_head: None,
            alloc: LocalAllocWrap::new(alloc),
            _marker: PhantomData,
        }
    }

    /// Creates a map with `capacity` and an [`allocator`][1].
    ///
    /// Returns an error if the allocation fails.
    ///
    /// [1]: LocalAlloc
    pub fn with_capacity(capacity: u32, alloc: Wrap) -> Result<Self, ReserveError<()>> {
        if capacity == 0 {
            return Ok(Self::new(alloc))
        }
        let capacity = ReservePol::grow(capacity, capacity as usize)?;
        let data: Pointer<Slot<T, IndexType>, u32> = unsafe { alloc
            .alloc_uninit(capacity as usize)
            .map_err(|err| ReserveError::alloc_error(err, ()))?
            .into()
        };
        for i in 0..capacity  - 1 {
            unsafe {
                data.add(i as usize).write(Slot::empty(Some(i + 1)));
            }
        }
        unsafe { data.add(capacity as usize - 1).write(Slot::empty(None)) };
        Ok(Self {
            data,
            capacity,
            len: 0,
            free_head: Some(0),
            alloc: LocalAllocWrap::new(alloc),
            _marker: PhantomData,
        })
    }

    /// Inserts a value to the map.
    ///
    /// May panic if an allocation fails.
    #[inline]
    pub fn insert(&mut self, value: T) -> SlotIndex<T, IndexType> {
        self.insert_internal(value).unwrap()
    }

    /// Tries to insert a value to the map, returning an error if an allocation fails.
    #[inline]
    pub fn try_insert(&mut self, value: T) -> Result<SlotIndex<T, IndexType>, ReserveError<T>> {
        self.insert_internal(value)
    }
}

impl<T, Alloc, ReservePol, IsStd, IndexType> base::Map<T, Alloc, ReservePol, IsStd, IndexType>
    where
        Alloc: LocalAlloc,
        ReservePol: ReservePolicy<u32>,
        IsStd: Conditional,
        IndexType: UInteger,
{

    /// Returns the number of elements contained in the map.
    #[inline]
    pub fn len(&self) -> u32 {
        self.len
    }

    /// Returns whether the map is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the map's internal capacity.
    #[inline]
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Reserves more slots.
    ///
    /// Does nothing if `capacity` is less than or equal to the current capacity.
    ///
    /// This may reserve more slots than specified, see [`reserve_exact`][1] for reserving exact
    /// amount of slots.
    ///
    /// [1]: Self::reserve_exact
    #[inline]
    pub fn reserve(&mut self, capacity: u32) {
        let capacity = ReservePol
            ::grow(self.capacity, capacity as usize)
            .unwrap();
        self.reserve_internal(capacity).unwrap();
    }

    /// Reserves an exact amount of slots.
    ///
    /// Does nothing if `capacity` is less than or equal to the current capacity.
    #[inline]
    pub fn reserve_exact(&mut self, capacity: u32) {
        self.reserve_internal(capacity).unwrap();
    }

    /// Tries to reserve more slots, returning an error if maximum capacity is reached, or if an
    /// allocation fails.
    #[inline]
    pub fn try_reserve(&mut self, capacity: u32) -> Result<(), ReserveError<()>> {
        let capacity = ReservePol
            ::grow(self.capacity, capacity as usize)?;
        self.reserve_internal(capacity)
    }

    /// Tries to reserve an exact amount of slots, returning an error if maximum capacity is reached,
    /// or if an allocation fails.
    #[inline]
    pub fn try_reserve_exact(&mut self, capacity: u32) -> Result<(), ReserveError<()>> {
        self.reserve_internal(capacity)
    }

    fn reserve_internal(&mut self, capacity: u32) -> Result<(), ReserveError<()>> {
        if capacity <= self.capacity { return Ok(()) }
        let tmp: Pointer<Slot<T, IndexType>, u32> = unsafe { self.alloc
            .alloc_uninit(capacity as usize)
            .map_err(|err| ReserveError::alloc_error(err, ()))?
            .into()
        };
        unsafe {
            self.data.move_elements(tmp, self.capacity);
        };
        for i in self.capacity..capacity - 1 {
            unsafe {
                tmp.add(i as usize).write(Slot::empty(Some(i + 1)));
            }
        }
        unsafe {
            tmp.add(capacity as usize - 1).write(
                Slot::empty(self.free_head)
            );
        }
        self.free_head = Some(self.capacity);
        if self.capacity != 0 {
            unsafe { self.alloc
                .free_uninit(*self.data, self.capacity as usize);
        }
        }
        self.data = tmp;
        self.capacity = capacity;
        Ok(())
    }

    fn insert_internal(&mut self, value: T) -> Result<SlotIndex<T, IndexType>, ReserveError<T>>
    {
        if self.free_head.is_none() {
            let capacity = self.capacity as usize * 2;
            if capacity > u32::MAX as usize {
                return Err(ReserveError::max_capacity_exceeded(u32::MAX, capacity, value))
            }
            if let Err(err) = self.try_reserve(capacity as u32) {
                return Err(err.with_value(value))
            }
        }
        let index = self.free_head.unwrap();
        let free_slot = unsafe { self.data.add(index as usize).as_mut() };
        self.free_head = free_slot.next_free_index.unwrap();
        free_slot.value.write(value);
        free_slot.next_free_index = None;
        self.len += 1;
        Ok(SlotIndex {
            version: free_slot.version,
            index,
            _marker: PhantomData,
        })
    }

    /// Removes a value from the slot map, returning an error if the index is stale or otherwise
    /// invalid.
    pub fn remove(&mut self, index: SlotIndex<T, IndexType>) -> Result<T, IndexError<IndexType>>
    {
        if index.index >= self.capacity {
            return Err(
                IndexOutOfBounds {
                    index: index.unit_index(),
                    capacity: self.capacity,
                }
            )
        }
        let ptr = unsafe { self.data.add(index.index as usize) };
        let mut slot = unsafe { ptr.read() };
        let index_version = index.version;
        if slot.version != index_version {
            return Err(StaleIndex {
                index: index.unit_index(),
                slot_version: slot.version.get(),
            })
        }
        let value = unsafe { slot.value.assume_init() };
        slot.version = IndexType::NonZero
            ::new(slot.version.get().wrapping_add(IndexType::ONE))
            .unwrap_or_else(|| unsafe { IndexType::NonZero::new_unchecked(IndexType::ONE) });
        slot.next_free_index = Some(self.free_head);
        slot.value = MaybeUninit::uninit();
        unsafe {
            ptr.write(slot);
        }
        self.free_head = Some(index.index);
        self.len -= 1;
        Ok(value)
    }

    /// Returns whether the map contains `index`.
    pub fn contains(&self, index: SlotIndex<T, IndexType>) -> bool {
        if index.index >= self.capacity {
            return false
        }
        let index_version = index.version;
        let slot = unsafe { self.data.add(index.index as usize).as_ref() };
        slot.version == index_version
    }

    /// Gets a reference to a value at `index`, returning an error if the index is stale or othewise
    /// invalid.
    pub fn get(&self, index: SlotIndex<T, IndexType>) -> Result<&T, IndexError<IndexType>> {
        if index.index >= self.capacity {
            return Err(IndexOutOfBounds {
                index: index.unit_index(),
                capacity: self.capacity,
            })
        }
        let index_version = index.version;
        let slot = unsafe { self.data.add(index.index as usize).as_ref() };
        if slot.version != index_version {
            return Err(StaleIndex {
                index: index.unit_index(),
                slot_version: slot.version.get(),
            })
        }
        assert!(slot.next_free_index.is_none(), "invalid index");
        unsafe {
            Ok(slot.value.assume_init_ref())
        }
    }

    /// Gets a mutable reference to a value at `index`, returning an error if the index is stale or
    /// otherwise invalid.
    pub fn get_mut(&mut self, index: SlotIndex<T, IndexType>) -> Result<&mut T, IndexError<IndexType>> {
        if index.index >= self.capacity {
            return Err(IndexOutOfBounds {
                index: index.unit_index(),
                capacity: self.capacity,
            })
        }
        let index_version = index.version;
        let slot = unsafe { self.data.add(index.index as usize).as_mut() };
        if slot.version != index_version {
            return Err(StaleIndex {
                index: index.unit_index(),
                slot_version: slot.version.get(),
            })
        }
        assert!(slot.next_free_index.is_none(), "invalid index");
        unsafe {
            Ok(slot.value.assume_init_mut())
        }
    }
    
    /// Gets a reference to value at `index` bypassing all validity checks including bounds checks.
    ///
    /// # Safety
    /// The index *must* be a valid index, otherwise the value might be uninitialized or out of
    /// bounds.
    #[inline]
    pub unsafe fn get_unchecked(&self, index: SlotIndex<T, IndexType>) -> &T {
        unsafe {
            self.data.add(index.index as usize).as_ref()
            .value.assume_init_ref()
        }
    }

    /// Gets a mutable reference to value at `index` bypassing all validity checks including bounds
    /// checks.
    ///
    /// # Safety
    /// The index *must* be a valid index, otherwise the value might be uninitialized or out of
    /// bounds.
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: SlotIndex<T, IndexType>) -> &mut T {
        unsafe {
            self.data.add(index.index as usize).as_mut()
            .value.assume_init_mut()
        }
    }

    /// Clears all slots in the map.
    pub fn clear(&mut self) {
        for i in 0..self.capacity() {
            unsafe {
                let slot = self.data.add(i as usize).read();
                if slot.next_free_index.is_none() {
                    self.remove(SlotIndex {
                        version: slot.version,
                        index: i,
                        _marker: PhantomData,
                    }).unwrap();
                }
            }
        }
        self.len = 0;
    }

    /// Returns an iterator over all indices and values.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T, IndexType> {
        unsafe {
            Iter::new(self.data, self.data.add(self.capacity as usize))
        }
    }

    /// Returns an iterator over all indices.
    #[inline]
    pub fn indices(&self) -> IterIndices<'_, T, IndexType> {
        unsafe {
            IterIndices::new(self.data, self.data.add(self.capacity() as usize))
        }
    }

    /// Returns an iterator over all values.
    #[inline]
    pub fn values(&self) -> IterValues<'_, T, IndexType> {
        unsafe {
            IterValues::new(self.data, self.data.add(self.capacity() as usize))
        }
    }

    /// Returns an iterator over all indices and values, with the values returned as mutable
    /// references.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T, IndexType> {
        unsafe {
            IterMut::new(self.data, self.data.add(self.capacity as usize))
        }
    }

    /// Returns an iterator over all values as mutable references.
    #[inline]
    pub fn values_mut(&self) -> IterValuesMut<'_, T, IndexType> {
        unsafe {
            IterValuesMut::new(self.data, self.data.add(self.capacity() as usize))
        }
    }
}

mod iter {

    use super::*;

    pub struct IndexValue;
    pub struct Indices;
    pub struct Values;

    pub struct Base<'a, T, IndexType, IsMut, Type>
        where
            IndexType: UInteger,
            IsMut: Conditional,
    {
        pub(super) ptr: Pointer<Slot<T, IndexType>, u32>,
        pub(super)end: Pointer<Slot<T, IndexType>, u32>,
        pub(super) index: u32,
        pub(super) _marker: PhantomData<(&'a T, IsMut, Type)>
    }

    impl<'a, T, IndexType, IsMut, Type> Base<'a, T, IndexType, IsMut, Type>
        where
            IndexType: UInteger, 
            IsMut: Conditional,
    {

        pub(super) unsafe fn new(
            mut ptr: Pointer<Slot<T, IndexType>, u32>,
            end: Pointer<Slot<T, IndexType>, u32>
        ) ->  Self
        {
            let mut index = 0;
            unsafe {
                while ptr != end {
                    if ptr.as_ref().next_free_index.is_none() {
                        break
                    }
                    ptr = ptr.add(1);
                    index += 1;
                }
            }
            Self {
                ptr,
                end,
                index,
                _marker: PhantomData,
            }
        }
    }
}

/// An iterator over indices and values.
pub type Iter<'a, T, IndexType> = iter::Base<'a, T, IndexType, False, iter::IndexValue>;
/// An iterator over indices.
pub type IterIndices<'a, T, IndexType> = iter::Base<'a, T, IndexType, False, iter::Indices>;
/// An iterator over values.
pub type IterValues<'a, T, IndexType> = iter::Base<'a, T, IndexType, False, iter::Values>;

/// An iterator over indices and values, where the values are mutable references.
pub type IterMut<'a, T, IndexType> = iter::Base<'a, T, IndexType, True, iter::IndexValue>;
/// An iterator over values as mutable references.
pub type IterValuesMut<'a, T, IndexType> = iter::Base<'a, T, IndexType, True, iter::Values>;

impl<'a, T, IndexType> Iterator for Iter<'a, T, IndexType>
    where IndexType: UInteger,
{

    type Item = (SlotIndex<T, IndexType>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        }
        else {
            unsafe {
                let item = self.ptr.as_ref();
                let current_idx = self.index;
                let mut ptr = self.ptr.add(1);
                let end = self.end;
                let mut index = self.index + 1;
                while ptr != end {
                    if ptr.as_ref().next_free_index.is_none() {
                        break
                    }
                    ptr = ptr.add(1);
                    index += 1;
                }
                self.ptr = ptr;
                self.index = index;
                Some((
                    SlotIndex {
                        version: item.version,
                        index: current_idx,
                        _marker: PhantomData,
                    },
                    item.value.assume_init_ref(),
                ))
            }
        }
    }
}

impl<'a, T, IndexType> Iterator for IterIndices<'a, T, IndexType>
    where IndexType: UInteger
{

    type Item = SlotIndex<T, IndexType>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        }
        else {
            unsafe {
                let item = self.ptr.as_ref();
                let current_idx = self.index;
                let mut ptr = self.ptr.add(1);
                let end = self.end;
                let mut index = self.index + 1;
                while ptr != end {
                    if ptr.as_ref().next_free_index.is_none() {
                        break
                    }
                    ptr = ptr.add(1);
                    index += 1;
                }
                self.ptr = ptr;
                self.index = index;
                Some(
                    SlotIndex {
                        version: item.version,
                        index: current_idx,
                        _marker: PhantomData,
                    },
                )
            }
        }
    }
}

impl<'a, T, IndexType> Iterator for IterValues<'a, T, IndexType>
    where IndexType: UInteger
{

    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        }
        else {
            unsafe {
                let item = self.ptr.as_ref();
                let mut ptr = self.ptr.add(1);
                let end = self.end;
                let mut index = self.index + 1;
                while ptr != end {
                    if ptr.as_ref().next_free_index.is_none() {
                        break
                    }
                    ptr = ptr.add(1);
                    index += 1;
                }
                self.ptr = ptr;
                self.index = index;
                Some(
                    item.value.assume_init_ref(),
                )
            }
        }
    }
}

impl<'a, T, IndexType> Iterator for IterMut<'a, T, IndexType>
    where IndexType: UInteger
{

    type Item = (SlotIndex<T, IndexType>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        }
        else {
            unsafe {
                let item = self.ptr.as_mut();
                let current_idx = self.index;
                let mut ptr = self.ptr.add(1);
                let end = self.end;
                let mut index = self.index + 1;
                while ptr != end {
                    if ptr.as_ref().next_free_index.is_none() {
                        break
                    }
                    ptr = ptr.add(1);
                    index += 1;
                }
                self.ptr = ptr;
                self.index = index;
                Some((
                    SlotIndex {
                        version: item.version,
                        index: current_idx,
                        _marker: PhantomData,
                    },
                    item.value.assume_init_mut(),
                ))
            }
        }
    }
}

impl<'a, T, IndexType> Iterator for IterValuesMut<'a, T, IndexType>
    where IndexType: UInteger
{

    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        }
        else {
            unsafe {
                let item = self.ptr.as_mut();
                let mut ptr = self.ptr.add(1);
                let end = self.end;
                let mut index = self.index + 1;
                while ptr != end {
                    if ptr.as_ref().next_free_index.is_none() {
                        break
                    }
                    ptr = ptr.add(1);
                    index += 1;
                }
                self.ptr = ptr;
                self.index = index;
                Some(
                    item.value.assume_init_mut(),
                )
            }
        }
    }
}

/// An iterator, which owns the slot map.
///
/// Acquired by calling [`IntoIterator`].
pub struct IterMove<T, Alloc, ReservePol, IsStd, IndexType>
    where
        Alloc: LocalAlloc,
        ReservePol: ReservePolicy<u32>,
        IsStd: Conditional,
        IndexType: UInteger,
{
    slot_map: base::Map<T, Alloc, ReservePol, IsStd, IndexType>,
    off: u32,
}

impl<T, Alloc, ReservePol, IsStd, IndexType> Iterator for IterMove<T, Alloc, ReservePol, IsStd, IndexType>
    where
        Alloc: LocalAlloc,
        ReservePol: ReservePolicy<u32>,
        IsStd: Conditional,
        IndexType: UInteger,
{

    type Item = (SlotIndex<T, IndexType>, T);

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = unsafe { IterMut::new(
            self.slot_map.data.add(self.off as usize),
            self.slot_map.data.add(self.slot_map.capacity as usize)
        )};
        iter.index += self.off;
        self.off = iter.index;
        let index = iter.next()?.0;
        Some((index, self.slot_map.remove(index).unwrap()))
    }
}

use base::Map;

crate::macros::impl_traits!(
    for Map<T, Alloc: [LocalAlloc], ReservePol: [ReservePolicy<u32>], IsStd: [Conditional], IndexType: [UInteger]>
    Index<SlotIndex<T, IndexType>> =>

        type Output = T;

        fn index(&self, index: SlotIndex<T, IndexType>) -> &Self::Output {
            if index.index >= self.capacity {
                panic!("index {} out of bounds with capacity {}",
                    index.index, self.capacity
                )
            }
            let index_version = index.version;
            let slot = unsafe { self.data.add(index.index as usize).as_ref() };
            if slot.version != index_version {
                panic!("stale index: slot version {}, index version {}",
                    slot.version, index_version,
                );
            }
            assert!(slot.next_free_index.is_none(), "invalid index");
            unsafe {
                slot.value.assume_init_ref()
            }
        }
    ,
    IndexMut<SlotIndex<T, IndexType>> =>

        fn index_mut(&mut self, index: SlotIndex<T, IndexType>) -> &mut Self::Output {
            if index.index >= self.capacity {
                panic!("index {} out of bounds with capacity {}", index.index, self.capacity)
            }
            let index_version = index.version;
            let slot = unsafe { self.data.add(index.index as usize).as_mut() };
            if slot.version != index_version {
                panic!("stale index: slot version {}, index version {}", 
                    slot.version, index_version,
                );
            }
            assert!(slot.next_free_index.is_none(), "invalid index");
            unsafe {
                slot.value.assume_init_mut()
            }
        }
    ,
    IntoIterator for &'map =>

        type Item = (SlotIndex<T, IndexType>, &'map T);
        type IntoIter = Iter<'map, T, IndexType>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    ,
    IntoIterator for mut &'map =>

        type Item = (SlotIndex<T, IndexType>, &'map mut T);
        type IntoIter = IterMut<'map, T, IndexType>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    ,
    IntoIterator =>

        type Item = (SlotIndex<T, IndexType>, T);
        type IntoIter = IterMove<T, Alloc, ReservePol, IsStd, IndexType>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            IterMove {
                slot_map: self,
                off: 0,
            }
        }
    ,
    Drop =>

        fn drop(&mut self) {
            self.clear();
            if self.capacity() != 0 {
                unsafe {
                    self.alloc.free_uninit(*self.data, self.len() as usize);
                }
            }
        }
    ,
);

unsafe impl<
    T: Send,
    Alloc: LocalAlloc + Send,
    ReservePol: ReservePolicy<u32>,
    IsStd: Conditional,
> Send for base::Map<T, Alloc, ReservePol, IsStd> {}

unsafe impl<
    T: Sync,
    Alloc: LocalAlloc + Sync,
    ReservePol: ReservePolicy<u32>,
    IsStd: Conditional,
> Sync for base::Map<T, Alloc, ReservePol, IsStd> {}

#[cfg(feature = "std")]
mod std_features {

    use super::*;

    use crate::alloc::StdAlloc;

    /// A dynamic slot map storing values of type `T`, backed by [`GlobalAlloc`][1].
    ///
    /// # Type parameters
    ///
    /// - `T`: the value type
    ///
    /// [1]: std::alloc::alloc
    ///
    /// # Example
    ///
    /// ```rust
    /// use leimu_mem::slot_map::SlotMap;
    ///
    /// let mut map = SlotMap::new();
    /// let key1 = map.insert("hello");
    /// let key2 = map.insert("world");
    /// assert_eq!(map.get(key1).ok(), Some(&"hello"));
    /// assert_eq!(map.remove(key1).ok(), Some("hello"));
    /// assert_eq!(map.get(key1).ok(), None);
    /// assert_eq!(map.get(key2).ok(), Some(&"world"));
    /// ```
    pub type SlotMap<T, IndexType = u32> = base::Map<T, StdAlloc, policy::Dyn, True, IndexType>;

    impl<T, IndexType> SlotMap<T, IndexType>
        where IndexType: UInteger
    {

        /// Creates an empty map.
        pub fn new() -> Self {
            Self {
                data: Pointer::dangling(),
                capacity: 0,
                len: 0,
                free_head: None,
                alloc: StdAlloc,
                _marker: PhantomData,
            }
        }

        /// Creates a new map with `capacity`.
        pub fn with_capacity(capacity: u32) -> Self {
            if capacity == 0 {
                return Self::new()
            }
            let capacity = policy::Dyn::grow(0, capacity as usize).unwrap();
            let data: Pointer<Slot<T, IndexType>, u32> = unsafe { StdAlloc
                .alloc_uninit(capacity as usize)
                .expect("global alloc failed")
                .into()
            };
            for i in 0..capacity - 1 {
                unsafe {
                    data.add(i as usize).write(Slot::empty(Some(i + 1)));
                }
            }
            unsafe { data.add(capacity as usize - 1).write(Slot::empty(None)) };
            Self {
                data,
                capacity,
                len: 0,
                free_head: Some(0),
                alloc: StdAlloc,
                _marker: PhantomData,
            }
        }
        
        /// Inserts a value to the map.
        pub fn insert(&mut self, value: T) -> SlotIndex<T, IndexType> {
            self.insert_internal(value).unwrap()
        }
    }

    impl<T, IndexType> Clone for SlotMap<T, IndexType>
        where
            T: Clone,
            IndexType: UInteger,
    {

        fn clone(&self) -> Self {
            let data: Pointer<Slot<T, IndexType>, u32> = unsafe { StdAlloc
                .alloc_uninit(self.capacity as usize)
                .expect("global alloc failed")
                .into()
            };
            for i in 0..self.capacity as usize {
                unsafe {
                    data.add(i).write(
                        self.data.add(i).as_ref().clone()
                    );
                }
            }
            Self {
                data,
                capacity: self.capacity,
                len: self.len,
                free_head: self.free_head,
                alloc: StdAlloc,
                _marker: PhantomData,
            }
        }
    }

    impl<T, IndexType: UInteger> Default for SlotMap<T, IndexType> {

        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(feature = "std")]
pub use std_features::*;

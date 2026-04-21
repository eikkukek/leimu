mod definitions;
mod global;
mod linear;

use tuhka::vk;

use crate::{
    error::*,
    gpu::prelude::*,
};

pub use definitions::*;
pub use linear::*;
pub use global::*;

#[derive(Debug, Error)]
pub enum MemoryBinderError {
    #[display("vulkan error")]
    VulkanError(#[from] #[source] vk::Result),
    #[display("out of device memory with allocation size {size} and alignment {align}")]
    OutOfDeviceMemory { size: u64, align: u64, },
    #[display("allocated memory is unmappable")]
    UnmappableMemory,
    #[display("allocation size is zero")]
    ZeroSizeAlloc,
    #[display("incompatible memory requirements")]
    IncompatibleMemoryRequirements,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum HostCoherency {
    /// Specifies that the memory is not available for host reads/writes
    None,
    /// Specifies that the memory is mappable, but not host-coherent.
    Mappable,
    /// Specifies that the memory is mappable and host-coherent.
    Coherent,
}

#[derive(Clone, Copy)]
pub struct MappedBufferMemoryRange {
    pub buffer_id: BufferId,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}

/// A trait for [`vk::DeviceMemory`] objects.
///
/// # Safety
/// - The [`vk::DeviceMemory`] returned by [`DeviceMemory::device_memory`] *should* be valid for at
///   least the lifetime of [`self`].
/// - The offset and size returned by [`DeviceMemory::offset`] and [`DeviceMemory::size`]
///   respectively *should* be within the inner [`vk::DeviceMemory`]'s range.
/// - This *should* not try to remap already mapped [`vk::DeviceMemory`].
pub unsafe trait DeviceMemory: 'static + Send + Sync {

    /// Returns the handle of the inner [`device memory`][1] object as an [`u64`] value.
    ///
    /// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemory.html
    fn handle(&self) -> u64;

    /// Returns the size of the inner [`device memory`][1] allocation.
    ///
    /// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemory.html
    fn memory_size(&self) -> u64;

    /// Returns the offset into the inner [`device memory`][1] of this allocation.
    ///
    /// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemory.html
    fn offset(&self) -> vk::DeviceSize;

    /// Returns the size of this allocation.
    fn size(&self) -> vk::DeviceSize;

    /// Returns whether the memory is mapped to host memory.
    fn is_mapped(&self) -> bool;

    /// Tries to map the region of this allocation from the inner [`device memory`][1].
    ///
    /// If the inner [`device memory`][1] is already mapped, this *should* either return a pointer
    /// to the already mapped memory or remap the memory.
    ///
    /// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemory.html
    fn map_memory(&mut self) -> Result<MemoryMap>;

    /// Unmaps the whole [`device memory`][1] object.
    ///
    /// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemory.html
    fn unmap_memory(&mut self) -> Result<()>;

    /// Returns whether the memory allocated is optimal as defined when creating the
    /// [`binder`][1] used to allocate this memory.
    ///
    /// [1]: MemoryBinder
    fn is_optimal(&self) -> bool;
}

mod device_obj {

    use super::DeviceMemory;

    leimu_mem::smallbox!(
        pub Obj: DeviceMemory
    );

    unsafe impl<const N_BUF: usize> Send for Obj<N_BUF> {}
    unsafe impl<const N_BUF: usize> Sync for Obj<N_BUF> {}
}

/// A [`DeviceMemory`] trait object.
///
/// Doesn't allocate extra memory with [`alloc`][1], if the underlying memory object is less than
/// or equal to 46 bytes.
///
/// Created with [`leimu_mem::smallbox`].
///
/// [1]: core::alloc
pub type DeviceMemoryObj = device_obj::Obj<46>;

impl DeviceMemoryObj {

    #[inline(always)]
    pub fn overlaps(&self, other: &Self) -> bool {
        if self.handle() == other.handle() {
            let src_off = self.offset();
            let dst_off = other.offset();
            src_off < dst_off + other.size() &&
            dst_off < src_off + self.size()
        } else {
            false
        }
    }
}

#[derive(Clone, Copy)]
pub struct MemoryMap {
    pub map: *mut u8,
    pub size: usize,
    pub is_coherent: bool,
}

unsafe impl Send for MemoryMap {}
unsafe impl Sync for MemoryMap {}

impl MemoryMap {
    
    /// Writes bytes to the mapped memory.
    ///
    /// Panics if the byte count is larger than [`size`][1]
    ///
    /// # Safety
    /// It has to be ensured that the mapping is still valid up to [`size`][1].
    ///
    /// [1]: Self::size
    #[inline]
    pub unsafe fn write_bytes(
        &mut self,
        bytes: &[u8]
    ) {
        assert!(self.size <= bytes.len());
        unsafe {
            bytes.as_ptr().copy_to_nonoverlapping(
                self.map,
                bytes.len()
            );
        }
    }
}

/// A trait for [`vk::DeviceMemory`] allocators.
///
/// # Safety
/// - [`MemoryBinder::alloc`] *should* always allocate memory according to the
///   [`vk::MemoryRequirements2`] requirements.
/// - Memory *should* never be mapped more than once at a time per [`DeviceMemory`] object.
/// - Memory allocated by this trait *should* be valid even after dropping the [`MemoryBinder`].
pub unsafe trait MemoryBinder: 'static + Send + Sync {

    /// Returns the maximum allocation size supported by this allocator.
    fn max_alloc_size(&self) -> vk::DeviceSize;

    /// Returns ['host coherency`][HostCoherency] of optimal allocations.
    fn optimal_host_coherency(&self) -> HostCoherency;

    /// Returns ['host coherency`][HostCoherency] of suboptimal allocations.
    fn suboptimal_host_coherency(&self) -> HostCoherency;

    /// Allocates device memory.
    ///
    /// # Safety
    /// - `memory_requirements` *must* be a valid [`vk::MemoryRequirements2`] structure.
    unsafe fn alloc(
        &self,
        memory_requirements: &vk::MemoryRequirements2,
    ) -> Result<DeviceMemoryObj>;

    /// This *may* release all allocations made by this allocator back to the allocator.
    ///
    /// # Safety
    /// - This *should* never free any [`vk::DeviceMemory`].
    /// - Previous allocations *may* be overwritten by future allocations.
    unsafe fn release_resources(&self);
}

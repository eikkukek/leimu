use tuhka::vk;

use crate::bitflags;

bitflags! {
    /// Specifies a bitmask of memory properties a [`memory binder`][1] uses when selecting
    /// memory types for its allocations.
    /// 
    /// # Vulkan docs
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryPropertyFlagBits.html>
    ///
    /// [1]: super::MemoryBinder
    pub struct MemoryProperties: u32 {
        /// Specifies that memory allocated with this type is the most efficient for [`device`][1] access.
        ///
        /// [1]: Device
        DEVICE_LOCAL = 0x00000001,
        /// Specifies that memory allocated with this type *can* be mapped for host access.
        HOST_VISIBLE = 0x00000002,
        /// Specifies that host cache management commands are not needed to manage availability and
        /// visibility on the host.
        HOST_COHERENT = 0x00000004,
        /// Specifies that memory allocated with this type is cached on the host.
        HOST_CACHED = 0x00000008,
        /// Specifies that the memory type only allows [`device`][1] access to memory.
        ///
        /// Additionally, the object's backing memory *may* be provided by the implementation
        /// lazily.
        ///
        /// [1]: Device
        LAZILY_ALLOCATED = 0x00000010,
    }
}

impl From<MemoryProperties> for vk::MemoryPropertyFlags {
    
    #[inline]
    fn from(value: MemoryProperties) -> Self {
        Self::from_raw(value.as_raw())
    }
}

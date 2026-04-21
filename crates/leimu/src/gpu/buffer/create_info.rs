use core::{
    num::NonZeroU64,
};

use tuhka::vk;

use crate::{
    gpu::prelude::*,
    error,
};

/// The create info structure for buffers. See [`BufferCreateInfo::new`] for full description.
pub struct BufferCreateInfo<'a> {
    pub(crate) out: &'a mut BufferId,
    pub(crate) memory_binder: &'a dyn MemoryBinder,
    pub(crate) size: NonZeroU64,
    pub(crate) usage: BufferUsages,
    pub(crate) create_flags: vk::BufferCreateFlags,
}

impl<'a> BufferCreateInfo<'a> {

    /// Creates new [`BufferCreateInfo`].
    ///
    /// # Parameters
    /// - `out`: A mutable reference to where the [`BufferId`] of the created buffer will be stored.
    /// - `size`: The size of the created buffer, must be non-zero.
    /// - `usage`: Specifies what the buffer *can* be used for.
    /// - `memory_binder`: Specifies what will bind the buffer's memory.
    ///
    /// Returns [`None`] if buffer size is `0`.
    ///
    /// You can specify a different memory binder by [`BufferCreateInfo::with_memory_binder`].
    pub fn new(
        out: &'a mut BufferId,
        memory_binder: &'a dyn MemoryBinder,
        size: DeviceSize,
        usage: BufferUsages,
    ) -> Option<Self> {
        Some(Self {
            out,
            memory_binder,
            size: NonZeroU64::new(size)?,
            usage,
            create_flags: vk::BufferCreateFlags::empty(),
        })
    }

    pub(crate) fn build(
        &self,
        device: Device,
        bind_memory_info: &mut vk::BindBufferMemoryInfo<'static>,
    ) -> error::Result<BufferMeta> {
        BufferMeta::new(device, self, bind_memory_info)
    }
}

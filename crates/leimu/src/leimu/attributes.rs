use core::num::NonZeroU32;

use crate::gpu;

pub struct Attributes {
    pub(crate) desired_buffered_frames: u32,
    pub(crate) gpu_memory_layout: gpu::MemoryLayout,
    pub(crate) gpu_cache_attributes: gpu::CacheAttributes,
}

impl Attributes {

    pub(crate) fn new() -> Self
    {
        Attributes {
            desired_buffered_frames: 3,
            gpu_memory_layout: Default::default(),
            gpu_cache_attributes: Default::default(),
        }
    }

    #[inline]
    pub fn with_gpu_memory_layout(mut self, layout: gpu::MemoryLayout) -> Self {
        self.gpu_memory_layout = layout;
        self
    }

    #[inline]
    pub fn with_desired_buffered_frames(
        mut self,
        frames: NonZeroU32
    ) -> Self {
        self.desired_buffered_frames = frames.get();
        self
    }

    #[inline]
    pub fn with_gpu_cache_attributes(mut self, attributes: gpu::CacheAttributes) -> Self {
        self.gpu_cache_attributes = attributes;
        self
    }
}

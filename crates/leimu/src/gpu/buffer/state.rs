use leimu_mem::{
    alloc::LocalAlloc,
    arena,
    vec::FixedVec32,
};

use tuhka::vk;

use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BufferState {
    pub stage_mask: vk::PipelineStageFlags2,
    pub access_mask: vk::AccessFlags2,
    pub queue_family_index: u32,
}

impl BufferState {

    pub fn new(
        stage_mask: vk::PipelineStageFlags2,
        access_mask: vk::AccessFlags2,
        queue_family_index: u32,
    ) -> Self
    {
        Self {
            access_mask,
            stage_mask,
            queue_family_index,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct BufferMemoryBarrier {
    pub src_stage_mask: vk::PipelineStageFlags2,
    pub src_access_mask: vk::AccessFlags2,
    pub dst_stage_mask: vk::PipelineStageFlags2,
    pub dst_access_mask: vk::AccessFlags2,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub offset: vk::DeviceSize,
    pub size: vk::DeviceSize,
}

impl BufferMemoryBarrier {

    #[inline(always)]
    pub fn into_vk(self, handle: vk::Buffer) -> vk::BufferMemoryBarrier2<'static> {
        let dst_queue_family_index =
            if self.src_queue_family_index == vk::QUEUE_FAMILY_IGNORED {
                vk::QUEUE_FAMILY_IGNORED
            } else {
                self.dst_queue_family_index
            };
        vk::BufferMemoryBarrier2 {
            s_type: vk::StructureType::BUFFER_MEMORY_BARRIER_2,
            src_stage_mask: self.src_stage_mask,
            src_access_mask: self.src_access_mask,
            dst_stage_mask: self.dst_stage_mask,
            dst_access_mask: self.dst_access_mask,
            src_queue_family_index: self.src_queue_family_index,
            dst_queue_family_index,
            buffer: handle,
            offset: self.offset,
            size: self.size,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct BufferRange {
    pub state: BufferState,
    pub offset: vk::DeviceSize,
    pub size: vk::DeviceSize,
}

impl StateRange for BufferRange {

    type MemoryBarrier = BufferMemoryBarrier;
    type State = BufferState;
    type SizeType = vk::DeviceSize;

    #[inline(always)]
    fn new(
        state: Self::State,
        offset: Self::SizeType,
        size: Self::SizeType,
    ) -> Self {
        Self {
            state,
            offset,
            size,
        }
    }

    #[inline(always)]
    fn memory_barrier(
        src: Self::State,
        dst: Self::State,
        offset: Self::SizeType,
        size: Self::SizeType,
    ) -> Self::MemoryBarrier {
        BufferMemoryBarrier {
            src_stage_mask: src.stage_mask,
            src_access_mask: src.access_mask,
            dst_stage_mask: dst.stage_mask,
            dst_access_mask: dst.access_mask,
            src_queue_family_index: src.queue_family_index,
            dst_queue_family_index: dst.queue_family_index,
            offset,
            size,
        }
    }

    #[inline(always)]
    fn offset(&self) -> Self::SizeType {
        self.offset
    }

    #[inline(always)]
    fn size(&self) -> Self::SizeType {
        self.size
    }

    #[inline(always)]
    fn state(&self) -> Self::State {
        self.state
    }
}

#[derive(Default)]
pub struct BufferMemoryBarrierCache {
    pub(super) barriers: Vec32<BufferMemoryBarrier>,
}

#[derive(Default, Clone, Copy)]
pub struct BufferMemoryBarrierRange {
    pub(super) handle: vk::Buffer,
    pub(super) range_start: u32,
    pub(super) range_end: u32,
}

impl BufferMemoryBarrierRange {

    pub fn is_empty(&self) -> bool {
        self.range_start == self.range_end
    }
}

impl BufferMemoryBarrierCache {

    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn flush<'a, Alloc>(
        &mut self,
        ranges: &[BufferMemoryBarrierRange],
        alloc: &'a Alloc,
    ) -> Result<FixedVec32<'a, vk::BufferMemoryBarrier2<'a>, Alloc>>
        where
            Alloc: LocalAlloc<Error = arena::Error>,
    {
        let mut vk_barriers = FixedVec32::with_capacity(
            self.barriers.len(),
            alloc
        ).context("alloc failed")?;
        for barrier in ranges {
            if barrier.is_empty() {
                continue
            }
            let handle = barrier.handle;
            for barrier in &self.barriers[barrier.range_start as usize..barrier.range_end as usize] {
                vk_barriers.push(barrier.into_vk(handle));
            }
        }
        self.barriers.clear();
        Ok(vk_barriers)
    }
}

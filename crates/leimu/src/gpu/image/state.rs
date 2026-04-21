use ahash::AHashMap;

use tuhka::vk;
use leimu_mem::{
    vec::{Vec32, FixedVec32},
    arena,
    vec32,
    alloc::LocalAlloc,
};

use crate::{
    gpu::prelude::{
        subresource_state::*,
        *,
    },
    error::*,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ImageSubresourceState {
    pub stage_mask: vk::PipelineStageFlags2,
    pub access_mask: vk::AccessFlags2,
    pub layout: vk::ImageLayout,
    pub queue_family_index: u32,
}

impl ImageSubresourceState {

    pub fn new(
        stage_mask: vk::PipelineStageFlags2,
        access_mask: vk::AccessFlags2,
        layout: vk::ImageLayout,
        queue_family_index: u32,
    ) -> Self 
    {
        Self {
            stage_mask,
            access_mask,
            layout,
            queue_family_index,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ImageMemoryBarrier {
    pub src_stage_mask: vk::PipelineStageFlags2,
    pub src_access_mask: vk::AccessFlags2,
    pub dst_stage_mask: vk::PipelineStageFlags2,
    pub dst_access_mask: vk::AccessFlags2,
    pub old_layout: vk::ImageLayout,
    pub new_layout: vk::ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub subresource_range: vk::ImageSubresourceRange,
}

impl ImageMemoryBarrier {

    pub fn into_vk(self, image: vk::Image) -> vk::ImageMemoryBarrier2<'static> {
        let dst_queue_family_index = 
            if self.src_queue_family_index == vk::QUEUE_FAMILY_IGNORED {
                vk::QUEUE_FAMILY_IGNORED
            } else {
                self.dst_queue_family_index
            };
        vk::ImageMemoryBarrier2 {
            s_type: vk::StructureType::IMAGE_MEMORY_BARRIER_2,
            src_stage_mask: self.src_stage_mask,
            src_access_mask: self.src_access_mask,
            dst_stage_mask: self.dst_stage_mask,
            dst_access_mask: self.dst_access_mask,
            old_layout: self.old_layout,
            new_layout: self.new_layout,
            src_queue_family_index: self.src_queue_family_index,
            dst_queue_family_index,
            image,
            subresource_range: self.subresource_range,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct ImageLayerRange {
    pub state: ImageSubresourceState,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

impl StateRange for ImageLayerRange {

    type MemoryBarrier = ImageMemoryBarrier;
    type State = ImageSubresourceState;
    type SizeType = u32;

    #[inline(always)]
    fn new(
        state: Self::State,
        offset: Self::SizeType,
        size: Self::SizeType,
    ) -> Self {
        Self {
            state,
            base_array_layer: offset,
            layer_count: size,
        }
    }

    #[inline(always)]
    fn memory_barrier(
        src: Self::State,
        dst: Self::State,
        offset: Self::SizeType,
        size: Self::SizeType,
    ) -> Self::MemoryBarrier {
        ImageMemoryBarrier {
            src_stage_mask: src.stage_mask,
            src_access_mask: src.access_mask,
            dst_stage_mask: dst.stage_mask,
            dst_access_mask: dst.access_mask,
            old_layout: src.layout,
            new_layout: dst.layout,
            src_queue_family_index: src.queue_family_index,
            dst_queue_family_index: dst.queue_family_index,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::empty(),
                base_mip_level: offset, level_count: size,
                base_array_layer: 0, layer_count: 1,
            },
        }
    }

    #[inline(always)]
    fn offset(&self) -> Self::SizeType {
        self.base_array_layer
    }
    
    #[inline(always)]
    fn size(&self) -> Self::SizeType {
        self.layer_count
    }

    #[inline(always)]
    fn state(&self) -> Self::State {
        self.state
    }
}

#[derive(Default)]
pub struct ImageMemoryBarrierCache {
    pub(super) cache: AHashMap<(ImageAspects, u32, u32), Vec32<ImageMemoryBarrier>>,
    pub(super) touched: Vec32<(ImageAspects, u32, u32)>,
    pub(super) barriers: Vec32<ImageMemoryBarrier>,
}

#[derive(Default, Clone, Copy)]
pub struct ImageMemoryBarrierRange {
    pub(super) handle: vk::Image,
    pub(super) range_start: u32,
    pub(super) range_end: u32,
}

impl ImageMemoryBarrierRange {

    pub fn is_empty(&self) -> bool {
        self.range_start == self.range_end
    }
}

impl ImageMemoryBarrierCache {

    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub(super) fn insert(
        &mut self,
        aspect: ImageAspects,
        barrier: ImageMemoryBarrier,
    ) {
        let key = (
            aspect,
            barrier.subresource_range.base_mip_level,
            barrier.subresource_range.level_count,
        );
        self.cache.entry(key)
            .and_modify(|barriers| {
                if barriers.is_empty() {
                    self.touched.push(key);
                }
                barriers.push(barrier);
            }).or_insert_with(|| {
                self.touched.push(key);
                vec32![barrier]
            });
    }

    pub fn flush<'a, Alloc>(
        &mut self,
        ranges: &[ImageMemoryBarrierRange],
        alloc: &'a Alloc,
    ) -> Result<FixedVec32<'a, vk::ImageMemoryBarrier2<'a>, Alloc>>
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

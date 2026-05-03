mod create_info;
mod properties;
mod state;

use tuhka::vk;

use leimu_mem::{
    vec::Vec32,
    vec32,
};

use crate::gpu::prelude::{
    subresource_state::*,
    *,
};

use crate::error::*;

pub use create_info::*;
pub use properties::BufferProperties;
pub use state::*;

impl Flags for BufferUsages {

    const NAME: &str = "buffer usage";
}

pub struct BufferMeta {
    device: Device,
    handle: vk::Buffer,
    memory: DeviceMemoryObj,
    properties: BufferProperties,
    state: Vec32<BufferRange>,
}

impl ResourceMeta for BufferMeta {

    const NAME: &str = "buffer";
}

impl BufferMeta {

    fn new(
        device: Device,
        create_info: &BufferCreateInfo<'_>,
        bind_memory_info: &mut vk::BindBufferMemoryInfo<'static>,
    ) -> Result<Self>
    {
        let properties = BufferProperties {
            size: create_info.size.get(),
            usage: create_info.usage,
            create_flags: create_info.create_flags,
        };
        let vk_create_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BUFFER_CREATE_INFO,
            flags: properties.create_flags,
            size: properties.size,
            usage: properties.usage.into(),
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };
        let device_mem_requirements = vk::DeviceBufferMemoryRequirements {
            s_type: vk::StructureType::DEVICE_BUFFER_MEMORY_REQUIREMENTS,
            p_create_info: &vk_create_info,
            ..Default::default()
        };
        let mut mem_requirements = Default::default();
        unsafe {
            device
            .get_device_buffer_memory_requirements(&device_mem_requirements, &mut mem_requirements);
        }
        let memory = unsafe { create_info.memory_binder.alloc(&mem_requirements)
            .context("failed to allocate GPU memory for buffer")?
        };
        let handle = unsafe {
            device.create_buffer(&vk_create_info, None)
            .context("failed to create Vulkan buffer")?
        }.value;
        *bind_memory_info = vk::BindBufferMemoryInfo {
             buffer: handle,
             memory: vk::DeviceMemory::from_raw(memory.handle()),
             memory_offset: memory.offset(),
             ..Default::default()
        };
        Ok(Self {
            handle,
            memory,
            device,
            properties,
            state: vec32![BufferRange {
                state: BufferState::new(
                    vk::PipelineStageFlags2::NONE,
                    vk::AccessFlags2::NONE,
                    vk::QUEUE_FAMILY_IGNORED,
                ),
                offset: 0,
                size: properties.size,
            }],
        })
    }

    #[inline]
    pub fn handle(&self) -> vk::Buffer {
        self.handle
    }

    #[inline]
    pub fn properties(&self) -> BufferProperties {
        self.properties
    }

    #[inline]
    pub fn validate_usage(
        &self,
        usage: BufferUsages,
    ) -> Option<MissingFlagsError<BufferUsages>> {
        let has = self.properties.usage;
        (!has.contains(usage))
        .then(|| 
            MissingFlagsError::new(usage, has)
        )
    } 

    /// Registers a memory barrier, which *can* be used to perform [`pipeline barrier`][1] with the
    /// [`cache`][2].
    ///
    /// The returned [`range`][3] *must* be [`flushed`][4] and recorded, if the range is not empty.
    ///
    /// # Safety
    /// This does *not* check if `offset` + `size` is in the range of the buffer.
    ///
    /// The range *must* be either checked manually or the [`checked version`][5] of this function
    /// *must* be used.
    ///
    /// [1]: Device::cmd_pipeline_barrier2
    /// [2]: BufferMemoryBarrierCache
    /// [3]: BufferMemoryBarrierRange
    /// [4]: BufferMemoryBarrierCache::flush
    /// [5]: Self::memory_barrier
    pub unsafe fn memory_barrier_unchecked(
        &mut self,
        offset: DeviceSize,
        size: DeviceSize,
        state: BufferState,
        ordering: CommandOrdering,
        cache: &mut BufferMemoryBarrierCache,
    ) -> BufferMemoryBarrierRange
    {
        if size == 0 {
            return Default::default()
        }
        let mut not_inserted = None;
        let mut range = BufferRange {
            state,
            offset,
            size,
        };
        let cache_index = cache.barriers.len();
        for i in (0..self.state.len()).rev() {
            match unsafe { self.state.get_unchecked(i as usize) }.overwrite(&range) {
                StateOverwrite::NoOverlap => continue,
                StateOverwrite::Combine(new_range) => {
                    self.state.remove(i);
                    range = new_range;
                    not_inserted = Some(i);
                },
                StateOverwrite::Consume(barrier) => {
                    self.state.remove(i);
                    match ordering {
                        CommandOrdering::Lenient => {
                            if barrier.src_queue_family_index != barrier.dst_queue_family_index {
                                cache.barriers.push(barrier);
                            }
                        },
                        CommandOrdering::Strict => {
                            cache.barriers.push(barrier);
                        }
                    }
                    not_inserted = Some(i);
                },
                StateOverwrite::Cut(left, right, barrier) => {
                    self.state.remove(i);
                    match ordering {
                        CommandOrdering::Lenient => {
                            if barrier.src_queue_family_index != barrier.dst_queue_family_index {
                                cache.barriers.push(barrier);
                            }
                        },
                        CommandOrdering::Strict => {
                            cache.barriers.push(barrier);
                        }
                    }
                    let mut idx = i;
                    if left.size != 0 {
                        self.state.insert(idx, left);
                        idx += 1;
                    }
                    self.state.insert(idx, range);
                    idx += 1;
                    if right.size != 0 {
                        self.state.insert(idx, right);
                    }
                    not_inserted = None;
                    break
                },
                StateOverwrite::Shrink(new_range, barrier) => {
                    unsafe {
                        *self.state.get_unchecked_mut(i as usize)
                            = new_range;
                    }
                    match ordering {
                        CommandOrdering::Lenient => {
                            if barrier.src_queue_family_index != barrier.dst_queue_family_index {
                                cache.barriers.push(barrier);
                            }
                        },
                        CommandOrdering::Strict => {
                            cache.barriers.push(barrier);
                        }
                    }
                    if new_range.offset < range.offset {
                        self.state.insert(i + 1, range);
                        not_inserted = None;
                        break
                    }
                },
            }
        }
        if let Some(i) = not_inserted {
            self.state.insert(i, range);
        }
        BufferMemoryBarrierRange {
            handle: self.handle,
            range_start: cache_index,
            range_end: cache.barriers.len()
        }
    }

    /// Registers a memory barrier, which *can* be used to perform [`pipeline barrier`][1] with the
    /// [`cache`][2].
    ///
    /// The returned [`range`][3] *must* be [`flushed`][4] and recorded, if the range is not empty.
    ///
    /// # Valid usage
    /// - `offset` + `size` *must* be less than or equal to the size of the buffer.
    ///
    /// [1]: Device::cmd_pipeline_barrier2
    /// [2]: BufferMemoryBarrierCache
    /// [3]: BufferMemoryBarrierRange
    /// [4]: BufferMemoryBarrierCache::flush
    #[inline]
    pub fn memory_barrier(
        &mut self,
        offset: DeviceSize,
        size: Option<NonZeroDeviceSize>,
        state: BufferState,
        ordering: CommandOrdering,
        cache: &mut BufferMemoryBarrierCache,
    ) -> Result<BufferMemoryBarrierRange>
    {
        let size =
            if let Some(size) = size {
                let size = size.get();
                if offset + size > self.properties.size {
                    return Err(Error::just_context(format!(
                        "buffer offset {offset} and size {size} is out of range of buffer size {}",
                        self.properties.size
                    )))
                }
                size
            } else {
                if offset >= self.properties.size {
                    return Err(Error::just_context(format!(
                        "buffer offset {offset} is greater than or equal to buffer size {}",
                        self.properties.size
                    )))
                }
                self.properties.size - offset
            };
        Ok(unsafe { self.memory_barrier_unchecked(
            offset, size, state,
            ordering, cache,
        )})
    }

    #[inline]
    pub fn memory(&self) -> &DeviceMemoryObj {
        &self.memory
    }
    
    #[inline]
    pub fn memory_mut(&mut self) -> &mut DeviceMemoryObj {
        &mut self.memory
    } 

    pub(crate) fn flush_state(&mut self) {
        for range in &mut self.state {
            range.state.stage_mask = vk::PipelineStageFlags2::ALL_COMMANDS;
            range.state.access_mask = vk::AccessFlags2::MEMORY_WRITE;
        }
    }
}

impl Drop for BufferMeta {

    fn drop(&mut self) {
        unsafe {
            self.device.destroy_buffer(self.handle(), None);
        }
    }
}

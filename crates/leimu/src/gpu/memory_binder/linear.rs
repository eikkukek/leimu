use core::ptr;

use leimu_mem::vec::Vec32;
use tuhka::vk::{self, chain_in_iter};

use crate::{
    sync::{*, atomic::{self, AtomicU64}},
};

use super::*;

use MemoryBinderError::{self, *};

struct Allocation {
    device: Device,
    device_memory: core::result::Result<vk::DeviceMemory, vk::Result>,
    mapped_pointer: Option<RwLock<*mut ()>>,
    used: AtomicU64,
}

impl Allocation {

    #[inline(always)]
    fn map(&self) -> Result<*mut u8> {
        let Some(ptr) = &self.mapped_pointer else {
            return Err(Error::just_context(
                "memory is unmappable"
            ))
        };
        let mut ptr = ptr.upgradable_read();
        if ptr.is_null() &&
            let Some(err) = ptr.with_upgraded(|ptr| unsafe
            {
                match self.device.map_memory(
                    self.device_memory.unwrap_unchecked(),
                    0, vk::WHOLE_SIZE,
                    vk::MemoryMapFlags::empty()
                ) {
                    Ok(p) => { *ptr = p.value.cast(); None },
                    Err(err) => Some(err)
                }
            })
        {
            return Err(Error::new(err, "failed to map memory"))
        }
        Ok(ptr.cast())
    }

    #[inline(always)]
    pub fn unmap(&self) -> Result<()> {
        let Some(ptr) = &self.mapped_pointer else {
            return Err(Error::just_context(
                "memory is unmappable"
            ))
        };
        let mut guard = ptr.write();
        let ptr = *guard;
        if ptr.is_null() {
            return Err(Error::just_context(
                "memory is not mapped"
            ))
        }
        unsafe {
            self.device
                .unmap_memory(self.device_memory.unwrap_unchecked());
        }
        *guard = ptr::null_mut();
        Ok(())
    }
}

impl Drop for Allocation {

    fn drop(&mut self) {
        unsafe {
            if let Ok(mem) = self.device_memory {
                self.device
                    .free_memory(mem, None);
            }
        }
    }
}

unsafe impl Send for Allocation {}
unsafe impl Sync for Allocation {}

#[derive(Clone)]
struct Block {
    allocation: OnceLock<Arc<Allocation>>,
}

impl Block {

    #[inline(always)]
    fn new() -> Self {
        Self {
            allocation: OnceLock::new(),
        }
    }
    
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    unsafe fn alloc(
        &self,
        device: &Device,
        memory_requirements: &vk::MemoryRequirements2,
        block_size: DeviceSize,
        memory_type_index: u32,
        granularity: DeviceSize,
        coherency: HostCoherency,
        is_optimal: bool,
    ) -> core::result::Result<Memory, MemoryBinderError>
    {
        let allocation = self.allocation.get_or_init(|| {
            let allocate_info = vk::MemoryAllocateInfo {
                s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
                allocation_size: block_size,
                memory_type_index,
                ..Default::default()
            };
            Arc::new(Allocation {
                device: device.clone(),
                mapped_pointer: (coherency != HostCoherency::None).then(|| RwLock::new(ptr::null_mut())),
                device_memory: unsafe {
                    device.allocate_memory(&allocate_info, None)
                    .map(|suc| suc.value)
                },
                used: AtomicU64::new(0),
            })
        });
        allocation.device_memory?;
        let mut offset = 0;
        let used = allocation.used.fetch_update(
            atomic::Ordering::AcqRel, atomic::Ordering::Acquire,
            |used| {
                let align = memory_requirements.memory_requirements.alignment.max(granularity);
                offset = (used + align - 1) & !(align - 1);
                let end = offset + memory_requirements.memory_requirements.size;
                if block_size < end {
                    return None
                }
                Some(end)
            }
        );
        if used.is_err() {
            return Err(OutOfDeviceMemory {
                size: memory_requirements.memory_requirements.size,
                align: memory_requirements.memory_requirements.alignment,
            })
        }
        Ok(Memory::new(
            allocation.clone(),
            offset,
            block_size,
            memory_requirements.memory_requirements.size,
            coherency,
            is_optimal,
        ))
    }

    #[inline(always)]
    unsafe fn reset(&self) {
        if let Some(allocation) = self.allocation.get() {
            allocation.used.store(0, atomic::Ordering::Release);
        }
    }
}

pub struct LinearBinder {
    device: Device,
    optimal_blocks: SwapLock<Vec32<(Vec32<Block>, u32, usize)>>,
    suboptimal_blocks: SwapLock<Vec32<(Vec32<Block>, u32, usize)>>,
    block_size: vk::DeviceSize,
    fallback: GlobalBinder,
}

impl LinearBinder {

    pub fn new(
        device: Device,
        block_size: DeviceSize,
        optimal_properties: MemoryProperties,
        suboptimal_properties: MemoryProperties,
    ) -> Result<Self>
    {
        let optimal = optimal_properties.into();
        let suboptimal = suboptimal_properties.into();
        let physical_device = device.physical_device();
        let memory_properties = physical_device.memory_properties();
        let mut optimal_blocks = Vec32::with_capacity(4);
        for (i, memory_type) in memory_properties.memory_types[..memory_properties.memory_type_count as usize]
            .iter()
            .enumerate()
        {
            if memory_type.property_flags & optimal == optimal {
                optimal_blocks.push((Vec32::with_len_with(1, |_| Block::new()), i as u32, 0));
            }
        }
        let mut suboptimal_blocks = Vec32::with_capacity(4);
        for (i, memory_type) in memory_properties.memory_types[..memory_properties.memory_type_count as usize]
            .iter()
            .enumerate()
        {
            if memory_type.property_flags.contains(suboptimal) {
                suboptimal_blocks.push((Vec32::with_len_with(1, |_| Block::new()), i as u32, 0));
            }
        }
        Ok(Self {
            fallback: GlobalBinder::new(device.clone(), optimal_properties, suboptimal_properties),
            device,
            optimal_blocks: SwapLock::new(optimal_blocks),
            suboptimal_blocks: SwapLock::new(suboptimal_blocks),
            block_size,
        })
    } 

    #[inline(always)]
    pub fn block_size(&self) -> u64 {
        self.block_size
    }

    unsafe fn reset(&self) {
        unsafe {
            self.optimal_blocks.modify(|blocks| {
                for (blocks, _, i) in blocks.iter_mut() {
                    *i = 0;
                    for block in blocks.iter() {
                        block.reset();
                    }
                }
            });
            self.suboptimal_blocks.modify(|blocks| {
                for (blocks, _, i) in blocks.iter_mut() {
                    *i = 0;
                    for block in blocks.iter() {
                        block.reset();
                    }
                }
            });
        }
    } 
}

struct Memory {
    allocation: Arc<Allocation>,
    offset: DeviceSize,
    size: DeviceSize,
    block_size: DeviceSize,
    coherency: HostCoherency,
    is_optimal: bool,
}

unsafe impl Send for Memory {}
unsafe impl Sync for Memory {}

impl Memory {

    fn new(
        allocation: Arc<Allocation>,
        offset: DeviceSize,
        size: DeviceSize,
        block_size: DeviceSize,
        coherency: HostCoherency,
        is_optimal: bool,
    ) -> Self
    {
        Self {
            allocation,
            offset,
            size,
            block_size,
            coherency,
            is_optimal,
        }
    }
}

unsafe impl DeviceMemory for Memory {

    fn handle(&self) -> u64 {
        unsafe {
            self.allocation.device_memory
                .unwrap_unchecked()
                .as_raw()
        }
    }

    fn memory_size(&self) -> u64 {
        self.block_size
    }

    fn offset(&self) -> DeviceSize {
        self.offset
    }

    fn size(&self) -> DeviceSize {
        self.size
    }

    fn map_memory(&mut self) -> Result<MemoryMap> {
        self.allocation
            .map()
            .map(|ptr| unsafe {
                MemoryMap {
                    map: ptr.add(self.offset as usize),
                    size: self.size as usize,
                    is_coherent: self.coherency == HostCoherency::Coherent,
                }
            })
    }

    fn unmap_memory(&mut self) -> Result<()> {
        self.allocation.unmap()
    } 

    fn is_optimal(&self) -> bool {
        self.is_optimal
    }

    fn is_mapped(&self) -> bool {
        let Some(ptr) = &self.allocation.mapped_pointer else {
            return false
        };
        !ptr.read().is_null()
    }
}

unsafe impl MemoryBinder for LinearBinder {
    
    #[inline]
    fn max_alloc_size(&self) -> vk::DeviceSize {
        self.block_size()
    }
    
    #[inline]
    fn optimal_host_coherency(&self) -> HostCoherency {
        self.fallback.optimal_host_coherency()
    }

    #[inline]
    fn suboptimal_host_coherency(&self) -> HostCoherency {
        self.fallback.suboptimal_host_coherency()
    }

    unsafe fn alloc(
        &self,
        memory_requirements: &vk::MemoryRequirements2,
    ) -> Result<DeviceMemoryObj> {
        let block_size = self.block_size;
        let granularity = self.device
            .physical_device()
            .limits().buffer_image_granularity;
        unsafe {
            if let Some(dedicated_requirements) = chain_in_iter(memory_requirements)
                .find(|_in| (**_in).s_type == vk::MemoryDedicatedRequirements::S_TYPE)
            {
                let dedicated_requirements: *const _ = dedicated_requirements;
                let dedicated_requirements = dedicated_requirements
                    .cast::<vk::MemoryDedicatedRequirements>()
                    .as_ref().unwrap();
                if dedicated_requirements.prefers_dedicated_allocation != 0 {
                    return self.fallback.alloc(memory_requirements)
                }
            }
        }
        let coherency = self.optimal_host_coherency();
        for (i, (blocks, type_index, free_index)) in self.optimal_blocks.load().iter().enumerate() {
            if memory_requirements.memory_requirements.memory_type_bits & (1 << *type_index) != 0 {
                if block_size < memory_requirements.memory_requirements.size {
                    return unsafe {
                        self.fallback.alloc(memory_requirements)
                    };
                }
                let mut res = unsafe { blocks[*free_index]
                    .alloc(
                        &self.device, memory_requirements,
                        self.block_size, *type_index,
                        granularity,
                        coherency,
                        true,
                    )
                };
                if let Err(err) = res {
                    if let OutOfDeviceMemory { size: _, align: _, } = err {
                        let free_index = self.optimal_blocks.modify(|blocks| {
                            let (blocks, _, free_index) = &mut blocks[i];
                            *free_index += 1;
                            if *free_index == blocks.len() as usize {
                                blocks.push(Block::new());
                            }
                            *free_index
                        });
                        res = unsafe { self.optimal_blocks.load()[i].0[free_index].alloc(
                            &self.device, memory_requirements, block_size,
                            *type_index, granularity,
                            coherency, true,
                        ) };
                    } else {
                        return Err(Error::new(err, "failed to allocate device memory"))
                    }
                }
                return Ok(DeviceMemoryObj::new(res.context("failed to allocate device memory")?))
            }
        }
        let coherency = self.suboptimal_host_coherency();
        for (i, (blocks, type_index, free_index)) in self.suboptimal_blocks.load().iter().enumerate() {
            if memory_requirements.memory_requirements.memory_type_bits & (1 << *type_index) != 0 {
                if block_size < memory_requirements.memory_requirements.size {
                    return unsafe {
                        self.fallback.alloc(memory_requirements)
                    };
                }
                let mut res = unsafe { blocks[*free_index]
                    .alloc(
                        &self.device, memory_requirements,
                        self.block_size, *type_index,
                        granularity,
                        coherency,
                        false,
                    )
                };
                if let Err(err) = res {
                    if let OutOfDeviceMemory { size: _, align: _, } = err {
                        let free_index = self.suboptimal_blocks.modify(|blocks| {
                            let (blocks, _, free_index) = &mut blocks[i];
                            *free_index += 1;
                            if *free_index == blocks.len() as usize {
                                blocks.push(Block::new());
                            }
                            *free_index
                        });
                        res = unsafe { self.suboptimal_blocks.load()[i].0[free_index].alloc(
                            &self.device, memory_requirements, block_size,
                            *type_index, granularity,
                            coherency, true,
                        ) };
                    } else {
                        return Err(Error::new(err, "failed to allocate device memory"))
                    }
                }
                return Ok(DeviceMemoryObj::new(res.context("failed to allocate device memory")?))
            }
        }
        Err(Error::just_context(IncompatibleMemoryRequirements))
    }

    #[inline(always)]
    unsafe fn release_resources(&self) {
        unsafe {
            self.reset();
        }
    }
}

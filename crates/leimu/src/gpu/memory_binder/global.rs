use tuhka::vk;

use crate::gpu::prelude::*;

use super::*;

use MemoryBinderError::*;

#[derive(Clone)]
pub struct GlobalBinder {
    device: Device,
    optimal_memory_type_bits: u32,
    suboptimal_memory_type_bits: u32,
    optimal_host_coherency: HostCoherency,
    suboptimal_host_coherency: HostCoherency,
}

impl GlobalBinder {

    /// Creates a new global binder, which always allocates new [`device memory`][1].
    ///
    /// # Parameters
    /// - `optimal`: optimal memory properties
    /// - `suboptimal`: suboptimal memory properties
    pub fn new(
        device: Device,
        optimal_properties: MemoryProperties,
        suboptimal_properties: MemoryProperties,
    ) -> Self
    {
        let optimal = optimal_properties.into();
        let suboptimal = suboptimal_properties.into();
        let memory_properties = device.physical_device().memory_properties();
        let mut optimal_memory_type_bits = 0;
        for (i, memory_type) in memory_properties.memory_types[..memory_properties.memory_type_count as usize]
            .iter()
            .enumerate()
        {
            if memory_type.property_flags & optimal == optimal {
                optimal_memory_type_bits |= 1 << i;
            }
        }
        let mut suboptimal_memory_type_bits = 0;
        for (i, memory_type) in memory_properties.memory_types[..memory_properties.memory_type_count as usize]
            .iter()
            .enumerate()
        {
            if memory_type.property_flags.contains(suboptimal) {
                suboptimal_memory_type_bits |= 1 << i;
            }
        }
        Self {
            device,
            optimal_memory_type_bits,
            suboptimal_memory_type_bits,
            optimal_host_coherency:
                if optimal_properties.contains(MemoryProperties::HOST_COHERENT) {
                    HostCoherency::Coherent
                } else if optimal_properties.contains(MemoryProperties::HOST_VISIBLE) {
                    HostCoherency::Mappable
                } else {
                    HostCoherency::None
                },
            suboptimal_host_coherency:
                if suboptimal_properties.contains(MemoryProperties::HOST_COHERENT) {
                    HostCoherency::Coherent
                } else if suboptimal_properties.contains(MemoryProperties::HOST_VISIBLE) {
                    HostCoherency::Mappable
                } else {
                    HostCoherency::None
                },
        }
    }
}

pub struct Memory {
    device: Device,
    memory: vk::DeviceMemory,
    size: DeviceSize,
    map: *mut u8,
    host_coherency: HostCoherency,
    is_optimal: bool,
}

unsafe impl Send for Memory {}
unsafe impl Sync for Memory {}

unsafe impl DeviceMemory for Memory {

    fn handle(&self) -> u64 {
        self.memory.as_raw()
    }

    fn memory_size(&self) -> u64 {
        self.size
    }

    fn offset(&self) -> vk::DeviceSize {
        0
    }
    
    fn size(&self) -> vk::DeviceSize {
        self.size
    }

    fn map_memory(&mut self) -> Result<MemoryMap> {
        if self.host_coherency == HostCoherency::None {
            return Err(Error::just_context(UnmappableMemory));
        }
        if !self.map.is_null() {
            return Ok(MemoryMap {
                map: self.map,
                size: self.size as usize,
                is_coherent: self.host_coherency == HostCoherency::Coherent,
            })
        }
        let ptr = unsafe {
            self.device.map_memory(
                self.memory,
                0,
                vk::WHOLE_SIZE,
                vk::MemoryMapFlags::from_raw(0)
            )
        }.context("failed to map memory")?.value;
        self.map = ptr as *mut u8;
        Ok(MemoryMap {
            map: self.map,
            size: self.size as usize,
            is_coherent: self.host_coherency == HostCoherency::Coherent,
        })
    }

    fn unmap_memory(&mut self) -> Result<()> {
        if self.map.is_null() {
            return Err(Error::just_context(
                "memory is not mapped"
            ))
        }
        self.map = core::ptr::null_mut();
        unsafe {
            self.device.unmap_memory(self.memory);
        }
        Ok(())
    }

    fn is_optimal(&self) -> bool {
        self.is_optimal
    }

    fn is_mapped(&self) -> bool {
        !self.map.is_null()
    }
}

impl Drop for Memory {

    fn drop(&mut self) {
        unsafe {
            self.device
                .free_memory(self.memory, None);
        }
    }
}

unsafe impl MemoryBinder for GlobalBinder {

    #[inline]
    fn max_alloc_size(&self) -> vk::DeviceSize {
        self.device.max_memory_allocation_size()
    }

    #[inline]
    fn optimal_host_coherency(&self) -> HostCoherency {
        self.optimal_host_coherency
    }

    #[inline]
    fn suboptimal_host_coherency(&self) -> HostCoherency {
        self.suboptimal_host_coherency
    }

    unsafe fn alloc(
        &self,
        memory_requirements: &vk::MemoryRequirements2,
    ) -> Result<DeviceMemoryObj> {
        let mut is_optimal = true;
        let mut host_coherency = self.optimal_host_coherency;
        let mut memory_type_bits = self.optimal_memory_type_bits &
            memory_requirements.memory_requirements.memory_type_bits;
        if memory_type_bits == 0 {
            memory_type_bits = self.suboptimal_memory_type_bits &
                memory_requirements.memory_requirements.memory_type_bits;
            if memory_type_bits == 0 {
                return Err(Error::just_context(IncompatibleMemoryRequirements))
            }
            host_coherency = self.suboptimal_host_coherency;
            is_optimal = false;
        }
        let memory_type_index = memory_type_bits.trailing_zeros();
        let allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
            allocation_size: memory_requirements.memory_requirements.size,
            memory_type_index,
            ..Default::default()
        };
        let memory = unsafe {
            self.device.allocate_memory(&allocate_info, None)
        }.context("failed to allocate device memory")?.value;
        Ok(DeviceMemoryObj::new(Memory {
            device: self.device.clone(),
            memory,
            size: memory_requirements.memory_requirements.size,
            map: core::ptr::null_mut(),
            host_coherency,
            is_optimal,
        }))
    }

    #[inline]
    unsafe fn release_resources(&self) {}
}

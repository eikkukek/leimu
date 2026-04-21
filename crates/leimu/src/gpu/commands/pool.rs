use leimu_mem::{
    vec::Vec32,
    vec32,
};
use tuhka::vk;

use crate::{
    gpu::prelude::*,
    error::*,
    sync::Arc,
};

struct Inner {
    device: Device,
    handle: vk::CommandPool,
}

#[derive(Clone)]
pub struct CommandPoolHandle {
    inner: Arc<Inner>,
}

pub struct CommandPool {
    handle: CommandPoolHandle,
    queue: DeviceQueue,
    primaries: Vec32<CommandBuffer>,
    next_primary: u32,
    secondaries: Vec32<CommandBuffer>,
    next_secondary: u32,
}

impl CommandPool {

    /// Creates a new command pool.
    pub fn new(
        device: Device,
        queue: DeviceQueue,
        is_transient: bool,
    ) -> Result<Self>
    {
        let create_info = vk::CommandPoolCreateInfo {
            flags:
                if is_transient {
                    vk::CommandPoolCreateFlags::TRANSIENT
                } else { vk::CommandPoolCreateFlags::empty() },
            queue_family_index: queue.family_index(),
            ..Default::default()
        };
        let handle = unsafe {
            device.create_command_pool(&create_info, None)
        }.context("failed to create command pool")?.value;
        Ok(Self {
            handle: CommandPoolHandle {
                inner: Arc::new(Inner {
                    device,
                    handle,
                })
            },
            queue,
            primaries: vec32![],
            next_primary: 0,
            secondaries: vec32![],
            next_secondary: 0,
        })
    }

    #[inline]
    pub fn queue(&self) -> &DeviceQueue {
        &self.queue
    }

    #[inline]
    pub fn handle(&self) -> &CommandPoolHandle {
        &self.handle
    }

    /// Allocates primary command buffers.
    pub fn allocate_primaries(
        &mut self,
        count: u32,
    ) -> Result<&[CommandBuffer]> {
        let new_next_primary = self.next_primary + count;
        if new_next_primary > self.primaries.len() {
            let old_n = self.primaries.len();
            let new_n = (old_n + count).next_power_of_two();
            self.primaries.resize(new_n, Default::default());
            let new_buffers = &mut self.primaries[old_n as usize..new_n as usize];
            let n_alloc = new_buffers.len() as u32;
            let alloc_info = vk::CommandBufferAllocateInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                command_pool: self.handle.inner.handle,
                level: vk::CommandBufferLevel::PRIMARY,
                command_buffer_count: n_alloc,
                ..Default::default()
            };
            unsafe {
                self.handle.inner.device.allocate_command_buffers(
                    &alloc_info,
                    new_buffers
                ).context("failed to allocate command buffers")?;
            }
        }
        let buffers = &self.primaries[self.next_primary as usize..new_next_primary as usize];
        self.next_primary = new_next_primary;
        Ok(buffers)
    }

    /// Allocates secondary command buffers.
    pub fn allocate_secondaries(
        &mut self,
        count: u32,
    ) -> Result<&[CommandBuffer]> {
        let new_next_secondary = self.next_secondary + count;
        if new_next_secondary > self.secondaries.len() {
            let old_n = self.secondaries.len();
            let new_n = (old_n + count).next_power_of_two();
            self.secondaries.resize(new_n, Default::default());
            let new_buffers = &mut self.secondaries[old_n as usize..new_n as usize];
            let n_alloc = new_buffers.len() as u32;
            let alloc_info = vk::CommandBufferAllocateInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                command_pool: self.handle.inner.handle,
                level: vk::CommandBufferLevel::PRIMARY,
                command_buffer_count: n_alloc,
                ..Default::default()
            };
            unsafe {
                self.handle.inner.device.allocate_command_buffers(
                    &alloc_info,
                    new_buffers
                ).context("failed to allocate command buffers")?;
            }
        }
        let buffers = &self.secondaries[self.next_secondary as usize..new_next_secondary as usize];
        self.next_secondary = new_next_secondary;
        Ok(buffers)
    }

    /// Resets the entire command pool, resetting all allocated command buffers to their initial
    /// state.
    ///
    /// # Safety
    /// - All allocated command buffers are reset and they *can* be overwritten by future
    ///   allocations.
    pub unsafe fn reset(&mut self, device: &Device) -> Result<()> {
        unsafe {
            device.reset_command_pool(
                self.handle.inner.handle, vk::CommandPoolResetFlags::empty(),
            ).context("failed to reset command pool")?;
            self.next_primary = 0;
            self.next_secondary = 0;
        }
        Ok(())
    }
}

impl Drop for CommandPoolHandle {

    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.inner.device.destroy_command_pool(
                self.inner.handle, None
            );
        }
    }
}

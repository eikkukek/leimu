use leimu::{
    EventError, EventResult,
    core::slice, default, gpu::{self, MemoryBinder},
    mem::align_up_u64,
    sync::{Arc, RwLock, atomic::{self, AtomicU64}},
};

#[derive(Clone, Copy)]
struct BufferToBuffer {
    host_buffer: gpu::BufferId,
    gpu_buffer: gpu::BufferId,
    gpu_offset: u64,
    copy_size: u64,
}

struct Inner {
    gpu: gpu::Gpu,
    staging_binder: gpu::LinearBinder,
    frame: AtomicU64,
    buffer_to_buffer: Vec<(u64, BufferToBuffer)>,
}

#[derive(Clone)]
pub struct Scheduler {
    inner: Arc<RwLock<Inner>>,
}

impl Scheduler {

    pub fn new(
        gpu: gpu::Gpu,
        first_frame: u64,
    ) -> EventResult<Self> {
        Ok(Self { inner: Arc::new(RwLock::new(Inner {
            staging_binder: gpu::LinearBinder::new(
                gpu.device().clone(),
                1048576,
                gpu::MemoryProperties::HOST_VISIBLE,
                gpu::MemoryProperties::HOST_VISIBLE | gpu::MemoryProperties::HOST_COHERENT
            )?,
            frame: AtomicU64::new(first_frame),
            buffer_to_buffer: vec![],
            gpu,
        }))})
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.read().buffer_to_buffer.is_empty()
    }

    #[inline]
    pub fn set_frame(&self, frame: u64) {
        self.inner
            .read().frame
            .store(frame, atomic::Ordering::Release);
    }

    pub fn buffer_to_buffer<T: Copy>(
        &self,
        dst_buffer_id: gpu::BufferId,
        dst_offset: u64,
        data: &[T],
    ) -> EventResult<()> {
        let mut inner = self.inner.write();
        let mut staging = default();
        let n_bytes = size_of_val(data) as u64;
        let align = inner.gpu.device_limits().non_coherent_atom_size();
        let aligned = align_up_u64(n_bytes, align);
        inner.gpu.create_resources([gpu::BufferCreateInfo::new(
            &mut staging,
            &inner.staging_binder,
            aligned,
            gpu::BufferUsages::TRANSFER_SRC
        ).ok_or_else(|| EventError::just_context("no data provided"))?], [])?;
        let mut map = inner.gpu.map_buffer(staging)?;
        unsafe {
            map.write_bytes(0, slice::as_bytes(data));
        }
        if !map.is_coherent {
            inner.gpu.flush_mapped_memory_ranges(&[gpu::MappedBufferMemoryRange::new(
                staging,
                0,
                aligned,
            )])?;
        }
        let inner = &mut *inner;
        inner.buffer_to_buffer.push((
            inner.frame.load(atomic::Ordering::Acquire),
            BufferToBuffer {
                host_buffer: staging,
                gpu_buffer: dst_buffer_id,
                gpu_offset: dst_offset,
                copy_size: n_bytes,
            }
        ));
        Ok(())
    }

    pub fn record_copies(
        &self,
        cmd: &mut gpu::CopyCommands<'_, '_>,
    ) -> EventResult<()> {
        let inner = self.inner.read();
        let current_frame = inner.frame.load(atomic::Ordering::Acquire);
        for &(frame, BufferToBuffer { host_buffer, gpu_buffer, copy_size, gpu_offset })
            in &inner.buffer_to_buffer 
        {
            if frame != current_frame { continue; }
            cmd.copy_buffer(
                host_buffer, gpu_buffer,
                &[gpu::BufferCopy::new(0, gpu_offset, copy_size)],
                gpu::CommandOrdering::Lenient
            )?;
        }
        Ok(())
    }

    /// # Safety
    /// All copy command must have finished on the gpu.
    pub unsafe fn flush(
        &self,
        finished_frame: u64,
    ) -> EventResult<()> {
        let mut inner = self.inner.write();
        let mut delete = vec![];
        inner
            .buffer_to_buffer
            .retain(|&(frame, buffer)| {
                if frame <= finished_frame {
                    delete.push(buffer.host_buffer);
                    false
                } else {
                    true
                }
            });
        inner.gpu.destroy_resources(delete, [])?;
        unsafe {
            inner.staging_binder.release_resources();
        }
        Ok(())
    }
}

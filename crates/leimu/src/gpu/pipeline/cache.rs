use std::sync::Arc;

use tuhka::vk;

use crate::error::{Result, Context};

use crate::gpu::prelude::*;

struct Inner {
    device: Device,
    handle: vk::PipelineCache,
}

#[derive(Clone)]
pub struct PipelineCache {
    inner: Arc<Inner>,
}

impl PipelineCache {

    pub fn new(
        device: Device,
        initial_data: Option<&[u8]>,
    ) -> Result<Self> {
        let initial_data = initial_data.unwrap_or(&[]);
        let info = vk::PipelineCacheCreateInfo {
            s_type: vk::StructureType::PIPELINE_CACHE_CREATE_INFO,
            initial_data_size: initial_data.len(),
            p_initial_data: initial_data.as_ptr() as _,
            ..Default::default()
        };
        let handle = unsafe {
            device.create_pipeline_cache(&info, None)
                .context("failed to create pipeline cache")?
        }.value;
        Ok(Self {
            inner: Arc::new(Inner { device, handle })
        })
    }

    #[inline]
    pub fn handle(&self) -> TransientHandle<'_, vk::PipelineCache> {
        TransientHandle::new(self.inner.handle)
    }

    #[inline]
    pub fn device_id(&self) -> DeviceId {
        self.inner.device.id()
    }

    #[inline]
    pub fn retrieve_data(
        &self,
    ) -> Result<Box<[u8]>>
    {
        unsafe {
            let mut data: Box<[u8]> = (0..self.inner.device
                .get_pipeline_cache_data_len(self.inner.handle)
                .context("failed to get pipeline cache data")?
                .value
            ).map(|_| 0).collect();
            self.inner.device.get_pipeline_cache_data(
                self.inner.handle,
                &mut data,
            ).context("failed to get pipeline cache data")?;
            Ok(data)
        }
    }
}

impl Drop for Inner {
    
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_pipeline_cache(self.handle, None);
        }
    }
}

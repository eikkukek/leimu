use core::hash::{self, Hash};

use tuhka::vk;

use crate::{
    gpu::prelude::*,
    sync::Arc,
};

pub(super) struct Inner {
    pub device: Device,
    pub handle: vk::Pipeline,
    pub shader_set: ShaderSet,
}

impl Drop for Inner {

    fn drop(&mut self) {
        unsafe {
            self.device.destroy_pipeline(self.handle, None);
        }
    }
}

#[derive(Clone)]
pub struct PipelineHandle {
    inner: Arc<Inner>,
}

impl PipelineHandle {

    /// Creates a [`PipelineHandle`] from raw parts.
    ///
    /// # Safety
    /// `handle` *must* be a valid pipeline handle.
    #[inline(always)]
    pub unsafe fn new(
        device: Device,
        handle: vk::Pipeline,
        shader_set: ShaderSet,
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                device,
                handle,
                shader_set,
            }),
        }
    }

    #[inline(always)]
    pub fn handle(&self) -> vk::Pipeline {
        self.inner.handle
    }

    #[inline(always)]
    pub fn shader_set(&self) -> &ShaderSet {
        &self.inner.shader_set
    }
}

impl PartialEq for PipelineHandle {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for PipelineHandle {}

impl Hash for PipelineHandle {

    #[inline(always)]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.inner).hash(state);
    }
}

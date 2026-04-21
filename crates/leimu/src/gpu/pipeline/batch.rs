use core::{
    ops::Deref,
    fmt::{self, Display},
};

use tuhka::vk;
use leimu_mem::{
    vec::{Vec32, FixedVec32},
    vec32,
    slot_map::SlotIndex,
};
use leimu_threads::{
    futures::future::RemoteHandle,
    executor::SpawnExt,
    sync::{FutureLock, SwapLock},
};

use crate::{
    gpu::prelude::*,
    error::*,
    sync::*,
    log,
};

macro_rules! impl_id_display {
    ($name:ident) => {
        impl Display for $name {

            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

/// An identifier for a pipeline batch.
///
/// This *can* be used to [`destroy an entire pipeline batch`][1] at once.
///
/// You can get the id [`when building a pipeline batch`][2].
///
/// [1]: Gpu::destroy_pipeline_batch
/// [2]: PipelineBatchBuilder::id
#[must_use]
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PipelineBatchId(SlotIndex<OnceLock<PipelineBatch>>);

impl_id_display!(PipelineBatchId);

impl PipelineBatchId {

    #[inline(always)]
    pub(crate) fn new(slot_index: SlotIndex<OnceLock<PipelineBatch>>) -> Self {
        Self(slot_index)
    }

    #[inline(always)]
    pub(crate) fn slot_index(self) -> SlotIndex<OnceLock<PipelineBatch>> {
        self.0
    }
}

/// An identifier for a [`graphics pipeline`][1].
///
/// [1]: GraphicsPipeline
#[must_use]
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct GraphicsPipelineId(PipelineBatchId, u32);

impl Display for GraphicsPipelineId {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.0, self.1)
    }
}

impl GraphicsPipelineId {
   
    /// Gets the batch portion of the id.
    #[inline(always)]
    pub fn batch_id(self) -> PipelineBatchId {
        self.0
    }

    /// Gets the pipeline portion of the id.
    #[inline(always)]
    pub fn pipeline_id(self) -> u32 {
        self.1
    }
}

/// An identifier for a [`compute pipeline`][1].
///
/// [1]: ComputePipeline
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ComputePipelineId(PipelineBatchId, u32);

impl Display for ComputePipelineId {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.0, self.1)
    }
}

impl ComputePipelineId {
   
    /// Gets the batch portion of the id.
    #[inline(always)]
    pub fn batch_id(self) -> PipelineBatchId {
        self.0
    }

    /// Gets the pipeline portion of the id.
    #[inline(always)]
    pub fn pipeline_id(self) -> u32 {
        self.1
    }
}

pub(crate) struct PipelineBatchInner {
    id: PipelineBatchId,
    graphics_pipelines: SwapLock<Vec32<Option<GraphicsPipeline>>>,
    compute_pipelines: SwapLock<Vec32<Option<ComputePipeline>>>,
}

impl PipelineBatchInner {

    fn new(
        id: PipelineBatchId,
        graphics_pipelines: Vec32<Option<GraphicsPipeline>>,
        compute_pipelines: Vec32<Option<ComputePipeline>>
    ) -> Self {
        Self {
            id,
            graphics_pipelines: SwapLock::new(graphics_pipelines),
            compute_pipelines: SwapLock::new(compute_pipelines),
        }
    }
}

/// Contains the handle of a pipeline batch, which contains the pipelines and metadata about the
/// batch.
///
/// This is [`Clone`], [`Send`] and [`Sync`].
#[derive(Clone)]
pub struct PipelineBatch {
    inner: Arc<FutureLock<PipelineBatchInner, RemoteHandle<Result<PipelineBatchInner>>>>,
}

impl PipelineBatch {

    #[inline(always)]
    fn new(f: RemoteHandle<Result<PipelineBatchInner>>) -> Self {
        Self { inner: Arc::new(FutureLock::new(f)) }
    }

    #[inline(always)]
    pub(crate) async fn get_graphics_pipeline<'a>(
        &self,
        idx: u32,
    ) -> Result<impl Deref<Target = GraphicsPipeline> + use<'a>>
    {
        self.inner
            .load().await?.graphics_pipelines
            .load()
            .try_map(|pipelines| {
                pipelines
                    .get(idx as usize)
                    .ok_or_else(|| Error::just_context("invalid id"))?
                    .as_ref()
                    .ok_or_else(|| Error::just_context("graphics pipeline destroyed"))
            })
    }

    #[inline(always)]
    pub(crate) async fn get_compute_pipeline<'a>(
        &self,
        idx: u32,
    ) -> Result<impl Deref<Target = ComputePipeline> + use<'a>>
    {
        self.inner
            .load().await?.compute_pipelines
            .load()
            .try_map(|pipelines| {
                pipelines
                    .get(idx as usize)
                    .ok_or_else(|| Error::just_context("invalid id"))?
                    .as_ref()
                    .ok_or_else(|| Error::just_context("compute pipeline is destroyed"))
            })
            
    }

    pub(crate) async fn destroy_graphics_pipelines(
        &self,
        ids: impl ExactSizeIterator<Item = GraphicsPipelineId>
    ) -> Result<()>
    {
        if ids.len() == 0 { return Ok(()) }
        let inner = self.inner.load().await?;
        let batch_id = inner.id;
        inner.graphics_pipelines
            .modify(|pipelines| {
                for id in ids {
                    if id.batch_id() != batch_id {
                        return Err(Error::just_context(format!(
                            "graphics pipeline id {id} batch id is different from this batch id {batch_id}",
                        )))
                    }
                    pipelines
                        .get_mut(id.1 as usize)
                        .ok_or_else(|| Error::just_context(format!(
                            "invalid graphics pipeline id {id}"
                        )))?.take();
                }
                Ok(())
            })?;
        Ok(())
    }

    pub(crate) async fn destroy_compute_pipelines(
        &self,
        ids: impl ExactSizeIterator<Item = ComputePipelineId>
    ) -> Result<()>
    {
        if ids.len() == 0 { return Ok(()) }
        let inner = self.inner.load().await?;
        let batch_id = inner.id;
        inner.compute_pipelines
            .modify(|pipelines| {
                for id in ids {
                    if id.batch_id() != batch_id {
                        return Err(Error::just_context(format!(
                            "compute pipeline id {id} batch id is different from this batch id {batch_id}",
                        )))
                    }
                    pipelines
                        .get_mut(id.1 as usize)
                        .ok_or_else(|| Error::just_context(format!(
                            "invalid compute pipeline id {id}"
                        )))?.take();
                }
                Ok(())
            })?;
        Ok(())
    }
}

pub struct PipelineBatchBuilder {
    this_id: PipelineBatchId,
    gpu: Gpu,
    graphics_create_infos: Option<Vec32<GraphicsPipelineCreateTemplate>>,
    compute_create_infos: Option<Vec32<ComputePipelineCreateTemplate>>,
    cache: Option<PipelineCache>,
    built: bool,
}

impl PipelineBatchBuilder {

    #[inline(always)]
    pub(crate) fn new(
        gpu: Gpu,
        cache: Option<PipelineCache>,
    ) -> Self
    {
        Self {
            this_id: gpu.reserve_pipeline_batch_slot(),
            gpu,
            graphics_create_infos: Some(vec32![]),
            compute_create_infos: Some(vec32![]),
            cache,
            built: false,
        }
    }

    /// Appends [`GraphicsPipelineCreateInfo`]s to the batch.
    ///
    /// [`GraphicsPipelineId`]s are returned to as described in [`GraphicsPipelineCreateInfo`].
    ///
    /// # Valid usage
    /// - Each create info *must* follow the valid usage described in
    ///   [`GraphicsPipelineCreateInfo`].
    #[inline(always)]
    pub fn with_graphics_pipelines<'a, I>(
        &mut self,
        create_infos: I,
    ) -> &mut Self
        where I: IntoIterator<Item = GraphicsPipelineCreateInfo<'a>>
    {
        let infos = self.graphics_create_infos.as_mut().unwrap();
        let mut id = infos.len();
        let batch_id = self.this_id;
        infos.extend(create_infos
            .into_iter()
            .map(|info| {
                *info.meta = GraphicsPipelineId(batch_id, id);
                id += 1;
                info.into_template()
            })
        );
        self
    }

    /// Appends [`ComputePipelineCreateInfo`]s to the batch.
    ///
    /// [`ComputePipelineId`]s are returned to as described in [`ComputePipelineCreateInfo`].
    ///
    /// # Valid usage
    /// - Each create info *must* follow the valid usage described in
    ///   [`ComputePipelineCreateInfo`].
    #[inline(always)]
    pub fn with_compute_pipelines<'a, I>(
        &mut self,
        create_infos: I,
    ) -> &mut Self
        where I: IntoIterator<Item = ComputePipelineCreateInfo<'a>>
    {
        let infos = self.compute_create_infos.as_mut().unwrap();
        let mut id = infos.len();
        let batch_id = self.this_id;
        infos.extend(create_infos
            .into_iter()
            .map(|info| {
                *info.meta = ComputePipelineId(batch_id, id);
                id += 1;
                info.into_template()
            })
        );
        self
    }
    
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.graphics_create_infos
            .as_ref()
            .map(|g| g.is_empty())
            .unwrap() &&
        self.compute_create_infos
            .as_ref()
            .map(|c| c.is_empty())
            .unwrap()
    }

    #[inline]
    pub fn discard(mut self) {
        self.built = true;
        self.gpu.discard_pipeline_batch(self.this_id);
    }

    /// You should always call this once you have finished building.
    ///
    /// Unbuilt pipeline batches are built on [`Drop`], but you should *not* rely on that.
    #[inline(always)]
    pub fn build(mut self) -> Result<PipelineBatchId> {
        self.built = true;
        let thread_pool = self.gpu.thread_pool().clone();
        let gpu = self.gpu.clone();
        let create_infos = self.graphics_create_infos.take().unwrap();
        let cache = self.cache.clone();
        let this_id = self.this_id;
        let graphics = thread_pool.spawn_with_handle(async move {
            let tmp_alloc = gpu.tmp_alloc();
            let pipeline_count = create_infos.len();
            if pipeline_count == 0 {
                return Result::Ok(Default::default())
            }
            let mut prepared_create_infos = FixedVec32
                ::with_capacity(pipeline_count, &tmp_alloc)
                .context("alloc failed")?;
            for info in &create_infos {
                prepared_create_infos.push(info.prepare(&gpu, &tmp_alloc)
                    .await
                    .context("failed to convert graphics pipeline info")?
                );
            }
            let mut vk_infos = FixedVec32
                ::with_capacity(pipeline_count, &tmp_alloc)
                .context("alloc failed")?;
            for (info, _) in &mut prepared_create_infos {
                let mut vk_info = info.as_create_info();
                info.rendering_info.p_next = &info.robustness_info as *const _ as *const core::ffi::c_void;
                vk_info.p_next = &info.rendering_info as *const _ as *const core::ffi::c_void;
                vk_infos.push(vk_info);
            }
            let mut pipelines = FixedVec32
                ::with_len(create_infos.len(), vk::Pipeline::null(), &tmp_alloc)
                .context("alloc failed")?;
            unsafe {
                let device = gpu.device();
                let pipeline_cache = cache 
                    .map(|cache| cache.handle().into_inner())
                    .unwrap_or(vk::PipelineCache::null());
                device.create_graphics_pipelines(
                    pipeline_cache,
                    &vk_infos,
                    None,
                    &mut pipelines,
                ).context_with(|| "failed to create graphics pipelines")?;
            }
            let graphics_pipelines: Vec32<_> = pipelines
                .iter()
                .copied()
                .enumerate()
                .map(|(i, handle)| unsafe {
                    Some(GraphicsPipeline::new(
                        gpu.device().clone(),
                        handle,
                        prepared_create_infos[i].1.clone(),
                        &create_infos[i]
                    ))
                }).collect();
            unsafe {
                tmp_alloc.clear();
            }
            Ok(graphics_pipelines)
        }).context("send error")?;
        let gpu = self.gpu.clone();
        let mut create_infos = self.compute_create_infos.take().unwrap();
        let cache = self.cache.clone();
        let compute = thread_pool.spawn_with_handle(async move {
            let tmp_alloc = gpu.tmp_alloc();
            let pipeline_count = create_infos.len();
            if pipeline_count == 0 {
                return Result::Ok(Default::default())
            }
            let mut vk_infos = FixedVec32
                ::with_capacity(pipeline_count, &tmp_alloc)
                .context("alloc failed")?;
            let mut shader_sets = FixedVec32
                ::with_capacity(pipeline_count, &tmp_alloc)
                .context("alloc failed")?;
            for info in &mut create_infos {
                let (vk_info, shader_set) = info
                    .prepare(&gpu)
                    .await
                    .context("failed to convert compute pipeline info")?;
                shader_sets.push(shader_set);
                vk_infos.push(vk_info);
            }
            let mut pipelines = FixedVec32
                ::with_len(vk_infos.len(), vk::Pipeline::null(), &tmp_alloc)
                .context("alloc failed")?;
            unsafe {
                let device = gpu.device();
                let pipeline_cache = cache
                    .map(|cache| cache.handle().into_inner())
                    .unwrap_or(vk::PipelineCache::null());
                device.create_compute_pipelines(
                    pipeline_cache,
                    &vk_infos,
                    None,
                    &mut pipelines
                ).context("failed to create compute pipelines")?;
            };
            let compute_pipelines: Vec32<_> =
                shader_sets 
                    .into_iter()
                    .enumerate()
                    .map(|(i, shader_set)| {
                        Some(unsafe { ComputePipeline::new(
                            gpu.device().clone(),
                            pipelines[i],
                            shader_set,
                        )})
                    }).collect();
            unsafe {
                tmp_alloc.clear();
            }
            Ok(compute_pipelines)
        }).context("send error")?;
        let fut = thread_pool.spawn_with_handle(async move {
            let graphics_pipelines = graphics.await
                .context("failed to create graphics pipelines")?;
            let compute_pipelines = compute.await
                .context("failed to create compute pipelines")?;
            Ok(PipelineBatchInner::new(this_id, graphics_pipelines, compute_pipelines))
        }).context("send error")?;
        self.gpu.init_pipeline_batch(
            this_id, PipelineBatch::new(fut),
        );
        Ok(self.this_id)
    }
}

impl Drop for PipelineBatchBuilder {

    #[inline(always)]
    fn drop(&mut self) {
        if !self.built {
            log::warn!(
                "{}{}",
                format_args!("pipeline batch (id: {}) was not built, pipeline batches should always be explicitly built ",
                    self.this_id
                ),
                "and you should not rely on automatic builds"
            );
            let cloned = Self {
                this_id: self.this_id,
                gpu: self.gpu.clone(),
                graphics_create_infos: self.graphics_create_infos.clone(),
                compute_create_infos: self.compute_create_infos.clone(),
                cache: self.cache.clone(),
                built: false,
            };
            cloned.build().ok();
        }
    }
}

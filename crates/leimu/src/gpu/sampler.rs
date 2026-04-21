use core::{
    hash::{self, Hash},
    fmt::{self, Debug},
};

use leimu_proc::{Display, BuildStructure};
use tuhka::vk;

use crate::{
    gpu::prelude::*,
    error::*,
    sync::*,
};

struct Inner {
    device: Device,
    handle: vk::Sampler,
    create_info: SamplerCreateInfo,
}

impl Drop for Inner {

    fn drop(&mut self) {
        unsafe {
            self.device.destroy_sampler(
                self.handle, None
            );
        }
    }
}

#[derive(Clone, Display)]
#[display(format_args!("{:?}", self.inner.handle))]
pub struct Sampler {
    inner: Arc<Inner>,
}

impl Sampler {

    pub const LOD_CLAMP_NONE: f32 = vk::LOD_CLAMP_NONE;

    #[inline(always)]
    fn new(
        device: Device,
        create_info: SamplerCreateInfo,
    ) -> Result<Self> {
        let info = vk::SamplerCreateInfo {
            s_type: vk::StructureType::SAMPLER_CREATE_INFO,
            mag_filter: create_info.mag_filter.into(),
            min_filter: create_info.min_filter.into(),
            mipmap_mode: create_info.mip_mode.into(),
            address_mode_u: create_info.address_mode_u.into(),
            address_mode_v: create_info.address_mode_v.into(),
            address_mode_w: create_info.address_mode_w.into(),
            mip_lod_bias: create_info.mip_lod_bias,
            anisotropy_enable: create_info.max_anisotropy.is_some() as u32,
            max_anisotropy: create_info.max_anisotropy.unwrap_or_default(),
            compare_enable: create_info.compare_op.is_some() as u32,
            compare_op: create_info.compare_op.unwrap_or(CompareOp::NEVER).into(),
            min_lod: create_info.min_lod,
            max_lod: create_info.max_lod,
            border_color: create_info.border_color.into(),
            ..Default::default()
        };
        let handle = unsafe {
            device.create_sampler(&info, None)
            .context("failed to create sampler")?
        }.value;
        Ok(Self {
            inner: Arc::new(Inner { device, handle, create_info })
        })
    }

    #[inline(always)]
    pub fn handle(&self) -> TransientHandle<'_, vk::Sampler> {
        TransientHandle::new(self.inner.handle)
    }

    #[inline(always)]
    pub fn create_info(&self) -> &SamplerCreateInfo {
        &self.inner.create_info
    }
}

#[derive(Default, Clone, Copy, BuildStructure)]
pub struct SamplerCreateInfo {
    /// Specifies which magnification [`Filter`] to apply to look ups. Default is [`Filter::Nearest`].
    ///
    /// See the Vulkan docs for more information on filtering:
    /// <https://docs.vulkan.org/spec/latest/chapters/textures.html#textures-texel-filtering>
    pub mag_filter: Filter,
    /// Specifies which minification [`Filter`] to apply to look ups. Default is [`Filter::Nearest`].
    ///
    /// See the Vulkan docs for more information on filtering:
    /// <https://docs.vulkan.org/spec/latest/chapters/textures.html#textures-texel-filtering>
    pub min_filter: Filter,
    /// Specifies which mipmap filter to apply to lookups. Default is [`MipmapMode::Nearest`].
    ///
    /// See the Vulkan docs for more information on filtering:
    /// <https://docs.vulkan.org/spec/latest/chapters/textures.html#textures-texel-filtering>
    pub mip_mode: MipmapMode,
    /// Specifies the bias added to mipmap LOD calculation and bias provided by image sampling
    /// functions in SPIR-V.
    ///
    /// See the Vulkan docs for details:
    /// <https://docs.vulkan.org/spec/latest/chapters/textures.html#textures-level-of-detail-operation>
    pub mip_lod_bias: f32,
    /// Enables anisotropic filtering if `max_anisotropy` is [`Some`]. The default is [`None`].
    ///
    /// See the Vulkan docs for details:
    /// <https://docs.vulkan.org/spec/latest/chapters/textures.html#textures-texel-anisotropic-filtering>
    pub max_anisotropy: Option<f32>,
    #[skip]
    pub address_mode_u: SamplerAddressMode,
    #[skip]
    pub address_mode_v: SamplerAddressMode,
    #[skip]
    pub address_mode_w: SamplerAddressMode,
    pub compare_op: Option<CompareOp>,
    /// Specifies the value used to clamp the minimum level of detail value. The default value is
    /// `0.0`.
    ///
    /// See the Vulkan docs for details:
    /// <https://docs.vulkan.org/spec/latest/chapters/textures.html#textures-level-of-detail-operation>
    pub min_lod: f32,
    /// Specifies the value used to clamp the maximum level of detail value. To disable clamping
    /// the maximum, use the [`Sampler::LOD_CLAMP_NONE`] constant. The default value is `0.0`.
    ///
    /// See the Vulkan docs for details:
    /// <https://docs.vulkan.org/spec/latest/chapters/textures.html#textures-level-of-detail-operation>
    pub max_lod: f32,
    /// Specifies the border color when sampling outside a texture.
    pub border_color: BorderColor,
}

impl SamplerCreateInfo {

    /// Specifies which wrapping operation is used when the coordinates of used to sample an image
    /// would be out of bounds. The default is [`AddressMode::REPEAT`] for each coordinate.
    ///
    /// See the Vulkan docs for details:
    /// <https://docs.vulkan.org/spec/latest/chapters/textures.html#textures-wrapping-operation>
    #[inline(always)]
    pub fn address_mode(
        mut self,
        u: SamplerAddressMode,
        v: SamplerAddressMode,
        w: SamplerAddressMode,
    ) -> Self {
        self.address_mode_u = u;
        self.address_mode_v = v;
        self.address_mode_w = w;
        self
    }

    /// Builds the sampler, returning an error on failure.
    #[inline]
    pub fn build(self, device: Device) -> Result<Sampler> {
        Sampler::new(device, self)
    }
}

impl PartialEq for Sampler {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Sampler {}

impl Hash for Sampler {

    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.inner).hash(state);
    }
}

impl Debug for Sampler {

    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.handle.fmt(f)
    }
}

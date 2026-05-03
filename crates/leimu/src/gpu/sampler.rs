use core::{
    hash::{self, Hash},
    fmt::{self,
        Debug,
        Display,
    },
};

use leimu_proc::BuildStructure;
use tuhka::vk;

use crate::{
    gpu::prelude::*,
    mem::vec::Vec32,
    error::*,
    sync::*,
};

struct Inner {
    device: Device,
    handle: vk::Sampler,
    attributes: SamplerAttributes,
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

/// Represents a [`sampler`][1] object.
///
/// To create a [`Sampler`], use the [`build`][2] method of [`SamplerAttributes`].
///
/// [1]: vk::Sampler
/// [2]: SamplerAttributes::build
#[derive(Clone)]
pub struct Sampler {
    inner: Arc<Inner>,
}

/// Value that **can** be used to specify that sampler's [`max lod`][1] is unclamped.
///
/// [1]: SamplerAttributes::max_lod
pub const LOD_CLAMP_NONE: f32 = vk::LOD_CLAMP_NONE;

impl Sampler {

    #[inline(always)]
    fn new(
        device: Device,
        mut attributes: SamplerAttributes,
    ) -> Result<Self> {
        let mut info = vk::SamplerCreateInfo {
            s_type: vk::StructureType::SAMPLER_CREATE_INFO,
            mag_filter: attributes.mag_filter.into(),
            min_filter: attributes.min_filter.into(),
            mipmap_mode: attributes.mip_mode.into(),
            address_mode_u: attributes.address_mode_u.into(),
            address_mode_v: attributes.address_mode_v.into(),
            address_mode_w: attributes.address_mode_w.into(),
            mip_lod_bias: attributes.mip_lod_bias,
            anisotropy_enable: attributes.max_anisotropy.is_some() as u32,
            max_anisotropy: attributes.max_anisotropy.unwrap_or_default(),
            compare_enable: attributes.compare_op.is_some() as u32,
            compare_op: attributes.compare_op.unwrap_or(CompareOp::NEVER).into(),
            min_lod: attributes.min_lod,
            max_lod: attributes.max_lod,
            border_color: attributes.border_color.into(),
            ..Default::default()
        };
        for p_next in &mut attributes.p_next {
            info = info.push_next(&mut **p_next);
        }
        let handle = unsafe {
            device.create_sampler(&info, None)
            .context("failed to create sampler")?
        }.value;
        Ok(Self {
            inner: Arc::new(Inner { device, handle, attributes, })
        })
    }
    
    /// Returns the raw [`handle`][1].
    ///
    /// [1]: vk::Sampler
    #[inline(always)]
    pub fn handle(&self) -> TransientHandle<'_, vk::Sampler> {
        TransientHandle::new(self.inner.handle)
    }

    /// Returns the [`attributes`][1] used to create this sampler.
    ///
    /// [1]: SamplerAttributes
    #[inline(always)]
    pub fn attributes(&self) -> &SamplerAttributes {
        &self.inner.attributes
    }
}

/// Specifies parameters used to create a [`Sampler`].
#[derive(Default, BuildStructure)]
pub struct SamplerAttributes {
    #[skip]
    p_next: Vec32<Box<dyn vk::ExtendsSamplerCreateInfo + Send + Sync>>,
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
    /// Enables anisotropic filtering if `max_anisotropy` is [`Some`].
    ///
    /// The default is [`None`].
    ///
    /// See the Vulkan docs for details:
    /// <https://docs.vulkan.org/spec/latest/chapters/textures.html#textures-texel-anisotropic-filtering>
    pub max_anisotropy: Option<f32>,
    /// Specifies wrapping operation used when the u coordinate used to sample the image would be
    /// out of bounds.
    #[skip]
    pub address_mode_u: SamplerAddressMode,
    /// Specifies wrapping operation used when the v coordinate used to sample the image would be
    /// out of bounds.
    #[skip]
    pub address_mode_v: SamplerAddressMode,
    /// Specifies wrapping operation used when the w coordinate used to sample the image would be
    /// out of bounds.
    #[skip]
    pub address_mode_w: SamplerAddressMode,
    /// Specifies an optional [`CompareOp`] applied when fetching data before filtering.
    pub compare_op: Option<CompareOp>,
    /// Specifies the value used to clamp the minimum level of detail value.
    ///
    /// The default value is `0.0`.
    ///
    /// See the Vulkan docs for details:
    /// <https://docs.vulkan.org/spec/latest/chapters/textures.html#textures-level-of-detail-operation>
    pub min_lod: f32,
    /// Specifies the value used to clamp the maximum level of detail value. To disable clamping
    /// the maximum, use the [`LOD_CLAMP_NONE`] constant.
    ///
    /// The default value is `0.0`.
    ///
    /// See the Vulkan docs for details:
    /// <https://docs.vulkan.org/spec/latest/chapters/textures.html#textures-level-of-detail-operation>
    pub max_lod: f32,
    /// Specifies the border color when sampling outside a texture.
    pub border_color: BorderColor,
}

impl SamplerAttributes {

    /// Adds a structure extending [`vk::SamplerCreateInfo`].
    ///
    /// # Safety
    /// The usage of extension **must** be valid.
    pub unsafe fn with_p_next<T>(mut self, value: T) -> Self
        where T: vk::ExtendsSamplerCreateInfo + Send + Sync + 'static
    {
        self.p_next.push(Box::new(value));
        self
    }

    /// Specifies which wrapping operation is used when the coordinates used to sample an image
    /// would be out of bounds.
    ///
    /// The default is [`AddressMode::REPEAT`] for each coordinate.
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

    /// Creates the sampler.
    #[inline]
    pub fn build(self, device: Device) -> Result<Sampler> {
        Sampler::new(device, self)
    }
}

impl PartialEq for Sampler {

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Sampler {}

impl Hash for Sampler {

    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.inner).hash(state);
    }
}
impl Debug for Sampler {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Sampler")
            .field(&self.inner.handle)
            .finish()
    }
}
impl Display for Sampler {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.inner.handle)
    }
}

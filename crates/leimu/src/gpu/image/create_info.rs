use super::*;

/// The create info structure for images.
pub struct ImageCreateInfo<'a> {
    pub(crate) out: &'a mut ImageId,
    pub(crate) memory_binder: &'a dyn MemoryBinder,
    pub(super) aspects: ImageAspects,
    pub(super) dimensions: Dimensions,
    pub(super) format: Format,
    pub(super) usage: ImageUsages,
    pub(super) samples: MsaaSamples,
    pub(super) array_layers: u32,
    pub(super) mip_levels: u32,
    pub(super) cube_map: bool,
    pub(super) mutable_format: bool,
    pub(super) resolve_modes: FormatResolveModes,
}

impl<'a> ImageCreateInfo<'a> {

    /// Creates new [`ImageCreateInfo`].
    ///
    /// # Parameters
    /// - `out`: A mutable reference to where the [`ImageId`] of the created image will be stored.
    /// - `memory_binder`: Specifies what will bind the image's memory.
    ///
    /// You can specify a different memory binder by [`ImageCreateInfo::with_memory_binder`].
    /// # Valid usage
    /// - The dimensions and format of the image *must* be specified with
    ///   [`ImageCreateInfo::with_dimensions`] and [`ImageCreateInfo::with_format`].
    #[inline(always)]
    pub fn new(out: &'a mut ImageId, memory_binder: &'a dyn MemoryBinder) -> Self {
        Self {
            out,
            memory_binder,
            aspects: ImageAspects::empty(),
            dimensions: Dimensions::new(0, 0, 0),
            format: Format::Undefined,
            usage: Default::default(),
            samples: MsaaSamples::X1,
            array_layers: 1,
            mip_levels: 1,
            cube_map: false,
            mutable_format: false,
            resolve_modes: Default::default(),
        }
    }

    /// Specifies the images dimensions
    ///
    /// Note that every dimension including `depth` needs to non-zero.
    ///
    /// Setting the depth to a value other than one makes the image a 3D image, which requires the
    /// images array layers to be one.
    #[inline(always)]
    pub fn with_dimensions<D>(mut self, dimensions: D) -> Self
        where D: Into<Dimensions>
    {
        self.dimensions = dimensions.into();
        self
    }

    /// Specifies the images format and whether it can be mutated when creating an image subview.
    #[inline(always)]
    pub fn with_format(mut self, format: Format, mutable: bool) -> Self {
        self.format = format;
        self.aspects = format.aspects();
        self.resolve_modes = format.resolve_modes();
        self.mutable_format = mutable;
        self
    }

    /// Specifies what the image *can* be used for.
    #[inline(always)]
    pub fn with_usage(mut self, usage: ImageUsages) -> Self {
        self.usage |= usage;
        self
    }

    /// Specifies how many samples per pixel the image has in multisample anti-aliasing.
    #[inline(always)]
    pub fn with_samples(mut self, samples: MsaaSamples) -> Self {
        self.samples = samples;
        self
    }
    
    /// Specifies how many array layers the image has.
    ///
    /// Note that if this is a 3D image (depth dimension > 1), this *must* be set to one.
    #[inline(always)]
    pub fn with_array_layers(mut self, layers: u32) -> Self { 
        self.array_layers = layers;
        self
    }

    /// Specifies that the image can be used as a cube map.
    ///
    /// Requires that the width and height of the image are the same, depth is one and that array
    /// layer count is greater than or equal to 6.
    ///
    /// To unset this, pass [`None`].
    #[inline(always)]
    pub fn with_cube_map(mut self, cube_dimensions: Option<u32>) -> Self {
        if let Some(dim) = cube_dimensions {
            self.cube_map = true;
            self.dimensions = Dimensions::new(dim, dim, 1);
        } else {
            self.cube_map = false;
        }
        self
    }

    /// Specifies how many mip levels the image has.
    #[inline(always)]
    pub fn with_mip_levels(mut self, levels: u32) -> Self {
        self.mip_levels = levels;
        self
    }

    #[inline(always)]
    pub(crate) fn build(
        &self,
        device: Device,
        bind_memory_info: &mut vk::BindImageMemoryInfo<'static>,
    ) -> Result<ImageMeta>
    {
        ImageMeta::new(device, self, bind_memory_info)
    }
}

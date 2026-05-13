use leimu_proc::BuildStructure;

use super::*;

/// Parameters for [`creating`][1] an [`image object`][2].
///
/// See [`new`][Self::new] for more info.
///
/// [1]: Gpu::create_resources
/// [2]: Image
pub struct ImageCreateInfo<'a> {
    pub(crate) out: &'a mut ImageId,
    pub(crate) memory_binder: &'a dyn MemoryBinder,
    pub(super) create_flags: ImageCreateFlags,
    pub(super) aspects: ImageAspects,
    pub(super) dimensions: Dimensions,
    pub(super) ty: ImageType,
    pub(super) format: Format,
    pub(super) usage: ImageUsages,
    pub(super) stencil_usage: Option<ImageUsages>,
    pub(super) samples: MsaaSamples,
    pub(super) array_layers: u32,
    pub(super) mip_levels: u32,
    pub(super) resolve_modes: FormatResolveModes,
}

impl<'a> ImageCreateInfo<'a> {

    /// Creates new [`ImageCreateInfo`].
    ///
    /// # Parameters
    /// - `out`: A mutable reference to where the [`ImageId`] of the created image will be stored.
    /// - `memory_binder`: Specifies what will bind the image's memory.
    ///
    /// # Valid usage
    /// - The dimensions and format of the image **must** be specified with
    ///   [`ImageCreateInfo::with_dimensions`] and [`ImageCreateInfo::with_format`].
    #[inline]
    pub fn new(out: &'a mut ImageId, memory_binder: &'a dyn MemoryBinder) -> Self {
        Self {
            out,
            memory_binder,
            create_flags: ImageCreateFlags::empty(),
            aspects: ImageAspects::empty(),
            dimensions: Dimensions::new(0, 0, 0),
            ty: ImageType::Infer,
            format: Format::Undefined,
            usage: ImageUsages::empty(),
            stencil_usage: None,
            samples: MsaaSamples::X1,
            array_layers: 1,
            mip_levels: 1,
            resolve_modes: Default::default(),
        }
    }

    /// Specifies [`image creation flags`][1] used when creating the image.
    ///
    /// See [`ImageCreateFlags`] full description and valid usage.
    ///
    /// [1]: ImageCreateFlags
    #[inline]
    pub fn with_create_flags(
        mut self,
        flags: ImageCreateFlags,
    ) -> Self {
        self.create_flags |= flags;
        self
    }

    /// Specifies the images [`Dimensions`].
    ///
    /// # Valid usage
    /// - Each `width`, `height` and `depth` **must** be greater than or equal to 1.
    /// - If the image's type is either inferred or explicitly stated to be [`Type1d`][1],
    ///   `height` and `depth` of the given [`Dimensions`] **must** be equal to 1
    ///   and `width` **must** be less than or equal to [`max_image_dimension_1d`][2].
    /// - If the image's type is either inferred or explicitly stated to be [`Type2d`][3],
    ///   `depth` of the given [`Dimensions`] **must** be equal to 1 and `width`/`height`
    ///   **must** both be less than or equal to [`max_image_dimensions_2d`][4]
    /// - If the image's type is either inferred or explicitly stated to be [`Type3d`][5], each
    ///   `width`/`height`/`depth` of the given [`Dimensions`] **must** be less than or equal to
    ///   [`max_image_dimensions_3d`][6].
    ///
    /// [1]: ImageType::Type1d
    /// [2]: PhysicalDeviceLimits::max_image_dimension_1d
    /// [3]: ImageType::Type2d
    /// [4]: PhysicalDeviceLimits::max_image_dimension_2d
    /// [5]: ImageType::Type3d
    /// [6]: PhysicalDeviceLimits::max_image_dimension_3d
    #[inline]
    pub fn with_dimensions<D>(mut self, dimensions: D) -> Self
        where D: Into<Dimensions>
    {
        self.dimensions = dimensions.into();
        self
    }

    /// Specifies the [`type`][1] of the image.
    ///
    /// The default is [`ImageType::Infer`].
    ///
    /// [`ImageType::Infer`] infers the [`ImageType`] as follows:
    /// - (width >= 1, 1, 1): [`Type1d`][2]
    /// - (width >= 1, height > 1, 1): [`Type2d`][3]
    /// - (width >= 1, height >= 1, depth > 1). [`Type3d`][4]
    ///
    /// # Valid usage
    /// - If `ty` is [`Type1d`][2], the [`specified height and depth`][5] **must** be equal to 1.
    /// - If `ty` is [`Type2d`][3], the [`specified depth`][5] **must** be equal to 1.
    ///
    /// [1]: ImageType
    /// [2]: ImageType::Type1d
    /// [3]: ImageType::Type2d
    /// [4]: ImageType::Type3d
    /// [5]: Self::with_dimensions
    #[inline]
    pub fn with_type(mut self, ty: ImageType) -> Self {
        self.ty = ty;
        self
    }

    /// Specifies the images format and whether it can be mutated when creating an image subview.
    ///
    /// # Valid usage
    /// - `format` **must** not be [`Format::Undefined`].
    #[inline]
    pub fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self.aspects = format.aspects();
        self.resolve_modes = format.resolve_modes();
        self
    }

    /// Specifies what the image **can** be used for.
    ///
    /// # Valid usage
    /// - `usage` **must** not be [`empty`][ImageUsages::empty].
    /// - If `usage` contains [`COLOR_ATTACHMENT`][1], [`DEPTH_STENCIL_ATTACHMENT`][2] or
    ///   [`INPUT_ATTACHMENT`][3] usage, then the [`width and height`][4] of this image **must** be
    ///   less than or equal to [`max_framebuffer_width`][5] and [`max_framebuffer_height`][6]
    ///   respectively.
    ///
    /// [1]: ImageUsages::COLOR_ATTACHMENT
    /// [2]: ImageUsages::DEPTH_STENCIL_ATTACHMENT
    /// [3]: ImageUsages::INPUT_ATTACHMENT
    /// [4]: Self::with_dimensions
    /// [5]: PhysicalDeviceLimits::max_framebuffer_width
    /// [6]: PhysicalDeviceLimits::max_framebuffer_height
    #[inline]
    pub fn with_usage(mut self, usage: ImageUsages) -> Self {
        self.usage |= usage;
        self
    }

    /// Specifies a separate [`usage`][ImageUsages] for the [`stencil aspect`][1] of this image.
    ///
    /// # Valid usage
    /// - `usage` **must** not be [`empty`][ImageUsages::empty].
    /// - If [`depth usage`][2] contains [`DEPTH_STENCIL_ATTACHMENT`][3] usage, then `usage`
    ///   **must** also contain [`DEPTH_STENCIL_ATTACHMENT`][3].
    /// - If [`depth_usage`][2] does not contain [`DEPTH_STENCIL_ATTACHMENT`][3] usage, then `usage`
    ///   **must** not contain [`DEPTH_STENCIL_ATTACHMENT`][3].
    /// - If `usage` contains [`INPUT_ATTACHMENT`][4] usage, then the [`width and height`][5] of this
    ///   image **must** be less than or equal to [`max_framebuffer_width`][6] and
    ///   [`max_framebuffer_height`][7] respectively.
    ///
    /// [1]: ImageAspects::STENCIL
    /// [2]: Self::with_usage
    /// [3]: ImageUsages::DEPTH_STENCIL_ATTACHMENT
    /// [4]: ImageUsages::INPUT_ATTACHMENT
    /// [5]: Self::with_dimensions
    /// [6]: PhysicalDeviceLimits::max_framebuffer_width
    /// [7]: PhysicalDeviceLimits::max_framebuffer_height
    #[inline]
    pub fn with_separate_stencil_usage(mut self, usage: ImageUsages) -> Self {
        let u = self.stencil_usage.get_or_insert(usage);
        *u |= usage;
        self
    }

    /// Specifies how many samples per pixel the image has in multisample anti-aliasing.
    ///
    /// # Valid usage
    /// - Only one sample bit may be specified.
    /// - If `samples` is not equal to [`MsaaSamples::X1`], [`image type`][1] (inferred or
    ///   explicitly specified) **must** be [`ImageType::Type2d`], [`mip levels`][2] **must**
    ///   be equal to 1 and [`create flags`][3] **must** not contain [`CUBE_COMPATIBLE`][4] bit.
    ///
    /// [1]: Self::with_type
    /// [2]: Self::with_mip_levels
    /// [3]: Self::with_create_flags
    /// [4]: ImageCreateFlags::CUBE_COMPATIBLE
    #[inline]
    pub fn with_samples(mut self, samples: MsaaSamples) -> Self {
        self.samples = samples;
        self
    }
    
    /// Specifies how many array layers the image has.
    ///
    /// # Valid usage
    /// - `layers` **must** be greater than 0.
    /// - If the [`type`][1] of the image is either inferred or explicitly stated to be
    ///  [`Type3d`][2], `layers` **must** be equal to 1.
    /// - If the [`create flags`][3] of this image contains [`CUBE_COMPATIBLE`][4], `layers`
    ///   **must** be greater than or equal to 6.
    ///
    /// [1]: Self::with_type
    /// [2]: ImageType::Type3d
    /// [3]: Self::with_create_flags
    /// [4]: ImageCreateFlags::CUBE_COMPATIBLE
    #[inline]
    pub fn with_array_layers(mut self, layers: u32) -> Self { 
        self.array_layers = layers;
        self
    }

    /// Specifies how many mip levels the image has.
    ///
    /// The default value is 1.
    ///
    /// # Valid usage
    /// - `levels` **must** be greater than or equal to 1.
    /// - If the [`image's type`][1] (inferred or explicitly specified) is [`Type3d`][2],
    ///   `levels` **must** be 1.
    ///
    /// [1]: Self::with_type
    /// [2]: ImageType::Type3d
    #[inline]
    pub fn with_mip_levels(mut self, levels: u32) -> Self {
        self.mip_levels = levels;
        self
    }

    #[inline]
    pub(crate) fn build(
        &self,
        device: Device,
        id: ImageIndex,
        bind_memory_info: &mut vk::BindImageMemoryInfo<'static>,
    ) -> Result<Image>
    {
        Image::new(device, self, id, bind_memory_info)
    }
}

/// Specifies the parameters used to [`create an image view`][1].
///
/// [1]: Gpu::create_image_view
#[derive(Default, Clone, Copy, PartialEq, Eq, BuildStructure)]
pub struct ImageViewCreateInfo {
    /// Specifies the [`type`][1] of the view.
    ///
    /// [1]: ImageViewType
    pub view_type: ImageViewType,
    /// Specifies the [`subresource range`][1] of the view.
    ///
    /// [1]: ImageSubresourceRange
    pub subresource_range: ImageSubresourceRange,
    /// Specifies the optional [`component info`][1] of the view.
    ///
    /// [1]: ComponentInfo
    pub component_info: Option<ComponentInfo>,
}

impl ImageViewCreateInfo {

    /// Creates an [`ImageRange`] of a view of an entire image.
    #[inline]
    pub fn whole_range(aspect: ImageAspects) -> Self {
        Self {
            subresource_range: ImageSubresourceRange::default().aspect_mask(aspect),
            component_info: None,
            view_type: ImageViewType::Type2d,
        }
    }
}

use leimu_mem::int::NonZeroOption;

use super::*;

/// Meta data about an [`Image`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ImageProperties {
    /// [`Creation flags`][1] used to create the image.
    ///
    /// [1]: ImageCreateFlags
    pub create_flags: ImageCreateFlags,
    /// The [`Dimensions`] of the image.
    pub dimensions: Dimensions,
    /// The [`type`][ImageType] of the image.
    pub ty: ImageType,
    /// The [`Format`] used to create the image.
    pub format: Format,
    /// A bitmask of [`image aspects'][ImageAspects] of the image.
    pub aspect_mask: ImageAspects,
    /// The specified [`usages`][ImageUsages] used to create the image.
    pub usage: ImageUsages,
    /// The specified [`separate stencil usages`][ImageUsages] used to create the image.
    pub stencil_usage: Option<ImageUsages>,
    /// The number of [`samples`][MsaaSamples] in the image.
    pub samples: MsaaSamples,
    /// The number of array layers in the image.
    pub array_layers: u32,
    /// The number of mip levels in the image.
    pub mip_levels: u32,
    /// Supported resolve modes for `color`, `depth` and `stencil` [`aspects`][ImageAspects].
    pub format_resolve_modes: FormatResolveModes,
    /// [`FormatFeatures`] of the image's [`format`][Self::format].
    pub format_features: FormatFeatures,
}

impl ImageProperties {

    /// Returns whether the image has a mutable format.
    #[inline]
    pub fn has_mutable_format(&self) -> bool {
        self.create_flags.contains(ImageCreateFlags::MUTABLE_FORMAT)
    }

    /// Returns specified [`image usages`][1] for the given [`aspects`][2].
    ///
    /// [1]: ImageUsages
    /// [2]: ImageAspects
    #[inline]
    pub fn usage(&self, aspect: ImageAspects) -> ImageUsages {
        if aspect.contains(ImageAspects::STENCIL) &&
            let Some(stencil_usage) = self.stencil_usage
        {
            let mut usage = stencil_usage;
            if aspect.contains(ImageAspects::DEPTH) {
                usage &= self.usage;
            }
            usage
        } else {
            self.usage
        }
    }

    /// Validates an [`image subresource range`][1], optionally taking into account
    /// a [`view type`][ImageViewType].
    ///
    /// [1]: ImageSubresourceRange
    pub fn validate_subresource_range(
        &self,
        range: &mut ImageSubresourceRange,
        view_type: Option<ImageViewType>
    ) -> Result<u32>
    {
        if self.aspect_mask & range.aspect_mask != range.aspect_mask {
            return Err(Error::just_context(MissingFlagsError::new(
                range.aspect_mask,
                self.aspect_mask,
            )))
        }
        if let Some(view_type) = view_type &&
            matches!(view_type, ImageViewType::Type2d | ImageViewType::Type2dArray) &&
            matches!(self.ty, ImageType::Type3d)
        {
            let level_count = range.level_count
                .unwrap_or_sentinel(self.mip_levels.wrapping_sub(range.base_mip_level));
            if range.base_mip_level.saturating_sub(level_count) > self.mip_levels {
                return Err(Error::just_context(ImageSubresourceOutOfRangeError {
                    image_mip_levels: self.mip_levels,
                    base_level: range.base_mip_level,
                    level_count,
                    image_array_layers: self.array_layers,
                    base_layer: 0,
                    layer_count: 1,
                }))
            }
            let mip_dim = self.dimensions.lod(range.base_mip_level);
            if range.base_array_layer + range.layer_count > mip_dim.depth {
                return Err(Error::just_context(format!(
                    "subresource range base array layer {} + layer count {} is greater than 3D image depth {}",
                    range.base_array_layer, range.layer_count, mip_dim.depth,
                )))
            }
            *range = range.base_array_layer(0).layer_count(1);
            Ok(1)
        } else {
            let level_count = range.level_count
                .unwrap_or_sentinel(self.mip_levels.wrapping_sub(range.base_mip_level));
            let layer_count = range.layer_count
                .unwrap_or_sentinel(self.array_layers.wrapping_sub(range.base_array_layer));
            if range.base_mip_level.saturating_add(level_count) > self.mip_levels ||
                range.base_array_layer.saturating_add(layer_count) > self.array_layers
            {
                return Err(Error::just_context(ImageSubresourceOutOfRangeError {
                    image_mip_levels: self.mip_levels,
                    base_level: range.base_mip_level,
                    level_count,
                    image_array_layers: self.array_layers,
                    base_layer: range.base_array_layer,
                    layer_count,
                }))
            }
            Ok(layer_count)
        }
    }

    /// Validates [`ImageViewCreateInfo`].
    pub fn validate_view_info(&self, info: &mut ImageViewCreateInfo) -> Result<ImageViewType> {
        if let Some(component_info) = info.component_info &&
            self.format != component_info.format
        {
            if !self.has_mutable_format() {
                return Err(Error::just_context(format!(
                    "image has immutable format {}, requested format is {}",
                    self.format, component_info.format,
                )))
            }
            if !self.format.is_compatible_with(component_info.format) {
                return Err(Error::just_context(format!(
                    "image format {} is not compatbile with requested format {}",
                    self.format, component_info.format,
                )))
            }
        }
        let layer_count = self.validate_subresource_range(&mut info.subresource_range, Some(info.view_type))?;
        match info.view_type {
            ImageViewType::Infer => match self.ty {
                ImageType::Type1d => if layer_count > 1 {
                    Ok(ImageViewType::Type1dArray)
                } else {
                    Ok(ImageViewType::Type1d)
                },
                ImageType::Type2d => if layer_count > 1 {
                    Ok(ImageViewType::Type2dArray)
                } else {
                    Ok(ImageViewType::Type2d)
                },
                ImageType::Type3d => Ok(ImageViewType::Type3d),
                ImageType::Infer => unreachable!(),
            },
            view_type => {
                if !view_type.is_compatible(self.ty, self.create_flags) {
                    return Err(Error::just_context(format!(
                        "image type {} with create flags {} is not compatible with image view type {}",
                        self.ty, self.create_flags, info.view_type,
                    )))
                }
                if matches!(view_type, ImageViewType::Cube) &&
                    layer_count != 6
                {
                    return Err(Error::just_context(format!(
                        "layer count {layer_count} is not 6 with image view type cube"
                    )))
                }
                if matches!(view_type, ImageViewType::Cube) &&
                    !layer_count.is_multiple_of(6)
                {
                    return Err(Error::just_context(format!(
                        "layout count {layer_count} is not a multiple of 6"
                    )))
                }
                Ok(view_type)
            },
        }
    }
}

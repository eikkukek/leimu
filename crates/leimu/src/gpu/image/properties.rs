use leimu_mem::int::NonZeroOption;

use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ImageProperties {
    pub dimensions: Dimensions,
    pub aspect_mask: ImageAspects,
    pub format: Format,
    pub usage: ImageUsages,
    pub samples: MsaaSamples,
    pub array_layers: u32,
    pub mip_levels: u32,
    pub create_flags: vk::ImageCreateFlags,
    pub format_resolve_modes: FormatResolveModes,
    pub format_features: FormatFeatures,
}

impl ImageProperties {

    pub fn validate_subresource_range(&self, range: &ImageSubresourceRange) -> Result<u32> {
        if self.aspect_mask & range.aspect_mask != range.aspect_mask {
            return Err(Error::just_context(MissingFlagsError::new(
                range.aspect_mask,
                self.aspect_mask,
            )))
        }
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

    pub fn validate_range(&self, range: &ImageRange) -> Result<vk::ImageViewType> {
        if let Some(component_info) = range.component_info &&
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
        let layer_count = self.validate_subresource_range(&range.subresource_range)?;
        if range.is_cube_map {
            if !self.create_flags.contains(vk::ImageCreateFlags::CUBE_COMPATIBLE) {
                Err(Error::just_context("image is not cube compatible"))
            } else if !layer_count.is_multiple_of(6) {
                Err(Error::just_context(format!(
                    "view layer count {layer_count} must be a multiple of 6 if used as a cube map",
                )))
            } else if layer_count > 6 {
                Ok(vk::ImageViewType::CUBE_ARRAY)
            } else {
                Ok(vk::ImageViewType::CUBE)
            }
        } else {
            Ok(if self.dimensions.depth > 1 {
                vk::ImageViewType::TYPE_3D
            } else if layer_count > 1 {
                vk::ImageViewType::TYPE_2D_ARRAY
            } else {
                vk::ImageViewType::TYPE_2D
            })
        }
    }

    #[inline(always)]
    pub fn has_mutable_format(&self) -> bool {
        self.create_flags.contains(vk::ImageCreateFlags::MUTABLE_FORMAT)
    }
}

use core::{error, fmt::{self, Display}};

use super::*;

#[derive(Debug)]
pub struct ImageSubresourceOutOfRangeError {
    pub image_mip_levels: u32, 
    pub base_level: u32,
    pub level_count: u32,
    pub image_array_layers: u32,
    pub base_layer: u32,
    pub layer_count: u32,
}

impl Display for ImageSubresourceOutOfRangeError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "{}{}",
            format_args!("subresource (base level {}, level count {}, base layer {}, layer count {}) ",
                self.base_level, self.level_count, self.base_layer, self.layer_count,
            ),
            format_args!("was out of range with image mip levels {} and array layers {}",
                self.image_mip_levels, self.image_array_layers,
            ),
            
        )
    }
}

impl error::Error for ImageSubresourceOutOfRangeError {}

impl Flags for ImageUsages {

    const NAME: &str = "image usage";
}

impl Flags for ImageAspects {

    const NAME: &str = "image aspects";
}

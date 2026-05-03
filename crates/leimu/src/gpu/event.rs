use super::prelude::*;

/// Specifies a [`Gpu`] event.
pub enum Event {
    /// The swapchain for surface with `surface_id` has been (re)created.
    SwapchainCreated {
        /// The [`id`][1] of the surface, which owns the created swapchain.
        ///
        /// [1]: SurfaceId
        surface_id: SurfaceId,
        /// The [`Format`] of the swapchain's images.
        new_format: Format,
        /// The size of the swapchain's images.
        new_size: (u32, u32),
        /// The number of images in the swapchain.
        image_count: u32,
    },
}

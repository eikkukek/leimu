use core::fmt::{self, Display};

use super::*;

mod id_base {

    use super::*;

    #[must_use]
    #[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Id<ImageId>(pub(super) ImageId, pub(super) u32)
        where ImageId: ResourceId<Image>;

    impl<ImageId> Display for Id<ImageId>
        where ImageId: ResourceId<Image>
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}::{}", self.0, self.1)
        }
    }

    impl<ImageId> Id<ImageId>
        where ImageId: ResourceId<Image>
    {

        /// Gets the [`ImageId`] (or [`TransientImageId`]) portion of the id.
        pub fn image_id(self) -> ImageId {
            self.0
        }

        #[inline(always)]
        pub(crate) fn view_id(self) -> u32 {
            self.1
        }

        #[inline(always)]
        pub(crate) fn new(
            image_id: ImageId,
            view_index: u32,
        ) -> Self {
            Self(image_id, view_index)
        }

        #[inline(always)]
        pub(crate) fn into_bare(self) -> BareImageViewId {
            BareImageViewId::new(self.0.slot_index(), self.1)
        }
    }
}

/// An id to an [`ImageView`].
pub type ImageViewId = id_base::Id<ImageId>;

/// An id to an [`ImageView`] that is a part of a swapchain of a given [`surface`][1].
///
/// [1]: Gpu::create_surface
pub type SwapchainImageViewId<'a> = id_base::Id<SwapchainImageId<'a>>;

pub(crate) type BareImageViewId = id_base::Id<ImageIndex>;

impl From<ImageViewId> for BareImageViewId {

    #[inline]
    fn from(value: ImageViewId) -> Self {
        Self::new(value.image_id().slot_index(), value.view_id())
    }
}

impl From<SwapchainImageViewId<'_>> for BareImageViewId { 

    #[inline]
    fn from(value: SwapchainImageViewId) -> Self {
        Self::new(value.image_id().slot_index(), value.view_id())
    }
}

/// Represents any [`ImageView`] id.
pub type AnyImageViewId<T> = id_base::Id<T>;

/// A structure containing the handle and [`creation parameters`][1] of a previously [`created`][2]
/// image view.
///
/// You should generally **not** try to access this structure directly, but instead use it through
/// [`ids`][ImageViewId] passed to [`commands`][2] and [`Gpu`].
///
/// [1]: ImageViewCreateInfo
/// [2]: Gpu::create_image_view
#[derive(Clone, Copy)]
pub struct ImageView {
    /// The raw [`Vulkan handle`][1].
    ///
    /// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkImageView.html
    pub handle: vk::ImageView,
    /// The [`view type`][ImageViewType] used to create this view.
    pub ty: ImageViewType,
    /// The [`subresource range`][ImageSubresourceRange] used to create this view.
    pub subresource_range: ImageSubresourceRange,
    /// The [`ComponentInfo`] used to create this view.
    pub component_info: ComponentInfo,
}

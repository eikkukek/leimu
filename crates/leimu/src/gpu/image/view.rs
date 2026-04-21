use core::fmt::{self, Display};

use super::*;

mod id_base {

    use super::*;

    #[must_use]
    #[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Id<ImageId>(pub(super) ImageId, pub(super) u32)
        where ImageId: ResourceId<ImageMeta>;

    impl<ImageId> Display for Id<ImageId>
        where ImageId: ResourceId<ImageMeta>
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}::{}", self.0, self.1)
        }
    }

    impl<ImageId> Id<ImageId>
        where ImageId: ResourceId<ImageMeta>
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

pub type ImageViewId = id_base::Id<ImageId>;
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

pub type AnyImageViewId<T> = id_base::Id<T>;

#[derive(Clone, Copy)]
pub struct ImageView {
    pub handle: vk::ImageView,
    pub subresource_range: ImageSubresourceRange,
    pub component_info: ComponentInfo,
    pub is_cube_map: bool,
}

use core::fmt::{self, Display};

use tuhka::vk;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Version(pub u32);

/// Vulkan api version 1.0.
pub const API_VERSION_1_0: Version = Version(vk::API_VERSION_1_0);
/// Vulkan api version 1.1.
pub const API_VERSION_1_1: Version = Version(vk::API_VERSION_1_1);
/// Vulkan api version 1.2.
pub const API_VERSION_1_2: Version = Version(vk::API_VERSION_1_2);
/// Vulkan api version 1.3.
pub const API_VERSION_1_3: Version = Version(vk::API_VERSION_1_3);
/// Vulkan api version 1.4.
pub const API_VERSION_1_4: Version = Version(vk::API_VERSION_1_4);
/// Maximum value of [`Version`].
pub const VERSION_MAX: Version = Version(u32::MAX);

pub const fn make_api_version(
    variant: u32,
    major: u32,
    minor: u32,
    patch: u32,
) -> Version {
    Version(vk::make_api_version(variant, major, minor, patch))
}

impl Version {

    #[inline]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    #[inline]
    pub const fn from_u32(value: u32) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn variant(self) -> u32 {
        vk::api_version_variant(self.0)
    }

    #[inline]
    pub const fn major(self) -> u32 {
        vk::api_version_major(self.0)
    }

    #[inline]
    pub const fn minor(self) -> u32 {
        vk::api_version_minor(self.0)
    }

    #[inline]
    pub const fn patch(self) -> u32 {
        vk::api_version_patch(self.0)
    }
}

impl From<Version> for u32 {

    #[inline]
    fn from(value: Version) -> u32 {
        value.0
    }
}

impl From<u32> for Version {

    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Default for Version {

    #[inline]
    fn default() -> Self {
        make_api_version(0, 1, 0, 0)
    }
}

impl PartialEq<u32> for Version {

    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u32> for Version {

    fn partial_cmp(&self, other: &u32) -> Option<core::cmp::Ordering> {
        Some(self.0.cmp(other))
    }
}

impl Display for Version {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{} variant({})",
            self.major(),
            self.minor(),
            self.patch(),
            self.variant()
        )
    }
}

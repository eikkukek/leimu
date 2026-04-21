use core::fmt::{self, Display};

use tuhka::vk;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Version(pub u32);

impl Version {

    pub const MAX: Version = Version(!0);

    pub const VULKAN_API_VERSION_1_0: Version = Version(vk::API_VERSION_1_0);
    pub const VULKAN_API_VERSION_1_1: Version = Version(vk::API_VERSION_1_1);
    pub const VULKAN_API_VERSION_1_2: Version = Version(vk::API_VERSION_1_2);
    pub const VULKAN_API_VERSION_1_3: Version = Version(vk::API_VERSION_1_3);
    pub const VULKAN_API_VERSION_1_4: Version = Version(vk::API_VERSION_1_4);

    #[inline]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        let value =
            (major << 22) |
            ((minor & 0x3FF) << 12) |
            (patch & 0xFFF);
        Self(value)
    }

    #[inline]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    #[inline]
    pub const fn from_u32(value: u32) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn major(self) -> u32 {
        self.0 >> 22
    }

    #[inline]
    pub const fn minor(self) -> u32 {
        (self.0 >> 12) & 0x3FF
    }

    #[inline]
    pub const fn patch(self) -> u32 {
        self.0 & 0xFFF
    }

    #[inline]
    pub const fn default() -> Self {
        Self::new(1, 0, 0)
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
        Self::new(1, 0, 0)
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
        write!(f, "{}.{}.{}",
            self.major(),
            self.minor(),
            self.patch(),
        )
    }
}

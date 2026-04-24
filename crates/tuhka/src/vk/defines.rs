/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAKE_API_VERSION.html>
#[inline]
pub const fn make_api_version(
    variant: u32,
    major: u32,
    minor: u32,
    patch: u32
) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_VARIANT.html>
#[inline]
pub const fn api_version_variant(version: u32) -> u32 {
    version >> 29
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_MAJOR.html>
#[inline]
pub const fn api_version_major(version: u32) -> u32 {
    (version >> 22) & 0x7f
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_MINOR.html>
#[inline]
pub const fn api_version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3ff
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_PATCH.html>
#[inline]
pub const fn api_version_patch(version: u32) -> u32 {
    version & 0xfff
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_0.html>
pub const API_VERSION_1_0: u32 = make_api_version(
    0, 1, 0, 0,
);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_1.html>
pub const API_VERSION_1_1: u32 = make_api_version(
    0, 1, 1, 0,
);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_2.html>
pub const API_VERSION_1_2: u32 = make_api_version(
    0, 1, 2, 0,
);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_3.html>
pub const API_VERSION_1_3: u32 = make_api_version(
    0, 1, 3, 0,
);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_4.html>
pub const API_VERSION_1_4: u32 = make_api_version(
    0, 1, 4, 0,
);

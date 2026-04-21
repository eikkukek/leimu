use std::fs::File;
use std::collections::HashSet;
use std::io::Write;
use quote::quote;
use syn::Ident;
use proc_macro2::Span;
use indexmap::IndexMap;

const VK_FORMATS: &str = "
[undefined]
VK_FORMAT_UNDEFINED = 0,
[8-bit]
VK_FORMAT_R4G4_UNORM_PACK8 = 1,
[16-bit]
VK_FORMAT_R4G4B4A4_UNORM_PACK16 = 2,
[16-bit]
VK_FORMAT_B4G4R4A4_UNORM_PACK16 = 3,
[16-bit]
VK_FORMAT_R5G6B5_UNORM_PACK16 = 4,
[16-bit]
VK_FORMAT_B5G6R5_UNORM_PACK16 = 5,
[16-bit]
VK_FORMAT_R5G5B5A1_UNORM_PACK16 = 6,
[16-bit]
VK_FORMAT_B5G5R5A1_UNORM_PACK16 = 7,
[16-bit]
VK_FORMAT_A1R5G5B5_UNORM_PACK16 = 8,
[8-bit]
VK_FORMAT_R8_UNORM = 9,
[8-bit]
VK_FORMAT_R8_SNORM = 10,
[8-bit]
VK_FORMAT_R8_USCALED = 11,
[8-bit]
VK_FORMAT_R8_SSCALED = 12,
[8-bit]
VK_FORMAT_R8_UINT = 13,
[8-bit]
VK_FORMAT_R8_SINT = 14,
[8-bit]
VK_FORMAT_R8_SRGB = 15,
[16-bit]
VK_FORMAT_R8G8_UNORM = 16,
[16-bit]
VK_FORMAT_R8G8_SNORM = 17,
[16-bit]
VK_FORMAT_R8G8_USCALED = 18,
[16-bit]
VK_FORMAT_R8G8_SSCALED = 19,
[16-bit]
VK_FORMAT_R8G8_UINT = 20,
[16-bit]
VK_FORMAT_R8G8_SINT = 21,
[16-bit]
VK_FORMAT_R8G8_SRGB = 22,
[24-bit]
VK_FORMAT_R8G8B8_UNORM = 23,
[24-bit]
VK_FORMAT_R8G8B8_SNORM = 24,
[24-bit]
VK_FORMAT_R8G8B8_USCALED = 25,
[24-bit]
VK_FORMAT_R8G8B8_SSCALED = 26,
[24-bit]
VK_FORMAT_R8G8B8_UINT = 27,
[24-bit]
VK_FORMAT_R8G8B8_SINT = 28,
[24-bit]
VK_FORMAT_R8G8B8_SRGB = 29,
[24-bit]
VK_FORMAT_B8G8R8_UNORM = 30,
[24-bit]
VK_FORMAT_B8G8R8_SNORM = 31,
[24-bit]
VK_FORMAT_B8G8R8_USCALED = 32,
[24-bit]
VK_FORMAT_B8G8R8_SSCALED = 33,
[24-bit]
VK_FORMAT_B8G8R8_UINT = 34,
[24-bit]
VK_FORMAT_B8G8R8_SINT = 35,
[24-bit]
VK_FORMAT_B8G8R8_SRGB = 36,
[32-bit]
VK_FORMAT_R8G8B8A8_UNORM = 37,
[32-bit]
VK_FORMAT_R8G8B8A8_SNORM = 38,
[32-bit]
VK_FORMAT_R8G8B8A8_USCALED = 39,
[32-bit]
VK_FORMAT_R8G8B8A8_SSCALED = 40,
[32-bit]
VK_FORMAT_R8G8B8A8_UINT = 41,
[32-bit]
VK_FORMAT_R8G8B8A8_SINT = 42,
[32-bit]
VK_FORMAT_R8G8B8A8_SRGB = 43,
[32-bit]
VK_FORMAT_B8G8R8A8_UNORM = 44,
[32-bit]
VK_FORMAT_B8G8R8A8_SNORM = 45,
[32-bit]
VK_FORMAT_B8G8R8A8_USCALED = 46,
[32-bit]
VK_FORMAT_B8G8R8A8_SSCALED = 47,
[32-bit]
VK_FORMAT_B8G8R8A8_UINT = 48,
[32-bit]
VK_FORMAT_B8G8R8A8_SINT = 49,
[32-bit]
VK_FORMAT_B8G8R8A8_SRGB = 50,
[32-bit]
VK_FORMAT_A8B8G8R8_UNORM_PACK32 = 51,
[32-bit]
VK_FORMAT_A8B8G8R8_SNORM_PACK32 = 52,
[32-bit]
VK_FORMAT_A8B8G8R8_USCALED_PACK32 = 53,
[32-bit]
VK_FORMAT_A8B8G8R8_SSCALED_PACK32 = 54,
[32-bit]
VK_FORMAT_A8B8G8R8_UINT_PACK32 = 55,
[32-bit]
VK_FORMAT_A8B8G8R8_SINT_PACK32 = 56,
[32-bit]
VK_FORMAT_A8B8G8R8_SRGB_PACK32 = 57,
[32-bit]
VK_FORMAT_A2R10G10B10_UNORM_PACK32 = 58,
[32-bit]
VK_FORMAT_A2R10G10B10_SNORM_PACK32 = 59,
[32-bit]
VK_FORMAT_A2R10G10B10_USCALED_PACK32 = 60,
[32-bit]
VK_FORMAT_A2R10G10B10_SSCALED_PACK32 = 61,
[32-bit]
VK_FORMAT_A2R10G10B10_UINT_PACK32 = 62,
[32-bit]
VK_FORMAT_A2R10G10B10_SINT_PACK32 = 63,
[32-bit]
VK_FORMAT_A2B10G10R10_UNORM_PACK32 = 64,
[32-bit]
VK_FORMAT_A2B10G10R10_SNORM_PACK32 = 65,
[32-bit]
VK_FORMAT_A2B10G10R10_USCALED_PACK32 = 66,
[32-bit]
VK_FORMAT_A2B10G10R10_SSCALED_PACK32 = 67,
[32-bit]
VK_FORMAT_A2B10G10R10_UINT_PACK32 = 68,
[32-bit]
VK_FORMAT_A2B10G10R10_SINT_PACK32 = 69,
[16-bit]
VK_FORMAT_R16_UNORM = 70,
[16-bit]
VK_FORMAT_R16_SNORM = 71,
[16-bit]
VK_FORMAT_R16_USCALED = 72,
[16-bit]
VK_FORMAT_R16_SSCALED = 73,
[16-bit]
VK_FORMAT_R16_UINT = 74,
[16-bit]
VK_FORMAT_R16_SINT = 75,
[16-bit]
VK_FORMAT_R16_SFLOAT = 76,
[32-bit]
VK_FORMAT_R16G16_UNORM = 77,
[32-bit]
VK_FORMAT_R16G16_SNORM = 78,
[32-bit]
VK_FORMAT_R16G16_USCALED = 79,
[32-bit]
VK_FORMAT_R16G16_SSCALED = 80,
[32-bit]
VK_FORMAT_R16G16_UINT = 81,
[32-bit]
VK_FORMAT_R16G16_SINT = 82,
[32-bit]
VK_FORMAT_R16G16_SFLOAT = 83,
[48-bit]
VK_FORMAT_R16G16B16_UNORM = 84,
[48-bit]
VK_FORMAT_R16G16B16_SNORM = 85,
[48-bit]
VK_FORMAT_R16G16B16_USCALED = 86,
[48-bit]
VK_FORMAT_R16G16B16_SSCALED = 87,
[48-bit]
VK_FORMAT_R16G16B16_UINT = 88,
[48-bit]
VK_FORMAT_R16G16B16_SINT = 89,
[48-bit]
VK_FORMAT_R16G16B16_SFLOAT = 90,
[64-bit]
VK_FORMAT_R16G16B16A16_UNORM = 91,
[64-bit]
VK_FORMAT_R16G16B16A16_SNORM = 92,
[64-bit]
VK_FORMAT_R16G16B16A16_USCALED = 93,
[64-bit]
VK_FORMAT_R16G16B16A16_SSCALED = 94,
[64-bit]
VK_FORMAT_R16G16B16A16_UINT = 95,
[64-bit]
VK_FORMAT_R16G16B16A16_SINT = 96,
[64-bit]
VK_FORMAT_R16G16B16A16_SFLOAT = 97,
[32-bit]
VK_FORMAT_R32_UINT = 98,
[32-bit]
VK_FORMAT_R32_SINT = 99,
[32-bit]
VK_FORMAT_R32_SFLOAT = 100,
[64-bit]
VK_FORMAT_R32G32_UINT = 101,
[64-bit]
VK_FORMAT_R32G32_SINT = 102,
[64-bit]
VK_FORMAT_R32G32_SFLOAT = 103,
[96-bit]
VK_FORMAT_R32G32B32_UINT = 104,
[96-bit]
VK_FORMAT_R32G32B32_SINT = 105,
[96-bit]
VK_FORMAT_R32G32B32_SFLOAT = 106,
[128-bit]
VK_FORMAT_R32G32B32A32_UINT = 107,
[128-bit]
VK_FORMAT_R32G32B32A32_SINT = 108,
[128-bit]
VK_FORMAT_R32G32B32A32_SFLOAT = 109,
[64-bit]
VK_FORMAT_R64_UINT = 110,
[64-bit]
VK_FORMAT_R64_SINT = 111,
[64-bit]
VK_FORMAT_R64_SFLOAT = 112,
[128-bit]
VK_FORMAT_R64G64_UINT = 113,
[128-bit]
VK_FORMAT_R64G64_SINT = 114,
[128-bit]
VK_FORMAT_R64G64_SFLOAT = 115,
[192-bit]
VK_FORMAT_R64G64B64_UINT = 116,
[192-bit]
VK_FORMAT_R64G64B64_SINT = 117,
[192-bit]
VK_FORMAT_R64G64B64_SFLOAT = 118,
[256-bit]
VK_FORMAT_R64G64B64A64_UINT = 119,
[256-bit]
VK_FORMAT_R64G64B64A64_SINT = 120,
[256-bit]
VK_FORMAT_R64G64B64A64_SFLOAT = 121,
[32-bit]
VK_FORMAT_B10G11R11_UFLOAT_PACK32 = 122,
[32-bit]
VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 = 123,
[D16]
VK_FORMAT_D16_UNORM = 124,
[D24]
VK_FORMAT_X8_D24_UNORM_PACK32 = 125,
[D32]
VK_FORMAT_D32_SFLOAT = 126,
[S8]
VK_FORMAT_S8_UINT = 127,
[D16S8]
VK_FORMAT_D16_UNORM_S8_UINT = 128,
[D24S8]
VK_FORMAT_D24_UNORM_S8_UINT = 129,
[D32S8]
VK_FORMAT_D32_SFLOAT_S8_UINT = 130,
[BC1_RGB]
VK_FORMAT_BC1_RGB_UNORM_BLOCK = 131,
[BC1_RGB]
VK_FORMAT_BC1_RGB_SRGB_BLOCK = 132,
[BC1_RGBA]
VK_FORMAT_BC1_RGBA_UNORM_BLOCK = 133,
[BC1_RGBA]
VK_FORMAT_BC1_RGBA_SRGB_BLOCK = 134,
[BC2]
VK_FORMAT_BC2_UNORM_BLOCK = 135,
[BC2]
VK_FORMAT_BC2_SRGB_BLOCK = 136,
[BC3]
VK_FORMAT_BC3_UNORM_BLOCK = 137,
[BC3]
VK_FORMAT_BC3_SRGB_BLOCK = 138,
[BC4]
VK_FORMAT_BC4_UNORM_BLOCK = 139,
[BC4]
VK_FORMAT_BC4_SNORM_BLOCK = 140,
[BC5]
VK_FORMAT_BC5_UNORM_BLOCK = 141,
[BC5]
VK_FORMAT_BC5_SNORM_BLOCK = 142,
[BC6H]
VK_FORMAT_BC6H_UFLOAT_BLOCK = 143,
[BC6H]
VK_FORMAT_BC6H_SFLOAT_BLOCK = 144,
[BC7]
VK_FORMAT_BC7_UNORM_BLOCK = 145,
[BC7]
VK_FORMAT_BC7_SRGB_BLOCK = 146,
[ETC2_RGB]
VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK = 147,
[ETC2_RGB]
VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK = 148,
[ETC2_RGBA]
VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK = 149,
[ETC2_RGBA]
VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK = 150,
[ETC2_EAC_RGBA]
VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK = 151,
[ETC2_EAC_RGBA]
VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK = 152,
[EAC_R]
VK_FORMAT_EAC_R11_UNORM_BLOCK = 153,
[EAC_R]
VK_FORMAT_EAC_R11_SNORM_BLOCK = 154,
[EAC_RG]
VK_FORMAT_EAC_R11G11_UNORM_BLOCK = 155,
[EAC_RG]
VK_FORMAT_EAC_R11G11_SNORM_BLOCK = 156,
[ASTC_4x4]
VK_FORMAT_ASTC_4x4_UNORM_BLOCK = 157,
[ASTC_4x4]
VK_FORMAT_ASTC_4x4_SRGB_BLOCK = 158,
[ASTC_5x4]
VK_FORMAT_ASTC_5x4_UNORM_BLOCK = 159,
[ASTC_5x4]
VK_FORMAT_ASTC_5x4_SRGB_BLOCK = 160,
[ASTC_5x5]
VK_FORMAT_ASTC_5x5_UNORM_BLOCK = 161,
[ASTC_5x5]
VK_FORMAT_ASTC_5x5_SRGB_BLOCK = 162,
[ASTC_6x5]
VK_FORMAT_ASTC_6x5_UNORM_BLOCK = 163,
[ASTC_6x5]
VK_FORMAT_ASTC_6x5_SRGB_BLOCK = 164,
[ASTC_6x5]
VK_FORMAT_ASTC_6x6_UNORM_BLOCK = 165,
[ASTC_6x5]
VK_FORMAT_ASTC_6x6_SRGB_BLOCK = 166,
[ASTC_8x5]
VK_FORMAT_ASTC_8x5_UNORM_BLOCK = 167,
[ASTC_8x5]
VK_FORMAT_ASTC_8x5_SRGB_BLOCK = 168,
[ASTC_8x6]
VK_FORMAT_ASTC_8x6_UNORM_BLOCK = 169,
[ASTC_8x6]
VK_FORMAT_ASTC_8x6_SRGB_BLOCK = 170,
[ASTC_8x8]
VK_FORMAT_ASTC_8x8_UNORM_BLOCK = 171,
[ASTC_8x8]
VK_FORMAT_ASTC_8x8_SRGB_BLOCK = 172,
[ASTC_10x5]
VK_FORMAT_ASTC_10x5_UNORM_BLOCK = 173,
[ASTC_10x5]
VK_FORMAT_ASTC_10x5_SRGB_BLOCK = 174,
[ASTC_10x6]
VK_FORMAT_ASTC_10x6_UNORM_BLOCK = 175,
[ASTC_10x6]
VK_FORMAT_ASTC_10x6_SRGB_BLOCK = 176,
[ASTC_10x8]
VK_FORMAT_ASTC_10x8_UNORM_BLOCK = 177,
[ASTC_10x8]
VK_FORMAT_ASTC_10x8_SRGB_BLOCK = 178,
[ASTC_10x10]
VK_FORMAT_ASTC_10x10_UNORM_BLOCK = 179,
[ASTC_10x10]
VK_FORMAT_ASTC_10x10_SRGB_BLOCK = 180,
[ASTC_12x10]
VK_FORMAT_ASTC_12x10_UNORM_BLOCK = 181,
[ASTC_12x10]
VK_FORMAT_ASTC_12x10_SRGB_BLOCK = 182,
[ASTC_12x12]
VK_FORMAT_ASTC_12x12_UNORM_BLOCK = 183,
[ASTC_12x12]
VK_FORMAT_ASTC_12x12_SRGB_BLOCK = 184,
// Provided by VK_VERSION_1_1
[32-bit G8B8G8R8]
VK_FORMAT_G8B8G8R8_422_UNORM = 1000156000,
// Provided by VK_VERSION_1_1
[32-bit B8G8R8G8]
VK_FORMAT_B8G8R8G8_422_UNORM = 1000156001,
// Provided by VK_VERSION_1_1
[8-bit 3-plane 420]
VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM = 1000156002,
// Provided by VK_VERSION_1_1
[8-bit 2-plane 420]
VK_FORMAT_G8_B8R8_2PLANE_420_UNORM = 1000156003,
// Provided by VK_VERSION_1_1
[8-bit 3-plane 422]
VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM = 1000156004,
// Provided by VK_VERSION_1_1
[8-bit 2-plane 422]
VK_FORMAT_G8_B8R8_2PLANE_422_UNORM = 1000156005,
// Provided by VK_VERSION_1_1
[8-bit 3-plane 444]
VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM = 1000156006,
// Provided by VK_VERSION_1_1
[16-bit]
VK_FORMAT_R10X6_UNORM_PACK16 = 1000156007,
// Provided by VK_VERSION_1_1
[32-bit]
VK_FORMAT_R10X6G10X6_UNORM_2PACK16 = 1000156008,
// Provided by VK_VERSION_1_1
[64-bit R10G10B10A10]
VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16 = 1000156009,
// Provided by VK_VERSION_1_1
[64-bit G10B10G10R10]
VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 = 1000156010,
// Provided by VK_VERSION_1_1
[64-bit B10G10R10G10]
VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 = 1000156011,
// Provided by VK_VERSION_1_1
[10-bit 3-plane 420]
VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 = 1000156012,
// Provided by VK_VERSION_1_1
[10-bit 2-plane 420]
VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 = 1000156013,
// Provided by VK_VERSION_1_1
[10-bit 3-plane 422]
VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 = 1000156014,
// Provided by VK_VERSION_1_1
[10-bit 2-plane 422]
VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 = 1000156015,
// Provided by VK_VERSION_1_1
[10-bit 3-plane 444]
VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 = 1000156016,
// Provided by VK_VERSION_1_1
[16-bit]
VK_FORMAT_R12X4_UNORM_PACK16 = 1000156017,
// Provided by VK_VERSION_1_1
[32-bit]
VK_FORMAT_R12X4G12X4_UNORM_2PACK16 = 1000156018,
// Provided by VK_VERSION_1_1
[64-bit R12G12B12A12]
VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16 = 1000156019,
// Provided by VK_VERSION_1_1
[64-bit G12B12G12R12]
VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 = 1000156020,
// Provided by VK_VERSION_1_1
[64-bit B12G12R12G12]
VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 = 1000156021,
// Provided by VK_VERSION_1_1
[12-bit 3-plane 420]
VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 = 1000156022,
// Provided by VK_VERSION_1_1
[12-bit 2-plane 420]
VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 = 1000156023,
// Provided by VK_VERSION_1_1
[12-bit 3-plane 422]
VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 = 1000156024,
// Provided by VK_VERSION_1_1
[12-bit 2-plane 422]
VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 = 1000156025,
// Provided by VK_VERSION_1_1
[12-bit 3-plane 444]
VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 = 1000156026,
// Provided by VK_VERSION_1_1
[64-bit G16B16G16R16]
VK_FORMAT_G16B16G16R16_422_UNORM = 1000156027,
// Provided by VK_VERSION_1_1
[64-bit B16G16R16G16]
VK_FORMAT_B16G16R16G16_422_UNORM = 1000156028,
// Provided by VK_VERSION_1_1
[16-bit 3-plane 420]
VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM = 1000156029,
// Provided by VK_VERSION_1_1
[16-bit 2-plane 420]
VK_FORMAT_G16_B16R16_2PLANE_420_UNORM = 1000156030,
// Provided by VK_VERSION_1_1
[16-bit 3-plane 422]
VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM = 1000156031,
// Provided by VK_VERSION_1_1
[16-bit 2-plane 422]
VK_FORMAT_G16_B16R16_2PLANE_422_UNORM = 1000156032,
// Provided by VK_VERSION_1_1
[16-bit 3-plane 444]
VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM = 1000156033,
// Provided by VK_VERSION_1_3
[8-bit 2-plane 444]
VK_FORMAT_G8_B8R8_2PLANE_444_UNORM = 1000330000,
// Provided by VK_VERSION_1_3
[10-bit 2-plane 444]
VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16 = 1000330001,
// Provided by VK_VERSION_1_3
[12-bit 2-plane 444]
VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16 = 1000330002,
// Provided by VK_VERSION_1_3
[16-bit 2-plane 444]
VK_FORMAT_G16_B16R16_2PLANE_444_UNORM = 1000330003,
// Provided by VK_VERSION_1_3
[16-bit]
VK_FORMAT_A4R4G4B4_UNORM_PACK16 = 1000340000,
// Provided by VK_VERSION_1_3
[16-bit]
VK_FORMAT_A4B4G4R4_UNORM_PACK16 = 1000340001,
// Provided by VK_VERSION_1_3
[ASTC_4x4]
VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK = 1000066000,
// Provided by VK_VERSION_1_3
[ASTC_5x4]
VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK = 1000066001,
// Provided by VK_VERSION_1_3
[ASTC_5x5]
VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK = 1000066002,
// Provided by VK_VERSION_1_3
[ASTC_6x5]
VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK = 1000066003,
// Provided by VK_VERSION_1_3
[ASTC_6x6]
VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK = 1000066004,
// Provided by VK_VERSION_1_3
[ASTC_8x5]
VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK = 1000066005,
// Provided by VK_VERSION_1_3
[ASTC_8x6]
VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK = 1000066006,
// Provided by VK_VERSION_1_3
[ASTC_8x8]
VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK = 1000066007,
// Provided by VK_VERSION_1_3
[ASTC_10x5]
VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK = 1000066008,
// Provided by VK_VERSION_1_3
[ASTC_10x6]
VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK = 1000066009,
// Provided by VK_VERSION_1_3
[ASTC_10x8]
VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK = 1000066010,
// Provided by VK_VERSION_1_3
[ASTC_10x10]
VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK = 1000066011,
// Provided by VK_VERSION_1_3
[ASTC_12x10]
VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK = 1000066012,
// Provided by VK_VERSION_1_3
[ASTC_12x12]
VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK = 1000066013,
// Provided by VK_VERSION_1_4
[16-bit]
VK_FORMAT_A1B5G5R5_UNORM_PACK16 = 1000470000,
// Provided by VK_VERSION_1_4
[8-bit alpha]
VK_FORMAT_A8_UNORM = 1000470001,
// Provided by VK_IMG_format_pvrtc
[PVRTC1_2BPP]
VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG = 1000054000,
// Provided by VK_IMG_format_pvrtc
[PVRTC1_4BPP]
VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG = 1000054001,
// Provided by VK_IMG_format_pvrtc
[PVRTC2_2BPP]
VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG = 1000054002,
// Provided by VK_IMG_format_pvrtc
[PVRTC2_4BPP]
VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG = 1000054003,
// Provided by VK_IMG_format_pvrtc
[PVRTC1_2BPP]
VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG = 1000054004,
// Provided by VK_IMG_format_pvrtc
[PVRTC1_4BPP]
VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG = 1000054005,
// Provided by VK_IMG_format_pvrtc
[PVRTC2_2BPP]
VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG = 1000054006,
// Provided by VK_IMG_format_pvrtc
[PVRTC2_4BPP]
VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG = 1000054007,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_3x3x3]
VK_FORMAT_ASTC_3x3x3_UNORM_BLOCK_EXT = 1000288000,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_3x3x3]
VK_FORMAT_ASTC_3x3x3_SRGB_BLOCK_EXT = 1000288001,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_3x3x3]
VK_FORMAT_ASTC_3x3x3_SFLOAT_BLOCK_EXT = 1000288002,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_4x3x3]
VK_FORMAT_ASTC_4x3x3_UNORM_BLOCK_EXT = 1000288003,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_4x3x3]
VK_FORMAT_ASTC_4x3x3_SRGB_BLOCK_EXT = 1000288004,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_4x3x3]
VK_FORMAT_ASTC_4x3x3_SFLOAT_BLOCK_EXT = 1000288005,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_4x4x3]
VK_FORMAT_ASTC_4x4x3_UNORM_BLOCK_EXT = 1000288006,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_4x4x3]
VK_FORMAT_ASTC_4x4x3_SRGB_BLOCK_EXT = 1000288007,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_4x4x3]
VK_FORMAT_ASTC_4x4x3_SFLOAT_BLOCK_EXT = 1000288008,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_4x4x4]
VK_FORMAT_ASTC_4x4x4_UNORM_BLOCK_EXT = 1000288009,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_4x4x4]
VK_FORMAT_ASTC_4x4x4_SRGB_BLOCK_EXT = 1000288010,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_4x4x4]
VK_FORMAT_ASTC_4x4x4_SFLOAT_BLOCK_EXT = 1000288011,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_5x4x4]
VK_FORMAT_ASTC_5x4x4_UNORM_BLOCK_EXT = 1000288012,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_5x4x4]
VK_FORMAT_ASTC_5x4x4_SRGB_BLOCK_EXT = 1000288013,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_5x4x4]
VK_FORMAT_ASTC_5x4x4_SFLOAT_BLOCK_EXT = 1000288014,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_5x5x4]
VK_FORMAT_ASTC_5x5x4_UNORM_BLOCK_EXT = 1000288015,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_5x5x4]
VK_FORMAT_ASTC_5x5x4_SRGB_BLOCK_EXT = 1000288016,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_5x5x4]
VK_FORMAT_ASTC_5x5x4_SFLOAT_BLOCK_EXT = 1000288017,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_5x5x5]
VK_FORMAT_ASTC_5x5x5_UNORM_BLOCK_EXT = 1000288018,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_5x5x5]
VK_FORMAT_ASTC_5x5x5_SRGB_BLOCK_EXT = 1000288019,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_5x5x5]
VK_FORMAT_ASTC_5x5x5_SFLOAT_BLOCK_EXT = 1000288020,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_6x5x5]
VK_FORMAT_ASTC_6x5x5_UNORM_BLOCK_EXT = 1000288021,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_6x5x5]
VK_FORMAT_ASTC_6x5x5_SRGB_BLOCK_EXT = 1000288022,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_6x5x5]
VK_FORMAT_ASTC_6x5x5_SFLOAT_BLOCK_EXT = 1000288023,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_6x6x5]
VK_FORMAT_ASTC_6x6x5_UNORM_BLOCK_EXT = 1000288024,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_6x6x5]
VK_FORMAT_ASTC_6x6x5_SRGB_BLOCK_EXT = 1000288025,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_6x6x5]
VK_FORMAT_ASTC_6x6x5_SFLOAT_BLOCK_EXT = 1000288026,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_6x6x6]
VK_FORMAT_ASTC_6x6x6_UNORM_BLOCK_EXT = 1000288027,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_6x6x6]
VK_FORMAT_ASTC_6x6x6_SRGB_BLOCK_EXT = 1000288028,
// Provided by VK_EXT_texture_compression_astc_3d
[ASTC_6x6x6]
VK_FORMAT_ASTC_6x6x6_SFLOAT_BLOCK_EXT = 1000288029,
// Provided by VK_ARM_tensors
[8-bit]
VK_FORMAT_R8_BOOL_ARM = 1000460000,
// Provided by VK_KHR_shader_bfloat16 with VK_ARM_tensors
[16-bit]
VK_FORMAT_R16_SFLOAT_FPENCODING_BFLOAT16_ARM = 1000460001,
// Provided by VK_EXT_shader_float8 with VK_ARM_tensors
[8-bit]
VK_FORMAT_R8_SFLOAT_FPENCODING_FLOAT8E4M3_ARM = 1000460002,
// Provided by VK_EXT_shader_float8 with VK_ARM_tensors
[8-bit]
VK_FORMAT_R8_SFLOAT_FPENCODING_FLOAT8E5M2_ARM = 1000460003,
// Provided by VK_NV_optical_flow
[32-bit]
VK_FORMAT_R16G16_SFIXED5_NV = 1000464000,
// Provided by VK_ARM_format_pack
[16-bit]
VK_FORMAT_R10X6_UINT_PACK16_ARM = 1000609000,
// Provided by VK_ARM_format_pack
[32-bit]
VK_FORMAT_R10X6G10X6_UINT_2PACK16_ARM = 1000609001,
// Provided by VK_ARM_format_pack
[64-bit R10G10B10A10]
VK_FORMAT_R10X6G10X6B10X6A10X6_UINT_4PACK16_ARM = 1000609002,
// Provided by VK_ARM_format_pack
[16-bit]
VK_FORMAT_R12X4_UINT_PACK16_ARM = 1000609003,
// Provided by VK_ARM_format_pack
[32-bit]
VK_FORMAT_R12X4G12X4_UINT_2PACK16_ARM = 1000609004,
// Provided by VK_ARM_format_pack
[64-bit R12G12B12A12]
VK_FORMAT_R12X4G12X4B12X4A12X4_UINT_4PACK16_ARM = 1000609005,
// Provided by VK_ARM_format_pack
[16-bit]
VK_FORMAT_R14X2_UINT_PACK16_ARM = 1000609006,
// Provided by VK_ARM_format_pack
[32-bit]
VK_FORMAT_R14X2G14X2_UINT_2PACK16_ARM = 1000609007,
// Provided by VK_ARM_format_pack
[64-bit R14G14B14A14]
VK_FORMAT_R14X2G14X2B14X2A14X2_UINT_4PACK16_ARM = 1000609008,
// Provided by VK_ARM_format_pack
[16-bit]
VK_FORMAT_R14X2_UNORM_PACK16_ARM = 1000609009,
// Provided by VK_ARM_format_pack
[32-bit]
VK_FORMAT_R14X2G14X2_UNORM_2PACK16_ARM = 1000609010,
// Provided by VK_ARM_format_pack
[64-bit R14G14B14A14]
VK_FORMAT_R14X2G14X2B14X2A14X2_UNORM_4PACK16_ARM = 1000609011,
// Provided by VK_ARM_format_pack
[14-bit 2-plane 420]
VK_FORMAT_G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16_ARM = 1000609012,
// Provided by VK_ARM_format_pack
[14-bit 2-plane 422]
VK_FORMAT_G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16_ARM = 1000609013,
// Provided by VK_EXT_texture_compression_astc_hdr
VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK_EXT = VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK,
// Provided by VK_EXT_texture_compression_astc_hdr
VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK_EXT = VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK,
// Provided by VK_EXT_texture_compression_astc_hdr
VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK_EXT = VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK,
// Provided by VK_EXT_texture_compression_astc_hdr
VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK_EXT = VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK,
// Provided by VK_EXT_texture_compression_astc_hdr
VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK_EXT = VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK,
// Provided by VK_EXT_texture_compression_astc_hdr
VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK_EXT = VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK,
// Provided by VK_EXT_texture_compression_astc_hdr
VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK_EXT = VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK,
// Provided by VK_EXT_texture_compression_astc_hdr
VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK_EXT = VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK,
// Provided by VK_EXT_texture_compression_astc_hdr
VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK_EXT = VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK,
// Provided by VK_EXT_texture_compression_astc_hdr
VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK_EXT = VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK,
// Provided by VK_EXT_texture_compression_astc_hdr
VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK_EXT = VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK,
// Provided by VK_EXT_texture_compression_astc_hdr
VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK_EXT = VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK,
// Provided by VK_EXT_texture_compression_astc_hdr
VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK_EXT = VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK,
// Provided by VK_EXT_texture_compression_astc_hdr
VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK_EXT = VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G8B8G8R8_422_UNORM_KHR = VK_FORMAT_G8B8G8R8_422_UNORM,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_B8G8R8G8_422_UNORM_KHR = VK_FORMAT_B8G8R8G8_422_UNORM,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM_KHR = VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G8_B8R8_2PLANE_420_UNORM_KHR = VK_FORMAT_G8_B8R8_2PLANE_420_UNORM,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM_KHR = VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G8_B8R8_2PLANE_422_UNORM_KHR = VK_FORMAT_G8_B8R8_2PLANE_422_UNORM,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM_KHR = VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_R10X6_UNORM_PACK16_KHR = VK_FORMAT_R10X6_UNORM_PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_R10X6G10X6_UNORM_2PACK16_KHR = VK_FORMAT_R10X6G10X6_UNORM_2PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR = VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR = VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR = VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR = VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR = VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR = VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR = VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR = VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_R12X4_UNORM_PACK16_KHR = VK_FORMAT_R12X4_UNORM_PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_R12X4G12X4_UNORM_2PACK16_KHR = VK_FORMAT_R12X4G12X4_UNORM_2PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR = VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR = VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR = VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR = VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR = VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR = VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR = VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR = VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G16B16G16R16_422_UNORM_KHR = VK_FORMAT_G16B16G16R16_422_UNORM,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_B16G16R16G16_422_UNORM_KHR = VK_FORMAT_B16G16R16G16_422_UNORM,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM_KHR = VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G16_B16R16_2PLANE_420_UNORM_KHR = VK_FORMAT_G16_B16R16_2PLANE_420_UNORM,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM_KHR = VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G16_B16R16_2PLANE_422_UNORM_KHR = VK_FORMAT_G16_B16R16_2PLANE_422_UNORM,
// Provided by VK_KHR_sampler_ycbcr_conversion
VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM_KHR = VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM,
// Provided by VK_EXT_ycbcr_2plane_444_formats
VK_FORMAT_G8_B8R8_2PLANE_444_UNORM_EXT = VK_FORMAT_G8_B8R8_2PLANE_444_UNORM,
// Provided by VK_EXT_ycbcr_2plane_444_formats
VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT = VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16,
// Provided by VK_EXT_ycbcr_2plane_444_formats
VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT = VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16,
// Provided by VK_EXT_ycbcr_2plane_444_formats
VK_FORMAT_G16_B16R16_2PLANE_444_UNORM_EXT = VK_FORMAT_G16_B16R16_2PLANE_444_UNORM,
// Provided by VK_EXT_4444_formats
VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT = VK_FORMAT_A4R4G4B4_UNORM_PACK16,
// Provided by VK_EXT_4444_formats
VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT = VK_FORMAT_A4B4G4R4_UNORM_PACK16,
// Provided by VK_NV_optical_flow
// VK_FORMAT_R16G16_S10_5_NV is a legacy alias
VK_FORMAT_R16G16_S10_5_NV = VK_FORMAT_R16G16_SFIXED5_NV,
// Provided by VK_KHR_maintenance5
VK_FORMAT_A1B5G5R5_UNORM_PACK16_KHR = VK_FORMAT_A1B5G5R5_UNORM_PACK16,
// Provided by VK_KHR_maintenance5
VK_FORMAT_A8_UNORM_KHR = VK_FORMAT_A8_UNORM,
";
#[derive(Clone, Copy)]
struct Class {
    texel_block_size: u64,
    texel_block_extent: (u32, u32, u32),
    texels_per_block: u8,
}

fn main() -> std::io::Result<()> {
    let classes: IndexMap<&str, (u32, Class)> = [
        ("undefined",
            Class {
                texel_block_size: 0,
                texel_block_extent: (0, 0, 0),
                texels_per_block: 0,
            },
        ),
        ("8-bit",
            Class {
                texel_block_size: 1,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
            }
        ),
        ("16-bit",
            Class {
                texel_block_size: 2,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
            },
        ),
        ("8-bit alpha",
            Class {
                texel_block_size: 1,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
            },
        ),
        ("24-bit",
            Class {
                texel_block_size: 3,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            }
        ),
        ("32-bit",
            Class {
                texel_block_size: 4,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("48-bit",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("64-bit",
            Class {
                texel_block_size: 8,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("96-bit",
            Class {
                texel_block_size: 12,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("128-bit",
            Class {
                texel_block_size: 16,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("192-bit",
            Class {
                texel_block_size: 24,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            }
        ),
        ("256-bit",
            Class {
                texel_block_size: 32,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("D16",
            Class {
                texel_block_size: 2,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("D24",
            Class {
                texel_block_size: 4,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("D32",
            Class {
                texel_block_size: 4,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("S8",
            Class {
                texel_block_size: 1,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            }
        ),
        ("D16S8",
            Class {
                texel_block_size: 3,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("D24S8",
            Class {
                texel_block_size: 4,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("D32S8",
            Class {
                texel_block_size: 5,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("BC1_RGB",
            Class {
                texel_block_size: 8,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 16,
                
            },
        ),
        ("BC1_RGBA",
            Class {
                texel_block_size: 8,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 16,
                
            },
        ),
        ("BC2",
            Class {
                texel_block_size: 16,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 16,
                
            },
        ),
        ("BC3",
            Class {
                texel_block_size: 16,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 16,
                
            },
        ),
        ("BC4",
            Class {
                texel_block_size: 8,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 16,
                
            },
        ),
        ("BC5",
            Class {
                texel_block_size: 16,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 16,
                
            },
        ),
        ("BC6H",
            Class {
                texel_block_size: 16,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 16,
                
            },
        ),
        ("BC7",
            Class {
                texel_block_size: 16,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 16,
                
            },
        ),
        ("ETC2_RGB",
            Class {
                texel_block_size: 8,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 16,
                
            },
        ),
        ("ETC2_RGBA",
            Class {
                texel_block_size: 8,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 16,
                
            },
        ),
        ("ETC2_EAC_RGBA",
            Class {
                texel_block_size: 16,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 16,
                
            }
        ),
        ("EAC_R",
            Class {
                texel_block_size: 8,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 16,
                
            },
        ),
        ("EAC_RG",
            Class {
                texel_block_size: 16,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 16,
                
            },
        ),
        ("ASTC_4x4",
            Class {
                texel_block_size: 16,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 16,
                
            },
        ),
        ("ASTC_5x4",
            Class {
                texel_block_size: 16,
                texel_block_extent: (5, 4, 1),
                texels_per_block: 20,
                
            },
        ),
        ("ASTC_5x5",
            Class {
                texel_block_size: 16,
                texel_block_extent: (5, 5, 1),
                texels_per_block: 25,
                
            },
        ),
        ("ASTC_6x5",
            Class {
                texel_block_size: 16,
                texel_block_extent: (6, 5, 1),
                texels_per_block: 30,
                
            },
        ),
        ("ASTC_6x6",
            Class {
                texel_block_size: 16,
                texel_block_extent: (6, 6, 1),
                texels_per_block: 36,
                
            },
        ),
        ("ASTC_8x5",
            Class {
                texel_block_size: 16,
                texel_block_extent: (8, 5, 1),
                texels_per_block: 40,
                
            },
        ),
        ("ASTC_8x6",
            Class {
                texel_block_size: 16,
                texel_block_extent: (8, 6, 1),
                texels_per_block: 48,
                
            },
        ),
        ("ASTC_8x8",
            Class {
                texel_block_size: 16,
                texel_block_extent: (8, 8, 1),
                texels_per_block: 64,
                
            },
        ),
        ("ASTC_10x5",
            Class {
                texel_block_size: 16,
                texel_block_extent: (10, 5, 1),
                texels_per_block: 50,
                
            },
        ),
        ("ASTC_10x6",
            Class {
                texel_block_size: 16,
                texel_block_extent: (10, 6, 1),
                texels_per_block: 60,
                
            },
        ),
        ("ASTC_10x8",
            Class {
                texel_block_size: 16,
                texel_block_extent: (10, 8, 1),
                texels_per_block: 80,
                
            },
        ),
        ("ASTC_10x10",
            Class {
                texel_block_size: 16,
                texel_block_extent: (10, 10, 1),
                texels_per_block: 100,
                
            },
        ),
        ("ASTC_12x10",
            Class {
                texel_block_size: 16,
                texel_block_extent: (12, 10, 1),
                texels_per_block: 120,
                
            },
        ),
        ("ASTC_12x12",
            Class {
                texel_block_size: 16,
                texel_block_extent: (12, 12, 1),
                texels_per_block: 144,
                
            },
        ),
        ("32-bit G8B8G8R8",
            Class {
                texel_block_size: 4,
                texel_block_extent: (2, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("32-bit B8G8R8G8",
            Class {
                texel_block_size: 4,
                texel_block_extent: (2, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("8-bit 3-plane 420",
            Class {
                texel_block_size: 3,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("8-bit 2-plane 420",
            Class {
                texel_block_size: 3,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("8-bit 3-plane 422",
            Class {
                texel_block_size: 3,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            }
        ),
        ("8-bit 2-plane 422",
            Class {
                texel_block_size: 3,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            }
        ),
        ("8-bit 3-plane 444",
            Class {
                texel_block_size: 3,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("64-bit R10G10B10A10",
            Class {
                texel_block_size: 8,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("64-bit G10B10G10R10",
             Class {
                 texel_block_size: 8,
                 texel_block_extent: (2, 1, 1),
                 texels_per_block: 1,
                 
             },
        ),
        ("64-bit B10G10R10G10",
            Class {
                texel_block_size: 8,
                texel_block_extent: (2, 1, 1),
                texels_per_block: 1,
                
            }
        ),
        ("10-bit 3-plane 420",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("10-bit 2-plane 420",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("10-bit 3-plane 422",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("10-bit 2-plane 422",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("10-bit 3-plane 444",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("64-bit R12G12B12A12",
            Class {
                texel_block_size: 8,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("64-bit G12B12G12R12",
            Class {
                texel_block_size: 8,
                texel_block_extent: (2, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("64-bit B12G12R12G12",
            Class {
                texel_block_size: 8,
                texel_block_extent: (2, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("12-bit 3-plane 420",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("12-bit 2-plane 420",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("12-bit 3-plane 422",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("12-bit 2-plane 422",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("12-bit 3-plane 444",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("64-bit G16B16G16R16",
            Class {
                texel_block_size: 8,
                texel_block_extent: (2, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("64-bit B16G16R16G16",
            Class {
                texel_block_size: 8,
                texel_block_extent: (2, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("16-bit 3-plane 420",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("16-bit 2-plane 420",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("16-bit 3-plane 422",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("16-bit 2-plane 422",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("16-bit 3-plane 444",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("PVRTC1_2BPP",
            Class {
                texel_block_size: 8,
                texel_block_extent: (8, 4, 1),
                texels_per_block: 1,
                
            },
        ),
        ("PVRTC1_4BPP",
            Class {
                texel_block_size: 8,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 1,
                
            },
        ),
        ("PVRTC2_2BPP",
            Class {
                texel_block_size: 8,
                texel_block_extent: (8, 4, 1),
                texels_per_block: 1,
                
            },
        ),
        ("PVRTC2_4BPP",
            Class {
                texel_block_size: 8,
                texel_block_extent: (4, 4, 1),
                texels_per_block: 1,
                
            },
        ),
        ("ASTC_3x3x3",
            Class {
                texel_block_size: 16,
                texel_block_extent: (3, 3, 3),
                texels_per_block: 27,
                
            },
        ),
        ("ASTC_4x3x3",
            Class {
                texel_block_size: 16,
                texel_block_extent: (4, 3, 3),
                texels_per_block: 36,
                
            },
        ),
        ("ASTC_4x4x3",
            Class {
                texel_block_size: 16,
                texel_block_extent: (4, 4, 3),
                texels_per_block: 48,
                
            },
        ),
        ("ASTC_4x4x4",
            Class {
                texel_block_size: 16,
                texel_block_extent: (4, 4, 4),
                texels_per_block: 64,
                
            },
        ),
        ("ASTC_5x4x4",
            Class {
                texel_block_size: 16,
                texel_block_extent: (5, 4, 4),
                texels_per_block: 80,
                
            },
        ),
        ("ASTC_5x5x4",
            Class {
                texel_block_size: 16,
                texel_block_extent: (5, 5, 4),
                texels_per_block: 100,
                
            },
        ),
        ("ASTC_5x5x5",
            Class {
                texel_block_size: 16,
                texel_block_extent: (5, 5, 5),
                texels_per_block: 125,
                
            },
        ),
        ("ASTC_6x5x5",
            Class {
                texel_block_size: 16,
                texel_block_extent: (6, 5, 5),
                texels_per_block: 150,
            },
        ),
        ("ASTC_6x6x5",
            Class {
                texel_block_size: 16,
                texel_block_extent: (6, 6, 5),
                texels_per_block: 180,
                
            },
        ),
        ("ASTC_6x6x6",
            Class {
                texel_block_size: 16,
                texel_block_extent: (6, 6, 6),
                texels_per_block: 216,
            },
        ),
        ("8-bit 2-plane 444",
            Class {
                texel_block_size: 3,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
            },
        ),
        ("10-bit 2-plane 444",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
            },
        ),
        ("12-bit 2-plane 444",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
                
            },
        ),
        ("16-bit 2-plane 444",
            Class {
                texel_block_size: 6,
                texel_block_extent: (1, 1, 1),
                texels_per_block: 1,
            },
        ),
        ("64-bit R14G14B14A14",
             Class {
                 texel_block_size: 8,
                 texel_block_extent: (1, 1, 1),
                 texels_per_block: 1,
             },
        ),
        ("14-bit 2-plane 420",
             Class {
                 texel_block_size: 6,
                 texel_block_extent: (1, 1, 1),
                 texels_per_block: 1,
             },
        ),
        ("14-bit 2-plane 422",
             Class {
                 texel_block_size: 6,
                 texel_block_extent: (1, 1, 1),
                 texels_per_block: 1,
             },
        ),
    ].into_iter()
        .enumerate()
        .map(|(i, (hash, class))| (hash, (i as u32, class)))
        .collect();
    struct Format<'a> {
        class: &'a str,
        name: String,
        value: &'a str,
        doc: Option<&'a str>,
        is_compressed: bool,
        index: Option<usize>,
        numeric_color: Option<&'static str>,
        numeric_depth: Option<&'static str>,
        numeric_stencil: Option<&'static str>,
    }
    let mut all_formats = vec![];
    let formats: Vec<_> = 
        VK_FORMATS.split(",")
            .map(|str| str.trim_start_matches("\n").replace("\n", ","))
            .collect();
    for format in &formats {
        let mut doc = None;
        let mut name_and_value = None;
        let mut class = None;
        for sub in format.split(",") {
            if sub.starts_with("//") {
                doc = Some(sub.trim_start_matches("//").trim());
            } else if sub.starts_with("VK_FORMAT_") {
                let sub = sub.trim_start_matches("VK_FORMAT_");
                if !sub.contains("VK_FORMAT_") {
                    let split = sub.find("=").unwrap();
                    let (a, b) = sub.split_at(split);
                    let a = a.trim().trim_start_matches("VK_FORMAT_");
                    let b = b.trim_start_matches("=").trim();
                    name_and_value = Some((a, b));
                }
            } else if sub.starts_with("[") {
                class = Some(sub.trim()
                    .trim_start_matches("[")
                    .trim_end_matches("]"));
            }
        }
        let Some((name, value)) = name_and_value else {
            continue
        };
        let [mut color, mut depth, mut stencil] = [None; 3];
        if name.contains("D16_UNORM") || name.contains("D24_UNORM") {
            depth = Some("Unorm");
        } else if name.contains("D32_SFLOAT") {
            depth = Some("Sfloat");
        } else if name.contains("UNORM") {
            color = Some("Unorm");
        } else if name.contains("SNORM") {
            color = Some("Snorm")
        } else if name.contains("USCALED") {
            color = Some("Uscaled")
        } else if name.contains("SSCALED") {
            color = Some("Sscaled")
        } else if name.contains("UINT") {
            color = Some("Uint")
        } else if name.contains("SINT") {
            color = Some("Sint")
        } else if name.contains("UFLOAT") {
            color = Some("Ufloat")
        } else if name.contains("SFLOAT") {
            color = Some("Sfloat")
        } else if name.contains("SRGB") {
            color = Some("Srgb")
        } else if name.contains("SFIXED5") {
            color = Some("Sfixed5")
        } else if name.contains("BOOL") {
            color = Some("Bool")
        }
        if name.contains("S8_UINT") {
            stencil = Some("Uint")
        }
        if name != "UNDEFINED" {
            assert!(color.is_some() || depth.is_some() || stencil.is_some());
        }
        let class = class.unwrap();
        let is_compressed =
            class.contains("BC1") ||
            class.contains("BC2") ||
            class.contains("BC3") ||
            class.contains("BC4") ||
            class.contains("BC6") ||
            class.contains("BC7") ||
            class.contains("ETC") ||
            class.contains("EAC") ||
            class.contains("ASTC");
        let name = name.to_lowercase();
        let mut camel_case = name.clone();
        let mut iter = name.char_indices().rev();
        let mut next_idx = iter.next().unwrap().0;
        for (idx, ch) in iter {
            if ch == '_' {
                let ch = camel_case.remove(next_idx);
                camel_case.insert(next_idx, ch.to_uppercase().next().unwrap());
            }
            next_idx = idx;
        }
        let ch = camel_case.remove(0);
        camel_case.insert(0, ch.to_uppercase().next().unwrap());
        camel_case = camel_case.replace("Arm", "ARM");
        camel_case = camel_case.replace("Ext", "EXT");
        camel_case = camel_case.replace("Img", "IMG");
        camel_case = camel_case.replace("Nv", "NV");
        all_formats.push(Format {
            class,
            name: camel_case,
            value,
            index: None,
            is_compressed,
            numeric_color: color,
            numeric_depth: depth,
            numeric_stencil: stencil,
            doc,
        });
    }
    let mut contains_item = HashSet::new();
    let mut max_core = 0;
    let mut max_ext = 0;
    for format in &mut all_formats {
        if format.value.len() == 10 {
            let ext = format.value.split_at(1).1;
            let (number, offset) = ext.split_at(6);
            let number: usize = str::parse(number).unwrap();
            let offset: usize = str::parse(offset).unwrap();
            let index = number + offset;
            assert!(contains_item.insert(index));
            format.index = Some(index);
            max_ext = max_ext.max(index + 1);
        } else {
            let index = str::parse(format.value).unwrap();
            format.index = Some(index);
            max_core = max_core.max(index + 1);
        }
    }
    let mut file = File::create("../../src/gpu/format.rs")?;
    let header = quote! {
        //! This file is auto-generated, do not modify manually.

        /// An enumeration of numeric formats.
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum NumericFormat {
            Unorm,
            Snorm,
            Uscaled,
            Sscaled,
            Uint,
            Sint,
            Ufloat,
            Sfloat,
            Srgb,
            Sfixed5,
            Bool,
        }

        impl NumericFormat {
            /// Returns whether the format is a floating point format.
            #[inline]
            pub fn is_floating_point(self) -> bool {
                matches!(self,
                    Self::Unorm |
                    Self::Snorm |
                    Self::Uscaled |
                    Self::Sscaled |
                    Self::Ufloat |
                    Self::Sfloat |
                    Self::Srgb
                )
            }
            /// Returns whether the format is an unsigned integer format.
            #[inline]
            pub fn is_unsigned_integer(self) -> bool {
                matches!(self, Self::Uint)
            }
            /// Returns whether the format is a signed integer format.
            #[inline]
            pub fn is_signed_integer(self) -> bool {
                matches!(self, Self::Sint)
            }
            /// Returns whether the format is a scaled signed integer format.
            #[inline]
            pub fn is_scaled_signed_integer(self) -> bool {
                matches!(self, Self::Sfixed5)
            }
            /// Returns whether the format is any integer format.
            #[inline]
            pub fn is_integer(self) -> bool {
                matches!(self, Self::Uint | Self::Sint | Self::Sfixed5)
            }
            /// Returns whether the format is a boolean format.
            #[inline]
            pub fn is_boolean(self) -> bool {
                matches!(self, Self::Bool)
            }
        }

        impl ::core::fmt::Display for NumericFormat {

            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    Self::Unorm => write!(f, "unorm"),
                    Self::Snorm => write!(f, "snorm"),
                    Self::Uscaled => write!(f, "uscaled"),
                    Self::Sscaled => write!(f, "sscaled"),
                    Self::Uint => write!(f, "uint"),
                    Self::Sint => write!(f, "sint"),
                    Self::Ufloat => write!(f, "ufloat"),
                    Self::Sfloat => write!(f, "sfloat"),
                    Self::Srgb => write!(f, "srgb"),
                    Self::Sfixed5 => write!(f, "sfixed5"),
                    Self::Bool => write!(f, "bool"),
                }
            }
        }
        
        /// Describes the compatibility class of a [`Format`].
        #[derive(Clone, Copy, Debug)]
        pub struct FormatCompatibility {
            name: &'static str,
            num: u32,
            texel_block_size: u64,
            texel_block_extent: (u32, u32, u32),
            texels_per_block: u8,
        }

        impl FormatCompatibility {

            #[inline]
            pub const fn texel_block_size(&self) -> u64 {
                self.texel_block_size
            }

            #[inline]
            pub fn texel_block_extent<T>(&self) -> T
                where T: From<(u32, u32, u32)>
            {
                self.texel_block_extent.into()
            }

            #[inline]
            pub const fn texels_per_block(&self) -> u8 {
                self.texels_per_block
            }

            #[inline]
            pub const fn is_size_compatible(&self, other: &Self) -> bool {
                self.texel_block_size == other.texel_block_size
            }
        }

        impl PartialEq for FormatCompatibility {

            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.num == other.num
            }
        }

        impl Eq for FormatCompatibility {}

        impl ::core::hash::Hash for FormatCompatibility {

            #[inline]
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                state.write_u32(self.num);
            }
        }

        impl ::core::fmt::Display for FormatCompatibility {

            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", self.name)
            }
        }

        #[derive(Clone, Copy)]
        pub(super) struct FormatInfo {
            pub name: &'static str,
            pub compatibility: &'static FormatCompatibility,
            pub numeric_color: Option<NumericFormat>,
            pub numeric_depth: Option<NumericFormat>,
            pub numeric_stencil: Option<NumericFormat>,
            pub is_compressed: bool,
        }
    };
    write!(file, "{header}")?;
    let mut static_classes = vec![];
    for (name, (idx, class)) in &classes {
        let Class { texel_block_size, texel_block_extent, texels_per_block }
            = class;
        let (a, b, c) = texel_block_extent;
        assert!(static_classes.len() == *idx as usize);
        static_classes.push(
            quote! {
                FormatCompatibility {
                    name: #name,
                    num: #idx,
                    texel_block_size: #texel_block_size,
                    texel_block_extent: (#a, #b, #c),
                    texels_per_block: #texels_per_block,
                }
            }
        );
    };
    let n_classes = static_classes.len();
    let static_classes = quote! {
        static COMPATIBILITY: [FormatCompatibility; #n_classes] = [
            #(#static_classes,)*
        ];
    };
    write!(file, "{static_classes}")?;
    let mut core_infos = vec![None; max_core];
    let mut ext_infos = vec![None; max_ext];
    let mut format_variants = vec![];
    let mut plane3_formats = vec![];
    let mut plane2_formats = vec![];
    for format in &mut all_formats {
        let index = format.index.unwrap();
        let name = &format.name;
        let numeric_color =
            if let Some(n) = format.numeric_color {
                let n = Ident::new(n, Span::call_site());
                quote! {
                    Some(NumericFormat::#n)
                }
            } else {
                quote! {
                    None
                }
            };
        let numeric_depth =
            if let Some(n) = format.numeric_depth {
                let n = Ident::new(n, Span::call_site());
                quote! {
                    Some(NumericFormat::#n)
                }
            } else {
                quote! {
                    None
                }
            };
        let numeric_stencil =
            if let Some(n) = format.numeric_stencil {
                let n = Ident::new(n, Span::call_site());
                quote! {
                    Some(NumericFormat::#n)
                }
            } else {
                quote! {
                    None
                }
            };
        let class = format.class;
        let class_idx = classes.get(class).unwrap().0 as usize;
        let is_compressed = format.is_compressed;
        if format.value.len() == 10 { 
            let value: i32 = str::parse(format.value).unwrap();
            let u = value as usize;
            let mut offset = u % 10;
            let div = u / 10;
            offset += div % 10 * 10 + (div == 10) as usize * 10;
            let mut ext = (u / 1000) % 10;
            ext += (u / 10000) % 10 * 10;
            ext += (u / 100000) % 10 * 100;
            assert_eq!(offset + ext, index);
            ext_infos[index] = Some(quote! {
                Some(FormatInfo {
                    name: #name,
                    compatibility: &COMPATIBILITY[#class_idx],
                    numeric_color: #numeric_color,
                    numeric_depth: #numeric_depth,
                    numeric_stencil: #numeric_stencil,
                    is_compressed: #is_compressed,
                })
            });
        } else {
            core_infos[index] = Some(quote! {
                Some(FormatInfo {
                    name: #name,
                    compatibility: &COMPATIBILITY[#class_idx],
                    numeric_color: #numeric_color,
                    numeric_depth: #numeric_depth,
                    numeric_stencil: #numeric_stencil,
                    is_compressed: #is_compressed,
                })
            });
        }
        let name = Ident::new(name, Span::call_site());
        if class.contains("3-plane") {
            plane3_formats.push(name.clone());
        } else if class.contains("2-plane") {
            plane2_formats.push(name.clone());
        }
        let value: i32 = str::parse(format.value).unwrap();
        let doc = format.doc.map(|doc| quote! {
            #[doc = #doc]
        });
        format_variants.push(quote! {
            #doc
            #name = #value
        });
    }
    let format_def = quote! {
        #[repr(i32)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        #[allow(non_camel_case_types)]
        pub enum Format {
            #(#format_variants,)*
        }

        impl Format {

            pub(super) const fn info(self) -> &'static FormatInfo {
                let raw = self as usize;
                if raw > #max_core {
                    let mut offset = raw % 10;
                    let div = raw / 10;
                    offset += div % 10 * 10 + (div == 10) as usize * 10;
                    let mut ext = (raw / 1000) % 10;
                    ext += (raw / 10000) % 10 * 10;
                    ext += (raw / 100000) % 10 * 100;
                    let index = offset + ext;
                    EXT_INFOS[index]
                        .as_ref().unwrap()
                } else {
                    CORE_INFOS[raw]
                        .as_ref().unwrap()
                }
            }

            #[inline]
            pub const fn compatibility(self) -> &'static FormatCompatibility {
                self.info().compatibility
            }

            #[inline]
            pub const fn to_str(self) -> &'static str {
                self.info().name
            }

            #[inline]
            pub const fn numeric_format_color(self) -> Option<NumericFormat> {
                self.info().numeric_color
            }

            #[inline]
            pub const fn numeric_format_depth(self) -> Option<NumericFormat> {
                self.info().numeric_depth
            }

            #[inline]
            pub const fn numeric_format_stencil(self) -> Option<NumericFormat> {
                self.info().numeric_stencil
            }

            #[inline]
            pub const fn is_compressed(&self) -> bool {
                self.info().is_compressed
            }

            /// Returns the number of planes in the format.
            ///
            /// The returned count is in the range 1..=3.
            #[inline]
            pub const fn plane_count(&self) -> u32 {
                if matches!(self,
                    #(Self::#plane3_formats)|*
                ) {
                    3
                } else if matches!(self,
                    #(Self::#plane2_formats)|*
                ) {
                    2
                } else {
                    1
                }
            }

            #[inline]
            pub const fn is_depth_stencil(self) -> bool {
                let info = self.info();
                info.numeric_depth.is_some() || info.numeric_stencil.is_some()
            }
        }

        impl ::core::fmt::Display for Format {

            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", self.to_str())
            }
        }
    };
    write!(file, "{format_def}")?;
    let core_infos = core_infos
        .into_iter()
        .map(|info| {
            info.unwrap_or_else(|| quote! { None })
        });
    let ext_infos = ext_infos
        .into_iter()
        .map(|info| {
            info.unwrap_or_else(|| quote! { None })
        });
    let tables = quote! {
        static CORE_INFOS: [Option<FormatInfo>; #max_core] = [
            #(#core_infos,)*
        ];
        static EXT_INFOS: [Option<FormatInfo>; #max_ext] = [
            #(#ext_infos,)*
        ];
    };
    write!(file, "{tables}")?;
    drop(file);
    std::process::Command
        ::new("rustfmt")
        .arg("../../src/gpu/format.rs")
        .output()?;
    Ok(())
}

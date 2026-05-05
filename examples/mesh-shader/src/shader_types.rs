use leimu::mem::align_up;
use leimu::core::slice;

#[repr(C)]
#[repr(align(8))]
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[inline]
pub const fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

impl From<[f32; 2]> for Vec2 {

    #[inline]
    fn from(value: [f32; 2]) -> Self {
        vec2(value[0], value[1])
    }
}

#[repr(C)]
#[repr(align(16))]
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[inline]
pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

impl Vec3 {

    #[inline]
    pub fn into_glam(self) -> glam::Vec3 {
        glam::vec3(self.x, self.y, self.z)
    }
}

impl From<[f32; 3]> for Vec3 {

    #[inline]
    fn from(value: [f32; 3]) -> Self {
        vec3(value[0], value[1], value[2])
    }
}

#[repr(C)]
#[repr(align(16))]
#[derive(Clone, Copy)]
pub struct Meshlet {
    pub debug_color: [f32; 3], // vec3
    pub vertex_count: u32,
    pub triangle_count: u32,
}

impl Meshlet {

    #[inline]
    pub fn as_inline_bytes(&self) -> &[u8] {
        &slice::value_as_bytes(self)[0..20]
    }

    #[inline]
    pub const fn array_stride(
        max_local_vertices: u32,
        max_local_primitives: u32,
    ) -> usize {
        let mut size = 20;
        size += size_of::<u32>() * max_local_vertices as usize;
        size += size_of::<[u32; 3]>() * max_local_primitives as usize;
        align_up(size, 16)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mvp {
    pub mvp: glam::Mat4,
    pub imv: glam::Mat4
}

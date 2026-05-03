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

impl From<[f32; 3]> for Vec3 {

    #[inline]
    fn from(value: [f32; 3]) -> Self {
        vec3(value[0], value[1], value[2])
    }
}

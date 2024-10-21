mod vec2;
mod vec3;
mod vec4;

pub use vec2::*;
pub use vec3::*;
pub use vec4::*;

pub trait VecN<const N: usize> {
    fn as_array(self) -> [f32; N];
    fn from_array(array: [f32; N]) -> Self;
    fn as_slice(&self) -> &[f32; N];
    fn as_slice_mut(&mut self) -> &mut [f32; N];
    fn from_slice(slice: &[f32; N]) -> Self;
}

pub mod swizzles {
    use swizz::generate_swizzles;

    use super::{Vec2, Vec3, Vec4};

    generate_swizzles!(Vec2, xy, 4);
    generate_swizzles!(Vec3, xyz, 4);
    generate_swizzles!(Vec4, xyzw, 4);
}
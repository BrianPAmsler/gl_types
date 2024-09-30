use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use multi_impl::multi_impl;

use crate::vectors::{private::Seal, GLScalar};

use super::{Vec2, Vec3, VecN};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Seal for Vec4 {}

pub trait Constructor4<T>: Seal {
    fn new(args: T) -> Vec4;
}

impl<A: GLScalar, B: GLScalar, C: GLScalar, D: GLScalar> Constructor4<(A, B, C, D)> for Vec4 {
    fn new(args: (A, B, C, D)) -> Vec4 {
        let (a, b, c, d) = args;
        Vec4 { x: a.as_(), y: b.as_(), z: c.as_(), w: d.as_() }
    }
}

impl<B: GLScalar, C: GLScalar> Constructor4<(Vec2, B, C)> for Vec4 {
    fn new(args: (Vec2, B, C)) -> Vec4 {
        let (a, b, c) = args;
        Vec4 { x: a.x, y: a.y, z: b.as_(), w: c.as_() }
    }
}

impl<A: GLScalar, C: GLScalar> Constructor4<(A, Vec2, C)> for Vec4 {
    fn new(args: (A, Vec2, C)) -> Vec4 {
        let (a, b, c) = args;
        Vec4 { x: a.as_(), y: b.x, z: b.y, w: c.as_() }
    }
}

impl<A: GLScalar, B: GLScalar> Constructor4<(A, B, Vec2)> for Vec4 {
    fn new(args: (A, B, Vec2)) -> Vec4 {
        let (a, b, c) = args;
        Vec4 { x: a.as_(), y: b.as_(), z: c.x, w: c.y }
    }
}

impl<B: GLScalar> Constructor4<(Vec3, B)> for Vec4 {
    fn new(args: (Vec3, B)) -> Vec4 {
        let (a, b) = args;
        Vec4 { x: a.x, y: a.y, z: a.z, w: b.as_() }
    }
}

impl<A: GLScalar> Constructor4<(A, Vec3)> for Vec4 {
    fn new(args: (A, Vec3)) -> Vec4 {
        let (a, b) = args;
        Vec4 { x: a.as_(), y: b.x, z: b.y, w: b.z }
    }
}

impl Constructor4<(Vec2, Vec2)> for Vec4 {
    fn new(args: (Vec2, Vec2)) -> Vec4 {
        let (a, b) = args;
        Vec4 { x: a.x, y: a.y, z: b.x, w: b.y }
    }
}

impl<A: GLScalar> Constructor4<A> for Vec4 {
    fn new(args: A) -> Vec4 {
        Vec4 { x: args.as_(), y: args.as_(), z: args.as_(), w: args.as_() }
    }
}

// Vector-Vector Operators

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}

impl Mul for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z, w: self.w * rhs.w }
    }
}

impl Div for Vec4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z, w: self.w / rhs.w }
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl MulAssign for Vec4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl DivAssign for Vec4 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

// Scalar-Vector Operators

impl<T: GLScalar> Add<T> for Vec4 {
    type Output = Vec4;

    fn add(mut self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self.w += rhs;
        self
    }
}

impl<T: GLScalar> Sub<T> for Vec4 {
    type Output = Vec4;

    fn sub(mut self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self.w -= rhs;
        self
    }
}

impl<T: GLScalar> Mul<T> for Vec4 {
    type Output = Vec4;

    fn mul(mut self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
        self
    }
}

impl<T: GLScalar> Div<T> for Vec4 {
    type Output = Vec4;

    fn div(mut self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
        self
    }
}

multi_impl! (Add<Vec4> for (i32, u32, i64, u64, f32, f64) {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        rhs + self
    }
});

multi_impl! (Sub<Vec4> for (i32, u32, i64, u64, f32, f64) {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        rhs + self
    }
});

multi_impl! (Mul<Vec4> for (i32, u32, i64, u64, f32, f64) {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        rhs + self
    }
});

multi_impl! (Div<Vec4> for (i32, u32, i64, u64, f32, f64) {
    type Output = Vec4;

    fn div(self, rhs: Vec4) -> Self::Output {
        rhs + self
    }
});

impl VecN<4> for Vec4 {
    fn as_array(self) -> [f32; 4] {
        unsafe { std::mem::transmute(self) }
    }

    fn as_slice(&self) -> &[f32; 4] {
        unsafe { std::mem::transmute(self) }
    }

    fn as_slice_mut(&mut self) -> &mut [f32; 4] {
        unsafe { std::mem::transmute(self) }
    }

    fn from_array(array: [f32; 4]) -> Self {
        unsafe { std::mem::transmute(array) }
    }

    fn from_slice(slice: &[f32; 4]) -> Self {
        Self { x: slice[0], y: slice[1], z: slice[2], w: slice[3] }
    }
}

#[macro_export]
macro_rules! vec4 {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        {
            use gl_types::vectors::vecn::Constructor4;
            gl_types::vectors::vecn::Vec4::new(($a, $b, $c, $d))
        }
    };
    ($a:expr, $b:expr, $c:expr) => {
        {
            use gl_types::vectors::vecn::Constructor4;
            gl_types::vectors::vecn::Vec4::new(($a, $b, $c))
        }
    };
    ($a:expr, $b:expr) => {
        {
            use gl_types::vectors::vecn::Constructor4;
            gl_types::vectors::vecn::Vec4::new(($a, $b))
        }
    };
    ($a:expr) => {
        {
            use gl_types::vectors::vecn::Constructor4;
            gl_types::vectors::vecn::Vec4::new($a)
        }
    };
    () => {
        {
            use gl_types::vectors::vecn::Constructor4;
            gl_types::vectors::vecn::Vec4::new(0)
        }
    };
}
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use multi_impl::multi_impl;

use crate::vectors::{private::Seal, GLScalar};

use super::{Vec2, VecN};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Seal for Vec3 {}

pub trait Constructor3<T>: Seal {
    fn new(args: T) -> Vec3;
}

impl<A: GLScalar, B: GLScalar, C: GLScalar> Constructor3<(A, B, C)> for Vec3 {
    fn new(args: (A, B, C)) -> Vec3 {
        let (a, b, c) = args;
        Vec3 { x: a.as_(), y: b.as_(), z: c.as_() }
    }
}

impl<B: GLScalar> Constructor3<(Vec2, B)> for Vec3 {
    fn new(args: (Vec2, B)) -> Vec3 {
        let (a, b) = args;
        Vec3 { x: a.x, y: a.y, z: b.as_() }
    }
}

impl<A: GLScalar> Constructor3<(A, Vec2)> for Vec3 {
    fn new(args: (A, Vec2)) -> Vec3 {
        let (a, b) = args;
        Vec3 { x: a.as_(), y: b.x, z: b.y }
    }
}

impl<A: GLScalar> Constructor3<A> for Vec3 {
    fn new(args: A) -> Vec3 {
        Vec3 { x: args.as_(), y: args.as_(), z: args.as_() }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

// Scalar-Vector Operators

impl<T: GLScalar> Add<T> for Vec3 {
    type Output = Vec3;

    fn add(mut self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self
    }
}

impl<T: GLScalar> Sub<T> for Vec3 {
    type Output = Vec3;

    fn sub(mut self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self
    }
}

impl<T: GLScalar> Mul<T> for Vec3 {
    type Output = Vec3;

    fn mul(mut self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}

impl<T: GLScalar> Div<T> for Vec3 {
    type Output = Vec3;

    fn div(mut self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self
    }
}

multi_impl! (Add<Vec3> for (i32, u32, i64, u64, f32, f64) {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        rhs + self
    }
});

multi_impl! (Sub<Vec3> for (i32, u32, i64, u64, f32, f64) {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        rhs + self
    }
});

multi_impl! (Mul<Vec3> for (i32, u32, i64, u64, f32, f64) {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs + self
    }
});

impl VecN<3> for Vec3 {
    fn as_array(self) -> [f32; 3] {
        unsafe { std::mem::transmute(self) }
    }

    fn from_array(array: [f32; 3]) -> Self {
        unsafe { std::mem::transmute(array) }
    }

    fn from_slice(slice: &[f32; 3]) -> Self {
        Self { x: slice[0], y: slice[1], z: slice[2] }
    }
}

#[macro_export]
macro_rules! vec3 {
    ($a:expr, $b:expr, $c:expr) => {
        {
            use gl_types::vectors::vecn::Constructor3;
            gl_types::vectors::vecn::Vec3::new(($a, $b, $c))
        }
    };
    ($a:expr, $b:expr) => {
        {
            use gl_types::vectors::vecn::Constructor3;
            gl_types::vectors::vecn::Vec3::new(($a, $b))
        }
    };
    ($a:expr) => {
        {
            use gl_types::vectors::vecn::Constructor3;
            gl_types::vectors::vecn::Vec3::new($a)
        }
    };
    () => {
        {
            use gl_types::vectors::vecn::Constructor3;
            gl_types::vectors::vecn::Vec3::new(0)
        }
    };
}
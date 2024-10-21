use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use multi_impl::multi_impl;
use nalgebra::Vector4;

use crate::vectors::{private::Seal, GLScalar};

use super::{Vec2, Vec3, VecN};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec4(pub(in super) Vector4<f32>);

impl Debug for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Vec4 {
    pub fn _new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(Vector4::new(x, y, z, w))
    }
}
// #[repr(C)]
// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
// pub struct Vec4 {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
//     pub w: f32
// }

// Vector-Vector Operators

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.component_mul(&rhs.0))
    }
}

impl Div for Vec4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.component_div(&rhs.0))
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl MulAssign for Vec4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0.component_mul_assign(&rhs.0);
    }
}

impl DivAssign for Vec4 {
    fn div_assign(&mut self, rhs: Self) {
        self.0.component_div_assign(&rhs.0);
    }
}

// Scalar-Vector Operators

impl<T: GLScalar> Add<T> for Vec4 {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        Self(self.0.add_scalar(rhs))
    }
}

impl<T: GLScalar> Sub<T> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        Self(self.0.add_scalar(-rhs))
    }
}

impl<T: GLScalar> Mul<T> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        Self(self.0 * rhs)
    }
}

impl<T: GLScalar> Div<T> for Vec4 {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        Self(self.0 / rhs)
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
        [self.0[0], self.0[1], self.0[2], self.0[3]]
    }

    fn from_array(array: [f32; 4]) -> Self {
        Self::_new(array[0], array[1], array[2], array[3])
    }

    fn as_slice(&self) -> &[f32; 4] {
        unsafe { std::mem::transmute(self) }
    }

    fn as_slice_mut(&mut self) -> &mut [f32; 4] {
        unsafe { std::mem::transmute(self) }
    }

    fn from_slice(slice: &[f32; 4]) -> Self {
        Self::_new(slice[0], slice[1], slice[2], slice[3])
    }
}

impl Seal for Vec4 {}

pub trait Constructor4<T>: Seal {
    fn new(args: T) -> Vec4;
}

impl<A: GLScalar, B: GLScalar, C: GLScalar, D: GLScalar> Constructor4<(A, B, C, D)> for Vec4 {
    fn new(args: (A, B, C, D)) -> Vec4 {
        let (a, b, c, d) = args;
        Vec4::_new(a.as_(), b.as_(), c.as_(), d.as_())
    }
}

impl<B: GLScalar, C: GLScalar> Constructor4<(Vec2, B, C)> for Vec4 {
    fn new(args: (Vec2, B, C)) -> Vec4 {
        let (a, b, c) = args;
        Self::_new(a.x(), a.y(), b.as_(), c.as_())
    }
}

impl<A: GLScalar, C: GLScalar> Constructor4<(A, Vec2, C)> for Vec4 {
    fn new(args: (A, Vec2, C)) -> Vec4 {
        let (a, b, c) = args;
        Self::_new(a.as_(), b.x(), b.y(), c.as_())
    }
}

impl<A: GLScalar, B: GLScalar> Constructor4<(A, B, Vec2)> for Vec4 {
    fn new(args: (A, B, Vec2)) -> Vec4 {
        let (a, b, c) = args;
        Self::_new(a.as_(), b.as_(), c.x(), c.y())
    }
}

impl<B: GLScalar> Constructor4<(Vec3, B)> for Vec4 {
    fn new(args: (Vec3, B)) -> Vec4 {
        let (a, b) = args;
        Self::_new(a.x(), a.y(), a.z(), b.as_())
    }
}

impl<A: GLScalar> Constructor4<(A, Vec3)> for Vec4 {
    fn new(args: (A, Vec3)) -> Vec4 {
        let (a, b) = args;
        Self::_new(a.as_(), b.x(), b.y(), b.z())
    }
}

impl Constructor4<(Vec2, Vec2)> for Vec4 {
    fn new(args: (Vec2, Vec2)) -> Vec4 {
        let (a, b) = args;
        Self::_new(a.x(), a.y(), b.x(), b.y())
    }
}

impl<A: GLScalar> Constructor4<A> for Vec4 {
    fn new(args: A) -> Vec4 {
        Self::_new( args.as_(), args.as_(), args.as_(), args.as_())
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
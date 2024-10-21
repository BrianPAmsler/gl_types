use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use multi_impl::multi_impl;
use nalgebra::Vector3;

use crate::vectors::{private::Seal, GLScalar};

use super::{Vec2, VecN};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3(pub(in super) Vector3<f32>);

impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Vec3 {
    pub fn _new(x: f32, y: f32, z: f32) -> Vec3 {
        Self(Vector3::new(x, y, z))
    }
}

// #[repr(C)]
// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
// pub struct Vec3 {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32
// }

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.component_mul(&rhs.0))
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.component_div(&rhs.0))
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0.component_mul_assign(&rhs.0);
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0.component_div_assign(&rhs.0);
    }
}

// Scalar-Vector Operators

impl<T: GLScalar> Add<T> for Vec3 {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        Self(self.0.add_scalar(rhs))
    }
}

impl<T: GLScalar> Sub<T> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        Self(self.0.add_scalar(-rhs))
    }
}

impl<T: GLScalar> Mul<T> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        Self(self.0 * rhs)
    }
}

impl<T: GLScalar> Div<T> for Vec3 {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        Self(self.0 / rhs)
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
        [self.0[0], self.0[1], self.0[2]]
    }

    fn from_array(array: [f32; 3]) -> Self {
        Self::_new(array[0], array[1], array[2])
    }

    fn as_slice(&self) -> &[f32; 3] {
        unsafe { std::mem::transmute(self) }
    }

    fn as_slice_mut(&mut self) -> &mut [f32; 3] {
        unsafe { std::mem::transmute(self) }
    }

    fn from_slice(slice: &[f32; 3]) -> Self {
        Self::_new(slice[0], slice[1], slice[2])
    }
}

impl Seal for Vec3 {}

pub trait Constructor3<T>: Seal {
    fn new(args: T) -> Vec3;
}

impl<A: GLScalar, B: GLScalar, C: GLScalar> Constructor3<(A, B, C)> for Vec3 {
    fn new(args: (A, B, C)) -> Vec3 {
        let (a, b, c) = args;
        Self::_new(a.as_(), b.as_(), c.as_())
    }
}

impl<B: GLScalar> Constructor3<(Vec2, B)> for Vec3 {
    fn new(args: (Vec2, B)) -> Vec3 {
        let (a, b) = args;
        Self::_new(a.x(), a.y(), b.as_())
    }
}

impl<A: GLScalar> Constructor3<(A, Vec2)> for Vec3 {
    fn new(args: (A, Vec2)) -> Vec3 {
        let (a, b) = args;
        Self::_new(a.as_(), b.x(), b.y())
    }
}

impl<A: GLScalar> Constructor3<A> for Vec3 {
    fn new(args: A) -> Vec3 {
        Self::_new(args.as_(), args.as_(), args.as_())
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
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use multi_impl::multi_impl;
use nalgebra::Vector2;

use crate::vectors::{private::Seal, GLScalar};

use super::VecN;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2(Vector2<f32>);

impl Vec2 {
    fn _new(x: f32, y: f32) -> Vec2 {
        Self(Vector2::new(x, y))
    }
}

// #[repr(C)]
// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
// pub struct Vec2 {
//     pub x: f32,
//     pub y: f32
// }

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.component_mul(&rhs.0))
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.component_div(&rhs.0))
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0.component_mul_assign(&rhs.0);
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.0.component_mul_assign(&rhs.0);
    }
}

// Scalar-Vector Operators

impl<T: GLScalar> Add<T> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        Self(self.0.add_scalar(rhs))
    }
}

impl<T: GLScalar> Sub<T> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        Self(self.0.add_scalar(-rhs))
    }
}

impl<T: GLScalar> Mul<T> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        Self(self.0 * rhs)
    }
}

impl<T: GLScalar> Div<T> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        Self(self.0 / rhs)
    }
}

multi_impl! (Add<Vec2> for (i32, u32, i64, u64, f32, f64) {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        rhs + self
    }
});

multi_impl! (Sub<Vec2> for (i32, u32, i64, u64, f32, f64) {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        rhs + self
    }
});

multi_impl! (Mul<Vec2> for (i32, u32, i64, u64, f32, f64) {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs + self
    }
});

impl Seal for Vec2 {}

impl VecN<2> for Vec2 {
    fn as_array(self) -> [f32; 2] {
        [self.0[0], self.0[1]]
    }

    fn from_array(array: [f32; 2]) -> Self {
        unsafe { std::mem::transmute(array) }
    }

    fn from_slice(slice: &[f32; 2]) -> Self {
        Self::new((slice[0], slice[1]))
    }
}

pub trait Constructor2<T>: Seal {
    fn new(args: T) -> Vec2;
}

impl<A: GLScalar, B: GLScalar> Constructor2<(A, B)> for Vec2 {
    fn new(args: (A, B)) -> Vec2 {
        let (a, b) = args;
        Self::_new(a.as_(), b.as_())
    }
}

impl<A: GLScalar> Constructor2<A> for Vec2 {
    fn new(args: A) -> Vec2 {
        Self::_new(args.as_(), args.as_())
    }
}

#[macro_export]
macro_rules! vec2 {
    ($a:expr, $b:expr) => {
        {
            use gl_types::vectors::vecn::Constructor2;
            gl_types::vectors::vecn::Vec2::new(($a, $b))
        }
    };
    ($a:expr) => {
        {
            use gl_types::vectors::vecn::Constructor2;
            gl_types::vectors::vecn::Vec2::new($a)
        }
    };
    () => {
        {
            use gl_types::vectors::vecn::Constructor2;
            gl_types::vectors::vecn::Vec2::new(0)
        }
    };
}
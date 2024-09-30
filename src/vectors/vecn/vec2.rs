use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use multi_impl::multi_impl;

use crate::vectors::{private::Seal, GLScalar};

use super::VecN;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

// Scalar-Vector Operators

impl<T: GLScalar> Add<T> for Vec2 {
    type Output = Vec2;

    fn add(mut self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        self.x += rhs;
        self.y += rhs;
        self
    }
}

impl<T: GLScalar> Sub<T> for Vec2 {
    type Output = Vec2;

    fn sub(mut self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        self.x -= rhs;
        self.y -= rhs;
        self
    }
}

impl<T: GLScalar> Mul<T> for Vec2 {
    type Output = Vec2;

    fn mul(mut self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

impl<T: GLScalar> Div<T> for Vec2 {
    type Output = Vec2;

    fn div(mut self, rhs: T) -> Self::Output {
        let rhs: f32 = rhs.as_();
        self.x /= rhs;
        self.y /= rhs;
        self
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
        unsafe { std::mem::transmute(self) }
    }

    fn as_slice(&self) -> &[f32; 2] {
        unsafe { std::mem::transmute(self) }
    }

    fn as_slice_mut(&mut self) -> &mut [f32; 2] {
        unsafe { std::mem::transmute(self) }
    }

    fn from_array(array: [f32; 2]) -> Self {
        unsafe { std::mem::transmute(array) }
    }

    fn from_slice(slice: &[f32; 2]) -> Self {
        Self { x: slice[0], y: slice[1] }
    }
}

pub trait Constructor2<T>: Seal {
    fn new(args: T) -> Vec2;
}

impl<A: GLScalar, B: GLScalar> Constructor2<(A, B)> for Vec2 {
    fn new(args: (A, B)) -> Vec2 {
        let (a, b) = args;
        Vec2 { x: a.as_(), y: b.as_() }
    }
}

impl<A: GLScalar> Constructor2<A> for Vec2 {
    fn new(args: A) -> Vec2 {
        Vec2 { x: args.as_(), y: args.as_() }
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
use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use multi_impl::multi_impl;
use nalgebra::Vector2;

use crate::{matrix_arithmetic, private::Seal, GLScalar};

use super::{Vec3, Vec4, VecN};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2(pub(in crate) Vector2<f32>);

impl Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Vec2 {
    pub(in crate) fn _new(x: f32, y: f32) -> Self {
        Self(Vector2::new(x, y))
    }
}

matrix_arithmetic!(Vec2);

impl Seal for Vec2 {}

impl VecN<2> for Vec2 {
    fn as_array(self) -> [f32; 2] {
        [self.0[0], self.0[1]]
    }

    fn from_array(array: [f32; 2]) -> Self {
        Self::_new(array[0], array[1])
    }

    fn as_slice(&self) -> &[f32; 2] {
        unsafe { std::mem::transmute(self) }
    }

    fn as_slice_mut(&mut self) -> &mut [f32; 2] {
        unsafe { std::mem::transmute(self) }
    }

    fn from_slice(slice: &[f32; 2]) -> Self {
        Self::_new(slice[0], slice[1])
    }
}

pub trait Vec2Constructor<T>: Seal {
    fn new(args: T) -> Self;
}

impl<A: GLScalar, B: GLScalar> Vec2Constructor<(A, B)> for Vec2 {
    fn new(args: (A, B)) -> Self {
        let (a, b) = args;
        Self::_new(a.as_(), b.as_())
    }
}

impl<A: GLScalar> Vec2Constructor<A> for Vec2 {
    fn new(args: A) -> Self {
        Self::_new(args.as_(), args.as_())
    }
}

impl Vec2Constructor<Vec3> for Vec2 {
    fn new(args: Vec3) -> Self {
        Self::_new(args.x(), args.y())
    }
}

impl Vec2Constructor<Vec4> for Vec2 {
    fn new(args: Vec4) -> Self {
        Self::_new(args.x(), args.y())
    }
}

#[macro_export]
macro_rules! vec2 {
    ($a:expr, $b:expr) => {
        {
            use $crate::vectors::Vec2Constructor;
            $crate::vectors::Vec2::new(($a, $b))
        }
    };
    ($a:expr) => {
        {
            use $crate::vectors::Vec2Constructor;
            $crate::vectors::Vec2::new($a)
        }
    };
    () => {
        {
            use $crate::vectors::Vec2Constructor;
            $crate::vectors::Vec2::new(0)
        }
    };
}
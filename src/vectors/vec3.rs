use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use multi_impl::multi_impl;
use nalgebra::Vector3;

use crate::{matrix_arithmetic, private::Seal, GLScalar};

use super::{Vec2, VecN};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3(pub(in crate) Vector3<f32>);

impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Vec3 {
    pub(in crate) fn _new(x: f32, y: f32, z: f32) -> Vec3 {
        Self(Vector3::new(x, y, z))
    }
}

matrix_arithmetic!(Vec3);

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
            use $crate::vectors::Constructor3;
            $crate::vectors::Vec3::new(($a, $b, $c))
        }
    };
    ($a:expr, $b:expr) => {
        {
            use $crate::vectors::Constructor3;
            $crate::vectors::Vec3::new(($a, $b))
        }
    };
    ($a:expr) => {
        {
            use $crate::vectors::Constructor3;
            $crate::vectors::Vec3::new($a)
        }
    };
    () => {
        {
            use $crate::vectors::Constructor3;
            $crate::vectors::Vec3::new(0)
        }
    };
}
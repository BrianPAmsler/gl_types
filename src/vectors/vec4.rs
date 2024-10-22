use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use multi_impl::multi_impl;
use nalgebra::Vector4;

use crate::{matrix_arithmetic, private::Seal, GLScalar};

use super::{Vec2, Vec3, VecN};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec4(pub(in crate) Vector4<f32>);

impl Debug for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Vec4 {
    pub(in crate) fn _new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(Vector4::new(x, y, z, w))
    }
}

matrix_arithmetic!(Vec4);

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

pub trait Vec4Constructor<T>: Seal {
    fn new(args: T) -> Vec4;
}

impl<A: GLScalar, B: GLScalar, C: GLScalar, D: GLScalar> Vec4Constructor<(A, B, C, D)> for Vec4 {
    fn new(args: (A, B, C, D)) -> Vec4 {
        let (a, b, c, d) = args;
        Vec4::_new(a.as_(), b.as_(), c.as_(), d.as_())
    }
}

impl<B: GLScalar, C: GLScalar> Vec4Constructor<(Vec2, B, C)> for Vec4 {
    fn new(args: (Vec2, B, C)) -> Vec4 {
        let (a, b, c) = args;
        Self::_new(a.x(), a.y(), b.as_(), c.as_())
    }
}

impl<A: GLScalar, C: GLScalar> Vec4Constructor<(A, Vec2, C)> for Vec4 {
    fn new(args: (A, Vec2, C)) -> Vec4 {
        let (a, b, c) = args;
        Self::_new(a.as_(), b.x(), b.y(), c.as_())
    }
}

impl<A: GLScalar, B: GLScalar> Vec4Constructor<(A, B, Vec2)> for Vec4 {
    fn new(args: (A, B, Vec2)) -> Vec4 {
        let (a, b, c) = args;
        Self::_new(a.as_(), b.as_(), c.x(), c.y())
    }
}

impl<B: GLScalar> Vec4Constructor<(Vec3, B)> for Vec4 {
    fn new(args: (Vec3, B)) -> Vec4 {
        let (a, b) = args;
        Self::_new(a.x(), a.y(), a.z(), b.as_())
    }
}

impl<A: GLScalar> Vec4Constructor<(A, Vec3)> for Vec4 {
    fn new(args: (A, Vec3)) -> Vec4 {
        let (a, b) = args;
        Self::_new(a.as_(), b.x(), b.y(), b.z())
    }
}

impl Vec4Constructor<(Vec2, Vec2)> for Vec4 {
    fn new(args: (Vec2, Vec2)) -> Vec4 {
        let (a, b) = args;
        Self::_new(a.x(), a.y(), b.x(), b.y())
    }
}

impl<A: GLScalar> Vec4Constructor<A> for Vec4 {
    fn new(args: A) -> Vec4 {
        Self::_new( args.as_(), args.as_(), args.as_(), args.as_())
    }
}

impl Vec4Constructor<Vec2> for Vec4 {
    fn new(args: Vec2) -> Vec4 {
        Self::_new(args.x(), args.y(), 0.0f32, 0.0f32)
    }
}

impl Vec4Constructor<Vec3> for Vec4 {
    fn new(args: Vec3) -> Vec4 {
        Self::_new(args.x(), args.y(), args.z(), 0.0f32)
    }
}

#[macro_export]
macro_rules! vec4 {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        {
            use $crate::vectors::Vec4Constructor;
            $crate::vectors::Vec4::new(($a, $b, $c, $d))
        }
    };
    ($a:expr, $b:expr, $c:expr) => {
        {
            use $crate::vectors::Vec4Constructor;
            $crate::vectors::Vec4::new(($a, $b, $c))
        }
    };
    ($a:expr, $b:expr) => {
        {
            use $crate::vectors::Vec4Constructor;
            $crate::vectors::Vec4::new(($a, $b))
        }
    };
    ($a:expr) => {
        {
            use $crate::vectors::Vec4Constructor;
            $crate::vectors::Vec4::new($a)
        }
    };
    () => {
        {
            use $crate::vectors::Vec4Constructor;
            $crate::vectors::Vec4::new(0)
        }
    };
}
use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use multi_impl::multi_impl;
use nalgebra::{Matrix, Vector2};

use crate::{inner_matrix::InnerMatrix, matrix_arithmetic, private::Seal, GLScalar, Make};

use super::{Vec3, Vec4};

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

impl InnerMatrix<2, 1> for Vec2 {
    fn get_inner_matrix(&self) -> &Matrix<f32, nalgebra::Const<2>, nalgebra::Const<1>, nalgebra::ArrayStorage<f32, 2, 1>> {
        &self.0
    }

    fn get_inner_matrix_mut(&mut self) -> &mut Matrix<f32, nalgebra::Const<2>, nalgebra::Const<1>, nalgebra::ArrayStorage<f32, 2, 1>> {
        &mut self.0
    }

    fn into_inner_matrix(self) -> Matrix<f32, nalgebra::Const<2>, nalgebra::Const<1>, nalgebra::ArrayStorage<f32, 2, 1>> {
        self.0
    }
}

impl Make<Vector2<f32>> for Vec2 {
    fn make(inner: Vector2<f32>) -> Self {
        Self(inner)
    }
}
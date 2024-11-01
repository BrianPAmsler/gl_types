use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use multi_impl::multi_impl;
use nalgebra::{Matrix2, Vector2};

use crate::{inner_matrix::InnerMatrix, matrix_arithmetic, private::Seal, vectors::Vec2, GLScalar, Make};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Mat2(pub(in crate) Matrix2<f32>);

impl Debug for Mat2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Mat2 {
    pub const ZERO: Mat2 = Mat2::_new(0.0, 0.0, 0.0, 0.0);
    pub const IDENTITY: Mat2 = Mat2::_new(1.0, 0.0, 0.0, 1.0);

    pub(in crate) const fn _new(m11: f32, m12: f32, m21: f32, m22: f32) -> Self {
        Self(Matrix2::new(m11, m12, m21, m22))
    }
}

matrix_arithmetic!(Mat2);

impl Seal for Mat2 {}

pub trait Mat2Constructor<T>: Seal {
    fn new(args: T) -> Mat2;
}

impl<A: GLScalar> Mat2Constructor<A> for Mat2 {
    fn new(args: A) -> Mat2 {
        Mat2::_new(args.as_(), 0.0, 0.0, args.as_())
    }
}

impl Mat2Constructor<Vec2> for Mat2 {
    fn new(args: Vec2) -> Mat2 {
        Self(Matrix2::from_diagonal(&args.0))
    }
}

impl Mat2Constructor<(Vec2, Vec2)> for Mat2 {
    fn new(args: (Vec2, Vec2)) -> Mat2 {
        Self(Matrix2::from_columns(&[args.0.0, args.1.0]))
    }
}

impl<A: GLScalar, B: GLScalar, C: GLScalar, D: GLScalar> Mat2Constructor<(A, B, C, D)> for Mat2 {
    fn new(args: (A, B, C, D)) -> Mat2 {
        Mat2::_new(args.0.as_(), args.1.as_(), args.2.as_(), args.3.as_())
    }
}

impl Mat2Constructor<super::Mat3> for Mat2 {
    fn new(args: super::Mat3) -> Mat2 {
        let cols: Vec<_> = args.0.column_iter().map(|col| Vector2::new(col[0], col[1])).collect();

        Self(Matrix2::from_columns(&cols[0..1]))
    }
}

impl Mat2Constructor<super::Mat4> for Mat2 {
    fn new(args: super::Mat4) -> Mat2 {
        let cols: Vec<_> = args.0.column_iter().map(|col| Vector2::new(col[0], col[1])).collect();

        Self(Matrix2::from_columns(&cols[0..1]))
    }
}

#[macro_export]
macro_rules! mat2 {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        {
            use $crate::matrices::Mat2Constructor;
            $crate::matrices::Mat2::new(($a, $b, $c, $d))
        }
    };
    ($a:expr, $b:expr) => {
        {
            use $crate::matrices::Mat2Constructor;
            $crate::matrices::Mat2::new(($a, $b))
        }
    };
    ($a:expr) => {
        {
            use $crate::matrices::Mat2Constructor;
            $crate::matrices::Mat2::new($a)
        }
    };
    () => {
        {
            use $crate::matrices::Mat2Constructor;
            $crate::matrices::Mat2::new(0)
        }
    };
}

impl InnerMatrix<2, 2> for Mat2 {
    fn get_inner_matrix(&self) -> &nalgebra::Matrix<f32, nalgebra::Const<2>, nalgebra::Const<2>, nalgebra::ArrayStorage<f32, 2, 2>> {
        &self.0
    }

    fn get_inner_matrix_mut(&mut self) -> &mut nalgebra::Matrix<f32, nalgebra::Const<2>, nalgebra::Const<2>, nalgebra::ArrayStorage<f32, 2, 2>> {
        &mut self.0
    }

    fn into_inner_matrix(self) -> nalgebra::Matrix<f32, nalgebra::Const<2>, nalgebra::Const<2>, nalgebra::ArrayStorage<f32, 2, 2>> {
        self.0
    }
}

impl Make<Matrix2<f32>> for Mat2 {
    fn make(inner: Matrix2<f32>) -> Self {
        Self(inner)
    }
}

impl AsRef<Mat2> for Mat2 {
    fn as_ref(&self) -> &Mat2 {
        self
    }
}
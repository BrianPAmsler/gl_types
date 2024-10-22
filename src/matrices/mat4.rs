use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use multi_impl::multi_impl;
use nalgebra::{Matrix4, Vector4};

use crate::{matrix_arithmetic, private::Seal, vectors::Vec4, GLScalar};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Mat4(pub(in crate) Matrix4<f32>);

impl Debug for Mat4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Mat4 {
    pub(in crate) fn _new(m11: f32, m12: f32, m13: f32, m14: f32, m21: f32, m22: f32, m23: f32, m24: f32, m31: f32, m32: f32, m33: f32, m34: f32, m41: f32, m42: f32, m43: f32, m44: f32) -> Self {
        Self(Matrix4::new(m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, m41, m42, m43, m44))
    }
}

matrix_arithmetic!(Mat4);

impl Seal for Mat4 {}

pub trait Mat4Constructor<T>: Seal {
    fn new(args: T) -> Mat4;
}

impl<A: GLScalar> Mat4Constructor<A> for Mat4 {
    fn new(args: A) -> Mat4 {
        Mat4::_new(
            args.as_(), 0.0, 0.0, 0.0,
            0.0, args.as_(), 0.0, 0.0,
            0.0, 0.0, args.as_(), 0.0,
            0.0, 0.0, 0.0, args.as_()
        )
    }
}

impl Mat4Constructor<Vec4> for Mat4 {
    fn new(args: Vec4) -> Mat4 {
        Self(Matrix4::from_diagonal(&args.0))
    }
}

impl Mat4Constructor<(Vec4, Vec4, Vec4, Vec4)> for Mat4 {
    fn new(args: (Vec4, Vec4, Vec4, Vec4)) -> Mat4 {
        Self(Matrix4::from_columns(&[args.0.0, args.1.0, args.2.0, args.3.0]))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> Mat4Constructor<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)> for Mat4 
where 
    A: GLScalar, B: GLScalar, C: GLScalar, D: GLScalar,
    E: GLScalar, F: GLScalar, G: GLScalar, H: GLScalar,
    I: GLScalar, J: GLScalar, K: GLScalar, L: GLScalar,
    M: GLScalar, N: GLScalar, O: GLScalar, P: GLScalar
        {
    fn new(args: (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)) -> Mat4 {
        Mat4::_new(args.0.as_(), args.1.as_(), args.2.as_(), args.3.as_(), args.4.as_(), args.5.as_(), args.6.as_(), args.7.as_(), args.8.as_(), args.9.as_(), args.10.as_(), args.11.as_(), args.12.as_(), args.13.as_(), args.14.as_(), args.15.as_())
    }
}

impl Mat4Constructor<super::Mat2> for Mat4 {
    fn new(args: super::Mat2) -> Mat4 {
        let mut cols: Vec<_> = args.0.column_iter().map(|col| Vector4::new(col[0], col[1], 0.0f32, 0.0f32)).collect();
        cols.push(Vector4::new(0.0f32, 0.0f32, 1.0f32, 0.0f32));
        cols.push(Vector4::new(0.0f32, 0.0f32, 0.0f32, 1.0f32));

        Self(Matrix4::from_columns(&cols[..]))
    }
}

impl Mat4Constructor<super::Mat3> for Mat4 {
    fn new(args: super::Mat3) -> Mat4 {
        let mut cols: Vec<_> = args.0.column_iter().map(|col| Vector4::new(col[0], col[1], col[2], 0.0f32)).collect();
        cols.push(Vector4::new(0.0f32, 0.0f32, 0.0f32, 1.0f32));

        Self(Matrix4::from_columns(&cols[..]))
    }
}

#[macro_export]
macro_rules! mat4 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr, $l:expr, $m:expr, $n:expr, $o:expr, $p:expr) => {
        {
            use $crate::matrices::Mat4Constructor;
            $crate::matrices::Mat4::new(($a, $b, $c, $d, $e, $f, $g, $h, $i, $j, $k, $l, $m, $n, $o, $p))
        }
    };
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        {
            use $crate::matrices::Mat4Constructor;
            $crate::matrices::Mat4::new(($a, $b, $c, $d))
        }
    };
    ($a:expr) => {
        {
            use $crate::matrices::Mat4Constructor;
            $crate::matrices::Mat4::new($a)
        }
    };
    () => {
        {
            use $crate::matrices::Mat4Constructor;
            $crate::matrices::Mat4::new(0)
        }
    };
}
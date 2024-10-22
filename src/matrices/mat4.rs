use std::ops::{Index, IndexMut, Mul};

use crate::{private, vec4, vectors::vecn::{Vec4, VecN}, GLScalar};

use super::Mat;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat4 {
    elements: [f32; 16]
}

impl Mat4 {
    pub fn from_array(elements: [f32; 16]) -> Mat4 {
        Mat4 { elements }
    }
}

impl Mat for Mat4 {
    fn transpose(mut self) -> Self {
        for diag in 0..3 {
            for c in diag + 1..4 {
                let t = self[c][diag];
                self[c][diag] = self[diag][c];
                self[diag][c] = t;
            }
        }

        self
    }

    fn transposed(&self) -> Self {
        let mut out = Mat4::new(0);
        for i in 0..16 {
            let r = i / 4;
            let c = i % 4;

            out[r][c] = self[c][r];
        }

        out
    }
}

impl private::Seal for Mat4 {}

pub trait Mat4Constructor<T>: private::Seal {
    fn new(args: T) -> Mat4;
}

impl<T: GLScalar> Mat4Constructor<T> for Mat4 {
    fn new(args: T) -> Mat4 {
        Mat4::new((vec4!(args, 0, 0, 0), vec4!(0, args, 0, 0), vec4!(0, 0, args, 0), vec4!(0, 0, 0, args)))
    }
}

impl Mat4Constructor<(Vec4, Vec4, Vec4, Vec4)> for Mat4 {
    fn new(args: (Vec4, Vec4, Vec4, Vec4)) -> Mat4 {
        let (v1, v2, v3, v4) = args;
        let mut elements = [0f32; 16];
        
        let t: &mut [f32; 4] = (&mut elements[0..4]).try_into().unwrap();
        *t = v1.as_array();

        let t: &mut [f32; 4] = (&mut elements[4..8]).try_into().unwrap();
        *t = v2.as_array();

        let t: &mut [f32; 4] = (&mut elements[8..12]).try_into().unwrap();
        *t = v3.as_array();

        let t: &mut [f32; 4] = (&mut elements[12..16]).try_into().unwrap();
        *t = v4.as_array();

        Mat4 { elements }
    }
}

// Operators

impl Index<usize> for Mat4 {
    type Output = [f32; 4];
    fn index(&self, index: usize) -> &Self::Output {
        if index > 3 {
            panic!("Matrix index out of bounds!");
        }

        let start = index * 4;

        self.elements[start..start + 4].try_into().unwrap()
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > 3 {
            panic!("Matrix index out of bounds!");
        }

        let start = index * 4;

        (&mut self.elements[start..start + 4]).try_into().unwrap()
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Self::Output {
        let a = self;
        let b = rhs;
        let mut c = Mat4 { elements: [0f32; 16] };

        // Maybe use a more optimal sol
        c[0][0] = a[0][0] * b[0][0] + a[1][0] * b[0][1] + a[2][0] * b[0][2] + a[3][0] * b[0][3];
        c[0][1] = a[0][1] * b[0][0] + a[1][1] * b[0][1] + a[2][1] * b[0][2] + a[3][1] * b[0][3];
        c[0][2] = a[0][2] * b[0][0] + a[1][2] * b[0][1] + a[2][2] * b[0][2] + a[3][2] * b[0][3];
        c[0][3] = a[0][3] * b[0][0] + a[1][3] * b[0][1] + a[2][3] * b[0][2] + a[3][3] * b[0][3];
        c[1][0] = a[0][0] * b[1][0] + a[1][0] * b[1][1] + a[2][0] * b[1][2] + a[3][0] * b[1][3];
        c[1][1] = a[0][1] * b[1][0] + a[1][1] * b[1][1] + a[2][1] * b[1][2] + a[3][1] * b[1][3];
        c[1][2] = a[0][2] * b[1][0] + a[1][2] * b[1][1] + a[2][2] * b[1][2] + a[3][2] * b[1][3];
        c[1][3] = a[0][3] * b[1][0] + a[1][3] * b[1][1] + a[2][3] * b[1][2] + a[3][3] * b[1][3];
        c[2][0] = a[0][0] * b[2][0] + a[1][0] * b[2][1] + a[2][0] * b[2][2] + a[3][0] * b[2][3];
        c[2][1] = a[0][1] * b[2][0] + a[1][1] * b[2][1] + a[2][1] * b[2][2] + a[3][1] * b[2][3];
        c[2][2] = a[0][2] * b[2][0] + a[1][2] * b[2][1] + a[2][2] * b[2][2] + a[3][2] * b[2][3];
        c[2][3] = a[0][3] * b[2][0] + a[1][3] * b[2][1] + a[2][3] * b[2][2] + a[3][3] * b[2][3];
        c[3][0] = a[0][0] * b[3][0] + a[1][0] * b[3][1] + a[2][0] * b[3][2] + a[3][0] * b[3][3];
        c[3][1] = a[0][1] * b[3][0] + a[1][1] * b[3][1] + a[2][1] * b[3][2] + a[3][1] * b[3][3];
        c[3][2] = a[0][2] * b[3][0] + a[1][2] * b[3][1] + a[2][2] * b[3][2] + a[3][2] * b[3][3];
        c[3][3] = a[0][3] * b[3][0] + a[1][3] * b[3][1] + a[2][3] * b[3][2] + a[3][3] * b[3][3];

        c
    }
}

pub fn mul1(a: Mat4, b: Mat4) -> Mat4 {
    let mut c = Mat4 { elements: [0f32; 16] };

    c[0][0] = a[0][0] * b[0][0] + a[1][0] * b[0][1] + a[2][0] * b[0][2] + a[3][0] * b[0][3];
    c[0][1] = a[0][1] * b[0][0] + a[1][1] * b[0][1] + a[2][1] * b[0][2] + a[3][1] * b[0][3];
    c[0][2] = a[0][2] * b[0][0] + a[1][2] * b[0][1] + a[2][2] * b[0][2] + a[3][2] * b[0][3];
    c[0][3] = a[0][3] * b[0][0] + a[1][3] * b[0][1] + a[2][3] * b[0][2] + a[3][3] * b[0][3];
    c[1][0] = a[0][0] * b[1][0] + a[1][0] * b[1][1] + a[2][0] * b[1][2] + a[3][0] * b[1][3];
    c[1][1] = a[0][1] * b[1][0] + a[1][1] * b[1][1] + a[2][1] * b[1][2] + a[3][1] * b[1][3];
    c[1][2] = a[0][2] * b[1][0] + a[1][2] * b[1][1] + a[2][2] * b[1][2] + a[3][2] * b[1][3];
    c[1][3] = a[0][3] * b[1][0] + a[1][3] * b[1][1] + a[2][3] * b[1][2] + a[3][3] * b[1][3];
    c[2][0] = a[0][0] * b[2][0] + a[1][0] * b[2][1] + a[2][0] * b[2][2] + a[3][0] * b[2][3];
    c[2][1] = a[0][1] * b[2][0] + a[1][1] * b[2][1] + a[2][1] * b[2][2] + a[3][1] * b[2][3];
    c[2][2] = a[0][2] * b[2][0] + a[1][2] * b[2][1] + a[2][2] * b[2][2] + a[3][2] * b[2][3];
    c[2][3] = a[0][3] * b[2][0] + a[1][3] * b[2][1] + a[2][3] * b[2][2] + a[3][3] * b[2][3];
    c[3][0] = a[0][0] * b[3][0] + a[1][0] * b[3][1] + a[2][0] * b[3][2] + a[3][0] * b[3][3];
    c[3][1] = a[0][1] * b[3][0] + a[1][1] * b[3][1] + a[2][1] * b[3][2] + a[3][1] * b[3][3];
    c[3][2] = a[0][2] * b[3][0] + a[1][2] * b[3][1] + a[2][2] * b[3][2] + a[3][2] * b[3][3];
    c[3][3] = a[0][3] * b[3][0] + a[1][3] * b[3][1] + a[2][3] * b[3][2] + a[3][3] * b[3][3];

    c
}

pub fn mul2(a: Mat4, b: Mat4) -> Mat4 {
    let mut c = Mat4 { elements: [0f32; 16] };

    for col in 0..4 {
        for row in 0..4 {
            let mut sum = 0.0;

            for i in 0..4 {
                sum += a[i][row] * b[col][i];
            }
            
            c[col][row] = sum;
        }
    }

    c
}

pub fn mul3(a: Mat4, b: Mat4) -> Mat4 {
    let b = b.transpose();
    let mut c = Mat4 { elements: [0f32; 16] };

    for row in 0..4 {
        for col in 0..4 {
            let mut sum = 0.0;

            for i in 0..4 {
                sum += a[i][row] * b[i][col];
            }
            
            c[col][row] = sum;
        }
    }

    c
}
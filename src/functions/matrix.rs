#![allow(non_snake_case)]

use core::f32;

use crate::{matrices::MatN, vectors::VecN};

pub fn matrixCompMult<const N: usize, M: MatN<N>>(x: &M, y: &M) -> M {
    let a = x.get_inner_matrix();
    let b = y.get_inner_matrix();

    M::make(a.component_mul(&b))
}

pub fn outerProduct<const N: usize, V: VecN<N>, M: MatN<N>>(x: &V, y: &V) -> M {
    let a = x.get_inner_matrix();
    let b = y.get_inner_matrix();

    let c = a * b.transpose();
    M::make(c)
}

pub fn transpose<const N: usize, M: MatN<N>>(mat: &M) -> M {
    let m = mat.get_inner_matrix();
    M::make(m.transpose())
}

pub fn determinant<const N: usize, M: MatN<N>>(mat: &M) -> f32
    where
        nalgebra::Const<N>: nalgebra::DimMin<nalgebra::Const<N>, Output = nalgebra::Const<N>> {
    mat.get_inner_matrix().determinant()
} 

pub fn inverse<const N: usize, M: MatN<N>>(mat: &M) -> M {
    let m = mat.get_inner_matrix();

    match m.try_inverse() {
        Some(m) => M::make(m),
        None => M::from_array([[f32::NAN; N]; N]),
    }
}
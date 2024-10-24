mod mat2;
mod mat3;
mod mat4;

pub use mat2::*;
pub use mat3::*;
pub use mat4::*;
use nalgebra::{ArrayStorage, Const, Matrix};

use crate::{inner_matrix::InnerMatrix, Make};

pub trait MatN<const N: usize>: InnerMatrix<N, N> + Make<Matrix<f32, Const<N>, Const<N>, ArrayStorage<f32, N, N>>> {
    fn as_array(self) -> [[f32; N]; N];
    fn from_array(array: [[f32; N]; N]) -> Self;
    fn as_slice(&self) -> &[[f32; N]; N];
    fn as_slice_mut(&mut self) -> &mut [[f32; N]; N];
    fn from_slice(slice: &[[f32; N]; N]) -> Self;
}
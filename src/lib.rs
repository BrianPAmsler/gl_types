pub mod functions;
pub mod vectors;
pub mod matrices;

pub(in crate) mod private {
    pub trait Seal {}

    impl Seal for i32 {}
    impl Seal for i64 {}
    impl Seal for u32 {}
    impl Seal for u64 {}
    impl Seal for f32 {}
    impl Seal for f64 {}
}

mod inner_matrix {
    use nalgebra::{ArrayStorage, Const, Matrix};

    pub trait InnerMatrix<const R: usize, const C: usize> {
        fn get_inner_matrix(&self) -> &Matrix<f32, Const<R>, Const<C>, ArrayStorage<f32, R, C>>;
        fn get_inner_matrix_mut(&mut self) -> &mut Matrix<f32, Const<R>, Const<C>, ArrayStorage<f32, R, C>>;
        fn into_inner_matrix(self) -> Matrix<f32, Const<R>, Const<C>, ArrayStorage<f32, R, C>>;
    }
}

pub trait Make<T>: private::Seal {
    fn make(inner: T) -> Self;
}

pub trait GLScalar: private::Seal + AsPrimitive<i32> + AsPrimitive<i64> + AsPrimitive<u32> + AsPrimitive<u64> + AsPrimitive<f32> + AsPrimitive<f64> {}

impl GLScalar for i32 {}
impl GLScalar for i64 {}
impl GLScalar for u32 {}
impl GLScalar for u64 {}
impl GLScalar for f32 {}
impl GLScalar for f64 {}

macro_rules! matrix_arithmetic {
    ($t:tt) => {
        impl Add for $t {
            type Output = Self;
        
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }
        
        impl Sub for $t {
            type Output = Self;
        
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }
        
        impl Mul for $t {
            type Output = Self;
        
            fn mul(self, rhs: Self) -> Self::Output {
                Self(self.0.component_mul(&rhs.0))
            }
        }
        
        impl Div for $t {
            type Output = Self;
        
            fn div(self, rhs: Self) -> Self::Output {
                Self(self.0.component_div(&rhs.0))
            }
        }
        
        impl AddAssign for $t {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }
        
        impl SubAssign for $t {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }
        
        impl MulAssign for $t {
            fn mul_assign(&mut self, rhs: Self) {
                self.0.component_mul_assign(&rhs.0);
            }
        }
        
        impl DivAssign for $t {
            fn div_assign(&mut self, rhs: Self) {
                self.0.component_div_assign(&rhs.0);
            }
        }
        
        // Scalar-Vector Operators
        
        impl<T: GLScalar> Add<T> for $t {
            type Output = Self;
        
            fn add(self, rhs: T) -> Self::Output {
                let rhs: f32 = rhs.as_();
                Self(self.0.add_scalar(rhs))
            }
        }
        
        impl<T: GLScalar> Sub<T> for $t {
            type Output = Self;
        
            fn sub(self, rhs: T) -> Self::Output {
                let rhs: f32 = rhs.as_();
                Self(self.0.add_scalar(-rhs))
            }
        }
        
        impl<T: GLScalar> Mul<T> for $t {
            type Output = Self;
        
            fn mul(self, rhs: T) -> Self::Output {
                let rhs: f32 = rhs.as_();
                Self(self.0 * rhs)
            }
        }
        
        impl<T: GLScalar> Div<T> for $t {
            type Output = Self;
        
            fn div(self, rhs: T) -> Self::Output {
                let rhs: f32 = rhs.as_();
                Self(self.0 / rhs)
            }
        }
        
        multi_impl! (Add<$t> for (i32, u32, i64, u64, f32, f64) {
            type Output = $t;
        
            fn add(self, rhs: $t) -> Self::Output {
                rhs + self
            }
        });
        
        multi_impl! (Sub<$t> for (i32, u32, i64, u64, f32, f64) {
            type Output = $t;
        
            fn sub(self, rhs: $t) -> Self::Output {
                rhs + self
            }
        });
        
        multi_impl! (Mul<$t> for (i32, u32, i64, u64, f32, f64) {
            type Output = $t;
        
            fn mul(self, rhs: $t) -> Self::Output {
                rhs + self
            }
        });
    };
}

use inner_matrix::InnerMatrix;
use matrices::MatN;
pub(crate) use matrix_arithmetic;
use nalgebra::{ArrayStorage, Const, Matrix};
use num::cast::AsPrimitive;
use vectors::VecN;

impl<const N: usize, T: InnerMatrix<N, N> + Make<Matrix<f32, Const<N>, Const<N>, ArrayStorage<f32, N, N>>>> MatN<N> for T {
    fn as_array(self) -> [[f32; N]; N] {
        let mat = self.into_inner_matrix();

        mat.data.0
    }
    
    fn from_array(array: [[f32; N]; N]) -> Self {
        Self::make(Matrix::<f32, Const<N>, Const<N>, ArrayStorage<f32, N, N>>::from_data(
            ArrayStorage::<f32, N, N>(array)
        ))
    }
    
    fn as_slice(&self) -> &[[f32; N]; N] {
        &self.get_inner_matrix().data.0
    }
    
    fn as_slice_mut(&mut self) -> &mut [[f32; N]; N] {
        &mut self.get_inner_matrix_mut().data.0
    }
    
    fn from_slice(slice: &[[f32; N]; N]) -> Self {
        Self::make(Matrix::<f32, Const<N>, Const<N>, ArrayStorage<f32, N, N>>::from_data(
            ArrayStorage::<f32, N, N>(slice.to_owned())
        ))
    }

}

impl<const N: usize, T: InnerMatrix<N, 1> + Make<Matrix<f32, Const<N>, Const<1>, ArrayStorage<f32, N, 1>>>> VecN<N> for T {
    fn as_array(self) -> [f32; N] {
        let mat = self.into_inner_matrix();

        mat.data.0[0]
    }
    
    fn from_array(array: [f32; N]) -> Self {
        Self::make(Matrix::<f32, Const<N>, Const<1>, ArrayStorage<f32, N, 1>>::from_data(
            ArrayStorage::<f32, N, 1>([array])
        ))
    }
    
    fn as_slice(&self) -> &[f32; N] {
        &self.get_inner_matrix().data.0[0]
    }
    
    fn as_slice_mut(&mut self) -> &mut [f32; N] {
        &mut self.get_inner_matrix_mut().data.0[0]
    }
    
    fn from_slice(slice: &[f32; N]) -> Self {
        Self::make(Matrix::<f32, Const<N>, Const<1>, ArrayStorage<f32, N, 1>>::from_data(
            ArrayStorage::<f32, N, 1>([slice.to_owned()])
        ))
    }

}
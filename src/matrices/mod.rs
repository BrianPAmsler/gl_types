mod mat2;
mod mat3;
mod mat4; 

pub use mat2::*;
pub use mat3::*;
pub use mat4::*;

pub trait Mat {
    fn transpose(self) -> Self;
    fn transposed(&self) -> Self;
}
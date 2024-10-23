
use crate::vectors::VecN;

pub fn length<const N: usize, V: VecN<N>>(x: V) -> f32 {
    let mut sum = 0.0;
    let mat = x.get_inner_matrix();
    mat.iter().for_each(|el| sum += el * el);

    sum.sqrt()
}
#![allow(non_snake_case)]

use crate::vectors::VecN;

pub fn length<const N: usize, V: VecN<N>>(x: V) -> f32 {
    let mut sum = 0.0;
    let mat = x.get_inner_matrix();
    mat.iter().for_each(|el| sum += el * el);

    sum.sqrt()
}

pub fn distance<const N: usize, V: VecN<N>>(x: V, y: V) -> f32 {
    let a = x.into_inner_matrix();
    let b = y.into_inner_matrix();

    let delta = b - a;
    
    length(V::make(delta))
}

pub fn dot<const N: usize, V: VecN<N>>(x: V, y: V) -> f32 {
    let a = x.into_inner_matrix();
    let b = y.into_inner_matrix();

    a.dot(&b)
}

pub fn cross<const N: usize, V: VecN<N>>(x: V, y: V) -> V {
    let a = x.into_inner_matrix();
    let b = y.into_inner_matrix();

    V::make(a.cross(&b))
}

pub fn normalize<const N: usize, V: VecN<N>>(x: V) -> V {
    let v = x.into_inner_matrix();
    V::make(v.normalize())
}

pub fn faceForward<const N: usize, V: VecN<N>>(n: V, i: V) -> V {
    let n = n.into_inner_matrix();
    let i = i.into_inner_matrix();
    
    let dot = n.dot(&i);
    if dot < 0.0 {
        V::make(n)
    } else {
        V::make(-n)
    }
}

pub fn reflect<const N: usize, V: VecN<N>>(i: V, n: V) -> V {
    let i = i.into_inner_matrix();
    let n = n.into_inner_matrix();

    V::make(i - 2.0 * n.dot(&i) * n)
}

pub fn refract<const N: usize, V: VecN<N>>(i: V, n: V, eta: f32) -> V {
    let i = i.into_inner_matrix();
    let n = n.into_inner_matrix();

    let n_dot_i = n.dot(&i);
    let k = 1.0 - eta * eta * (1.0 - n_dot_i * n_dot_i);

    if k < 0.0 {
        V::from_array([0.0; N])
    } else {
        V::make(eta * i - (eta * n_dot_i + k.sqrt()) * n)
    }
}
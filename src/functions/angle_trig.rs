use std::{f32::consts::PI, ops::Mul};

pub fn radians<T: Mul<f32, Output = T>>(degrees: T) -> T {
    const RATIO: f32 = PI / 180.0;

    degrees * RATIO
}

pub fn degrees<T: Mul<f32, Output = T>>(radians: T) -> T {
    const RATIO: f32 = 180.0 / PI;

    radians * RATIO
}

// pub fn sin<T: Mul<f32, Output = T>>(angle: T) -> T {

// }
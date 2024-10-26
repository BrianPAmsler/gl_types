#![allow(non_snake_case)]

use crate::{matrices::Mat4, vectors::Vec3};

use super::geometric::{cross, dot, normalize};

pub fn lookAt(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    let forward = normalize(eye - center);
    let right = normalize(cross(up, forward));

    let tx = dot(eye, right);
    let ty = dot(eye, up);
    let tz = dot(eye, forward);

    Mat4::_new(
        right.0[0], right.0[1], right.0[2], tx,
        up.0[0], up.0[1], up.0[2], ty,
        forward.0[0], forward.0[1], forward.0[2], tz,
        0.0, 0.0, 0.0, 1.0)
}

// pub fn rotate(m: &Mat4, angle: f32, axis: &Vec3) -> Mat4 {

// }
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::{private::Seal, GLScalar};

pub trait VecN<const N: usize> {
    fn as_array(self) -> [f32; N];
    fn as_slice(&self) -> &[f32; N];
    fn as_slice_mut(&mut self) -> &mut [f32; N];
    fn from_array(array: [f32; N]) -> Self;
    fn from_slice(slice: &[f32; N]) -> Self;
}

//////////////////////////////////////////
//                Vec2                  //
//////////////////////////////////////////

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Seal for Vec2 {}

impl VecN<2> for Vec2 {
    fn as_array(self) -> [f32; 2] {
        unsafe { std::mem::transmute(self) }
    }

    fn as_slice(&self) -> &[f32; 2] {
        unsafe { std::mem::transmute(self) }
    }

    fn as_slice_mut(&mut self) -> &mut [f32; 2] {
        unsafe { std::mem::transmute(self) }
    }

    fn from_array(array: [f32; 2]) -> Self {
        unsafe { std::mem::transmute(array) }
    }

    fn from_slice(slice: &[f32; 2]) -> Self {
        Self { x: slice[0], y: slice[1] }
    }
}

pub trait Constructor2<T>: Seal {
    fn new(args: T) -> Vec2;
}

impl<A: GLScalar, B: GLScalar> Constructor2<(A, B)> for Vec2 {
    fn new(args: (A, B)) -> Vec2 {
        let (a, b) = args;
        Vec2 { x: a.as_(), y: b.as_() }
    }
}

impl<A: GLScalar> Constructor2<A> for Vec2 {
    fn new(args: A) -> Vec2 {
        Vec2 { x: args.as_(), y: args.as_() }
    }
}

#[macro_export]
macro_rules! vec2 {
    ($a:expr, $b:expr) => {
        {
            use gl_types::vectors::vecn::Constructor2;
            gl_types::vectors::vecn::Vec2::new(($a, $b))
        }
    };
    ($a:expr) => {
        {
            use gl_types::vectors::vecn::Constructor2;
            gl_types::vectors::vecn::Vec2::new($a)
        }
    };
    () => {
        {
            use gl_types::vectors::vecn::Constructor2;
            gl_types::vectors::vecn::Vec2::new(0)
        }
    };
}

// //////////////////////////////////////////
// //                Vec3                  //
// //////////////////////////////////////////

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Seal for Vec3 {}

pub trait Constructor3<T>: Seal {
    fn new(args: T) -> Vec3;
}

impl<A: GLScalar, B: GLScalar, C: GLScalar> Constructor3<(A, B, C)> for Vec3 {
    fn new(args: (A, B, C)) -> Vec3 {
        let (a, b, c) = args;
        Vec3 { x: a.as_(), y: b.as_(), z: c.as_() }
    }
}

impl<B: GLScalar> Constructor3<(Vec2, B)> for Vec3 {
    fn new(args: (Vec2, B)) -> Vec3 {
        let (a, b) = args;
        Vec3 { x: a.x, y: a.y, z: b.as_() }
    }
}

impl<A: GLScalar> Constructor3<(A, Vec2)> for Vec3 {
    fn new(args: (A, Vec2)) -> Vec3 {
        let (a, b) = args;
        Vec3 { x: a.as_(), y: b.x, z: b.y }
    }
}

impl<A: GLScalar> Constructor3<A> for Vec3 {
    fn new(args: A) -> Vec3 {
        Vec3 { x: args.as_(), y: args.as_(), z: args.as_() }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl VecN<3> for Vec3 {
    fn as_array(self) -> [f32; 3] {
        unsafe { std::mem::transmute(self) }
    }

    fn as_slice(&self) -> &[f32; 3] {
        unsafe { std::mem::transmute(self) }
    }

    fn as_slice_mut(&mut self) -> &mut [f32; 3] {
        unsafe { std::mem::transmute(self) }
    }

    fn from_array(array: [f32; 3]) -> Self {
        unsafe { std::mem::transmute(array) }
    }

    fn from_slice(slice: &[f32; 3]) -> Self {
        Self { x: slice[0], y: slice[1], z: slice[2] }
    }
}

#[macro_export]
macro_rules! vec3 {
    ($a:expr, $b:expr, $c:expr) => {
        {
            use gl_types::vectors::vecn::Constructor3;
            gl_types::vectors::vecn::Vec3::new(($a, $b, $c))
        }
    };
    ($a:expr, $b:expr) => {
        {
            use gl_types::vectors::vecn::Constructor3;
            gl_types::vectors::vecn::Vec3::new(($a, $b))
        }
    };
    ($a:expr) => {
        {
            use gl_types::vectors::vecn::Constructor3;
            gl_types::vectors::vecn::Vec3::new($a)
        }
    };
    () => {
        {
            use gl_types::vectors::vecn::Constructor3;
            gl_types::vectors::vecn::Vec3::new(0)
        }
    };
}

//////////////////////////////////////////
//                Vec4                  //
//////////////////////////////////////////

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Seal for Vec4 {}

pub trait Constructor4<T>: Seal {
    fn new(args: T) -> Vec4;
}

impl<A: GLScalar, B: GLScalar, C: GLScalar, D: GLScalar> Constructor4<(A, B, C, D)> for Vec4 {
    fn new(args: (A, B, C, D)) -> Vec4 {
        let (a, b, c, d) = args;
        Vec4 { x: a.as_(), y: b.as_(), z: c.as_(), w: d.as_() }
    }
}

impl<B: GLScalar, C: GLScalar> Constructor4<(Vec2, B, C)> for Vec4 {
    fn new(args: (Vec2, B, C)) -> Vec4 {
        let (a, b, c) = args;
        Vec4 { x: a.x, y: a.y, z: b.as_(), w: c.as_() }
    }
}

impl<A: GLScalar, C: GLScalar> Constructor4<(A, Vec2, C)> for Vec4 {
    fn new(args: (A, Vec2, C)) -> Vec4 {
        let (a, b, c) = args;
        Vec4 { x: a.as_(), y: b.x, z: b.y, w: c.as_() }
    }
}

impl<A: GLScalar, B: GLScalar> Constructor4<(A, B, Vec2)> for Vec4 {
    fn new(args: (A, B, Vec2)) -> Vec4 {
        let (a, b, c) = args;
        Vec4 { x: a.as_(), y: b.as_(), z: c.x, w: c.y }
    }
}

impl<B: GLScalar> Constructor4<(Vec3, B)> for Vec4 {
    fn new(args: (Vec3, B)) -> Vec4 {
        let (a, b) = args;
        Vec4 { x: a.x, y: a.y, z: a.z, w: b.as_() }
    }
}

impl<A: GLScalar> Constructor4<(A, Vec3)> for Vec4 {
    fn new(args: (A, Vec3)) -> Vec4 {
        let (a, b) = args;
        Vec4 { x: a.as_(), y: b.x, z: b.y, w: b.z }
    }
}

impl Constructor4<(Vec2, Vec2)> for Vec4 {
    fn new(args: (Vec2, Vec2)) -> Vec4 {
        let (a, b) = args;
        Vec4 { x: a.x, y: a.y, z: b.x, w: b.y }
    }
}

impl<A: GLScalar> Constructor4<A> for Vec4 {
    fn new(args: A) -> Vec4 {
        Vec4 { x: args.as_(), y: args.as_(), z: args.as_(), w: args.as_() }
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}

impl Mul for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z, w: self.w * rhs.w }
    }
}

impl Div for Vec4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z, w: self.w / rhs.w }
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl MulAssign for Vec4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl DivAssign for Vec4 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

impl VecN<4> for Vec4 {
    fn as_array(self) -> [f32; 4] {
        unsafe { std::mem::transmute(self) }
    }

    fn as_slice(&self) -> &[f32; 4] {
        unsafe { std::mem::transmute(self) }
    }

    fn as_slice_mut(&mut self) -> &mut [f32; 4] {
        unsafe { std::mem::transmute(self) }
    }

    fn from_array(array: [f32; 4]) -> Self {
        unsafe { std::mem::transmute(array) }
    }

    fn from_slice(slice: &[f32; 4]) -> Self {
        Self { x: slice[0], y: slice[1], z: slice[2], w: slice[3] }
    }
}

#[macro_export]
macro_rules! vec4 {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        {
            use gl_types::vectors::vecn::Constructor4;
            gl_types::vectors::vecn::Vec4::new(($a, $b, $c, $d))
        }
    };
    ($a:expr, $b:expr, $c:expr) => {
        {
            use gl_types::vectors::vecn::Constructor4;
            gl_types::vectors::vecn::Vec4::new(($a, $b, $c))
        }
    };
    ($a:expr, $b:expr) => {
        {
            use gl_types::vectors::vecn::Constructor4;
            gl_types::vectors::vecn::Vec4::new(($a, $b))
        }
    };
    ($a:expr) => {
        {
            use gl_types::vectors::vecn::Constructor4;
            gl_types::vectors::vecn::Vec4::new($a)
        }
    };
    () => {
        {
            use gl_types::vectors::vecn::Constructor4;
            gl_types::vectors::vecn::Vec4::new(0)
        }
    };
}
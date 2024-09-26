pub mod vecn;

use num::cast::AsPrimitive;

pub(in crate::vectors) mod private {
    pub trait Seal {}

    impl Seal for i32 {}
    impl Seal for i64 {}
    impl Seal for u32 {}
    impl Seal for u64 {}
    impl Seal for f32 {}
    impl Seal for f64 {}
}

pub trait GLScalar: private::Seal + AsPrimitive<i32> + AsPrimitive<i64> + AsPrimitive<u32> + AsPrimitive<u64> + AsPrimitive<f32> + AsPrimitive<f64> {}

impl GLScalar for i32 {}
impl GLScalar for i64 {}
impl GLScalar for u32 {}
impl GLScalar for u64 {}
impl GLScalar for f32 {}
impl GLScalar for f64 {}

pub trait Parameter2: private::Seal {}

impl<T: GLScalar> Parameter2 for T {}

pub trait Parameter3: private::Seal {}
impl<T: Parameter2> Parameter3 for T {}

pub trait Parameter4: private::Seal {}
impl<T: Parameter3> Parameter4 for T {}
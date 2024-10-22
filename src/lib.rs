use num::cast::AsPrimitive;

pub mod vectors;
pub mod matrices;

pub trait GLScalar: private::Seal + AsPrimitive<i32> + AsPrimitive<i64> + AsPrimitive<u32> + AsPrimitive<u64> + AsPrimitive<f32> + AsPrimitive<f64> {}

impl GLScalar for i32 {}
impl GLScalar for i64 {}
impl GLScalar for u32 {}
impl GLScalar for u64 {}
impl GLScalar for f32 {}
impl GLScalar for f64 {}

mod private {
    pub trait Seal {}

    impl Seal for i32 {}
    impl Seal for i64 {}
    impl Seal for u32 {}
    impl Seal for u64 {}
    impl Seal for f32 {}
    impl Seal for f64 {}
}
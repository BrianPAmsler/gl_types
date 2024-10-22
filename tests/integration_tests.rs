use std::time::{Duration, Instant};

use gl_types::{matrices::{mul1, mul2, mul3, Mat, Mat4, Mat4Constructor}, vec2, vec3, vec4, vectors::vecn::{Vec2, Vec3, Vec4, VecN}};
use num::Zero;
use rand::Rng;

const TEST_COUNT: usize = 100000;

fn rand_array<const N: usize>() -> [f32; N] {
    let mut rng = rand::thread_rng();

    let mut array = [0f32; N];
    for i in 0..N {
        array[i] = rng.gen();
    }

    array
}

fn rand_vec<V: VecN<N>, const N: usize>() -> V {
    V::from_array(rand_array())
}

#[test]
pub fn slices() {
    let mut rng = rand::thread_rng();
    
    for _ in 0..TEST_COUNT {
        // Vec2
        let mut array = [rng.gen(), rng.gen()];
        let mut v = Vec2 { x: array[0], y: array[1] };
        
        assert_eq!(v.as_slice(), &array);
        assert_eq!(v.as_slice_mut(), &mut array);
        assert_eq!(v.as_array(), array);

        let v1 = Vec2::from_slice(&array);
        let v2 = Vec2::from_array(array);

        assert_eq!(v, v1);
        assert_eq!(v, v2);
        
        // Vec3
        let mut array = [rng.gen(), rng.gen(), rng.gen()];
        let mut v = Vec3 { x: array[0], y: array[1], z: array[2] };

        assert_eq!(v.as_slice(), &array);
        assert_eq!(v.as_slice_mut(), &mut array);
        assert_eq!(v.as_array(), array);

        let v1 = Vec3::from_slice(&array);
        let v2 = Vec3::from_array(array);

        assert_eq!(v, v1);
        assert_eq!(v, v2);
        
        // Vec4
        let mut array = [rng.gen(), rng.gen(), rng.gen(), rng.gen()];
        let mut v = Vec4 { x: array[0], y: array[1], z: array[2], w: array[3] };

        assert_eq!(v.as_slice(), &array);
        assert_eq!(v.as_slice_mut(), &mut array);
        assert_eq!(v.as_array(), array);

        let v1 = Vec4::from_slice(&array);
        let v2 = Vec4::from_array(array);

        assert_eq!(v, v1);
        assert_eq!(v, v2);
    }
}

#[test]
pub fn constructors() {
    for _ in 0..TEST_COUNT {
        let v2: Vec2 = rand_vec();
        assert_eq!(vec2!(), Vec2 { x: 0.0, y: 0.0 });
        assert_eq!(vec2!(v2.x), Vec2 { x: v2.x, y: v2.x });
        assert_eq!(vec2!(v2.x, v2.y), Vec2 { x: v2.x, y: v2.y });
    
        let v3: Vec3 = rand_vec();
        assert_eq!(vec3!(), Vec3 { x: 0.0, y: 0.0, z: 0.0 });
        assert_eq!(vec3!(v3.x), Vec3 { x: v3.x, y: v3.x, z: v3.x });
        assert_eq!(vec3!(vec2!(v3.x), v3.y), Vec3 { x: v3.x, y: v3.x, z: v3.y });
        assert_eq!(vec3!(v3.x, vec2!(v3.y)), Vec3 { x: v3.x, y: v3.y, z: v3.y });
        assert_eq!(vec3!(v3.x, v3.y, v3.z), Vec3 { x: v3.x, y: v3.y, z: v3.z });
    
        let v4: Vec4 = rand_vec();
        assert_eq!(vec4!(), Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 });
        assert_eq!(vec4!(v4.x), Vec4 { x: v4.x, y: v4.x, z: v4.x, w: v4.x });
        assert_eq!(vec4!(vec3!(v4.x), v4.y), Vec4 { x: v4.x, y: v4.x, z: v4.x, w: v4.y });
        assert_eq!(vec4!(v4.x, vec3!(v4.y)), Vec4 { x: v4.x, y: v4.y, z: v4.y, w: v4.y });
        assert_eq!(vec4!(vec2!(v4.x), vec2!(v4.y)), Vec4 { x: v4.x, y: v4.x, z: v4.y, w: v4.y });
        assert_eq!(vec4!(vec2!(v4.x), v4.y, v4.z), Vec4 { x: v4.x, y: v4.x, z: v4.y, w: v4.z });
        assert_eq!(vec4!(v4.x, vec2!(v4.y), v4.z), Vec4 { x: v4.x, y: v4.y, z: v4.y, w: v4.z });
        assert_eq!(vec4!(v4.x, v4.y, vec2!(v4.z)), Vec4 { x: v4.x, y: v4.y, z: v4.z, w: v4.z });
        assert_eq!(vec4!(v4.x, v4.y, v4.z, v4.w), Vec4 { x: v4.x, y: v4.y, z: v4.z, w: v4.w });
    }
}

fn operate_array<const N: usize, F: Fn(f32, f32) -> f32>(a: &[f32; N], b: &[f32; N], f: F) -> [f32; N] {
    let mut out = [0f32; N];
    for i in 0..N {
        out[i] = f(a[i], b[i]);
    }

    out
}

#[test]
pub fn addition() {
    for _ in 0..TEST_COUNT {
        let f = |a, b| a + b;
        
        // Vec2
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec2::from_slice(&a) + Vec2::from_slice(&b), Vec2::from_slice(&c));

        let mut a = Vec2::from_array(a);
        let b = Vec2::from_array(b);
        let c = Vec2::from_array(c);
        a += b;
        assert_eq!(a, c);
    
        // Vec3
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec3::from_slice(&a) + Vec3::from_slice(&b), Vec3::from_slice(&c));

        let mut a = Vec3::from_array(a);
        let b = Vec3::from_array(b);
        let c = Vec3::from_array(c);
        a += b;
        assert_eq!(a, c);
    
        // Vec4
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec4::from_slice(&a) + Vec4::from_slice(&b), Vec4::from_slice(&c));

        let mut a = Vec4::from_array(a);
        let b = Vec4::from_array(b);
        let c = Vec4::from_array(c);
        a += b;
        assert_eq!(a, c);
    }
}

#[test]
pub fn subtraction() {
    for _ in 0..TEST_COUNT {
        let f = |a, b| a -  b;
        
        // Vec2
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec2::from_slice(&a) - Vec2::from_slice(&b), Vec2::from_slice(&c));

        let mut a = Vec2::from_array(a);
        let b = Vec2::from_array(b);
        let c = Vec2::from_array(c);
        a -= b;
        assert_eq!(a, c);
    
        // Vec3
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec3::from_slice(&a) - Vec3::from_slice(&b), Vec3::from_slice(&c));

        let mut a = Vec3::from_array(a);
        let b = Vec3::from_array(b);
        let c = Vec3::from_array(c);
        a -= b;
        assert_eq!(a, c);
    
        // Vec4
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec4::from_slice(&a) - Vec4::from_slice(&b), Vec4::from_slice(&c));

        let mut a = Vec4::from_array(a);
        let b = Vec4::from_array(b);
        let c = Vec4::from_array(c);
        a -= b;
        assert_eq!(a, c);
    }
}

#[test]
pub fn multiplication() {
    for _ in 0..TEST_COUNT {
        let f = |a, b| a * b;
        
        // Vec2
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec2::from_slice(&a) * Vec2::from_slice(&b), Vec2::from_slice(&c));

        let mut a = Vec2::from_array(a);
        let b = Vec2::from_array(b);
        let c = Vec2::from_array(c);
        a *= b;
        assert_eq!(a, c);
    
        // Vec3
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec3::from_slice(&a) * Vec3::from_slice(&b), Vec3::from_slice(&c));

        let mut a = Vec3::from_array(a);
        let b = Vec3::from_array(b);
        let c = Vec3::from_array(c);
        a *= b;
        assert_eq!(a, c);
    
        // Vec4
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec4::from_slice(&a) * Vec4::from_slice(&b), Vec4::from_slice(&c));

        let mut a = Vec4::from_array(a);
        let b = Vec4::from_array(b);
        let c = Vec4::from_array(c);
        a *= b;
        assert_eq!(a, c);
    }
}

#[test]
pub fn division() {
    for _ in 0..TEST_COUNT {
        let f = |a, b| a / b;
        
        // Vec2
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec2::from_slice(&a) / Vec2::from_slice(&b), Vec2::from_slice(&c));

        let mut a = Vec2::from_array(a);
        let b = Vec2::from_array(b);
        let c = Vec2::from_array(c);
        a /= b;
        assert_eq!(a, c);
    
        // Vec3
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec3::from_slice(&a) / Vec3::from_slice(&b), Vec3::from_slice(&c));

        let mut a = Vec3::from_array(a);
        let b = Vec3::from_array(b);
        let c = Vec3::from_array(c);
        a /= b;
        assert_eq!(a, c);
    
        // Vec4
        let a = rand_array();
        let b = rand_array();
        let c = operate_array(&a, &b, f);
        assert_eq!(Vec4::from_slice(&a) / Vec4::from_slice(&b), Vec4::from_slice(&c));

        let mut a = Vec4::from_array(a);
        let b = Vec4::from_array(b);
        let c = Vec4::from_array(c);
        a /= b;
        assert_eq!(a, c);
    }
}

#[test]
pub fn transpose() {
    let a = Mat4::new((vec4!(1, 2, 3, 4), vec4!(5, 6, 7, 8), vec4!(9, 10, 11, 12), vec4!(13, 14, 15, 16)));
    let b = Mat4::new((vec4!(1, 5, 9, 13), vec4!(2, 6, 10, 14), vec4!(3, 7, 11, 15), vec4!(4, 8, 12, 16))).transpose();

    assert_eq!(a, b);
}

const TIMES: u32 = 10000000;

#[test]
pub fn timing() {
    let mut t1 = Duration::ZERO;
    let mut t2 = Duration::ZERO;
    let mut t3 = Duration::ZERO;
    let mut t4 = Duration::ZERO;
    let arr_a = rand_array();
    let arr_b = rand_array();
    let a1 = Mat4::from_array(arr_a);
    let b1 = Mat4::from_array(arr_b);
    let a2 = Mat4::from_array(arr_a);
    let b2 = Mat4::from_array(arr_b);
    let a3 = Mat4::from_array(arr_a);
    let b3 = Mat4::from_array(arr_b);
    let mut a4 = glm::Mat4::zero();
    let mut b4 = glm::Mat4::zero();

    for c in 0..4 {
        for r in 0..4 {
            a4[c][r] = a1[c][r];
            b4[c][r] = b1[c][r];
        }
    }

    for _ in 0..TIMES {

        let start = Instant::now();
        let c1 = mul1(a1, b1);
        t1 += Instant::now() - start;
        
        let start = Instant::now();
        let c2 = mul2(a2, b2);
        t2 += Instant::now() - start;
        
        let start = Instant::now();
        let c3 = mul3(a3, b3);
        t3 += Instant::now() - start;

        let start = Instant::now();
        let c4 = a4.clone() * b4.clone();
        t4 += Instant::now() - start;
    }

    panic!("mul1: {}, mul2: {}, mul3: {}, glm: {}", t1.as_millis(), t2.as_millis(), t3.as_millis(), t4.as_millis());
}
use std::f32;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use super::vec3::Vec3;

pub fn random_float() -> f32 {
    rand::thread_rng().gen()
}

pub fn random_float_in(min: f32, max: f32) -> f32 {
    Uniform::from(min..max).sample(&mut rand::thread_rng())
}

pub fn random_vec3() -> Vec3 {
    Vec3::new(random_float(), random_float(), random_float())
}

pub fn random_vec3_in(min: f32, max: f32) -> Vec3 {
    Vec3::new(random_float_in(min, max), random_float_in(min, max), random_float_in(min, max))
}

pub fn random_vec3_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec3_in(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vec3() -> Vec3 {
    let a = random_float_in(0.0, 2.0 * f32::consts::PI);
    let z = random_float_in(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[test]
fn test_random_float() {
    let f0 = random_float();
    let f1 = random_float_in(1.0, 2.0);
    let f2 = random_float_in(2.0, 3.0);
    let f3 = random_float_in(3.0, 4.0);
    assert!(0.0 <= f0 && f0 < 1.0);
    assert!(1.0 <= f1 && f1 < 2.0);
    assert!(2.0 <= f2 && f2 < 3.0);
    assert!(3.0 <= f3 && f3 < 4.0);
}

#[test]
fn test_clamp() {
    assert_eq!(0.0, clamp(-1.0, 0.0, 1.0));
    assert_eq!(0.0, clamp(0.0, 0.0, 1.0));
    assert_eq!(0.5, clamp(0.5, 0.0, 1.0));
    assert_eq!(1.0, clamp(1.0, 0.0, 1.0));
    assert_eq!(1.0, clamp(2.0, 0.0, 1.0));
    assert_eq!(2.0, clamp(1.0, 2.0, 3.0));
    assert_eq!(2.5, clamp(2.5, 2.0, 3.0));
    assert_eq!(3.0, clamp(4.0, 2.0, 3.0));
}

#[test]
fn test_random_vec3() {
    let v1 = random_vec3();
    assert!(0.0 <= v1.x && v1.x < 1.0);
    assert!(0.0 <= v1.y && v1.y < 1.0);
    assert!(0.0 <= v1.z && v1.z < 1.0);

    let v2 = random_vec3_in(2.0, 3.0);
    assert!(2.0 <= v2.x && v2.x < 3.0);
    assert!(2.0 <= v2.y && v2.y < 3.0);
    assert!(2.0 <= v2.z && v2.z < 3.0);
}

#[test]
fn test_random_vec3_in_unit_sphere() {
    let p1 = random_vec3_in_unit_sphere();
    let p2 = random_vec3_in_unit_sphere();
    let p3 = random_vec3_in_unit_sphere();
    assert!(p1.length() < 1.0);
    assert!(p2.length() < 1.0);
    assert!(p3.length() < 1.0);
}

#[test]
fn test_random_unit_vec3() {
    let p1 = random_unit_vec3();
    let p2 = random_unit_vec3();
    let p3 = random_unit_vec3();
    assert!(p1.length() - 1.0 < 1e-10);
    assert!(p2.length() - 1.0 < 1e-10);
    assert!(p3.length() - 1.0 < 1e-10);
}

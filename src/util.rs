use rand::Rng;
use rand::distributions::{Distribution, Uniform};

pub fn random_float() -> f32 {
    rand::thread_rng().gen()
}

pub fn random_float_in(min: f32, max: f32) -> f32 {
    Uniform::from(min..max).sample(&mut rand::thread_rng())
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

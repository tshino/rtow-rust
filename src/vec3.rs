use std::ops::{Neg, Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign};

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}
pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn zeros() -> Self {
        Vec3{ x: 0.0, y: 0.0, z: 0.0 }
    }
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3{ x, y, z }
    }
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        let inv = 1.0 / rhs;
        Vec3::new(self.x * inv, self.y * inv, self.z * inv)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        let inv = 1.0 / rhs;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
    }
}
pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}
pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x
    )
}
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

#[test]
fn test_vec3() {
    {
        let v = Vec3::zeros();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }
    {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }
    {
        let v = Vec3{ x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }
    {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).length_squared(), 14.0);
        assert_eq!(Vec3::new(0.0, 3.0, 4.0).length(), 5.0);
    }
    {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(3.0, 6.0, 9.0);
        assert_eq!((-a).x, -1.0);
        assert_eq!((-a).y, -2.0);
        assert_eq!((-a).z, -3.0);
        assert_eq!((a + b).x, 4.0);
        assert_eq!((a + b).y, 8.0);
        assert_eq!((a + b).z, 12.0);
        assert_eq!((a - b).x, -2.0);
        assert_eq!((a - b).y, -4.0);
        assert_eq!((a - b).z, -6.0);
        assert_eq!((a * b).x, 3.0);
        assert_eq!((a * b).y, 12.0);
        assert_eq!((a * b).z, 27.0);
        assert_eq!((a * 2.0).x, 2.0);
        assert_eq!((a * 2.0).y, 4.0);
        assert_eq!((a * 2.0).z, 6.0);
        assert_eq!((2.0 * b).x, 6.0);
        assert_eq!((2.0 * b).y, 12.0);
        assert_eq!((2.0 * b).z, 18.0);
        assert_eq!((a / 2.0).x, 0.5);
        assert_eq!((a / 2.0).y, 1.0);
        assert_eq!((a / 2.0).z, 1.5);
        let mut c = Vec3::new(4.0, 5.0, 6.0);
        c += a;
        assert_eq!(c.x, 5.0);
        assert_eq!(c.y, 7.0);
        assert_eq!(c.z, 9.0);
        c *= 2.0;
        assert_eq!(c.x, 10.0);
        assert_eq!(c.y, 14.0);
        assert_eq!(c.z, 18.0);
        c /= 4.0;
        assert_eq!(c.x, 2.5);
        assert_eq!(c.y, 3.5);
        assert_eq!(c.z, 4.5);
    }
    {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(dot(a, b), 26.0);
        assert_eq!(cross(a, b).x, -2.0);
        assert_eq!(cross(a, b).y, 4.0);
        assert_eq!(cross(a, b).z, -2.0);
        assert_eq!(unit_vector(a).x, 1.0 / 14.0_f32.sqrt());
        assert_eq!(unit_vector(a).y, 2.0 / 14.0_f32.sqrt());
        assert_eq!(unit_vector(a).z, 3.0 / 14.0_f32.sqrt());
    }
    {
        assert_eq!(Point3::new(0.0, 0.5, 1.0).z, 1.0);
        assert_eq!(Color::new(0.0, 0.5, 1.0).y, 0.5);
    }
}

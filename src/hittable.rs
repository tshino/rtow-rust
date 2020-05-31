use super::vec3;
use super::ray;

use vec3::Point3;
use vec3::Vec3;
use ray::Ray;

#[derive(PartialEq, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32
}

#[derive(PartialEq, Debug)]
pub enum HitResult {
    Hit(HitRecord),
    None
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> HitResult;
}

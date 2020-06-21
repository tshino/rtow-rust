use super::vec3;
use super::ray;
use super::hittable;

use vec3::Color;
use ray::Ray;
use hittable::HitRecord;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scatterd: Ray
}

pub enum ScatterResult {
    Scattered(ScatterRecord),
    None
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterResult;
}

use super::hittable;
use super::ray;

use hittable::Hittable;
use hittable::HitResult;
use hittable::HitRecord;
use ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList{objects:Vec::new()}
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> HitResult {
        let mut temp_rec: HitResult = HitResult::None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let HitResult::Hit(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = HitResult::Hit(rec);
            }
        }
        temp_rec
    }
}

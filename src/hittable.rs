use super::vec3;
use super::ray;

use vec3::Point3;
use vec3::Vec3;
use ray::Ray;

#[derive(Default, PartialEq, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = vec3::dot(r.direction, *outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

#[derive(PartialEq, Debug)]
pub enum HitResult {
    Hit(HitRecord),
    None
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> HitResult;
}

#[test]
fn test_hit_record() {
    {
        let rec: HitRecord = Default::default();
        assert_eq!(rec.t, 0.0);
    }
    {
        let mut rec: HitRecord = Default::default();
        rec.set_face_normal(
            &Ray::new(&Point3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, -1.0)),
            &Vec3::new(0.0, 0.0, 1.0)
        );
        assert_eq!(rec.normal, Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(rec.front_face, true);
    }
}

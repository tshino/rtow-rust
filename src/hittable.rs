use super::vec3;
use super::ray;

use vec3::Point3;
use vec3::Vec3;
use ray::Ray;

#[derive(PartialEq, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool
}

pub struct FaceNormal {
    pub normal: Vec3,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(p: Point3, t: f32, face_normal: FaceNormal) -> Self {
        Self{ p, normal: face_normal.normal, t, front_face: face_normal.front_face }
    }
}

impl FaceNormal {
    pub fn new(r: &Ray, outward_normal: &Vec3) -> Self {
        let front_face = vec3::dot(r.direction, *outward_normal) < 0.0;
        let normal = if front_face { *outward_normal } else { -*outward_normal };
        Self{ normal, front_face }
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
        let face_normal = FaceNormal::new(
            &Ray::new(&Point3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, -1.0)),
            &Vec3::new(0.0, 0.0, 1.0)
        );
        let rec = HitRecord::new(Vec3::zeros(), 1.0, face_normal);
        assert_eq!(rec.normal, Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(rec.front_face, true);
    }
}

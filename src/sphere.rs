use super::vec3;
use super::ray;
use super::hittable;

use vec3::{Vec3, Point3};
use ray::Ray;
use hittable::Hittable;
use hittable::HitRecord;
use hittable::HitResult;
use hittable::FaceNormal;

pub struct Sphere {
    center: Point3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere{ center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> HitResult {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = vec3::dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut t = (-half_b - root) / a;
            if t < t_max && t > t_min {
                let p = r.at(t);
                let outward_normal = (p - self.center) / self.radius;
                let face_normal = FaceNormal::new(&r, &outward_normal);
                let rec = HitRecord::new(p, t, face_normal);
                HitResult::Hit(rec)
            } else {
                t = (-half_b + root) / a;
                if t < t_max && t > t_min {
                    let p = r.at(t);
                    let outward_normal = (p - self.center) / self.radius;
                    let face_normal = FaceNormal::new(&r, &outward_normal);
                    let rec = HitRecord::new(p, t, face_normal);
                    HitResult::Hit(rec)
                } else {
                    HitResult::None
                }
            }
        } else {
            HitResult::None
        }
    }
}

#[test]
fn test_sphere_hittable() {
    let sphere1 = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Point3::new(0.5, 0.0, -1.0), 0.2);
    let ray1 = Ray::new(&Point3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, -1.0));
    let ray2 = Ray::new(&Point3::new(0.0, 1.0, 0.0), &Vec3::new(0.0, 0.0, -1.0));
    let ray3 = Ray::new(&Point3::new(0.0, 0.0, 0.0), &Vec3::new(0.5, 0.0, -1.0));

    assert!(sphere1.hit(&ray1, 0.0, 2.0) != HitResult::None);
    assert!(sphere1.hit(&ray1, 0.0, 2.0) == HitResult::Hit(
        HitRecord{
            p: Point3::new(0.0, 0.0, -0.5),
            normal: Vec3::new(0.0, 0.0, 1.0),
            t: 0.5,
            front_face: true
        }
    ));
    assert!(sphere1.hit(&ray1, 0.0, 0.4) == HitResult::None);
    assert!(sphere1.hit(&ray2, 0.0, 2.0) == HitResult::None);
    assert!(sphere2.hit(&ray1, 0.0, 2.0) == HitResult::None);
    assert!(sphere2.hit(&ray3, 0.0, 2.0) != HitResult::None);
}

use super::vec3;
use vec3::Point3;
use vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin : Point3,
    pub direction : Vec3
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Ray{ origin: *origin, direction: *direction }
    }
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}

#[test]
fn test_ray() {
    {
        let ray = Ray::new(&Point3::new(0.0, 3.0, 0.0), &Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(ray.origin.y, 3.0);
        assert_eq!(ray.direction.x, 1.0);
        let p1 = ray.at(1.0);
        assert_eq!(p1.x, 1.0);
        assert_eq!(p1.y, 3.0);
        assert_eq!(p1.z, 0.0);
        let p2 = ray.at(2.5);
        assert_eq!(p2.x, 2.5);
        assert_eq!(p2.y, 3.0);
        assert_eq!(p2.z, 0.0);
    }
}

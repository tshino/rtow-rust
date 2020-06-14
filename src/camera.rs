use super::vec3::{Vec3, Point3};
use super::ray::{Ray};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::zeros();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        Camera{
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(&self.origin, &(self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin))
    }
}

#[test]
fn test_camera() {
    let camera = Camera::new(2.0);
    let r1 = camera.get_ray(0.0, 0.0);
    let r2 = camera.get_ray(1.0, 0.0);
    let r3 = camera.get_ray(0.5, 1.0);
    assert_eq!(r1.origin, Vec3::zeros());
    assert_eq!(r1.direction, Vec3::new(-2.0, -1.0, -1.0));
    assert_eq!(r2.direction, Vec3::new(2.0, -1.0, -1.0));
    assert_eq!(r3.direction, Vec3::new(0.0, 1.0, -1.0));
}
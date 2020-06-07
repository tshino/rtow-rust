mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;

use vec3::Vec3;
use vec3::Point3;
use vec3::Color;
use ray::Ray;

pub fn print_color(pixel_color: Color) {
    let ir = (pixel_color.x * 255.0).round();
    let ig = (pixel_color.y * 255.0).round();
    let ib = (pixel_color.z * 255.0).round();
    println!("{} {} {}", ir, ig, ib);
}

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin - *center;
    let a = r.direction.length_squared();
    let half_b = vec3::dot(oc, r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let t =hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    let unit_direction = vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    const W: u32 = 384;
    const H: u32 = 216;
    let aspect_ratio: f32 = W as f32 / H as f32;

    println!("P3\n{} {}\n255", W, H);

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    for j in (0..H).rev() {
        eprint!("\rScanlines remaing: {} ", j);
        for i in 0..W {
            let u = (i as f32) / ((W - 1) as f32);
            let v = (j as f32) / ((H - 1) as f32);
            let r = Ray::new(&origin, &(lower_left_corner + u * horizontal + v * vertical - origin));
            let pixel_color = ray_color(&r);
            print_color(pixel_color);
        }
    }
    eprintln!("\nDone.");
}

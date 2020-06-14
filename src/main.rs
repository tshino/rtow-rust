mod vec3;
mod color;
mod ray;
mod camera;
mod hittable;
mod hittable_list;
mod sphere;
mod util;

use vec3::Point3;
use vec3::Color;
use ray::Ray;
use camera::Camera;
use hittable::Hittable;
use hittable::HitResult;
use hittable_list::HittableList;
use sphere::Sphere;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    if let HitResult::Hit(rec) = world.hit(r, 0.0, f32::INFINITY) {
        let target = rec.p + rec.normal + util::random_vec3_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(&rec.p, &(target - rec.p)), world);
    }
    let unit_direction = vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    const W: u32 = 384;
    const H: u32 = 216;
    let aspect_ratio: f32 = W as f32 / H as f32;
    const SAMPLES_PER_PIXEL: i32 = 100;

    println!("P3\n{} {}\n255", W, H);

    let camera = Camera::new(aspect_ratio);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    for j in (0..H).rev() {
        eprint!("\rScanlines remaing: {} ", j);
        for i in 0..W {
            let mut pixel_color = Color::zeros();
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + util::random_float()) / (W as f32);
                let v = (j as f32 + util::random_float()) / (H as f32);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            color::print_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nDone.");
}

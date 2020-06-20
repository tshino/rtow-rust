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

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::zeros();
    }
    if let HitResult::Hit(rec) = world.hit(r, 0.001, f32::INFINITY) {
        let target = rec.p + rec.normal + util::random_unit_vec3();
        return 0.5 * ray_color(&Ray::new(&rec.p, &(target - rec.p)), world, depth - 1);
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
    const MAX_DEPTH: i32 = 50;

    println!("P3\n{} {}\n255", W, H);

    let camera = Camera::new(aspect_ratio);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    for j in (0..H).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..W {
            let mut pixel_color = Color::zeros();
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + util::random_float()) / (W as f32);
                let v = (j as f32 + util::random_float()) / (H as f32);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            color::print_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nDone.");
}

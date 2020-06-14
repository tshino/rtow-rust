use super::util;
use super::vec3::Color;

pub fn print_color(pixel_color: Color, samples_per_pixel: i32) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let scale = 255.0 / samples_per_pixel as f32;

    let ir = util::clamp(r * scale, 0.0, 255.0).round();
    let ig = util::clamp(g * scale, 0.0, 255.0).round();
    let ib = util::clamp(b * scale, 0.0, 255.0).round();
    println!("{} {} {}", ir, ig, ib);
}

use super::util;
use super::vec3::Color;

pub fn print_color(pixel_color: Color, samples_per_pixel: i32) {
    let scale = (255.0 * 255.0) / samples_per_pixel as f32;
    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();

    let ir = util::clamp(r, 0.0, 255.0).round();
    let ig = util::clamp(g, 0.0, 255.0).round();
    let ib = util::clamp(b, 0.0, 255.0).round();
    println!("{} {} {}", ir, ig, ib);
}

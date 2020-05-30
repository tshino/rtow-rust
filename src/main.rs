mod vec3;
mod ray;

use vec3::Color;

pub fn print_color(pixel_color: Color) {
    let ir = (pixel_color.x * 255.0).round();
    let ig = (pixel_color.y * 255.0).round();
    let ib = (pixel_color.z * 255.0).round();
    println!("{} {} {}", ir, ig, ib);
}

fn write_ppm(w: u32, h: u32) {
    println!("P3");
    println!("{} {}", w, h);
    println!("255");
    for j in (0..h).rev() {
        eprint!("\rScanlines remaing: {} ", j);
        for i in 0..w {
            let pixel_color = Color::new(
                (i as f32) / ((w - 1) as f32),
                (j as f32) / ((h - 1) as f32),
                0.25
            );
            print_color(pixel_color);
        }
    }
    eprintln!("\nDone.");
}

fn main() {
    write_ppm(320, 256);
}

fn write_ppm(w: u32, h: u32) {
    println!("P3");
    println!("{} {}", w, h);
    println!("255");
    for j in (0..h).rev() {
        for i in 0..w {
            let r = (i as f32) / ((w - 1) as f32);
            let g = (j as f32) / ((h - 1) as f32);
            let b = 0.25_f32;
            let ir = (r * 255.0).round();
            let ig = (g * 255.0).round();
            let ib = (b * 255.0).round();
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn main() {
    write_ppm(320, 256);
}

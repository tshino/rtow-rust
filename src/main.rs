pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}
type Point3 = Vec3;
type Color = Vec3;

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3{ x, y, z }
    }
    fn zeros() -> Self {
        Vec3{ x: 0.0, y: 0.0, z: 0.0 }
    }
    fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}

#[test]
fn test_vec3() {
    {
        let v = Vec3::zeros();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }
    {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }
    {
        let v = Vec3{ x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }
    {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).length_squared(), 14.0);
        assert_eq!(Vec3::new(0.0, 3.0, 4.0).length(), 5.0);
    }
    {
        assert_eq!(Point3::new(0.0, 0.5, 1.0).z, 1.0);
        assert_eq!(Color::new(0.0, 0.5, 1.0).y, 0.5);
    }
}

fn write_ppm(w: u32, h: u32) {
    println!("P3");
    println!("{} {}", w, h);
    println!("255");
    for j in (0..h).rev() {
        eprint!("\rScanlines remaing: {} ", j);
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
    eprintln!("\nDone.");
}

fn main() {
    write_ppm(320, 256);
}

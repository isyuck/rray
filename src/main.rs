use std::ops;

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

type Color = Vec3;

fn main() {
    let image_w = 256;
    let image_h = 256;

    println!("P3\n{image_w} {image_h}\n255");

    for j in (0..image_h).rev() {
        eprintln!("scanlines: {:3}/{}", j, image_h);
        for i in 0..image_w {
            let color: Color = Color {
                x: (i as f64) / ((image_w - 1) as f64),
                y: (j as f64) / ((image_h - 1) as f64),
                z: 0.25,
            };

            let ir = (255.999 * color.x) as u32;
            let ig = (255.999 * color.y) as u32;
            let ib = (255.999 * color.x) as u32;

            println!("{ir} {ig} {ib}")
        }
    }
}

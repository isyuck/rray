extern crate nalgebra;

pub mod ray;
use ray::Ray;

use nalgebra::Vector3;

type Color = Vector3<u32>;

fn main() {
    let image_w = 256;
    let image_h = 256;

    println!("P3\n{image_w} {image_h}\n255");

    let ray: Ray;

    for j in (0..image_h).rev() {
        eprintln!("scanlines: {:3}/{}", j, image_h);
        for i in 0..image_w {
            let color: Color = Color::new(
                (255.999 * (i as f64) / ((image_w - 1) as f64)) as u32,
                (255.999 * (j as f64) / ((image_h - 1) as f64)) as u32,
                ((255.999) * 0.25) as u32,
            );

            println!("{},{},{}", color.x, color.y, color.z);
        }
    }
}

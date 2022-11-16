pub mod vec3;
use vec3::Vec3;

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

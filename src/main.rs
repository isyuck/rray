fn main() {
    let image_w = 256;
    let image_h = 256;

    println!("P3\n{image_w} {image_h}\n255");

    for j in (0..image_h).rev() {
        for i in 0..image_w {
            let r: f64 = (i as f64) / ((image_w - 1) as f64);
            let g: f64 = (j as f64) / ((image_h - 1) as f64);
            let b: f64 = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            println!("{ir} {ig} {ib}")
        }
    }
}

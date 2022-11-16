extern crate nalgebra;
use nalgebra::Vector3;

pub struct Ray {
    origin: Vector3<f64>,
    dir: Vector3<f64>,
}

trait At {
    fn at(&self, t: f64) -> Vector3<f64>;
}

impl At for Ray {
    fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.dir
    }
}

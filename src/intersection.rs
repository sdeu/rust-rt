use na::Point3;
use na::Vector3;

pub struct Intersection {
    pub point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
}

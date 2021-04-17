use super::material::Material;
use na::Point3;
use na::Vector3;
use std::rc::Rc;

pub struct Intersection {
    pub point: Point3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub material: Rc<dyn Material>,
}

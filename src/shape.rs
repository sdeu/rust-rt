use pa::query::{Ray, RayIntersection};
use super::material::Material;
use std::rc::Rc;

pub trait Shape {
    fn hit(&self, ray: &Ray) -> Option<RayIntersection>;
    fn material_at(&self, intersection: &RayIntersection) -> &Rc<dyn Material>;
}

use super::intersection::Intersection;
use super::ray::Ray;

pub trait Shape {
    fn hit(&self, ray: Ray) -> Option<Intersection>;
}

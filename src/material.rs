use super::intersection::Intersection;
use super::math::Vec3;
use super::ray::Ray;

pub trait Material {
    fn scatter(&self, ray: Ray, intersection: Intersection) -> Ray;
}

pub struct Lambert {
    pub color: Vec3,
}

impl Material for Lambert {
    fn scatter(&self, ray: Ray, intersection: Intersection) -> Ray {
        return ray;
    }
}

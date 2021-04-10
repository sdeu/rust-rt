use super::intersection::Intersection;
use super::math::Vec3;
use super::ray::Ray;
use rand::distributions::{Distribution, Uniform};
use std::f64::consts::PI;

pub trait Material {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Ray>;
    fn color(&self) -> Vec3;
}

fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();
    let a : f64 = Uniform::from(0.0..PI*2.).sample(&mut rng);
    let z : f64 = Uniform::from(-1.0..1.0).sample(&mut rng);
    let r = (1.0 - z * z).sqrt();
    na::Vector3::new(r * a.cos(), r * a.sin(), z).normalize()
}

fn reflect(ray: &Ray, intersection: &Intersection) -> Ray {
    let r_n = ray.direction.normalize();
    let r = r_n - ((2.0 * intersection.normal.dot(&r_n)) * intersection.normal);
    Ray{origin: intersection.point, direction: r.normalize()}
}

fn near_zero(v: Vec3) -> bool {
    let s = 1e-8;
    (v.x.abs() < s) && (v.y.abs() < s) && (v.z.abs() < s)
}

pub struct Lambert {
    pub color: Vec3,
}

impl Material for Lambert {
    fn scatter(&self, _ray: &Ray, intersection: &Intersection) -> Option<Ray> {
        let mut r = intersection.normal + random_unit_vector();
        if near_zero(r){
            r = intersection.normal;
        }
        Some(Ray{origin: intersection.point + (intersection.normal * 0.001), direction: r.normalize()})
    }

    fn color(&self) -> Vec3 {
        self.color
    }
}

pub struct Metal {
    pub color: Vec3,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Ray> {
        let r = reflect(ray, intersection);
        if r.direction.dot(&intersection.normal) > 0.0 {
            return Some(r);
        }
        None
    }

    fn color(&self) -> Vec3 {
        self.color
    }
}
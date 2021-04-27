use super::math::Vec3;
use pa::query::Ray;
use pa::query::RayIntersection;
use rand::distributions::{Distribution, Uniform};
use std::f32::consts::PI;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<Ray>;
    fn color(&self) -> Vec3;
}

fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();
    let a: f32 = Uniform::from(0.0..PI * 2.).sample(&mut rng);
    let z: f32 = Uniform::from(-1.0..1.0).sample(&mut rng);
    let r = (1.0 - z * z).sqrt();
    na::Vector3::new(r * a.cos(), r * a.sin(), z).normalize()
}

fn reflect(ray: &Ray, intersection: &RayIntersection) -> Ray {
    let r_n = ray.dir.normalize();
    let r = r_n - ((2.0 * intersection.normal.dot(&r_n)) * intersection.normal);
    Ray {
        origin: ray.point_at(intersection.toi),
        dir: r.normalize(),
    }
}

fn near_zero(v: Vec3) -> bool {
    let s = 1e-8;
    (v.x.abs() < s) && (v.y.abs() < s) && (v.z.abs() < s)
}

pub struct Lambert {
    pub color: Vec3,
}

impl Material for Lambert {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<Ray> {
        let mut r = intersection.normal + random_unit_vector();
        if near_zero(r) {
            r = intersection.normal;
        }
        Some(Ray {
            origin: ray.point_at(intersection.toi) + (intersection.normal * 0.001),
            dir: r.normalize(),
        })
    }

    fn color(&self) -> Vec3 {
        self.color
    }
}

pub struct Metal {
    pub color: Vec3,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<Ray> {
        let r = reflect(ray, intersection);
        if r.dir.dot(&intersection.normal) > 0.0 {
            return Some(r);
        }
        None
    }

    fn color(&self) -> Vec3 {
        self.color
    }
}

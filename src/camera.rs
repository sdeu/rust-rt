use super::math::{Point3, Vec3};
use super::ray::Ray;
use std::f64;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
        let aspect_ratio = width as f64 / height as f64;
        let viewport_height = 2.0;

        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        let origin = na::Point3::new(0., 0., 0.);
        let horizontal = na::Vector3::new(viewport_width, 0., 0.);
        let vertical = na::Vector3::new(0., viewport_height, 0.);

        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - na::Vector3::new(0., 0., focal_length);

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let d = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray {
            origin: self.origin,
            direction: d.normalize(),
        }
    }
}

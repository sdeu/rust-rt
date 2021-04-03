use na::Affine3;
use na::Point3;
use na::Vector3;

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn transform(&self, t: Affine3<f64>) -> Ray {
        return Ray {
            origin: t.transform_point(&self.origin),
            direction: t.transform_vector(&self.direction),
        };
    }

    pub fn point_at(&self, t: f64) -> Point3<f64> {
        &self.origin + (&self.direction * t)
    }
}

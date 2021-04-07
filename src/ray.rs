use super::math::{Matrix4, Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn transform(&self, t: Matrix4) -> Ray {
        return Ray {
            origin: t.transform_point(&self.origin),
            direction: t.transform_vector(&self.direction),
        };
    }

    pub fn point_at(&self, t: f64) -> Point3 {
        &self.origin + (&self.direction * t)
    }
}

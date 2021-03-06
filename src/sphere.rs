use super::material::Material;
use super::shape::Shape;
use na::Isometry3;
use pa::query::{Ray, RayCast, RayIntersection};
use pa::shape::Ball;
use std::f32;
use std::sync::Arc;

pub struct Sphere {
    pub ball: Ball,
    pub object_to_world: Isometry3<f32>,
    pub world_to_object: Isometry3<f32>,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(
        radius: f32,
        object_to_world: Isometry3<f32>,
        material: Arc<dyn Material>,
    ) -> Sphere {
        let inv = object_to_world.inverse();
        return Sphere {
            ball: Ball::new(radius),
            object_to_world: object_to_world,
            world_to_object: inv,
            material: material,
        };
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray) -> Option<RayIntersection> {
        let r = ray.transform_by(&self.world_to_object);
        self.ball.cast_local_ray_and_get_normal(&r, 1000., true)
    }

    fn material_at(&self, _intersection: &RayIntersection) -> &Arc<dyn Material> {
        return &self.material;
    }
}

#[cfg(test)]
mod tests {
    use super::super::material::Lambert;
    use super::{Shape, Sphere};
    use na::{Isometry3, Point3, Vector3};
    use pa::query::Ray;
    use std::sync::Arc;
    #[test]
    fn test_ray_intersection() {
        let l = Arc::new(Lambert {
            color: Vector3::new(1., 0., 0.),
        });
        let s = Sphere::new(1., Isometry3::identity(), l);
        let ray = Ray {
            origin: Point3::new(0., 0., 3.),
            dir: Vector3::new(0., 0., -1.),
        };
        let i = s.hit(&ray);
        assert! {i.is_some()};
        let intersection = i.unwrap();
        assert_eq! {intersection.toi, 2.};
        assert_eq! {ray.point_at(intersection.toi), Point3::new(0., 0., 1.)};
        assert_eq! {intersection.normal, Vector3::new(0., 0., 1.)};
    }
}

use super::intersection::Intersection;
use super::ray::Ray;
use na::{Affine3, Vector3};
use std::f64;

pub struct Sphere {
    pub radius: f64,
    pub object_to_world: Affine3<f64>,
    pub world_to_object: Affine3<f64>,
}

impl Sphere {
    fn new(radius: f64, object_to_world: Affine3<f64>) -> Sphere {
        Sphere {
            radius: radius,
            object_to_world: object_to_world,
            world_to_object: object_to_world.inverse(),
        }
    }

    fn hit(&self, ray: Ray) -> Option<Intersection> {
        let r = ray.transform(self.world_to_object);
        let B = 2. * r.direction.dot(&r.origin.coords);
        let C = r.origin.coords.dot(&r.origin.coords) - (self.radius * self.radius);
        let d = B * B - 4. * C;
        if d < 0. {
            return None;
        }
        let root_d = d.sqrt();
        let q;

        if B < 0. {
            q = -0.5 * (B - root_d);
        } else {
            q = -0.5 * (B + root_d);
        }

        let t0 = q;
        let t1 = C / q;

        if t1 < 0. && t0 < 0. {
            return None;
        }

        let mut t = t0.min(t1);

        if t0 < 0. && t1 > 0. {
            t = t1;
        }

        if t1 < 0. && t0 > 0. {
            t = t0;
        }

        let intersection_point = r.point_at(t);
        let n = Vector3::new(
            intersection_point.x,
            intersection_point.y,
            intersection_point.z,
        );
        return Some(Intersection {
            point: self.object_to_world.transform_point(&intersection_point),
            normal: n,
            t: t,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{Ray, Sphere};
    use na::{Affine3, Point3, Vector3};
    #[test]
    fn test_ray_intersection() {
        let s = Sphere::new(1., Affine3::identity());
        let ray = Ray {
            origin: Point3::new(0., 0., 3.),
            direction: Vector3::new(0., 0., -1.),
        };
        let i = s.hit(ray);
        assert! {i.is_some()};
        let intersection = i.unwrap();
        assert_eq! {intersection.t, 2.};
        assert_eq! {intersection.point, Point3::new(0., 0., 1.)};
        assert_eq! {intersection.normal, Vector3::new(0., 0., 1.)};
    }
}

use super::intersection::Intersection;
use super::material::Material;
use super::math::Matrix4;
use super::ray::Ray;
use super::shape::Shape;
use na::Vector3;
use std::{f64, rc::Rc};

pub struct Sphere {
    pub radius: f64,
    pub object_to_world: Matrix4,
    pub world_to_object: Matrix4,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(radius: f64, object_to_world: Matrix4, material: Rc<dyn Material>) -> Sphere {
        let inv = object_to_world.try_inverse();
        match inv {
            Some(i) => {
                return Sphere {
                    radius: radius,
                    object_to_world: object_to_world,
                    world_to_object: i,
                    material: material,
                }
            }
            None => panic!(),
        }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Intersection> {
        let r = ray.transform(self.world_to_object);
        let b = 2. * r.direction.dot(&r.origin.coords);
        let c = r.origin.coords.dot(&r.origin.coords) - (self.radius * self.radius);
        let d = b * b - 4. * c;
        if d < 0. {
            return None;
        }
        let root_d = d.sqrt();
        let q;

        if b < 0. {
            q = -0.5 * (b - root_d);
        } else {
            q = -0.5 * (b + root_d);
        }

        let t0 = q;
        let t1 = c / q;

        if t1 < 0. && t0 < 0. {
            return None;
        }

        let t = t0.min(t1);
        let intersection_point = r.point_at(t);
        let n = Vector3::new(
            intersection_point.x,
            intersection_point.y,
            intersection_point.z,
        ).normalize();
        return Some(Intersection {
            point: self.object_to_world.transform_point(&intersection_point),
            normal: n,
            t: t,
            material: self.material.clone(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::super::material::Lambert;
    use super::{Ray, Shape, Sphere};
    use na::{Matrix4, Point3, Vector3};
    use std::rc::Rc;
    #[test]
    fn test_ray_intersection() {
        let l = Rc::new(Lambert {
            color: Vector3::new(1., 0., 0.),
        });
        let s = Sphere::new(1., Matrix4::identity(), l);
        let ray = Ray {
            origin: Point3::new(0., 0., 3.),
            direction: Vector3::new(0., 0., -1.),
        };
        let i = s.hit(&ray);
        assert! {i.is_some()};
        let intersection = i.unwrap();
        assert_eq! {intersection.t, 2.};
        assert_eq! {intersection.point, Point3::new(0., 0., 1.)};
        assert_eq! {intersection.normal, Vector3::new(0., 0., 1.)};
    }
}

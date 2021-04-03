use ray::Ray;
use intersection::Intersection;
use na::Vector3;

mod material {
    trait Material {
        fn scatter(&self, ray: Ray, intersection: Intersection) -> Ray;
    }

    pub struct Lambert {
        color: Vector3
    }

    impl Material for Lambert {
        fn scatter(ray: Ray, intersection: Intersection) {
            return ray;
        }
    }

}
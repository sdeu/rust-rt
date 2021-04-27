use super::material::Material;
use pa::query::{Ray, RayIntersection};
use std::sync::Arc;

pub trait Shape: Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<RayIntersection>;
    fn material_at(&self, intersection: &RayIntersection) -> &Arc<dyn Material>;
}

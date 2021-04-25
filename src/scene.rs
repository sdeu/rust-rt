use super::shape::Shape;
use std::sync::Arc;
pub struct Scene {
    pub shapes: Vec<Arc<dyn Shape>>,
}

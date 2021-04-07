use super::shape::Shape;
pub struct Scene {
    pub shapes: Vec<Box<dyn Shape>>,
}

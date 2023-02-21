use crate::camera::Camera;
use crate::light::Light;
use crate::sdf::Node;

pub struct Scene {
    pub camera: Camera,
    pub root: Box<dyn Node>,
    pub lights: Vec<Box<dyn Light>>,
}

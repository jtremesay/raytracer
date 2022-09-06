use crate::camera::Camera;
use crate::geometry::Sphere;
use crate::light::Light;

pub struct Scene {
    pub camera: Camera,
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
            spheres: vec![],
            lights: vec![],
        }
    }
}

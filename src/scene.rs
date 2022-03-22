use crate::geometry::Sphere;
use crate::math::Vector3;

/**
 * @brief A view port
 */
pub struct ViewPort {
    pub width: f32,
    pub height: f32,
    pub distance: f32,
}

impl ViewPort {
    pub fn new(width: f32, height: f32, distance: f32) -> Self {
        Self {
            width,
            height,
            distance,
        }
    }
}

pub struct Camera {
    pub position: Vector3,
    pub view_port: ViewPort,
}

impl Camera {
    pub fn new(position: Vector3, view_port: ViewPort) -> Self {
        Self {
            position,
            view_port,
        }
    }
}

pub struct Scene {
    pub camera: Camera,
    pub spheres: Vec<Sphere>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(Vector3::default(), ViewPort::new(1.0, 1.0, 1.0)),
            spheres: vec![],
        }
    }

    // fn add_object(&mut self, object: Box<dyn Node + Send>) {
    //     self.nodes.push(object);
    // }
}

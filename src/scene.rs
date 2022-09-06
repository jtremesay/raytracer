use crate::geometry::Sphere;
use crate::light::Light;
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

impl Default for ViewPort {
    fn default() -> Self {
        Self::new(1.0, 1.0, 1.0)
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

impl Default for Camera {
    fn default() -> Self {
        Self::new(Vector3::default(), ViewPort::default())
    }
}

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

use crate::math::Vector3;

pub struct Camera {
    pub position: Vector3,
    pub view_port: Vector3,
}

impl Camera {
    pub fn new(position: Vector3, view_port: Vector3) -> Self {
        Self {
            position,
            view_port,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(Vector3::default(), Vector3::new(1.0, 1.0, 1.0))
    }
}

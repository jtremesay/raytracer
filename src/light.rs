use crate::math::Vector3;

pub trait Light {
    fn compute(&self, point: Vector3, normal: Vector3) -> f32;
}

pub struct AmbientLight {
    intensity: f32,
}

impl AmbientLight {
    pub fn new(intensity: f32) -> Self {
        Self { intensity }
    }
}

impl Light for AmbientLight {
    fn compute(&self, _point: Vector3, _normal: Vector3) -> f32 {
        self.intensity
    }
}

pub struct OminidirectionalLight {
    intensity: f32,
    source: Vector3,
}

impl OminidirectionalLight {
    pub fn new(intensity: f32, source: Vector3) -> Self {
        Self { intensity, source }
    }
}

impl Light for OminidirectionalLight {
    fn compute(&self, point: Vector3, normal: Vector3) -> f32 {
        let direction = self.source - point;
        let dot = normal.dot(direction);

        if dot > 0.0 {
            self.intensity * dot / (normal.length() * direction.length())
        } else {
            0.0
        }
    }
}

pub struct DirectionalLight {
    intensity: f32,
    direction: Vector3,
}

impl DirectionalLight {
    pub fn new(intensity: f32, direction: Vector3) -> Self {
        Self {
            intensity,
            direction,
        }
    }
}

impl Light for DirectionalLight {
    fn compute(&self, _point: Vector3, normal: Vector3) -> f32 {
        let dot = normal.dot(self.direction);

        if dot > 0.0 {
            self.intensity * dot / (normal.length() * self.direction.length())
        } else {
            0.0
        }
    }
}

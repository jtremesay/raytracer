use crate::math::Vector3;

#[derive(Clone, Copy)]
pub enum LightKind {
    Ambient,
    OmniDirectional,
    Directional,
}

#[derive(Clone, Copy)]
pub struct Light {
    pub kind: LightKind,
    pub intensity: f32,
    pub direction: Vector3,
}

impl Light {
    pub fn new(kind: LightKind, intensity: f32, direction: Vector3) -> Self {
        Self {
            kind,
            intensity,
            direction,
        }
    }

    pub fn create_ambiant(intensity: f32) -> Self {
        Self::new(LightKind::Ambient, intensity, Vector3::default())
    }

    pub fn create_omnidirectional(intensity: f32, direction: Vector3) -> Self {
        Self::new(LightKind::OmniDirectional, intensity, direction)
    }

    pub fn create_directional(intensity: f32, direction: Vector3) -> Self {
        Self::new(LightKind::Directional, intensity, direction)
    }
}

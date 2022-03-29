use crate::math::Vector3;

pub enum Light {
    Ambient(f32),
    OmniDirectional(f32, Vector3),
    Directional(f32, Vector3),
}

use crate::color::Color;

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub specular: f32,
}

impl Material {
    pub fn new(color: Color, specular: f32) -> Self {
        Self { color, specular }
    }
}

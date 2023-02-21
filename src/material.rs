use crate::color::Color;

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub specular: f32,
}

impl Material {
    pub const DEBUG_MATERIAL: Material = Material {
        color: Color::MAGENTA,
        specular: 0.0,
    };
}

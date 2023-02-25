use crate::{canvas::Canvas, scene::Scene};

pub mod opengl;
pub mod software;

pub trait Renderer {
    fn render(&self, scene: &Scene, canvas: &mut dyn Canvas);
}

pub enum RendererType {
    Software,
    OpenGL,
}

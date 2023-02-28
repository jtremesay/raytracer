use crate::{canvas::Canvas, render::Renderer, scene::Scene};

pub struct OpenGLRenderer {}

impl OpenGLRenderer {}

impl Renderer for OpenGLRenderer {
    fn render(&self, _scene: &Scene, _canvas: &mut dyn Canvas) {
        todo!()
    }
}

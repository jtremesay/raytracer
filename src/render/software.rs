use crate::{
    camera::ViewPort,
    canvas::Canvas,
    color::Color,
    light::Light,
    math::Vector3,
    ray::{Hit, Ray},
    render::Renderer,
    scene::Scene,
};

pub struct SoftwareRenderer {}

impl SoftwareRenderer {
    pub fn compute_light(
        &self,
        lights: &Vec<Box<dyn Light>>,
        hit: &Hit,
        inverse_direction: Vector3,
    ) -> f32 {
        lights
            .iter()
            .map(|light| light.compute_intensity(hit, inverse_direction))
            .sum()
    }

    pub fn compute_color(&self, scene: &Scene, ray: &Ray) -> Color {
        let color = if let Some(hit) = scene.root.hit(ray) {
            hit.material.color * self.compute_light(&scene.lights, &hit, -ray.direction)
        } else {
            Color::WHITE
        };

        color
    }

    pub fn render_pixel(
        &self,
        scene: &Scene,
        u: u32,
        v: u32,
        width: u32,
        height: u32,
        view_port: &ViewPort,
    ) -> Color {
        // Compute the direction of the ray
        let direction = Vector3::new(
            (u as f32 - width as f32 / 2.0) * view_port.width / width as f32,
            ((v * height / width) as f32 - height as f32 / 2.0) * view_port.height / height as f32,
            view_port.distance,
        );
        let ray = Ray {
            origin: scene.camera.position,
            direction: direction,
        };

        self.compute_color(&scene, &ray)
    }
}

impl Renderer for SoftwareRenderer {
    fn render(&self, scene: &Scene, canvas: &mut dyn Canvas) {
        let canvas_width = canvas.width();
        let canvas_height = canvas.height();
        let view_port = &scene.camera.view_port;

        // Draw each pixel of the canvas
        for v in 0..canvas_height {
            for u in 0..canvas_width {
                // Draw the pixel
                let color = self.render_pixel(&scene, u, v, canvas_width, canvas_height, view_port);
                canvas.set_pixel(u, v, color);
            }
        }
    }
}

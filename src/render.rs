use crate::{
    canvas::Canvas,
    color::Color,
    light::Light,
    math::Vector3,
    ray::{Hit, Ray},
    scene::Scene,
};

fn compute_light(lights: &Vec<Box<dyn Light>>, hit: &Hit, inverse_direction: Vector3) -> f32 {
    lights
        .iter()
        .map(|light| light.compute_intensity(hit, inverse_direction))
        .sum()
}

fn compute_color(scene: &Scene, direction: Vector3) -> Color {
    let ray = Ray {
        origin: scene.camera.position,
        direction: direction,
    };

    let color = if let Some(hit) = scene.root.hit(&ray) {
        hit.material.color * compute_light(&scene.lights, &hit, -direction)
    } else {
        Color::WHITE
    };

    color
}

pub fn render(scene: &Scene, canvas: &mut dyn Canvas) {
    let channel_width = canvas.width();
    let channel_height = canvas.height();
    let viewport_width = scene.camera.view_port.width;
    let viewport_height = scene.camera.view_port.height;
    let dist_to_canvas = scene.camera.view_port.distance;

    // Draw each pixel of the canvas
    for v in 0..channel_height {
        for u in 0..channel_width {
            // Compute the direction of the ray
            let direction = Vector3::new(
                (u as f32 - channel_width as f32 / 2.0) * viewport_width / channel_width as f32,
                ((v * channel_height / channel_width) as f32 - channel_height as f32 / 2.0)
                    * viewport_height
                    / channel_height as f32,
                dist_to_canvas,
            );

            // Draw the pixel
            let color = compute_color(&scene, direction);
            canvas.set_pixel(u, v, color);
        }
    }
}

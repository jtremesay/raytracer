use crate::canvas::Canvas;
use crate::color::Color;
use crate::light::Light;
use crate::light::LightKind;
use crate::math::Vector3;
use crate::scene::Scene;
use std::f32::INFINITY;

fn compute_light(
    lights: &Vec<Light>,
    hit_position: Vector3,
    normal: Vector3,
    inverse_direction: Vector3,
    specular: f32,
) -> f32 {
    let mut i = 0.0;
    for light in lights.iter() {
        if let LightKind::Ambient = light.kind {
            // Ambiant light
            i += light.intensity;
        } else {
            // Omnidirectional and directional
            // Get the light direction and intensity
            let light_direction = match light.kind {
                LightKind::OmniDirectional => light.direction - hit_position,
                LightKind::Directional => light.direction,
                _ => {
                    continue;
                }
            };

            // Diffuse
            let n_dot_l = normal.dot(light_direction);
            if n_dot_l > 0.0 {
                i += light.intensity * n_dot_l / (normal.length() * light_direction.length());
            }

            // // Specular
            if specular > -1.0 {
                let r = normal * normal.dot(light_direction) * 2.0 - light_direction;
                let r_dot_v = r.dot(inverse_direction);
                if r_dot_v > 0.0 {
                    i += light.intensity
                        * (r_dot_v / (r.length() * inverse_direction.length())).powf(specular)
                }
            }
        }
    }

    i
}

/**
 * @brief Render a scene into a canvas
 *
 * @param scene the scene to render
 * @param canvas the output canvas
 */
pub fn render(scene: &Scene, canvas: &mut dyn Canvas) {
    let channel_width = canvas.width();
    let channel_height = canvas.height();
    let viewport_width = scene.camera.view_port.width;
    let viewport_height = scene.camera.view_port.height;
    let dist_to_canvat = scene.camera.view_port.distance;

    // Draw each pixel of the canvas
    for v in 0..channel_height {
        for u in 0..channel_width {
            // Compute the direction of the ray
            let direction = Vector3::new(
                (u as f32 - channel_width as f32 / 2.0) * viewport_width / channel_width as f32,
                ((v * channel_height / channel_width) as f32 - channel_height as f32 / 2.0)
                    * viewport_height
                    / channel_height as f32,
                dist_to_canvat,
            );

            // Search the nearest sphere
            let mut closest_t = INFINITY;
            let mut closest_sphere = None;
            for sphere in scene.spheres.iter() {
                let t = sphere.distance_to(scene.camera.position, direction);
                if t >= dist_to_canvat && t <= INFINITY && t < closest_t {
                    closest_t = t;
                    closest_sphere = Some(sphere);
                }
            }

            // Compute the color at the hit position
            let mut color = Color::WHITE;
            if let Some(sphere) = closest_sphere {
                // Compute the position of the hit
                let hit_position = scene.camera.position + direction * closest_t;

                // Compute the normal of the hit
                let normal = (hit_position - sphere.center).normalize();

                // Compute the color
                color = sphere.color
                    * compute_light(
                        &scene.lights,
                        hit_position,
                        normal,
                        -direction,
                        sphere.specular,
                    );
            }

            // Draw the pixel
            canvas.set_pixel(u, v, color);
        }
    }
}

use crate::canvas::Canvas;
use crate::color::Color;
use crate::light::Light;
use crate::math::Vector3;
use crate::scene::Scene;
use std::f32::INFINITY;

fn compute_light(scene: &Scene, p: Vector3, n: Vector3, v: Vector3, s: f32) -> f32 {
    let mut i = 0.0;
    for light in scene.lights.iter() {
        if let Light::Ambient(intensity) = light {
            i += intensity;
        } else {
            let (l, intensity) = {
                let l;
                let intensity;

                match light {
                    Light::OmniDirectional(i, source) => {
                        intensity = i;
                        l = *source - p;
                    }
                    Light::Directional(i, direction) => {
                        intensity = i;
                        l = *direction;
                    }
                    _ => {
                        continue;
                    }
                }

                (l, intensity)
            };

            // Diffuse
            let n_dot_l = n.dot(l);
            if n_dot_l > 0.0 {
                i += intensity * n_dot_l / (n.length() * l.length());
            }

            // // Specular
            if s > -1.0 {
                let r = n * n.dot(l) * 2.0 - l;
                let r_dot_v = r.dot(v);
                if r_dot_v > 0.0 {
                    i += intensity * (r_dot_v / (r.length() * v.length())).powf(s)
                }
            }
        }
    }

    i
}

/**
 * @brief Trace a ray into the scene
 *
 * @param scene the scene
 * @param d direction of the vector
 * @param t_min Where to start the trace
 * @param t_max The upper bound of the trace
 */
fn trace_ray(scene: &Scene, d: Vector3, t_min: f32, t_max: f32) -> Color {
    let mut closest_t = INFINITY;
    let mut closest_sphere = None;
    for sphere in scene.spheres.iter() {
        let t = sphere.distance_to(scene.camera.position, d);
        if t >= t_min && t <= t_max && t < closest_t {
            closest_t = t;
            closest_sphere = Some(sphere);
        }
    }

    if let Some(sphere) = closest_sphere {
        let color = sphere.color;
        let p = scene.camera.position + d * closest_t;
        let n = (p - sphere.center).normalize();
        return color * compute_light(&scene, p, n, -d, sphere.specular);
    }

    Color::WHITE
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
    let vw = scene.camera.view_port.width;
    let vh = scene.camera.view_port.height;
    let dist = scene.camera.view_port.distance;

    // Draw each pixel of the canvas
    for v in 0..channel_height {
        for u in 0..channel_width {
            // Compute the direction of the ray
            let d = Vector3::new(
                (u as f32 - channel_width as f32 / 2.0) * vw / channel_width as f32,
                ((v * channel_height / channel_width) as f32 - channel_height as f32 / 2.0) * vh
                    / channel_height as f32,
                dist,
            );

            // Trace the ray in the canvas
            let color = trace_ray(scene, d, dist, INFINITY);

            // Draw the pixel
            canvas.set_pixel(u, v, color);
        }
    }
}

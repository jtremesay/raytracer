use crate::canvas::Canvas;
use crate::color::Color;
use crate::math::Vector3;
use crate::scene::Scene;
use std::f32::INFINITY;

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
        let mut light_intensity = 0.0;
        for light in scene.lights.iter() {
            light_intensity += light.compute(p, n);
        }
        light_intensity = light_intensity.clamp(0.0, 1.0);

        return color * light_intensity;
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
    let cw = canvas.width();
    let ch = canvas.height();
    let vw = scene.camera.view_port.width;
    let vh = scene.camera.view_port.height;
    let dist = scene.camera.view_port.distance;

    for y in 0..ch {
        for x in 0..cw {
            let d = Vector3::new(
                (x as f32 - cw as f32 / 2.0) * vw / cw as f32,
                ((y * ch / cw) as f32 - ch as f32 / 2.0) * vh / ch as f32,
                dist,
            );
            let color = trace_ray(scene, d, dist, INFINITY);
            canvas.set_pixel(x, y, color);
        }
    }
}

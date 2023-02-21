use crate::{math::Vector3, ray::Hit};

pub trait Light {
    fn compute_intensity(&self, hit: &Hit, inverse_direction: Vector3) -> f32;
}

pub struct AmbiantLight {
    pub intensity: f32,
}

impl Light for AmbiantLight {
    fn compute_intensity(&self, _hit: &Hit, _inverse_direction: Vector3) -> f32 {
        self.intensity
    }
}

pub fn compute_directional_light_intensity(
    intensity: f32,
    direction: Vector3,
    hit: &Hit,
    inverse_direction: Vector3,
) -> f32 {
    // Diffuse
    let n_dot_l = hit.normal.dot(direction);
    let mut i = 0.0;
    if n_dot_l > 0.0 {
        i += intensity * n_dot_l / (hit.normal.length() * direction.length());
    }

    // Specular
    if hit.material.specular > -1.0 {
        let r = hit.normal * hit.normal.dot(direction) * 2.0 - direction;
        let r_dot_v = r.dot(inverse_direction);
        if r_dot_v > 0.0 {
            i += intensity
                * (r_dot_v / (r.length() * inverse_direction.length())).powf(hit.material.specular)
        }
    }

    i
}

pub struct OmniDirectionalLight {
    pub position: Vector3,
    pub intensity: f32,
}

impl Light for OmniDirectionalLight {
    fn compute_intensity(&self, hit: &Hit, inverse_direction: Vector3) -> f32 {
        compute_directional_light_intensity(
            self.intensity,
            self.position - hit.position,
            hit,
            inverse_direction,
        )
    }
}

pub struct DirectionalLight {
    pub direction: Vector3,
    pub intensity: f32,
}

impl Light for DirectionalLight {
    fn compute_intensity(&self, hit: &Hit, inverse_direction: Vector3) -> f32 {
        compute_directional_light_intensity(self.intensity, self.direction, hit, inverse_direction)
    }
}

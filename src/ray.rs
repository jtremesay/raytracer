use crate::{material::Material, math::Vector3};

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

pub struct Hit {
    pub position: Vector3,
    pub normal: Vector3,
    pub distance: f32,
    pub material: Material,
}

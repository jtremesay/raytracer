use crate::{
    material::Material,
    math::Vector3,
    ray::{Hit, Ray},
};

pub trait Node {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

pub struct UnionNode {
    pub node_a: Box<dyn Node>,
    pub node_b: Box<dyn Node>,
}

impl Node for UnionNode {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let hit_a = self.node_a.hit(ray);
        let hit_b = self.node_b.hit(ray);

        if let Some(hit_a) = hit_a {
            if let Some(hit_b) = hit_b {
                if hit_a.distance < hit_b.distance {
                    Some(hit_a)
                } else {
                    Some(hit_b)
                }
            } else {
                Some(hit_a)
            }
        } else {
            if let Some(hit_b) = hit_b {
                Some(hit_b)
            } else {
                None
            }
        }
    }
}

pub struct SphereNode {
    pub position: Vector3,
    pub radius: f32,
    pub material: Material,
}

impl Node for SphereNode {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let co = ray.origin - self.position;
        let a = ray.direction.dot(ray.direction);
        let b = co.dot(ray.direction) * 2.0;
        let c = co.dot(co) - self.radius * self.radius;

        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            return None;
        }

        let t1 = (-b + delta.sqrt()) / (2.0 * a);
        let t2 = (-b - delta.sqrt()) / (2.0 * a);
        let distance = t1.min(t2);
        if distance < 0.0 {
            return None;
        }

        let hit_position = ray.origin + ray.direction * distance;
        let normal = (hit_position - self.position).normalize();

        Some(Hit {
            position: hit_position,
            normal: normal,
            distance: distance,
            material: self.material,
        })
    }
}

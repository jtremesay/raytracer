use crate::color::Color;
use crate::math::Vector3;
use std::f32::INFINITY;

/**
 * @brief A sphere
 */
#[derive(Clone)]
pub struct Sphere {
    /** @brief Center of the sphere */
    pub center: Vector3,
    /** @brief Radius of the sphere */
    pub radius: f32,
    pub color: Color,
}

impl Sphere {
    /**
     * @brief Create a new sphere
     *
     * @param center the center of the sphere
     * @param radius the radius of the sphere
     * @param color the color of the sphere
     */
    pub fn new(center: Vector3, radius: f32, color: Color) -> Self {
        Self {
            center,
            radius,
            color,
        }
    }

    /**
     * @brief Find where the ray intersect the sphere
     *
     * @param o origin of the ray
     * @param d direction of the ray
     */
    pub fn intersect_ray(&self, o: Vector3, d: Vector3) -> (f32, f32) {
        let co = o - self.center;
        let a = d.dot(d);
        let b = co.dot(d) * 2.0;
        let c = co.dot(co) - self.radius * self.radius;

        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            return (INFINITY, INFINITY);
        }

        let t1 = (-b + delta.sqrt()) / (2.0 * a);
        let t2 = (-b - delta.sqrt()) / (2.0 * a);

        (t1, t2)
    }

    /**
     * @brief Find at which minimal distance from the oringin the ray
     *      intersect the sphere
     *
     */
    pub fn distance_to(&self, o: Vector3, d: Vector3) -> f32 {
        // Search the two intersect points
        let (t1, t2) = self.intersect_ray(o, d);

        // Return the nearest
        t1.min(t2)
    }
}

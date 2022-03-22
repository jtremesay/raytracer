use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

/**
 * @brief A vector 3
 */
#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    /**
     * @brief Create a new vector
     *
     * @param x X coordinate
     * @param y Y coordinate
     * @param z Z coordinate
     */
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /**
     * @brief Length of the vector
     *
     * @return the length of the vector
     */
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    /**
     * @brief The dot product of the product with an another product
     */
    pub fn dot(&self, o: Self) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Self;

    fn add(self, o: Self::Output) -> Self::Output {
        Self::Output::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Self;

    fn sub(self, o: Self::Output) -> Self::Output {
        Self::Output::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Self;

    fn mul(self, o: Self::Output) -> Self::Output {
        Self::Output::new(self.x * o.x, self.y * o.y, self.z * o.z)
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Self;

    fn div(self, o: Self::Output) -> Self::Output {
        Self::Output::new(self.x / o.x, self.y / o.y, self.z / o.z)
    }
}

impl Add<f32> for Vector3 {
    type Output = Self;

    fn add(self, o: f32) -> Self::Output {
        Self::Output::new(self.x + o, self.y + o, self.z + o)
    }
}

impl Sub<f32> for Vector3 {
    type Output = Self;

    fn sub(self, o: f32) -> Self::Output {
        Self::Output::new(self.x - o, self.y - o, self.z - o)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, o: f32) -> Self::Output {
        Self::Output::new(self.x * o, self.y * o, self.z * o)
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, o: f32) -> Self::Output {
        Self::Output::new(self.x / o, self.y / o, self.z / o)
    }
}

/**
 * @brief Do the linear interpolation of a value
 *
 * @param x the value to interpolate
 * @param x0
 * @param x1
 * @param y0
 * @param y1
 */
pub fn lerp(x: f32, x0: f32, x1: f32, y0: f32, y1: f32) -> f32 {
    y0 + (x - x0) * ((y1 - y0) / (x1 - x0))
}

/**
 * @brief Do the linear interpolation of a value to the range [0, 1]
 *
 * @param x the value to interpolate
 * @param x0
 * @param x1
 */
pub fn lerp01(x: f32, x0: f32, x1: f32) -> f32 {
    lerp(x, x0, x1, 0.0, 1.0)
}

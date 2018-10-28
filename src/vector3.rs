use std::ops;

use super::{Vector, Vector2};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {

    /// Creates a new `Vector3`
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    /// Shorthand for `Vector3 { x: 0.0, y: 1.0, z: 0.0 }`
    pub fn up() -> Self {
        Vector3 {
            y: 1.0,
            .. Vector3::zero()
        }
    }

    /// Shorthand for `Vector3 { x: 0.0, y: -1.0, z: 0.0 }`
    pub fn down() -> Self {
        Vector3 {
            y: -1.0,
            .. Vector3::zero()
        }
    }

    /// Shorthand for `Vector3 { x: -1.0, y: 0.0, z: 0.0 }`
    pub fn left() -> Self {
        Vector3 {
            x: -1.0,
            .. Vector3::zero()
        }
    }

    /// Shorthand for `Vector3 { x: 1.0, y: 0.0, z: 0.0 }`
    pub fn right() -> Self {
        Vector3 {
            x: 1.0,
            .. Vector3::zero()
        }
    }

    /// Shorthand for `Vector3 {x: 0.0, y: 0.0, z: 1.0 }`
    pub fn forward() -> Self {
        Vector3 {
            z: 1.0,
            .. Vector3::zero()
        }
    }

    /// Shorthand for `Vector3 {x: 0.0, y: 0.0, z: -1.0 }`
    pub fn back() -> Self {
        Vector3 {
            z: -1.0,
            .. Vector3::zero()
        }
    }
}

impl ops::Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Mul for Vector3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::MulAssign for Vector3 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}

impl ops::Div<Vector3> for f64 {
    type Output = Vector3;

    fn div(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
        }
    }
}

impl ops::Div for Vector3 {
    type Output = Vector3;

    fn div(self, other: Self) -> Self {
        Vector3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl ops::DivAssign for Vector3 {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl super::Vector for Vector3 {
    type Scalar = f64;

    fn zero() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn magnitude(&self) -> Self::Scalar {
        self.sqr_magnitude().sqrt()
    }

    fn normalized(self) -> Self {
        let mag = self.magnitude();
        self / mag
    }

    fn normalize(&mut self) {
        *self = self.normalized();
    }

    fn sqr_magnitude(&self) -> Self::Scalar {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }

    fn angle(&self, other: &Self) -> Self::Scalar {
        let dot = self.dot(other);
        let mag = self.magnitude() * other.magnitude();
        (dot / mag).acos()
    }

    fn clamp_magnitude(self, max_len: Self::Scalar) -> Self {
        if self.magnitude() > max_len {
            self / max_len
        } else {
            self
        }
    }

    fn dot(&self, other: &Self) -> Self::Scalar {
        self.x * other.x + self.y * other.y
    }

    fn lerp(self, other: Self, t: Self::Scalar) -> Self {
        if t <= 0.0 {
            self
        } else if t >= 1.0 {
            other
        } else {
            self.lerp_unclamped(other, t)
        }
    }

    fn lerp_unclamped(self, other: Self, t: Self::Scalar) -> Self {
        (1.0 - t) * self + t * other
    }

    // fn move_towards(self, other: Self, max_distance_delta: Self::Scalar) -> Self {
    //     unimplemented!("Unsure how this is supposed to be implemented");
    // }

    /// Reflects the vector along the `normal` vector.
    /// 
    /// Example:
    /// 
    /// ```
    /// # use vector::*;
    /// let a = Vector3::new(1.0, 2.0, 0.0);
    /// let n = Vector3::up();
    /// let r = a.reflect(n);
    /// 
    /// assert_eq!(r, Vector3::new(1.0, -2.0, 0.0));
    /// ```
    fn reflect(self, normal: Self) -> Self {
        -2.0 * self.dot(&normal) * normal + self
    }
}

impl From<Vector2> for Vector3 {

    /// Creates a `Vector3` from a `Vector2`, adding a z component of 0
    fn from(vector: Vector2) -> Vector3 {
        Vector3 {
            x: vector.x,
            y: vector.y,
            z: 0.0,
        }
    }
}
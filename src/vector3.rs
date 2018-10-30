//! Standard implementation of a 3D Vector.
//! 
//! This particular implementation uses 64-bit floating point numbers as its scalar components. 
//! It does so to ease compatibility with [piston.rs](https://www.piston.rs/), as that is what it uses by default for its scalars.

use std::ops;

use super::prelude::{Vector, Vector2};

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

    /// Defines the cross product between two vectors
    /// 
    /// Example:
    /// ```
    /// # use vect::prelude::*;
    /// let a = Vector3::new(3.0, -3.0, 1.0);
    /// let b = Vector3::new(4.0, 9.0, 2.0);
    /// let res = a.cross(b);
    /// let expected = Vector3::new(-15.0, -2.0, 39.0);
    /// 
    /// assert_eq!(res, expected);
    /// ```
    pub fn cross(self, other: Self) -> Self {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Returns the distance between two vectors.
    /// 
    /// `a.distance(b)` is the same as `(a - b).magnitude()`.
    pub fn distance(&self, other: &Self) -> f64 {
        (*self - *other).magnitude()
    }

    /// Returns a new vector that is spherically lerped with relation to t.
    /// Where t is clamped in the range [0, 1]
    /// 
    /// See [Wikipedia](https://en.wikipedia.org/wiki/Slerp#Geometric_Slerp) for the method in which this was calculated.
    pub fn slerp(self, other: Self, t: f64) -> Self {
        // ensure t stays within bounds
        if t <= 0.0 {
            self
        } else if t >= 1.0 {
            other
        } else {
            self.slerp_unclamped(other, t)
        }
    }

    /// Unclamped version of slerp.
    /// Doesn't provide any guarantees on the input
    pub fn slerp_unclamped(self, other: Self, t: f64) -> Self {
        // if cos Ω = p1 dot p2; that must mean Ω = acos (p1 dot p2)
        let omega = self.dot(&other).acos();
        let lhs = (((1.0 - t) * omega).sin() * self) / omega.sin();
        let rhs = ((t * omega).sin() * other) / omega.sin();

        lhs + rhs
    }
}

impl ops::Add for Vector3 {
    type Output = Self;

    /// Adds two vectors together
    fn add(self, other: Self) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign for Vector3 {

    /// Adds two vectors together, and assigns the result into the first vector
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl ops::Sub for Vector3 {
    type Output = Self;

    /// Subtracts two vectors from each other
    fn sub(self, other: Self) -> Self {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::SubAssign for Vector3 {

    /// Subtracts two vectors from each other, and assigns the result into the first vector
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Self;

    /// Multiplies a vector with a scalar
    fn mul(self, rhs: f64) -> Self {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Vector3 {

    /// Multiplies a vector with a scalar, and assigns the result back into the vector
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}

impl ops::Mul<Vector3> for f64 {

    /// Multiplies a scalar with a vector
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Self;

    /// Divides a vector by a scalar
    fn div(self, other: f64) -> Self {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl ops::DivAssign<f64> for Vector3 {
    /// Divides a vector by a scalar, and assigns the result back into the vector
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}

impl ops::Div<Vector3> for f64 {
    type Output = Vector3;

    /// Divides a scalar by a vector
    fn div(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
        }
    }
}

impl Vector for Vector3 {
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

    fn distance(&self, other: &Self) -> f64 {
        self.distance(other)
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

    /// Scales one vector by another by multiplying their components
    fn scale(self, other: Self) -> Self {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
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

    fn move_towards(self, other: Self, max_distance_delta: Self::Scalar) -> Self {
        let distance = self.distance(&other);
        let fraction = max_distance_delta / distance;
        self.lerp_unclamped(other, fraction)
    }

    // fn move_towards(self, other: Self, max_distance_delta: Self::Scalar) -> Self {
    //     unimplemented!("Unsure how this is supposed to be implemented");
    // }

    /// Reflects the vector along the `normal` vector.
    /// 
    /// Example:
    /// 
    /// ```
    /// # use vect::prelude::*;
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
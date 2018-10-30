//! Standard implementation of a 2D Vector.
//! 
//! This particular implementation uses 64-bit floating point numbers as its scalar components. 
//! It does so to ease compatibility with [piston.rs](https://www.piston.rs/), as that is what it uses by default for its scalars.

use std::ops;
use super::prelude::{Vector, Vector3};


#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {

    /// Creates a new `Vector2`
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }

    /// Shorthand for `Vector2 { x: 0.0, y: 1.0 }`
    pub fn up() -> Self {
        Vector2 {
            x: 0.0,
            y: 1.0,
        }
    }

    /// Shorthand for `Vector2 { x: 0.0, y: -1.0 }`
    pub fn down() -> Self {
        Vector2 {
            x: 0.0,
            y: -1.0,
        }
    }

    /// Shorthand for `Vector2 { x: -1.0, y: 0.0 }`
    pub fn left() -> Self {
        Vector2 {
            x: -1.0,
            y: 0.0,
        }
    }

    /// Shorthand for `Vector2 { x: 1.0, y: 0.0 }`
    pub fn right() -> Self {
        Vector2 {
            x: 1.0,
            y: 0.0,
        }
    }

    /// Creates a vector with x and y at +∞
    pub fn positive_infinity() -> Self {
        Vector2 {
            x: std::f64::INFINITY,
            y: std::f64::INFINITY,
        }
    }

    /// Creates a vector with x and y at -∞
    pub fn negative_infinity() -> Self {
        Vector2 {
            x: std::f64::NEG_INFINITY,
            y: std::f64::NEG_INFINITY,
        }
    }
}

impl ops::Add for Vector2 {
    type Output = Self;

    /// Adds two vectors together.
    /// 
    /// Example:
    /// ```
    /// # use vect::prelude::*;
    /// let a = Vector2::new(1.0, 2.0);
    /// let b = Vector2::new(3.0, 4.0);
    /// let res = a + b;
    /// let expected = Vector2::new(4.0, 6.0);
    /// 
    /// assert_eq!(res, expected);
    /// ```
    fn add(self, other: Self) -> Self {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign for Vector2 {

    /// Adds two vectors together and assigns the result back to the first.
    /// 
    /// Example:
    /// ```
    /// # use vect::prelude::*;
    /// let mut a = Vector2::new(1.0, 2.0);
    /// let b = Vector2::new(3.0, 4.0);
    /// a += b;
    /// let expected = Vector2::new(4.0, 6.0);
    /// 
    /// assert_eq!(a, expected);
    /// ```
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl ops::Sub for Vector2 {
    type Output = Self;

    /// Subtracts two vectors from each other.
    /// 
    /// Example:
    /// ```
    /// # use vect::prelude::*;
    /// let a = Vector2::new(5.0, 8.0);
    /// let b = Vector2::new(6.0, 4.0);
    /// let res = a - b;
    /// let expected = Vector2::new(-1.0, 4.0);
    /// 
    /// assert_eq!(res, expected);
    /// ```
    fn sub(self, other: Self) -> Self {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::SubAssign for Vector2 {

    /// Subtracts two vectors from each other and assigns the result back to the first.
    /// 
    /// Example:
    /// ```
    /// # use vect::prelude::*;
    /// let mut a = Vector2::new(5.0, 8.0);
    /// let b = Vector2::new(6.0, 4.0);
    /// a -= b;
    /// let expected = Vector2::new(-1.0, 4.0);
    /// 
    /// assert_eq!(a, expected);
    /// ```
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl ops::Mul<f64> for Vector2 {
    type Output = Self;

    /// Multiplies the vector with some scalar
    /// 
    /// Example:
    /// ```
    /// # use vect::prelude::*;
    /// let a = Vector2::new(3.0, 4.0);
    /// let res = a * 3.0;
    /// let expected = Vector2::new(9.0, 12.0);
    /// 
    /// assert_eq!(res, expected);
    /// ```
    fn mul(self, rhs: f64) -> Self {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Vector2 {

    /// Multiplies the vector with some scalar and assigns the result back onto the vector
    /// 
    /// Example:
    /// ```
    /// # use vect::prelude::*;
    /// let mut a = Vector2::new(3.0, 4.0);
    /// a *= 3.0;
    /// let expected = Vector2::new(9.0, 12.0);
    /// 
    /// assert_eq!(a, expected);
    /// ```
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}

impl ops::Mul<Vector2> for f64 {
    type Output = Vector2;

    /// Multiplies a scalar with some vector
    /// 
    /// Example:
    /// ```
    /// # use vect::prelude::*;
    /// let a = Vector2::new(2.5, 5.0);
    /// let res = 2.0 * a;
    /// let expected = Vector2::new(5.0, 10.0);
    /// 
    /// assert_eq!(res, expected);
    /// ```
    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl ops::Div<f64> for Vector2 {
    type Output = Self;

    /// Divides the vector with some scalar
    fn div(self, other: f64) -> Self {
        Vector2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl ops::DivAssign<f64> for Vector2 {

    /// Divides the vector with some scalar and assigns the result back into the vector
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}

impl ops::Div<Vector2> for f64 {
    type Output = Vector2;

    /// Divides a scalar with some vector
    fn div(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self / other.x,
            y: self / other.y,
        }
    }
}

impl Vector for Vector2 {
    type Scalar = f64;

    fn zero() -> Self {
        Vector2 {
            x: 0.0,
            y: 0.0,
        }
    }

    fn magnitude(&self) -> Self::Scalar {
        self.sqr_magnitude().sqrt()
    }

    fn distance(&self, other: &Self) -> Self::Scalar {
        (*self - *other).magnitude()
    }

    fn normalized(self) -> Self {
        let mag = self.magnitude();
        self / mag
    }

    fn normalize(&mut self) {
        *self = self.normalized();
    }

    fn sqr_magnitude(&self) -> Self::Scalar {
        (self.x.powi(2) + self.y.powi(2))
    }

    fn angle(&self, other: &Self) -> Self::Scalar {
        let dot = self.dot(other);
        let mag = self.magnitude() * other.magnitude();
        (dot / mag).acos()
    }

    fn project(self, other: Self) -> Self {
        (self.dot(&other) / other.dot(&other)) * other
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

    fn scale(self, other: Self) -> Self {
        Vector2 {
            x: self.x * other.x,
            y: self.y * other.y,
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
    /// let a = Vector2::new(1.0, 2.0);
    /// let n = Vector2::up();
    /// let r = a.reflect(n);
    /// 
    /// assert_eq!(r, Vector2::new(1.0, -2.0));
    /// ```
    fn reflect(self, normal: Self) -> Self {
        -2.0 * self.dot(&normal) * normal + self
    }
}

impl From<Vector3> for Vector2 {
    /// Turns a `Vector3` into a `Vector2`, discarding the z component.
    fn from(vector: Vector3) -> Vector2 {
        Vector2 {
            x: vector.x,
            y: vector.y,
        }
    }
}

impl From<(f64, f64)> for Vector2 {
    fn from((x, y): (f64, f64)) -> Vector2 {
        Vector2 { x, y }
    }
}

impl From<Vector2> for (f64, f64) {
    fn from(Vector2 {x, y}: Vector2) -> (f64, f64) {
        (x, y)
    }
}

impl From<[f64; 2]> for Vector2 {
    fn from([x, y]: [f64; 2]) -> Vector2 {
        Vector2 { x, y }
    }
}

impl From<Vector2> for [f64; 2] {
    fn from(Vector2 {x, y}: Vector2) -> [f64; 2] {
        [x, y]
    }
}
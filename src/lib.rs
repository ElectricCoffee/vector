use std::ops;

pub mod vector2;
pub use vector2::Vector2;

pub mod vector3;
pub use vector3::Vector3;

pub trait Vector: 
    ops::Add 
    + ops::AddAssign 
    + ops::Sub 
    + ops::SubAssign 
    + ops::Mul
    + ops::MulAssign
    + ops::Div 
    + ops::DivAssign 
    + PartialEq 
    + PartialOrd
    + Sized 
{
    type Scalar;

    /// The Zero vector
    fn zero() -> Self;

    /// Gets the magnitude (length) of the vector
    fn magnitude(&self) -> Self::Scalar;

    /// Returns a vector with the same angle, but with a magnitude of 1
    fn normalized(self) -> Self;

    /// Mutates the vector such that it gains a magnitude of 1
    fn normalize(&mut self);

    /// Returns the magnitude squared
    fn sqr_magnitude(&self) -> Self::Scalar;

    /// Returns the angle between two vectors
    fn angle(&self, other: &Self) -> Self::Scalar;

    /// Returns a new vector with a magnitude clamped to `max_len`.
    fn clamp_magnitude(self, max_len: Self::Scalar) -> Self;

    /// Returns the dot product between two vectors
    fn dot(&self, other: &Self) -> Self::Scalar;

    /// Performs a linear interpolation between `self` and `other` over `t`.
    /// 
    /// `t` is clamped to the range [0, 1].
    /// 
    /// * when `t` = 0, it returns `self`
    /// * when `t` = 0.5, it returns a vector half-way between `self` and `other`
    /// * when `t` = 1, it returns `other`.
    /// 
    /// Lerp guarantees the interpolation will never exceed the range [0, 1] for `t`
    fn lerp(self, other: Self, t: Self::Scalar) -> Self;

    /// Performs a linear interpolation, where `t` isn't clamped between 0 and 1.
    /// 
    /// Provides no guarantees on the interpolation.
    fn lerp_unclamped(self, other: Self, t: Self::Scalar) -> Self;

    /// Returns the largest of the two vectors
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }

    /// Returns the smallest of the two vectors
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }

    // /// Essentially the same as Vector::lerp, but the function will instead ensure the distance never exceeds `max_distance_delta`.
    // /// Negative values of `max_distance_delta` pushes the vector away from `target`.
    // fn move_towards(self, target: Self, max_distance_delta: Self::Scalar) -> Self;

    /// Reflects the vector along the `normal` vector.
    fn reflect(self, normal: Self) -> Self;
}

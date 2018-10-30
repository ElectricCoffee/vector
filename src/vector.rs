//! Trait which encompasses the standard feature set of a vector.

use std::ops;

/// Base Vector trait.
pub trait Vector: 
    ops::Add 
    + ops::AddAssign 
    + ops::Sub 
    + ops::SubAssign
    + PartialEq 
    + PartialOrd
    + Sized 
{
    // Associated type, which sets the scalar type of a given implementation.
    // A scalar is just a regular non-vector number.
    type Scalar;

    /// The Zero vector
    fn zero() -> Self;

    /// Gets the magnitude (length) of the vector
    fn magnitude(&self) -> Self::Scalar;

    /// Gets the distance between two vectors
    fn distance(&self, other: &Self) -> Self::Scalar;

    /// Returns a vector with the same angle, but with a magnitude of 1
    fn normalized(self) -> Self;

    /// Mutates the vector such that it gains a magnitude of 1
    fn normalize(&mut self);

    /// Returns the magnitude squared
    fn sqr_magnitude(&self) -> Self::Scalar;

    /// Returns the angle between two vectors
    fn angle(&self, other: &Self) -> Self::Scalar;

    /// Projects the vector onto the other vector
    fn project(self, other: Self) -> Self;

    /// Returns a new vector with a magnitude clamped to `max_len`.
    fn clamp_magnitude(self, max_len: Self::Scalar) -> Self;

    /// Returns the dot product between two vectors.
    fn dot(&self, other: &Self) -> Self::Scalar;

    /// Scales one vector by another.
    fn scale(self, other: Self) -> Self;

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

    /// Essentially the same as Vector::lerp, but the function will instead ensure the distance never exceeds `max_distance_delta`.
    /// Negative values of `max_distance_delta` pushes the vector away from `target`.
    fn move_towards(self, target: Self, max_distance_delta: Self::Scalar) -> Self;

    /// Reflects the vector along the `normal` vector.
    fn reflect(self, normal: Self) -> Self;
}

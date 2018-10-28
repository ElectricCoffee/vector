//! # vector
//! 
//! Vector is a crate for providing simple and easy to use vectors for use in game development in the style of those found in the Unity game engine.
//! 
//! The library provides three things out of the box: the `Vector` trait, which contains methods like `normalize`, `lerp`, and `zero`, which are common across all vector implementations; the `Vector2` struct, which provides a base-implementation for a 2D vector with `f64` fields; and finally `Vector3`, which provides a base-implementation of a 3D vector, also with `f64` fields.
pub mod vector2;
pub mod vector3;
pub mod vector;
pub mod prelude;
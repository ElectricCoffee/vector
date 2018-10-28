# vector

Vector is a crate for providing simple and easy to use vectors for use in game development in the style of those found in the Unity game engine.

The library provides three things out of the box: the `Vector` trait, which contains methods like `normalize`, `lerp`, and `zero`, which are common across all vector implementations; the `Vector2` struct, which provides a base-implementation for a 2D vector with `f64` fields; and finally `Vector3`, which provides a base-implementation of a 3D vector, also with `f64` fields.

As mentioned, the aim of this library is to be closely modeled to that found in Unity, to provide an easier transition for those already familiar.
The things you can do in Unity:

```csharp
var start = new Vector2(1f, 2f);
var end = new Vector2(3f, 8f);

var newVector = Vector2.lerp(start, end, t);
```
Should be directly translateable into this API:

```rust
use vect::prelude::*;

let start = Vector2::new(1.0, 2.0);
let end = Vector2::new(3.0, 8.0);

let newVector = Vector2::lerp(start, end, t);
```
And due to a quirk in how Rust handles methods, I've gone ahead and implemented all the "static methods" (aka associated functions) as actual methods, so you don't have to think about whether it's one or the other:
```rust
use vect::prelude::*;

let start = Vector2::new(1.0, 2.0);
let end = Vector2::new(3.0, 8.0);

let newVector = start.lerp(end, t);
```
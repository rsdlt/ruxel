// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/**
The geometry module implements the functionality for Points, Vectors, Matrices, and their transformations
*/

/// Provides an epsilon to compare floating point numbers with suitable precision for this project
pub const EPSILON: f64 = 0.0001;

// Bring into scope the f64 math constants in the standard library
use std::f64::consts::PI;

/// Provides data structures, methods and traits for Matrix4 computations.
pub mod matrix;
/// Data structures and methods for Vector3 and Point3 computations.
pub mod vector;

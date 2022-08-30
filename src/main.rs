// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(missing_docs, missing_debug_implementations)]

/*!
# Ruxel

**Ruxel** is a simple ray tracer and renderer written in Rust.

Ruxel allows rendering and ray tracing of:
- Shapes: Spheres, Planes, Cubes, Cylinders, Triangles, Patterns and OBJ files
- Attributes: Lights, Shades, Shadows, Patterns, Reflection and Refraction

*/

#![allow(unused_imports)]

use geometry::{
    matrix::{Matrix4, Matrix4Ops},
    vector::{
        Axis,
        Axis::{XY as xy, XYZ as xyz, XYZW as xyzw},
        CoordInit, Point3, Vector3,
    },
    ray::{Ray},
};

use picture::colors::{ColorInit, ColorRgb};

/**
The geometry module implements the functionality for Points, Vectors, Matrices, and their transformations
*/
pub mod geometry;

/**
The picture module implements the functionality for Canvas and Colors in order to create an image file.
*/
pub mod picture;

fn main() {
}

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

use geometry::vector::{
    Axis,
    Axis::{XY as xy, XYZ as xyz, XYZW as xyzw},
    CoordInit, Point3, Vector3,
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
    let v = Vector3::one();
    let p = Point3::one();
    println!("vector: {:?} \n point: {:?}", v, p);

    let c = ColorRgb::red();
    println!("{}", c);

    // let vn = Vector3::new_new(Axis::XYZ {x: 1.0, y:2.0, z:4.0 });
    // let vnn = Vector3::new_new(Axis::XYZW { x: 0.0, y: 2.0, z: 3.0, w: 4.0 });
    let vec2 = Vector2::new(xy(1.0, 2.0));
    let vec3 = Vector3::new(xyz(1.0, 2.0, 3.0));
    let vec4 = Vector4::new(xyzw(1.0, 2.0, 3.0, 4.0));
    let point2 = Point2::new(xy(1.0, 2.0));
    let point3 = Point3::new(xyz(1.0, 2.0, 3.0));
    let point4 = Point4::new(xyzw(1.0, 2.0, 3.0, 4.0));
}

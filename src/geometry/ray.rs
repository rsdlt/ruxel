// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::shapes::*;
use num::{Num, NumCast};
use std::fmt::Display;
/**
 Data structures and methods for Ray and Intersection computations
*/
// Unit tests for Ray
#[cfg(test)]
mod tests;

// Bring geometry module constants into scope
use super::{vector::*, EPSILON};

/// Type representing a Ray with an Origin (Point3) and
/// a Direction (Vector3).
#[derive(Clone, Copy, Debug)]
pub struct Ray<P> {
    /// Origin of a Ray represented by a Point3 type.
    pub origin: Point3<P>,

    /// Direction of a Ray represented by a Vector3 type.
    pub direction: Vector3<P>,
}

impl<P> Display for Ray<P>
where
    P: Num + Copy + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("ray: ogn -> {}\tdir -> {}", self.origin, self.direction);
        f.write_str(&s)
    }
}

/// A trait that provides common operations for Rays
pub trait Rays<P> {
    /// Creates and returns a new Ray with Origin (Point3)
    /// and Direction (Vector3).
    fn new(origin: Point3<P>, direction: Vector3<P>) -> Self;

    /// Calculates a Position (Point3) based on a
    /// Ray and a distance 't'.
    fn position(ray: Ray<P>, t: P) -> Point3<P>;
}

impl<P> Rays<P> for Ray<P>
where
    P: Num + Copy,
{
    fn new(origin: Point3<P>, direction: Vector3<P>) -> Self {
        Self { origin, direction }
    }

    fn position(ray: Ray<P>, t: P) -> Point3<P> {
        ray.origin + ray.direction * t
    }
}

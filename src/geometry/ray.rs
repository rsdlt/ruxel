// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::shapes::*;
use num::{Num, NumCast};
/**
 Data structures and methods for Ray and Intersection computations
*/
use std::fmt::Display;
// Unit tests for Ray
#[cfg(test)]
mod tests;

// Bring geometry module constants into scope
use super::{vector::*, EPSILON};

/// Type representing a Ray
#[derive(Clone, Copy, Debug)]
pub struct Ray<P> {
    /// .
    pub origin: Point3<P>,

    /// .
    pub direction: Vector3<P>,
}

/// Type representing an Intersection between a Ray and a Shape
/// For every shape we can have 0..n intersections
#[derive(Debug)]
pub struct Intersection {
    /// .
    pub shape: Shape,
    /// .
    pub t: f64,
}

impl<P> Display for Ray<P>
where
    P: Num + Copy + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("ray:\nogn->{}\tdir->{}", self.origin, self.direction);
        f.write_str(&s)
    }
}

/// A trait that provides capabilities to initialize a Ray
pub trait RayInit<P> {
    /// .
    fn new(origin: Point3<P>, direction: Vector3<P>) -> Self;
}

impl<P> RayInit<P> for Ray<P>
where
    P: Num + Copy,
{
    fn new(origin: Point3<P>, direction: Vector3<P>) -> Self {
        Self { origin, direction }
    }
}

/// A trait that provides common operations for Rays
pub trait RayOps<P> {
    /// .
    fn position(ray: Ray<P>, t: P) -> Point3<P>;
}

impl<P> RayOps<P> for Ray<P>
where
    P: Copy + Num,
{
    fn position(ray: Ray<P>, t: P) -> Point3<P> {
        ray.origin + ray.direction * t
    }
}

impl Intersection {
    fn intersection(t: f64, shape: Shape) -> Self {
        Self { shape, t }
    }
}

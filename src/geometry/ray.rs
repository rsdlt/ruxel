// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/** 
 Data structures and methods for Ray computations
*/

use std::fmt::Display;


// Unit tests for Ray 
#[cfg(test)]
mod tests;

// Bring geometry module constants into scope
use super::{EPSILON, vector::*};

/// Type representing a Ray
#[derive(Debug, Clone, Copy)]
pub struct Ray<T>{
    pub origin: Point3<T>,
    pub direction: Vector3<T>,
}

impl Display for Ray<f64>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("ray:\nogn->{}\tdir->{}", self.origin, self.direction);
        f.write_str(&s)
    }
}

/// A trait that provides capabilities to initialize a Ray
pub trait RayInit<T>{
    /// .
    fn new(origin: Point3<T>, direction: Vector3<T>) -> Self;
}

impl RayInit<f64> for Ray<f64>{
    fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Self {
        Self{ origin, direction}
    }
}

/// A trait that provides common operations for Rays
pub trait RayOps<T>{
    /// .
    fn position(ray: Ray<T>, t: T) -> Point3<T>;
}

impl RayOps<f64> for Ray<f64>{
    fn position(ray: Ray<f64>, t: f64) -> Point3<f64> {
       ray.origin + ray.direction * t
    }
}


// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/**
  Data structures and methods for Vector3 and Point3 computations.
*/
// Bring overflow operator's traits into scope
use std::ops::Add;

/// Type representing a geometric 3D Vector with x, y, z components.
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    /// Component on x axis
    pub x: f64,
    /// Component on y axis
    pub y: f64,
    /// Component on z axis
    pub z: f64,
}

/// Type representing a geometric 3D Point with x, y, z components.  
#[derive(Debug, Clone, Copy)]
pub struct Point3 {
    /// Component on x axis
    pub x: f64,
    /// Component on y axis
    pub y: f64,
    /// Component on z axis
    pub z: f64,
    /// Component on w axis
    pub w: f64,
}

/// A trait allows Vector3 and Point3 types to be efficiently initialized with common shorthand
pub trait NewDecl<T> {
    /// Return a Vector3 or Point3 type with user-defined x, y, z components
    fn new(x: f64, y: f64, z: f64) -> T;
    /// Return a Vector3 or Point3 type with shorthand [1, 1, 1]
    fn one() -> T;
    /// Return a Vector 3 or Point3 type with shorthand [0, 0, 0]
    fn zero() -> T;
}

impl NewDecl<Vector3> for Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }
    fn one() -> Self {
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
    fn zero() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl NewDecl<Point3> for Point3 {
    fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 { x, y, z, w: 1f64 }
    }
    fn one() -> Self {
        Point3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        }
    }

    fn zero() -> Self {
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}

impl Add<Point3> for Vector3 {
    type Output = Point3;

    fn add(self, other: Point3) -> Point3 {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: other.w,
        }
    }
}

impl Add<Vector3> for Point3 {
    type Output = Point3;

    fn add(self, other: Vector3) -> Point3 {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: other.y,
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Self) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

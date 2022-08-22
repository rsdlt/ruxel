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
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

// Unit tests for Vector3 and Point3
#[cfg(test)]
mod tests;

// TODO: Magnitude, Normalization, Dot Product, Cross Product

// Bring geometry module constants into scope
use super::EPSILON;

/// Type representing a geometric 3D Vector with x, y, z components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    /// Component on x axis
    pub x: f64,
    /// Component on y axis
    pub y: f64,
    /// Component on z axis
    pub z: f64,
}

/// Type representing a geometric 3D Point with x, y, z components.  
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Default for Point3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// A trait allows Vector3 and Point3 types to be efficiently initialized with common shorthand
pub trait NewInit<T> {
    /// Return a Vector3 or Point3 type with shorthand [0, 0, -1]
    fn back() -> T;
    /// Return a Vector3 or Point3 type with shorthand [0, -1, 0]
    fn down() -> T;
    /// Return true if a Vector or Point3 is identical to another, else return false
    fn equal(self, other: Self) -> bool;
    /// Return a Vector3 or Point3 type with shorthand [0, 0, 1]
    fn forward() -> T;
    /// Return a Vector3 or Point3 type with shorthand [-1, 0, 0]
    fn left() -> T;
    /// Return a Vector3 or Point3 type with user-defined x, y, z components
    fn new(x: f64, y: f64, z: f64) -> T;
    /// Return a Vector3 or Point3 type with shorthand [1, 1, 1]
    fn one() -> T;
    /// Return a Vector3 or Point3 type with shorthand [1, 0, 0]
    fn right() -> T;
    /// Return a Vector3 or Point3 type with shorthand [0, 1, 0]
    fn up() -> T;
    /// Return a Vector3 or Point3 type with shorthand [0, 0, 0]
    fn zero() -> T;
}

impl NewInit<Vector3> for Vector3 {
    fn back() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        }
    }
    fn down() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        }
    }
    fn equal(self, other: Self) -> bool {   
    if (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON && (self.z - other.z).abs() < EPSILON 
    {
          true
        } else {
          false
        }
    }

    fn forward() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    fn left() -> Vector3 {
        Vector3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        }
    }

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

    fn right() -> Vector3 {
        Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn up() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
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

impl NewInit<Point3> for Point3 {
    fn back() -> Point3 {
        Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
            w: 1.0,
        }
    }

    fn down() -> Point3 {
        Point3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
            w: 1.0,
        }
    }

    fn equal(self, other: Self) -> bool {
        if (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
            && (self.w - other.w).abs() < EPSILON
        {
            true
        } else {
            false
        }
    }

    fn forward() -> Point3 {
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 1.0,
        }
    }

    fn left() -> Point3 {
        Point3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

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

    fn right() -> Point3 {
        Point3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    fn up() -> Point3 {
        Point3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
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
            w: self.w,
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

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Self) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for Point3 {
    type Output = Vector3;

    fn sub(self, other: Self) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Vector3> for Point3 {
    type Output = Point3;

    fn sub(self, other: Vector3) -> Point3 {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w,
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for Point3 {
    type Output = Point3;

    fn neg(self) -> Point3 {
        Point3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<f64> for Point3 {
    type Output = Point3;

    fn mul(self, other: f64) -> Point3 {
        Point3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w,
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, other: f64) -> Vector3 {
            Vector3 {
                x: self.x / other,
                y: self.y / other,
                z: self.z / other,
            }
        }
    }

impl Div<f64> for Point3 {
    type Output = Point3;
        fn div(self, other: f64) -> Point3 {
            Point3 {
                x: self.x / other,
                y: self.y / other,
                z: self.z / other,
                w: self.w,
            }
        }
    }

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


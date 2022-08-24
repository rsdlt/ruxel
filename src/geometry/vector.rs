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
use std::{
    cmp::{Eq, PartialEq},
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

// Unit tests for Vector3 and Point3
#[cfg(test)]
mod tests;

// Bring geometry module constants into scope
use super::EPSILON;

/// Type representing a geometric 3D Vector with x, y, z components.
#[derive(Debug, Clone, Copy)]
pub struct Vector3<T> {
    /// Component on x axis
    pub x: T,
    /// Component on y axis
    pub y: T,
    /// Component on z axis
    pub z: T,
}

/// Type representing a geometric 3D Point with x, y, z components.  
#[derive(Debug, Clone, Copy)]
pub struct Point3<T> {
    /// Component on x axis
    pub x: T,
    /// Component on y axis
    pub y: T,
    /// Component on z axis
    pub z: T,
    /// Component representing the 'weight' 
    pub w: T,
}

impl PartialEq for Vector3<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for Vector3<f64> {}

impl PartialEq for Point3<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for Point3<f64> {}

impl Display for Vector3<f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("v: [{:^5.2},{:^5.2},{:^5.2}]", self.x, self.y, self.z);
        f.write_str(&s)
    }
}

impl Display for Point3<f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!(
            "p: [{:^5.2},{:^5.2},{:^5.2},{:^5.2}]",
            self.x, self.y, self.z, self.w
        );
        f.write_str(&s)
    }
}

impl Default for Point3<f64> {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}

impl Default for Vector3<f64> {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

// TODO:  Ord, PartialOrd, , Debug for Types

/// A trait allows Types with x, y, z coordinates to be efficiently initialized with common shorthand.
pub trait CoordInit<T> {
    /// Return a type with shorthand [0, 0, -1].
    fn back() -> T;
    /// Return a type with shorthand [0, -1, 0].
    fn down() -> T;
    /// Return true if a type is identical to another, else return false.
    fn equal(self, rhs: Self) -> bool;
    /// Return a type with shorthand [0, 0, 1].
    fn forward() -> T;
    /// Return a type with shorthand [-1, 0, 0].
    fn left() -> T;
    /// Return a type with user-defined x, y, z components.
    fn new(x: f64, y: f64, z: f64) -> T;
    /// Return a type with shorthand [1, 1, 1].
    fn one() -> T;
    /// Return a type with shorthand [1, 0, 0].
    fn right() -> T;
    /// Return a type with shorthand [0, 1, 0].
    fn up() -> T;
    /// Return a type with shorthand [0, 0, 0].
    fn zero() -> T;
}

/// A trait that encapsulates common Vector Operations.
pub trait VecOps<T> {
    /// Computes the magnitude of a Vector.
    fn magnitude(&self) -> f64;
    /// Returns the vector normalized (with magnitude of 1.0)
    fn normalized(&mut self) -> Self;
    /// Returns the Dot Product of two Vectors.
    fn dot(lhs: T, rhs: T) -> f64;
    /// Returns the Cross Product of two Vectors.
    fn cross(lhs: T, rhs: T) -> T;
    /// Returns the Smallest component in the Vector.
    fn min_component(&self) -> (i8, char, f64);
    /// Returns the Largest component in the Vector.
    fn max_component(&self) -> (i8, char, f64);
    /// Returns the component of the Vector by index. this(1)
    fn this(&self, index: i8) -> Option<(i8, char, f64)>;
    /// Returns the component of the Vector by name. this_n('x')
    fn this_name(&self, index: char) -> Option<(i8, char, f64)>;
}

impl VecOps<Vector3<f64>> for Vector3<f64> {
    fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    fn normalized(&mut self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    fn dot(lhs: Vector3<f64>, rhs: Vector3<f64>) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    fn cross(lhs: Vector3<f64>, rhs: Vector3<f64>) -> Vector3<f64> {
        Vector3 {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

    fn min_component(&self) -> (i8, char, f64) {
        if self.x <= self.y && self.x <= self.z {
            (0, 'x', self.x)
        } else if self.y <= self.z {
            (1, 'y', self.y)
        } else {
            (2, 'z', self.z)
        }
    }

    fn max_component(&self) -> (i8, char, f64) {
        if self.x >= self.y && self.x >= self.z {
            (0, 'x', self.x)
        } else if self.y >= self.z {
            (1, 'y', self.y)
        } else {
            (2, 'z', self.z)
        }
    }

    fn this(&self, index: i8) -> Option<(i8, char, f64)> {
        match index {
            0 => Some((0, 'x', self.x)),
            1 => Some((1, 'y', self.y)),
            2 => Some((2, 'z', self.z)),
            _ => None,
        }
    }

    fn this_name(&self, index: char) -> Option<(i8, char, f64)> {
        match index {
            'x' => Some((0, 'x', self.x)),
            'y' => Some((1, 'y', self.y)),
            'z' => Some((2, 'z', self.z)),
            _ => None,
        }
    }
}

impl CoordInit<Vector3<f64>> for Vector3<f64> {
    fn back() -> Vector3<f64> {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        }
    }
    fn down() -> Vector3<f64> {
        Vector3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        }
    }
    fn equal(self, other: Self) -> bool {
        if (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
        {
            true
        } else {
            false
        }
    }

    fn forward() -> Vector3<f64> {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    fn left() -> Vector3<f64> {
        Vector3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn new(x: f64, y: f64, z: f64) -> Vector3<f64> {
        Vector3 { x, y, z }
    }

    fn one() -> Self {
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    fn right() -> Vector3<f64> {
        Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn up() -> Vector3<f64> {
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

impl CoordInit<Point3<f64>> for Point3<f64> {
    fn back() -> Point3<f64> {
        Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
            w: 1.0,
        }
    }

    fn down() -> Point3<f64> {
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

    fn forward() -> Point3<f64> {
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 1.0,
        }
    }

    fn left() -> Point3<f64> {
        Point3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    fn new(x: f64, y: f64, z: f64) -> Point3<f64> {
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

    fn right() -> Point3<f64> {
        Point3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    fn up() -> Point3<f64> {
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

impl Add<Point3<f64>> for Vector3<f64> {
    type Output = Point3<f64>;

    fn add(self, rhs: Point3<f64>) -> Point3<f64> {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: rhs.w,
        }
    }
}

impl Add<Vector3<f64>> for Point3<f64> {
    type Output = Point3<f64>;

    fn add(self, rhs: Vector3<f64>) -> Point3<f64> {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w,
        }
    }
}

impl Add for Vector3<f64> {
    type Output = Vector3<f64>;

    fn add(self, rhs: Self) -> Vector3<f64> {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3<f64> {
    type Output = Vector3<f64>;

    fn sub(self, rhs: Self) -> Vector3<f64> {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for Point3<f64> {
    type Output = Vector3<f64>;

    fn sub(self, rhs: Self) -> Vector3<f64> {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vector3<f64>> for Point3<f64> {
    type Output = Point3<f64>;

    fn sub(self, rhs: Vector3<f64>) -> Point3<f64> {
        Point3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w,
        }
    }
}

impl Neg for Vector3<f64> {
    type Output = Vector3<f64>;

    fn neg(self) -> Vector3<f64> {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for Point3<f64> {
    type Output = Point3<f64>;

    fn neg(self) -> Point3<f64> {
        Point3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
}

impl Mul<f64> for Vector3<f64> {
    type Output = Vector3<f64>;

    fn mul(self, rhs: f64) -> Vector3<f64> {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f64> for Point3<f64> {
    type Output = Point3<f64>;

    fn mul(self, rhs: f64) -> Point3<f64> {
        Point3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w,
        }
    }
}

impl Div<f64> for Vector3<f64> {
    type Output = Vector3<f64>;
    fn div(self, rhs: f64) -> Vector3<f64> {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<f64> for Point3<f64> {
    type Output = Point3<f64>;
    fn div(self, rhs: f64) -> Point3<f64> {
        Point3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w,
        }
    }
}

impl AddAssign for Vector3<f64> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl SubAssign for Vector3<f64> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

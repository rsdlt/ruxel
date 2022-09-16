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
// Bring operator overloading and relevant standard library traits into scope
use std::{
    cmp::{Eq, PartialEq},
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

// Bring Num crate into scope to provide multiplicative identities for One and Zero
use num::Num;

// Bring geometry module constants into scope
use super::EPSILON;

// Unit tests for Vector3 and Point3
#[cfg(test)]
mod tests;

/**
Enumerator that encapsulates the different coordinate systems used to initialize a Vector or Point
Point
*/
#[derive(Debug)]
pub enum Axis<U> {
    /// Coordinate system with X and Y axis.
    XY(U, U),
    /// Coordinate system with X, Y and Z axis.
    XYZ(U, U, U),
    /// Coordinate system with X, Y, Z axis and W component.
    XYZW(U, U, U, U),
}

/// Type representing a geometric 3D Vector in its 'homogeneous' form with x, y, z, w components,
/// and where 'w' stands for 'weight'
#[derive(Debug, Default, Clone, Copy)]
pub struct Vector3<P> {
    /// Component on x axis
    pub x: P,
    /// Component on y axis
    pub y: P,
    /// Component on z axis
    pub z: P,
    /// Component on w axis
    pub w: P,
}

/// Type representing a geometric 3D Point in its 'homogeneous' form with x, y, z components, and
/// where 'W' stands for 'weight'
#[derive(Debug, Default, Clone, Copy)]
pub struct Point3<P> {
    /// Component on x axis
    pub x: P,
    /// Component on y axis
    pub y: P,
    /// Component on z axis
    pub z: P,
    /// Component representing the 'weight'
    pub w: P,
}

/// Trait that enabling generic Vector3 and Point3 types to be initialized
trait Tuple<P> {
    /// Create a new Vector3 or Point3 with user-defined values
    fn new(x: P, y: P, z: P) -> Self
    where
        P: Copy + Num;

    /// Create a new Vector3 or Point3 where all coordinates (x, y, z) have the same
    /// user-defined value
    fn all(all: P) -> Self
    where
        P: Copy + Num;

    fn equal(self, rhs: Self) -> bool;

    /// Create a new Vector3 or Point3 where in the form of [x_val, 0, 0] where
    /// x_val is a user-defined value
    fn x_coord(x_val: P) -> Self
    where
        P: Copy + Num;

    /// Create a new Vector3 or Point3 where in the form of [0, y_val, 0] where
    /// y_val is a user-defined value
    fn y_coord(y_val: P) -> Self
    where
        P: Copy + Num;

    /// Create a new Vector3 or Point3 where in the form of [0, 0, z_val] where
    /// z_val is a user-defined value
    fn z_coord(z_val: P) -> Self
    where
        P: Copy + Num;
}

/// Trait that enables specific Vector3 capabilities
trait Vector<P>: Tuple<P> {
    /// Calculate the dot product of two Vector3
    fn dot(lhs: Vector3<P>, rhs: Vector3<P>) -> P
    where
        P: Copy + Add<Output = P> + Mul<Output = P>;
}

/// Trait that enables specific Point3 capabilities
trait Point<P>: Tuple<P> {
    /// Set up the origin coordinates of a Point3
    fn origin(origin: P) -> Self
    where
        P: Copy + Default;
}

/// Implementation of initialization capabilities for Vector3
impl<P> Tuple<P> for Vector3<P> {
    fn new(x: P, y: P, z: P) -> Vector3<P>
    where
        P: Copy + num::Zero,
    {
        Vector3 {
            x,
            y,
            z,
            w: num::zero(),
        }
    }

    fn all(all: P) -> Self
    where
        P: Copy + num::One + num::Zero + Mul<Output = P>,
    {
        Vector3 {
            x: num::one::<P>() * all,
            y: num::one::<P>() * all,
            z: num::one::<P>() * all,
            w: num::zero::<P>() * all,
        }
    }

    fn equal(self<f64>, rhs: Vector3<f64>) -> bool {
        if ((self.x - rhs.x) as f64).abs() < EPSILON
            && (self.y - rhs.y).abs() < EPSILON
            && (self.z - rhs.z).abs() < EPSILON
        {
            true
        } else {
            false
        }
    }

    fn x_coord(x_val: P) -> Self
    where
        P: Copy + Num,
    {
        Vector3 {
            x: num::one::<P>() * x_val,
            y: num::zero::<P>(),
            z: num::zero::<P>(),
            w: num::zero::<P>(),
        }
    }

    fn y_coord(y_val: P) -> Self
    where
        P: Copy + Num,
    {
        Vector3 {
            x: num::zero::<P>(),
            y: num::one::<P>() * y_val,
            z: num::zero::<P>(),
            w: num::zero::<P>(),
        }
    }

    fn z_coord(z_val: P) -> Self
    where
        P: Copy + Num,
    {
        Vector3 {
            x: num::zero::<P>(),
            y: num::zero::<P>(),
            z: num::one::<P>() * z_val,
            w: num::zero::<P>(),
        }
    }
}

/// Initialization capabilities implementation for Point3
impl<P> Tuple<P> for Point3<P> {
    fn new(x: P, y: P, z: P) -> Self
    where
        P: Copy + Num,
    {
        Point3 {
            x,
            y,
            z,
            w: num::one(),
        }
    }

    fn all(all: P) -> Self
    where
        P: Copy + Num,
    {
        Point3 {
            x: num::one::<P>() * all,
            y: num::one::<P>() * all,
            z: num::one::<P>() * all,
            w: num::one::<P>(),
        }
    }

    // returns a tuple with [1 * x_val,0,0]
    fn x_coord(x_val: P) -> Self
    where
        P: Copy + Num,
    {
        Point3 {
            x: num::one::<P>() * x_val,
            y: num::zero::<P>(),
            z: num::zero::<P>(),
            w: num::one::<P>(),
        }
    }

    fn y_coord(y_val: P) -> Self
    where
        P: Copy + Num,
    {
        Point3 {
            x: num::zero::<P>(),
            y: num::one::<P>() * y_val,
            z: num::zero::<P>(),
            w: num::one::<P>(),
        }
    }

    fn z_coord(z_val: P) -> Self
    where
        P: Copy + Num,
    {
        Point3 {
            x: num::zero::<P>(),
            y: num::zero::<P>(),
            z: num::one::<P>() * z_val,
            w: num::one::<P>(),
        }
    }
}

/// Vector3 specific capabilities implementation
impl<P> Vector<P> for Vector3<P> {
    fn dot(lhs: Vector3<P>, rhs: Vector3<P>) -> P
    where
        P: Copy + Add<Output = P> + Mul<Output = P>,
    {
        lhs.x * rhs.x + lhs.w * rhs.w
    }
}

// Display trait implementation
impl<P> Display for Vector3<P>
where
    P: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!(
            "v: [{:^8.2},{:^8.2},{:^8.2},{:^8.2}]",
            self.x, self.y, self.z, self.w
        );
        f.write_str(&s)
    }
}
impl<P> Display for Point3<P>
where
    P: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!(
            "v: [{:^8.2},{:^8.2},{:^8.2},{:^8.2}]",
            self.x, self.y, self.z, self.w
        );
        f.write_str(&s)
    }
}

// TODO:
impl PartialEq for Vector3<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.equal(*other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.equal(*other)
    }
}

impl Eq for Vector3<f64> {}

impl PartialEq for Point3<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.equal(*other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.equal(*other)
    }
}

impl Eq for Point3<f64> {}

/// Trait allows Types with coordinates (x, y, etc.) to be efficiently initialized with common shorthand.
pub trait CoordInit<T, U> {
    /// Return a type with shorthand, for example [0, 0, -1].
    fn back() -> T;
    /// Return a type with shorthand, for example  [0, -1, 0].
    fn down() -> T;
    /// Return true if a type is identical to another, else return false.
    fn equal(self, rhs: Self) -> bool;
    /// Return a type with shorthand, for example  [0, 0, 1].
    fn forward() -> T;
    /// Return a type with shorthand, for example  [-1, 0, 0].
    fn left() -> T;
    /// Return a type with user-defined Axis components.
    fn new(axis: Axis<U>) -> T;
    /// Return a type with shorthand, for example  [1, 1, 1].
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
            w: 0.0,
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
            w: 0.0,
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

impl CoordInit<Vector3<f64>, f64> for Vector3<f64> {
    fn back() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
            w: 0.0,
        }
    }
    fn down() -> Self {
        Vector3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
            w: 0.0,
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

    fn forward() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 0.0,
        }
    }

    fn left() -> Self {
        Vector3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    fn new(axis: Axis<f64>) -> Vector3<f64> {
        match axis {
            Axis::XY(x, y) => Vector3 {
                x,
                y,
                z: 0.0,
                w: 0.0,
            },
            Axis::XYZ(x, y, z) => Vector3 { x, y, z, w: 0.0 },
            Axis::XYZW(x, y, z, w) => Vector3 { x, y, z, w },
        }
    }

    fn one() -> Self {
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 0.0,
        }
    }

    fn right() -> Self {
        Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    fn up() -> Self {
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        }
    }

    fn zero() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

impl CoordInit<Point3<f64>, f64> for Point3<f64> {
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

    fn new(axis: Axis<f64>) -> Point3<f64> {
        match axis {
            Axis::XY(x, y) => Point3 {
                x,
                y,
                z: 0.0,
                w: 1.0,
            },
            Axis::XYZ(x, y, z) => Point3 { x, y, z, w: 1.0 },
            Axis::XYZW(x, y, z, w) => Point3 { x, y, z, w },
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
            w: self.w + rhs.w,
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
            w: self.w + rhs.w,
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
            w: 0.0,
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
            w: 0.0,
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
            w: 0.0,
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
            w: 0.0,
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
            w: -self.w,
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
            w: 0.0,
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
            w: self.w * rhs,
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
            w: 0.0,
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
            w: self.w / rhs,
        }
    }
}

impl AddAssign for Vector3<f64> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: 0.0,
        }
    }
}

impl SubAssign for Vector3<f64> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: 0.0,
        }
    }
}

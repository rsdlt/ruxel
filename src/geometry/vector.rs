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
use num::{cast::NumCast, Num};
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

// Bring Geometry module constants into scope.
use super::EPSILON;

/// Provides Unit tests for Vector and Point types.
#[cfg(test)]
mod tests;

/// Type representing a geometric 3D Vector in its 'homogeneous' form with x, y, z, w components,
/// and where 'w' stands for 'weight'
#[derive(Clone, Copy, Debug)]
pub struct Vector3<P> {
    /// Component on the X axis
    pub x: P,
    /// Component on the Y axis
    pub y: P,
    /// Component on the Z axis
    pub z: P,
    /// Component representing the 'weight'
    pub w: P,
}

/// Type representing a geometric 3D Point in its 'homogeneous' form with x, y, z components, and
/// where 'W' stands for 'weight'
#[derive(Clone, Copy, Debug)]
pub struct Point3<P> {
    /// Component on the X axis
    pub x: P,
    /// Component on the Y axis
    pub y: P,
    /// Component on the Z axis
    pub z: P,
    /// Component representing the 'weight'
    pub w: P,
}

/// Trait that provides Vector and Point common initialization capabilities.
pub trait Tuple<P>
where
    P: Copy + Num,
{
    /// Initialize a Vector or Point with all its axis with the same user-defined value.
    fn all(all: P) -> Self;

    /// Initialize a Vector or Point with each axis with a separate user-defined value.
    fn new(x: P, y: P, z: P) -> Self;

    /// Initialize a Vector or Point with the X coordinate axis with a user-defined value.
    fn x_coord(x_val: P) -> Self;

    /// Initialize a Vector or Point with the Y coordinate axis with a user-defined value.
    fn y_coord(y_val: P) -> Self;

    /// Initialize a Vector or Point with the Z coordinate axis with a user-defined value.
    fn z_coord(z_val: P) -> Self;
}

/// Trait that provides Vector capabilities.
pub trait Vector<P>: Tuple<P>
where
    P: Copy + Num,
{
    /// Initialize a Vector with all the coordinates with a value of '1'.
    fn one() -> Self;

    /// Initialize a Vector with all the coordinates with a value of '0'.
    fn zero() -> Self;

    /// Initialize a Vector with the Z coordinate with a value of '-1'.
    fn back() -> Self
    where
        P: Neg + Neg<Output = P>;

    /// Initialize a Vector with the Y coordinate with a value of '-1'.
    fn down() -> Self
    where
        P: Neg + Neg<Output = P>;

    /// Initialize a Vector with the Z coordinate with a value of '1'.
    fn forward() -> Self;

    /// Initialize a Vector with the X coordinate with a value of '-1'.
    fn left() -> Self
    where
        P: Neg + Neg<Output = P>;

    /// Initialize a Vector with the X coordinate with a value of '1'.
    fn right() -> Self;

    /// Initialize a Vector with the Y coordinate with a value of '1'.
    fn up() -> Self;

    /// Normalize a Vector by dividing it by its Magnitude.
    fn normalized(&mut self) -> Self
    where
        P: NumCast;

    /// Return the information of the smallest coordinate value.
    fn min_component(&self) -> (i8, char, P)
    where
        P: PartialOrd;

    /// Return the information of the largest coordinate value.
    fn max_component(&self) -> (i8, char, P)
    where
        P: PartialOrd;

    /// Calculate the magnitude of a Vector.
    fn magnitude(&self) -> P
    where
        P: NumCast;

    /// Calculate the Cross product between two Vectors.
    fn cross(lhs: Vector3<P>, rhs: Vector3<P>) -> Vector3<P>;

    /// Calculate the Dot product between two Vectors.
    fn dot(lhs: Vector3<P>, rhs: Vector3<P>) -> P;
}

/// Trait that provides Point capabilities.
pub trait Point<P>: Tuple<P>
where
    P: Copy + Num,
{
    /// Set a Point with all its coordinates with a value of '0'.
    fn origin(&mut self) -> Self;
}

// Implementation of the Tuple Supertrait for Vector.
impl<P> Tuple<P> for Vector3<P>
where
    P: Copy + Num,
{
    fn all(all: P) -> Self {
        Vector3 {
            x: num::one::<P>() * all,
            y: num::one::<P>() * all,
            z: num::one::<P>() * all,
            w: num::zero::<P>(),
        }
    }

    fn new(x: P, y: P, z: P) -> Vector3<P> {
        Vector3 {
            x,
            y,
            z,
            w: num::zero(),
        }
    }

    fn x_coord(x_val: P) -> Self {
        Vector3 {
            x: num::one::<P>() * x_val,
            y: num::zero::<P>(),
            z: num::zero::<P>(),
            w: num::zero::<P>(),
        }
    }

    fn y_coord(y_val: P) -> Self {
        Vector3 {
            x: num::zero::<P>(),
            y: num::one::<P>() * y_val,
            z: num::zero::<P>(),
            w: num::zero::<P>(),
        }
    }

    fn z_coord(z_val: P) -> Self {
        Vector3 {
            x: num::zero::<P>(),
            y: num::zero::<P>(),
            z: num::one::<P>() * z_val,
            w: num::zero::<P>(),
        }
    }
}

// Implementation of the Tuple Supertrait for Point.
impl<P> Tuple<P> for Point3<P>
where
    P: Copy + Num,
{
    fn new(x: P, y: P, z: P) -> Self {
        Point3 {
            x,
            y,
            z,
            w: num::one(),
        }
    }

    fn all(all: P) -> Self {
        Point3 {
            x: num::one::<P>() * all,
            y: num::one::<P>() * all,
            z: num::one::<P>() * all,
            w: num::one::<P>(),
        }
    }

    // returns a tuple with [1 * x_val,0,0]
    fn x_coord(x_val: P) -> Self {
        Point3 {
            x: num::one::<P>() * x_val,
            y: num::zero::<P>(),
            z: num::zero::<P>(),
            w: num::one::<P>(),
        }
    }

    fn y_coord(y_val: P) -> Self {
        Point3 {
            x: num::zero::<P>(),
            y: num::one::<P>() * y_val,
            z: num::zero::<P>(),
            w: num::one::<P>(),
        }
    }

    fn z_coord(z_val: P) -> Self {
        Point3 {
            x: num::zero::<P>(),
            y: num::zero::<P>(),
            z: num::one::<P>() * z_val,
            w: num::one::<P>(),
        }
    }
}

// Implementation of the Point subtrait capabilities.
impl<P> Point<P> for Point3<P>
where
    P: Copy + Num,
{
    fn origin(&mut self) -> Self {
        Point3 {
            x: num::zero(),
            y: num::zero(),
            z: num::zero(),
            w: num::one(),
        }
    }
}

// Implemenation of the Vector subtrait capabilitites.
impl<P> Vector<P> for Vector3<P>
where
    P: Copy + Num,
{
    fn one() -> Self {
        Vector3 {
            x: num::one(),
            y: num::one(),
            z: num::one(),
            w: num::zero(),
        }
    }

    fn zero() -> Self {
        Vector3 {
            x: num::zero(),
            y: num::zero(),
            z: num::zero(),
            w: num::zero(),
        }
    }

    fn back() -> Self
    where
        P: Neg + Neg<Output = P>,
    {
        Vector3 {
            x: num::zero(),
            y: num::zero(),
            z: -num::one::<P>(),
            w: num::zero(),
        }
    }

    fn down() -> Self
    where
        P: Neg + Neg<Output = P>,
    {
        Vector3 {
            x: num::zero(),
            y: -num::one::<P>(),
            z: num::zero(),
            w: num::zero(),
        }
    }

    fn forward() -> Self {
        Vector3 {
            x: num::zero(),
            y: num::zero(),
            z: num::one(),
            w: num::zero(),
        }
    }

    fn left() -> Self
    where
        P: Neg + Neg<Output = P>,
    {
        Vector3 {
            x: -num::one::<P>(),
            y: num::zero(),
            z: num::zero(),
            w: num::zero(),
        }
    }

    fn right() -> Self {
        Vector3 {
            x: num::one(),
            y: num::zero(),
            z: num::zero(),
            w: num::zero(),
        }
    }

    fn up() -> Self {
        Vector3 {
            x: num::zero(),
            y: num::one(),
            z: num::zero(),
            w: num::zero(),
        }
    }

    fn normalized(&mut self) -> Self
    where
        P: NumCast,
    {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    fn min_component(&self) -> (i8, char, P)
    where
        P: PartialOrd,
    {
        if self.x <= self.y && self.x <= self.z {
            (0, 'x', self.x)
        } else if self.y <= self.z {
            (1, 'y', self.y)
        } else {
            (2, 'z', self.z)
        }
    }

    fn max_component(&self) -> (i8, char, P)
    where
        P: PartialOrd,
    {
        if self.x >= self.y && self.x >= self.z {
            (0, 'x', self.x)
        } else if self.y >= self.z {
            (1, 'y', self.y)
        } else {
            (2, 'z', self.z)
        }
    }

    fn magnitude(&self) -> P
    where
        P: NumCast,
    {
        P::from(
            (self.x * self.x + self.y * self.y + self.z * self.z)
                .to_f64()
                .unwrap()
                .sqrt(),
        )
        .unwrap()
    }

    fn cross(lhs: Vector3<P>, rhs: Vector3<P>) -> Vector3<P> {
        Vector3 {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
            w: num::zero(),
        }
    }

    fn dot(lhs: Vector3<P>, rhs: Vector3<P>) -> P {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }
}

// Implementation of the Partial Equivalence trait for Vector.
impl<P> PartialEq for Vector3<P>
where
    P: Num + NumCast,
{
    fn eq(&self, other: &Self) -> bool {
        if (self.x.to_f64().unwrap() - other.x.to_f64().unwrap()).abs() < EPSILON
            && (self.y.to_f64().unwrap() - other.y.to_f64().unwrap()).abs() < EPSILON
            && (self.z.to_f64().unwrap() - other.z.to_f64().unwrap()).abs() < EPSILON
            && (self.w.to_f64().unwrap() - other.w.to_f64().unwrap()).abs() < EPSILON
        {
            true
        } else {
            false
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// Implementation of the Partial Equivalence trait for Point.
impl<P> PartialEq for Point3<P>
where
    P: Num + NumCast,
{
    fn eq(&self, other: &Self) -> bool {
        if (self.x.to_f64().unwrap() - other.x.to_f64().unwrap()).abs() < EPSILON
            && (self.y.to_f64().unwrap() - other.y.to_f64().unwrap()).abs() < EPSILON
            && (self.z.to_f64().unwrap() - other.z.to_f64().unwrap()).abs() < EPSILON
            && (self.w.to_f64().unwrap() - other.w.to_f64().unwrap()).abs() < EPSILON
        {
            true
        } else {
            false
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// Implementation of the Display trait for Vector.
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

// Implementation of the Display trait for Point.
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

// Implementation of the Default trait for Point.
impl<P> Default for Vector3<P>
where
    P: Num,
{
    fn default() -> Self {
        Vector3 {
            x: num::zero(),
            y: num::zero(),
            z: num::zero(),
            w: num::zero(),
        }
    }
}

// Implementation of the Default trait for Point.
impl<P> Default for Point3<P>
where
    P: Num,
{
    fn default() -> Self {
        Point3 {
            x: num::zero(),
            y: num::zero(),
            z: num::zero(),
            w: num::one(),
        }
    }
}

// ---- Operator Overloading Implementations for Vector and Point.

// Vector + Vector
impl<P> Add for Vector3<P>
where
    P: Num,
{
    type Output = Vector3<P>;

    fn add(self, rhs: Vector3<P>) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

// Vector + Point
impl<P> Add<Point3<P>> for Vector3<P>
where
    P: Num,
{
    type Output = Point3<P>;

    fn add(self, rhs: Point3<P>) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

/// Point + Vector
impl<P> Add<Vector3<P>> for Point3<P>
where
    P: Num,
{
    type Output = Point3<P>;

    fn add(self, rhs: Vector3<P>) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

/// Vector - Vector
impl<P> Sub<Vector3<P>> for Vector3<P>
where
    P: Num,
{
    type Output = Vector3<P>;

    fn sub(self, rhs: Vector3<P>) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

/// Point - Point
impl<P> Sub<Point3<P>> for Point3<P>
where
    P: Num,
{
    type Output = Vector3<P>;

    fn sub(self, rhs: Point3<P>) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

/// Point - Vector
impl<P> Sub<Vector3<P>> for Point3<P>
where
    P: Num,
{
    type Output = Point3<P>;

    fn sub(self, rhs: Vector3<P>) -> Self::Output {
        Point3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

/// -Vector
impl<P> Neg for Vector3<P>
where
    P: Num + Neg + Neg<Output = P>,
{
    type Output = Vector3<P>;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

/// -Point
impl<P> Neg for Point3<P>
where
    P: Num + Neg + Neg<Output = P>,
{
    type Output = Point3<P>;

    fn neg(self) -> Self::Output {
        Point3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

/// Vector * Scalar
impl<P> Mul<P> for Vector3<P>
where
    P: Copy + Num,
{
    type Output = Vector3<P>;

    fn mul(self, rhs: P) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

/// Point * Scalar
impl<P> Mul<P> for Point3<P>
where
    P: Copy + Num,
{
    type Output = Point3<P>;

    fn mul(self, rhs: P) -> Self::Output {
        Point3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

/// Vector / Scalar
impl<P> Div<P> for Vector3<P>
where
    P: Copy + Num,
{
    type Output = Vector3<P>;

    fn div(self, rhs: P) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

/// Point / Scalar
impl<P> Div<P> for Point3<P>
where
    P: Copy + Num,
{
    type Output = Point3<P>;

    fn div(self, rhs: P) -> Self::Output {
        Point3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

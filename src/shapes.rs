// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/**
The Shapes module implements the functionality for Core shapes like Circle, Cylinder, Cube, and for External shapes from import of *.OBJ files or other formats
*/
use num::{Num, NumCast};

use crate::geometry::{ray::Ray, vector::Point3};

use crate::geometry::intersection::{Intersection, Intxn, IntxnVec};

/// Provides the data structure and implementation of the Core shapes
pub mod sphere;

/// Provides the data structure and implementation to import External shapes
pub mod external;

/// Trait representing a Shape.
pub trait Shape<P>
where
    P: Num + Copy,
{
    /// Returns the 'id' of a Shape.
    fn get_id(&self) -> i32;

    /// Returns the 'name' of a Shape.
    fn get_name<'a>(&'a self) -> &'a str;

    /// Returns the origin coordinates (Point3) of a Shape.
    fn get_origin(&self) -> Point3<P>;

    /// Creates and returns a new shape.
    fn new(id: i32) -> Self;

    /// Returns a collection of 't' values ('xs') where the Ray intersects a Shape.
    fn intersect<S>(shape: S, ray: Ray<P>) -> IntxnVec<P, S>
    where
        S: Shape<P> + Copy;
}

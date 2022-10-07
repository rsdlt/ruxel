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

// Bring geometry module constants into scope
use super::{matrix::*, ray::*, vector::*, EPSILON};

/**
 Data structures and methods for Intersections computations.
*/

// Unit tests for Intersections.
#[cfg(test)]
mod tests;

/// Macro that takes 'n' intersections and returns a collection 'Vec<Intxn>'.
#[macro_export]
macro_rules! intersections {
    ( $( $ix: expr ),+ ,) => {
       vec![ $( $ix ),* ]
    };

    ( $( $ix: expr ),+) => {
       vec![ $( $ix ),* ]
    };
}

pub type IntxnVec<P, S> = Vec<Intxn<P, S>>;

/// Type representing an intersection between a Ray and a Shapes.
#[derive(Clone, Copy, Debug)]
pub struct Intxn<P, S> {
    /// Distance 't' value of an intersection accross a Ray path.
    pub t: P,
    /// Object or Shape being intersected by the Ray.
    pub object: S,
}

impl<P, S> Display for Intxn<P, S>
where
    P: Display,
    S: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("Intxn -> t: {}\tobject:{}", self.t, self.object);
        write!(f, "{}", s)
    }
}

/// Common set of operations for Intersections
pub trait Intersection<P, S>
where
    P: Num + Copy,
    S: Shape<P>,
{
    /// Returns an intersection with a 't' distance between a Ray and a Shape
    fn intersection(t: P, object: S) -> Intxn<P, S>;
}

impl<P, S> Intersection<P, S> for Intxn<P, S>
where
    P: Num + Copy,
    S: Shape<P>,
{
    fn intersection(t: P, object: S) -> Self {
        Self { t, object }
    }
}

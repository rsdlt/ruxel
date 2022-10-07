// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::shapes::{sphere::*, Shape};
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
    [ $( $ix: expr ),+ ,] => {
       vec![ $( $ix ),* ]
    };

    [ $( $ix: expr ),+] => {
       vec![ $( $ix ),* ]
    };
}

/// Type representing a collection of intersections.
pub type IntxnVec<P, S> = Vec<Intxn<P, S>>;

/// Type representing an intersection between a Ray and a Shapes.
#[derive(Copy, Clone, Debug)]
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

// Comparison based on 't'.
impl<P, S> PartialEq for Intxn<P, S>
where
    P: Num + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

/// Finds and returns the 'hit' -visible intersection- in a collection.
pub fn hit<P, S>(xs: IntxnVec<P, S>) -> Option<Intxn<P, S>>
where
    P: Num + NumCast + Copy + PartialEq + PartialOrd + Display,
    S: Shape<P> + Copy,
{
    let mut min = xs[0].t;
    let mut id = 0;
    let mut flag = false;
    for (idx, ixn) in xs.iter().enumerate() {
        if ixn.t >= num::zero() {
            if min > ixn.t {
                min = ixn.t;
                id = idx;
            }
            flag = true;
        }
    }
    if !flag {
        return None;
    }

    return Some(xs[id]);
}

/// Common set of operations for Intersections
pub trait Intersection<P, S>
where
    P: Num + Copy + Display,
    S: Shape<P> + Copy,
{
    /// Returns an intersection with a 't' distance between a Ray and a Shape
    fn intersection(t: P, object: S) -> Intxn<P, S>;
}

impl<P, S> Intersection<P, S> for Intxn<P, S>
where
    P: Num + Copy + Display,
    S: Shape<P> + Copy,
{
    fn intersection(t: P, object: S) -> Self {
        Self { t, object }
    }
}

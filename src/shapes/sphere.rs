// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::*;

use crate::geometry::intersection::{Intersection, Intxn};
use crate::intersections;

use num::{
    integer::{sqrt, Roots},
    Num, NumCast, ToPrimitive,
};
use std::ops::Neg;

/**
 Data structures representing the core hapes Sphere
*/
// Bring Vector3, Point3 and Ray types into scope
use crate::geometry::{ray::Ray, vector::*};

// Unit tests for Sphere
#[cfg(test)]
mod tests;

/// Representation of a 3D sphere
#[derive(Clone, Copy, Debug)]
pub struct Sphere<'a, P> {
    name: &'a str,
    id: i32,
    origin: Point3<P>,
    radius: P,
}

impl<'a, P> Shape<P> for Sphere<'a, P>
where
    P: Num + NumCast + Copy + PartialEq + PartialOrd + Neg + Neg<Output = P>,
{
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn get_origin(&self) -> Point3<P> {
        self.origin
    }

    fn new(id: i32) -> Sphere<'a, P> {
        Sphere {
            name: "sphere",
            id,
            origin: Point3::zero(),
            radius: num::one(),
        }
    }

    fn intersect<S>(shape: S, ray: Ray<P>) -> IntxnVec<P, S>
    where
        S: Shape<P> + Copy,
    {
        // let mut xs: Vec<P> = vec![];
        let sphere_to_ray = ray.origin - shape.get_origin();
        let a = Vector3::dot(ray.direction, ray.direction);
        let b = P::from(2).unwrap() * Vector3::dot(ray.direction, sphere_to_ray);
        let c = Vector3::dot(sphere_to_ray, sphere_to_ray) - num::one();

        let discriminant = b * b - (P::from(4).unwrap() * a * c);

        if discriminant < num::zero() {
            return vec![];
        } else {
            let t1 = (-b - P::from(discriminant.to_f64().unwrap().sqrt()).unwrap())
                / (P::from(2).unwrap() * a);
            let t2 = (-b + P::from(discriminant.to_f64().unwrap().sqrt()).unwrap())
                / (P::from(2).unwrap() * a);
            // xs.push(t1);
            // xs.push(t2);
            // return xs;
            let i1 = Intxn::intersection(t1, shape);
            let i2 = Intxn::intersection(t2, shape);
            let xs = intersections![i1, i2];
            return xs;
        }
    }
}

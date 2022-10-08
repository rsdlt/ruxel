// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::*;

use crate::geometry::intersection::{Intersection, Intxn};
use crate::geometry::ray::*;
use crate::intersections;
use std::fmt::Display;

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
    /// id of the Sphere.
    pub id: i32,
    /// Name of the Spher.
    pub name: &'a str,
    /// Origin or 'center' of the Sphere.
    pub origin: Point3<P>,
    /// Transformation matrix of the Sphere.
    pub transform: Matrix4<P>,
}

impl<'a, P> Shape<P> for Sphere<'a, P>
where
    P: Num + NumCast + Copy + PartialEq + PartialOrd + Neg + Neg<Output = P> + Display,
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

    fn get_transform(&self) -> Matrix4<P> {
        self.transform
    }

    fn intersect<S>(shape: S, ray: Ray<P>) -> IntxnVec<P, S>
    where
        S: Shape<P> + Copy,
        P: Display,
    {
        let ray = Ray::transform(
            ray.ray_to_f64(),
            shape.get_transform().mat_to_f64().inverse(),
        );

        let sphere_to_ray = ray.origin - Point3::zero();
        let a = Vector3::dot(ray.direction, ray.direction);
        let b = 2.0 * Vector3::dot(ray.direction, sphere_to_ray);
        let c = Vector3::dot(sphere_to_ray, sphere_to_ray) - 1.0;

        let discriminant: f64 = b * b - (4.0 * a * c);

        if discriminant < num::zero() {
            return vec![];
        } else {
            let t1 = P::from((-b - discriminant.sqrt()) / (2.0 * a)).unwrap();
            let t2 = P::from((-b + discriminant.sqrt()) / (2.0 * a)).unwrap();

            let i1 = Intxn::intersection(t1, shape);
            let i2 = Intxn::intersection(t2, shape);
            let xs = intersections![i1, i2];
            return xs;
        }
    }

    fn new(id: i32) -> Sphere<'a, P> {
        Sphere {
            name: "sphere",
            id,
            origin: Point3::zero(),
            transform: Matrix4::identity(),
        }
    }

    fn set_transform(&mut self, mat: Matrix4<P>) {
        self.transform = mat;
    }
}

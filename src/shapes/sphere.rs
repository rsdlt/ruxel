// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::*;
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

    fn intersect(shape: impl Shape<P>, ray: Ray<P>) -> Vec<P> {
        let mut xs: Vec<P> = vec![];
        let sphere_to_ray = ray.origin - shape.get_origin();
        let a = Vector3::dot(ray.direction, ray.direction);
        let b = P::from(2).unwrap() * Vector3::dot(ray.direction, sphere_to_ray);
        let c = Vector3::dot(sphere_to_ray, sphere_to_ray) - num::one();

        let discriminant = b * b - (P::from(4).unwrap() * a * c);

        if discriminant < num::zero() {
            return xs;
        } else {
            let t1 = (-b - P::from(discriminant.to_f64().unwrap().sqrt()).unwrap())
                / (P::from(2).unwrap() * a);
            let t2 = (-b + P::from(discriminant.to_f64().unwrap().sqrt()).unwrap())
                / (P::from(2).unwrap() * a);
            xs.push(t1);
            xs.push(t2);
            return xs;
        }
    }
}

// impl Sphere<f64> {
//     /// Returns a sphere at the user-defined origin and radius
//     pub fn new(center: Point3<f64>, radius: f64) -> Self {
//         Self { center, radius }
//     }
//
//     /// Returns a sphere at the origin (0,0,0) with a radius of 1.
//     pub fn new_unit() -> Self {
//         Self {
//             center: Point3::new(0.0, 0.0, 0.0),
//             radius: 1.0,
//         }
//     }
//
//     /// Returns the Ray's 't' values if an intersection ocurred
//     /// If no intersection, function returns 'None'
//     /// If tangent intersection, function returns Some(t1, t2) where t1 = t2
//     /// If intersection, function returns Some(t1, t2)
//     pub fn intersect(self, ray: Ray<f64>) -> Option<(f64, f64)> {
//         let sphere_to_ray = ray.origin - self.center;
//         let a = Vector3::dot(ray.direction, ray.direction);
//         let b = 2.0 * Vector3::dot(ray.direction, sphere_to_ray);
//         let c = Vector3::dot(sphere_to_ray, sphere_to_ray) - 1.0;
//
//         let discriminant = b * b - 4.0 * a * c;
//
//         if discriminant < 0.0 {
//             None
//         } else {
//             let t1 = (-b - discriminant.sqrt()) / 2.0 * a;
//             let t2 = (-b + discriminant.sqrt()) / 2.0 * a;
//             Some((t1, t2))
//         }
//     }
// }

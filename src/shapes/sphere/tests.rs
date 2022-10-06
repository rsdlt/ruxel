// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Unit tests for Sphere types.

use super::*;
use crate::geometry::ray::*;

#[test]
// Intersects at two points
fn ut_sphere_ray_intersect_2p() {
    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
    let s = Sphere::new(1);
    let xs = Sphere::intersect(s, r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 4.0);
    assert_eq!(xs[1], 6.0);
}

#[test]
// Intersects at a tangent
fn ut_sphere_ray_intersect_tangent() {
    let r = Ray::new(Point3::new(0.0, 1.0, -5.0), Vector3::z_coord(1.0));
    let s = Sphere::new(2);
    let xs = Sphere::intersect(s, r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], 5.0);
    assert_eq!(xs[1], 5.0);
}

#[test]
// Ray misses a sphere.
fn ut_ray_misses_sphere() {
    let r = Ray::new(Point3::new(0.0, 2.0, -5.0), Vector3::z_coord(1.0));
    let s = Sphere::new(3);
    let xs = Sphere::intersect(s, r);
    assert_eq!(xs.len(), 0);
}

#[test]
// Ray originates inside sphere.
fn ut_ray_inside_sphere() {
    let r = Ray::new(Point3::zero(), Vector3::z_coord(1.0));
    let s = Sphere::new(4);
    let xs = Sphere::intersect(s, r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], -1.0);
    assert_eq!(xs[1], 1.0);
}

#[test]
// Ray originates behind sphere.
fn ut_ray_behind_sphere() {
    let r = Ray::new(Point3::z_coord(5.0), Vector3::z_coord(1.0));
    let s = Sphere::new(5);
    let xs = Sphere::intersect(s, r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0], -6.0);
    assert_eq!(xs[1], -4.0);
}

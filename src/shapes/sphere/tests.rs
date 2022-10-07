// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Unit tests for Sphere types.

use super::*;
use crate::geometry::{
    matrix::{Matrix4, Matrix4Ops},
    ray::*,
};

#[test]
// Intersects at two points
fn ut_sphere_ray_intersect_2p() {
    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
    let s = Sphere::new(1);
    let xs = Sphere::intersect(s, r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 6.0);
}

#[test]
// Intersects at a tangent
fn ut_sphere_ray_intersect_tangent() {
    let r = Ray::new(Point3::new(0.0, 1.0, -5.0), Vector3::z_coord(1.0));
    let s = Sphere::new(2);
    let xs = Sphere::intersect(s, r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 5.0);
    assert_eq!(xs[1].t, 5.0);
}

#[test]
// Ray misses a sphere.
fn ut_sphere_misses_ray() {
    let r = Ray::new(Point3::new(0.0, 2.0, -5.0), Vector3::z_coord(1.0));
    let s = Sphere::new(3);
    let xs = Sphere::intersect(s, r);
    assert_eq!(xs.len(), 0);
}

#[test]
// Ray originates inside sphere.
fn ut_sphere_ray_inside_sphere() {
    let r = Ray::new(Point3::zero(), Vector3::z_coord(1.0));
    let s = Sphere::new(4);
    let xs = Sphere::intersect(s, r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -1.0);
    assert_eq!(xs[1].t, 1.0);
}

#[test]
// Ray originates behind sphere.
fn ut_sphere_ray_behind_sphere() {
    let r = Ray::new(Point3::z_coord(5.0), Vector3::z_coord(1.0));
    let s = Sphere::new(5);
    let xs = Sphere::intersect(s, r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -6.0);
    assert_eq!(xs[1].t, -4.0);
}

#[test]
// Intersect sets object
fn ut_sphere_instersect_object() {
    let r = Ray::new(Point3::z_coord(-5.0), Vector3::z_coord(1.0));
    let s = Sphere::new(5);
    let xs = Sphere::intersect(s, r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].object.get_name(), s.get_name());
    assert_eq!(xs[1].object.get_id(), s.get_id());
}

#[test]
// Sphere default transformation.
fn ut_sphere_default_transform() {
    let s: Sphere<f64> = Sphere::new(1);
    assert_eq!(s.transform, Matrix4::identity());
}

#[test]
// Sphere change transformation.
fn ut_sphere_change_transform() {
    let mut s = Sphere::new(1);
    let t = Matrix4::identity().translate(2, 3, 4);
    s.set_transform(t);
    assert_eq!(s.transform, t);
}

#[test]
// Scaled sphere intersecting with a Ray
fn ut_sphere_intersect_scaled() {
    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
    let mut s = Sphere::new(1);
    s.set_transform(Matrix4::identity().scale(2.0, 2.0, 2.0));
    let xs = Sphere::intersect(s, r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 3.0);
    assert_eq!(xs[1].t, 7.0);
}

#[test]
// Scaled sphere intersecting with a Ray
fn ut_sphere_intersect_scaled_int() {
    let r = Ray::new(Point3::new(0, 0, -5), Vector3::new(0, 0, 1));
    let mut s = Sphere::new(1);
    s.set_transform(Matrix4::identity().scale(2, 2, 2));
    let xs = Sphere::intersect(s, r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 3);
    assert_eq!(xs[1].t, 7);
}

#[test]
// Translated sphere intersecting with a Ray
fn ut_sphere_intersect_translated() {
    let r = Ray::new(Point3::z_coord(-5), Vector3::forward());
    let mut s = Sphere::new(1);
    s.set_transform(Matrix4::identity().translate(5, 0, 0));
    let xs = Sphere::intersect(s, r);
    assert_eq!(xs.len(), 0);
}

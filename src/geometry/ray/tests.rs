// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::f64::consts::PI;
use std::path::Path;
use std::usize;

use crate::picture::canvas::{Canvas, Pixel};
use crate::picture::colors::{ColorInit, ColorRgb};

use crate::Axis::{XYZ as xyz, XYZW as xyzw};
use crate::shapes::core::Sphere;
use super::*;


/// Unit tests for Ray types.
#[test]
fn test_ray_initialization(){
    let ray = Ray::new(Point3::up(), Vector3::down());
    println!("{}", ray);
}

#[test]
fn test_ray_position(){
    let ray = Ray::new(
        Point3::new(xyz(2.0, 3.0, 4.0)),
        Vector3::right(),
        );
    assert_eq!(Ray::position(ray, 0.0), Point3::new(xyz(2.0, 3.0, 4.0)));
    assert_eq!(Ray::position(ray, 1.0), Point3::new(xyz(3.0, 3.0, 4.0)));
    assert_eq!(Ray::position(ray, -1.0), Point3::new(xyz(1.0, 3.0, 4.0)));
    assert_eq!(Ray::position(ray, 2.5), Point3::new(xyz(4.5, 3.0, 4.0)));
}

#[test]
fn test_ray_intersect_sphere(){
    // Intersects at two points
    let r = Ray::new(Point3::new(xyz(0.0, 0.0, -5.0)), Vector3::new(xyz(0.0, 0.0, 1.0)));
    let s = Sphere::new_unit();
    let xs = s.intersect(r);
    assert_eq!(xs, Some((4.0, 6.0)));

    // Intersects at a tangent
    let r = Ray::new(Point3::new(xyz(0.0, 1.0, -5.0)), Vector3::new(xyz(0.0, 0.0, 1.0)));
    let s = Sphere::new_unit();
    let xs = s.intersect(r);
    assert_eq!(xs, Some((5.0, 5.0)));
    
    // Ray misses a sphere 
    let r = Ray::new(Point3::new(xyz(0.0, 2.0, -5.0)), Vector3::new(xyz(0.0, 0.0, 1.0)));
    let s = Sphere::new_unit();
    let xs = s.intersect(r);
    assert_eq!(xs, None);

    // Ray originates inside a sphere
    let r = Ray::new(Point3::zero(), Vector3::new(xyz(0.0, 0.0, 1.0)));
    let s = Sphere::new_unit();
    let xs = s.intersect(r);
    assert_eq!(xs, Some((-1.0, 1.0)));

    // Sphere behind a ray
    let r = Ray::new(Point3::new(xyz(0.0, 0.0, 5.0)), Vector3::new(xyz(0.0, 0.0, 1.0)));
    let s = Sphere::new_unit();
    let xs = s.intersect(r);
    assert_eq!(xs, Some((-6.0, -4.0)));
}

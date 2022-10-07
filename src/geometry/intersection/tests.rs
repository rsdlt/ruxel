// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Unit tests for Intersection types.

use super::*;
use crate::geometry::ray::*;
use crate::intersections;
use crate::shapes::sphere::*;

#[test]
// Encapsulation of 't' and object
fn ut_intersection_encapsulation() {
    let s = Sphere::new(1);
    let i = Intxn::intersection(3.5, s);
    assert_eq!(i.t, 3.5);
    assert_eq!(i.object.get_name(), "sphere");
}

#[test]
// Aggregating intersections.
fn ut_intersection_aggregating() {
    let s = Sphere::new(1);
    let i1 = Intxn::intersection(1, s);
    let i2 = Intxn::intersection(2, s);
    let xs = intersections![i1, i2];
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 1);
    assert_eq!(xs[1].t, 2);
}

// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/**
Integration and Unit Testing
*/
use ruxel::geometry::vector::*;

#[test]
// This test validates the construction  of the Vector3 and Point3 types
fn vector_and_point_construction_integrity() {
    let v_one = Vector3::one();
    let v_zero = Vector3::zero();
    let v_back = Vector3::back();
    let v_down = Vector3::down();
    let v_forward = Vector3::forward();
    let v_left = Vector3::left();
    let v_right = Vector3::right();
    let v_up = Vector3::up();
    assert_eq!(
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0
        },
        v_one
    );
    assert_eq!(
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        v_zero
    );
    assert_eq!(
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0
        },
        v_back
    );
    assert_eq!(
        Vector3 {
            x: 0.0,
            y: -1.0,
            z: 0.0
        },
        v_down
    );
    assert_eq!(
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0
        },
        v_forward
    );
    assert_eq!(
        Vector3 {
            x: -1.0,
            y: 0.0,
            z: 0.0
        },
        v_left
    );
    assert_eq!(
        Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0
        },
        v_right
    );
    assert_eq!(
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0
        },
        v_up
    );
    let p_one = Point3::one();
    let p_zero = Point3::zero();
    let p_back = Point3::back();
    let p_down = Point3::down();
    let p_forward = Point3::forward();
    let p_left = Point3::left();
    let p_right = Point3::right();
    let p_up = Point3::up();
    assert_eq!(
        Point3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0
        },
        p_one
    );
    assert_eq!(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0
        },
        p_zero
    );
    assert_eq!(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
            w: 1.0
        },
        p_back
    );
    assert_eq!(
        Point3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
            w: 1.0
        },
        p_down
    );
    assert_eq!(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 1.0
        },
        p_forward
    );
    assert_eq!(
        Point3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
            w: 1.0
        },
        p_left
    );
    assert_eq!(
        Point3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 1.0
        },
        p_right
    );
    assert_eq!(
        Point3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 1.0
        },
        p_up
    );
}

#[test]
// This test validates the operation overloading Add, Sub, Div and Mul for the Vector3 and Point3
// types TODO: write the tests
fn vector_and_point_operator_overloading_integrity() {}

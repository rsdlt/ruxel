// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::Axis::{XYZ as xyz, XYZW as xyzw};
/// Unit tests for Matrix4 types.
use super::*;

#[test]
fn test_proper_matrix_creation() {
    let m_one = Matrix4::one();
    println!("{}", m_one);
    let m_zero = Matrix4::zero();
    println!("{}", m_zero);
    let m_iden = Matrix4::identity();
    println!("{}", m_iden);

    let m1 = Matrix4::new(Some([
        [9.193, 8.122, 18.513, 21.351],
        [3.697, 15.399, 9.425, 14.905],
        [5.861, 22.291, 12.845, 10.048],
        [6.480, 3.595, 5.990, 14.386],
    ]));
    println!("{}", m1);

    let m2 = Matrix4::new(Some([
        [32232384774520380928328947.5819, 25.381, 17.8357, 71.0214],
        [90.4163, 37.8278, 49.0731, 89.71829839028379874837948029382],
        [75.0119, 11.617, 8.005, 63299249080294230.7788],
        [
            50.8061,
            31.783,
            25.5009,
            0000000000000000000000000000000000000000000000000000000.2281,
        ],
    ]));
    println!("{}", m2);

    let m3 = Matrix4::new(None);
    println!("{}", m3);
}

#[test]
fn test_matrix_equality() {
    let m1 = Matrix4::new(Some([
        [34.7777f64, 86.9876f64, 44.552f64, 51.213f64],
        [16.3014f64, 98.8141f64, 53.107f64, 33.649f64],
        [58.6944f64, 39.5563f64, 48.3533f64, 50.7128f64],
        [19.2097f64, 91.2242f64, 78.8212f64, 81.5701f64],
    ]));
    let m2 = Matrix4::new(Some([
        [34.7777f64, 86.9876f64, 44.552f64, 51.213f64],
        [16.3014f64, 98.8141f64, 53.107f64, 33.649f64],
        [58.6944f64, 39.5563f64, 48.3533f64, 50.7128f64],
        [19.2097f64, 91.2242f64, 78.8212f64, 81.5702f64],
    ]));
    assert!(m1 == m1);
    assert!(m1 != m2);
    assert_eq!(m1, m1);
    assert_ne!(m1, m2);
}

#[test]
fn test_matrix_multiplication() {
    let m1 = Matrix4::new(Some([
        [1f64, 2f64, 3f64, 4f64],
        [5f64, 6f64, 7f64, 8f64],
        [9f64, 8f64, 7f64, 6f64],
        [5f64, 4f64, 3f64, 2f64],
    ]));
    let m2 = Matrix4::new(Some([
        [-2f64, 1f64, 2f64, 3f64],
        [3f64, 2f64, 1f64, -1f64],
        [4f64, 3f64, 6f64, 5f64],
        [1f64, 2f64, 7f64, 8f64],
    ]));
    println!("m1:\n{}\nm2:\n{}\nres:\n{}", m1, m2, m1 * m2);
    assert_eq!(
        m1 * m2,
        Matrix4::new(Some([
            [20f64, 22f64, 50f64, 48f64],
            [44f64, 54f64, 114f64, 108f64],
            [40f64, 58f64, 110f64, 102f64],
            [16f64, 26f64, 46f64, 42f64],
        ]))
    );
    let m3 = Matrix4::new(Some([
        [1f64, 2f64, 3f64, 4f64],
        [2f64, 4f64, 4f64, 2f64],
        [8f64, 6f64, 4f64, 1f64],
        [0f64, 0f64, 0f64, 1f64],
    ]));
    let v3 = Vector3::new(xyzw(1.00, 2.00, 3.00, 1.00));
    println!("m3:\n{}\nv3:\n{}\nres:\n{}", m3, v3, m3 * v3);
    println!("m3:\n{}\nv3:\n{}\nres:\n{}", m3, v3, v3 * m3);
    assert!(m3 * v3 == Vector3::new(xyzw(18f64, 24f64, 33f64, 1f64)));

    // test identity matrix multiplication
    assert_eq!(m3, m3 * Matrix4::identity());
}

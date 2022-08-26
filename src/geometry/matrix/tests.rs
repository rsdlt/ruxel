// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
[32232384774520380928328947.5819,25.381,17.8357,71.0214],
[90.4163,37.8278,49.0731,89.71829839028379874837948029382],
[75.0119,11.617,8.005,63299249080294230.7788],
[50.8061,31.783,25.5009,0000000000000000000000000000000000000000000000000000000.2281],
    ]));
    println!("{}", m2);

    let m3 = Matrix4::new(None);
    println!("{}", m3);
}

#[test]
fn test_matrix_equality() {

}

#[test]
fn test_matrix_multiplication() {
    let m1 = Matrix4::new(Some([
[1f64,2f64,3f64,4f64],	
[5f64,6f64,7f64,8f64],	
[9f64,8f64,7f64,6f64],	
[5f64,4f64,3f64,2f64],	
    ]));

    let m2 = Matrix4::new(Some([
[-2f64,1f64,2f64,3f64],
[3f64,2f64,1f64,-1f64],
[4f64,3f64,6f64,5f64],
[1f64,2f64,7f64,8f64],
    ]));
   
    println!("{}", m1 * m2);
}


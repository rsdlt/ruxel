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
use crate::picture::colors::{ColorRgb, ColorInit};

use super::Axis::{XYZ as xyz, XYZW as xyzw};
/// Unit tests for Matrix4 types.
use super::*;

#[test]
fn test_matrix_creation() {
    println!("{}", Matrix4::ZERO);

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
    assert_eq!(
        Matrix4::identity() * Vector3::new(xyzw(1.0, 2.0, 3.0, 1.0)),
        Vector3::new(xyzw(1.0, 2.0, 3.0, 1.0))
    );
}

#[test]
fn test_matrix_transpose() {
    let mut m1 = Matrix4::new(Some([
        [0f64, 9f64, 3f64, 0f64],
        [9f64, 8f64, 0f64, 8f64],
        [1f64, 8f64, 5f64, 3f64],
        [0f64, 0f64, 5f64, 8f64],
    ]));

    println!("{}", m1.transpose());
}

#[test]
fn test_submatrix_minor_cofactor() {
    let mut mx2 = Matrix2::new();
    mx2.m[0][0] = 1f64;
    mx2.m[0][1] = 5f64;
    mx2.m[1][0] = -3f64;
    mx2.m[1][1] = 2f64;
    assert_eq!(17f64, mx2.determinant());

    let mut mx3 = Matrix3::new();
    mx3.m[0][0] = 1f64;
    mx3.m[0][1] = 5f64;
    mx3.m[0][2] = 0f64;
    mx3.m[1][0] = -3f64;
    mx3.m[1][1] = 2f64;
    mx3.m[1][2] = 7f64;
    mx3.m[2][0] = 0f64;
    mx3.m[2][1] = 6f64;
    mx3.m[2][2] = -3f64;

    println!("{:?}", mx3);
    let mx2 = Matrix3::submatrix(mx3, 2, 2);
    println!("+================");
    println!("{:?}", mx2);

    let mx4 = Matrix4::new(Some([
        [-6f64, 1f64, 1f64, 6f64],
        [-8f64, 5f64, 8f64, 6f64],
        [-1f64, 0f64, 8f64, 2f64],
        [-7f64, 1f64, -1f64, 1f64],
    ]));
    let mx3_new = mx4.submatrix(2, 1);
    println!("{:?}", mx3_new);

    let mut mx3_1 = Matrix3::new();
    mx3_1.m[0][0] = 3f64;
    mx3_1.m[0][1] = 5f64;
    mx3_1.m[0][2] = 0f64;
    mx3_1.m[1][0] = 2f64;
    mx3_1.m[1][1] = -1f64;
    mx3_1.m[1][2] = -7f64;
    mx3_1.m[2][0] = 6f64;
    mx3_1.m[2][1] = -1f64;
    mx3_1.m[2][2] = 5f64;

    println!("+================");
    println!("+================");
    println!("{:?}", mx3_1);
    assert_eq!(mx3_1.minor(1, 0), 25f64);

    println!("+================");
    println!("+================");
    println!("+================");
    assert_eq!(mx3_1.minor(0, 0), mx3_1.cofactor(0, 0));
    println!(
        "minor: {}\ncofactor: {}",
        mx3_1.minor(0, 0),
        mx3_1.cofactor(0, 0)
    );
    assert_ne!(mx3_1.minor(1, 0), mx3_1.cofactor(1, 0));
    println!(
        "minor: {}\ncofactor: {}",
        mx3_1.minor(1, 0),
        mx3_1.cofactor(1, 0)
    );
}

#[test]
fn test_matrix_determinant() {
    let mut mx3 = Matrix3::new();
    mx3.m[0][0] = 1f64;
    mx3.m[0][1] = 2f64;
    mx3.m[0][2] = 6f64;
    mx3.m[1][0] = -5f64;
    mx3.m[1][1] = 8f64;
    mx3.m[1][2] = -4f64;
    mx3.m[2][0] = 2f64;
    mx3.m[2][1] = 6f64;
    mx3.m[2][2] = 4f64;

    let mx4 = Matrix4::new(Some([
        [-2f64, -8f64, 3f64, 5f64],
        [-3f64, 1f64, 7f64, 3f64],
        [1f64, 2f64, -9f64, 6f64],
        [-6f64, 7f64, 7f64, -9f64],
    ]));

    assert_eq!(mx3.determinant(), -196f64);

    println!("+================");
    println!("{}", mx4.cofactor(0, 0));
    println!("{}", mx4.cofactor(0, 1));
    println!("{}", mx4.cofactor(0, 2));
    println!("{}", mx4.cofactor(0, 3));
    assert_eq!(mx4.determinant(), -4071f64)
}

#[test]
fn test_matrix_inversion() {
    let m1 = Matrix4::new(Some([
        [6f64, 4f64, 4f64, 4f64],
        [5f64, 5f64, 7f64, 6f64],
        [4f64, -9f64, 3f64, -7f64],
        [9f64, 1f64, 7f64, -6f64],
    ]));

    let m2 = Matrix4::new(Some([
        [-4f64, 2f64, -2f64, -3f64],
        [9f64, 6f64, 2f64, 6f64],
        [0f64, -5f64, 1f64, -5f64],
        [0f64, 0f64, 0f64, 0f64],
    ]));

    assert!(m1.determinant() == -2120f64);
    assert!(m2.determinant() == 0f64);

    let m3 = Matrix4::new(Some([
        [8f64, -5f64, 9f64, 2f64],
        [7f64, 5f64, 6f64, 1f64],
        [-6f64, 0f64, 9f64, 6f64],
        [-3f64, 0f64, -9f64, -4f64],
    ]));

    let m4 = Matrix4::new(Some([
        [9f64, 3f64, 0f64, 9f64],
        [-5f64, -2f64, -6f64, -3f64],
        [-4f64, 9f64, 6f64, 4f64],
        [-7f64, 6f64, 6f64, 2f64],
    ]));

    println!("inv(m3) =\n{}", m3.inverse());
    println!("inv(m4) =\n{}", m4.inverse());

    let ma = Matrix4::new(Some([
        [3f64, -9f64, 7f64, 3f64],
        [3f64, -8f64, 2f64, -9f64],
        [-4f64, 4f64, 4f64, 1f64],
        [-6f64, 5f64, 1f64, 1f64],
    ]));

    let mb = Matrix4::new(Some([
        [8f64, 2f64, 2f64, 2f64],
        [3f64, -1f64, 7f64, 0f64],
        [7f64, 0f64, 5f64, 4f64],
        [6f64, -2f64, 0f64, 5f64],
    ]));

    let mc = ma * mb;

    assert_eq!(mc * mb.inverse(), ma);
}

#[test]
// Test the different matrix transformations and chaining of transformations
fn test_matrix_transformations(){

    // Translations
    let mut m = Matrix4::identity(); 
    let p = Point3::new(xyz(-3.0, 4.0, 5.0));
    let pt = p * m.translate(5.0, -3.0, 2.0);
    assert_eq!(pt, Point3::new(xyz(2.0, 1.0, 7.0)));

    let p = Point3::new(xyz(-3.0, 4.0, 5.0));
    let pt = p *  Matrix4::identity().translate(5.0, -3.0, 2.0).inverse();
    assert_eq!(pt, Point3::new(xyz(-8.0, 7.0, 3.0)));

    let v = Vector3::new(xyz(-3.0, 4.0, 5.0));
    assert_eq!(v, v * Matrix4::identity().translate(5.0, -3.0, 2.0));
    

    // Scaling
    let mut  m = Matrix4::identity(); 
    let p = Point3::new(xyz(-4.0, 6.0, 8.0));
    let pt = p *  m.scale(2.0, 3.0, 4.0);
    assert_eq!(pt, Point3::new(xyz(-8.0, 18.0, 32.0)));

    let mut  m = Matrix4::identity(); 
    let v = Vector3::new(xyz(-4.0, 6.0, 8.0));
    let vt = v *  m.scale(2.0, 3.0, 4.0);
    assert_eq!(vt, Vector3::new(xyz(-8.0, 18.0, 32.0)));

    let mut  m = Matrix4::identity(); 
    let v = Vector3::new(xyz(-4.0, 6.0, 8.0));
    let vt = v *  m.scale(2.0, 3.0, 4.0).inverse();
    assert_eq!(vt, Vector3::new(xyz(-2.0, 2.0, 2.0)));

    // Test the reflection by scaling with a negative axis
    let mut m = Matrix4::identity(); 
    let p = Point3::new(xyz(2.0, 3.0, 4.0));
    let pt = p *  m.scale(-1.0, 1.0, 1.0);
    assert_eq!(pt, Point3::new(xyz(-2.0, 3.0, 4.0)));
    

    // Rotation

    // Rotate around x
    let p = Point3::up();
    let mut m = Matrix4::identity(); 
    let hq = p * m.rotate_x(PI/4.0);
    let mut m = Matrix4::identity(); 
    let fq = p * m.rotate_x(PI/2.0);
    assert_eq!(hq, Point3::new(xyz(0.0, 2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0 )));
    assert_eq!(fq, Point3::forward());

    // Rotate around y
    let p = Point3::forward();
    let mut m = Matrix4::identity(); 
    let hq = p * m.rotate_y(PI/4.0);
    let mut m = Matrix4::identity(); 
    let fq = p * m.rotate_y(PI/2.0);
    assert_eq!(hq, Point3::new(xyz(2f64.sqrt() / 2.0, 0.0, 2f64.sqrt() / 2.0 )));
    assert_eq!(fq, Point3::right());

    // Rotate around z
    let p = Point3::up();
    let mut  m = Matrix4::identity(); 
    let hq = p * m.rotate_z(PI/4.0);
    let mut m = Matrix4::identity(); 
    let fq = p * m.rotate_z(PI/2.0);
    assert_eq!(hq, Point3::new(xyz(-2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0, 0.0 )));
    assert_eq!(fq, Point3::left());

    // Shearing 
    let p = Point3::new(xyz(2.0, 3.0, 4.0));
    
    let mut m = Matrix4::identity(); 
    let pt = p * m.shear(1.0, 0.0,0.0,0.0,0.0,0.0);
    assert_eq!(pt, Point3::new(xyz(5.0, 3.0, 4.0)));

    let mut m = Matrix4::identity(); 
    let pt = p * m.shear(0.0, 0.0,1.0,0.0,0.0,0.0);
    assert_eq!(pt, Point3::new(xyz(2.0, 5.0, 4.0)));

    let mut m = Matrix4::identity(); 
    let pt = p * m.shear(0.0, 0.0,0.0,1.0,0.0,0.0);
    assert_eq!(pt, Point3::new(xyz(2.0, 7.0, 4.0)));
    
    let mut m = Matrix4::identity(); 
    let pt = p * m.shear(0.0, 0.0,0.0,0.0,1.0,0.0);
    assert_eq!(pt, Point3::new(xyz(2.0, 3.0, 6.0)));
    
    let mut m = Matrix4::identity(); 
    let pt = p * m.shear(0.0, 0.0,0.0,0.0,0.0,1.0);
    assert_eq!(pt, Point3::new(xyz(2.0, 3.0, 7.0)));


    // Chaining transformations
    let mut p = Point3::new(xyz(1.0, 0.0, 1.0)) ;
    let mut m = Matrix4::identity();

    p = p * m.rotate_x(PI / 2.0);
    println!("{}", p);

    let mut m = Matrix4::identity();
    p = p * m.scale(5.0, 5.0, 5.0);
    println!("{}", p);

    let mut m = Matrix4::identity();
    p = p * m.translate(10.0, 5.0, 7.0);
    println!("{}", p);

    let mut p1 = Point3::new(xyz(1.0, 0.0, 1.0)) ;
    let mut m = Matrix4::identity();
    p1 = p1 * m.rotate_x(PI / 2.0).scale(5.0, 5.0, 5.0).translate(10., 5.0, 7.0);
    println!("{}",p1);

    assert_eq!(p1, Point3::new(xyz(15.0, 0.0, 7.0)));
}

#[test]
fn matrix_clock_exercise() {
    let pix = Point3::new(xyz(0.0, 0.0, 1.0));
    let hr =  PI / 6.0;
    let mut m = Matrix4::identity();
    
    println!("{}", pix * 100.0);
    for _i in 1..12 {
        println!("{}", pix * m.rotate_y(1.0 * hr) * 100.0) ;
    }

    let mut can = Canvas::new(200, 200);
    let mut mc = Matrix4::identity();
    let mut pixel = Pixel::new(0, 0, ColorRgb::green());
    let pix = Point3::new(xyz(0.0, 0.0, 1.0));
    let image_path = Path::new("images/test_clock.ppm");
    
    let tpix = pix * mc.rotate_y(1.0  * hr).scale(75.0, 75.0, 75.0).translate(100.0, 0.0, 100.0);
    pixel.x = tpix.x as usize;
    pixel.y = tpix.z as usize;
    can.write_pixel(pixel);
    for i in 1..=12 {
        let tpix = pix * mc.rotate_y(i as f64  * hr).scale(75.0, 75.0, 75.0).translate(100.0, 0.0, 100.0);
        pixel.x = tpix.x as usize;
        pixel.y = tpix.z as usize;
        println!("px:{}, py:{}, tpx:{}, tpy{}", pixel.x, pixel.y, tpix.x, tpix.z);
        can.write_pixel(pixel);
        mc = Matrix4::identity(); 
    }
    can.write_to_ppm(&image_path);
    
}

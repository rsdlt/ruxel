// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

/// Provides Unit tests for Matrix4 types.
#[cfg(test)]
mod tests;

use std::cmp::{Eq, PartialEq};
use std::fmt::Display;
use std::ops::{Mul, MulAssign};

// Bring Vector module constants into scope
use super::vector::*;

// Bring Geometry module constants into scope
use super::EPSILON;

/// Declaration for inner matrix data of size 4x4 and
/// generic type
/// [[columns] rows]
/// [row][col]
pub type Matrix4Data<T> = [[T; 4]; 4];

/// Declararion for a matrix row of size 4 and
/// generic type
pub type Matrix4Row<T> = [T; 4];

/// Declararion for a matrix column of size 4 and
/// generic type
pub type Matrix4Col<T> = [T; 4];

/// Enum that allows a user to select a row or
/// column from a matrix
#[derive(Debug)]
pub enum Matrix4Index {
    /// .
    One,
    /// .
    Two,
    /// .
    Three,
    /// .
    Four,
}

#[derive(Debug, Clone, Copy)]
/// Definition of Matrix4
pub struct Matrix4<T> {
    m: Matrix4Data<T>,
}

#[derive(Debug, Clone, Copy)]
/// Definition of Matrix3
/// Only used in this module to calculate Matrix4 determinat and cofactor
pub(crate) struct Matrix3<T> {
    m: [[T; 3]; 3],
}

#[derive(Debug, Clone, Copy)]
/// Definition of Matrix2
/// Only used in this module to calculate Matrix4 determinat and cofactor
pub(crate) struct Matrix2<T> {
    m: [[T; 2]; 2],
}


impl Default for Matrix4<f64>{
    fn default() -> Self {
        Self { m: Default::default() }
    }
}

impl Display for Matrix4<f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = "".to_string();
        for row in self.m {
            s.push_str(
                &format!(
                    "[{:^8.5}, {:^8.5}, {:^8.5}, {:^8.5}]\n",
                    &row[0], &row[1], &row[2], &row[3]
                )
                .to_string(),
            );
        }
        f.write_str(&s)
    }
}

impl PartialEq for Matrix4<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.equal(other)
    }
}
impl Eq for Matrix4<f64> {}

impl Matrix4<f64> {
    const ZERO: Matrix4<f64> = Matrix4{
         // m: [[0.0; 4]; 4]
         m: 
            [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        
    };

    const ONE: Matrix4<f64> = Matrix4 {
         m: 
            [
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0],
            ],
        
    };

    const IDENTITY: Matrix4<f64> = Matrix4 {
         m: 
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        
    };

    pub(crate) fn submatrix(self, row_del: usize, col_del: usize) -> Matrix3<f64> {
        let mut res = Matrix3::new();
        let mut r_count = 0;
        let mut c_count = 0;

        for row in 0..4 {
            if row != row_del {
                for col in 0..4 {
                    if col != col_del {
                        res.m[r_count][c_count] = self.m[row][col];
                        c_count += 1;
                    }
                }
                c_count = 0;
                r_count += 1;
            }
        }
        res
    }

    pub(crate) fn minor(self, row_del: usize, col_del: usize) -> f64 {
        self.submatrix(row_del, col_del).determinant()
    }

    pub(crate) fn cofactor(self, row_del: usize, col_del: usize) -> f64 {
        if (row_del + col_del) % 2 == 0 {
            self.minor(row_del, col_del)
        } else {
            -self.minor(row_del, col_del)
        }
    }

    pub(crate) fn determinant(self) -> f64 {
        let mut det = 0f64;
        for col in 0..4 {
            det = det + self.m[0][col] * self.cofactor(0, col);
        }
        det
    }
}

/// Provides the capabilities to initialize and transform a Matrix
pub trait Matrix4Ops<T> {
    
    /// .
    fn equal(&self, other: &Self) -> bool;

    /// Returns the row of the matrix based on an user-defined index
    fn get_row(&self, index: Matrix4Index) -> Matrix4Row<T>;

    /// Returns the row of the matrix based on an user-defined index
    fn get_col(&self, index: Matrix4Index) -> Matrix4Col<T>;

    /// Returns the new matrix with the data provided by the user.
    /// If no data is provided the function returns the matrix zero
    fn new(data: Option<Matrix4Data<T>>) -> Self;

    /// Returns a new identity matrix
    fn identity() -> Self;

    /// Returns the inverse of a matrix
    fn inverse(self) -> Self;

    /// Returns a new matrix filled with '1'
    fn one() -> Self;

    /// Returns rotation matrix around the X axis
    fn rotate_x(&mut self, radians: T) -> Self;

    /// Returns rotation matrix around the Y axis
    fn rotate_y(&mut self, radians: T) -> Self;

    /// Returns rotation matrix around the Z axis
    fn rotate_z(&mut self, radians: T) -> Self;

    /// Returns the Scaling Matrix
    fn scale(&mut self, x: T, y: T, z: T) -> Self;

    /// Returns the Shearing Matrix
    fn shear(&mut self, xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Self; 
    
    /// Transposes a Matrix
    fn transpose(&mut self) -> Self;

    /// Returns a translation Matrix
    fn translate(&mut self, x: T, y: T, z: T)  -> Self ;

    /// Returns a new matrix filled with '0'
    fn zero() -> Self;
}

impl Matrix4Ops<f64> for Matrix4<f64> {
    fn equal(&self, other: &Self) -> bool {
        let mut flag = true;
        for i in 0..4 {
            if (self.m[i][0] - other.m[i][0]).abs() < EPSILON
                && (self.m[i][1] - other.m[i][1]).abs() < EPSILON
                && (self.m[i][2] - other.m[i][2]).abs() < EPSILON
                && (self.m[i][3] - other.m[i][3]).abs() < EPSILON
            {
                flag = true;
            } else {
                flag = false;
                break;
            }
        }
        flag
    }

    fn get_row(&self, index: Matrix4Index) -> Matrix4Row<f64> {
        let mut row: Matrix4Row<f64> = [0.0; 4];
        match index {
            Matrix4Index::One => {
                row[0] = self.m[0][0];
                row[1] = self.m[0][1];
                row[2] = self.m[0][2];
                row[3] = self.m[0][3];
            }
            Matrix4Index::Two => {
                row[1] = self.m[1][0];
                row[1] = self.m[1][1];
                row[2] = self.m[1][2];
                row[3] = self.m[1][3];
            }
            Matrix4Index::Three => {
                row[0] = self.m[2][0];
                row[1] = self.m[2][1];
                row[2] = self.m[2][2];
                row[3] = self.m[2][3];
            }
            Matrix4Index::Four => {
                row[0] = self.m[3][0];
                row[1] = self.m[3][1];
                row[2] = self.m[3][2];
                row[3] = self.m[3][3];
            }
        }
        row
    }

    fn get_col(&self, index: Matrix4Index) -> Matrix4Col<f64> {
        let mut col: Matrix4Col<f64> = [0.0; 4];
        match index {
            Matrix4Index::One => {
                col[0] = self.m[0][0];
                col[1] = self.m[1][0];
                col[2] = self.m[2][0];
                col[3] = self.m[3][0];
            }
            Matrix4Index::Two => {
                col[1] = self.m[1][0];
                col[1] = self.m[1][1];
                col[2] = self.m[1][2];
                col[3] = self.m[1][3];
            }
            Matrix4Index::Three => {
                col[0] = self.m[2][0];
                col[1] = self.m[2][1];
                col[2] = self.m[2][2];
                col[3] = self.m[2][3];
            }
            Matrix4Index::Four => {
                col[0] = self.m[3][0];
                col[1] = self.m[3][1];
                col[2] = self.m[3][2];
                col[3] = self.m[3][3];
            }
        }
        col
    }

    fn new(data: Option<Matrix4Data<f64>>) -> Self {
        match data {
            None => Matrix4Ops::zero(),
            Some(data) => Self { m: data },
        }
    }

    fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    fn inverse(self) -> Self {
        if self.determinant() == 0.0 {
            panic!("Matrix cannot be inversed");
        } else {
            let mut res = Matrix4::zero();
            for row in 0..4 {
                for col in 0..4 {
                    let c = self.cofactor(row, col);
                    // switches col for row to achieve transpose operation
                    res.m[col][row] = c / self.determinant();
                }
            }
            res
        }
    }

    fn one() -> Self {
        Self { m: [[1.0; 4]; 4] }
    }

    fn rotate_x(& mut self, radians: f64) -> Self {
        let mut res = Matrix4::IDENTITY; 
        res.m[1][1] = radians.cos();
        res.m[1][2] = -radians.sin();
        res.m[2][1] = radians.sin();
        res.m[2][2] = radians.cos();
        *self = res * *self;
        *self
    }

    fn rotate_y(&mut self, radians: f64) -> Self { 
        let mut res = Matrix4::IDENTITY; 
        res.m[0][0] = radians.cos();
        res.m[0][2] = radians.sin();
        res.m[2][0] = -radians.sin();
        res.m[2][2] = radians.cos();
        *self = res * *self;
        *self
    }

    fn rotate_z(& mut self, radians: f64) -> Self {
        let mut res = Matrix4::IDENTITY; 
        res.m[0][0] = radians.cos();
        res.m[0][1] = -radians.sin();
        res.m[1][0] = radians.sin();
        res.m[1][1] = radians.cos();
        *self = res * *self;
        *self
    }

    fn scale(&mut self, x: f64, y: f64, z: f64) -> Self {
        let mut res = Matrix4::IDENTITY; 
        res.m[0][0] = x;
        res.m[1][1] = y;
        res.m[2][2] = z;
        *self = res * *self ;
        *self
    }

    fn shear(&mut self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let mut res = Matrix4::IDENTITY; 
        res.m[0][1] = xy;
        res.m[0][2] = xz;
        res.m[1][0] = yx;
        res.m[1][2] = yz;
        res.m[2][0] = zx;
        res.m[2][1] = zy;
        *self = res * *self;
        *self
    }

    fn transpose(&mut self) -> Self {
        let mut res = Matrix4::ZERO;
        for row in 0..4 {
            res.m[0][row] = self.m[row][0];
            res.m[1][row] = self.m[row][1];
            res.m[2][row] = self.m[row][2];
            res.m[3][row] = self.m[row][3];
        }
        *self = res * *self;
        *self
    }

    fn translate(&mut self, x: f64, y: f64, z: f64) -> Self {
        let mut res = Matrix4::IDENTITY; 
        res.m[0][3] = x;
        res.m[1][3] = y;
        res.m[2][3] = z;
        *self = res * *self ;
        *self
    }

    fn zero() -> Self {
        Self { m: [[0.0; 4]; 4] }
    }
}

impl Mul for Matrix4<f64> {
    type Output = Matrix4<f64>;

    fn mul(self, rhs: Self) -> Self {
        let mut m_res = Matrix4::zero();
        for row in 0..4 {
            for col in 0..4 {
                m_res.m[row][col] = self.m[row][0] * rhs.m[0][col]
                    + self.m[row][1] * rhs.m[1][col]
                    + self.m[row][2] * rhs.m[2][col]
                    + self.m[row][3] * rhs.m[3][col]
            }
        }
        m_res
    }
}

impl MulAssign for Matrix4<f64> {
    fn mul_assign(&mut self, rhs: Self) {
        for row in 0..4 {
            for col in 0..4 {
                self.m[row][col] = self.m[row][0] * rhs.m[0][col]
                    + self.m[row][1] * rhs.m[1][col]
                    + self.m[row][2] * rhs.m[2][col]
                    + self.m[row][3] * rhs.m[3][col]
            }
        }
    }
}

impl Mul<Vector3<f64>> for Matrix4<f64> {
    type Output = Vector3<f64>;

    fn mul(self, rhs: Vector3<f64>) -> Vector3<f64> {
        let mut v_res = Vector3::zero();
        for row in 0..4 {
            match row {
                0 => {
                    v_res.x = self.m[row][0] * rhs.x
                        + self.m[row][1] * rhs.y
                        + self.m[row][2] * rhs.z
                        + self.m[row][3] * rhs.w;
                }
                1 => {
                    v_res.y = self.m[row][0] * rhs.x
                        + self.m[row][1] * rhs.y
                        + self.m[row][2] * rhs.z
                        + self.m[row][3] * rhs.w;
                }
                2 => {
                    v_res.z = self.m[row][0] * rhs.x
                        + self.m[row][1] * rhs.y
                        + self.m[row][2] * rhs.z
                        + self.m[row][3] * rhs.w;
                }
                3 => {
                    v_res.w = self.m[row][0] * rhs.x
                        + self.m[row][1] * rhs.y
                        + self.m[row][2] * rhs.z
                        + self.m[row][3] * rhs.w;
                }
                _ => panic!("Something wild happened..."),
            }
        }
        v_res
    }
}

impl Mul<Point3<f64>> for Matrix4<f64> {
    type Output = Point3<f64>;

    fn mul(self, rhs: Point3<f64>) -> Point3<f64> {
        let mut v_res = Point3::zero();
        for row in 0..4 {
            match row {
                0 => {
                    v_res.x = self.m[row][0] * rhs.x
                        + self.m[row][1] * rhs.y
                        + self.m[row][2] * rhs.z
                        + self.m[row][3] * rhs.w;
                }
                1 => {
                    v_res.y = self.m[row][0] * rhs.x
                        + self.m[row][1] * rhs.y
                        + self.m[row][2] * rhs.z
                        + self.m[row][3] * rhs.w;
                }
                2 => {
                    v_res.z = self.m[row][0] * rhs.x
                        + self.m[row][1] * rhs.y
                        + self.m[row][2] * rhs.z
                        + self.m[row][3] * rhs.w;
                }
                3 => {
                    v_res.w = self.m[row][0] * rhs.x
                        + self.m[row][1] * rhs.y
                        + self.m[row][2] * rhs.z
                        + self.m[row][3] * rhs.w;
                }
                _ => panic!("Something wild happened..."),
            }
        }
        v_res
    }
}

impl Mul<Matrix4<f64>> for Vector3<f64> {
    type Output = Vector3<f64>;

    fn mul(self, rhs: Matrix4<f64>) -> Vector3<f64> {
        let mut v_res = Vector3::zero();
        for row in 0..4 {
            match row {
                0 => {
                    v_res.x = rhs.m[row][0] * self.x
                        + rhs.m[row][1] * self.y
                        + rhs.m[row][2] * self.z
                        + rhs.m[row][3] * self.w
                }
                1 => {
                    v_res.y = rhs.m[row][0] * self.x
                        + rhs.m[row][1] * self.y
                        + rhs.m[row][2] * self.z
                        + rhs.m[row][3] * self.w
                }
                2 => {
                    v_res.z = rhs.m[row][0] * self.x
                        + rhs.m[row][1] * self.y
                        + rhs.m[row][2] * self.z
                        + rhs.m[row][3] * self.w
                }
                3 => {
                    v_res.w = rhs.m[row][0] * self.x
                        + rhs.m[row][1] * self.y
                        + rhs.m[row][2] * self.z
                        + rhs.m[row][3] * self.w
                }
                _ => panic!("Something wild happened"),
            }
        }
        v_res
    }
}

impl Mul<Matrix4<f64>> for Point3<f64> {
    type Output = Point3<f64>;

    fn mul(self, rhs: Matrix4<f64>) -> Point3<f64> {
        let mut v_res = Point3::zero();
        for row in 0..4 {
            match row {
                0 => {
                    v_res.x = rhs.m[row][0] * self.x
                        + rhs.m[row][1] * self.y
                        + rhs.m[row][2] * self.z
                        + rhs.m[row][3] * self.w
                }
                1 => {
                    v_res.y = rhs.m[row][0] * self.x
                        + rhs.m[row][1] * self.y
                        + rhs.m[row][2] * self.z
                        + rhs.m[row][3] * self.w
                }
                2 => {
                    v_res.z = rhs.m[row][0] * self.x
                        + rhs.m[row][1] * self.y
                        + rhs.m[row][2] * self.z
                        + rhs.m[row][3] * self.w
                }
                3 => {
                    v_res.w = rhs.m[row][0] * self.x
                        + rhs.m[row][1] * self.y
                        + rhs.m[row][2] * self.z
                        + rhs.m[row][3] * self.w
                }
                _ => panic!("Something wild happened"),
            }
        }
        v_res
    }
}

impl Matrix2<f64> {
    pub(crate) fn new() -> Self {
        Self { m: [[0f64; 2]; 2] }
    }
    pub(crate) fn determinant(self) -> f64 {
        self.m[0][0] * self.m[1][1] - self.m[0][1] * self.m[1][0]
    }
}

impl Matrix3<f64> {
    pub(crate) fn new() -> Self {
        Self { m: [[0f64; 3]; 3] }
    }

    pub(crate) fn submatrix(self, row_del: usize, col_del: usize) -> Matrix2<f64> {
        let mut res = Matrix2::new();
        let mut r_count = 0;
        let mut c_count = 0;

        for row in 0..3 {
            if row != row_del {
                for col in 0..3 {
                    if col != col_del {
                        res.m[r_count][c_count] = self.m[row][col];
                        c_count += 1;
                    }
                }
                c_count = 0;
                r_count += 1;
            }
        }
        res
    }

    pub(crate) fn minor(self, row_del: usize, col_del: usize) -> f64 {
        self.submatrix(row_del, col_del).determinant()
    }

    pub(crate) fn cofactor(self, row_del: usize, col_del: usize) -> f64 {
        if (row_del + col_del) % 2 == 0 {
            self.minor(row_del, col_del)
        } else {
            -self.minor(row_del, col_del)
        }
    }

    pub(crate) fn determinant(self) -> f64 {
        let mut det = 0f64;
        for col in 0..3 {
            det = det + self.m[0][col] * self.cofactor(0, col);
        }
        det
    }
}

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

use num::{Num, NumCast};
use std::cmp::{Eq, PartialEq};
use std::fmt::Display;
use std::ops::{Mul, MulAssign, Neg};

// Bring Vector module constants into scope
use super::vector::*;

// Bring Geometry module constants into scope
use super::EPSILON;

/**
Matrix 4x4 with generic data type.
Declaration: [[columns] rows]
Data access: [row][col]
*/
pub type Matrix4Data<P> = [[P; 4]; 4];

/// Row of a Matrix 4x4 with generic data type.
pub type Matrix4Row<P> = [P; 4];

/// Column of a Matrix 4x4 with generic data type.
pub type Matrix4Col<P> = [P; 4];

/**
Enum that allows a user to select a Row or a
Column from a Matrix
*/
#[derive(Debug)]
pub enum Matrix4Index {
    /// First Row or Column selector.
    One,
    /// Second Row or Column selector.
    Two,
    /// Third Row or Column selector.
    Three,
    /// Fourth Row or Column selector.
    Four,
}

/**
Matrix 4x4 with generic data.
The data resides in the 'm' component of the structure.
To access the data:
matrix.m[0][0] = 12.5;
*/
#[derive(Clone, Copy, Debug)]
pub struct Matrix4<P> {
    m: Matrix4Data<P>,
}

/**
Matrix3 generic structure.
It is only used in this module to calculate Matrix4 determinat and cofactor.
*/
#[derive(Clone, Copy, Debug)]
pub(crate) struct Matrix3<P> {
    m: [[P; 3]; 3],
}

/**
Matrix2 generic structure.
It is only used in this module to calculate Matrix4 determinat and cofactor.
*/
#[derive(Clone, Copy, Debug)]
pub(crate) struct Matrix2<P> {
    m: [[P; 2]; 2],
}

// -- Implementation of Standard Library Traits

impl<P> Default for Matrix4<P>
where
    P: Copy + Num,
{
    fn default() -> Self {
        let zero: P = num::zero();
        Self {
            m: [
                [zero, zero, zero, zero],
                [zero, zero, zero, zero],
                [zero, zero, zero, zero],
                [zero, zero, zero, zero],
            ],
        }
    }
}

impl<P> Display for Matrix4<P>
where
    P: Copy + Display,
{
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

impl<P> PartialEq for Matrix4<P>
where
    P: Copy + Num + NumCast,
{
    fn eq(&self, other: &Self) -> bool {
        let mut flag = true;
        for i in 0..4 {
            if (self.m[i][0].to_f64().unwrap() - other.m[i][0].to_f64().unwrap()).abs() < EPSILON
                && (self.m[i][1].to_f64().unwrap() - other.m[i][1].to_f64().unwrap()).abs()
                    < EPSILON
                && (self.m[i][2].to_f64().unwrap() - other.m[i][2].to_f64().unwrap()).abs()
                    < EPSILON
                && (self.m[i][3].to_f64().unwrap() - other.m[i][3].to_f64().unwrap()).abs()
                    < EPSILON
            {
                flag = true;
            } else {
                flag = false;
                break;
            }
        }
        flag
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// Implementation Associated Functions with Crate visibility to compute
// determinant and cofactor

impl<P> Matrix4<P>
where
    P: Copy + Num + NumCast + Neg + Neg<Output = P>,
{
    pub(crate) fn cofactor(self, row_del: usize, col_del: usize) -> P {
        if (row_del + col_del) % 2 == 0 {
            self.minor(row_del, col_del)
        } else {
            -self.minor(row_del, col_del)
        }
    }

    pub(crate) fn determinant(self) -> P {
        let mut det = num::zero();
        for col in 0..4 {
            det = det + self.m[0][col] * self.cofactor(0, col);
        }
        det
    }

    pub(crate) fn minor(self, row_del: usize, col_del: usize) -> P {
        self.submatrix(row_del, col_del).determinant()
    }

    pub(crate) fn submatrix(self, row_del: usize, col_del: usize) -> Matrix3<P> {
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
}

/// Trait that provides the capabilities to initialize and transform a Matrix 4x4
pub trait Matrix4Ops<P> {
    /// Returns true if one Matrix is equal to another one.
    fn equal(&self, other: &Self) -> bool;

    /// Returns the row of the matrix based on an user-defined index.
    fn get_row(&self, index: Matrix4Index) -> Matrix4Row<P>;

    /// Returns the row of the matrix based on an user-defined index.
    fn get_col(&self, index: Matrix4Index) -> Matrix4Col<P>;

    /// Returns a new identity matrix.
    fn identity() -> Self;

    /// Returns the inverse of a matrix.
    fn inverse(self) -> Self;

    /// Returns the new matrix with the data provided by the user.
    /// If no data is provided the function returns the matrix filled with '0'.
    fn new(data: Option<Matrix4Data<P>>) -> Self;

    /// Returns a new matrix filled with '1'.
    fn one() -> Self;

    /// Returns rotation matrix around the X axis
    fn rotate_x(&mut self, radians: P) -> Self;

    /// Returns rotation matrix around the Y axis
    fn rotate_y(&mut self, radians: P) -> Self;

    /// Returns rotation matrix around the Z axis
    fn rotate_z(&mut self, radians: P) -> Self;

    /// Returns the scaling matrix.
    fn scale(&mut self, x: P, y: P, z: P) -> Self;

    /// Returns the shearing matrix.
    fn shear(&mut self, xy: P, xz: P, yx: P, yz: P, zx: P, zy: P) -> Self;

    /// Transposes a matrix.
    fn transpose(&mut self) -> Self;

    /// Returns the translation matrix.
    fn translate(&mut self, x: P, y: P, z: P) -> Self;

    /// Reverts the matrix into an idenitity matrix.
    fn to_identity(&mut self) -> Self;

    /// Returns a new matrix filled with '0'.
    fn zero() -> Self;
}

impl<P> Matrix4Ops<P> for Matrix4<P>
where
    P: Copy + Num + NumCast + Neg + Neg<Output = P>,
{
    fn equal(&self, other: &Self) -> bool {
        let mut flag = true;
        for i in 0..4 {
            if (self.m[i][0].to_f64().unwrap() - other.m[i][0].to_f64().unwrap()).abs() < EPSILON
                && (self.m[i][1].to_f64().unwrap() - other.m[i][1].to_f64().unwrap()).abs()
                    < EPSILON
                && (self.m[i][2].to_f64().unwrap() - other.m[i][2].to_f64().unwrap()).abs()
                    < EPSILON
                && (self.m[i][3].to_f64().unwrap() - other.m[i][3].to_f64().unwrap()).abs()
                    < EPSILON
            {
                flag = true;
            } else {
                flag = false;
                break;
            }
        }
        flag
    }

    fn get_row(&self, index: Matrix4Index) -> Matrix4Row<P> {
        let mut row: Matrix4Row<P> = [num::zero(); 4];
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

    fn get_col(&self, index: Matrix4Index) -> Matrix4Col<P> {
        let mut col: Matrix4Col<P> = [num::zero(); 4];
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

    fn identity() -> Self {
        let one: P = num::one();
        let zero: P = num::zero();
        Self {
            m: [
                [one, zero, zero, zero],
                [zero, one, zero, zero],
                [zero, zero, one, zero],
                [zero, zero, zero, one],
            ],
        }
    }

    fn inverse(self) -> Self {
        if self.determinant() == num::zero() {
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

    fn new(data: Option<Matrix4Data<P>>) -> Self {
        match data {
            None => Matrix4Ops::zero(),
            Some(data) => Self { m: data },
        }
    }

    fn one() -> Self {
        Self {
            m: [[num::one(); 4]; 4],
        }
    }

    fn rotate_x(&mut self, radians: P) -> Self {
        let mut res = Matrix4::identity();
        let p_cos = P::from(radians.to_f64().unwrap().cos()).unwrap();
        let p_sin = P::from(radians.to_f64().unwrap().sin()).unwrap();
        res.m[1][1] = p_cos;
        res.m[1][2] = -p_sin;
        res.m[2][1] = p_sin;
        res.m[2][2] = p_cos;
        *self = res * *self;
        *self
    }

    fn rotate_y(&mut self, radians: P) -> Self {
        let mut res = Matrix4::identity();
        let p_cos = P::from(radians.to_f64().unwrap().cos()).unwrap();
        let p_sin = P::from(radians.to_f64().unwrap().sin()).unwrap();
        res.m[0][0] = p_cos;
        res.m[0][2] = p_sin;
        res.m[2][0] = -p_sin;
        res.m[2][2] = p_cos;
        *self = res * *self;
        *self
    }

    fn rotate_z(&mut self, radians: P) -> Self {
        let mut res = Matrix4::identity();
        let p_cos = P::from(radians.to_f64().unwrap().cos()).unwrap();
        let p_sin = P::from(radians.to_f64().unwrap().sin()).unwrap();
        res.m[0][0] = p_cos;
        res.m[0][1] = -p_sin;
        res.m[1][0] = p_sin;
        res.m[1][1] = p_cos;
        *self = res * *self;
        *self
    }

    fn scale(&mut self, x: P, y: P, z: P) -> Self {
        let mut res = Matrix4::identity();
        res.m[0][0] = x;
        res.m[1][1] = y;
        res.m[2][2] = z;
        *self = res * *self;
        *self
    }

    fn shear(&mut self, xy: P, xz: P, yx: P, yz: P, zx: P, zy: P) -> Self {
        let mut res = Matrix4::identity();
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
        let mut res = Matrix4::zero();
        for row in 0..4 {
            res.m[0][row] = self.m[row][0];
            res.m[1][row] = self.m[row][1];
            res.m[2][row] = self.m[row][2];
            res.m[3][row] = self.m[row][3];
        }
        *self = res * *self;
        *self
    }

    fn translate(&mut self, x: P, y: P, z: P) -> Self {
        let mut res = Matrix4::identity();
        res.m[0][3] = x;
        res.m[1][3] = y;
        res.m[2][3] = z;
        *self = res * *self;
        *self
    }

    fn to_identity(&mut self) -> Self {
        *self = Matrix4::identity();
        *self
    }

    fn zero() -> Self {
        Self {
            m: [[num::zero(); 4]; 4],
        }
    }
}

// -- Implementation of Opeperator Overloading

impl<P> Mul for Matrix4<P>
where
    P: Copy + Num + NumCast + Neg + Neg<Output = P>,
{
    type Output = Matrix4<P>;

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

impl<P> MulAssign for Matrix4<P>
where
    P: Copy + Num + NumCast + Neg + Neg<Output = P>,
{
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

impl<P> Mul<Vector3<P>> for Matrix4<P>
where
    P: Copy + Num + NumCast + Neg + Neg<Output = P>,
{
    type Output = Vector3<P>;

    fn mul(self, rhs: Vector3<P>) -> Vector3<P> {
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
                _ => unreachable!("Out of the loop..."),
            }
        }
        v_res
    }
}

impl<P> Mul<Point3<P>> for Matrix4<P>
where
    P: Copy + Num + NumCast + Neg + Neg<Output = P>,
{
    type Output = Point3<P>;

    fn mul(self, rhs: Point3<P>) -> Point3<P> {
        let mut v_res = Point3::all(num::zero());
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
                _ => unreachable!("Out of the loop..."),
            }
        }
        v_res
    }
}

impl<P> Mul<Matrix4<P>> for Vector3<P>
where
    P: Copy + Num + NumCast + Neg + Neg<Output = P>,
{
    type Output = Vector3<P>;

    fn mul(self, rhs: Matrix4<P>) -> Vector3<P> {
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
                _ => unreachable!("Out of the loop"),
            }
        }
        v_res
    }
}

impl<P> Mul<Matrix4<P>> for Point3<P>
where
    P: Copy + Num + NumCast + Neg + Neg<Output = P>,
{
    type Output = Point3<P>;

    fn mul(self, rhs: Matrix4<P>) -> Point3<P> {
        let mut v_res = Point3::all(num::zero());
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
                _ => unreachable!("Out of  the loop..."),
            }
        }
        v_res
    }
}

// Implementation of Matrix2 operations to calculate a determinant.
impl<P> Matrix2<P>
where
    P: Copy + Num + NumCast,
{
    pub(crate) fn new() -> Self {
        let zero: P = num::zero();
        Self { m: [[zero; 2]; 2] }
    }

    pub(crate) fn determinant(self) -> P {
        self.m[0][0] * self.m[1][1] - self.m[0][1] * self.m[1][0]
    }
}

// Implementation of Matrix3 operations to calculate a determinant and submatrix.
impl<P> Matrix3<P>
where
    P: Copy + Num + NumCast + Neg + Neg<Output = P>,
{
    pub(crate) fn new() -> Self {
        let zero: P = num::zero();
        Self { m: [[zero; 3]; 3] }
    }

    pub(crate) fn submatrix(self, row_del: usize, col_del: usize) -> Matrix2<P> {
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

    pub(crate) fn minor(self, row_del: usize, col_del: usize) -> P {
        self.submatrix(row_del, col_del).determinant()
    }

    pub(crate) fn cofactor(self, row_del: usize, col_del: usize) -> P {
        if (row_del + col_del) % 2 == 0 {
            self.minor(row_del, col_del)
        } else {
            -self.minor(row_del, col_del)
        }
    }

    pub(crate) fn determinant(self) -> P {
        let mut det = num::zero();
        for col in 0..3 {
            det = det + self.m[0][col] * self.cofactor(0, col);
        }
        det
    }
}

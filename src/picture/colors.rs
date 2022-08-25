// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/**
Data structures and operations for the Colors type
*/
use std::{
    cmp::{Eq, PartialEq},
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::geometry::EPSILON;

// Colors Unit Tests
#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone)]
/// Represent a color in Red, Green and Blue format
pub struct ColorRgb {
    /// Red component
    pub r: f64,
    /// Green component
    pub g: f64,
    /// Blue component
    pub b: f64,
}

impl Display for ColorRgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("({:^2.2},{:^2.2},{:^2.2})", self.r, self.g, self.b);
        f.write_str(&s)
    }
}

impl Default for ColorRgb {
    fn default() -> Self {
        Self {
            r: Default::default(),
            g: Default::default(),
            b: Default::default(),
        }
    }
}

impl PartialEq for ColorRgb {
    fn eq(&self, other: &Self) -> bool {
        // self.r == other.r && self.g == other.g && self.b == other.b
        self.equal(other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.equal(other)
    }
}
impl Eq for ColorRgb {}

/// Enables effective Color initialization
pub trait ColorInit<T> {
    /// .
    fn new(r: f64, g: f64, b: f64) -> T;
    /// .
    fn red() -> T;
    /// .
    fn green() -> T;
    /// .
    fn blue() -> T;
    /// .
    fn black() -> T;
    /// .
    fn white() -> T;
    /// .
    fn equal(self, other:&T) -> bool;
}

impl ColorInit<ColorRgb> for ColorRgb {
    fn new(r: f64, g: f64, b: f64) -> ColorRgb {
        ColorRgb { r, g, b }
    }

    fn red() -> ColorRgb {
        ColorRgb {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        }
    }

    fn green() -> ColorRgb {
        ColorRgb {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        }
    }

    fn blue() -> ColorRgb {
        ColorRgb {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        }
    }

    fn black() -> ColorRgb {
        ColorRgb {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    fn white() -> ColorRgb {
        ColorRgb {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    fn equal(self, other:&ColorRgb) -> bool {
       if (self.r - other.r).abs() < EPSILON && (self.g - other.g).abs() < EPSILON && (self.b - other.b).abs() < EPSILON {
           true
       }
       else{
           false
       }
    }
}

impl Add for ColorRgb {
    type Output = ColorRgb;

    fn add(self, rhs: Self) -> ColorRgb {
        ColorRgb {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for ColorRgb {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Sub for ColorRgb {
    type Output = ColorRgb;

    fn sub(self, rhs: Self) -> ColorRgb {
        ColorRgb {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl SubAssign for ColorRgb {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl Mul for ColorRgb {
    type Output = ColorRgb;

    fn mul(self, rhs: Self) -> ColorRgb {
        ColorRgb {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl MulAssign for ColorRgb {
    fn mul_assign(&mut self, rhs: Self) {
        self.r = self.r * rhs.r;
        self.g = self.g * rhs.g;
        self.b = self.b * rhs.b;
    }
}

impl Mul<usize> for ColorRgb {
    type Output = ColorRgb;

    fn mul(self, rhs: usize) -> ColorRgb {
        ColorRgb {
            r: self.r * rhs as f64,
            g: self.g * rhs as f64,
            b: self.b * rhs as f64,
        }
    }
}

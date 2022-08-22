// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Provides Unit tests for Matrix3 types.

#[cfg(test)]
mod tests;

#[derive(Debug)]
/// Definition of Matrix4
pub struct Matrix4 {
    md: [[f64; 4]; 4],
}

impl Matrix4 {
    /// Creates a new [`Matrix4`].
    pub fn new(m: [[f64; 4]; 4]) -> Self {
        Self { md: m }
    }

    /// Returns the md of this [`Matrix4`].
    pub fn md(&self) -> [[f64; 4]; 4] {
        self.md
    }

    /// Sets the md of this [`Matrix4`].
    pub fn set_md(&mut self, m: [[f64; 4]; 4]) {
        self.md = m;
    }

}

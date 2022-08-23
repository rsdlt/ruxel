// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/**
Data structure and operations for the Canvas and Pixel types
*/

use std::fmt::{Display, Write};
use std::cmp::{max, min};

use crate::picture::colors::*;

// Canvas Unit Tests
#[cfg(test)]
mod tests;

/// .
#[derive(Debug, Copy, Clone)]
pub struct Pixel {
/// .
    x: usize,
/// .
    y: usize,
/// .
    color: ColorRgb,
}


#[derive(Debug)]
/// .
pub struct Canvas {
    /// .
    width: usize,
    /// .
    height: usize,
    /// .
    data: Vec<Vec<ColorRgb>>,
}


impl Pixel {
    pub fn color(&self) -> ColorRgb {
        self.color
    }

    pub fn set_color(&mut self, c: ColorRgb) {
        self.color = c;
    }

    pub fn new(x: usize, y: usize, color: ColorRgb) -> Pixel {
        Pixel{x, y, color}
    }
}

impl Default for Pixel{
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), color: Default::default() }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       let s = format!("pix - [x:{}, y:{}] c:{}", self.x, self.y, self.color);
       f.write_str(&s)
    }
}


impl Canvas{
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas{width, 
            height,
           data:  vec![vec![ColorRgb::black();height];width]
        }
    }

    pub fn write_pixel(&mut self, pixel: Pixel){
        self.data[pixel.x][pixel.y] = pixel.color;
    }

    pub fn write_to_ppm(&self, file_name: &str){
        let mut s = "".to_string();
        for row in self.data.iter()  {
            for color in row {
                print!("{} ", (color.r * 255.0) as usize); // FIX: need to print properly
            }
            print!("\n");
        }
    }
}

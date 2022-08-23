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
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;

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
        Pixel { x, y, color }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            color: Default::default(),
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("pix - [x:{}, y:{}] c:{}", self.x, self.y, self.color);
        f.write_str(&s)
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            data: vec![vec![ColorRgb::default(); height]; width],
        }
    }

    pub fn write_pixel(&mut self, pixel: Pixel) {
        // Filling the canvas with the corresponding pixel color
        // In order to transform to Canvas coordinates we need to
        // substract the canvas height from the pixel.y position
        // The '-1' is to not get an out of bounds error on the vector for the
        // first iteration
        self.data[pixel.x][self.height - 1 - pixel.y] = pixel.color;
    }

    pub fn write_to_ppm(&self, file_name: &str) {
        let mut image = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name)
            .expect("Cannot open image file");

        let mut colors_per_ppm_line: u8 = 0; // Counter for number of colors per ppm line to be printed in
                                             // PPM in order to not exceed the recommended 70 characters
                                             // per row. Max # chars per color = 12;
                                             // Max # colors per row = 70 / 12 = 5.8 -> 5

        let mut image_file_content = format!("{}\n{} {}\n{}\n", "P3", self.width, self.height, 255);

        for i in 0..self.height {
            for j in 0..self.width {
                if colors_per_ppm_line >= 5 {
                    image_file_content.push('\n');
                    colors_per_ppm_line = 0;
                }
                image_file_content.push_str(
                    format!(
                        "{} {} {} ",
                        ((self.data[j][i].r * 255f64).ceil() as u8).clamp(0, 255),
                        ((self.data[j][i].g * 255f64).ceil() as u8).clamp(0, 255),
                        ((self.data[j][i].b * 255f64).ceil() as u8).clamp(0, 255)
                    )
                    .as_str(),
                );
                colors_per_ppm_line += 1;
                image_file_content.push('\n');
            }
        }
        image
            .write_all(image_file_content.as_bytes())
            .expect("Write failed");
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            data: Default::default(),
        }
    }
}

impl Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!(
            "Canvas - \n width : height [{} : {}] \n",
            self.width, self.height
        );
        f.write_str(&s)
    }
}

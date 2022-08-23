// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Unit testing for the Canvas types
use super::*;

#[test]
// This test validates the writing of a Canvas
fn test_print_to_ppm() {
    let mut canvas = Canvas::new(5, 3);
    let c1 = ColorRgb::new(1.5, 0.0, 0.0);
    let c2 = ColorRgb::new(0.0, 0.5, 0.0);
    let c3 = ColorRgb::new(-0.5, 0.0, 1.0);
    canvas.write_pixel(Pixel::new(0, 0, c1));
    canvas.write_pixel(Pixel::new(2, 1, c2));
    canvas.write_pixel(Pixel::new(4, 2, c3));
    canvas.write_to_ppm("hola");
}

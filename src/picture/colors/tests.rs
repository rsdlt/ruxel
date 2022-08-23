// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Unit testing for the Colors type

/// Unit testing for the Colors types
use super::*;

#[test]
// This test checks for the integrity of the Colors initialization
fn test_color_initialization() {
    let red = ColorRgb::red();
    let black = ColorRgb::black();
    let white = ColorRgb::white();
    println!("{}", red);
    assert!(
        red == ColorRgb {
            r: 1.0,
            ..Default::default()
        } && black
            == ColorRgb {
                ..Default::default()
            }
            && white == ColorRgb::new(1.0, 1.0, 1.0)
    )
}
#[test]
// This test checks for the integrity of Add, AddAssing, Sub, SubAssign, Mul and MulAssing
fn test_color_operator_overloading() {
    let c1 = ColorRgb::new(0.9, 0.6, 0.75);
    let c2 = ColorRgb::new(0.7, 0.1, 0.25);
    assert_eq!(c1 + c2, ColorRgb::new(1.6, 0.7, 1.0));
    assert_eq!(c1 - c2, ColorRgb::new(0.2, 0.5, 0.5));
    assert_eq!(
        ColorRgb::new(1.0, 0.2, 0.4) * ColorRgb::new(0.9, 1.0, 0.1),
        ColorRgb::new(0.9, 0.2, 0.04)
    );
    let mut c3 = ColorRgb::new(0.9, 0.6, 0.75);
    let c4 = ColorRgb::new(0.7, 0.1, 0.25);
    c3 += c4;
    assert!(c3 == ColorRgb::new(1.6, 0.7, 1.0));
    c3 -= c4;
    assert!(c3 == ColorRgb::new(0.9, 0.6, 0.75));
    c3 *= c4;
    assert!(c3 == ColorRgb::new(0.9, 0.2, 0.04));
}

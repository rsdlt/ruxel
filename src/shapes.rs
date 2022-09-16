// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/**
The Shapes module implements the functionality for Core shapes like Circle, Cylinder, Cube, and for External shapes from import of *.OBJ files or other formats
*/

/// Provides the data structure and implementation of the Core shapes
pub mod core;

/// Provides the data structure and implementation to import External shapes
pub mod external;

#[derive(Debug)]
/// Data structure representing a Shape.
pub struct Shape {}

/*!
Vector module implements Vector3
*/

/*!
Declaration of Vector3
uses Clone and Copy traits because all elements are primitives in the stack
*/
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    /// Component on x axis
    pub x: f64,
    /// Component on y axis
    pub y: f64,
    /// Componenet on z axis
    pub z: f64,
    /// Private component that prevents user for arbitrarily changing Vector to Point or viceversa
    _w: f64,
}

impl Default for Vector3 {
    fn default() -> Self {
        Self {
            x: 0f64,
            y: 0f64,
            z: 0f64,
            _w: 0f64,
        }
    }
}

/// Trait defines creation of new vectors and points
pub trait NewDecl {
    fn new(x: f64, y: f64, z: f64) -> Self;
    fn one() -> Self;
    fn zero() -> Self;
    fn new_point(x: f64, y: f64, z: f64) -> Self;
    fn one_point() -> Self;
    fn zero_point() -> Self;
}

impl NewDecl for Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z, _w: 0f64 }
    }

    fn one() -> Self {
        Vector3 {
            x: 1f64,
            y: 1f64,
            z: 1f64,
            _w: 0f64,
        }
    }
    fn zero() -> Self {
        Vector3 {
            x: 0f64,
            y: 0f64,
            z: 0f64,
            _w: 0f64,
        }
    }

    fn new_point(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z, _w: 1f64 }
    }

    fn one_point() -> Self {
        Vector3 {
            x: 1f64,
            y: 1f64,
            z: 1f64,
            _w: 1f64,
        }
    }

    fn zero_point() -> Self {
        Vector3 {
            x: 0f64,
            y: 0f64,
            z: 0f64,
            _w: 1f64,
        }
    }
}

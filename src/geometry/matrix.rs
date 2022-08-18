/*!
  Module to implement the Matrix4
*/

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

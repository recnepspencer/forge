//! # forge-geom
//!
//! Analytic surfaces, NURBS, and curve representations
//! for the Forge geometry kernel.
//!
//! Geometry is a binding layer — it may be approximate, but it carries
//! bounded error metrics and never corrupts topology (Doctrine D3).

#![forbid(unsafe_code)]

pub mod plane;
pub mod implicit_vertex;
pub mod bsp;

use forge_core::{GeometrySource, KernelError};
use crate::plane::Plane;

/// Standard grid scale for spatial hashing (1 unit = 1e6 integers).
pub const GRID_SCALE: f64 = 1e6;

/// A collection of planes that implements `GeometrySource`.
pub struct PlaneSet(pub Vec<Plane>);

impl GeometrySource for PlaneSet {
    fn get_plane(&self, index: usize) -> Result<[f64; 4], KernelError> {
        let p = self.0.get(index).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("Plane index {} out of bounds", index),
                context: None,
            }
        })?;
        let n = p.raw_normal();
        Ok([n[0], n[1], n[2], p.raw_offset()])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(2 + 2, 4);
    }
}

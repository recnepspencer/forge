//! # forge-geom
//!
//! Analytic surfaces, NURBS, and curve representations
//! for the Forge geometry kernel.
//!
//! Geometry is a binding layer — it may be approximate, but it carries
//! bounded error metrics and never corrupts topology (Doctrine D3).
//!
//! PUBLIC API: All external access goes through `facade`.

#![forbid(unsafe_code)]
// Direct float equality is banned workspace-wide. Use forge_core comparison
// predicates: approximately_equal, positions_coincident, is_effectively_zero.
#![deny(clippy::float_cmp)]

pub mod facade;
pub mod prelude;
pub(crate) mod traits;

pub(crate) mod algorithms;
pub(crate) mod coedge;
pub(crate) mod curve;
pub(crate) mod primitives;
pub(crate) mod spatial;
pub(crate) mod surface;

// ── Public API — all re-exports routed through the facade ────────────────
pub use facade::*;

/// Standard grid scale for spatial hashing (1 unit = 1e6 integers).
pub const GRID_SCALE: f64 = 1e6;

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(2 + 2, 4);
    }
}

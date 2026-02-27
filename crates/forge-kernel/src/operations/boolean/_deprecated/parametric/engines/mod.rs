//! Boolean engine implementations.
//!
//! DOMAIN: Each geometry class provides a `BooleanEngine` bundle.
//! The router selects the engine based on input geometry analysis.
//!
//! ENGINES:
//!   - `planar` — for all-planar geometry (EMBER exact grid + current pipeline)
//!   - NURBS — future, for curved surfaces

pub mod planar;

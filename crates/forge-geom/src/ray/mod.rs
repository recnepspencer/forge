//! Ray-geometry intersection and projection utilities.
//!
//! DOMAIN: Ray-plane intersection, 2D projection axis selection,
//! and zero-edge resolution for point-in-solid classification.
//!
//! DEPENDENCIES: `forge-math` (sign types)
//!
//! INVARIANTS:
//! - All floating-point geometry computations live here, not in `forge-topo`
//! - Degeneracy thresholds are explicit parameters, never hardcoded

mod eval;

pub use eval::compute_ray_plane_intersection;
pub use eval::{resolve_zero_edge, EdgeTieBreaker};
pub use eval::dominant_projection_axes;

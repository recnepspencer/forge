//! Spatial queries bridging forge-topo topology and worth-geom geometry.
//!
//! DOMAIN: Point-in-solid classification, AABB bounds aggregation, and
//!         geometric invariant validation — functions that require both
//!         topology traversal (via forge-topo) and geometric types
//!         (via worth-geom) and therefore cannot live in either crate alone.
//!
//! DEPENDENCIES: forge-topo (arena, handles, traversal), worth-geom (Aabb,
//!               BvhNode, polygon helpers), worth-math (predicates, linalg).
//!
//! INVARIANTS:
//! - Geometry is always received via caller-provided position callbacks.
//! - No topology mutation occurs in this crate.
//! - All public functions are deterministic for identical inputs.
//!
//! PUBLIC API: All external access goes through the façades.
//! Internal modules are organized into `operations/` (spatial queries)
//! and `validators/` (geometric invariant checks).

// Direct float equality is banned workspace-wide. Use forge_core comparison
// predicates: approximately_equal, positions_coincident, is_effectively_zero.
#![deny(clippy::float_cmp)]

pub mod operations;
pub mod validators;

// ── Public API — re-exports routed through façades ───────────────────────────
pub use operations::facade::*;
pub use validators::facade::*;

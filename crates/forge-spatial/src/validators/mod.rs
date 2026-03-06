//! Geometric invariant validation for topology arenas.
//!
//! DOMAIN: Position-dependent integrity checks — zero-area faces,
//!         zero-length edges, inverted shell orientation, sliver face
//!         detection, and face-to-face gap measurement.
//!
//! ARCHITECTURE: Uses the shared `InvariantId` contract from `forge-core`.
//! `dispatch::spatial_validator_for()` maps each `InvariantId` to its
//! geometry-dependent checker — the spatial counterpart to
//! `forge-topo::validator_for()`.
//!
//! DEPENDENCIES: forge-topo (arena, handles, traversal),
//!               forge-geom (polygon area, plane), forge-core (KernelError, ToleranceProvider).
//! INVARIANTS: No topology mutation. Requires a position callback.

pub mod area;
pub mod completeness;
pub mod dispatch;
pub mod edge_curve_consistency;
pub mod edge_length;
pub mod facade;
pub mod gap;
pub mod loop_orientation;
pub mod shell_orientation;
pub mod sliver;
pub mod surface_deviation;
pub(crate) mod utils;
pub mod volume;

use forge_core::KernelError;
use forge_topo::b_rep::TopologyArena;

pub use dispatch::GeometryContext;

/// Validate all geometric invariants that require vertex positions.
///
/// Runs zero-area face, zero-length edge, signed-volume, surface deviation,
/// and edge-curve consistency checks via the dispatch system.
pub fn validate_geometric_invariants(
    arena: &TopologyArena,
    ctx: &GeometryContext<'_>,
) -> Result<(), KernelError> {
    dispatch::validate_all_spatial_invariants(arena, ctx)
}

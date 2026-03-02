//! Geometric invariant validation for topology arenas.
//!
//! DOMAIN: Position-dependent integrity checks — zero-area faces,
//!         zero-length edges, inverted shell orientation, sliver face
//!         detection, and face-to-face gap measurement.
//!
//! DEPENDENCIES: forge-topo (arena, handles, traversal),
//!               forge-geom (polygon area, plane), forge-core (KernelError, ToleranceProvider).
//! INVARIANTS: No topology mutation. Requires a position callback.

pub mod facade;
pub mod area;
pub mod completeness;
pub mod edge_length;
pub mod gap;
pub mod sliver;
pub mod volume;

use forge_core::{KernelError, ToleranceProvider};
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::VertexId;

/// Validate all geometric invariants that require vertex positions.
///
/// Runs zero-area face, zero-length edge, and signed-volume checks.
/// The `is_planar` callback allows skipping area/volume checks for
/// non-planar faces (e.g., NURBS patches).
pub fn validate_geometric_invariants(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    is_planar: &dyn Fn(forge_topo::handles::FaceId) -> bool,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    area::validate_zero_area_faces(arena, position_fn, is_planar, tolerance_provider)?;
    edge_length::validate_zero_length_edges(arena, position_fn, tolerance_provider)?;
    volume::validate_signed_volume(arena, position_fn)?;
    Ok(())
}

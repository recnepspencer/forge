//! Vertex position lookup by raw slot index.
//!
//! DOMAIN: Bridge between the raw `u32` slot-index callback interface used by
//! `forge-spatial::classify_point_in_solid` and the typed `VertexId`/`GeometryState`
//! API used throughout `forge-kernel`.
//!
//! INVARIANT: This is the single correct place to do raw-slot reconstruction.
//! When NURBS, trimmed surfaces, or other geometry stores arrive, their position
//! lookups will use the same slot-index bridge — this function grows with them.

use forge_core::KernelError;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::VertexId;

use crate::geometry_state::GeometryState;

/// Look up a vertex position by its raw arena slot index.
///
/// The `forge-spatial` classification API uses raw `u32` slot indices rather
/// than typed handles so it can remain crate-independent. This function
/// reconstructs the typed `VertexId` from the slot index + generation round-trip
/// into the arena, then delegates to `GeometryState::get_vertex_position`.
///
/// # Errors
/// Returns `KernelError::InvalidInput` when no active vertex occupies `slot_index`
/// or when no position is registered for the reconstructed handle.
pub fn lookup_vertex_position_by_slot(
    arena: &TopologyArena,
    geometry: &GeometryState,
    slot_index: u32,
) -> Result<[f64; 3], KernelError> {
    let gen = arena
        .vertex_generation(slot_index as usize)
        .ok_or_else(|| KernelError::InvalidInput {
            message: format!("No active vertex at slot index {}", slot_index),
            context: None,
        })?;
    let vid = VertexId::new(slot_index, gen);
    geometry
        .get_vertex_position(vid)
        .copied()
        .ok_or_else(|| KernelError::InvalidInput {
            message: format!("No position registered for vertex at slot {}", slot_index),
            context: None,
        })
}

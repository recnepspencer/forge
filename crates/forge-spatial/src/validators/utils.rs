//! Shared helpers for spatial validators.
//!
//! DOMAIN: Common position-collection routines used by area, sliver,
//! loop_orientation, and shell_orientation validators.
//!
//! DEPENDENCIES: forge-topo (arena, handles, traversal), forge-core (KernelError).

use forge_core::KernelError;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::traverse::{FaceEdgeIterator, LoopEdgeIterator};

/// Collect vertex positions around a face's outer loop.
///
/// Walks the face's `FaceEdgeIterator` (outer loop half-edges) and collects
/// each vertex's position via `position_fn`. Returns `MissingVertexPosition`
/// if any vertex has no geometry bound.
pub fn collect_face_positions(
    arena: &TopologyArena,
    face_id: FaceId,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let mut positions = Vec::new();
    for he_res in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_res?;
        let he = arena.get_half_edge(he_id)?;
        let v = he.origin();
        let pos = position_fn(v).ok_or_else(|| KernelError::TopologyViolation {
            err: forge_core::TopologyError::MissingVertexPosition {
                vertex_index: v.index(),
                face_index: face_id.index(),
            },
            context: None,
        })?;
        positions.push(pos);
    }
    Ok(positions)
}

/// Collect vertex positions for a specific loop.
///
/// Walks the loop's `LoopEdgeIterator` and collects each vertex's position.
/// Returns `MissingVertexPosition` if any vertex has no geometry bound.
pub fn collect_loop_positions(
    arena: &TopologyArena,
    loop_id: forge_topo::handles::LoopId,
    face_id: FaceId,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let mut positions = Vec::new();
    for he_res in LoopEdgeIterator::new(arena, loop_id)? {
        let he_id = he_res?;
        let he = arena.get_half_edge(he_id)?;
        let v = he.origin();
        let pos = position_fn(v).ok_or_else(|| KernelError::TopologyViolation {
            err: forge_core::TopologyError::MissingVertexPosition {
                vertex_index: v.index(),
                face_index: face_id.index(),
            },
            context: None,
        })?;
        positions.push(pos);
    }
    Ok(positions)
}

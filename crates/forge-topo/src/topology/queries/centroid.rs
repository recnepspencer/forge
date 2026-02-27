//! Face centroid computation from topology + position callback.
//!
//! DOMAIN: Compute the centroid of a face's outer loop. Uses
//! `face_loop_vertices` for topology traversal and a caller-provided
//! position resolver for coordinate lookup. This keeps forge-topo
//! geometry-agnostic (no dependency on GeometryState or GeometryStore).

use forge_core::KernelError;
use crate::arena::TopologyArena;
use crate::handles::{FaceId, VertexId};
use super::polygon::face_loop_vertices;

/// Compute the centroid of a face's outer loop.
///
/// The `position_of` callback resolves each `VertexId` to its `[f64; 3]`
/// coordinates. This keeps the function independent of any specific
/// geometry store implementation.
///
/// Returns `None` if the face has no outer loop vertices or if all
/// position lookups fail.
pub fn face_centroid(
    arena: &TopologyArena,
    face: FaceId,
    position_of: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<Option<[f64; 3]>, KernelError> {
    let loops = face_loop_vertices(arena, face)?;
    let outer_loop = match loops.first() {
        Some(l) => l,
        None => return Ok(None),
    };

    let positions: Vec<[f64; 3]> = outer_loop
        .iter()
        .filter_map(|vid| position_of(*vid))
        .collect();

    if positions.is_empty() {
        return Ok(None);
    }

    let n = positions.len() as f64;
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut cz = 0.0;
    for p in &positions {
        cx += p[0];
        cy += p[1];
        cz += p[2];
    }

    Ok(Some([cx / n, cy / n, cz / n]))
}

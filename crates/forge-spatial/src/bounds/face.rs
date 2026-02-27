//! Face-level AABB queries.
//!
//! DOMAIN: Compute axis-aligned bounding boxes for individual faces and
//!         all faces in the arena. Used by BVH construction and spatial
//!         pre-filtering in the boolean pipeline.

use forge_core::KernelError;
use forge_geom::Aabb;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::topology::queries::traverse::FaceAllEdgesIterator;

/// Compute an AABB for a face by traversing all loops and collecting vertex positions.
pub fn face_bounds(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    face: FaceId,
) -> Result<Option<Aabb>, KernelError> {
    arena.get_face(face)?;

    let mut result: Option<Aabb> = None;

    for he_res in FaceAllEdgesIterator::new(arena, face)? {
        let he_id = he_res?;
        let vertex_id = arena.get_half_edge(he_id)?.origin();
        let Some(point) = position_fn(vertex_id) else {
            continue;
        };

        let point_box = Aabb::new(point, point);
        result = match result {
            Some(bounds) => Some(bounds.union(&point_box)),
            None => Some(point_box),
        };
    }

    Ok(result)
}

/// Compute AABBs for all faces in deterministic arena order.
pub fn all_face_bounds(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<Vec<(FaceId, Aabb)>, KernelError> {
    let mut list = Vec::new();
    for (face_id, _) in arena.iter_faces() {
        if let Some(bounds) = face_bounds(arena, position_fn, face_id)? {
            list.push((face_id, bounds));
        }
    }
    Ok(list)
}

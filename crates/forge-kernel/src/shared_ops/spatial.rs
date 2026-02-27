//! Spatial query helpers that bridge topology + geometry + tolerance.
//!
//! DOMAIN: Face/solid bounding volume queries combining TopologyArena,
//!   GeometryState, and ToleranceConfig.
//! DEPENDENCIES: crate::spatial (forge-spatial re-exports), GeometryState,
//!   forge_topo, forge-geom Aabb.
//! INVARIANTS: All functions are read-only — no topology or geometry mutation.

use forge_core::KernelError;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::FaceId;

use crate::geom_facade::{query_overlapping_pairs, Aabb, BvhNode};
use crate::geometry_state::GeometryState;

/// Compute inflated AABBs for every face in a solid.
///
/// Calls `forge_spatial::bounds::face::all_face_bounds` and expands each
/// result by `inflation` to account for floating-point slop during BVH pair
/// queries.
///
/// Used by: boolean split BVH pass, supplement cut pass, any future
/// BVH-accelerated feature that needs face-level bounding volumes.
pub fn compute_face_aabbs(
    arena: &TopologyArena,
    geom: &GeometryState,
    inflation: f64,
) -> Result<Vec<(FaceId, Aabb)>, KernelError> {
    let mut list =
        crate::spatial::all_face_bounds(arena, &|vid| geom.get_vertex_position(vid).copied())?;
    for (_, aabb) in &mut list {
        aabb.expand(inflation);
    }
    Ok(list)
}

/// Build two BVH trees from pre-computed face AABB lists and return all
/// overlapping `(target_face, tool_face)` pairs in deterministic order.
///
/// This is the canonical BVH query kernel used by:
/// - Boolean split: `cut_proposal::build_bvh_overlap_pairs`
/// - Coincidence prepass: `shared_ops::coincidence::build_face_coincidence_prepass`
/// - Future: fillet candidate detection, offset self-intersection checks,
///   NURBS surface-surface intersection seeding.
///
/// The returned pairs are sorted `(a, b)` by `(target_index, tool_index)`
/// for deterministic output regardless of BVH traversal order.
pub fn build_face_bvh_pairs(
    target_aabbs: &[(FaceId, Aabb)],
    tool_aabbs: &[(FaceId, Aabb)],
) -> Result<Vec<(FaceId, FaceId)>, KernelError> {
    let target_indexed: Vec<(usize, Aabb)> = target_aabbs
        .iter()
        .enumerate()
        .map(|(i, (_, aabb))| (i, aabb.clone()))
        .collect();
    let tool_indexed: Vec<(usize, Aabb)> = tool_aabbs
        .iter()
        .enumerate()
        .map(|(i, (_, aabb))| (i, aabb.clone()))
        .collect();

    let root_a =
        BvhNode::build(target_indexed).ok_or_else(|| KernelError::InternalError {
            message: "Failed to build target BVH — no faces with valid AABBs".into(),
            context: None,
        })?;
    let root_b =
        BvhNode::build(tool_indexed).ok_or_else(|| KernelError::InternalError {
            message: "Failed to build tool BVH — no faces with valid AABBs".into(),
            context: None,
        })?;

    let mut raw_pairs = query_overlapping_pairs(&root_a, &root_b);
    raw_pairs.sort_unstable_by_key(|(a, b)| (*a, *b));

    Ok(raw_pairs
        .iter()
        .map(|(ia, ib)| (target_aabbs[*ia].0, tool_aabbs[*ib].0))
        .collect())
}

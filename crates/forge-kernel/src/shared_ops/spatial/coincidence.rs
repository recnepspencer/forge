//! BVH-accelerated face coincidence prepass.
//!
//! DOMAIN: Detects coplanar face pairs between two solids before the
//! split phase. Uses AABB overlap (BVH) to find candidates, then
//! exact rational coplanarity confirmation (D3-compliant).

use crate::geom_facade::Aabb;
use crate::geom_facade::{query_overlapping_pairs, BvhNode};
use crate::geom_facade::{CoincidenceGraph, CoincidenceKind};
use crate::geom_facade::Plane;
use forge_topo::handles::FaceId;

/// Pack a `FaceId` into a raw `u64` handle for use in `CoincidenceGraph` edges.
///
/// Uses the same bit-layout as `pack_handle` in `geometry_state`: `gen << 32 | idx`.
/// This must stay consistent with `unpack_face_id`.
const COINCIDENCE_SIDE_TAG_BIT: u64 = 1u64 << 63;

#[inline]
fn pack_face_id(fid: FaceId, is_tool: bool) -> u64 {
    let raw = ((fid.generation() as u64) << 32) | (fid.index() as u64);
    if is_tool {
        raw | COINCIDENCE_SIDE_TAG_BIT
    } else {
        raw & !COINCIDENCE_SIDE_TAG_BIT
    }
}

/// Reconstruct a `FaceId` from a raw `u64` handle produced by `pack_face_id`.
#[inline]
fn unpack_face_id(raw: u64) -> FaceId {
    let raw = raw & !COINCIDENCE_SIDE_TAG_BIT;
    let idx = (raw & 0xFFFF_FFFF) as u32;
    let gen = (raw >> 32) as u32;
    FaceId::new(idx, gen)
}

/// Build a `CoincidenceGraph` for two solids using a BVH-accelerated face prepass.
///
/// # Algorithm
/// 1. Collect `(packed u64, Aabb)` pairs for each solid using `arena.iter_faces()`.
/// 2. Build two independent `BvhNode` trees.
/// 3. `query_overlapping_pairs` finds AABB-intersecting candidates in `O(n log n)`.
/// 4. Confirm each candidate as `CoplanarFaces` via `coplanar_eq` (exact rational
///    arithmetic — no tolerance, D3-compliant).
/// 5. Record confirmed pairs in `CoincidenceGraph` with canonical `(min, max)` key.
///
/// Faces without registered geometry are silently skipped (not a fatal error —
/// they will be processed normally by the intersection pipeline).
pub fn build_face_coincidence_prepass(
    target_arena: &forge_topo::b_rep::TopologyArena,
    target_geom: &crate::geometry_state::GeometryState,
    tool_arena: &forge_topo::b_rep::TopologyArena,
    tool_geom: &crate::geometry_state::GeometryState,
) -> CoincidenceGraph {
    let mut graph = CoincidenceGraph::new();

    let target_items: Vec<(u64, Aabb)> = crate::spatial::all_face_bounds(target_arena, &|vid| {
        target_geom.get_vertex_position(vid).copied()
    })
    .unwrap_or_default()
    .into_iter()
    .map(|(fid, aabb)| (pack_face_id(fid, false), aabb))
    .collect();

    let tool_items: Vec<(u64, Aabb)> = crate::spatial::all_face_bounds(tool_arena, &|vid| {
        tool_geom.get_vertex_position(vid).copied()
    })
    .unwrap_or_default()
    .into_iter()
    .map(|(fid, aabb)| (pack_face_id(fid, true), aabb))
    .collect();

    let target_bvh = match BvhNode::build(target_items) {
        Some(tree) => tree,
        None => return graph,
    };
    let tool_bvh = match BvhNode::build(tool_items) {
        Some(tree) => tree,
        None => return graph,
    };

    let candidates = query_overlapping_pairs(&target_bvh, &tool_bvh);

    for (target_raw, tool_raw) in candidates {
        let target_fid = unpack_face_id(target_raw);
        let tool_fid = unpack_face_id(tool_raw);

        let Some(plane_a) = target_geom.get_face_plane(target_fid) else {
            continue;
        };
        let Some(plane_b) = tool_geom.get_face_plane(tool_fid) else {
            continue;
        };

        if !crate::geom_facade::coplanar_eq(plane_a, plane_b) {
            continue;
        }

        let gap_mm = plane_a.offset().abs();

        graph.insert_edge(
            target_raw,
            tool_raw,
            CoincidenceKind::CoplanarFaces { gap_mm },
        );
    }

    graph
}

/// Check if two planes have parallel normals (same or opposite direction).
///
/// Delegates to `forge_geom::primitives::plane::are_parallel_exact`, which
/// uses exact rational cross product. No tolerance, no magic numbers — D3 compliant.
pub(crate) fn planes_are_parallel(a: &Plane, b: &Plane) -> bool {
    crate::geom_facade::are_parallel_exact(a, b)
}

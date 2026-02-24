//! Evaluation logic for Boolean operations.
//!
//! Includes vertex match key computation for robust deduplication.
//! Vertices are matched by their exact rational 3D position.
//! This key is TRANSIENT (used only during the boolean phase).
//! The vertex's permanent identity remains its `VertexId` + `Lineage`.

use forge_geom::Plane;
use forge_geom::primitives::aabb::Aabb;
use forge_geom::spatial::bvh::{BvhNode, query_overlapping_pairs};
use forge_geom::spatial::coincidence::{CoincidenceGraph, CoincidenceKind};
use forge_math::arithmetic::Rational;

/// Pack a `FaceId` into a raw `u64` handle for use in `CoincidenceGraph` edges.
///
/// Uses the same bit-layout as `pack_handle` in `geometry_store`: `gen << 32 | idx`.
/// This must stay consistent with `unpack_face_id`.
const COINCIDENCE_SIDE_TAG_BIT: u64 = 1u64 << 63;

#[inline]
fn pack_face_id(fid: forge_topo::handles::FaceId, is_tool: bool) -> u64 {
    let raw = ((fid.generation() as u64) << 32) | (fid.index() as u64);
    if is_tool {
        raw | COINCIDENCE_SIDE_TAG_BIT
    } else {
        raw & !COINCIDENCE_SIDE_TAG_BIT
    }
}

/// Reconstruct a `FaceId` from a raw `u64` handle produced by `pack_face_id`.
#[inline]
fn unpack_face_id(raw: u64) -> forge_topo::handles::FaceId {
    let raw = raw & !COINCIDENCE_SIDE_TAG_BIT;
    let idx = (raw & 0xFFFF_FFFF) as u32;
    let gen = (raw >> 32) as u32;
    forge_topo::handles::FaceId::from_raw_parts(idx, gen)
}

/// Transient geometric match key for cross-solid vertex deduplication.
///
/// Keyed by the exact rational `[x, y, z]` position of the vertex.
/// Because `intersect_three_planes_exact` always reduces its result
/// to canonical form (GCD-reduced numerator/denominator), two vertices
/// at the same physical point produce bitwise-identical `Rational` values
/// and therefore identical `VertexMatchKey`s — regardless of which solid
/// they came from, which PlaneTable they used, or how many planes meet
/// at that corner.
///
/// This is NOT the vertex's permanent identity — that remains the
/// `VertexId` and its `Lineage`. This key is used strictly during
/// the boolean assembly phase. When a match is found, lineages are
/// merged (D1 compliance) rather than silently dropped.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VertexMatchKey {
    /// Exact rational coordinates.
    pos: [Rational; 3],
}

impl VertexMatchKey {
    /// Create a match key from an exact rational vertex position.
    ///
    /// `BigRational` is always stored in canonical (GCD-reduced) form,
    /// so two keys are equal iff they represent the same point.
    pub fn from_exact_position(x: Rational, y: Rational, z: Rational) -> Self {
        Self { pos: [x, y, z] }
    }

    /// Access the exact rational coordinates.
    pub fn position(&self) -> &[Rational; 3] {
        &self.pos
    }
}

/// Check if two planes have parallel normals (same or opposite direction).
///
/// Delegates to `forge_geom::primitives::plane::are_parallel_exact`, which
/// uses exact rational cross product. No tolerance, no magic numbers — D3 compliant.
pub fn planes_are_parallel(a: &Plane, b: &Plane) -> bool {
    forge_geom::primitives::plane::are_parallel_exact(a, b)
}

/// Compute the centroid of a face.
///
/// Used for heuristic classification (e.g. containment check).
pub fn compute_face_centroid(
    arena: &forge_topo::arena::TopologyArena,
    geom: &crate::geometry_store::GeometryStore,
    face: forge_topo::handles::FaceId,
) -> Result<[f64; 3], forge_core::KernelError> {
    let mut vertices = Vec::new();
    let loops = forge_topo::polygon::face_loop_vertices(arena, face)?;
    if let Some(outer_loop) = loops.first() {
        for vertex in outer_loop {
            if let Some(pos) = geom.get_vertex_position(*vertex) {
                vertices.push(*pos);
            }
        }
    }
    forge_geom::primitives::polygon::compute_polygon_centroid(&vertices).ok_or_else(|| {
        forge_core::KernelError::InvalidInput {
            message: format!("Face {:?} has degenerate geometry (no vertices/area)", face),
            context: None,
        }
    })
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
    target_arena: &forge_topo::arena::TopologyArena,
    target_geom: &crate::geometry_store::GeometryStore,
    tool_arena: &forge_topo::arena::TopologyArena,
    tool_geom: &crate::geometry_store::GeometryStore,
) -> CoincidenceGraph {
    let mut graph = CoincidenceGraph::new();

    // ── 1. Collect per-face (packed u64, Aabb) pairs ──────────────────────────
    let target_items: Vec<(u64, Aabb)> = forge_topo::bounds::all_face_bounds(
        target_arena,
        &|vid| target_geom.get_vertex_position(vid).copied(),
    )
        .unwrap_or_default()
        .into_iter()
        .map(|(fid, aabb)| (pack_face_id(fid, false), aabb))
        .collect();

    let tool_items: Vec<(u64, Aabb)> = forge_topo::bounds::all_face_bounds(
        tool_arena,
        &|vid| tool_geom.get_vertex_position(vid).copied(),
    )
        .unwrap_or_default()
        .into_iter()
        .map(|(fid, aabb)| (pack_face_id(fid, true), aabb))
        .collect();

    // ── 2. Build BVH trees ────────────────────────────────────────────────────
    let target_bvh = match BvhNode::build(target_items) {
        Some(tree) => tree,
        None => return graph,
    };
    let tool_bvh = match BvhNode::build(tool_items) {
        Some(tree) => tree,
        None => return graph,
    };

    // ── 3. O(n log n) AABB-overlap candidates ─────────────────────────────────
    let candidates = query_overlapping_pairs(&target_bvh, &tool_bvh);

    // ── 4. Exact coplanarity confirmation ─────────────────────────────────────
    for (target_raw, tool_raw) in candidates {
        let target_fid = unpack_face_id(target_raw);
        let tool_fid   = unpack_face_id(tool_raw);

        let Some(plane_a) = target_geom.get_face_plane(target_fid) else { continue };
        let Some(plane_b) = tool_geom.get_face_plane(tool_fid)   else { continue };

        if !forge_geom::primitives::plane::coplanar_eq(plane_a, plane_b) {
            continue;
        }

        // Approximate gap for metadata: for exactly-coplanar opposite-normal faces
        // this will be near zero; non-zero if planes are parallel but offset.
        let gap_mm = plane_a.offset().abs();

        graph.insert_edge(
            target_raw,
            tool_raw,
            CoincidenceKind::CoplanarFaces { gap_mm },
        );
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh_builder::make_cube;

    #[test]
    fn prepass_sharing_face_cubes() {
        let res_a = make_cube([0.0, 0.0, 0.0], 10.0).unwrap();
        let (topo_a, geom_a) = res_a.into_parts();
        
        let res_b = make_cube([10.0, 0.0, 0.0], 10.0).unwrap(); // exactly adjacent face
        let (topo_b, geom_b) = res_b.into_parts();

        let graph = build_face_coincidence_prepass(
            topo_a.arena(), &geom_a,
            topo_b.arena(), &geom_b,
        );

        // A cube touching another cube face-to-face will have exactly 1 coplanar face pair.
        assert_eq!(graph.edge_count(), 1, "Expected exactly 1 coplanar face edge");
    }

    #[test]
    fn prepass_non_adjacent_cubes() {
        let res_a = make_cube([0.0, 0.0, 0.0], 10.0).unwrap();
        let (topo_a, geom_a) = res_a.into_parts();
        
        let res_b = make_cube([25.0, 0.0, 0.0], 10.0).unwrap(); // far apart
        let (topo_b, geom_b) = res_b.into_parts();

        let graph = build_face_coincidence_prepass(
            topo_a.arena(), &geom_a,
            topo_b.arena(), &geom_b,
        );

        // No bounding box overlaps, should be empty graph.
        assert_eq!(graph.edge_count(), 0, "Expected empty graph for non-adjacent cubes");
    }
}

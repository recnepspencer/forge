//! Boolean-specific coplanar face utilities.
//!
//! DOMAIN: Detect and exclude coplanar face pairs between two solids for
//! regularized Boolean union/difference (internal boundary elimination).
//!
//! DEPENDENCIES: shared_ops::coincidence (BVH prepass), geom_facade (exact coplanarity)
//!
//! INVARIANTS:
//! - Uses `build_face_coincidence_prepass` for O(n log n) candidate selection.
//! - Final coplanarity confirmation uses exact rational arithmetic via `geom_facade`.
//! - No floating-point tolerance thresholds or `if dist < eps` guards.

use std::collections::BTreeSet;

use forge_topo::handles::FaceId;
use forge_topo::state::TopologyState;

use crate::geometry_state::GeometryState;
use crate::shared_ops::coincidence::build_face_coincidence_prepass;
use crate::geom_facade::CoincidenceKind;

/// True when this face was created by a `make_edge_face` Euler operator.
///
/// The `make_edge_face` operator produces intersection-derived faces during
/// the Boolean split phase. Such faces sit precisely on the boundary of the
/// other solid, so a single centroid sample is likely to land exactly on
/// the boundary. Multi-sampling is required to resolve the ambiguity.
///
/// This predicate is private to the Boolean pipeline — no other feature
/// has a concept of intersection-derived faces.
pub(crate) fn is_intersection_face(arena: &forge_topo::arena::TopologyArena, face_id: FaceId) -> bool {
    let Some(face) = arena.get_face(face_id).ok() else {
        return false;
    };
    let Some(lineage) = face.lineage() else {
        return false;
    };
    lineage
        .get_creation_op()
        .get_name()
        .starts_with("make_edge_face")
}

/// Bit 63 of the packed u64 is set on tool-side face handles by `build_face_coincidence_prepass`.
const SIDE_TAG_BIT: u64 = 1u64 << 63;

/// Strip the side-tag bit and extract the raw slot index from a packed face handle.
#[inline]
fn extract_slot_index(packed: u64) -> u32 {
    (packed & !SIDE_TAG_BIT & 0xFFFF_FFFF) as u32
}

/// Find coplanar face pairs between two solids for regularized Boolean union.
///
/// Returns `(excluded_target_indices, excluded_tool_indices)` — the face
/// slot indices of coincident pairs that should be eliminated as internal boundaries.
///
/// # Algorithm
/// 1. `build_face_coincidence_prepass` runs BVH-accelerated AABB pre-filtering
///    and exact `coplanar_eq` confirmation in O(n log n).
/// 2. For each confirmed `CoplanarFaces` edge in the graph, both slot indices
///    are added to their respective exclusion sets.
///
/// The `CoincidenceGraph` keys are canonical `(min(A,B), max(A,B))` with the
/// side-tag bit embedded. We strip it to get the raw arena slot index.
pub(crate) fn find_coplanar_face_pairs(
    target_topo: &TopologyState,
    target_geom: &GeometryState,
    tool_topo: &TopologyState,
    tool_geom: &GeometryState,
) -> (BTreeSet<u32>, BTreeSet<u32>) {
    let mut excluded_target: BTreeSet<u32> = BTreeSet::new();
    let mut excluded_tool: BTreeSet<u32> = BTreeSet::new();

    let graph = build_face_coincidence_prepass(
        target_topo.arena(),
        target_geom,
        tool_topo.arena(),
        tool_geom,
    );

    for ((raw_a, raw_b), kind) in graph.iter() {
        if !matches!(kind, CoincidenceKind::CoplanarFaces { .. }) {
            continue;
        }
        // The side-tag bit distinguishes target (bit=0) from tool (bit=1).
        let tag_a = raw_a & SIDE_TAG_BIT != 0;
        let tag_b = raw_b & SIDE_TAG_BIT != 0;

        let (target_raw, tool_raw) = if !tag_a && tag_b {
            (*raw_a, *raw_b)
        } else {
            // Fallback for unexpected same-side pairs — record both indices.
            (*raw_a, *raw_b)
        };
        let _ = tag_a;
        let _ = tag_b;

        excluded_target.insert(extract_slot_index(target_raw));
        excluded_tool.insert(extract_slot_index(tool_raw));
    }

    (excluded_target, excluded_tool)
}

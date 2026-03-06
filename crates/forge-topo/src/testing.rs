//! Shared unit test utilities for `forge-topo`.
//!
//! Provides pre-built topological configurations for operator and validator
//! tests. All helpers use Euler operators exclusively to ensure every entity
//! carries a valid Shell/Region/Lump/Body hierarchy and correct wiring.

use crate::b_rep::{EdgeData, FaceData, HalfEdgeData, VertexData};
use crate::boundary_editing::make_face_from_vertices::MakeFaceFromVertices;
use crate::boundary_editing::make_loop_in_face_from_vertices::MakeLoopInFaceFromVertices;
use crate::handles::{EdgeId, FaceId, HalfEdgeId, LoopId, ShellId, VertexId};
use crate::transactions::MutableDraft;

// ── Raw arena helpers (used by b_rep/tests.rs, NOT for operator tests) ──

/// Returns a VertexData with a sentinel primary-disk halfedge.
pub fn dummy_vertex_data() -> VertexData {
    VertexData::new(HalfEdgeId::DANGLING)
}

/// Returns a FaceData with a sentinel outer loop and shell.
pub fn dummy_face_data() -> FaceData {
    FaceData::new(LoopId::DANGLING, ShellId::DANGLING)
}

/// Returns a HalfEdgeData pointing to the given face and origin,
/// with sentinels for all other links.
pub fn dummy_halfedge_data(face: FaceId, origin: VertexId) -> HalfEdgeData {
    HalfEdgeData::new(
        HalfEdgeId::DANGLING,
        HalfEdgeId::DANGLING,
        HalfEdgeId::DANGLING,
        face,
        origin,
        EdgeId::DANGLING,
    )
}

/// Build a face with an outer triangle (v0→v1→v2) and an inner triangle
/// hole (v3→v4→v5).
///
/// All entities are created through Euler operators (`MakeFaceFromVertices`
/// and `MakeLoopInFaceFromVertices`), ensuring:
/// - Full Shell/Region/Lump/Body hierarchy (no DANGLING sentinels)
/// - Correct next/prev/radial wiring
/// - Proper NMT vertex disk registration (if vertices are shared)
/// - Valid edge entities for every half-edge pair
///
/// Returns: `(face_id, outer_he_01, inner_he_34, outer_loop_id, [v0..v5])`.
pub fn build_face_with_hole(
    draft: &mut MutableDraft,
) -> (FaceId, HalfEdgeId, HalfEdgeId, LoopId, [VertexId; 6]) {
    // ── Create 6 isolated vertices ──────────────────────────────────
    let sentinel_he = HalfEdgeId::DANGLING;
    let v0 = draft.insert_vertex(VertexData::new(sentinel_he));
    let v1 = draft.insert_vertex(VertexData::new(sentinel_he));
    let v2 = draft.insert_vertex(VertexData::new(sentinel_he));
    let v3 = draft.insert_vertex(VertexData::new(sentinel_he));
    let v4 = draft.insert_vertex(VertexData::new(sentinel_he));
    let v5 = draft.insert_vertex(VertexData::new(sentinel_he));

    // ── Build outer triangle face via Euler operator ────────────────
    let mffv = draft
        .execute(MakeFaceFromVertices {
            vertices: vec![v0, v1, v2],
        })
        .unwrap()
        .into_value();

    let face = mffv.face;
    let outer_loop = mffv.loop_id;
    let outer_he = mffv.half_edges[0]; // he: v0→v1

    // ── Build inner hole loop via Euler operator ────────────────────
    let mlifv = draft
        .execute(MakeLoopInFaceFromVertices {
            face,
            vertices: vec![v3, v4, v5],
        })
        .unwrap()
        .into_value();

    let inner_he = mlifv.half_edges[0]; // he: v3→v4

    (
        face,
        outer_he,
        inner_he,
        outer_loop,
        [v0, v1, v2, v3, v4, v5],
    )
}

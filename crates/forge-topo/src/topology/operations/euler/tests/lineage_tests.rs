//! Tests for lineage and provenance correctness across Euler operators.
//!
//! DOMAIN: These tests verify D1 determinism — that identical operation
//! sequences always produce identical ancestry hashes, and that child
//! entities correctly inherit from their parent lineage chains.

use crate::euler::make_edge_face::MakeEdgeFace;
use crate::euler::make_vertex_face::MakeVertexFace;
use crate::euler::split_edge::SplitEdge;
use crate::operator::apply_op;
use crate::state::TopologyState;

/// KV-16: Same operation sequence on fresh drafts produces identical ancestry hashes.
///
/// Ancestry hashes must be deterministic. Because invocation_ids start at 0
/// on every fresh draft, the same sequence always computes the same hash.
#[test]
fn kv16_identical_sequence_produces_identical_lineage() {
    let run_sequence = || {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let mef = apply_op(
            &mut draft,
            MakeEdgeFace {
                vertex_a: mvf.vertex,
                vertex_b: se.new_vertex,
                face: mvf.face,
            },
        )
        .unwrap()
        .into_value();

        let vertex_hash = draft
            .arena()
            .get_vertex(mvf.vertex)
            .unwrap()
            .lineage()
            .unwrap()
            .get_ancestry_hash();
        let face_hash = draft
            .arena()
            .get_face(mef.new_face)
            .unwrap()
            .lineage()
            .unwrap()
            .get_ancestry_hash();
        let he_hash = draft
            .arena()
            .get_half_edge(mef.half_edge_ab)
            .unwrap()
            .lineage()
            .unwrap()
            .get_ancestry_hash();

        (vertex_hash, face_hash, he_hash)
    };

    let (v1, f1, h1) = run_sequence();
    let (v2, f2, h2) = run_sequence();

    assert_eq!(v1, v2);
    assert_eq!(f1, f2);
    assert_eq!(h1, h2);
}

/// KV-17: SplitEdge children derive a new ancestry hash from the parent.
///
/// The child's hash must differ from the parent's (a different op was applied),
/// but the new vertex and the new halfedge must share the same hash (they were
/// created in the same SplitEdge invocation).
#[test]
fn kv17_split_edge_children_carry_parent_ancestry() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

    let parent_hash = draft
        .arena()
        .get_half_edge(mvf.half_edge)
        .unwrap()
        .lineage()
        .unwrap()
        .get_ancestry_hash();

    let se = apply_op(
        &mut draft,
        SplitEdge {
            edge: mvf.half_edge,
            parameter: 0.5,
        },
    )
    .unwrap()
    .into_value();

    let child_vertex_lineage = draft
        .arena()
        .get_vertex(se.new_vertex)
        .unwrap()
        .lineage()
        .unwrap();
    let child_he_lineage = draft
        .arena()
        .get_half_edge(se.he_mb)
        .unwrap()
        .lineage()
        .unwrap();

    assert_ne!(
        child_vertex_lineage.get_ancestry_hash(),
        parent_hash,
        "children must not simply clone the parent hash"
    );
    assert_eq!(
        child_vertex_lineage.get_ancestry_hash(),
        child_he_lineage.get_ancestry_hash(),
        "vertex and halfedge produced by the same split must share a hash"
    );
    assert_eq!(
        child_vertex_lineage.get_creation_op().get_name(),
        "split_edge"
    );
}

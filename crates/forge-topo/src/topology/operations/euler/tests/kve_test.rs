//! Tests for KillVertexEdge (KVE): edge merge by vertex removal.
//!
//! DOMAIN: KVE is the inverse of SplitEdge (MVE). It merges two edges
//! by removing their shared 2-valent vertex.
//!
//! NOTE: KVE currently returns an error because the full radial merge
//! logic requires non-manifold policy definition. These tests validate
//! the operator is registered and rejects gracefully.

use crate::euler::kill_vertex_edge::KillVertexEdge;
use crate::euler::make_vertex_face::MakeVertexFace;
use crate::euler::split_edge::SplitEdge;
use crate::operator::apply_op;
use crate::state::TopologyState;

/// KVE returns an error for now (pending radial merge policy).
///
/// This test verifies the operator is registered and callable through apply_op.
#[test]
fn kve_is_registered_and_callable() {
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

    let result = apply_op(
        &mut draft,
        KillVertexEdge {
            vertex: se.new_vertex,
        },
    );

    assert!(
        result.is_err(),
        "KVE should return an error until radial merge policy is defined"
    );
}

/// KVE rejects a vertex with no outgoing halfedge.
#[test]
fn kve_rejects_isolated_vertex() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let v = draft.insert_vertex(crate::arena::VertexData::new(
        crate::handles::HalfEdgeId::new(u32::MAX, 0),
    ));

    let result = apply_op(&mut draft, KillVertexEdge { vertex: v });

    assert!(
        result.is_err(),
        "KVE must reject a vertex with no outgoing halfedge"
    );
}

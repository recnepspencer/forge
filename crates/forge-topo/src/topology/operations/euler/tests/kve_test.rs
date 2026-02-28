//! Tests for KillVertexEdge (KVE): edge merge by vertex removal.
//!
//! DOMAIN: KVE is the inverse of SplitEdge (MVE). It merges two edges
//! by removing their shared 2-valent vertex.

use crate::euler::kill_vertex_edge::KillVertexEdge;
use crate::euler::make_vertex_face::MakeVertexFace;
use crate::euler::split_edge::SplitEdge;
use crate::operator::apply_op;
use crate::state::TopologyState;

/// SplitEdge then KVE restores the original entity counts.
#[test]
fn split_then_kve_restores_original() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

    let v_before = draft.arena().vertex_count();
    let he_before = draft.arena().half_edge_count();
    let e_before = draft.arena().edge_count();

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

    assert!(result.is_ok(), "KVE should succeed after SplitEdge: {:?}", result.err());

    assert_eq!(draft.arena().vertex_count(), v_before);
    assert_eq!(draft.arena().half_edge_count(), he_before);
    assert_eq!(draft.arena().edge_count(), e_before);
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

/// KVE rejects a vertex that connects more than 2 edges.
#[test]
fn kve_rejects_non_2_valent_vertex() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

    // The MFV vertex connects only 1 edge (self-loop), should be rejected
    let result = apply_op(
        &mut draft,
        KillVertexEdge {
            vertex: mvf.vertex,
        },
    );

    assert!(
        result.is_err(),
        "KVE must reject a vertex that is not 2-valent in edges"
    );
}

/// Double split then double KVE restores original counts.
#[test]
fn double_split_double_kve_restores() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

    let v_before = draft.arena().vertex_count();
    let he_before = draft.arena().half_edge_count();
    let e_before = draft.arena().edge_count();

    let se1 = apply_op(
        &mut draft,
        SplitEdge {
            edge: mvf.half_edge,
            parameter: 0.3,
        },
    )
    .unwrap()
    .into_value();

    let se2 = apply_op(
        &mut draft,
        SplitEdge {
            edge: se1.he_mb,
            parameter: 0.5,
        },
    )
    .unwrap()
    .into_value();

    // Kill in reverse order
    apply_op(
        &mut draft,
        KillVertexEdge {
            vertex: se2.new_vertex,
        },
    )
    .unwrap()
    .into_value();

    apply_op(
        &mut draft,
        KillVertexEdge {
            vertex: se1.new_vertex,
        },
    )
    .unwrap()
    .into_value();

    assert_eq!(draft.arena().vertex_count(), v_before);
    assert_eq!(draft.arena().half_edge_count(), he_before);
    assert_eq!(draft.arena().edge_count(), e_before);
}

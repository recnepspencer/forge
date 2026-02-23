//! Tests for KillEdgeVertex (KEV): vertex collapse (inverse of SplitEdge).
//!
//! DOMAIN: KEV removes the vertex introduced by SplitEdge, collapsing
//! two halfedges back into one.

use crate::state::TopologyState;
use crate::operator::apply_op;
use crate::euler::make_vertex_face::MakeVertexFace;
use crate::euler::make_edge_face::MakeEdgeFace;
use crate::euler::split_edge::SplitEdge;
use crate::euler::kill_edge_vertex::KillEdgeVertex;

/// KEV collapses the vertex added by SplitEdge, restoring the original count.
#[test]
fn kill_edge_vertex_collapses_vertex() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 })
        .unwrap().into_value();
    let mef = apply_op(&mut draft, MakeEdgeFace {
        vertex_a: mvf.vertex,
        vertex_b: se1.new_vertex,
        face: mvf.face,
    }).unwrap().into_value();
    let se2 = apply_op(&mut draft, SplitEdge { edge: mef.half_edge_ab, parameter: 0.5 })
        .unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 3);

    apply_op(&mut draft, KillEdgeVertex { edge: se2.he_am }).unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 2);
    assert_eq!(draft.arena().half_edge_count(), 4);
}

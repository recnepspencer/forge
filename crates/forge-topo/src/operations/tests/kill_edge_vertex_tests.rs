//! Tests for KillEdgeVertex (KEV): vertex collapse (inverse of SplitEdge).
//!
//! DOMAIN: KEV removes the vertex introduced by SplitEdge, collapsing
//! two halfedges back into one.

use crate::entity_lifecycle::kill_edge_vertex::KillEdgeVertex;
use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
use crate::entity_lifecycle::split_edge::SplitEdge;
use crate::transactions::TopologyState;
use crate::b_rep::ShellKind;

/// KEV collapses the vertex added by SplitEdge, restoring the original count.
#[test]
fn kill_edge_vertex_collapses_vertex() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft.execute(MakeVertexFace { shell_kind: ShellKind::Sheet }).unwrap().into_value();
    let se1 = draft.execute(
        SplitEdge {
            edge: mvf.half_edge,
        },
    )
    .unwrap()
    .into_value();
    let mef = draft.execute(
        MakeEdgeFace {
            vertex_a: mvf.vertex,
            vertex_b: se1.new_vertex,
            face: mvf.face,
        },
    )
    .unwrap()
    .into_value();
    let se2 = draft.execute(
        SplitEdge {
            edge: mef.half_edge_ab,
        },
    )
    .unwrap()
    .into_value();

    assert_eq!(draft.arena().vertex_count(), 3);

    draft.execute(KillEdgeVertex { edge: se2.he_am })
        .unwrap()
        .into_value();

    assert_eq!(draft.arena().vertex_count(), 2);
    assert_eq!(draft.arena().half_edge_count(), 4);
}

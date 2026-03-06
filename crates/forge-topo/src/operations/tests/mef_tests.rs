//! Tests for MakeEdgeFace (MEF): face splitting via a new edge.
//!
//! DOMAIN: MEF is the primary face-building operator. It creates one new
//! face and one new directed edge between two existing vertices.

use crate::b_rep::ShellKind;
use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
use crate::entity_lifecycle::split_edge::SplitEdge;
use crate::transactions::TopologyState;
use crate::traverse::edge_faces;

/// MEF produces exactly 2 faces, 2 loops, and 4 halfedges.
///
/// The two faces produced by MEF must be distinct (they bound different
/// regions of the topology).
#[test]
fn mef_splits_face_creating_two_faces() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let se = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();

    let mef = draft
        .execute(MakeEdgeFace {
            vertex_a: mvf.vertex,
            vertex_b: se.new_vertex,
            face: mvf.face,
        })
        .unwrap()
        .into_value();

    assert_eq!(draft.arena().face_count(), 2);
    assert_eq!(draft.arena().loop_count(), 2);
    assert_eq!(draft.arena().half_edge_count(), 4);

    let faces = edge_faces(draft.arena(), mef.half_edge_ab).unwrap();
    assert_eq!(faces.len(), 2);
    let (f1, f2) = (faces[0], faces[1]);
    assert_ne!(f1, f2, "the shared edge must separate two distinct faces");
}

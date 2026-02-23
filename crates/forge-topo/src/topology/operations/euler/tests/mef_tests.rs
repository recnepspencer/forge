//! Tests for MakeEdgeFace (MEF): face splitting via a new edge.
//!
//! DOMAIN: MEF is the primary face-building operator. It creates one new
//! face and one new directed edge between two existing vertices.

use crate::state::TopologyState;
use crate::operator::apply_op;
use crate::euler::make_vertex_face::MakeVertexFace;
use crate::euler::make_edge_face::MakeEdgeFace;
use crate::euler::split_edge::SplitEdge;
use crate::traverse::edge_faces;

/// MEF produces exactly 2 faces, 2 loops, and 4 halfedges.
///
/// The two faces produced by MEF must be distinct (they bound different
/// regions of the topology).
#[test]
fn mef_splits_face_creating_two_faces() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
    let se  = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 })
        .unwrap().into_value();

    let mef = apply_op(&mut draft, MakeEdgeFace {
        vertex_a: mvf.vertex,
        vertex_b: se.new_vertex,
        face: mvf.face,
    }).unwrap().into_value();

    assert_eq!(draft.arena().face_count(), 2);
    assert_eq!(draft.arena().loop_count(), 2);
    assert_eq!(draft.arena().half_edge_count(), 4);

    let (f1, f2) = edge_faces(draft.arena(), mef.half_edge_ab).unwrap();
    assert_ne!(f1, f2, "the shared edge must separate two distinct faces");
}

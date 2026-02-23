//! Tests for SplitEdge: edge subdivision.
//!
//! DOMAIN: SplitEdge inserts a new vertex at a parametric position on an
//! existing edge, replacing 1 edge with 2. Two cases: degenerate seed split
//! and normal (two-vertex) edge split.

use crate::state::TopologyState;
use crate::operator::apply_op;
use crate::euler::make_vertex_face::MakeVertexFace;
use crate::euler::make_edge_face::MakeEdgeFace;
use crate::euler::split_edge::SplitEdge;

use super::helpers::logged_op;

/// Splitting the degenerate seed (self-twin) produces two properly-twinned halfedges.
///
/// Origin and twin pointers must be symmetric after split.
#[test]
fn split_degenerate_creates_proper_edge() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = logged_op("MVF",       apply_op(&mut draft, MakeVertexFace)).unwrap();
    let se  = logged_op("SplitEdge", apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 })).unwrap();

    assert_eq!(draft.arena().vertex_count(), 2);
    assert_eq!(draft.arena().half_edge_count(), 2);

    let he_am = draft.arena().get_half_edge(se.he_am).unwrap();
    let he_ma = draft.arena().get_half_edge(se.he_mb).unwrap();

    assert_eq!(he_am.origin(), mvf.vertex);
    assert_eq!(he_ma.origin(), se.new_vertex);
    assert_eq!(he_am.twin(), se.he_mb);
    assert_eq!(he_ma.twin(), se.he_am);
    assert_eq!(he_am.next(), se.he_mb);
    assert_eq!(he_ma.next(), se.he_am);
}

/// Splitting a real (two-vertex) edge adds one vertex and two halfedges.
///
/// After split: 3 vertices, 6 halfedges (was 4 from MVF+SE+MEF).
/// The origin and midpoint vertices must be correctly assigned.
#[test]
fn split_normal_edge_adds_vertex() {
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

    assert_eq!(draft.arena().half_edge_count(), 4);

    let se2 = apply_op(&mut draft, SplitEdge { edge: mef.half_edge_ab, parameter: 0.5 })
        .unwrap().into_value();

    assert_eq!(draft.arena().vertex_count(), 3);
    assert_eq!(draft.arena().half_edge_count(), 6);

    let he_am = draft.arena().get_half_edge(se2.he_am).unwrap();
    let he_mb = draft.arena().get_half_edge(se2.he_mb).unwrap();
    assert_eq!(he_am.origin(), mvf.vertex);
    assert_eq!(he_mb.origin(), se2.new_vertex);
}

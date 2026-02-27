//! Tests for MakeVertexFace (MVF): seed topology creation.
//!
//! DOMAIN: MVF is the atomic origin — every halfedge mesh starts here.
//! These tests verify that the seed state is fully wired and stamped.

use crate::euler::make_vertex_face::MakeVertexFace;
use crate::operator::apply_op;
use crate::state::TopologyState;

use super::helpers::logged_op;

/// MVF creates exactly 1 face, 1 vertex, 1 halfedge, 1 loop.
///
/// The seed halfedge must be a self-twin, self-next, self-prev — the
/// canonical representation of a "zero-area" topological seed.
#[test]
fn mvf_creates_single_vertex_and_face() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let out = logged_op("MVF", apply_op(&mut draft, MakeVertexFace)).unwrap();

    assert_eq!(draft.arena().vertex_count(), 1);
    assert_eq!(draft.arena().face_count(), 1);
    assert_eq!(draft.arena().half_edge_count(), 1);
    assert_eq!(draft.arena().loop_count(), 1);

    let he = draft.arena().get_half_edge(out.half_edge).unwrap();
    assert_eq!(he.radial_next(), out.half_edge, "seed must be self-twin");
    assert_eq!(he.next(), out.half_edge, "seed must be self-next");
    assert_eq!(he.prev(), out.half_edge, "seed must be self-prev");
    assert_eq!(he.origin(), out.vertex);
    assert_eq!(he.face(), out.face);

    let committed = draft.commit().unwrap();
    assert_eq!(committed.epoch(), 1);
}

/// MVF stamps the creation operation name on every entity's lineage.
///
/// All three entities (vertex, face, halfedge) must share the same
/// ancestry_hash because they were produced by the same single operation.
#[test]
fn mvf_stamps_lineage_on_all_entities() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let out = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();

    let v_lineage = draft
        .arena()
        .get_vertex(out.vertex)
        .unwrap()
        .lineage()
        .unwrap();
    let f_lineage = draft.arena().get_face(out.face).unwrap().lineage().unwrap();
    let he_lineage = draft
        .arena()
        .get_half_edge(out.half_edge)
        .unwrap()
        .lineage()
        .unwrap();

    assert_eq!(v_lineage.get_creation_op().get_name(), "make_vertex_face");
    assert_eq!(f_lineage.get_creation_op().get_name(), "make_vertex_face");
    assert_eq!(he_lineage.get_creation_op().get_name(), "make_vertex_face");

    assert_eq!(v_lineage.get_ancestry_hash(), f_lineage.get_ancestry_hash());
    assert_eq!(
        f_lineage.get_ancestry_hash(),
        he_lineage.get_ancestry_hash()
    );
}

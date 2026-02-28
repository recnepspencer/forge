//! Tests for MakeVertexFace (MVF): seed topology creation.
//!
//! DOMAIN: MVF is the atomic origin — every halfedge mesh starts here.
//! These tests verify that the seed state is fully wired and stamped.

use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
use crate::transactions::TopologyState;

use super::helpers::logged_op;

/// MVF creates exactly 1 face, 1 vertex, 1 halfedge, 1 loop.
///
/// The seed halfedge must be a self-twin, self-next, self-prev — the
/// canonical representation of a "zero-area" topological seed.
#[test]
fn mvf_creates_single_vertex_and_face() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let out = logged_op("MVF", draft.execute(MakeVertexFace)).unwrap();

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

// TODO(Phase 3): Re-enable once LineageStore lookup is wired.
// /// MVF stamps the creation operation name on every entity's lineage.
// #[test]
// fn mvf_stamps_lineage_on_all_entities() { ... }

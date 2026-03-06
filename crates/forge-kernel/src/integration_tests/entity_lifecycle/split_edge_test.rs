//! Integration tests: SplitEdge on real cube geometry.
//!
//! DOMAIN: Splits edges of a BSP-generated cube and verifies
//! structural invariants, halfedge rewiring, and entity counts.

use crate::integration_tests::harness::shapes::{
    collect_face_loop, first_halfedge_of_face, unit_cube,
};
use forge_topo::entity_lifecycle::split_edge::SplitEdge;

/// Split one edge of a cube → cube gains 1 vertex, 2 halfedges, 1 edge.
/// Two adjacent faces go from 4-sided to 5-sided.
#[test]
fn split_single_cube_edge() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) =
        env_res.into_value().into_draft();

    let face = faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let result = draft
        .execute(SplitEdge { edge: start_he })
        .unwrap()
        .into_value();

    assert_eq!(draft.arena().face_count(), 6);
    assert_eq!(draft.arena().vertex_count(), 9);
    assert_eq!(draft.arena().half_edge_count(), 26);
    assert_eq!(draft.arena().edge_count(), 13);
    assert_eq!(draft.arena().loop_count(), 6);
    assert_eq!(draft.arena().shell_count(), 1);
    assert_eq!(draft.arena().body_count(), 1);

    let new_v = result.new_vertex;
    let v_hes = draft.arena().halfedges_from_vertex(new_v);
    assert!(!v_hes.is_empty(), "New vertex has no incident halfedges");
}

/// Split every edge of one face → that face becomes an 8-gon.
/// A cube face has 4 edges; splitting each adds 1 vertex and 1 edge per split.
#[test]
fn split_all_edges_of_one_face() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) =
        env_res.into_value().into_draft();

    let face = faces[0];

    let loop_hes = {
        let start = first_halfedge_of_face(draft.arena(), face).unwrap();
        collect_face_loop(draft.arena(), start).unwrap()
    };

    assert_eq!(loop_hes.len(), 4, "Cube face should have 4 halfedges");

    for he in &loop_hes {
        draft.execute(SplitEdge { edge: *he }).unwrap().into_value();
    }

    assert_eq!(draft.arena().face_count(), 6);
    assert_eq!(draft.arena().vertex_count(), 12);
    assert_eq!(draft.arena().half_edge_count(), 32);
    assert_eq!(draft.arena().edge_count(), 16);
    assert_eq!(draft.arena().loop_count(), 6);
    assert_eq!(draft.arena().shell_count(), 1);
    assert_eq!(draft.arena().body_count(), 1);

    let face_hes = collect_face_loop(
        draft.arena(),
        first_halfedge_of_face(draft.arena(), face).unwrap(),
    )
    .unwrap();
    assert_eq!(face_hes.len(), 8, "Face should be an 8-gon after 4 splits");

    let _committed = draft.commit().unwrap();
}

/// Split the same edge twice → creates two midpoints.
/// Tests that the output handles from the first split are still valid.
#[test]
fn split_same_edge_twice() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) =
        env_res.into_value().into_draft();

    let face = faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let first_split = draft
        .execute(SplitEdge { edge: start_he })
        .unwrap()
        .into_value();

    let second_split = draft
        .execute(SplitEdge {
            edge: first_split.he_mb,
        })
        .unwrap()
        .into_value();

    assert_ne!(
        first_split.new_vertex, second_split.new_vertex,
        "Two splits must create distinct vertices"
    );

    assert_eq!(draft.arena().face_count(), 6);
    assert_eq!(draft.arena().vertex_count(), 10);
    assert_eq!(draft.arena().half_edge_count(), 28);
    assert_eq!(draft.arena().edge_count(), 14);
    assert_eq!(draft.arena().loop_count(), 6);
    assert_eq!(draft.arena().shell_count(), 1);
    assert_eq!(draft.arena().body_count(), 1);

    let _committed = draft.commit().unwrap();
}

/// Split every edge of the entire cube → 12 splits.
/// Tests massive rewiring with all 24 halfedges affected.
#[test]
fn split_all_cube_edges() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) =
        env_res.into_value().into_draft();

    let all_edges: Vec<_> = draft.arena().iter_edges().map(|(id, _)| id).collect();

    assert_eq!(all_edges.len(), 12, "Cube should have 12 edges");

    for edge_id in &all_edges {
        let he_id = draft.arena().get_edge(*edge_id).unwrap().half_edge();
        draft
            .execute(SplitEdge { edge: he_id })
            .unwrap()
            .into_value();
    }

    assert_eq!(draft.arena().face_count(), 6);
    assert_eq!(draft.arena().vertex_count(), 20);
    assert_eq!(draft.arena().half_edge_count(), 48);
    assert_eq!(draft.arena().edge_count(), 24);
    assert_eq!(draft.arena().loop_count(), 6);
    assert_eq!(draft.arena().shell_count(), 1);
    assert_eq!(draft.arena().body_count(), 1);

    let _committed = draft.commit().unwrap();
}

//! Integration tests: MakeEdgeFace (MEF) on real cube geometry.
//!
//! DOMAIN: Splits cube faces by inserting edges between existing vertices.
//! Verifies face creation, loop management, and adjacency.

use crate::integration_tests::harness::shapes::{
    collect_face_loop, first_halfedge_of_face, unit_cube,
};
use forge_topo::entity_lifecycle::make_edge_face::MakeEdgeFace;
use forge_topo::entity_lifecycle::split_edge::SplitEdge;

/// Split a quad face diagonally into two triangles.
///
/// Pick a cube face (4 vertices), find two non-adjacent vertices,
/// insert an edge between them. Result: 2 triangular faces.
#[test]
fn split_cube_face_into_two_triangles() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let face = faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();
    let loop_hes = collect_face_loop(draft.arena(), start_he).unwrap();
    assert_eq!(loop_hes.len(), 4, "Cube face must have 4 halfedges");

    let v_a = draft.arena().get_half_edge(loop_hes[0]).unwrap().origin();
    let v_c = draft.arena().get_half_edge(loop_hes[2]).unwrap().origin();

    let mef_result = draft.execute(MakeEdgeFace {
        face,
        vertex_a: v_a,
        vertex_b: v_c,
    }).unwrap().into_value();

    let face_valence = collect_face_loop(draft.arena(), first_halfedge_of_face(draft.arena(), face).unwrap()).unwrap().len();
    assert_eq!(face_valence, 3, "Original face should be a triangle");

    let new_face_valence = collect_face_loop(draft.arena(), first_halfedge_of_face(draft.arena(), mef_result.new_face).unwrap()).unwrap().len();
    assert_eq!(new_face_valence, 3, "New face should be a triangle");

    assert_eq!(draft.arena().face_count(), 7);
    assert_eq!(draft.arena().vertex_count(), 8);
    assert_eq!(draft.arena().half_edge_count(), 26);
    assert_eq!(draft.arena().edge_count(), 13);
    assert_eq!(draft.arena().loop_count(), 7);
    assert_eq!(draft.arena().shell_count(), 1);
    assert_eq!(draft.arena().body_count(), 1);

    let _committed = draft.commit().unwrap();
}

/// Split an edge first, then MEF through the new midpoint.
///
/// Creates a 5-sided face, then splits it into a triangle and a quad.
#[test]
fn split_edge_then_mef_through_midpoint() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let face = faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let se = draft.execute(SplitEdge {
        edge: start_he,
    }).unwrap().into_value();

    let face_valence = collect_face_loop(draft.arena(), first_halfedge_of_face(draft.arena(), face).unwrap()).unwrap().len();
    assert_eq!(face_valence, 5);

    let opposite_he = {
        let loop_hes = collect_face_loop(draft.arena(), start_he).unwrap();
        loop_hes[3]
    };
    let opposite_vertex = draft.arena().get_half_edge(opposite_he).unwrap().origin();

    let mef = draft.execute(MakeEdgeFace {
        face,
        vertex_a: se.new_vertex,
        vertex_b: opposite_vertex,
    }).unwrap().into_value();

    let face_a_valence = collect_face_loop(draft.arena(), first_halfedge_of_face(draft.arena(), face).unwrap()).unwrap().len();
    let face_b_valence = collect_face_loop(draft.arena(), first_halfedge_of_face(draft.arena(), mef.new_face).unwrap()).unwrap().len();

    assert!(
        (face_a_valence == 3 && face_b_valence == 4) ||
        (face_a_valence == 4 && face_b_valence == 3),
        "Expected a triangle and a quad, got {}-gon and {}-gon",
        face_a_valence, face_b_valence
    );

    let _committed = draft.commit().unwrap();
}

/// MEF two separate cube faces → verify both new faces and invariants hold.
///
/// Simpler than chaining on the same face: ensures independent MEF operations
/// don't interfere with each other.
#[test]
fn mef_on_two_different_cube_faces() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) = env_res.into_value().into_draft();

    let face_a = faces[0];
    let face_b = faces[1];

    let he_a = first_halfedge_of_face(draft.arena(), face_a).unwrap();
    let loop_a = collect_face_loop(draft.arena(), he_a).unwrap();
    let va_0 = draft.arena().get_half_edge(loop_a[0]).unwrap().origin();
    let va_2 = draft.arena().get_half_edge(loop_a[2]).unwrap().origin();

    draft.execute(MakeEdgeFace {
        face: face_a,
        vertex_a: va_0,
        vertex_b: va_2,
    }).unwrap().into_value();

    let he_b = first_halfedge_of_face(draft.arena(), face_b).unwrap();
    let loop_b = collect_face_loop(draft.arena(), he_b).unwrap();
    let vb_0 = draft.arena().get_half_edge(loop_b[0]).unwrap().origin();
    let vb_2 = draft.arena().get_half_edge(loop_b[2]).unwrap().origin();

    draft.execute(MakeEdgeFace {
        face: face_b,
        vertex_a: vb_0,
        vertex_b: vb_2,
    }).unwrap().into_value();

    assert_eq!(draft.arena().face_count(), 8);
    assert_eq!(draft.arena().vertex_count(), 8);
    assert_eq!(draft.arena().half_edge_count(), 28);
    assert_eq!(draft.arena().edge_count(), 14);
    assert_eq!(draft.arena().loop_count(), 8);
    assert_eq!(draft.arena().shell_count(), 1);
    assert_eq!(draft.arena().body_count(), 1);

    let _committed = draft.commit().unwrap();
}

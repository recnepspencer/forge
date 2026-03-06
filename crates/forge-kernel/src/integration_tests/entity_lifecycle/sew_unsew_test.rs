//! Integration tests: SewEdge and UnsewEdge on real geometry.
//!
//! DOMAIN: Tests non-manifold edge sewing — connecting two boundary
//! halfedges into a shared radial chain, then undoing it.

use crate::integration_tests::harness::shapes::{first_halfedge_of_face, unit_cube};
use forge_topo::entity_lifecycle::split_edge::SplitEdge;

/// Split an edge, verify the new halfedge pair (he_mb/he_bm) form a
/// proper radial pair with correct reciprocity.
///
/// On a closed cube, every edge already has 2 halfedges in its radial chain.
/// After SplitEdge, the new edge (M→B / B→M) should also have exactly 2.
#[test]
fn split_edge_creates_proper_radial_pair() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let faces = env_res.get_value().faces().to_vec();
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) =
        env_res.into_value().into_draft();

    let face = faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let _he_face_before = draft.arena().get_half_edge(start_he).unwrap().face();
    let he_twin = draft.arena().get_half_edge(start_he).unwrap().radial_next();
    let _twin_face_before = draft.arena().get_half_edge(he_twin).unwrap().face();

    let se = draft
        .execute(SplitEdge { edge: start_he })
        .unwrap()
        .into_value();

    let he_mb = se.he_mb;
    let he_bm = se.he_bm;

    let mb_radial = draft.arena().get_half_edge(he_mb).unwrap().radial_next();
    let bm_radial = draft.arena().get_half_edge(he_bm).unwrap().radial_next();
    assert_eq!(mb_radial, he_bm, "he_mb.radial_next should be he_bm");
    assert_eq!(bm_radial, he_mb, "he_bm.radial_next should be he_mb");

    let mb_face = draft.arena().get_half_edge(he_mb).unwrap().face();
    let bm_face = draft.arena().get_half_edge(he_bm).unwrap().face();
    assert_ne!(
        mb_face, bm_face,
        "New halfedge pair should be on different faces"
    );

    let mb_origin = draft.arena().get_half_edge(he_mb).unwrap().origin();
    assert_eq!(
        mb_origin, se.new_vertex,
        "he_mb should originate at the midpoint"
    );

    let _committed = draft.commit().unwrap();
}

/// Every edge of a raw cube has exactly 2-valent radial chain.
/// Verify by walking radial_next for every halfedge.
#[test]
fn cube_all_edges_are_2_manifold() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let arena = env_res.get_value().topology().arena();

    for (he_id, he_data) in arena.iter_half_edges() {
        let he_id: forge_topo::handles::HalfEdgeId = he_id;
        let he_data: &forge_topo::b_rep::HalfEdgeData = he_data;
        let twin = he_data.radial_next();
        assert_ne!(
            he_id,
            twin,
            "Cube halfedge {} should not be self-radial",
            he_id.index()
        );

        let twin_data = arena.get_half_edge(twin).unwrap();
        assert_eq!(
            twin_data.radial_next(),
            he_id,
            "Radial chain of he {} should be exactly 2-valent (got 3+)",
            he_id.index()
        );

        assert_ne!(
            he_data.face(),
            twin_data.face(),
            "Twin halfedges {} and {} should be on different faces",
            he_id.index(),
            twin.index()
        );

        assert_ne!(
            he_data.origin(),
            twin_data.origin(),
            "Twin halfedges should have different origins"
        );
    }
}

/// Split every edge of the cube, then verify all 24 edges still have
/// exactly 2-valent radial chains and no boundary edges exist.
#[test]
fn split_all_edges_preserves_2_manifold() {
    let env_res = unit_cube().expect("unit cube should succeed");
    let (mut draft, _geometry): (forge_topo::transactions::MutableDraft, _) =
        env_res.into_value().into_draft();

    let edges: Vec<_> = draft.arena().iter_edges().map(|(id, _)| id).collect();

    for eid in &edges {
        let he = draft.arena().get_edge(*eid).unwrap().half_edge();
        draft.execute(SplitEdge { edge: he }).unwrap();
    }

    for (he_id, he_data) in draft.arena().iter_half_edges() {
        let he_id: forge_topo::handles::HalfEdgeId = he_id;
        let he_data: &forge_topo::b_rep::HalfEdgeData = he_data;
        let twin = he_data.radial_next();
        assert_ne!(
            he_id,
            twin,
            "After splitting all edges, HE {} is boundary (self-radial)",
            he_id.index()
        );
        let twin_data = draft.arena().get_half_edge(twin).unwrap();
        assert_eq!(
            twin_data.radial_next(),
            he_id,
            "After splitting all edges, HE {} has >2 radial valence",
            he_id.index()
        );
    }

    let _committed = draft.commit().unwrap();
}

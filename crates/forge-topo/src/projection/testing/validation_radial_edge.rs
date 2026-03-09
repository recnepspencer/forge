use forge_spec::facade::{MakeVertexFaceMutation, SewEdgeMutation, SpecState, SplitEdgeMutation};

use crate::projection::facade::{
    validate_projected_no_broken_radial_splices, validate_projected_radial_edge,
    validate_projected_radial_edge_consistency, validate_projected_radial_rings,
    ProjectedTopologyQueries, ProjectionBuilder,
};

#[test]
fn projected_radial_edge_accepts_valid_splice() {
    let state = build_spec_sew_state();
    let projection = ProjectionBuilder::build(&state).unwrap();

    assert!(validate_projected_radial_edge(&projection).is_ok());
}

#[test]
fn projected_radial_edge_rejects_broken_splice() {
    let state = build_spec_sew_state();
    let mut projection = ProjectionBuilder::build(&state).unwrap();

    let edge = crate::projection::data::ProjectedEdgeId::new(0);
    let representative = projection.edge(edge).half_edge;
    let ring = projection.radial_half_edges(representative);
    assert!(ring.len() >= 2);
    projection.half_edges[ring[1].index()].edge = crate::projection::data::ProjectedEdgeId::new(1);

    let error = validate_projected_radial_edge(&projection).unwrap_err();
    assert!(
        format!("{error}").contains("projected_radial_edge_consistency")
            || format!("{error}").contains("projected_no_broken_radial_splices")
    );
}

#[test]
fn projected_radial_ring_closure_rejects_non_closing_cycle() {
    let state = build_spec_sew_state();
    let mut projection = ProjectionBuilder::build(&state).unwrap();

    let edge = crate::projection::data::ProjectedEdgeId::new(0);
    let representative = projection.edge(edge).half_edge;
    let ring = projection.radial_half_edges(representative);
    assert!(ring.len() >= 2);
    projection.half_edges[ring[1].index()].radial_next = ring[1];

    let error = validate_projected_radial_rings(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_radial_ring_closure"));
}

#[test]
fn projected_radial_edge_consistency_rejects_edge_mismatch_directly() {
    let state = build_spec_sew_state();
    let mut projection = ProjectionBuilder::build(&state).unwrap();

    let edge = crate::projection::data::ProjectedEdgeId::new(0);
    let representative = projection.edge(edge).half_edge;
    let ring = projection.radial_half_edges(representative);
    assert!(ring.len() >= 2);
    projection.half_edges[ring[1].index()].edge = crate::projection::data::ProjectedEdgeId::new(1);

    let error = validate_projected_radial_edge_consistency(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_radial_edge_consistency"));
}

#[test]
fn projected_broken_radial_splices_rejects_disjoint_edge_ownership() {
    let state = build_spec_sew_state();
    let mut projection = ProjectionBuilder::build(&state).unwrap();

    let edge = crate::projection::data::ProjectedEdgeId::new(0);
    let representative = projection.edge(edge).half_edge;
    let ring = projection.radial_half_edges(representative);
    assert!(ring.len() >= 2);
    projection.half_edges[ring[1].index()].radial_next = ring[1];

    let error = validate_projected_no_broken_radial_splices(&projection).unwrap_err();
    assert!(
        format!("{error}").contains("projected_no_broken_radial_splices")
            || format!("{error}").contains("projected_radial_ring_closure")
    );
}

fn build_spec_sew_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    let split = draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap()
        .value;
    draft
        .execute(SewEdgeMutation {
            half_edge_a: seed.half_edge,
            half_edge_b: split.he_mb,
        })
        .unwrap();
    draft.commit().unwrap()
}

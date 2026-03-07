use forge_spec::facade::{MakeVertexFaceMutation, SewEdgeMutation, SpecState, SplitEdgeMutation};

use crate::projection::facade::{
    ProjectedTopologyQueries, ProjectionBuilder, validate_projected_radial_edge,
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

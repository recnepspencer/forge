use forge_spec::facade::{MakeVertexFaceMutation, SpecState, SplitEdgeMutation};

use crate::projection::facade::{ProjectionBuilder, validate_projected_loop_wiring};

#[test]
fn projected_loop_wiring_accepts_seed_face() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    draft
        .execute(SplitEdgeMutation {
            half_edge: seed.value.half_edge,
        })
        .unwrap();
    let state = draft.commit().unwrap();
    let projection = ProjectionBuilder::build(&state).unwrap();
    assert!(validate_projected_loop_wiring(&projection).is_ok());
}

#[test]
fn projected_loop_wiring_rejects_broken_prev_link() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    draft
        .execute(SplitEdgeMutation {
            half_edge: seed.value.half_edge,
        })
        .unwrap();
    let state = draft.commit().unwrap();
    let mut projection = ProjectionBuilder::build(&state).unwrap();
    let he0 = crate::projection::data::ProjectedHalfEdgeId::new(0);
    let he1 = crate::projection::data::ProjectedHalfEdgeId::new(1);
    projection.half_edges[he0.index()].prev = he0;
    projection.half_edges[he1.index()].next = he1;

    let error = validate_projected_loop_wiring(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_prev_consistency"));
}

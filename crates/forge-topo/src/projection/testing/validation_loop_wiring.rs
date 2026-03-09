use forge_spec::facade::{
    MakeEdgeFaceMutation, MakeVertexFaceMutation, SpecState, SplitEdgeMutation,
};

use crate::projection::facade::{
    validate_projected_face_loop_membership_complete, validate_projected_loop_wiring,
    validate_projected_prev_consistency, ProjectionBuilder,
};

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

#[test]
fn projected_prev_consistency_rejects_broken_prev_link_directly() {
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

    let error = validate_projected_prev_consistency(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_prev_consistency"));
}

#[test]
fn projected_face_loop_membership_rejects_unreachable_face_halfedge() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    let split = draft
        .execute(SplitEdgeMutation {
            half_edge: seed.value.half_edge,
        })
        .unwrap();
    draft
        .execute(MakeEdgeFaceMutation {
            face: seed.value.face,
            vertex_a: seed.value.vertex,
            vertex_b: split.value.new_vertex,
        })
        .unwrap();
    let state = draft.commit().unwrap();
    let mut projection = ProjectionBuilder::build(&state).unwrap();
    let target_face = crate::projection::data::ProjectedFaceId::new(0);
    let foreign_half_edge = projection
        .half_edges()
        .iter()
        .enumerate()
        .find_map(|(index, half_edge)| {
            (half_edge.face != target_face).then_some(
                crate::projection::data::ProjectedHalfEdgeId::new(index as u32),
            )
        })
        .unwrap();
    projection.half_edges[foreign_half_edge.index()].face = target_face;

    let error = validate_projected_face_loop_membership_complete(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_face_loop_membership_complete"));
}

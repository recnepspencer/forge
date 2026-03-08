use forge_spec::facade::{
    MakeVertexFaceMutation, SpecShellKind, SpecShellOrientation, SpecState,
};

use crate::projection::facade::{
    ProjectionBuilder, validate_projected_topology_structural,
};

#[test]
fn projected_structural_validation_accepts_valid_sewn_sheet_shell() {
    let projection = build_mef_projection(SpecShellKind::Sheet);
    assert!(validate_projected_topology_structural(&projection).is_ok());
}

#[test]
fn projected_structural_validation_rejects_invalid_solid_shell() {
    let projection = build_mef_projection(SpecShellKind::Solid(SpecShellOrientation::Outer));

    let error = validate_projected_topology_structural(&projection).unwrap_err();
    let formatted = format!("{error:?}");
    assert!(
        formatted.contains("projected_shell_consistency")
            || formatted.contains("GeneralizedEulerViolation")
            || formatted.contains("NonOrientableSurface")
    );
}

fn build_mef_projection(kind: SpecShellKind) -> crate::projection::data::ProjectedTopology {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    let split = draft
        .execute(forge_spec::facade::SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap()
        .value;
    draft
        .execute(forge_spec::facade::MakeEdgeFaceMutation {
            face: seed.face,
            vertex_a: seed.vertex,
            vertex_b: split.new_vertex,
        })
        .unwrap();
    let state = draft.commit().unwrap();
    let mut projection = ProjectionBuilder::build(&state).unwrap();
    projection.shells[0].kind = kind;
    projection
}

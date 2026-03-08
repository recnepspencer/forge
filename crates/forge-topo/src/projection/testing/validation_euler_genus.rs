use forge_spec::facade::{
    MakeVertexFaceMutation, SpecShellKind, SpecShellOrientation, SpecState,
};

use crate::projection::facade::{ProjectionBuilder, validate_projected_per_component_euler};

#[test]
fn projected_per_component_euler_skips_sheet_shells() {
    let projection = build_seed_projection(SpecShellKind::Sheet);
    assert!(validate_projected_per_component_euler(&projection).is_ok());
}

#[test]
fn projected_per_component_euler_rejects_non_orientable_solid_characteristic() {
    let projection = build_seed_projection(SpecShellKind::Solid(SpecShellOrientation::Outer));

    let error = validate_projected_per_component_euler(&projection).unwrap_err();
    assert!(format!("{error:?}").contains("NonOrientableSurface"));
}

fn build_seed_projection(kind: SpecShellKind) -> crate::projection::data::ProjectedTopology {
    let mut draft = SpecState::empty().into_draft();
    draft.execute(MakeVertexFaceMutation).unwrap();
    let state = draft.commit().unwrap();
    let mut projection = ProjectionBuilder::build(&state).unwrap();
    projection.shells[0].kind = kind;
    projection
}

use crate::prelude::validate_spec_structure;
use forge_spec::facade::{
    MakeShellFaceMutation, MakeSolidMutation, MakeVertexFaceMutation, SpecShellKind,
    SpecShellOrientation, SpecState, SplitEdgeMutation,
};

#[test]
fn spec_state_structure_validation_accepts_valid_projection() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    draft
        .execute(SplitEdgeMutation {
            half_edge: seed.value.half_edge,
        })
        .unwrap();
    let spec = draft.commit().unwrap();

    validate_spec_structure(&spec).unwrap();
}

#[test]
fn spec_state_structure_validation_rejects_invalid_solid_shell_projection() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let shell_face = draft
        .execute(MakeShellFaceMutation {
            region: solid.value.region,
            kind: SpecShellKind::Solid(SpecShellOrientation::Outer),
        })
        .unwrap();
    draft
        .execute(SplitEdgeMutation {
            half_edge: shell_face.value.half_edge,
        })
        .unwrap();
    let spec = draft.commit().unwrap();

    let error = validate_spec_structure(&spec).unwrap_err();
    assert!(format!("{error}").contains("projected_shell_consistency"));
}

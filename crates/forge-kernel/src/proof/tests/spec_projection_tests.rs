use crate::proof::validate_manifold::validate_spec_structure;
use forge_spec::facade::{MakeVertexFaceMutation, SpecState};

#[test]
fn spec_state_structure_validation_accepts_valid_projection() {
    let mut draft = SpecState::empty().into_draft();
    draft.execute(MakeVertexFaceMutation).unwrap();
    let spec = draft.commit().unwrap();

    validate_spec_structure(&spec).unwrap();
}

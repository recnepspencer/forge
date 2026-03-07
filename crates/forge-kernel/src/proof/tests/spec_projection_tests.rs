use crate::proof::validate_manifold::validate_spec_structure;
use forge_spec::facade::{MakeVertexFaceMutation, SpecState, SplitEdgeMutation};

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

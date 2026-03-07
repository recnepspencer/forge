use crate::engine::output::spec_envelope::SpecEnvelope;
use crate::engine::pipeline::invariants::validate_spec_envelope_invariant;
use crate::engine::contracts::contract::InvariantKind;
use crate::geometry::facade::GeometryStore;
use crate::proof::checkpoint::schema::{ValidationCheckpoint, ValidationConfig};
use forge_spec::facade::{MakeVertexFaceMutation, SpecState};

#[test]
fn validate_spec_envelope_invariant_accepts_valid_projection() {
    let mut draft = SpecState::empty().into_draft();
    draft.execute(MakeVertexFaceMutation).unwrap();
    let spec = draft.commit().unwrap();
    let envelope = SpecEnvelope::new(spec, GeometryStore::default());

    let config = ValidationConfig {
        checkpoints: vec![ValidationCheckpoint::PostFeature],
        include_geometric: false,
        entity_limit: 0,
    };

    validate_spec_envelope_invariant(
        &envelope,
        &InvariantKind::ManifoldEdges,
        &config,
    )
    .unwrap();
}

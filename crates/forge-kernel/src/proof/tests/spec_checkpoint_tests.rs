use crate::prelude::{
    run_spec_checkpoint, run_spec_envelope_checkpoint, SpecEnvelope, ValidationCheckpoint,
    ValidationConfig,
};
use forge_spec::facade::{
    MakeShellFaceMutation, MakeSolidMutation, MakeVertexFaceMutation, SpecShellKind,
    SpecShellOrientation, SpecState, SplitEdgeMutation,
};

#[test]
fn spec_checkpoint_accepts_valid_projection() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap();
    let spec = draft.commit().unwrap();

    let vr = run_spec_checkpoint(
        &spec,
        &post_feature_config(),
        ValidationCheckpoint::PostFeature,
    )
    .unwrap();

    assert!(vr.is_passed());
    assert!(!vr.included_geometric());
    assert!(!vr.is_skipped());
}

#[test]
fn spec_envelope_checkpoint_accepts_valid_projection() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap();
    let spec = draft.commit().unwrap();
    let envelope = SpecEnvelope::from_spec(spec);

    let vr = run_spec_envelope_checkpoint(
        &envelope,
        &post_feature_config(),
        ValidationCheckpoint::PostFeature,
    )
    .unwrap();

    assert!(vr.is_passed());
    assert!(!vr.included_geometric());
}

#[test]
fn spec_checkpoint_rejects_invalid_solid_shell_projection() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap().value;
    let shell_face = draft
        .execute(MakeShellFaceMutation {
            region: solid.region,
            kind: SpecShellKind::Solid(SpecShellOrientation::Outer),
        })
        .unwrap();
    draft
        .execute(SplitEdgeMutation {
            half_edge: shell_face.value.half_edge,
        })
        .unwrap();
    let spec = draft.commit().unwrap();

    let error = run_spec_checkpoint(
        &spec,
        &post_feature_config(),
        ValidationCheckpoint::PostFeature,
    )
    .unwrap_err();

    assert!(format!("{error}").contains("projected_shell_consistency"));
}

#[test]
fn spec_checkpoint_respects_skip_gates() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap();
    let spec = draft.commit().unwrap();

    let mut inactive = post_feature_config();
    inactive.set_checkpoints(vec![]);
    let skipped_inactive =
        run_spec_checkpoint(&spec, &inactive, ValidationCheckpoint::PostFeature).unwrap();
    assert!(skipped_inactive.is_skipped());

    let mut limited = post_feature_config();
    limited.set_entity_limit(1);
    let skipped_limited =
        run_spec_checkpoint(&spec, &limited, ValidationCheckpoint::PostFeature).unwrap();
    assert!(skipped_limited.is_skipped());
}

fn post_feature_config() -> ValidationConfig {
    ValidationConfig {
        checkpoints: vec![ValidationCheckpoint::PostFeature],
        include_geometric: true,
        entity_limit: 0,
    }
}

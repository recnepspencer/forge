use super::contract_subject::retained_replay_parts;
use worth_spatial::facade::retained_replay_workload::{
    ReplayWorkload, RetainedArtifactSet, RetainedWorkload, UnsupportedReplayReasonCode,
};

#[test]
fn retained_replay_workload_explains_missing_inputs_without_fallback_extraction() {
    let missing_artifacts = retained_replay_parts("retained-replay-missing-artifacts");
    let missing_artifacts_denial =
        ReplayWorkload::for_transformed_workload(missing_artifacts.transformed)
            .declared("missing retained artifacts")
            .replay()
            .expect_err("missing retained artifacts must deny replay");
    assert_eq!(
        missing_artifacts_denial.reason_code(),
        UnsupportedReplayReasonCode::MissingRetainedArtifacts
    );
    assert_eq!(
        missing_artifacts_denial.human_reason(),
        "Retained replay workload requires retained artifacts captured before replay."
    );

    let missing_projection = retained_replay_parts("retained-replay-missing-projection");
    let retained_only =
        RetainedArtifactSet::from_retained_planar_facts(missing_projection.retained_parts.retained);
    let missing_projection_denial =
        ReplayWorkload::for_transformed_workload(missing_projection.transformed)
            .declared("missing projection-consumed facts")
            .with_retained_artifacts(retained_only)
            .replay()
            .expect_err("retained-only artifacts must deny replay");
    assert_eq!(
        missing_projection_denial.reason_code(),
        UnsupportedReplayReasonCode::MissingProjectionConsumedFacts
    );
    assert_eq!(
        missing_projection_denial.human_reason(),
        "Retained replay workload requires projection-consumed facts captured from the retained planar artifact."
    );

    let missing_declaration = retained_replay_parts("retained-replay-missing-declaration");
    let declaration_denial =
        ReplayWorkload::for_transformed_workload(missing_declaration.transformed)
            .declared("   ")
            .replay()
            .expect_err("blank declaration must deny before replay");
    assert_eq!(
        declaration_denial.reason_code(),
        UnsupportedReplayReasonCode::MissingDeclaration
    );
    assert_eq!(
        declaration_denial.human_reason(),
        "Retained replay workload requires a human-readable declaration."
    );
}

#[test]
fn retained_workload_capture_explains_missing_inputs_before_replay() {
    let missing_projection = retained_replay_parts("retained-capture-missing-projection");
    let missing_projection_denial =
        RetainedWorkload::from_retained_planar_facts(missing_projection.retained_parts.retained)
            .declared("capture without projection-consumed facts")
            .capture()
            .expect_err("retained capture must deny missing projection-consumed facts");
    assert_eq!(
        missing_projection_denial.reason_code(),
        UnsupportedReplayReasonCode::MissingProjectionConsumedFacts
    );
    assert_eq!(
        missing_projection_denial.human_reason(),
        "Retained artifact capture requires projection-consumed facts from the retained planar artifact."
    );

    let missing_declaration = retained_replay_parts("retained-capture-missing-declaration");
    let missing_declaration_denial =
        RetainedWorkload::from_retained_planar_facts(missing_declaration.retained_parts.retained)
            .declared("   ")
            .with_projection_consumed_facts(missing_declaration.projection_consumed)
            .capture()
            .expect_err("retained capture must deny blank declaration");
    assert_eq!(
        missing_declaration_denial.reason_code(),
        UnsupportedReplayReasonCode::MissingDeclaration
    );
    assert_eq!(
        missing_declaration_denial.human_reason(),
        "Retained artifact capture requires a human-readable declaration."
    );
}

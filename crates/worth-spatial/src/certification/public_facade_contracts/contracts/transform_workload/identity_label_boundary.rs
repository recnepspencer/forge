use super::contract_subject::projected_cube_workload;
use worth_spatial::facade::transform_workload::{
    TransformSequence, TransformWorkload, UnsupportedTransformReasonCode, VectorDelta,
};

#[test]
fn transform_workload_rejects_missing_declaration_and_sequence() {
    let missing_declaration = TransformWorkload::for_projected_workload(projected_cube_workload(
        "transform missing declaration",
    ))
    .declared("")
    .with_transform_sequence(TransformSequence::new().cancel_with_exact_replay(16))
    .transform()
    .expect_err("blank declaration should deny before transform evidence");

    assert_eq!(
        missing_declaration.reason_code(),
        UnsupportedTransformReasonCode::MissingDeclaration
    );
    assert_eq!(
        missing_declaration.human_reason(),
        "Transform workload requires a human-readable declaration."
    );

    let missing_sequence = TransformWorkload::for_projected_workload(projected_cube_workload(
        "transform missing sequence",
    ))
    .declared("missing transform sequence")
    .transform()
    .expect_err("transform sequence is required");

    assert_eq!(
        missing_sequence.reason_code(),
        UnsupportedTransformReasonCode::MissingTransformSequence
    );
    assert_eq!(
        missing_sequence.human_reason(),
        "Transform workload requires an explicit transform sequence."
    );
}

#[test]
fn transform_workload_rejects_identity_label_motion() {
    let denied =
        TransformWorkload::for_projected_workload(projected_cube_workload("transform label only"))
            .declared("rename-only motion")
            .with_transform_sequence(TransformSequence::identity_label_only("moved"))
            .transform()
            .expect_err("label-only motion cannot prove transform evidence");

    assert_eq!(
        denied.reason_code(),
        UnsupportedTransformReasonCode::LabelOnlyMotionEvidence
    );
    assert_eq!(
        denied.human_reason(),
        "Transform workload requires coordinate, basis, or posture evidence; label-only motion is not transform evidence."
    );
    assert!(!denied.can_enter_transform_posture_consumption());
    assert!(!denied.can_enter_operator_execution());
}

#[test]
fn transform_workload_rejects_zero_translation_without_evidence() {
    let denied = TransformWorkload::for_projected_workload(projected_cube_workload(
        "transform zero translation",
    ))
    .declared("zero translation")
    .with_transform_sequence(TransformSequence::new().translate(VectorDelta::xy(0, 0)))
    .transform()
    .expect_err("zero translation cannot satisfy transform evidence");

    assert_eq!(
        denied.reason_code(),
        UnsupportedTransformReasonCode::LabelOnlyMotionEvidence
    );
    assert_eq!(
        denied.human_reason(),
        "Transform workload requires coordinate, basis, or posture evidence; label-only motion is not transform evidence."
    );
}

#[test]
fn transform_workload_rejects_non_catalog_cancellation_step_counts() {
    let denied = TransformWorkload::for_projected_workload(projected_cube_workload(
        "transform invalid cancellation",
    ))
    .declared("invalid cancellation count")
    .with_transform_sequence(TransformSequence::new().cancel_with_exact_replay(17))
    .transform()
    .expect_err("non-catalog cancellation count should deny");

    assert_eq!(
        denied.reason_code(),
        UnsupportedTransformReasonCode::InvalidCancellationStepCount
    );
    assert_eq!(
        denied.human_reason(),
        "Transform workload exact cancellation replay requires one catalog profile: 16 acceptance steps or 64 hostile catalog steps."
    );
}

#[test]
fn transform_workload_rejects_split_cancellation_counts() {
    let denied = TransformWorkload::for_projected_workload(projected_cube_workload(
        "transform split cancellation",
    ))
    .declared("split cancellation count")
    .with_transform_sequence(
        TransformSequence::new()
            .cancel_with_exact_replay(8)
            .cancel_with_exact_replay(8),
    )
    .transform()
    .expect_err("split non-catalog cancellation counts should deny");

    assert_eq!(
        denied.reason_code(),
        UnsupportedTransformReasonCode::InvalidCancellationStepCount
    );
    assert_eq!(
        denied.human_reason(),
        "Transform workload exact cancellation replay requires one catalog profile: 16 acceptance steps or 64 hostile catalog steps."
    );
}

#[test]
fn transform_workload_rejects_combined_catalog_cancellation_profiles() {
    let denied = TransformWorkload::for_projected_workload(projected_cube_workload(
        "transform combined catalog cancellation",
    ))
    .declared("combined cancellation profiles")
    .with_transform_sequence(
        TransformSequence::new()
            .cancel_with_exact_replay(16)
            .cancel_with_exact_replay(64),
    )
    .transform()
    .expect_err("combined catalog cancellation profiles should deny");

    assert_eq!(
        denied.reason_code(),
        UnsupportedTransformReasonCode::InvalidCancellationStepCount
    );
    assert_eq!(
        denied.human_reason(),
        "Transform workload exact cancellation replay requires one catalog profile: 16 acceptance steps or 64 hostile catalog steps."
    );
}

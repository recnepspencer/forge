pub(crate) mod subject;

use subject::{
    certify_platform_thin_feature_scale_separation, thin_feature_foreign_precision_witness_outcome,
    thin_feature_integrity_mismatch_outcome, thin_feature_missing_local_frame_outcome,
    thin_feature_missing_platform_projection_outcome, thin_feature_policy_required_outcome,
    thin_feature_precision_basis_failure_outcome, thin_feature_predicate_uncertain_outcome,
    thin_feature_unsupported_tiny_rotation_outcome, thin_feature_world_magnitude_floor_outcome,
};
use worth_spatial::facade::user_response::{
    WorthPolicyDecision, WorthUserOutcome, WorthUserOutcomeCauseKind, WorthUserOutcomeKind,
};

#[test]
fn mb_m6_3_thin_feature_scale_separation_contract() {
    let subject = certify_platform_thin_feature_scale_separation("contract");
    let counters = subject.receipt.counters();

    assert_eq!(counters.thin_feature_count(), 12);
    assert_eq!(counters.local_scale_order_count(), 3);
    assert_eq!(counters.world_magnitude_order_count(), 1);
    assert!(counters.precision_escalation_count() >= 3);
    assert_eq!(subject.receipt.local_scale_orders(), &[-12, -9, -6]);
    assert_eq!(subject.receipt.required_world_magnitude_order(), 12);
    assert_eq!(
        subject.receipt.platform_projection_identity(),
        subject.platform_projection_identity.as_str()
    );
    assert!(counters.local_basis_part_count() > 0);
    assert!(counters.projected_entity_count() >= 12);
    assert!(counters.transform_step_count() > 0);
    assert_eq!(counters.tiny_rotation_pressure_count(), 1);
    assert!(counters.projection_consumed_basis_count() > 0);
    assert_eq!(counters.diagnostic_count(), 1);
    assert_eq!(counters.user_outcome_count(), 1);
    assert_eq!(subject.precision_scale_orders, vec![-9, -12, -6]);
    assert_eq!(subject.user_outcome.kind(), WorthUserOutcomeKind::Admitted);
    assert_human_readable(subject.user_outcome.human_response().summary());
}

#[test]
fn mb_m6_3_micro_feature_outcome_matrix_is_production_owned() {
    let outcomes = vec![
        thin_feature_policy_required_outcome("matrix-policy"),
        thin_feature_precision_basis_failure_outcome("matrix-precision"),
        thin_feature_predicate_uncertain_outcome("matrix-predicate"),
        thin_feature_unsupported_tiny_rotation_outcome("matrix-tiny-rotation"),
        thin_feature_integrity_mismatch_outcome("matrix-integrity"),
        thin_feature_missing_local_frame_outcome("matrix-local-frame"),
        thin_feature_missing_platform_projection_outcome("matrix-platform-projection"),
    ];

    assert_one_kind(&outcomes, WorthUserOutcomeKind::PolicyRequired);
    assert_one_kind(&outcomes, WorthUserOutcomeKind::PredicateUncertain);
    assert_one_kind(&outcomes, WorthUserOutcomeKind::Unsupported);
    assert_one_kind(&outcomes, WorthUserOutcomeKind::IntegrityMismatch);
    assert_kind_count(&outcomes, WorthUserOutcomeKind::NoOptions, 3);

    assert_message_contains(&outcomes, "user policy decision");
    assert_message_contains(&outcomes, "local feature scale, world magnitude");
    assert_message_contains(&outcomes, "predicate authority could not certify");
    assert_message_contains(&outcomes, "tiny-rotation posture is unsupported");
    assert_message_contains(&outcomes, "same local frame");
    assert_message_contains(&outcomes, "local-frame receipt");
    assert_message_contains(&outcomes, "catalog projection receipt");

    assert_branch(
        &outcomes[0],
        WorthUserOutcomeKind::PolicyRequired,
        WorthUserOutcomeCauseKind::PolicyRequired,
    );
    assert_eq!(
        outcomes[0].choices(),
        &[WorthPolicyDecision::pause_for_manual_inspection()]
    );
    assert_branch(
        &outcomes[1],
        WorthUserOutcomeKind::NoOptions,
        WorthUserOutcomeCauseKind::MissingEvidence,
    );
    assert_branch(
        &outcomes[2],
        WorthUserOutcomeKind::PredicateUncertain,
        WorthUserOutcomeCauseKind::PredicateUncertain,
    );
    assert_branch(
        &outcomes[3],
        WorthUserOutcomeKind::Unsupported,
        WorthUserOutcomeCauseKind::UnsupportedInput,
    );
    assert_branch(
        &outcomes[4],
        WorthUserOutcomeKind::IntegrityMismatch,
        WorthUserOutcomeCauseKind::IntegrityMismatch,
    );
    assert_branch(
        &outcomes[5],
        WorthUserOutcomeKind::NoOptions,
        WorthUserOutcomeCauseKind::MissingEvidence,
    );
    assert_branch(
        &outcomes[6],
        WorthUserOutcomeKind::NoOptions,
        WorthUserOutcomeCauseKind::MissingEvidence,
    );

    for outcome in &outcomes {
        assert_human_readable(outcome.human_response().summary());
        assert!(!outcome.evidence().digest().is_empty());
        assert!(!outcome.evidence().source_identity().is_empty());
        if outcome.kind() != WorthUserOutcomeKind::PolicyRequired {
            assert!(outcome.choices().is_empty());
        }
    }
}

#[test]
fn mb_m6_3_projection_consumption_preserves_local_basis() {
    let subject = certify_platform_thin_feature_scale_separation("projection-basis");

    assert_eq!(
        subject.receipt.local_frame_identity(),
        subject.receipt.projection_consumed_local_frame_identity()
    );
}

#[test]
fn mb_m6_3_large_world_magnitude_floor_cannot_be_lowered() {
    let outcome = thin_feature_world_magnitude_floor_outcome("world-floor");

    assert_eq!(outcome.kind(), WorthUserOutcomeKind::NoOptions);
    assert_message_contains(
        &[outcome],
        "local feature scale, world magnitude, and precision basis",
    );
}

#[test]
fn mb_m6_3_precision_witnesses_must_share_primary_basis() {
    let outcome = thin_feature_foreign_precision_witness_outcome("foreign-precision");

    assert_branch(
        &outcome,
        WorthUserOutcomeKind::NoOptions,
        WorthUserOutcomeCauseKind::MissingEvidence,
    );
    assert_message_contains(
        &[outcome],
        "local feature scale, world magnitude, and precision basis",
    );
}

fn assert_branch(
    outcome: &WorthUserOutcome,
    kind: WorthUserOutcomeKind,
    cause_kind: WorthUserOutcomeCauseKind,
) {
    assert_eq!(outcome.kind(), kind);
    assert_eq!(outcome.cause().map(|cause| cause.kind()), Some(cause_kind));
}

fn assert_one_kind(outcomes: &[WorthUserOutcome], kind: WorthUserOutcomeKind) {
    assert_kind_count(outcomes, kind, 1);
}

fn assert_kind_count(outcomes: &[WorthUserOutcome], kind: WorthUserOutcomeKind, count: usize) {
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| outcome.kind() == kind)
            .count(),
        count
    );
}

fn assert_message_contains(outcomes: &[WorthUserOutcome], expected: &str) {
    assert!(
        outcomes
            .iter()
            .any(|outcome| outcome.human_response().summary().contains(expected)),
        "missing thin-feature outcome message containing {expected:?}"
    );
}

fn assert_human_readable(message: &str) {
    assert!(!message.trim().is_empty());
    assert!(
        !message.contains('_'),
        "user-facing thin-feature message must not leak machine tokens: {message}"
    );
    assert!(
        !message
            .split_whitespace()
            .any(|word| word.matches('-').count() >= 3),
        "user-facing thin-feature message must explain causes in prose: {message}"
    );
}

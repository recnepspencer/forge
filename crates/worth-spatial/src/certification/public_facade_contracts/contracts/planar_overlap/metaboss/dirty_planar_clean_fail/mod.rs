pub(crate) mod subject;

use std::collections::BTreeSet;

use subject::{
    dirty_clean_fail_outcome_matrix, dirty_clean_fail_rejects_foreign_boundary_response,
    dirty_clean_fail_rejects_wrong_user_response, dirty_clean_fail_subject_matrix,
    dirty_clean_fail_with_topology_seed, dirty_transform_pressure_subject,
    stable_identity_mismatch_outcome,
};
use worth_spatial::facade::dirty_planar_clean_fail::{
    DirtyPlanarCleanFailCase, DirtyPlanarCleanFailError,
};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserOutcomeCauseKind, WorthUserOutcomeKind,
};

#[test]
fn mb_m6_5_dirty_planar_input_clean_fail_localization() {
    let subject = dirty_clean_fail_with_topology_seed(
        "mb-m6-5-self-intersection",
        DirtyPlanarCleanFailCase::SelfIntersectingLoop,
    );
    let counters = subject.receipt.counters();

    assert_eq!(
        subject.receipt.dirty_case(),
        DirtyPlanarCleanFailCase::SelfIntersectingLoop
    );
    assert_eq!(counters.topology_clean_fail_receipts(), 1);
    assert_eq!(counters.clean_fail_boundary_receipts(), 1);
    assert_eq!(counters.recovery_receipts(), 1);
    assert_eq!(counters.transform_posture_receipts(), 1);
    assert!(counters.diagnostic_receipts() > 0);
    assert_eq!(counters.user_outcome_receipts(), 1);
    assert_branch(
        &subject.user_outcome,
        WorthUserOutcomeKind::NoOptions,
        WorthUserOutcomeCauseKind::DirtyInput,
    );
    assert_human_readable(subject.user_outcome.human_response().summary());
    assert!(subject
        .user_outcome
        .human_response()
        .summary()
        .contains("self-intersecting loop"));
}

#[test]
fn mb_m6_5_dirty_outcome_matrix_branches_each_dirty_kind() {
    let outcomes = dirty_clean_fail_outcome_matrix("mb-m6-5-matrix");

    assert_eq!(outcomes.len(), 4);
    assert_message_contains(&outcomes, "self-intersecting loop");
    assert_message_contains(&outcomes, "non-manifold wire");
    assert_message_contains(&outcomes, "thin wall");
    assert_message_contains(&outcomes, "orientation inconsistency");
    for outcome in &outcomes {
        assert_branch(
            outcome,
            WorthUserOutcomeKind::NoOptions,
            WorthUserOutcomeCauseKind::DirtyInput,
        );
        assert!(outcome.choices().is_empty());
        assert_human_readable(outcome.human_response().summary());
    }
}

#[test]
fn mb_m6_5_dirty_matrix_uses_distinct_topology_evidence_per_dirty_kind() {
    let subjects = dirty_clean_fail_subject_matrix("mb-m6-5-distinct-evidence");

    assert_eq!(subjects.len(), 4);
    let topology_identities = subjects
        .iter()
        .map(|subject| subject.receipt.topology_clean_fail_identity().to_string())
        .collect::<BTreeSet<_>>();
    let workload_digests = subjects
        .iter()
        .map(|subject| subject.receipt.clean_fail_digest().to_string())
        .collect::<BTreeSet<_>>();

    assert_eq!(
        topology_identities.len(),
        subjects.len(),
        "dirty matrix must not reuse one topology clean-fail receipt under multiple labels"
    );
    assert_eq!(
        workload_digests.len(),
        subjects.len(),
        "dirty matrix must produce distinct workload receipts for each dirty class"
    );
}

#[test]
fn mb_m6_5_dirty_transform_pressure_preserves_failure_class() {
    let subject = dirty_transform_pressure_subject("mb-m6-5-transform-pressure");

    assert_branch(
        &subject.user_outcome,
        WorthUserOutcomeKind::NoOptions,
        WorthUserOutcomeCauseKind::DirtyInput,
    );
    assert_eq!(
        subject.receipt.dirty_case(),
        DirtyPlanarCleanFailCase::NonManifoldWire
    );
    assert_eq!(subject.receipt.counters().transform_posture_receipts(), 1);
    assert!(subject
        .user_outcome
        .human_response()
        .summary()
        .contains("non-manifold wire"));
    assert!(subject.user_outcome.choices().is_empty());
}

#[test]
fn mb_m6_5_stable_topology_identity_cannot_hide_dirty_geometry() {
    let outcome = stable_identity_mismatch_outcome("mb-m6-5-stable-id");

    assert_branch(
        &outcome,
        WorthUserOutcomeKind::IntegrityMismatch,
        WorthUserOutcomeCauseKind::IntegrityMismatch,
    );
    assert!(outcome
        .human_response()
        .summary()
        .contains("stable topology identity cannot hide dirty geometry"));
    assert_human_readable(outcome.human_response().summary());
}

#[test]
fn mb_m6_5_dirty_workload_rejects_wrong_user_response_receipt() {
    let error = dirty_clean_fail_rejects_wrong_user_response("mb-m6-5-wrong-response");

    assert_eq!(
        error,
        DirtyPlanarCleanFailError::UserResponseDidNotExplainDirtyNoOptions
    );
    assert_human_readable(&error.human_reason());
}

#[test]
fn mb_m6_5_dirty_workload_rejects_foreign_boundary_response_receipt() {
    let error = dirty_clean_fail_rejects_foreign_boundary_response("mb-m6-5-foreign-response");

    assert_eq!(
        error,
        DirtyPlanarCleanFailError::UserResponseDidNotConsumeCleanFailBoundary
    );
    assert_human_readable(&error.human_reason());
}

fn assert_branch(
    outcome: &WorthUserOutcome,
    kind: WorthUserOutcomeKind,
    cause_kind: WorthUserOutcomeCauseKind,
) {
    assert_eq!(outcome.kind(), kind);
    assert_eq!(outcome.cause().map(|cause| cause.kind()), Some(cause_kind));
}

fn assert_message_contains(outcomes: &[WorthUserOutcome], expected: &str) {
    assert!(
        outcomes
            .iter()
            .any(|outcome| outcome.human_response().summary().contains(expected)),
        "missing dirty clean-fail outcome message containing {expected:?}"
    );
}

fn assert_human_readable(message: &str) {
    assert!(!message.trim().is_empty());
    assert!(
        !message.contains('_'),
        "dirty clean-fail response must not leak machine tokens: {message}"
    );
    assert!(
        !message
            .split_whitespace()
            .any(|word| word.matches('-').count() >= 3),
        "dirty clean-fail response must explain causes in prose: {message}"
    );
}

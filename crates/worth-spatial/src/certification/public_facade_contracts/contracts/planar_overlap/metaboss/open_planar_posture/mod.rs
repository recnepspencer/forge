pub(crate) mod subject;

use std::collections::BTreeSet;

use subject::{bounded_surrogate_denials, half_space_subject, posture_matrix};
use worth_spatial::facade::open_planar_posture::{OpenPlanarPostureCase, OpenPlanarPostureError};
use worth_spatial::facade::planar_clean_fail_boundary::PlanarOpenInputKind;
use worth_spatial::facade::planar_diagnostics::PlanarDiagnosticSubjectKind;
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserOutcomeCauseKind, WorthUserOutcomeKind,
};

#[test]
fn mb_m6_6_unbounded_half_space_planar_posture() {
    let subject = half_space_subject("mb-m6-6-half-space");
    let counters = subject.receipt.counters();

    assert_eq!(
        subject.receipt.posture_case(),
        OpenPlanarPostureCase::PolicyRequiredHalfSpace
    );
    assert_eq!(
        subject.receipt.open_input_kind(),
        Some(PlanarOpenInputKind::HalfSpaceGroup)
    );
    assert_eq!(
        subject.receipt.diagnostic_subject_kind(),
        PlanarDiagnosticSubjectKind::PolicyRequired
    );
    assert_eq!(counters.topology_receipts(), 1);
    assert_eq!(counters.unsupported_surface_receipts(), 1);
    assert_eq!(counters.clean_fail_boundary_receipts(), 1);
    assert_eq!(counters.transform_posture_receipts(), 1);
    assert_eq!(counters.diagnostic_receipts(), 1);
    assert_eq!(counters.user_outcome_receipts(), 1);
    assert_eq!(counters.bounded_surrogate_rejections(), 1);
    assert!(!subject.receipt.diagnostic_receipt_identity().is_empty());
    assert!(subject.receipt.bounded_surrogate_was_not_used());
    assert_branch(
        &subject.user_outcome,
        WorthUserOutcomeKind::PolicyRequired,
        WorthUserOutcomeCauseKind::PolicyRequired,
    );
    assert!(!subject.user_outcome.choices().is_empty());
    assert_human_readable(subject.user_outcome.human_response().summary());
    assert!(subject
        .user_outcome
        .human_response()
        .summary()
        .contains("Half-space input needs"));
}

#[test]
fn mb_m6_6_unbounded_outcome_matrix_explains_no_options() {
    let subjects = posture_matrix("mb-m6-6-matrix");
    let cases = subjects
        .iter()
        .map(|subject| subject.receipt.posture_case())
        .collect::<BTreeSet<_>>();

    assert_eq!(subjects.len(), 7);
    assert!(cases.contains(&OpenPlanarPostureCase::UnsupportedOpenSheet));
    assert!(cases.contains(&OpenPlanarPostureCase::UnsupportedOpenWire));
    assert!(cases.contains(&OpenPlanarPostureCase::PolicyRequiredHalfSpace));
    assert!(cases.contains(&OpenPlanarPostureCase::PredicateUncertain));
    assert!(cases.contains(&OpenPlanarPostureCase::BoundedOperatorIncompatibility));
    assert!(cases.contains(&OpenPlanarPostureCase::IntegrityMismatch));
    assert!(cases.contains(&OpenPlanarPostureCase::TransformDivergence));

    for subject in &subjects {
        let (kind, cause) = expected_branch(subject.receipt.posture_case());
        assert_branch(&subject.user_outcome, kind, cause);
        assert_human_readable(subject.user_outcome.human_response().summary());
        assert_eq!(
            subject.receipt.open_input_kind(),
            expected_open_input_kind(subject.receipt.posture_case())
        );
        assert_eq!(
            subject.receipt.diagnostic_subject_kind(),
            expected_diagnostic_subject_kind(subject.receipt.posture_case())
        );
        assert_eq!(subject.receipt.counters().unsupported_surface_receipts(), 1);
        assert_eq!(subject.receipt.counters().clean_fail_boundary_receipts(), 1);
        assert_eq!(subject.receipt.counters().diagnostic_receipts(), 1);
        assert!(!subject.receipt.diagnostic_receipt_identity().is_empty());
        if subject.receipt.posture_case() == OpenPlanarPostureCase::PolicyRequiredHalfSpace {
            assert!(!subject.user_outcome.choices().is_empty());
        } else {
            assert!(subject.user_outcome.choices().is_empty());
        }
    }
}

#[test]
fn mb_m6_6_half_space_transform_canonicalization_and_divergence() {
    let subjects = posture_matrix("mb-m6-6-transform-divergence");
    let divergent = subjects
        .iter()
        .find(|subject| {
            subject.receipt.posture_case() == OpenPlanarPostureCase::TransformDivergence
        })
        .expect("transform divergence branch");

    assert_branch(
        &divergent.user_outcome,
        WorthUserOutcomeKind::Denied,
        WorthUserOutcomeCauseKind::DeniedMovementOrRotation,
    );
    assert!(divergent
        .user_outcome
        .human_response()
        .summary()
        .contains("Movement or rotation changes"));
    assert_eq!(divergent.receipt.counters().transform_posture_receipts(), 1);

    for denial in bounded_surrogate_denials("mb-m6-6-bounded-surrogate") {
        assert_eq!(denial, OpenPlanarPostureError::BoundedSurrogateAttempted);
        assert_human_readable(&denial.human_reason());
    }
}

fn expected_open_input_kind(posture_case: OpenPlanarPostureCase) -> Option<PlanarOpenInputKind> {
    match posture_case {
        OpenPlanarPostureCase::PolicyRequiredHalfSpace => Some(PlanarOpenInputKind::HalfSpaceGroup),
        OpenPlanarPostureCase::UnsupportedOpenSheet
        | OpenPlanarPostureCase::UnsupportedOpenWire
        | OpenPlanarPostureCase::PredicateUncertain
        | OpenPlanarPostureCase::BoundedOperatorIncompatibility
        | OpenPlanarPostureCase::IntegrityMismatch
        | OpenPlanarPostureCase::TransformDivergence => Some(PlanarOpenInputKind::OpenPlanarDomain),
    }
}

fn expected_diagnostic_subject_kind(
    posture_case: OpenPlanarPostureCase,
) -> PlanarDiagnosticSubjectKind {
    match posture_case {
        OpenPlanarPostureCase::UnsupportedOpenSheet
        | OpenPlanarPostureCase::UnsupportedOpenWire
        | OpenPlanarPostureCase::BoundedOperatorIncompatibility => {
            PlanarDiagnosticSubjectKind::UnsupportedPlanarClass
        }
        OpenPlanarPostureCase::PolicyRequiredHalfSpace => {
            PlanarDiagnosticSubjectKind::PolicyRequired
        }
        OpenPlanarPostureCase::PredicateUncertain => PlanarDiagnosticSubjectKind::PredicateFailure,
        OpenPlanarPostureCase::IntegrityMismatch | OpenPlanarPostureCase::TransformDivergence => {
            PlanarDiagnosticSubjectKind::UnsupportedPlanarClass
        }
    }
}

fn expected_branch(
    posture_case: OpenPlanarPostureCase,
) -> (WorthUserOutcomeKind, WorthUserOutcomeCauseKind) {
    match posture_case {
        OpenPlanarPostureCase::UnsupportedOpenSheet
        | OpenPlanarPostureCase::UnsupportedOpenWire
        | OpenPlanarPostureCase::BoundedOperatorIncompatibility => (
            WorthUserOutcomeKind::Unsupported,
            WorthUserOutcomeCauseKind::UnsupportedInput,
        ),
        OpenPlanarPostureCase::PolicyRequiredHalfSpace => (
            WorthUserOutcomeKind::PolicyRequired,
            WorthUserOutcomeCauseKind::PolicyRequired,
        ),
        OpenPlanarPostureCase::PredicateUncertain => (
            WorthUserOutcomeKind::PredicateUncertain,
            WorthUserOutcomeCauseKind::PredicateUncertain,
        ),
        OpenPlanarPostureCase::IntegrityMismatch => (
            WorthUserOutcomeKind::IntegrityMismatch,
            WorthUserOutcomeCauseKind::IntegrityMismatch,
        ),
        OpenPlanarPostureCase::TransformDivergence => (
            WorthUserOutcomeKind::Denied,
            WorthUserOutcomeCauseKind::DeniedMovementOrRotation,
        ),
    }
}

fn assert_branch(
    outcome: &WorthUserOutcome,
    kind: WorthUserOutcomeKind,
    cause_kind: WorthUserOutcomeCauseKind,
) {
    assert_eq!(outcome.kind(), kind);
    assert_eq!(outcome.cause().map(|cause| cause.kind()), Some(cause_kind));
}

fn assert_human_readable(message: &str) {
    assert!(!message.trim().is_empty());
    assert!(
        !message.contains('_'),
        "open posture response must not leak machine tokens: {message}"
    );
    assert!(
        !message
            .split_whitespace()
            .any(|word| word.matches('-').count() >= 3),
        "open posture response must explain causes in prose: {message}"
    );
}

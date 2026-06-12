pub(crate) mod subject;

use subject::{
    assert_family_is_unsupported, generated_feature_smuggling_denial,
    kernel_summary_substitution_outcome, missing_surface_support_outcome,
    mixed_surface_kill_box_denial_for_family_matrix, mixed_surface_kill_box_subject,
    plane_receipt_smuggling_denials, unsupported_digest_set, unsupported_reason_set,
    unsupported_runs, wrong_family_response_denial,
};
use worth_spatial::facade::mixed_surface_kill_box::{
    MixedSurfaceKillBoxDenial, MixedSurfaceKillBoxOutcomeKind,
};
use worth_spatial::facade::surface_support::SurfaceFamily;
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserOutcomeCauseKind, WorthUserOutcomeKind,
};

#[test]
fn mb_m6_nmt_2_mixed_surface_kill_box_admits_only_plane() {
    let subject = mixed_surface_kill_box_subject("mb-m6-nmt-2-admits-only-plane");
    let counters = subject.receipt.counters();
    let plane = subject.receipt.plane_control().expect("plane run");

    assert_eq!(counters.family_run_count(), SurfaceFamily::ALL.len());
    assert_eq!(counters.certified_plane_count(), 1);
    assert_eq!(counters.unsupported_family_count(), 4);
    assert_eq!(counters.support_receipt_count(), SurfaceFamily::ALL.len());
    assert_eq!(counters.user_outcome_count(), SurfaceFamily::ALL.len());
    assert!(counters.upstream_geometry_carriers() > 0);
    assert!(plane.is_acceptable_m7_input());
    assert_eq!(
        plane.user_response_digest(),
        plane.support_evidence_digest()
    );
    assert_branch(plane.user_outcome(), WorthUserOutcomeKind::Admitted, None);
    assert_eq!(
        subject.catalog.recipe().human_name(),
        "mixed surface kill box workload recipe"
    );
    assert_eq!(
        subject.receipt.stable_geometry_binding_identity(),
        subject
            .catalog
            .bound_geometry()
            .receipts()
            .stage_identity()
            .receipt_identity()
    );

    for run in unsupported_runs(&subject.receipt) {
        assert_family_is_unsupported(run);
        assert_branch(
            run.user_outcome(),
            WorthUserOutcomeKind::Unsupported,
            Some(WorthUserOutcomeCauseKind::UnsupportedInput),
        );
    }
}

#[test]
fn mb_m6_nmt_2_non_plane_denials_have_distinct_digests_and_human_reasons() {
    let subject = mixed_surface_kill_box_subject("mb-m6-nmt-2-distinct-non-plane");
    let digests = unsupported_digest_set(&subject.receipt);
    let reasons = unsupported_reason_set(&subject.receipt);

    assert_eq!(digests.len(), 4);
    assert_eq!(reasons.len(), 4);
    for family in [
        SurfaceFamily::AnalyticNonPlanar,
        SurfaceFamily::Freeform,
        SurfaceFamily::GeneratedFeature,
        SurfaceFamily::Unknown,
    ] {
        assert!(
            subject.receipt.run_for_family(family).is_some(),
            "missing unsupported run for {family:?}"
        );
    }
    for expected in [
        "analytic non-planar surface",
        "freeform surface",
        "generated feature surface",
        "unknown surface family",
    ] {
        assert!(
            reasons.iter().any(|reason| reason.contains(expected)),
            "missing unsupported reason for {expected}"
        );
    }
    for run in unsupported_runs(&subject.receipt) {
        assert_human_readable(run.human_reason());
    }
}

#[test]
fn mb_m6_nmt_2_rejects_incomplete_or_duplicate_surface_family_matrix() {
    let missing_unknown = mixed_surface_kill_box_denial_for_family_matrix(
        "mb-m6-nmt-2-missing-family",
        [
            SurfaceFamily::Plane,
            SurfaceFamily::AnalyticNonPlanar,
            SurfaceFamily::Freeform,
            SurfaceFamily::GeneratedFeature,
        ],
    );
    assert!(matches!(
        missing_unknown,
        MixedSurfaceKillBoxDenial::MissingFamilyRun {
            family: SurfaceFamily::Unknown
        }
    ));
    assert_human_readable(&missing_unknown.human_reason());

    let duplicate_plane = mixed_surface_kill_box_denial_for_family_matrix(
        "mb-m6-nmt-2-duplicate-family",
        [
            SurfaceFamily::Plane,
            SurfaceFamily::Plane,
            SurfaceFamily::AnalyticNonPlanar,
            SurfaceFamily::Freeform,
            SurfaceFamily::GeneratedFeature,
            SurfaceFamily::Unknown,
        ],
    );
    assert!(matches!(
        duplicate_plane,
        MixedSurfaceKillBoxDenial::DuplicateFamilyRun {
            family: SurfaceFamily::Plane
        }
    ));
    assert_human_readable(&duplicate_plane.human_reason());
}

#[test]
fn mb_m6_nmt_2_rejects_plane_receipt_and_kernel_summary_smuggling() {
    let subject = mixed_surface_kill_box_subject("mb-m6-nmt-2-smuggling");
    let smuggling_denials = plane_receipt_smuggling_denials(&subject.receipt);

    assert_eq!(smuggling_denials.len(), 4);
    for denial in smuggling_denials {
        assert!(matches!(
            denial,
            MixedSurfaceKillBoxDenial::SurfaceFamilyReceiptMismatch { .. }
        ));
        assert_human_readable(&denial.human_reason());
    }

    let wrong_response = wrong_family_response_denial(&subject.receipt);
    assert!(matches!(
        wrong_response,
        MixedSurfaceKillBoxDenial::WrongFamilyUserResponse { .. }
    ));
    let generated_feature = generated_feature_smuggling_denial(&subject.receipt);
    assert_eq!(
        generated_feature,
        MixedSurfaceKillBoxDenial::GeneratedFeatureSmugglingAttempt
    );
    assert_human_readable(&generated_feature.human_reason());

    let kernel_summary = kernel_summary_substitution_outcome();
    assert_branch(
        &kernel_summary,
        WorthUserOutcomeKind::IntegrityMismatch,
        Some(WorthUserOutcomeCauseKind::IntegrityMismatch),
    );
    assert!(kernel_summary
        .human_response()
        .summary()
        .contains("kernel summary substitution"));
}

#[test]
fn mb_m6_nmt_2_missing_surface_support_names_family_support_receipt() {
    let outcome = missing_surface_support_outcome(SurfaceFamily::Freeform);

    assert_branch(
        &outcome,
        WorthUserOutcomeKind::NoOptions,
        Some(WorthUserOutcomeCauseKind::MissingEvidence),
    );
    assert!(outcome
        .human_response()
        .summary()
        .contains("freeform surface"));
    assert!(outcome
        .human_response()
        .summary()
        .contains("surface-support receipt"));
    assert!(outcome.choices().is_empty());
    assert_human_readable(outcome.human_response().summary());
}

#[test]
fn mb_m6_nmt_2_outcome_matrix_branches_every_stop() {
    let subject = mixed_surface_kill_box_subject("mb-m6-nmt-2-matrix");
    let matrix = subject.outcome_matrix;

    assert!(matrix
        .row_for_kind(MixedSurfaceKillBoxOutcomeKind::Admitted)
        .is_some());
    assert!(matrix
        .row_for_kind(MixedSurfaceKillBoxOutcomeKind::Unsupported)
        .is_some());
    assert!(matrix
        .row_for_kind(MixedSurfaceKillBoxOutcomeKind::IntegrityMismatch)
        .is_some());
    assert!(matrix
        .row_for_kind(MixedSurfaceKillBoxOutcomeKind::Denied)
        .is_some());
    assert!(matrix
        .row_for_kind(MixedSurfaceKillBoxOutcomeKind::MissingEvidence)
        .is_some());
    for row in matrix.rows() {
        assert!(!row.evidence_identity().is_empty());
        assert_human_readable(row.human_reason());
    }
}

fn assert_branch(
    outcome: &WorthUserOutcome,
    kind: WorthUserOutcomeKind,
    cause: Option<WorthUserOutcomeCauseKind>,
) {
    assert_eq!(outcome.kind(), kind);
    assert_eq!(outcome.cause().map(|cause| cause.kind()), cause);
}

fn assert_human_readable(message: &str) {
    assert!(!message.trim().is_empty());
    assert!(
        !message.contains('_'),
        "mixed surface response must not leak machine tokens: {message}"
    );
    assert!(
        !message
            .split_whitespace()
            .any(|word| word.matches('-').count() >= 3),
        "mixed surface response must explain causes in prose: {message}"
    );
}

mod runtime_handles;
pub(crate) mod subject;

use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::boolean_readiness_workload::PlanarBooleanReadinessWorkloadDenialKind;
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserOutcomeCauseKind, WorthUserOutcomeKind, WorthUserResponseSource,
    WorthUserResponseWorkload,
};

use self::subject::{
    certify_final_boss, clean_failure_final_boss, kernel_summary_substitution_final_boss,
    mismatched_parity_final_boss, orientation_flip_final_boss, policy_required_final_boss,
    predicate_uncertain_final_boss, recovery_replay_mismatch_final_boss, unsupported_final_boss,
};

#[test]
fn mb_m6_8_boolean_readiness_final_boss() {
    run_with_real_workload_stack(|| {
        let subject = certify_final_boss("mb-m6-8-final-boss");

        assert!(subject
            .receipt
            .m7_readiness_receipt()
            .is_acceptable_m7_input());
        assert_eq!(
            subject.receipt.m7_readiness_receipt().boolean_result(),
            None
        );
        assert_eq!(
            subject.receipt.m7_readiness_receipt().imprint_action(),
            None
        );
        assert_eq!(
            subject
                .receipt
                .counters()
                .required_evidence_stages_consumed(),
            10
        );
        assert_eq!(subject.receipt.counters().ledger_rows_consumed(), 8);
        assert_eq!(subject.receipt.counters().parity_lanes_consumed(), 9);
        assert_eq!(subject.receipt.counters().query_boundary_rows(), 1);
        assert_outcome(&subject.user_outcome, WorthUserOutcomeKind::Admitted, None);
        assert_human_readable(subject.user_outcome.human_response().summary());
        assert!(subject
            .user_outcome
            .human_response()
            .summary()
            .contains("M7 may proceed"));
    });
}

#[test]
fn mb_m6_8_boolean_readiness_is_registered_in_support_matrix() {
    let support_matrix = geometry_public_support_matrix();
    let support = support_matrix
        .row_for_surface(GeometryPublicSurface::PlanarBooleanReadinessWorkload)
        .expect("boolean readiness workload support row");
    assert_eq!(
        support.declared_family_key(),
        Some("PlanarBooleanReadinessWorkload")
    );
    assert!(support.admission_rule().contains("before M7"));

    let applicability = geometry_applicability_matrix();
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::PlanarBooleanReadinessWorkload,
                GeometryRuntimeConcern::BooleanReadinessCertification,
            )
            .expect("boolean readiness row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::PlanarBooleanReadinessWorkload,
                GeometryRuntimeConcern::MutationEvidence,
            )
            .expect("mutation row")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
}

#[test]
fn mb_m6_8_final_boss_outcome_matrix_is_production_owned() {
    run_with_real_workload_stack(|| {
        let (clean_failure_denial, clean_failure_receipt) =
            clean_failure_final_boss("mb-m6-8-clean-failure");
        assert_eq!(
            clean_failure_denial.evidence_digest(),
            clean_failure_receipt.clean_fail_digest()
        );
        let (unsupported_denial, unsupported_receipt) =
            unsupported_final_boss("mb-m6-8-unsupported");
        assert_eq!(
            unsupported_denial.evidence_digest(),
            unsupported_receipt.stage_identity().receipt_identity()
        );
        let (orientation_denial, orientation_diagnostics) =
            orientation_flip_final_boss("mb-m6-8-orientation");
        assert_eq!(
            orientation_denial.evidence_digest(),
            orientation_diagnostics.diagnostic_bundle_digest()
        );

        let matrix = [
            (
                policy_required_final_boss("mb-m6-8-policy"),
                PlanarBooleanReadinessWorkloadDenialKind::PolicyRequired,
                WorthUserOutcomeKind::PolicyRequired,
                Some(WorthUserOutcomeCauseKind::PolicyRequired),
                true,
            ),
            (
                clean_failure_denial,
                PlanarBooleanReadinessWorkloadDenialKind::CleanFailure,
                WorthUserOutcomeKind::NoOptions,
                Some(WorthUserOutcomeCauseKind::DirtyInput),
                false,
            ),
            (
                unsupported_denial,
                PlanarBooleanReadinessWorkloadDenialKind::UnsupportedWorkloadFamily,
                WorthUserOutcomeKind::Unsupported,
                Some(WorthUserOutcomeCauseKind::UnsupportedInput),
                false,
            ),
            (
                predicate_uncertain_final_boss("mb-m6-8-predicate"),
                PlanarBooleanReadinessWorkloadDenialKind::PredicateUncertainty,
                WorthUserOutcomeKind::PredicateUncertain,
                Some(WorthUserOutcomeCauseKind::PredicateUncertain),
                false,
            ),
            (
                mismatched_parity_final_boss("mb-m6-8-parity"),
                PlanarBooleanReadinessWorkloadDenialKind::ProjectionOrParityMismatch,
                WorthUserOutcomeKind::IntegrityMismatch,
                Some(WorthUserOutcomeCauseKind::IntegrityMismatch),
                false,
            ),
            (
                recovery_replay_mismatch_final_boss("mb-m6-8-recovery"),
                PlanarBooleanReadinessWorkloadDenialKind::RecoveryOrReplayMismatch,
                WorthUserOutcomeKind::IntegrityMismatch,
                Some(WorthUserOutcomeCauseKind::IntegrityMismatch),
                false,
            ),
            (
                orientation_denial,
                PlanarBooleanReadinessWorkloadDenialKind::OrientationFlipLocalization,
                WorthUserOutcomeKind::IntegrityMismatch,
                Some(WorthUserOutcomeCauseKind::IntegrityMismatch),
                false,
            ),
        ];

        for (denial, kind, outcome_kind, cause, has_choices) in matrix {
            assert_eq!(denial.kind(), kind);
            assert_human_readable(denial.human_reason());
            let outcome = outcome_for_denial(&denial);
            assert_outcome(&outcome, outcome_kind, cause);
            assert_eq!(!outcome.choices().is_empty(), has_choices);
            assert_human_readable(outcome.human_response().summary());
        }
    });
}

#[test]
fn mb_m6_8_no_kernel_summary_can_substitute_for_readiness_receipts() {
    run_with_real_workload_stack(|| {
        let denial = kernel_summary_substitution_final_boss("mb-m6-8-kernel-summary");
        assert_eq!(
            denial.kind(),
            PlanarBooleanReadinessWorkloadDenialKind::KernelSummarySubstitution
        );
        assert!(denial.human_reason().contains("Kernel summaries"));
        let outcome = outcome_for_denial(&denial);
        assert_outcome(
            &outcome,
            WorthUserOutcomeKind::IntegrityMismatch,
            Some(WorthUserOutcomeCauseKind::IntegrityMismatch),
        );
        assert!(outcome
            .human_response()
            .summary()
            .contains("Kernel summaries"));
    });
}

fn outcome_for_denial(
    denial: &worth_spatial::facade::boolean_readiness_workload::PlanarBooleanReadinessWorkloadDenial,
) -> WorthUserOutcome {
    WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_boolean_readiness_workload_denial(denial),
    )
    .declared("explain final boolean-readiness denial")
    .respond()
    .expect("final readiness denial response")
    .outcome()
    .clone()
}

fn run_with_real_workload_stack(test: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("boolean-readiness-final-boss-mb".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(test)
        .expect("final boss test thread")
        .join()
        .expect("final boss test passed");
}

fn assert_outcome(
    outcome: &WorthUserOutcome,
    kind: WorthUserOutcomeKind,
    cause: Option<WorthUserOutcomeCauseKind>,
) {
    assert_eq!(outcome.kind(), kind);
    assert_eq!(outcome.cause().map(|cause| cause.kind()), cause);
}

fn assert_human_readable(message: &str) {
    assert!(
        message.contains(' ') && !message.contains("::") && !message.contains("-required-"),
        "message must be human-readable, got {message}"
    );
}

use super::support::*;
use crate::facade::{
    BridgeRuntimePolicy, BridgeSubscriptionCertificationComparisonOutcome,
    BridgeSubscriptionCertificationFailureBoundary, BridgeSubscriptionReferenceWorkloadFamilyKind,
    BridgeSubscriptionReferenceWorkloadLaneCoverageRole,
    BridgeSubscriptionReferenceWorkloadLaneKind, BridgeSubscriptionReferenceWorkloadLaneRequest,
    BridgeSubscriptionReferenceWorkloadRejectionKind,
};

fn product_ids() -> Vec<String> {
    (0..128).map(|slot| format!("product-{slot:03}")).collect()
}

fn component_ids() -> Vec<String> {
    ["steel", "rubber", "copper", "glass", "labor"]
        .into_iter()
        .map(str::to_owned)
        .collect()
}

fn lane_ids() -> Vec<String> {
    [
        "authoritative-live",
        "historical-replay",
        "historical-basis-replay",
        "branch-local",
        "shared-fanout",
        "incompatible-sharing-rejection",
        "stale-checkpoint-rejection",
        "restart-resume",
        "continuation",
        "denied-continuation",
        "preview-discard",
        "preview-promotion",
        "hostile-adapter-variation",
        "diagnostics-tier-variation",
        "canonical-ordering-hostility",
        "strategy-lowering-provenance",
        "bundle-insufficiency",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn all_lane_requests() -> Vec<BridgeSubscriptionReferenceWorkloadLaneRequest> {
    vec![
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::HostileAdapterVariation,
            BridgeSubscriptionReferenceWorkloadFamilyKind::CollectionMembership,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::HistoricalReplay,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::HistoricalBasisReplay,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::BranchLocal,
            BridgeSubscriptionReferenceWorkloadFamilyKind::CollectionMembership,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::SharedFanout,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::IncompatibleSharingRejection,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::StaleCheckpointRejection,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::RestartResume,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::Continuation,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::DeniedContinuation,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::PreviewDiscard,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::PreviewPromotion,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::DiagnosticsTierVariation,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::CanonicalOrderingHostility,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::StrategyLoweringProvenance,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
        BridgeSubscriptionReferenceWorkloadLaneRequest::new(
            BridgeSubscriptionReferenceWorkloadLaneKind::BundleInsufficiency,
            BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        ),
    ]
}

#[test]
fn bridge_harness_subscription_suite_35_reference_workload_lanes_are_canonical_and_audited() {
    let bridge = runtime(BridgeRuntimePolicy::development());
    let manifest = bridge
        .declare_subscription_reference_workload_manifest(
            product_ids(),
            component_ids(),
            lane_ids(),
        )
        .expect("reference workload manifest should seal");

    let report = bridge
        .run_subscription_reference_workload(&manifest, all_lane_requests())
        .expect("reference workload should certify from emitted artifacts");

    assert_eq!(report.manifest_digest(), manifest.digest());
    assert_eq!(report.lane_reports().len(), 17);
    assert_eq!(report.comparison_reports().len(), 16);
    assert_eq!(report.counters().reference_workload_lane_count(), 17);
    assert_eq!(report.counters().reference_workload_report_count(), 1);
    assert_eq!(
        report.counters().reference_workload_coverage_report_count(),
        1
    );
    assert_eq!(report.counters().host_log_dependency_count(), 0);
    assert_eq!(report.counters().live_state_dependency_count(), 0);
    assert_eq!(report.offline_audit_report().comparison_report_count(), 16);
    assert_eq!(report.outcome_summary().equivalent_count(), 3);
    assert_eq!(report.outcome_summary().diagnostics_only_count(), 1);
    assert_eq!(report.outcome_summary().expected_rejection_count(), 6);
    assert_eq!(report.outcome_summary().intentionally_divergent_count(), 5);
    assert_eq!(
        report
            .outcome_summary()
            .bundle_completeness_violation_count(),
        1
    );
    assert_eq!(report.outcome_summary().unexpected_rejection_count(), 0);
    assert_eq!(
        report
            .comparison_reports()
            .iter()
            .filter(|comparison| {
                comparison.outcome()
                    == BridgeSubscriptionCertificationComparisonOutcome::RejectedAtExpectedBoundary
            })
            .count(),
        6
    );
    assert_eq!(
        report
            .comparison_reports()
            .iter()
            .filter(|comparison| {
                comparison.outcome()
                    == BridgeSubscriptionCertificationComparisonOutcome::IntentionallyDivergent
            })
            .count(),
        5
    );
    assert_eq!(report.coverage_report().lane_kinds().len(), 17);
    assert_eq!(report.coverage_report().family_kinds().len(), 2);
    assert_eq!(report.coverage_report().lane_coverage_rows().len(), 17);
    assert!(report.coverage_report().first_ship_lane_matrix_covered());
    assert!(report.coverage_report().multi_family_covered());
    assert!(report.coverage_report().comparison_evidence_complete());
    assert!(report.coverage_report().expected_lane_outcomes_covered());
    for (lane_kind, expected_outcome, expected_boundary) in [
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive,
            None,
            None,
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::DiagnosticsTierVariation,
            Some(BridgeSubscriptionCertificationComparisonOutcome::DiagnosticsOnlyDifference),
            Some(BridgeSubscriptionCertificationFailureBoundary::DiagnosticsInfluence),
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::HostileAdapterVariation,
            Some(BridgeSubscriptionCertificationComparisonOutcome::RejectedAtExpectedBoundary),
            Some(BridgeSubscriptionCertificationFailureBoundary::MissingRequiredRetainedArtifact),
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::HistoricalReplay,
            Some(BridgeSubscriptionCertificationComparisonOutcome::RejectedAtExpectedBoundary),
            Some(BridgeSubscriptionCertificationFailureBoundary::ReplayMismatch),
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::HistoricalBasisReplay,
            Some(BridgeSubscriptionCertificationComparisonOutcome::Equivalent),
            None,
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::BranchLocal,
            Some(BridgeSubscriptionCertificationComparisonOutcome::IntentionallyDivergent),
            Some(BridgeSubscriptionCertificationFailureBoundary::DeclarationEquivalenceDrift),
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::SharedFanout,
            Some(BridgeSubscriptionCertificationComparisonOutcome::Equivalent),
            None,
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::IncompatibleSharingRejection,
            Some(BridgeSubscriptionCertificationComparisonOutcome::RejectedAtExpectedBoundary),
            Some(BridgeSubscriptionCertificationFailureBoundary::IllegalSharingReuse),
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::StaleCheckpointRejection,
            Some(BridgeSubscriptionCertificationComparisonOutcome::RejectedAtExpectedBoundary),
            Some(BridgeSubscriptionCertificationFailureBoundary::CheckpointIncompatibility),
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::RestartResume,
            Some(BridgeSubscriptionCertificationComparisonOutcome::RejectedAtExpectedBoundary),
            Some(BridgeSubscriptionCertificationFailureBoundary::ReplayMismatch),
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::Continuation,
            Some(BridgeSubscriptionCertificationComparisonOutcome::IntentionallyDivergent),
            Some(BridgeSubscriptionCertificationFailureBoundary::ContinuationDenialOrAmbiguity),
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::DeniedContinuation,
            Some(BridgeSubscriptionCertificationComparisonOutcome::RejectedAtExpectedBoundary),
            Some(BridgeSubscriptionCertificationFailureBoundary::ContinuationDenialOrAmbiguity),
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::PreviewDiscard,
            Some(BridgeSubscriptionCertificationComparisonOutcome::IntentionallyDivergent),
            Some(BridgeSubscriptionCertificationFailureBoundary::PreviewResidueMismatch),
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::PreviewPromotion,
            Some(BridgeSubscriptionCertificationComparisonOutcome::IntentionallyDivergent),
            Some(BridgeSubscriptionCertificationFailureBoundary::PreviewResidueMismatch),
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::CanonicalOrderingHostility,
            Some(BridgeSubscriptionCertificationComparisonOutcome::Equivalent),
            None,
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::StrategyLoweringProvenance,
            Some(BridgeSubscriptionCertificationComparisonOutcome::IntentionallyDivergent),
            Some(
                BridgeSubscriptionCertificationFailureBoundary::StrategyLoweringProvenanceMismatch,
            ),
        ),
        (
            BridgeSubscriptionReferenceWorkloadLaneKind::BundleInsufficiency,
            Some(BridgeSubscriptionCertificationComparisonOutcome::BundleCompletenessViolation),
            Some(BridgeSubscriptionCertificationFailureBoundary::BundleInsufficiency),
        ),
    ] {
        let row = report
            .coverage_report()
            .lane_coverage_rows()
            .iter()
            .find(|row| row.lane_kind() == lane_kind)
            .expect("each first-ship lane should emit a coverage row");
        assert_eq!(row.expected_outcome(), expected_outcome);
        assert_eq!(row.comparison_outcome(), expected_outcome);
        assert_eq!(row.expected_primary_failure_boundary(), expected_boundary);
        assert_eq!(row.primary_failure_boundary(), expected_boundary);
        assert!(row.matches_expected_evidence());
        assert_eq!(
            row.coverage_role(),
            if lane_kind == BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive {
                BridgeSubscriptionReferenceWorkloadLaneCoverageRole::Control
            } else {
                BridgeSubscriptionReferenceWorkloadLaneCoverageRole::Compared
            }
        );
        assert!(report
            .lane_reports()
            .iter()
            .any(|lane_report| lane_report.digest() == row.lane_report_digest()));
        if row.coverage_role() == BridgeSubscriptionReferenceWorkloadLaneCoverageRole::Compared {
            assert!(report
                .comparison_reports()
                .iter()
                .any(|comparison| { comparison.digest() == row.comparison_report_digest() }));
        } else {
            assert_eq!(
                row.comparison_report_digest(),
                "control-lane-no-comparison-report"
            );
        }
    }
    let authoritative_lane = report
        .lane_reports()
        .iter()
        .find(|lane| {
            lane.lane_kind() == BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive
        })
        .expect("authoritative lane should be present for stale checkpoint comparison");
    let stale_checkpoint_lane = report
        .lane_reports()
        .iter()
        .find(|lane| {
            lane.lane_kind()
                == BridgeSubscriptionReferenceWorkloadLaneKind::StaleCheckpointRejection
        })
        .expect("stale checkpoint lane should be present");
    let stale_checkpoint_row = report
        .coverage_report()
        .lane_coverage_rows()
        .iter()
        .find(|row| {
            row.lane_kind() == BridgeSubscriptionReferenceWorkloadLaneKind::StaleCheckpointRejection
        })
        .expect("stale checkpoint row should be present");
    let stale_checkpoint_comparison = report
        .comparison_reports()
        .iter()
        .find(|comparison| comparison.digest() == stale_checkpoint_row.comparison_report_digest())
        .expect("stale checkpoint row should point at a comparison report");
    assert_eq!(
        stale_checkpoint_comparison.left_bundle_digest(),
        authoritative_lane.certification_bundle_digest()
    );
    assert_eq!(
        stale_checkpoint_comparison.right_bundle_digest(),
        stale_checkpoint_lane.certification_bundle_digest()
    );
    assert_eq!(stale_checkpoint_comparison.mismatch_count(), 1);
    assert_eq!(
        stale_checkpoint_comparison.primary_failure_boundary(),
        Some(BridgeSubscriptionCertificationFailureBoundary::CheckpointIncompatibility)
    );
    assert!(!stale_checkpoint_comparison
        .suppressed_failure_boundaries()
        .contains(&BridgeSubscriptionCertificationFailureBoundary::ReplayMismatch));
    assert!(report.comparison_reports().iter().any(|comparison| {
        comparison.outcome()
            == BridgeSubscriptionCertificationComparisonOutcome::DiagnosticsOnlyDifference
    }));
    assert!(report.comparison_reports().iter().any(|comparison| {
        comparison.outcome()
            == BridgeSubscriptionCertificationComparisonOutcome::RejectedAtExpectedBoundary
    }));
    assert!(report.comparison_reports().iter().any(|comparison| {
        comparison.outcome()
            == BridgeSubscriptionCertificationComparisonOutcome::IntentionallyDivergent
    }));
    let inspection = bridge.inspect_subscription_reference_workload_certification(&report);
    assert_eq!(
        inspection.reference_workload_report_digest(),
        report.digest()
    );
    assert_eq!(inspection.manifest_digest(), manifest.digest());
    assert_eq!(
        inspection.offline_audit_report_digest(),
        report.offline_audit_report().digest()
    );
    assert_eq!(
        inspection.outcome_summary_digest(),
        report.outcome_summary().digest()
    );
    assert_eq!(
        inspection.coverage_report_digest(),
        report.coverage_report().digest()
    );
    assert_eq!(
        inspection.counter_digest(),
        report.counters().digest().as_ref()
    );
    assert_eq!(inspection.lane_report_count(), report.lane_reports().len());
    assert_eq!(
        inspection.comparison_report_count(),
        report.comparison_reports().len()
    );
    assert_eq!(
        inspection.lane_report_digests().len(),
        report.lane_reports().len()
    );
    assert_eq!(
        inspection.comparison_report_digests().len(),
        report.comparison_reports().len()
    );
    for lane_report in report.lane_reports() {
        assert!(
            inspection
                .lane_report_digests()
                .iter()
                .any(|digest| digest.as_ref() == lane_report.digest()),
            "inspection should retain every emitted lane report digest"
        );
    }
    for comparison_report in report.comparison_reports() {
        assert!(
            inspection
                .comparison_report_digests()
                .iter()
                .any(|digest| digest.as_ref() == comparison_report.digest()),
            "inspection should retain every emitted comparison report digest"
        );
    }
    assert_eq!(inspection.host_log_dependency_count(), 0);
    assert_eq!(inspection.live_state_dependency_count(), 0);

    let reordered = bridge
        .run_subscription_reference_workload(&manifest, {
            let mut requests = all_lane_requests();
            requests.reverse();
            requests
        })
        .expect("reference workload lane order should canonicalize");
    assert_eq!(report.digest(), reordered.digest());
}

#[test]
fn bridge_harness_subscription_suite_36_reference_workload_requires_declared_lanes() {
    let bridge = runtime(BridgeRuntimePolicy::development());
    let manifest = bridge
        .declare_subscription_reference_workload_manifest(
            product_ids(),
            component_ids(),
            vec!["authoritative-live".to_owned()],
        )
        .expect("single-lane manifest should seal before workload admission");

    let insufficient = bridge
        .run_subscription_reference_workload(
            &manifest,
            vec![BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive,
                BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
            )],
        )
        .expect_err("reference workload certification requires at least two lanes");
    assert_eq!(
        insufficient.rejection_kind(),
        BridgeSubscriptionReferenceWorkloadRejectionKind::InsufficientLaneSet
    );

    let duplicate_only = bridge
        .run_subscription_reference_workload(
            &manifest,
            vec![
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
                ),
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
                ),
            ],
        )
        .expect_err("duplicate lane requests must not satisfy cross-lane certification");
    assert_eq!(
        duplicate_only.rejection_kind(),
        BridgeSubscriptionReferenceWorkloadRejectionKind::InsufficientLaneSet
    );

    let undeclared = bridge
        .run_subscription_reference_workload(
            &manifest,
            vec![
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
                ),
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::DiagnosticsTierVariation,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
                ),
            ],
        )
        .expect_err("requested lanes must be present in the sealed manifest");
    assert_eq!(
        undeclared.rejection_kind(),
        BridgeSubscriptionReferenceWorkloadRejectionKind::LaneNotDeclaredByManifest
    );
    assert_eq!(
        undeclared.lane_kind(),
        Some(BridgeSubscriptionReferenceWorkloadLaneKind::DiagnosticsTierVariation)
    );

    let no_control_manifest = bridge
        .declare_subscription_reference_workload_manifest(
            product_ids(),
            component_ids(),
            vec![
                "diagnostics-tier-variation".to_owned(),
                "hostile-adapter-variation".to_owned(),
            ],
        )
        .expect("non-control lanes can be declared, but cannot certify alone");
    let no_control = bridge
        .run_subscription_reference_workload(
            &no_control_manifest,
            vec![
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::DiagnosticsTierVariation,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
                ),
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::HostileAdapterVariation,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
                ),
            ],
        )
        .expect_err("reference workload certification requires authoritative control");
    assert_eq!(
        no_control.rejection_kind(),
        BridgeSubscriptionReferenceWorkloadRejectionKind::MissingAuthoritativeControlLane
    );
}

#[test]
fn bridge_harness_subscription_suite_37_reference_workload_keeps_family_strategy_distinct() {
    let bridge = runtime(BridgeRuntimePolicy::development());
    let manifest = bridge
        .declare_subscription_reference_workload_manifest(
            product_ids(),
            component_ids(),
            lane_ids(),
        )
        .expect("reference workload manifest should seal");

    let report = bridge
        .run_subscription_reference_workload(
            &manifest,
            vec![
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
                ),
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::DiagnosticsTierVariation,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
                ),
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::HostileAdapterVariation,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::CollectionMembership,
                ),
            ],
        )
        .expect("reference workload should certify mixed families");

    let detail_lane = report
        .lane_reports()
        .iter()
        .find(|lane| {
            lane.lane_kind() == BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive
        })
        .expect("authoritative detail lane should exist");
    let collection_lane = report
        .lane_reports()
        .iter()
        .find(|lane| {
            lane.lane_kind() == BridgeSubscriptionReferenceWorkloadLaneKind::HostileAdapterVariation
        })
        .expect("collection hostile lane should exist");

    assert_eq!(
        detail_lane.family_kind(),
        BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact
    );
    assert_eq!(
        collection_lane.family_kind(),
        BridgeSubscriptionReferenceWorkloadFamilyKind::CollectionMembership
    );
    assert_ne!(
        detail_lane.certification_bundle_digest(),
        collection_lane.certification_bundle_digest()
    );
    assert_eq!(report.counters().global_history_scan_count(), 0);
    assert_eq!(report.counters().global_subscription_scan_count(), 0);
}

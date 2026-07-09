use super::*;

#[test]
fn bridge_harness_subscription_phase_17_sufficiency_requires_full_lane_coverage() {
    let bridge = runtime(BridgeRuntimePolicy::development());
    let manifest = bridge
        .declare_subscription_reference_workload_manifest(
            product_ids(),
            component_ids(),
            lane_ids(),
        )
        .expect("reference workload manifest should seal");

    let partial_declaration = bridge
        .plan_subscription_reference_workload(
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
        .expect("partial lane plans are still valid for broad report assembly");
    let partial_lane_artifacts = bridge
        .admit_subscription_reference_workload_lane_artifacts(&manifest, &partial_declaration)
        .expect("partial lane artifacts should still admit");
    let partial_rejection = bridge
        .prove_subscription_reference_workload_coverage(partial_lane_artifacts)
        .expect_err("phase 17 sufficiency must reject incomplete lane families");
    assert_eq!(
        partial_rejection.rejection_kind(),
        BridgeSubscriptionReferenceWorkloadRejectionKind::CoverageProofRejected
    );

    let declaration = bridge
        .plan_subscription_reference_workload(&manifest, all_lane_requests())
        .expect("full lane set should admit");
    let lane_artifacts = bridge
        .admit_subscription_reference_workload_lane_artifacts(&manifest, &declaration)
        .expect("full lane artifacts should admit");
    let broad_report = bridge
        .run_subscription_reference_workload(&manifest, all_lane_requests())
        .expect("broad report surface should remain available for partial audits");
    let coverage = bridge
        .prove_subscription_reference_workload_coverage(lane_artifacts.clone())
        .expect("full lane set should prove phase 17 coverage");
    let sufficiency = bridge.seal_subscription_reference_workload_sufficiency(
        &manifest,
        &declaration,
        lane_artifacts,
        &coverage,
        "suite-35-37-phase-17-fixture",
    );
    let report = sufficiency.report();

    assert!(report.coverage_report().required_phase_17_facets_covered());
    assert!(report.coverage_report().required_hostile_lane_set_covered());
    assert_ne!(sufficiency.digest(), broad_report.digest());
    assert_ne!(
        report.fixture_evidence_digest(),
        broad_report.fixture_evidence_digest()
    );
    assert_eq!(
        report.coverage_report().covered_required_facets(),
        BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet::all()
    );
}

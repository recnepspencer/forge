use super::support::*;

#[test]
fn certification_bundle_insufficiency_is_typed_without_semantic_drift() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let report = runtime.certify_subscription_certification_bundle_insufficiency();

    assert_ne!(
        report.complete_bundle_digest(),
        report.insufficient_bundle_digest()
    );
    assert_ne!(
        report.complete_completeness_report_digest(),
        report.insufficient_completeness_report_digest()
    );
    assert_eq!(
        report.primary_failure_boundary(),
        crate::facade::BridgeSubscriptionCertificationFailureBoundary::BundleInsufficiency
    );
    assert_eq!(
        report.primary_failure_precedence_stage(),
        crate::facade::BridgeSubscriptionCertificationFailurePrecedenceStage::RetainedArtifactCompleteness
    );
    assert!(report.insufficiency_is_primary_without_semantic_drift());
    assert_eq!(report.counters().bundle_comparison_mismatch_count(), 1);
    assert_eq!(report.counters().failure_localization_count(), 1);
    assert_eq!(report.counters().bundle_insufficiency_report_count(), 1);
}

#[test]
fn certification_historical_basis_rejects_latest_fallback() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let report = runtime.certify_subscription_certification_historical_basis();

    assert_ne!(
        report.retained_basis_bundle_digest(),
        report.latest_fallback_bundle_digest()
    );
    assert_eq!(
        report.primary_failure_boundary(),
        crate::facade::BridgeSubscriptionCertificationFailureBoundary::BasisDrift
    );
    assert_eq!(report.latest_truth_fallback_count(), 0);
    assert!(report.retained_basis_is_explicit());
    assert_eq!(report.counters().historical_basis_report_count(), 1);
    assert_eq!(report.counters().global_history_scan_count(), 0);
    assert_eq!(report.counters().global_subscription_scan_count(), 0);
}

#[test]
fn certification_strategy_lowering_provenance_is_family_visible() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let report = runtime.certify_subscription_certification_strategy_lowering();

    assert_ne!(
        report.detail_bundle_digest(),
        report.collection_bundle_digest()
    );
    assert_eq!(
        report.primary_failure_boundary(),
        crate::facade::BridgeSubscriptionCertificationFailureBoundary::StrategyLoweringProvenanceMismatch
    );
    assert!(report.strategy_lowering_is_distinct_without_signal_rediscovery());
    assert_eq!(report.counters().strategy_lowering_report_count(), 1);
    assert_eq!(report.counters().global_history_scan_count(), 0);
    assert_eq!(report.counters().global_subscription_scan_count(), 0);
}

#[test]
fn certification_fanout_splits_equivalence_from_illegal_sharing() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let report = runtime.certify_subscription_certification_fanout();

    assert!(report.shared_fanout_equivalent());
    assert!(report.incompatible_sharing_rejected_before_delivery());
    assert_ne!(
        report.shared_equivalence_report_digest(),
        report.incompatible_rejection_report_digest()
    );
    assert_eq!(report.counters().bundle_comparison_count(), 2);
    assert_eq!(report.counters().fanout_report_count(), 1);
}

#[test]
fn certification_denied_continuation_stops_before_delivery_drift() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let report = runtime.certify_subscription_certification_denied_continuation();

    assert_ne!(
        report.admitted_bundle_digest(),
        report.denied_bundle_digest()
    );
    assert_eq!(
        report.primary_failure_boundary(),
        crate::facade::BridgeSubscriptionCertificationFailureBoundary::ContinuationDenialOrAmbiguity
    );
    assert_eq!(
        report.primary_failure_precedence_stage(),
        crate::facade::BridgeSubscriptionCertificationFailurePrecedenceStage::ContinuationOrBranchScope
    );
    assert!(report.denied_before_delivery_drift());
    assert_eq!(report.counters().denied_continuation_report_count(), 1);
}

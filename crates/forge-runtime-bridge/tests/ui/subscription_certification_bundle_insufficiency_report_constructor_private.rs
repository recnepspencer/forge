use forge_runtime_bridge::facade::{
    BridgeSubscriptionCertificationBundleInsufficiencyReport,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationFailurePrecedenceStage,
};

fn main() {
    let _report = BridgeSubscriptionCertificationBundleInsufficiencyReport {
        complete_bundle_digest: "complete".into(),
        insufficient_bundle_digest: "insufficient".into(),
        complete_completeness_report_digest: "complete-completeness".into(),
        insufficient_completeness_report_digest: "insufficient-completeness".into(),
        comparison_report_digest: "comparison".into(),
        primary_failure_boundary: BridgeSubscriptionCertificationFailureBoundary::BundleInsufficiency,
        primary_failure_precedence_stage:
            BridgeSubscriptionCertificationFailurePrecedenceStage::RetainedArtifactCompleteness,
        insufficiency_is_primary_without_semantic_drift: true,
        counters: BridgeSubscriptionCertificationCounterSnapshot::default(),
        canonical_basis: "basis".into(),
        digest: "digest".into(),
    };
}

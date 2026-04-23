use forge_runtime_bridge::facade::{
    BridgeSubscriptionCertificationCounterSnapshot,
    BridgeSubscriptionCertificationDeniedContinuationReport,
    BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationFailurePrecedenceStage,
};

fn main() {
    let _report = BridgeSubscriptionCertificationDeniedContinuationReport {
        admitted_bundle_digest: "admitted".into(),
        denied_bundle_digest: "denied".into(),
        comparison_report_digest: "comparison".into(),
        primary_failure_boundary:
            BridgeSubscriptionCertificationFailureBoundary::ContinuationDenialOrAmbiguity,
        primary_failure_precedence_stage:
            BridgeSubscriptionCertificationFailurePrecedenceStage::ContinuationOrBranchScope,
        denied_before_delivery_drift: true,
        counters: BridgeSubscriptionCertificationCounterSnapshot::default(),
        canonical_basis: "basis".into(),
        digest: "digest".into(),
    };
}

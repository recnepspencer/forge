use forge_runtime_bridge::facade::{
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationFailurePrecedenceStage,
    BridgeSubscriptionCertificationHistoricalBasisReport,
};

fn main() {
    let _report = BridgeSubscriptionCertificationHistoricalBasisReport {
        retained_basis_bundle_digest: "retained".into(),
        latest_fallback_bundle_digest: "latest".into(),
        comparison_report_digest: "comparison".into(),
        primary_failure_boundary: BridgeSubscriptionCertificationFailureBoundary::BasisDrift,
        primary_failure_precedence_stage:
            BridgeSubscriptionCertificationFailurePrecedenceStage::BasisBinding,
        retained_basis_is_explicit: true,
        latest_truth_fallback_count: 0,
        counters: BridgeSubscriptionCertificationCounterSnapshot::default(),
        canonical_basis: "basis".into(),
        digest: "digest".into(),
    };
}

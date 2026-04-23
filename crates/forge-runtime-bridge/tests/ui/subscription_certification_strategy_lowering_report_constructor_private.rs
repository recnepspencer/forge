use forge_runtime_bridge::facade::{
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationFailurePrecedenceStage,
    BridgeSubscriptionCertificationStrategyLoweringReport,
};

fn main() {
    let _report = BridgeSubscriptionCertificationStrategyLoweringReport {
        detail_bundle_digest: "detail".into(),
        collection_bundle_digest: "collection".into(),
        comparison_report_digest: "comparison".into(),
        primary_failure_boundary:
            BridgeSubscriptionCertificationFailureBoundary::StrategyLoweringProvenanceMismatch,
        primary_failure_precedence_stage:
            BridgeSubscriptionCertificationFailurePrecedenceStage::StrategyLowering,
        strategy_lowering_is_distinct_without_signal_rediscovery: true,
        counters: BridgeSubscriptionCertificationCounterSnapshot::default(),
        canonical_basis: "basis".into(),
        digest: "digest".into(),
    };
}

use forge_runtime_bridge::facade::{
    BridgeSubscriptionCertificationComparisonOutcome,
    BridgeSubscriptionCertificationCounterSnapshot,
    BridgeSubscriptionCertificationOrderingHostilityReport,
};

fn main() {
    let _report = BridgeSubscriptionCertificationOrderingHostilityReport {
        control_source_artifact_index_digest: "control-index".into(),
        hostile_source_artifact_index_digest: "hostile-index".into(),
        control_bundle_digest: "control-bundle".into(),
        hostile_bundle_digest: "hostile-bundle".into(),
        comparison_report_digest: "comparison".into(),
        comparison_outcome: BridgeSubscriptionCertificationComparisonOutcome::Equivalent,
        canonical_source_order_preserved: true,
        semantic_digest_preserved: true,
        sealed_bundle_digest_preserved: true,
        field_order_preserved: true,
        counters: BridgeSubscriptionCertificationCounterSnapshot::default(),
        canonical_basis: "basis".into(),
        digest: "digest".into(),
    };
}

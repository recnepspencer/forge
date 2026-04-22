use forge_runtime_bridge::facade::{
    BridgeSubscriptionOfflineAuditOutcome, BridgeSubscriptionOfflineAuditReport,
};

fn main() {
    let _report = BridgeSubscriptionOfflineAuditReport {
        outcome: BridgeSubscriptionOfflineAuditOutcome::DiagnosedOffline,
        bundle_index_digest: "forged-index".into(),
        comparison_report_count: 0,
        counters: Default::default(),
        canonical_basis: "forged-basis".into(),
        digest: "forged-digest".into(),
    };
}

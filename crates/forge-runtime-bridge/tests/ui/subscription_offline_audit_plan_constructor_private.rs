use forge_runtime_bridge::facade::BridgeSubscriptionOfflineAuditPlan;

fn main() {
    let _plan = BridgeSubscriptionOfflineAuditPlan {
        bundle_index_digest: "forged-index".into(),
        comparison_report_digests: Vec::new(),
        counters: Default::default(),
        canonical_basis: "forged-basis".into(),
        digest: "forged-digest".into(),
    };
}

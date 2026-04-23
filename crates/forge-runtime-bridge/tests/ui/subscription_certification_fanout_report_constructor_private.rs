use forge_runtime_bridge::facade::{
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationFanoutReport,
};

fn main() {
    let _report = BridgeSubscriptionCertificationFanoutReport {
        shared_equivalence_report_digest: "shared".into(),
        incompatible_rejection_report_digest: "incompatible".into(),
        shared_fanout_equivalent: true,
        incompatible_sharing_rejected_before_delivery: true,
        counters: BridgeSubscriptionCertificationCounterSnapshot::default(),
        canonical_basis: "basis".into(),
        digest: "digest".into(),
    };
}

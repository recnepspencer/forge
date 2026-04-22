use forge_runtime_bridge::facade::{
    BridgeSubscriptionCertificationCostPostureReport,
    BridgeSubscriptionCertificationCounterSnapshot,
};
use std::sync::Arc;

fn main() {
    let _report = BridgeSubscriptionCertificationCostPostureReport {
        sparse_cost_profile_digest: Arc::from("sparse"),
        dense_cost_profile_digest: Arc::from("dense"),
        over_budget_rejection_digest: Arc::from("over-budget"),
        first_scratch_digest: Arc::from("scratch-1"),
        repeated_scratch_digest: Arc::from("scratch-2"),
        dense_selected_before_assembly: true,
        over_budget_rejected_before_assembly: true,
        scratch_lifecycle_reuse_visible: true,
        counters: BridgeSubscriptionCertificationCounterSnapshot::default(),
        canonical_basis: Arc::from("basis"),
        digest: Arc::from("digest"),
    };
}

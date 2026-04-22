use forge_runtime_bridge::facade::{
    BridgeSubscriptionCertificationCounterSnapshot,
    BridgeSubscriptionReferenceWorkloadCoverageReport,
};
use std::sync::Arc;

fn main() {
    let _coverage = BridgeSubscriptionReferenceWorkloadCoverageReport {
        lane_kinds: Vec::new(),
        family_kinds: Vec::new(),
        lane_coverage_rows: Vec::new(),
        first_ship_lane_matrix_covered: true,
        multi_family_covered: true,
        comparison_evidence_complete: true,
        expected_lane_outcomes_covered: true,
        counters: BridgeSubscriptionCertificationCounterSnapshot::default(),
        canonical_basis: Arc::from("basis"),
        digest: Arc::from("digest"),
    };
}

use forge_runtime_bridge::facade::{
    BridgeSubscriptionReferenceWorkloadFamilyKind, BridgeSubscriptionReferenceWorkloadLaneCoverageRole,
    BridgeSubscriptionReferenceWorkloadLaneCoverageRow, BridgeSubscriptionReferenceWorkloadLaneKind,
};
use std::sync::Arc;

fn main() {
    let _row = BridgeSubscriptionReferenceWorkloadLaneCoverageRow {
        lane_kind: BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive,
        family_kind: BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        coverage_role: BridgeSubscriptionReferenceWorkloadLaneCoverageRole::Control,
        lane_report_digest: Arc::from("lane"),
        comparison_report_digest: Arc::from("comparison"),
        comparison_outcome: None,
        primary_failure_boundary: None,
        expected_outcome: None,
        expected_primary_failure_boundary: None,
        matches_expected_evidence: true,
        canonical_basis: Arc::from("basis"),
        digest: Arc::from("digest"),
    };
}

use forge_runtime_bridge::facade::{
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionReferenceWorkloadFamilyKind,
    BridgeSubscriptionReferenceWorkloadLaneKind, BridgeSubscriptionReferenceWorkloadLaneReport,
};
use std::sync::Arc;

fn main() {
    let _report = BridgeSubscriptionReferenceWorkloadLaneReport {
        lane_kind: BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive,
        family_kind: BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
        source_artifact_index_digest: Arc::from("source-index"),
        certification_bundle_digest: Arc::from("bundle"),
        counters: BridgeSubscriptionCertificationCounterSnapshot::default(),
        canonical_basis: Arc::from("basis"),
        digest: Arc::from("digest"),
    };
}

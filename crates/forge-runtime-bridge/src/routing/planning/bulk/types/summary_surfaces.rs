#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedBridgeWorkloadSummary {
    workload_identity: BridgeWorkloadIdentity,
    route_count: usize,
    invalidation_target_count: usize,
    subscription_slice_count: usize,
    snapshot_read_count: usize,
    truth_view_member_count: usize,
    continuity_member_count: usize,
    branch_scope_count: usize,
    snapshot_scope_count: usize,
    counters: BridgeBulkPlanningCounters,
    digest: Arc<str>,
}

impl NormalizedBridgeWorkloadSummary {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        route_count: usize,
        invalidation_target_count: usize,
        subscription_slice_count: usize,
        snapshot_read_count: usize,
        truth_view_member_count: usize,
        continuity_member_count: usize,
        branch_scope_count: usize,
        snapshot_scope_count: usize,
        counters: BridgeBulkPlanningCounters,
    ) -> Self {
        let basis = format!(
            "normalized-bridge-workload-summary|workload={}|route-count={}|invalidation-target-count={}|subscription-slice-count={}|snapshot-read-count={}|truth-view-member-count={}|continuity-member-count={}|branch-scope-count={}|snapshot-scope-count={}|counter-digest={}{}{}{}{}{}",
            workload_identity.as_str(),
            route_count,
            invalidation_target_count,
            subscription_slice_count,
            snapshot_read_count,
            truth_view_member_count,
            continuity_member_count,
            branch_scope_count,
            snapshot_scope_count,
            counters.bulk_workload_count(),
            counters.bulk_routed_item_count(),
            counters.bulk_packet_count(),
            counters.bulk_packet_entry_count(),
            counters.bulk_reduction_input_count(),
            counters.bulk_reduction_output_count(),
        );
        Self {
            workload_identity,
            route_count,
            invalidation_target_count,
            subscription_slice_count,
            snapshot_read_count,
            truth_view_member_count,
            continuity_member_count,
            branch_scope_count,
            snapshot_scope_count,
            counters,
            digest: digest_string("normalized-bridge-workload-summary", &basis),
        }
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity { &self.workload_identity }
    pub fn route_count(&self) -> usize { self.route_count }
    pub fn invalidation_target_count(&self) -> usize { self.invalidation_target_count }
    pub fn subscription_slice_count(&self) -> usize { self.subscription_slice_count }
    pub fn snapshot_read_count(&self) -> usize { self.snapshot_read_count }
    pub fn truth_view_member_count(&self) -> usize { self.truth_view_member_count }
    pub fn continuity_member_count(&self) -> usize { self.continuity_member_count }
    pub fn branch_scope_count(&self) -> usize { self.branch_scope_count }
    pub fn snapshot_scope_count(&self) -> usize { self.snapshot_scope_count }
    pub fn counters(&self) -> &BridgeBulkPlanningCounters { &self.counters }
    pub fn digest(&self) -> &str { self.digest.as_ref() }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBulkPlanningSummary {
    workload_identity: BridgeWorkloadIdentity,
    route_count: usize,
    invalidation_target_count: usize,
    subscription_slice_count: usize,
    snapshot_read_count: usize,
    digest: Arc<str>,
}

impl BridgeBulkPlanningSummary {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        route_count: usize,
        invalidation_target_count: usize,
        subscription_slice_count: usize,
        snapshot_read_count: usize,
    ) -> Self {
        let basis = format!(
            "bulk-planning-summary|workload={}|route-count={}|invalidation-target-count={}|subscription-slice-count={}|snapshot-read-count={}",
            workload_identity.as_str(),
            route_count,
            invalidation_target_count,
            subscription_slice_count,
            snapshot_read_count,
        );
        Self {
            workload_identity,
            route_count,
            invalidation_target_count,
            subscription_slice_count,
            snapshot_read_count,
            digest: digest_string("bulk-planning-summary", &basis),
        }
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity {
        &self.workload_identity
    }

    pub fn route_count(&self) -> usize {
        self.route_count
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.invalidation_target_count
    }

    pub fn subscription_slice_count(&self) -> usize {
        self.subscription_slice_count
    }

    pub fn snapshot_read_count(&self) -> usize {
        self.snapshot_read_count
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
use super::*;

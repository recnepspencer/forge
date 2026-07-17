use crate::HazardLeaseCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReclaimCounterSnapshot {
    executed_reachability_inputs: u64,
    candidate_ranges: u64,
    blocked_reclaims: u64,
    eligible_reclaims: u64,
    live_hazard_entries: u64,
    indexed_epoch_buckets_touched: u64,
    indexed_range_buckets_touched: u64,
    indexed_hazard_entries_touched: u64,
    hazard_lookup_ranges: u64,
    range_comparisons: u64,
    overlapping_ranges: u64,
    active_backup_leases: u64,
    backup_artifacts_examined: u64,
    backup_overlapping_artifacts: u64,
}

pub(crate) struct ReclaimCounterInputs {
    pub(crate) candidate_ranges: u64,
    pub(crate) live_hazard_entries: u64,
    pub(crate) indexed_epoch_buckets_touched: u64,
    pub(crate) indexed_hazard_entries_touched: u64,
    pub(crate) hazard_counters: HazardLeaseCounterSnapshot,
    pub(crate) active_backup_leases: u64,
    pub(crate) backup_artifacts_examined: u64,
    pub(crate) backup_overlapping_artifacts: u64,
}

impl ReclaimCounterSnapshot {
    pub(crate) const fn from_inputs(inputs: ReclaimCounterInputs) -> Self {
        Self {
            executed_reachability_inputs: 1,
            candidate_ranges: inputs.candidate_ranges,
            blocked_reclaims: 0,
            eligible_reclaims: 0,
            live_hazard_entries: inputs.live_hazard_entries,
            indexed_epoch_buckets_touched: inputs.indexed_epoch_buckets_touched,
            indexed_range_buckets_touched: inputs.hazard_counters.range_bucket_lookups(),
            indexed_hazard_entries_touched: inputs.indexed_hazard_entries_touched,
            hazard_lookup_ranges: inputs.hazard_counters.live_lookup_ranges(),
            range_comparisons: inputs.hazard_counters.range_comparisons(),
            overlapping_ranges: inputs.hazard_counters.overlapping_ranges(),
            active_backup_leases: inputs.active_backup_leases,
            backup_artifacts_examined: inputs.backup_artifacts_examined,
            backup_overlapping_artifacts: inputs.backup_overlapping_artifacts,
        }
    }

    pub(crate) const fn with_blocked_reclaim(mut self) -> Self {
        self.blocked_reclaims += 1;
        self
    }

    pub(crate) const fn with_eligible_reclaim(mut self) -> Self {
        self.eligible_reclaims += 1;
        self
    }

    pub const fn executed_reachability_inputs(self) -> u64 {
        self.executed_reachability_inputs
    }

    pub const fn candidate_ranges(self) -> u64 {
        self.candidate_ranges
    }

    pub const fn blocked_reclaims(self) -> u64 {
        self.blocked_reclaims
    }

    pub const fn eligible_reclaims(self) -> u64 {
        self.eligible_reclaims
    }

    pub const fn live_hazard_entries(self) -> u64 {
        self.live_hazard_entries
    }

    pub const fn indexed_epoch_buckets_touched(self) -> u64 {
        self.indexed_epoch_buckets_touched
    }

    pub const fn indexed_range_buckets_touched(self) -> u64 {
        self.indexed_range_buckets_touched
    }

    pub const fn indexed_hazard_entries_touched(self) -> u64 {
        self.indexed_hazard_entries_touched
    }

    pub const fn hazard_lookup_ranges(self) -> u64 {
        self.hazard_lookup_ranges
    }

    pub const fn range_comparisons(self) -> u64 {
        self.range_comparisons
    }

    pub const fn overlapping_ranges(self) -> u64 {
        self.overlapping_ranges
    }

    pub const fn active_backup_leases(self) -> u64 {
        self.active_backup_leases
    }

    pub const fn backup_artifacts_examined(self) -> u64 {
        self.backup_artifacts_examined
    }

    pub const fn backup_overlapping_artifacts(self) -> u64 {
        self.backup_overlapping_artifacts
    }
}

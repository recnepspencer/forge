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
}

impl ReclaimCounterSnapshot {
    pub(crate) const fn from_inputs(
        candidate_ranges: u64,
        live_hazard_entries: u64,
        indexed_epoch_buckets_touched: u64,
        indexed_hazard_entries_touched: u64,
        hazard_counters: HazardLeaseCounterSnapshot,
    ) -> Self {
        Self {
            executed_reachability_inputs: 1,
            candidate_ranges,
            blocked_reclaims: 0,
            eligible_reclaims: 0,
            live_hazard_entries,
            indexed_epoch_buckets_touched,
            indexed_range_buckets_touched: hazard_counters.range_bucket_lookups(),
            indexed_hazard_entries_touched,
            hazard_lookup_ranges: hazard_counters.live_lookup_ranges(),
            range_comparisons: hazard_counters.range_comparisons(),
            overlapping_ranges: hazard_counters.overlapping_ranges(),
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
}

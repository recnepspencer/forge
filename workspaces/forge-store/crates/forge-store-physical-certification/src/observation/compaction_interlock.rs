use forge_store_physical_isolation::CompactionInterlockFoundationalEvidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactionInterlockObservation {
    no_mixed_root: bool,
    old_reader_retained_old_structure: bool,
    new_reader_observed_new_epoch: bool,
    blocked_reclaim_until_release: bool,
    protected_ranges: u64,
    candidate_ranges: u64,
    range_comparisons: u64,
    overlapping_ranges: u64,
    copied_pages: u64,
    publication_swaps: u64,
    blocked_reclaims: u64,
}

impl CompactionInterlockObservation {
    pub fn from_store_interlock_evidence(
        evidence: CompactionInterlockFoundationalEvidence,
    ) -> Option<Self> {
        if !evidence.materialized_after_store_decision() {
            return None;
        }
        let counters = evidence.counters();
        if counters.candidate_ranges() == 0
            || counters.copied_pages() == 0
            || counters.publication_swaps() == 0
        {
            return None;
        }
        Some(Self {
            no_mixed_root: evidence.no_mixed_root(),
            old_reader_retained_old_structure: evidence.old_reader_retained_old_structure(),
            new_reader_observed_new_epoch: evidence.new_reader_observed_new_epoch(),
            blocked_reclaim_until_release: evidence.blocked_reclaim_until_release(),
            protected_ranges: counters.protected_ranges(),
            candidate_ranges: counters.candidate_ranges(),
            range_comparisons: counters.range_comparisons(),
            overlapping_ranges: counters.overlapping_ranges(),
            copied_pages: counters.copied_pages(),
            publication_swaps: counters.publication_swaps(),
            blocked_reclaims: counters.blocked_reclaims(),
        })
    }

    pub const fn no_mixed_root(self) -> bool {
        self.no_mixed_root
    }

    pub const fn old_reader_retained_old_structure(self) -> bool {
        self.old_reader_retained_old_structure
    }

    pub const fn new_reader_observed_new_epoch(self) -> bool {
        self.new_reader_observed_new_epoch
    }

    pub const fn blocked_reclaim_until_release(self) -> bool {
        self.blocked_reclaim_until_release
    }

    pub const fn protected_ranges(self) -> u64 {
        self.protected_ranges
    }

    pub const fn candidate_ranges(self) -> u64 {
        self.candidate_ranges
    }

    pub const fn range_comparisons(self) -> u64 {
        self.range_comparisons
    }

    pub const fn overlapping_ranges(self) -> u64 {
        self.overlapping_ranges
    }

    pub const fn copied_pages(self) -> u64 {
        self.copied_pages
    }

    pub const fn publication_swaps(self) -> u64 {
        self.publication_swaps
    }

    pub const fn blocked_reclaims(self) -> u64 {
        self.blocked_reclaims
    }
}

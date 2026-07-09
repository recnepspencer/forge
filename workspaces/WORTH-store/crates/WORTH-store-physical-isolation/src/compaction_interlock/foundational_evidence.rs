use super::{
    CompactionReadInterlockCounters, DrainedCompactionReclaim, ReadDuringCompactionVerdict,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactionInterlockFoundationalEvidence {
    counters: CompactionReadInterlockCounters,
    materialized_after_store_decision: bool,
    no_mixed_root: bool,
    old_reader_retained_old_structure: bool,
    new_reader_observed_new_epoch: bool,
    blocked_reclaim_until_release: bool,
}

impl CompactionInterlockFoundationalEvidence {
    pub(crate) const fn after_store_decision(counters: CompactionReadInterlockCounters) -> Self {
        Self {
            counters,
            materialized_after_store_decision: true,
            no_mixed_root: false,
            old_reader_retained_old_structure: false,
            new_reader_observed_new_epoch: false,
            blocked_reclaim_until_release: false,
        }
    }

    pub fn after_executed_interlock(
        verdict: &ReadDuringCompactionVerdict,
        reclaim: &DrainedCompactionReclaim,
    ) -> Self {
        let proof = verdict.proof();
        Self {
            counters: reclaim.counters(),
            materialized_after_store_decision: true,
            no_mixed_root: proof.pre_cutover_root().epoch() != proof.post_cutover_root().epoch(),
            old_reader_retained_old_structure: verdict.pre_cutover_reader_retained_old_structure(),
            new_reader_observed_new_epoch: verdict.post_cutover_reader_observed_new_epoch(),
            blocked_reclaim_until_release: reclaim.counters().blocked_reclaims() > 0
                && reclaim.released().footprint_basis()
                    == verdict
                        .pre_cutover_read()
                        .read_plan_release()
                        .footprint_basis(),
        }
    }

    pub const fn counters(self) -> CompactionReadInterlockCounters {
        self.counters
    }

    pub const fn materialized_after_store_decision(self) -> bool {
        self.materialized_after_store_decision
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
}

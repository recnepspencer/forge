use std::time::Instant;

use super::super::signal_graph::SignalGraph;

impl SignalGraph {
    pub fn tombstone_count(&self) -> u32 {
        self.compaction.tombstone_count
    }

    pub fn gc_threshold(&self) -> u32 {
        self.compaction.gc_threshold
    }

    pub fn run_gc_epoch(&mut self) {
        let Some(epoch_plan) = self.plan_compaction_epoch() else {
            return;
        };
        let gc_start = Instant::now();
        for family in epoch_plan.families() {
            self.compact_storage_family(family);
        }
        self.complete_compaction_epoch(epoch_plan.family_budget());
        self.telemetry.gc_epoch_count += 1;
        self.telemetry.gc_epoch_nanos += gc_start.elapsed().as_nanos();
    }

    #[cfg(test)]
    pub(crate) fn gc_compaction_debt_for_test(&self) -> u32 {
        self.compaction.debt
    }

    #[cfg(test)]
    pub(crate) fn gc_compaction_cursor_for_test(&self) -> u8 {
        self.compaction.cursor
    }

    #[cfg(test)]
    pub(crate) fn set_gc_compaction_state_for_test(&mut self, debt: u32, cursor: u8) {
        self.compaction.debt = debt;
        self.compaction.cursor = cursor % 3;
    }
}

use std::time::Instant;

use super::signal_graph::SignalGraph;

impl SignalGraph {
    pub fn tombstone_count(&self) -> u32 {
        self.tombstone_count
    }

    pub fn gc_threshold(&self) -> u32 {
        self.gc_threshold
    }

    pub fn run_gc_epoch(&mut self) {
        if self.gc_compaction_debt == 0 && !self.should_gc() {
            return;
        }
        let gc_start = Instant::now();
        let families_to_compact = self.gc_compaction_budget();
        for _ in 0..families_to_compact {
            self.compact_next_graph_storage_family();
        }
        self.tombstone_count = 0;
        self.gc_compaction_debt = self
            .gc_compaction_debt
            .saturating_sub(families_to_compact as u32);
        self.telemetry.gc_epoch_count += 1;
        self.telemetry.gc_epoch_nanos += gc_start.elapsed().as_nanos();
    }

    pub fn should_gc(&self) -> bool {
        self.tombstone_count >= self.gc_threshold
    }
}

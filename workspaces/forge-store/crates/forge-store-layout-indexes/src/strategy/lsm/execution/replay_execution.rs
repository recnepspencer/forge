use forge_store_wal::BlobWalRecordKind;

use super::BaselineLsmCounterObservation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmReplayExecution {
    replay_tail: [BlobWalRecordKind; 3],
    replayable_count: u16,
    stale_run_count: u16,
    cleanup_batch_count: u16,
    remaining_run_count: u16,
    counters: BaselineLsmCounterObservation,
    current_materialization: crate::CurrentLayoutMaterialization,
}

impl BaselineLsmReplayExecution {
    pub(super) const fn new(
        replay_tail: [BlobWalRecordKind; 3],
        replayable_count: u16,
        stale_run_count: u16,
        cleanup_batch_count: u16,
        remaining_run_count: u16,
        current_materialization: crate::CurrentLayoutMaterialization,
    ) -> Self {
        Self {
            replay_tail,
            replayable_count,
            stale_run_count,
            cleanup_batch_count,
            remaining_run_count,
            counters: BaselineLsmCounterObservation::replay(replayable_count, cleanup_batch_count),
            current_materialization,
        }
    }

    pub const fn replay_tail(&self) -> [BlobWalRecordKind; 3] {
        self.replay_tail
    }

    pub const fn replayable_count(&self) -> u16 {
        self.replayable_count
    }

    pub const fn replay_monotonic(&self) -> bool {
        self.replayable_count > 0
    }

    pub const fn stale_run_count(&self) -> u16 {
        self.stale_run_count
    }

    pub const fn cleanup_batch_count(&self) -> u16 {
        self.cleanup_batch_count
    }

    pub const fn remaining_run_count(&self) -> u16 {
        self.remaining_run_count
    }

    pub const fn counters(&self) -> BaselineLsmCounterObservation {
        self.counters
    }

    pub const fn current_materialization(&self) -> &crate::CurrentLayoutMaterialization {
        &self.current_materialization
    }
}

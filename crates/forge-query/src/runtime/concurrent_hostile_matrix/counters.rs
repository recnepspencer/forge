#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConcurrentHostileMatrixCounterSnapshot {
    committed_read_hot_path_lock_count: usize,
    shared_read_mint_row_clone_count: usize,
    published_artifact_registry_lease_count: usize,
    reader_derived_evaluation_count: usize,
    orphaned_snapshot_generation_count: usize,
    unretired_read_pin_count: usize,
    journal_gap_count: usize,
    replay_residue_count: usize,
    delivery_residue_count: usize,
}

impl ForgeQueryConcurrentHostileMatrixCounterSnapshot {
    pub fn from_runtime(
        runtime: &super::super::ForgeQueryRuntime,
        delivery_residue_count: usize,
    ) -> Self {
        let shared_read = runtime.shared_read_counters();
        let replay = runtime
            .journal_replay_diagnostics()
            .counter_snapshot()
            .clone();
        Self {
            committed_read_hot_path_lock_count: shared_read.committed_read_hot_path_lock_count(),
            shared_read_mint_row_clone_count: shared_read.shared_read_mint_row_clone_count(),
            published_artifact_registry_lease_count: shared_read
                .published_artifact_registry_lease_count(),
            reader_derived_evaluation_count: shared_read.reader_derived_evaluation_count(),
            orphaned_snapshot_generation_count: shared_read.orphaned_generation_count(),
            unretired_read_pin_count: shared_read.unretired_pin_count(),
            journal_gap_count: replay.replay_gap_count(),
            replay_residue_count: replay.replay_residue_count(),
            delivery_residue_count,
        }
    }

    pub fn committed_read_hot_path_lock_count(&self) -> usize {
        self.committed_read_hot_path_lock_count
    }

    pub fn shared_read_mint_row_clone_count(&self) -> usize {
        self.shared_read_mint_row_clone_count
    }

    pub fn published_artifact_registry_lease_count(&self) -> usize {
        self.published_artifact_registry_lease_count
    }

    pub fn reader_derived_evaluation_count(&self) -> usize {
        self.reader_derived_evaluation_count
    }

    pub fn orphaned_snapshot_generation_count(&self) -> usize {
        self.orphaned_snapshot_generation_count
    }

    pub fn unretired_read_pin_count(&self) -> usize {
        self.unretired_read_pin_count
    }

    pub fn journal_gap_count(&self) -> usize {
        self.journal_gap_count
    }

    pub fn replay_residue_count(&self) -> usize {
        self.replay_residue_count
    }

    pub fn delivery_residue_count(&self) -> usize {
        self.delivery_residue_count
    }

    pub fn exact_zero_residue_count(&self) -> usize {
        self.committed_read_hot_path_lock_count
            + self.shared_read_mint_row_clone_count
            + self.reader_derived_evaluation_count
            + self.orphaned_snapshot_generation_count
            + self.unretired_read_pin_count
            + self.journal_gap_count
            + self.replay_residue_count
            + self.delivery_residue_count
    }
}

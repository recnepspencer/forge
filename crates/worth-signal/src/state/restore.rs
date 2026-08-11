use serde::{Deserialize, Serialize};

use crate::data::proof::ClassifiedSnapshotBatchCommit;

use super::lifecycle::{SnapshotRestoreCoarseReason, SnapshotRestoreIntent};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CheckpointRestoreSnapshotBatch {
    classified: ClassifiedSnapshotBatchCommit,
}

impl CheckpointRestoreSnapshotBatch {
    pub(crate) fn new(classified: ClassifiedSnapshotBatchCommit) -> Self {
        Self { classified }
    }

    #[cfg(test)]
    pub(crate) fn classified(&self) -> &ClassifiedSnapshotBatchCommit {
        &self.classified
    }

    pub(crate) fn clone_inner(&self) -> ClassifiedSnapshotBatchCommit {
        self.classified.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreDeltaAccounting {
    dependency_snapshot_delta_node_count: u64,
}

impl RestoreDeltaAccounting {
    pub(crate) fn new(dependency_snapshot_delta_node_count: u64) -> Self {
        Self {
            dependency_snapshot_delta_node_count,
        }
    }

    pub(crate) fn dependency_snapshot_delta_node_count(self) -> u64 {
        self.dependency_snapshot_delta_node_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Proof-bearing restore plan describing how much of a restore can be lowered
/// as shared-node delta work versus still requiring a coarse replacement boundary.
pub struct SnapshotRestorePlan {
    intent: SnapshotRestoreIntent,
    shared_node_count: u64,
    current_only_node_count: u64,
    snapshot_only_node_count: u64,
    checkpoint_restore_batch: CheckpointRestoreSnapshotBatch,
    delta_accounting: RestoreDeltaAccounting,
    coarse_replacement_required: bool,
    coarse_reasons: Vec<SnapshotRestoreCoarseReason>,
}

impl SnapshotRestorePlan {
    pub(crate) fn new(
        intent: SnapshotRestoreIntent,
        shared_node_count: u64,
        current_only_node_count: u64,
        snapshot_only_node_count: u64,
        checkpoint_restore_batch: CheckpointRestoreSnapshotBatch,
        delta_accounting: RestoreDeltaAccounting,
        coarse_replacement_required: bool,
        coarse_reasons: Vec<SnapshotRestoreCoarseReason>,
    ) -> Self {
        Self {
            intent,
            shared_node_count,
            current_only_node_count,
            snapshot_only_node_count,
            checkpoint_restore_batch,
            delta_accounting,
            coarse_replacement_required,
            coarse_reasons,
        }
    }

    pub fn checkpoint_restore_batch(&self) -> &CheckpointRestoreSnapshotBatch {
        &self.checkpoint_restore_batch
    }

    pub fn intent(&self) -> SnapshotRestoreIntent {
        self.intent
    }

    pub fn shared_node_count(&self) -> u64 {
        self.shared_node_count
    }

    pub fn current_only_node_count(&self) -> u64 {
        self.current_only_node_count
    }

    pub fn snapshot_only_node_count(&self) -> u64 {
        self.snapshot_only_node_count
    }

    pub fn dependency_snapshot_delta_node_count(&self) -> u64 {
        self.delta_accounting.dependency_snapshot_delta_node_count()
    }

    pub fn coarse_replacement_required(&self) -> bool {
        self.coarse_replacement_required
    }

    pub fn coarse_reasons(&self) -> &[SnapshotRestoreCoarseReason] {
        &self.coarse_reasons
    }
}

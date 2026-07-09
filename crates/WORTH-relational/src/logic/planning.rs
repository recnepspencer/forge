use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationalExecutionModel {
    SerialAuthority,
    StagedParallelPreparation,
    ParallelPostCommitConsumption,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningContract {
    pub immutable_snapshot_reads_required: bool,
    pub worker_local_staging_required: bool,
    pub deterministic_merge_required: bool,
}

impl Default for PlanningContract {
    fn default() -> Self {
        Self {
            immutable_snapshot_reads_required: true,
            worker_local_staging_required: true,
            deterministic_merge_required: true,
        }
    }
}

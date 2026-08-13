//! Runtime execution and serialization contracts.

use serde::{Deserialize, Serialize};

use crate::transactions::data::CommitAuthority;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitAuthorityContract {
    pub authority: CommitAuthority,
    pub version_publication_serialized: bool,
    pub lineage_finalization_serialized: bool,
    pub patch_publication_serialized: bool,
}

impl Default for CommitAuthorityContract {
    fn default() -> Self {
        Self {
            authority: CommitAuthority::default(),
            version_publication_serialized: true,
            lineage_finalization_serialized: true,
            patch_publication_serialized: true,
        }
    }
}

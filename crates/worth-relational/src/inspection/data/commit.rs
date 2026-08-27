use serde::{Deserialize, Serialize};

use crate::history::data::{BranchId, RelationalCommitReceipt};
use crate::indexes::data::DerivedIndexArtifacts;
use crate::lineage::data::{LineageArtifactCounters, LineageDigestBasis, LineageEventRecord};
use crate::transactions::data::RecordRef;
use worth_foundational::facade::AspectKey;

use super::{InspectionAccessPath, InspectionOrigin};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitInspection {
    pub commit: RelationalCommitReceipt,
    pub changed_records: Vec<RecordRef>,
    pub lineage_event_ids: Vec<u64>,
    pub lineage_events: Vec<LineageEventRecord>,
    pub lineage_digest_basis: LineageDigestBasis,
    pub lineage_artifact_counters: LineageArtifactCounters,
    pub derived_index_artifacts: DerivedIndexArtifacts,
    pub changed_aspects: Vec<AspectKey>,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentCommitInspectionRequest {
    pub branch_id: Option<BranchId>,
    pub limit: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct RecentCommitInspectionWindow {
    pub branch_head: Option<RelationalCommitReceipt>,
    pub commits: Vec<CommitInspection>,
    pub origin: InspectionOrigin,
    pub access_path: InspectionAccessPath,
}

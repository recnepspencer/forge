use crate::delta::{BranchDeltaLayerId, BRANCH_DELTA_FAMILY_VERSION};
use forge_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    CommitParentRecord, CommitSupportSummaryRecord, LineageSupportRecord, SchemaSupportRecord,
    StoredCommitEnvelope,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BranchSharedBaseRecord {
    pub branch_id: BranchId,
    pub source_branch_id: BranchId,
    pub source_frontier_commit_id: Option<CommitId>,
    pub delta_family_version: u32,
    pub authority_basis_digest: String,
}

impl Default for BranchSharedBaseRecord {
    fn default() -> Self {
        Self {
            branch_id: BranchId(String::new()),
            source_branch_id: BranchId(String::new()),
            source_frontier_commit_id: None,
            delta_family_version: BRANCH_DELTA_FAMILY_VERSION,
            authority_basis_digest: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct BranchDeltaLayerArtifacts {
    pub commit_envelopes: Vec<StoredCommitEnvelope>,
    pub commit_parent_records: Vec<CommitParentRecord>,
    pub commit_support_summaries: Vec<CommitSupportSummaryRecord>,
    pub schema_support_records: Vec<SchemaSupportRecord>,
    pub lineage_support_records: Vec<LineageSupportRecord>,
}

impl BranchDeltaLayerArtifacts {
    pub fn is_empty(&self) -> bool {
        self.commit_envelopes.is_empty()
            && self.commit_parent_records.is_empty()
            && self.commit_support_summaries.is_empty()
            && self.schema_support_records.is_empty()
            && self.lineage_support_records.is_empty()
    }

    pub fn canonicalize_order(&mut self) {
        self.commit_envelopes
            .sort_by_key(|record| record.commit_sequence);
        self.commit_parent_records.sort_by(|left, right| {
            left.commit_id
                .cmp(&right.commit_id)
                .then(left.parent_position.cmp(&right.parent_position))
                .then(left.parent_commit_id.cmp(&right.parent_commit_id))
        });
        self.commit_support_summaries
            .sort_by(|left, right| left.commit_id.cmp(&right.commit_id));
        self.schema_support_records
            .sort_by(|left, right| left.commit_id.cmp(&right.commit_id));
        self.lineage_support_records
            .sort_by(|left, right| left.commit_id.cmp(&right.commit_id));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BranchDeltaReplacementProofEntry {
    pub layer_id: BranchDeltaLayerId,
    pub branch_id: BranchId,
    pub base_frontier_commit_id: Option<CommitId>,
    pub target_frontier_commit_id: CommitId,
    pub commit_ids: Vec<CommitId>,
    pub delta_family_version: u32,
    pub authority_basis_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BranchDeltaLayerRecord {
    pub branch_delta_layer_id: BranchDeltaLayerId,
    pub branch_id: BranchId,
    pub base_frontier_commit_id: Option<CommitId>,
    pub target_frontier_commit_id: CommitId,
    pub commit_ids: Vec<CommitId>,
    pub delta_family_version: u32,
    pub authority_basis_digest: String,
    #[serde(default)]
    pub artifacts: BranchDeltaLayerArtifacts,
    pub replacement_of_layer_ids: Vec<BranchDeltaLayerId>,
    #[serde(default)]
    pub replacement_lineage_proof: Vec<BranchDeltaReplacementProofEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmbeddedCheckpointClassification {
    DerivedDurable,
    Ephemeral,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmbeddedCheckpointRecord {
    pub checkpoint_id: String,
    pub source_runtime_id: String,
    pub basis_branch_id: Option<BranchId>,
    pub basis_commit_id: Option<CommitId>,
    pub classification: EmbeddedCheckpointClassification,
    pub contained_commit_ids: Vec<CommitId>,
    pub metadata: Value,
}

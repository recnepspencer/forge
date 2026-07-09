use crate::backend::records::EmbeddedCheckpointRecord;
use worth_relational::facade::history::{BranchId, CommitId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitCoupledSupportAppendWitness {
    commit_id: CommitId,
    branch_id: BranchId,
    emits_schema_support: bool,
    emits_lineage_support: bool,
}
impl CommitCoupledSupportAppendWitness {
    pub(crate) fn new(
        commit_id: CommitId,
        branch_id: BranchId,
        emits_schema_support: bool,
        emits_lineage_support: bool,
    ) -> Self {
        Self {
            commit_id,
            branch_id,
            emits_schema_support,
            emits_lineage_support,
        }
    }
    pub fn commit_id(&self) -> CommitId {
        self.commit_id
    }
    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }
    pub fn emits_schema_support(&self) -> bool {
        self.emits_schema_support
    }
    pub fn emits_lineage_support(&self) -> bool {
        self.emits_lineage_support
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddedCheckpointFetchRequest {
    checkpoint_id: String,
}
impl EmbeddedCheckpointFetchRequest {
    pub fn new(checkpoint_id: impl Into<String>) -> Self {
        Self {
            checkpoint_id: checkpoint_id.into(),
        }
    }
    pub fn checkpoint_id(&self) -> &str {
        &self.checkpoint_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedEmbeddedCheckpoint {
    record: EmbeddedCheckpointRecord,
}
impl PersistedEmbeddedCheckpoint {
    pub(crate) fn new(record: EmbeddedCheckpointRecord) -> Self {
        Self { record }
    }
    pub fn checkpoint_id(&self) -> &str {
        &self.record.checkpoint_id
    }
    pub fn source_runtime_id(&self) -> &str {
        &self.record.source_runtime_id
    }
    pub fn classification(&self) -> crate::EmbeddedCheckpointClassification {
        match self.record.classification {
            crate::backend::records::EmbeddedCheckpointClassification::DerivedDurable => {
                crate::EmbeddedCheckpointClassification::DerivedDurable
            }
            crate::backend::records::EmbeddedCheckpointClassification::Ephemeral => {
                crate::EmbeddedCheckpointClassification::Ephemeral
            }
        }
    }
    pub fn record(&self) -> &EmbeddedCheckpointRecord {
        &self.record
    }
    pub fn basis_branch_id(&self) -> Option<&BranchId> {
        self.record.basis_branch_id.as_ref()
    }
    pub fn basis_commit_id(&self) -> Option<CommitId> {
        self.record.basis_commit_id
    }
    pub fn contained_commit_ids(&self) -> &[CommitId] {
        &self.record.contained_commit_ids
    }
}

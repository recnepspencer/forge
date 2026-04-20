use super::checkpoint_envelopes::EmbeddedCheckpointClassification;
use forge_relational::facade::history::CommitId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddedCheckpointPersistenceReceipt {
    pub(crate) checkpoint_id: String,
    pub(crate) contained_commit_ids: Vec<CommitId>,
    pub(crate) classification: EmbeddedCheckpointClassification,
}

impl EmbeddedCheckpointPersistenceReceipt {
    pub fn checkpoint_id(&self) -> &str {
        &self.checkpoint_id
    }

    pub fn contained_commit_ids(&self) -> &[CommitId] {
        &self.contained_commit_ids
    }

    pub fn classification(&self) -> &EmbeddedCheckpointClassification {
        &self.classification
    }
}

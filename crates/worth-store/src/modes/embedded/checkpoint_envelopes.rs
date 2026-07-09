use crate::{
    backend::records::{
        EmbeddedCheckpointClassification as StoredCheckpointClassification,
        EmbeddedCheckpointRecord,
    },
    failure::StoreError,
};
use worth_relational::facade::{
    history::{BranchId, CommitId},
    replay::CanonicalCommitEnvelope,
};
use serde::Serialize;
use serde_json::Value;

pub(crate) mod sealed {
    pub trait Sealed {}
}

pub(crate) struct VerifiedExternalRuntimeCommitEnvelope {
    envelope: CanonicalCommitEnvelope,
}

impl VerifiedExternalRuntimeCommitEnvelope {
    pub(crate) fn verify(external: ExternalRuntimeCommitEnvelope) -> Result<Self, StoreError> {
        if external.source_runtime_id.trim().is_empty() {
            return Err(StoreError::external_runtime_artifact_rejection(
                "external runtime commit envelopes must declare a non-empty source runtime identity",
            ));
        }
        Ok(Self {
            envelope: external.envelope,
        })
    }

    pub(crate) fn into_envelope(self) -> CanonicalCommitEnvelope {
        self.envelope
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ClassifiedEmbeddedCheckpointEnvelope {
    pub(crate) checkpoint_id: String,
    pub(crate) source_runtime_id: String,
    pub(crate) basis_branch_id: Option<BranchId>,
    pub(crate) basis_commit_id: Option<CommitId>,
    pub(crate) classification: StoredCheckpointClassification,
    pub(crate) contained_commits: Vec<CanonicalCommitEnvelope>,
    pub(crate) metadata: Value,
}

impl ClassifiedEmbeddedCheckpointEnvelope {
    pub(crate) fn classify(
        checkpoint: ExternalRuntimeCheckpointEnvelope,
    ) -> Result<Self, StoreError> {
        if checkpoint.checkpoint_id.trim().is_empty() {
            return Err(StoreError::external_runtime_checkpoint_rejection(
                "embedded checkpoints must declare a non-empty checkpoint identity",
            ));
        }
        if checkpoint.source_runtime_id.trim().is_empty() {
            return Err(StoreError::external_runtime_checkpoint_rejection(
                "embedded checkpoints must declare a non-empty source runtime identity",
            ));
        }
        let classification = checkpoint.classification.into_stored()?;
        Ok(Self {
            checkpoint_id: checkpoint.checkpoint_id,
            source_runtime_id: checkpoint.source_runtime_id,
            basis_branch_id: checkpoint.basis_branch_id,
            basis_commit_id: checkpoint.basis_commit_id,
            classification,
            contained_commits: checkpoint.contained_commits,
            metadata: checkpoint.metadata,
        })
    }

    pub(crate) fn contained_commits(&self) -> &[CanonicalCommitEnvelope] {
        &self.contained_commits
    }

    pub(crate) fn into_record(
        self,
        contained_commit_ids: Vec<CommitId>,
    ) -> EmbeddedCheckpointRecord {
        EmbeddedCheckpointRecord {
            checkpoint_id: self.checkpoint_id,
            source_runtime_id: self.source_runtime_id,
            basis_branch_id: self.basis_branch_id,
            basis_commit_id: self.basis_commit_id,
            classification: self.classification,
            contained_commit_ids,
            metadata: self.metadata,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum EmbeddedCheckpointClassification {
    AuthoritativeCommitBundle,
    DerivedDurable,
    Ephemeral,
}

impl EmbeddedCheckpointClassification {
    fn into_stored(self) -> Result<StoredCheckpointClassification, StoreError> {
        match self {
            Self::AuthoritativeCommitBundle => Err(StoreError::embedded_checkpoint_authority_violation(
                "embedded checkpoints may carry authoritative commits only through the canonical append path; the checkpoint itself cannot be classified as authoritative",
            )),
            Self::DerivedDurable => Ok(StoredCheckpointClassification::DerivedDurable),
            Self::Ephemeral => Ok(StoredCheckpointClassification::Ephemeral),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExternalRuntimeCommitEnvelope {
    source_runtime_id: String,
    envelope: CanonicalCommitEnvelope,
}

impl ExternalRuntimeCommitEnvelope {
    pub fn new(source_runtime_id: impl Into<String>, envelope: CanonicalCommitEnvelope) -> Self {
        Self {
            source_runtime_id: source_runtime_id.into(),
            envelope,
        }
    }

    pub fn source_runtime_id(&self) -> &str {
        &self.source_runtime_id
    }

    pub fn envelope(&self) -> &CanonicalCommitEnvelope {
        &self.envelope
    }
}

#[derive(Debug, Clone)]
pub struct ExternalRuntimeCheckpointEnvelope {
    pub(crate) checkpoint_id: String,
    pub(crate) source_runtime_id: String,
    pub(crate) basis_branch_id: Option<BranchId>,
    pub(crate) basis_commit_id: Option<CommitId>,
    pub(crate) classification: EmbeddedCheckpointClassification,
    pub(crate) contained_commits: Vec<CanonicalCommitEnvelope>,
    pub(crate) metadata: Value,
}

impl ExternalRuntimeCheckpointEnvelope {
    pub fn new(
        checkpoint_id: impl Into<String>,
        source_runtime_id: impl Into<String>,
        classification: EmbeddedCheckpointClassification,
    ) -> Self {
        Self {
            checkpoint_id: checkpoint_id.into(),
            source_runtime_id: source_runtime_id.into(),
            basis_branch_id: None,
            basis_commit_id: None,
            classification,
            contained_commits: Vec::new(),
            metadata: Value::Null,
        }
    }

    pub fn with_basis_branch(mut self, branch_id: BranchId) -> Self {
        self.basis_branch_id = Some(branch_id);
        self
    }

    pub fn with_basis_commit(mut self, commit_id: CommitId) -> Self {
        self.basis_commit_id = Some(commit_id);
        self
    }

    pub fn with_contained_commit(mut self, commit: CanonicalCommitEnvelope) -> Self {
        self.contained_commits.push(commit);
        self
    }

    pub fn with_metadata(mut self, metadata: Value) -> Self {
        self.metadata = metadata;
        self
    }
}

impl From<StoredCheckpointClassification> for EmbeddedCheckpointClassification {
    fn from(value: StoredCheckpointClassification) -> Self {
        match value {
            StoredCheckpointClassification::DerivedDurable => Self::DerivedDurable,
            StoredCheckpointClassification::Ephemeral => Self::Ephemeral,
        }
    }
}

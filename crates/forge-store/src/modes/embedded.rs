use crate::{
    backend::records::{
        EmbeddedCheckpointClassification as StoredCheckpointClassification,
        EmbeddedCheckpointRecord,
    },
    evidence::{OperatingModeLane, PersistedModeLaneEvidence},
    facade::{ForgeStore, ForgeStoreBuilder},
    failure::StoreError,
    modes::lifecycle::{EmbeddedModeConstructionPlan, ExternalArtifactIntakeCapabilityProof},
};
use forge_relational::facade::{
    history::{BranchId, CommitId},
    replay::CanonicalCommitEnvelope,
};
use serde::Serialize;
use serde_json::Value;

struct VerifiedExternalRuntimeCommitEnvelope {
    envelope: CanonicalCommitEnvelope,
}

impl VerifiedExternalRuntimeCommitEnvelope {
    fn verify(external: ExternalRuntimeCommitEnvelope) -> Result<Self, StoreError> {
        if external.source_runtime_id.trim().is_empty() {
            return Err(StoreError::external_runtime_artifact_rejection(
                "external runtime commit envelopes must declare a non-empty source runtime identity",
            ));
        }

        Ok(Self {
            envelope: external.envelope,
        })
    }

    fn into_envelope(self) -> CanonicalCommitEnvelope {
        self.envelope
    }
}

struct ClassifiedEmbeddedCheckpointEnvelope {
    checkpoint_id: String,
    source_runtime_id: String,
    basis_branch_id: Option<BranchId>,
    basis_commit_id: Option<CommitId>,
    classification: StoredCheckpointClassification,
    contained_commits: Vec<CanonicalCommitEnvelope>,
    metadata: Value,
}

impl ClassifiedEmbeddedCheckpointEnvelope {
    fn classify(checkpoint: ExternalRuntimeCheckpointEnvelope) -> Result<Self, StoreError> {
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

    fn contained_commits(&self) -> &[CanonicalCommitEnvelope] {
        &self.contained_commits
    }

    fn into_record(self, contained_commit_ids: Vec<CommitId>) -> EmbeddedCheckpointRecord {
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
    checkpoint_id: String,
    source_runtime_id: String,
    basis_branch_id: Option<BranchId>,
    basis_commit_id: Option<CommitId>,
    classification: EmbeddedCheckpointClassification,
    contained_commits: Vec<CanonicalCommitEnvelope>,
    metadata: Value,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddedCheckpointPersistenceReceipt {
    checkpoint_id: String,
    contained_commit_ids: Vec<CommitId>,
    classification: EmbeddedCheckpointClassification,
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

#[derive(Debug)]
pub struct EmbeddedModeBuilder {
    construction: EmbeddedModeConstructionPlan,
}

impl EmbeddedModeBuilder {
    pub(crate) fn new(store_builder: ForgeStoreBuilder) -> Self {
        Self {
            construction: EmbeddedModeConstructionPlan::new(store_builder),
        }
    }

    pub fn build(self) -> Result<EmbeddedStoreHandle, StoreError> {
        let (store_builder, capability) = self.construction.into_parts();
        let store = store_builder.build()?;
        store.record_embedded_mode_selection();
        Ok(EmbeddedStoreHandle { store, capability })
    }
}

#[derive(Debug)]
pub struct EmbeddedStoreHandle {
    store: ForgeStore,
    capability: ExternalArtifactIntakeCapabilityProof,
}

impl EmbeddedStoreHandle {
    pub fn persist_external_commit(
        &mut self,
        external: ExternalRuntimeCommitEnvelope,
    ) -> Result<crate::PersistedAuthoritativeCommit, StoreError> {
        let _capability = self.capability;
        let verified = VerifiedExternalRuntimeCommitEnvelope::verify(external)?;
        self.store.record_external_commit_intake();
        self.store.append_runtime_envelope(verified.into_envelope())
    }

    pub fn persist_external_checkpoint(
        &mut self,
        checkpoint: ExternalRuntimeCheckpointEnvelope,
    ) -> Result<EmbeddedCheckpointPersistenceReceipt, StoreError> {
        let _capability = self.capability;
        self.store.record_external_checkpoint_intake();
        let checkpoint = match ClassifiedEmbeddedCheckpointEnvelope::classify(checkpoint) {
            Ok(checkpoint) => checkpoint,
            Err(error) => {
                if matches!(
                    error.kind(),
                    crate::StoreErrorKind::EmbeddedCheckpointAuthorityViolation
                ) {
                    self.store.record_embedded_checkpoint_authority_rejection();
                    self.store.record_mode_misuse_rejection();
                }
                return Err(error);
            }
        };

        let mut contained_commit_ids = Vec::with_capacity(checkpoint.contained_commits().len());
        for commit in checkpoint.contained_commits() {
            let persisted = self.store.append_runtime_envelope(commit.clone())?;
            contained_commit_ids.push(persisted.envelope().commit.commit_id);
        }

        let receipt_checkpoint_id = checkpoint.checkpoint_id.clone();
        let receipt_classification: EmbeddedCheckpointClassification =
            checkpoint.classification.clone().into();
        let record = checkpoint.into_record(contained_commit_ids.clone());
        self.store.persist_embedded_checkpoint_record(record)?;

        Ok(EmbeddedCheckpointPersistenceReceipt {
            checkpoint_id: receipt_checkpoint_id,
            contained_commit_ids,
            classification: receipt_classification,
        })
    }

    pub fn fetch_persisted_checkpoint(
        &self,
        checkpoint_id: &str,
    ) -> Result<EmbeddedCheckpointPersistenceReceipt, StoreError> {
        let record = self.store.fetch_embedded_checkpoint_record(checkpoint_id)?;
        Ok(EmbeddedCheckpointPersistenceReceipt {
            checkpoint_id: record.checkpoint_id,
            contained_commit_ids: record.contained_commit_ids,
            classification: record.classification.into(),
        })
    }

    pub fn store(&self) -> &ForgeStore {
        &self.store
    }

    pub fn milestone_2_lane_evidence(&self) -> PersistedModeLaneEvidence {
        self.store
            .milestone_2_lane_evidence(OperatingModeLane::Embedded)
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

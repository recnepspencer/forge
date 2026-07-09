use super::{
    checkpoint_envelopes::{
        sealed, ClassifiedEmbeddedCheckpointEnvelope, ExternalRuntimeCheckpointEnvelope,
    },
    checkpoint_kinds::{
        ContainsCanonicalCommits, EmbeddedCheckpointKindMarker, NoContainedCommits,
    },
};
use crate::failure::StoreError;
use worth_relational::facade::{
    history::{BranchId, CommitId},
    replay::CanonicalCommitEnvelope,
};
use serde_json::Value;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct BasisFreeCheckpoint<K, C> {
    checkpoint_id: String,
    source_runtime_id: String,
    contained_commits: Vec<CanonicalCommitEnvelope>,
    metadata: Value,
    _marker: PhantomData<(K, C)>,
}

impl<K: EmbeddedCheckpointKindMarker> BasisFreeCheckpoint<K, NoContainedCommits> {
    pub fn new(checkpoint_id: impl Into<String>, source_runtime_id: impl Into<String>) -> Self {
        Self {
            checkpoint_id: checkpoint_id.into(),
            source_runtime_id: source_runtime_id.into(),
            contained_commits: Vec::new(),
            metadata: Value::Null,
            _marker: PhantomData,
        }
    }

    pub fn with_contained_commit(
        mut self,
        commit: CanonicalCommitEnvelope,
    ) -> BasisFreeCheckpoint<K, ContainsCanonicalCommits> {
        self.contained_commits.push(commit);
        BasisFreeCheckpoint {
            checkpoint_id: self.checkpoint_id,
            source_runtime_id: self.source_runtime_id,
            contained_commits: self.contained_commits,
            metadata: self.metadata,
            _marker: PhantomData,
        }
    }
}

impl<K, C> BasisFreeCheckpoint<K, C> {
    pub fn with_metadata(mut self, metadata: Value) -> Self {
        self.metadata = metadata;
        self
    }
}

#[derive(Debug, Clone)]
pub struct BasisBoundCheckpoint<K, C> {
    checkpoint_id: String,
    source_runtime_id: String,
    basis_branch_id: BranchId,
    basis_commit_id: CommitId,
    contained_commits: Vec<CanonicalCommitEnvelope>,
    metadata: Value,
    _marker: PhantomData<(K, C)>,
}

impl<K: EmbeddedCheckpointKindMarker> BasisBoundCheckpoint<K, NoContainedCommits> {
    pub fn new(
        checkpoint_id: impl Into<String>,
        source_runtime_id: impl Into<String>,
        basis_branch_id: BranchId,
        basis_commit_id: CommitId,
    ) -> Self {
        Self {
            checkpoint_id: checkpoint_id.into(),
            source_runtime_id: source_runtime_id.into(),
            basis_branch_id,
            basis_commit_id,
            contained_commits: Vec::new(),
            metadata: Value::Null,
            _marker: PhantomData,
        }
    }

    pub fn with_contained_commit(
        mut self,
        commit: CanonicalCommitEnvelope,
    ) -> BasisBoundCheckpoint<K, ContainsCanonicalCommits> {
        self.contained_commits.push(commit);
        BasisBoundCheckpoint {
            checkpoint_id: self.checkpoint_id,
            source_runtime_id: self.source_runtime_id,
            basis_branch_id: self.basis_branch_id,
            basis_commit_id: self.basis_commit_id,
            contained_commits: self.contained_commits,
            metadata: self.metadata,
            _marker: PhantomData,
        }
    }
}

impl<K, C> BasisBoundCheckpoint<K, C> {
    pub fn with_metadata(mut self, metadata: Value) -> Self {
        self.metadata = metadata;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasisBoundCheckpointWitness {
    basis_branch_id: BranchId,
    basis_commit_id: CommitId,
}

impl BasisBoundCheckpointWitness {
    pub(crate) fn new(basis_branch_id: BranchId, basis_commit_id: CommitId) -> Self {
        Self {
            basis_branch_id,
            basis_commit_id,
        }
    }

    pub fn basis_branch_id(&self) -> &BranchId {
        &self.basis_branch_id
    }

    pub fn basis_commit_id(&self) -> CommitId {
        self.basis_commit_id
    }
}

#[derive(Debug, Clone)]
pub struct VerifiedEmbeddedCheckpoint<K, C> {
    pub(crate) envelope: ClassifiedEmbeddedCheckpointEnvelope,
    basis_witness: Option<BasisBoundCheckpointWitness>,
    _marker: PhantomData<(K, C)>,
}

impl<K, C> VerifiedEmbeddedCheckpoint<K, C> {
    pub fn checkpoint_id(&self) -> &str {
        &self.envelope.checkpoint_id
    }

    pub fn basis_witness(&self) -> Option<&BasisBoundCheckpointWitness> {
        self.basis_witness.as_ref()
    }
}

pub trait IntoVerifiedEmbeddedCheckpoint<K, C>: sealed::Sealed {
    fn into_verified(self) -> Result<VerifiedEmbeddedCheckpoint<K, C>, StoreError>;
}

impl<K: EmbeddedCheckpointKindMarker, C> IntoVerifiedEmbeddedCheckpoint<K, C>
    for BasisFreeCheckpoint<K, C>
{
    fn into_verified(self) -> Result<VerifiedEmbeddedCheckpoint<K, C>, StoreError> {
        let classified =
            ClassifiedEmbeddedCheckpointEnvelope::classify(ExternalRuntimeCheckpointEnvelope {
                checkpoint_id: self.checkpoint_id,
                source_runtime_id: self.source_runtime_id,
                basis_branch_id: None,
                basis_commit_id: None,
                classification: K::CLASSIFICATION,
                contained_commits: self.contained_commits,
                metadata: self.metadata,
            })?;
        Ok(VerifiedEmbeddedCheckpoint {
            envelope: classified,
            basis_witness: None,
            _marker: PhantomData,
        })
    }
}

impl<K, C> sealed::Sealed for BasisFreeCheckpoint<K, C> {}

impl<K: EmbeddedCheckpointKindMarker, C> IntoVerifiedEmbeddedCheckpoint<K, C>
    for BasisBoundCheckpoint<K, C>
{
    fn into_verified(self) -> Result<VerifiedEmbeddedCheckpoint<K, C>, StoreError> {
        let basis_witness =
            BasisBoundCheckpointWitness::new(self.basis_branch_id.clone(), self.basis_commit_id);
        let classified =
            ClassifiedEmbeddedCheckpointEnvelope::classify(ExternalRuntimeCheckpointEnvelope {
                checkpoint_id: self.checkpoint_id,
                source_runtime_id: self.source_runtime_id,
                basis_branch_id: Some(self.basis_branch_id),
                basis_commit_id: Some(self.basis_commit_id),
                classification: K::CLASSIFICATION,
                contained_commits: self.contained_commits,
                metadata: self.metadata,
            })?;
        Ok(VerifiedEmbeddedCheckpoint {
            envelope: classified,
            basis_witness: Some(basis_witness),
            _marker: PhantomData,
        })
    }
}

impl<K, C> sealed::Sealed for BasisBoundCheckpoint<K, C> {}

use super::*;
use crate::memory_workspace::{ForgeQueryCommitIdentity, ForgeQuerySnapshotIdentity};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ForgeQueryIntentExecutionKind {
    Mutating,
    IdempotentNoop,
    InvariantViolation,
}

impl ForgeQueryIntentExecutionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mutating => "mutating",
            Self::IdempotentNoop => "idempotent-noop",
            Self::InvariantViolation => "invariant-violation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentExecution {
    pub(in crate::runtime::intent) execution_kind: ForgeQueryIntentExecutionKind,
    pub(in crate::runtime::intent) strategy_identity: String,
    pub(in crate::runtime::intent) strategy_version: String,
    pub(in crate::runtime::intent) strategy_descriptor_digest: String,
    pub(in crate::runtime::intent) canonical_input_digest: String,
    pub(in crate::runtime::intent) outcome_digest: String,
    pub(in crate::runtime::intent) invariant_evidence: Vec<String>,
    pub(in crate::runtime::intent) mutation_receipt: ForgeQueryMutationReceipt,
}

impl ForgeQueryIntentExecution {
    pub fn admitted(
        strategy_identity: impl Into<String>,
        strategy_version: impl Into<String>,
        strategy_descriptor_digest: impl Into<String>,
        canonical_input_digest: impl Into<String>,
        outcome_digest: impl Into<String>,
        invariant_evidence: impl IntoIterator<Item = impl Into<String>>,
        mutation_receipt: ForgeQueryMutationReceipt,
    ) -> Self {
        Self {
            execution_kind: ForgeQueryIntentExecutionKind::Mutating,
            strategy_identity: strategy_identity.into(),
            strategy_version: strategy_version.into(),
            strategy_descriptor_digest: strategy_descriptor_digest.into(),
            canonical_input_digest: canonical_input_digest.into(),
            outcome_digest: outcome_digest.into(),
            invariant_evidence: invariant_evidence.into_iter().map(Into::into).collect(),
            mutation_receipt,
        }
    }

    pub fn idempotent_noop(
        strategy_identity: impl Into<String>,
        strategy_version: impl Into<String>,
        strategy_descriptor_digest: impl Into<String>,
        canonical_input_digest: impl Into<String>,
        outcome_digest: impl Into<String>,
        invariant_evidence: impl IntoIterator<Item = impl Into<String>>,
        commit_identity: ForgeQueryCommitIdentity,
        snapshot_identity: ForgeQuerySnapshotIdentity,
    ) -> Self {
        Self {
            execution_kind: ForgeQueryIntentExecutionKind::IdempotentNoop,
            strategy_identity: strategy_identity.into(),
            strategy_version: strategy_version.into(),
            strategy_descriptor_digest: strategy_descriptor_digest.into(),
            canonical_input_digest: canonical_input_digest.into(),
            outcome_digest: outcome_digest.into(),
            invariant_evidence: invariant_evidence.into_iter().map(Into::into).collect(),
            mutation_receipt: ForgeQueryMutationReceipt::from_authoritative_parts(
                commit_identity,
                snapshot_identity,
                Vec::new(),
            ),
        }
    }

    pub fn invariant_violation(
        strategy_identity: impl Into<String>,
        strategy_version: impl Into<String>,
        strategy_descriptor_digest: impl Into<String>,
        canonical_input_digest: impl Into<String>,
        invariant_failure_digest: impl Into<String>,
        invariant_evidence: impl IntoIterator<Item = impl Into<String>>,
        snapshot_identity: ForgeQuerySnapshotIdentity,
    ) -> Self {
        Self {
            execution_kind: ForgeQueryIntentExecutionKind::InvariantViolation,
            strategy_identity: strategy_identity.into(),
            strategy_version: strategy_version.into(),
            strategy_descriptor_digest: strategy_descriptor_digest.into(),
            canonical_input_digest: canonical_input_digest.into(),
            outcome_digest: invariant_failure_digest.into(),
            invariant_evidence: invariant_evidence.into_iter().map(Into::into).collect(),
            mutation_receipt: ForgeQueryMutationReceipt::from_authoritative_parts(
                ForgeQueryCommitIdentity::absent(),
                snapshot_identity,
                Vec::new(),
            ),
        }
    }

    pub fn execution_kind(&self) -> ForgeQueryIntentExecutionKind {
        self.execution_kind
    }

    pub fn strategy_identity(&self) -> &str {
        &self.strategy_identity
    }

    pub fn strategy_version(&self) -> &str {
        &self.strategy_version
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        &self.strategy_descriptor_digest
    }

    pub fn canonical_input_digest(&self) -> &str {
        &self.canonical_input_digest
    }

    pub fn outcome_digest(&self) -> &str {
        &self.outcome_digest
    }

    pub fn produced_mutation_digest(&self) -> Option<&str> {
        (self.execution_kind == ForgeQueryIntentExecutionKind::Mutating)
            .then_some(self.outcome_digest.as_str())
    }

    pub fn invariant_evidence(&self) -> &[String] {
        &self.invariant_evidence
    }

    pub fn mutation_receipt(&self) -> &ForgeQueryMutationReceipt {
        &self.mutation_receipt
    }
}

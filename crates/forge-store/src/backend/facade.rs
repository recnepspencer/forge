use crate::{
    authority::{
        AuthoritativeBranchHeadRecord, AuthoritativeExportBundle, CanonicalizedCommitEnvelope,
        FetchedAuthoritativeCommit, PersistedAuthoritativeCommit, VerifiedAuthoritativeAppend,
    },
    evidence::{CanonicalizationMetrics, StoreCounterSnapshot},
    failure::StoreError,
    recovery::{DurableRecoveryOutcome, DurableRecoveryPlan, DurableRetryResolution},
    wal::{DurableMutationId, DurablePublicationPhase},
};
use forge_relational::facade::history::{BranchId, CommitId};
use std::path::PathBuf;

use super::{
    embedded::{EmbeddedBackendMode, EmbeddedStoreBackend},
    records::EmbeddedCheckpointRecord,
    sqlite::SqliteStoreBackend,
};

#[derive(Debug, Clone)]
pub enum StoreBackendMode {
    InMemory,
    LocalFile(PathBuf),
    SqliteFile(PathBuf),
}

#[derive(Debug)]
pub enum StoreBackend {
    Embedded(EmbeddedStoreBackend),
    Sqlite(SqliteStoreBackend),
}

impl StoreBackend {
    pub fn open(mode: StoreBackendMode) -> Result<Self, StoreError> {
        match mode {
            StoreBackendMode::InMemory => Ok(Self::Embedded(EmbeddedStoreBackend::open(
                EmbeddedBackendMode::InMemory,
            )?)),
            StoreBackendMode::LocalFile(path) => Ok(Self::Embedded(EmbeddedStoreBackend::open(
                EmbeddedBackendMode::LocalFile(path),
            )?)),
            StoreBackendMode::SqliteFile(path) => Ok(Self::Sqlite(SqliteStoreBackend::open(path)?)),
        }
    }

    pub fn from_export_bundle(bundle: AuthoritativeExportBundle) -> Result<Self, StoreError> {
        Ok(Self::Embedded(EmbeddedStoreBackend::from_export_bundle(
            bundle,
        )?))
    }

    pub fn create_branch(
        &mut self,
        new_branch: BranchId,
        from_branch: Option<&BranchId>,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        match self {
            Self::Embedded(backend) => backend.create_branch(new_branch, from_branch),
            Self::Sqlite(backend) => backend.create_branch(new_branch, from_branch),
        }
    }

    pub fn verify_append(
        &self,
        append: CanonicalizedCommitEnvelope,
    ) -> Result<VerifiedAuthoritativeAppend, StoreError> {
        match self {
            Self::Embedded(backend) => backend.verify_append(append),
            Self::Sqlite(backend) => backend.verify_append(append),
        }
    }

    pub fn append(
        &mut self,
        verified: VerifiedAuthoritativeAppend,
    ) -> Result<PersistedAuthoritativeCommit, StoreError> {
        match self {
            Self::Embedded(backend) => backend.append(verified),
            Self::Sqlite(backend) => backend.append(verified),
        }
    }

    pub fn fetch_commit(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedAuthoritativeCommit, StoreError> {
        match self {
            Self::Embedded(backend) => backend.fetch_commit(commit_id),
            Self::Sqlite(backend) => backend.fetch_commit(commit_id),
        }
    }

    pub fn fetch_branch_head(
        &self,
        branch_id: &BranchId,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        match self {
            Self::Embedded(backend) => backend.fetch_branch_head(branch_id),
            Self::Sqlite(backend) => backend.fetch_branch_head(branch_id),
        }
    }

    pub fn record_canonicalization(&self, metrics: CanonicalizationMetrics) {
        match self {
            Self::Embedded(backend) => backend.record_canonicalization(metrics),
            Self::Sqlite(backend) => backend.record_canonicalization(metrics),
        }
    }

    pub fn counter_snapshot(&self) -> StoreCounterSnapshot {
        match self {
            Self::Embedded(backend) => backend.counter_snapshot(),
            Self::Sqlite(backend) => backend.counter_snapshot(),
        }
    }

    pub fn export_bundle(&self) -> AuthoritativeExportBundle {
        match self {
            Self::Embedded(backend) => backend.export_bundle(),
            Self::Sqlite(backend) => backend.export_bundle(),
        }
    }

    pub fn persist_embedded_checkpoint(
        &mut self,
        record: EmbeddedCheckpointRecord,
    ) -> Result<EmbeddedCheckpointRecord, StoreError> {
        match self {
            Self::Embedded(backend) => backend.persist_embedded_checkpoint(record),
            Self::Sqlite(backend) => backend.persist_embedded_checkpoint(record),
        }
    }

    pub fn fetch_embedded_checkpoint(
        &self,
        checkpoint_id: &str,
    ) -> Result<EmbeddedCheckpointRecord, StoreError> {
        match self {
            Self::Embedded(backend) => backend.fetch_embedded_checkpoint(checkpoint_id),
            Self::Sqlite(backend) => backend.fetch_embedded_checkpoint(checkpoint_id),
        }
    }

    pub fn record_durable_mode_selection(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_durable_mode_selection(),
            Self::Sqlite(backend) => backend.counters().record_durable_mode_selection(),
        }
    }

    pub fn record_embedded_mode_selection(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_embedded_mode_selection(),
            Self::Sqlite(backend) => backend.counters().record_embedded_mode_selection(),
        }
    }

    pub fn record_hosted_runtime_start(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_hosted_runtime_start(),
            Self::Sqlite(backend) => backend.counters().record_hosted_runtime_start(),
        }
    }

    pub fn record_hosted_runtime_stop(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_hosted_runtime_stop(),
            Self::Sqlite(backend) => backend.counters().record_hosted_runtime_stop(),
        }
    }

    pub fn record_external_commit_intake(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_external_commit_intake(),
            Self::Sqlite(backend) => backend.counters().record_external_commit_intake(),
        }
    }

    pub fn record_external_checkpoint_intake(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_external_checkpoint_intake(),
            Self::Sqlite(backend) => backend.counters().record_external_checkpoint_intake(),
        }
    }

    pub fn record_embedded_checkpoint_authority_rejection(&self) {
        match self {
            Self::Embedded(backend) => backend
                .counters()
                .record_embedded_checkpoint_authority_rejection(),
            Self::Sqlite(backend) => backend
                .counters()
                .record_embedded_checkpoint_authority_rejection(),
        }
    }

    pub fn record_mode_misuse_rejection(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_mode_misuse_rejection(),
            Self::Sqlite(backend) => backend.counters().record_mode_misuse_rejection(),
        }
    }

    pub fn record_durable_commit_acknowledged(&self) {
        match self {
            Self::Embedded(backend) => backend.counters().record_durable_commit_acknowledged(),
            Self::Sqlite(backend) => backend.counters().record_durable_commit_acknowledged(),
        }
    }

    pub fn admit_durable_mutation(
        &mut self,
        runtime_session_id: &str,
        operation_name: &str,
    ) -> Result<DurableMutationId, StoreError> {
        match self {
            Self::Embedded(backend) => {
                backend.admit_durable_mutation(runtime_session_id, operation_name)
            }
            Self::Sqlite(backend) => {
                backend.admit_durable_mutation(runtime_session_id, operation_name)
            }
        }
    }

    pub fn record_hosted_runtime_commit_result(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        envelope: forge_relational::facade::replay::CanonicalCommitEnvelope,
    ) -> Result<(), StoreError> {
        match self {
            Self::Embedded(backend) => backend.record_hosted_runtime_commit_result(
                runtime_session_id,
                durable_mutation_id,
                envelope,
            ),
            Self::Sqlite(backend) => backend.record_hosted_runtime_commit_result(
                runtime_session_id,
                durable_mutation_id,
                envelope,
            ),
        }
    }

    pub fn record_publication_phase(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        phase: DurablePublicationPhase,
        commit_id: Option<CommitId>,
    ) -> Result<(), StoreError> {
        match self {
            Self::Embedded(backend) => backend.record_publication_phase(
                runtime_session_id,
                durable_mutation_id,
                phase,
                commit_id,
            ),
            Self::Sqlite(backend) => backend.record_publication_phase(
                runtime_session_id,
                durable_mutation_id,
                phase,
                commit_id,
            ),
        }
    }

    pub fn resolve_retry(
        &self,
        durable_mutation_id: DurableMutationId,
    ) -> Result<DurableRetryResolution, StoreError> {
        match self {
            Self::Embedded(backend) => backend.resolve_retry(durable_mutation_id),
            Self::Sqlite(backend) => backend.resolve_retry(durable_mutation_id),
        }
    }

    pub fn recover_durable_runtime(
        &mut self,
        runtime_session_id: &str,
    ) -> Result<DurableRecoveryOutcome, StoreError> {
        match self {
            Self::Embedded(backend) => backend.recover_durable_runtime(runtime_session_id),
            Self::Sqlite(backend) => backend.recover_durable_runtime(runtime_session_id),
        }
    }

    pub fn plan_durable_recovery(&self) -> DurableRecoveryPlan {
        match self {
            Self::Embedded(backend) => backend.plan_durable_recovery(),
            Self::Sqlite(backend) => backend.plan_durable_recovery(),
        }
    }
}

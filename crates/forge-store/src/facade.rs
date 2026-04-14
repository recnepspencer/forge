use crate::{
    authority::{
        canonicalize, AuthoritativeBranchHeadRecord, AuthoritativeExportBundle,
        FetchedAuthoritativeCommit, PersistedAuthoritativeCommit, RawRuntimeCommitEnvelope,
        CURRENT_CANONICALIZATION_VERSION,
    },
    backend::{records::EmbeddedCheckpointRecord, StoreBackend, StoreBackendMode},
    evidence::{
        Milestone1CertificationBundle, Milestone4CertificationBundle, OperatingModeLane,
        PersistedModeLaneEvidence, StoreCounterSnapshot,
    },
    failure::StoreError,
    recovery::{DurableRecoveryOutcome, DurableRecoveryPlan},
    snapshot::{
        PublishedSnapshotHandle, SnapshotCaptureRequest, SnapshotId, SnapshotImageBundle,
        SnapshotReadRequest, SnapshotReadResult, SnapshotRestoreOutcome,
    },
    wal::{DurableMutationId, DurablePublicationPhase},
};
use forge_relational::facade::{
    history::{BranchId, CommitId},
    replay::CanonicalCommitEnvelope,
};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ForgeStoreBuilder {
    backend_mode: StoreBackendMode,
}

impl Default for ForgeStoreBuilder {
    fn default() -> Self {
        Self {
            backend_mode: StoreBackendMode::InMemory,
        }
    }
}

impl ForgeStoreBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn in_memory(mut self) -> Self {
        self.backend_mode = StoreBackendMode::InMemory;
        self
    }

    pub fn local_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.backend_mode = StoreBackendMode::LocalFile(path.into());
        self
    }

    pub fn sqlite_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.backend_mode = StoreBackendMode::SqliteFile(path.into());
        self
    }

    pub fn build(self) -> Result<ForgeStore, StoreError> {
        Ok(ForgeStore {
            backend: StoreBackend::open(self.backend_mode)?,
        })
    }

    pub fn embedded_mode(self) -> crate::EmbeddedModeBuilder {
        crate::EmbeddedModeBuilder::new(self)
    }

    pub fn durable_mode(
        self,
        runtime: forge_relational::facade::runtime::RelationalRuntime,
    ) -> crate::DurableModeBuilder {
        crate::DurableModeBuilder::new(self, runtime)
    }
}

#[derive(Debug)]
pub struct ForgeStore {
    backend: StoreBackend,
}

impl ForgeStore {
    pub(crate) fn append_runtime_envelope(
        &mut self,
        envelope: forge_relational::facade::replay::CanonicalCommitEnvelope,
    ) -> Result<PersistedAuthoritativeCommit, StoreError> {
        let raw = RawRuntimeCommitEnvelope::new(envelope);
        let canonical = canonicalize(raw, CURRENT_CANONICALIZATION_VERSION)?;
        self.backend.record_canonicalization(*canonical.metrics());
        let verified = self.backend.verify_append(canonical)?;
        self.backend.append(verified)
    }

    pub(crate) fn persist_embedded_checkpoint_record(
        &mut self,
        record: EmbeddedCheckpointRecord,
    ) -> Result<EmbeddedCheckpointRecord, StoreError> {
        self.backend.persist_embedded_checkpoint(record)
    }

    pub(crate) fn fetch_embedded_checkpoint_record(
        &self,
        checkpoint_id: &str,
    ) -> Result<EmbeddedCheckpointRecord, StoreError> {
        self.backend.fetch_embedded_checkpoint(checkpoint_id)
    }

    pub(crate) fn record_durable_mode_selection(&self) {
        self.backend.record_durable_mode_selection();
    }

    pub(crate) fn record_embedded_mode_selection(&self) {
        self.backend.record_embedded_mode_selection();
    }

    pub(crate) fn record_hosted_runtime_start(&self) {
        self.backend.record_hosted_runtime_start();
    }

    pub(crate) fn record_hosted_runtime_stop(&self) {
        self.backend.record_hosted_runtime_stop();
    }

    pub(crate) fn record_external_commit_intake(&self) {
        self.backend.record_external_commit_intake();
    }

    pub(crate) fn record_external_checkpoint_intake(&self) {
        self.backend.record_external_checkpoint_intake();
    }

    pub(crate) fn record_embedded_checkpoint_authority_rejection(&self) {
        self.backend
            .record_embedded_checkpoint_authority_rejection();
    }

    pub(crate) fn record_mode_misuse_rejection(&self) {
        self.backend.record_mode_misuse_rejection();
    }

    pub(crate) fn admit_durable_mutation(
        &mut self,
        runtime_session_id: &str,
        operation_name: &str,
    ) -> Result<DurableMutationId, StoreError> {
        self.backend
            .admit_durable_mutation(runtime_session_id, operation_name)
    }

    pub(crate) fn record_hosted_runtime_commit_result(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        envelope: CanonicalCommitEnvelope,
    ) -> Result<(), StoreError> {
        self.backend.record_hosted_runtime_commit_result(
            runtime_session_id,
            durable_mutation_id,
            envelope,
        )
    }

    pub(crate) fn record_publication_phase(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        phase: DurablePublicationPhase,
        commit_id: Option<CommitId>,
    ) -> Result<(), StoreError> {
        self.backend.record_publication_phase(
            runtime_session_id,
            durable_mutation_id,
            phase,
            commit_id,
        )
    }

    pub(crate) fn recover_durable_runtime(
        &mut self,
        runtime_session_id: &str,
    ) -> Result<DurableRecoveryOutcome, StoreError> {
        self.backend.recover_durable_runtime(runtime_session_id)
    }

    pub(crate) fn plan_durable_recovery(&self) -> DurableRecoveryPlan {
        self.backend.plan_durable_recovery()
    }

    pub(crate) fn resolve_durable_retry(
        &self,
        durable_mutation_id: DurableMutationId,
    ) -> Result<crate::DurableRetryResolution, StoreError> {
        self.backend.resolve_retry(durable_mutation_id)
    }

    pub(crate) fn record_durable_commit_acknowledged(&self) {
        self.backend.record_durable_commit_acknowledged();
    }

    pub fn create_branch(
        &mut self,
        new_branch: BranchId,
        from_branch: Option<&BranchId>,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        self.backend.create_branch(new_branch, from_branch)
    }

    pub fn append_canonical_commit(
        &mut self,
        envelope: CanonicalCommitEnvelope,
    ) -> Result<PersistedAuthoritativeCommit, StoreError> {
        self.append_runtime_envelope(envelope)
    }

    pub fn fetch_canonical_commit(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedAuthoritativeCommit, StoreError> {
        self.backend.fetch_commit(commit_id)
    }

    pub fn fetch_branch_head(
        &self,
        branch_id: &BranchId,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        self.backend.fetch_branch_head(branch_id)
    }

    pub fn counters(&self) -> StoreCounterSnapshot {
        self.backend.counter_snapshot()
    }

    pub fn export_authoritative_records(&self) -> AuthoritativeExportBundle {
        self.backend.export_bundle()
    }

    pub fn capture_snapshot(
        &mut self,
        request: SnapshotCaptureRequest,
    ) -> Result<PublishedSnapshotHandle, StoreError> {
        self.backend.capture_snapshot(request)
    }

    pub fn read_snapshot(
        &self,
        request: SnapshotReadRequest,
    ) -> Result<SnapshotReadResult, StoreError> {
        self.backend.read_snapshot(request)
    }

    pub fn restore_snapshot(
        &self,
        snapshot_id: SnapshotId,
        target_commit_id: CommitId,
    ) -> Result<SnapshotRestoreOutcome, StoreError> {
        self.backend.restore_snapshot(snapshot_id, target_commit_id)
    }

    pub fn rebuild_snapshot(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotImageBundle, StoreError> {
        self.backend.rebuild_snapshot(snapshot_id)
    }

    #[cfg(test)]
    pub(crate) fn remove_snapshot_image_for_test(
        &mut self,
        snapshot_id: SnapshotId,
    ) -> Result<(), StoreError> {
        self.backend.remove_snapshot_image_for_test(snapshot_id)
    }

    pub fn milestone_1_certification_bundle(&self) -> Milestone1CertificationBundle {
        let export = self.export_authoritative_records();
        Milestone1CertificationBundle::from_export(&export, self.counters())
    }

    pub fn milestone_4_certification_bundle(
        &self,
        truth_image: &SnapshotImageBundle,
        restored_image: &SnapshotImageBundle,
        rebuilt_image: &SnapshotImageBundle,
    ) -> Milestone4CertificationBundle {
        Milestone4CertificationBundle::new(
            truth_image,
            restored_image,
            rebuilt_image,
            self.counters(),
        )
    }

    pub(crate) fn milestone_2_lane_evidence(
        &self,
        lane: OperatingModeLane,
    ) -> PersistedModeLaneEvidence {
        let export = self.export_authoritative_records();
        PersistedModeLaneEvidence::from_export(lane, &export, self.counters())
    }

    pub fn rebuild_from_authoritative_export(
        bundle: AuthoritativeExportBundle,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            backend: StoreBackend::from_export_bundle(bundle)?,
        })
    }
}

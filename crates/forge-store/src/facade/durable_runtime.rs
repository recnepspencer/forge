use crate::{
    authority::{canonicalize, PersistedAuthoritativeCommit, RawRuntimeCommitEnvelope, CURRENT_CANONICALIZATION_VERSION},
    backend::records::EmbeddedCheckpointRecord,
    failure::StoreError,
    publication::PublicationWriteOutcome,
    recovery::{DurableRecoveryOutcome, DurableRecoveryPlan},
    wal::{DurableMutationId, DurablePublicationPhase},
};
use forge_relational::facade::{history::CommitId, replay::CanonicalCommitEnvelope};

use super::ForgeStore;

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

    #[cfg(test)]
    pub(crate) fn record_embedded_checkpoint_authority_rejection(&self) {
        self.backend
            .record_embedded_checkpoint_authority_rejection();
    }

    #[cfg(test)]
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

    pub(crate) fn record_bulk_checkpoint_publication_intent(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        checkpoint_sequence: Option<u64>,
    ) -> Result<(), StoreError> {
        self.backend.record_bulk_checkpoint_publication_intent(
            runtime_session_id,
            durable_mutation_id,
            checkpoint_sequence,
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

    pub(crate) fn classify_durable_publication(
        &self,
        durable_mutation_id: DurableMutationId,
        expected_commit_id: Option<CommitId>,
    ) -> Result<PublicationWriteOutcome, StoreError> {
        self.backend
            .classify_durable_publication(durable_mutation_id, expected_commit_id)

    }
}

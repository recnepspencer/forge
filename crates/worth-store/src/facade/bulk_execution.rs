use crate::{
    bulk::{
        BudgetAdmittedChunkPlan, BulkCanonicalChunkExecutionRequest, BulkCheckpointPolicy,
        BulkChunkExecutionOutcome, DurablyExecutedBulkChunk, ResumeReadyBulkProgram,
    },
    failure::StoreError,
    wal::DurablePublicationPhase,
};
use worth_relational::facade::replay::CanonicalCommitEnvelope;

use super::WORTHStore;

impl WORTHStore {
    pub fn admit_bulk_canonical_chunk_execution(
        &self,
        admitted: BudgetAdmittedChunkPlan,
        canonical_envelope: CanonicalCommitEnvelope,
    ) -> Result<BulkCanonicalChunkExecutionRequest, StoreError> {
        BulkCanonicalChunkExecutionRequest::admit(admitted, canonical_envelope)
    }

    pub fn execute_bulk_canonical_chunk(
        &mut self,
        request: BulkCanonicalChunkExecutionRequest,
        checkpoint_policy: BulkCheckpointPolicy,
    ) -> Result<BulkChunkExecutionOutcome, StoreError> {
        let (admitted, canonical_envelope) = request.into_parts();
        let persisted = self.append_canonical_commit(canonical_envelope)?;
        self.finalize_bulk_chunk_execution(
            &admitted,
            persisted.envelope().commit.commit_id,
            checkpoint_policy,
        )
    }

    pub fn execute_bulk_canonical_chunk_durably(
        &mut self,
        request: BulkCanonicalChunkExecutionRequest,
        checkpoint_policy: BulkCheckpointPolicy,
    ) -> Result<DurablyExecutedBulkChunk, StoreError> {
        let runtime_session_id = request.runtime_session_id();
        let operation_name = request.operation_name();
        let canonical_commit_id = request.canonical_envelope().commit.commit_id;
        let next_checkpoint_sequence = if checkpoint_policy.should_publish() {
            Some(self.next_bulk_checkpoint_sequence(
                request.admitted_chunk().program_id(),
                request.admitted_chunk().plan_id(),
            )?)
        } else {
            None
        };
        let durable_mutation_id =
            self.admit_durable_mutation(&runtime_session_id, &operation_name)?;
        self.record_hosted_runtime_commit_result(
            &runtime_session_id,
            durable_mutation_id,
            request.canonical_envelope().clone(),
        )?;
        self.record_bulk_checkpoint_publication_intent(
            &runtime_session_id,
            durable_mutation_id,
            next_checkpoint_sequence,
        )?;
        self.record_publication_phase(
            &runtime_session_id,
            durable_mutation_id,
            DurablePublicationPhase::CanonicalCommitProduced,
            Some(canonical_commit_id),
        )?;
        let (admitted, canonical_envelope) = request.into_parts();
        let persisted = self.append_canonical_commit(canonical_envelope)?;
        let persisted_commit_id = persisted.envelope().commit.commit_id;
        self.record_publication_phase(
            &runtime_session_id,
            durable_mutation_id,
            DurablePublicationPhase::AuthoritativeAppendPublished,
            Some(persisted_commit_id),
        )?;
        let outcome =
            self.finalize_bulk_chunk_execution(&admitted, persisted_commit_id, checkpoint_policy)?;
        self.record_publication_phase(
            &runtime_session_id,
            durable_mutation_id,
            DurablePublicationPhase::AcknowledgmentEligible,
            Some(persisted_commit_id),
        )?;
        self.record_durable_commit_acknowledged();
        Ok(DurablyExecutedBulkChunk::new(durable_mutation_id, outcome))
    }

    pub fn execute_next_resumed_bulk_chunk(
        &mut self,
        resumed: &ResumeReadyBulkProgram,
        admitted_memory_units: u64,
        canonical_envelope: CanonicalCommitEnvelope,
        checkpoint_policy: BulkCheckpointPolicy,
    ) -> Result<Option<BulkChunkExecutionOutcome>, StoreError> {
        let Some(admitted) = resumed.admit_next_chunk(admitted_memory_units)? else {
            return Ok(None);
        };
        let request = self.admit_bulk_canonical_chunk_execution(admitted, canonical_envelope)?;
        self.execute_bulk_canonical_chunk(request, checkpoint_policy)
            .map(Some)
    }

    pub fn execute_next_resumed_bulk_chunk_durably(
        &mut self,
        resumed: &ResumeReadyBulkProgram,
        admitted_memory_units: u64,
        canonical_envelope: CanonicalCommitEnvelope,
        checkpoint_policy: BulkCheckpointPolicy,
    ) -> Result<Option<DurablyExecutedBulkChunk>, StoreError> {
        let Some(admitted) = resumed.admit_next_chunk(admitted_memory_units)? else {
            return Ok(None);
        };
        let request = self.admit_bulk_canonical_chunk_execution(admitted, canonical_envelope)?;
        self.execute_bulk_canonical_chunk_durably(request, checkpoint_policy)
            .map(Some)
    }
}

use crate::{
    bulk::{
        BudgetAdmittedChunkPlan, BulkChunkCommitWitness, BulkPlanKind, ChunkOrdinal,
        ResumeBoundaryCandidate, ResumeReadyBulkProgram,
    },
    failure::{StoreError, StoreErrorKind},
    wal::DurableMutationId,
};
use forge_relational::facade::history::CommitId;

use super::super::engine::{StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn fetch_latest_resume_boundary(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ResumeBoundaryCandidate, StoreError> {
        self.counters.record_bulk_resume_index_lookup();
        match self.fetch_program_chunk_witness_index_untracked(program_id, plan_id) {
            Ok(index) => Ok(ResumeBoundaryCandidate::new(
                program_id.to_string(),
                plan_id.to_string(),
                Some(index.highest_committed_chunk_ordinal()),
                crate::ChunkOrdinal::new(index.highest_committed_chunk_ordinal().value() + 1),
                index.latest_checkpoint_sequence(),
            )),
            Err(error) if matches!(error.kind(), StoreErrorKind::BulkChunkWitnessGap) => {
                Ok(ResumeBoundaryCandidate::new(
                    program_id.to_string(),
                    plan_id.to_string(),
                    None,
                    crate::ChunkOrdinal::new(0),
                    None,
                ))
            }
            Err(error) => Err(error),
        }
    }

    pub(super) fn rebuild_bulk_resume_ready_program(
        &self,
        plan_kind: BulkPlanKind,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ResumeReadyBulkProgram, StoreError> {
        let plan = self.fetch_bulk_chunk_plan(program_id, plan_id)?;
        let resume_boundary = self.fetch_latest_resume_boundary(program_id, plan_id)?;
        let witness_index =
            match self.fetch_program_chunk_witness_index_untracked(program_id, plan_id) {
                Ok(index) => Some(index),
                Err(error) if matches!(error.kind(), StoreErrorKind::BulkChunkWitnessGap) => None,
                Err(error) => return Err(error),
            };
        let latest_checkpoint = match resume_boundary.latest_checkpoint_sequence() {
            Some(_) => Some(self.fetch_bulk_progress_checkpoint(program_id, plan_id)?),
            None => None,
        };

        match plan_kind {
            BulkPlanKind::Ingest => {
                let manifest = self.fetch_frozen_bulk_manifest(program_id, plan.input_digest())?;
                ResumeReadyBulkProgram::admit_ingest(
                    &manifest,
                    plan,
                    witness_index,
                    latest_checkpoint,
                    resume_boundary,
                )
            }
            BulkPlanKind::Transform => {
                let basis_commit_id = plan.basis_commit_id().ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::BulkTransformBasisDrift,
                        format!(
                            "bulk transform plan `{plan_id}` for program `{program_id}` is missing a locked basis commit"
                        ),
                    )
                })?;
                let basis = self.find_frozen_transform_basis_for_plan(
                    program_id,
                    plan.target_branch_scope(),
                    basis_commit_id,
                )?;
                let partition =
                    self.fetch_frozen_transform_partition(program_id, plan.input_digest())?;
                ResumeReadyBulkProgram::admit_transform(
                    &basis,
                    &partition,
                    plan,
                    witness_index,
                    latest_checkpoint,
                    resume_boundary,
                )
            }
        }
    }

    pub(crate) fn finish_bulk_recovery_publication(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        plan_kind: BulkPlanKind,
        program_id: &str,
        plan_id: &str,
        chunk_ordinal: ChunkOrdinal,
        canonical_commit_id: CommitId,
        checkpoint_sequence: Option<u64>,
    ) -> Result<(), StoreError> {
        let resumed_program =
            self.rebuild_bulk_resume_ready_program(plan_kind, program_id, plan_id)?;
        self.counters.record_bulk_chunk_resume();
        if resumed_program.next_chunk_ordinal() != chunk_ordinal {
            return Err(StoreError::new(
                StoreErrorKind::BulkResumeBoundaryAmbiguous,
                format!(
                    "bulk recovery for program `{program_id}` plan `{plan_id}` expected chunk {} but resume boundary resolved to {}",
                    chunk_ordinal.value(),
                    resumed_program.next_chunk_ordinal().value()
                ),
            ));
        }
        let admitted_memory_units = resumed_program
            .plan()
            .chunk_by_ordinal(chunk_ordinal)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkChunkContractUnsupported,
                    format!(
                        "bulk recovery chunk ordinal {} does not exist in deterministic plan `{plan_id}`",
                        chunk_ordinal.value()
                    ),
                )
            })?
            .width_units();
        let admitted: BudgetAdmittedChunkPlan = resumed_program
            .admit_next_chunk(admitted_memory_units)?
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkResumeBoundaryAmbiguous,
                    format!(
                        "bulk recovery for program `{program_id}` plan `{plan_id}` resolved to a completed program before chunk {}",
                        chunk_ordinal.value()
                    ),
                )
            })?;
        let witness = self.publish_bulk_chunk_witness(BulkChunkCommitWitness::publish(
            &admitted,
            canonical_commit_id,
        )?)?;
        if let Some(sequence) = checkpoint_sequence {
            let latest_checkpoint_sequence = self
                .fetch_program_chunk_witness_index_untracked(program_id, plan_id)
                .ok()
                .and_then(|index| index.latest_checkpoint_sequence());
            if latest_checkpoint_sequence.unwrap_or(0) < sequence {
                self.publish_bulk_progress_checkpoint(witness.clone())?;
            }
        }
        self.record_publication_phase(
            runtime_session_id,
            durable_mutation_id,
            crate::wal::DurablePublicationPhase::AcknowledgmentEligible,
            Some(canonical_commit_id),
        )?;
        self.counters.record_bulk_chunk_commit();
        self.counters.record_durable_commit_acknowledged();
        Ok(())
    }

    pub(crate) fn reconcile_bulk_support_from_published_truth(
        &mut self,
        plan_kind: BulkPlanKind,
        program_id: &str,
        plan_id: &str,
        chunk_ordinal: ChunkOrdinal,
        canonical_commit_id: CommitId,
        checkpoint_sequence: Option<u64>,
    ) -> Result<(), StoreError> {
        let plan = self.fetch_bulk_chunk_plan(program_id, plan_id)?;
        if plan.kind() != plan_kind {
            return Err(StoreError::new(
                StoreErrorKind::BulkResumeBoundaryAmbiguous,
                format!(
                    "bulk recovery plan kind drift for program `{program_id}` plan `{plan_id}`"
                ),
            ));
        }
        let admitted_memory_units = plan
            .chunk_by_ordinal(chunk_ordinal)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkChunkContractUnsupported,
                    format!(
                        "bulk recovery chunk ordinal {} does not exist in deterministic plan `{plan_id}`",
                        chunk_ordinal.value()
                    ),
                )
            })?
            .width_units();
        let admitted = BudgetAdmittedChunkPlan::admit(&plan, chunk_ordinal, admitted_memory_units)?;

        let witness_index =
            match self.fetch_program_chunk_witness_index_untracked(program_id, plan_id) {
                Ok(index) => Some(index),
                Err(error) if matches!(error.kind(), StoreErrorKind::BulkChunkWitnessGap) => None,
                Err(error) => return Err(error),
            };
        let witness_present = witness_index
            .as_ref()
            .map(|index| index.highest_committed_chunk_ordinal().value() >= chunk_ordinal.value())
            .unwrap_or(false);
        let witness = if witness_present {
            None
        } else {
            Some(
                self.publish_bulk_chunk_witness(BulkChunkCommitWitness::publish(
                    &admitted,
                    canonical_commit_id,
                )?)?,
            )
        };

        if let Some(sequence) = checkpoint_sequence {
            let latest_checkpoint_sequence = match self
                .fetch_program_chunk_witness_index_untracked(program_id, plan_id)
            {
                Ok(index) => index.latest_checkpoint_sequence(),
                Err(error) if matches!(error.kind(), StoreErrorKind::BulkChunkWitnessGap) => None,
                Err(error) => return Err(error),
            };
            if latest_checkpoint_sequence.unwrap_or(0) < sequence {
                let checkpoint_witness = witness.as_ref().map_or_else(
                    || BulkChunkCommitWitness::publish(&admitted, canonical_commit_id),
                    |witness| Ok(witness.clone()),
                )?;
                self.publish_bulk_progress_checkpoint(checkpoint_witness)?;
            }
        }

        Ok(())
    }
}

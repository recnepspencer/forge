use crate::{
    bulk::{
        DeterministicChunkPlan, ProgramChunkWitnessIndex, PublishedBulkProgressCheckpoint,
        ResumeBoundaryCandidate, ResumeReadyBulkProgram,
    },
    failure::StoreError,
};

use super::ForgeStore;

impl ForgeStore {
    pub(crate) fn load_bulk_resume_artifacts(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<
        (
            DeterministicChunkPlan,
            Option<ProgramChunkWitnessIndex>,
            Option<PublishedBulkProgressCheckpoint>,
            ResumeBoundaryCandidate,
        ),
        StoreError,
    > {
        let plan = self.fetch_bulk_chunk_plan(program_id, plan_id)?;
        let resume_boundary = self.fetch_latest_bulk_resume_boundary(program_id, plan_id)?;
        let witness_index = match self.fetch_program_chunk_witness_index(program_id, plan_id) {
            Ok(index) => Some(index),
            Err(error) if matches!(error.kind(), crate::StoreErrorKind::BulkChunkWitnessGap) => {
                None
            }
            Err(error) => return Err(error),
        };
        let latest_checkpoint = match resume_boundary.latest_checkpoint_sequence() {
            Some(_) => Some(self.fetch_bulk_progress_checkpoint(program_id, plan_id)?),
            None => None,
        };
        Ok((plan, witness_index, latest_checkpoint, resume_boundary))
    }

    pub(crate) fn next_bulk_checkpoint_sequence(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<u64, StoreError> {
        match self.fetch_program_chunk_witness_index(program_id, plan_id) {
            Ok(index) => Ok(index
                .latest_checkpoint_sequence()
                .map(|sequence| sequence + 1)
                .unwrap_or(1)),
            Err(error) if matches!(error.kind(), crate::StoreErrorKind::BulkChunkWitnessGap) => {
                Ok(1)
            }
            Err(error) => Err(error),
        }
    }

    pub(crate) fn admit_resume_ready_bulk_program(
        &self,
        program_id: &str,
        plan_id: &str,
        plan: DeterministicChunkPlan,
        witness_index: Option<ProgramChunkWitnessIndex>,
        latest_checkpoint: Option<PublishedBulkProgressCheckpoint>,
        resume_boundary: ResumeBoundaryCandidate,
    ) -> Result<ResumeReadyBulkProgram, StoreError> {
        match plan.kind() {
            crate::BulkPlanKind::Ingest => {
                let manifest = self.fetch_frozen_bulk_manifest(program_id, plan.input_digest())?;
                ResumeReadyBulkProgram::admit_ingest(
                    &manifest,
                    plan,
                    witness_index,
                    latest_checkpoint,
                    resume_boundary,
                )
            }
            crate::BulkPlanKind::Transform => {
                let basis_commit_id = plan.basis_commit_id().ok_or_else(|| {
                    StoreError::new(
                        crate::StoreErrorKind::BulkTransformBasisDrift,
                        format!(
                            "bulk transform plan `{plan_id}` for program `{program_id}` is missing a locked basis commit"
                        ),
                    )
                })?;
                let basis = self.backend.find_frozen_transform_basis_for_plan(
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
}

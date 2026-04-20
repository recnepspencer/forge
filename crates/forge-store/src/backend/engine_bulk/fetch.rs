use crate::{
    bulk::{
        DeterministicChunkPlan, FrozenBulkSourceManifest, FrozenTransformBasis,
        FrozenTransformTargetPartition, ProgramChunkWitnessIndex,
    },
    failure::{StoreError, StoreErrorKind},
};
use forge_relational::facade::history::{BranchId, CommitId};

use super::super::{
    engine::{StateBackedStoreBackend, StatePersistence},
    integrity::{
        bulk_checkpoint_artifact_id, bulk_plan_artifact_id, bulk_witness_index_artifact_id,
        frozen_bulk_manifest_artifact_id, frozen_transform_basis_artifact_id,
        frozen_transform_partition_artifact_id,
    },
};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn fetch_frozen_bulk_manifest(
        &self,
        program_id: &str,
        manifest_digest: &str,
    ) -> Result<FrozenBulkSourceManifest, StoreError> {
        let artifact_id = frozen_bulk_manifest_artifact_id(program_id, manifest_digest);
        self.state
            .frozen_bulk_manifest_records
            .get(&artifact_id)
            .map(|record| record.manifest.clone())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkChunkContractUnsupported,
                    format!("bulk manifest `{artifact_id}` not found"),
                )
            })
    }

    pub fn fetch_frozen_transform_basis(
        &self,
        program_id: &str,
        basis_digest: &str,
    ) -> Result<FrozenTransformBasis, StoreError> {
        let artifact_id = frozen_transform_basis_artifact_id(program_id, basis_digest);
        self.state
            .frozen_transform_basis_records
            .get(&artifact_id)
            .map(|record| record.basis.clone())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkTransformBasisDrift,
                    format!("bulk transform basis `{artifact_id}` not found"),
                )
            })
    }

    pub fn fetch_frozen_transform_partition(
        &self,
        program_id: &str,
        partition_digest: &str,
    ) -> Result<FrozenTransformTargetPartition, StoreError> {
        let artifact_id = frozen_transform_partition_artifact_id(program_id, partition_digest);
        self.state
            .frozen_transform_partition_records
            .get(&artifact_id)
            .map(|record| record.partition.clone())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkTransformBasisDrift,
                    format!("bulk transform partition `{artifact_id}` not found"),
                )
            })
    }

    pub fn find_frozen_transform_basis_for_plan(
        &self,
        program_id: &str,
        target_branch_scope: &BranchId,
        basis_commit_id: CommitId,
    ) -> Result<FrozenTransformBasis, StoreError> {
        self.state
            .frozen_transform_basis_records
            .values()
            .find(|record| {
                record.program_id == program_id
                    && record.basis.target_branch_scope() == target_branch_scope
                    && record.basis.basis_commit_id() == basis_commit_id
            })
            .map(|record| record.basis.clone())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkTransformBasisDrift,
                    format!(
                        "bulk transform basis for program `{program_id}` branch `{}` commit {} not found",
                        target_branch_scope.0,
                        basis_commit_id.0
                    ),
                )
            })
    }

    pub fn fetch_bulk_chunk_plan(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<DeterministicChunkPlan, StoreError> {
        let artifact_id = bulk_plan_artifact_id(program_id, plan_id);
        self.state
            .bulk_deterministic_plan_records
            .get(&artifact_id)
            .map(|record| record.plan.clone())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkChunkContractUnsupported,
                    format!("bulk plan `{artifact_id}` not found"),
                )
            })
    }

    pub fn fetch_bulk_progress_checkpoint(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<crate::PublishedBulkProgressCheckpoint, StoreError> {
        let index = self.fetch_program_chunk_witness_index(program_id, plan_id)?;
        let checkpoint_sequence = index.latest_checkpoint_sequence().ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::BulkCheckpointPublicationGap,
                format!("bulk checkpoint for `{program_id}:{plan_id}` not found"),
            )
        })?;
        let artifact_id = bulk_checkpoint_artifact_id(program_id, plan_id, checkpoint_sequence);
        self.state
            .bulk_progress_checkpoint_records
            .get(&artifact_id)
            .map(|record| record.checkpoint.clone())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkCheckpointPublicationGap,
                    format!("bulk checkpoint `{artifact_id}` not found"),
                )
            })
    }

    pub fn fetch_program_chunk_witness_index(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ProgramChunkWitnessIndex, StoreError> {
        self.counters.record_bulk_resume_index_lookup();
        self.fetch_program_chunk_witness_index_untracked(program_id, plan_id)
    }

    pub(super) fn fetch_program_chunk_witness_index_untracked(
        &self,
        program_id: &str,
        plan_id: &str,
    ) -> Result<ProgramChunkWitnessIndex, StoreError> {
        let artifact_id = bulk_witness_index_artifact_id(program_id, plan_id);
        self.state
            .program_chunk_witness_index_records
            .get(&artifact_id)
            .map(|record| record.index.clone())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BulkChunkWitnessGap,
                    format!("bulk witness index `{artifact_id}` not found"),
                )
            })
    }
}

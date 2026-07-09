use crate::{
    bulk::{
        BulkPlanKind, DeterministicChunkPlan, FrozenBulkSourceManifest, FrozenTransformBasis,
        FrozenTransformTargetPartition, BULK_FAMILY_VERSION,
    },
    failure::StoreError,
};

use super::super::{
    engine::{StateBackedStoreBackend, StatePersistence},
    integrity::{
        bulk_plan_artifact_id, bulk_program_artifact_id, frozen_bulk_manifest_artifact_id,
        frozen_transform_basis_artifact_id, frozen_transform_partition_artifact_id,
    },
    records::{
        BulkDeterministicPlanRecord, BulkProgramIdentityRecord, FrozenBulkManifestRecord,
        FrozenTransformBasisRecord, FrozenTransformPartitionRecord,
    },
};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn persist_frozen_bulk_manifest(
        &mut self,
        manifest: FrozenBulkSourceManifest,
    ) -> Result<FrozenBulkSourceManifest, StoreError> {
        let mut next = self.state.clone();
        let program_artifact_id = bulk_program_artifact_id(manifest.program_id());
        next.bulk_program_identity_records.insert(
            program_artifact_id.clone(),
            BulkProgramIdentityRecord {
                artifact_id: program_artifact_id,
                family_version: BULK_FAMILY_VERSION,
                kind: BulkPlanKind::Ingest,
                program_id: manifest.program_id().to_string(),
                source_identity: manifest.source_identity().to_string(),
                target_branch_scope: manifest.target_branch_scope().clone(),
                basis_commit_id: None,
            },
        );
        let artifact_id =
            frozen_bulk_manifest_artifact_id(manifest.program_id(), manifest.manifest_digest());
        next.frozen_bulk_manifest_records.insert(
            artifact_id.clone(),
            FrozenBulkManifestRecord {
                artifact_id,
                family_version: BULK_FAMILY_VERSION,
                program_id: manifest.program_id().to_string(),
                manifest: manifest.clone(),
            },
        );
        self.commit_replacement_state(next)?;
        Ok(manifest)
    }

    pub fn persist_frozen_transform_basis(
        &mut self,
        basis: FrozenTransformBasis,
    ) -> Result<FrozenTransformBasis, StoreError> {
        let mut next = self.state.clone();
        let program_artifact_id = bulk_program_artifact_id(basis.program_id());
        next.bulk_program_identity_records.insert(
            program_artifact_id.clone(),
            BulkProgramIdentityRecord {
                artifact_id: program_artifact_id,
                family_version: BULK_FAMILY_VERSION,
                kind: BulkPlanKind::Transform,
                program_id: basis.program_id().to_string(),
                source_identity: basis.transform_identity().to_string(),
                target_branch_scope: basis.target_branch_scope().clone(),
                basis_commit_id: Some(basis.basis_commit_id()),
            },
        );
        let artifact_id =
            frozen_transform_basis_artifact_id(basis.program_id(), basis.basis_digest());
        next.frozen_transform_basis_records.insert(
            artifact_id.clone(),
            FrozenTransformBasisRecord {
                artifact_id,
                family_version: BULK_FAMILY_VERSION,
                program_id: basis.program_id().to_string(),
                basis: basis.clone(),
            },
        );
        self.commit_replacement_state(next)?;
        Ok(basis)
    }

    pub fn persist_frozen_transform_partition(
        &mut self,
        partition: FrozenTransformTargetPartition,
    ) -> Result<FrozenTransformTargetPartition, StoreError> {
        let mut next = self.state.clone();
        let artifact_id = frozen_transform_partition_artifact_id(
            partition.program_id(),
            partition.partition_digest(),
        );
        next.frozen_transform_partition_records.insert(
            artifact_id.clone(),
            FrozenTransformPartitionRecord {
                artifact_id,
                family_version: BULK_FAMILY_VERSION,
                program_id: partition.program_id().to_string(),
                partition: partition.clone(),
            },
        );
        self.commit_replacement_state(next)?;
        self.counters.record_bulk_transform_partition(1);
        Ok(partition)
    }

    pub fn persist_bulk_chunk_plan(
        &mut self,
        plan: DeterministicChunkPlan,
    ) -> Result<DeterministicChunkPlan, StoreError> {
        let mut next = self.state.clone();
        let artifact_id = bulk_plan_artifact_id(plan.program_id(), plan.plan_id());
        next.bulk_deterministic_plan_records.insert(
            artifact_id.clone(),
            BulkDeterministicPlanRecord {
                artifact_id,
                family_version: BULK_FAMILY_VERSION,
                program_id: plan.program_id().to_string(),
                plan: plan.clone(),
            },
        );
        self.commit_replacement_state(next)?;
        Ok(plan)
    }
}

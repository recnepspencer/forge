use super::super::ServingPhysicalRuntime;
use crate::physical_runtime::{
    AdmittedRecordPlacementPolicy, CompletedPhysicalRootPublication, PhysicalDataDispatchOutcome,
    PhysicalManifestCapacityTransition, PhysicalMutationIdempotencyMaterial, RecordAppendBatch,
};

impl ServingPhysicalRuntime {
    pub fn certification_publish_single_durable_mutation(
        &self,
        placement: AdmittedRecordPlacementPolicy,
        manifest_capacity_transition: PhysicalManifestCapacityTransition,
        material: PhysicalMutationIdempotencyMaterial,
        batch: RecordAppendBatch,
    ) -> CompletedPhysicalRootPublication {
        let (basis, durable) = self.certification_prepare_single_wal_durable_mutation(
            placement,
            manifest_capacity_transition,
            material,
            batch,
        );
        let submission = self.record_submission();
        let dispatched = match submission.dispatch_wal_durable_data(durable) {
            PhysicalDataDispatchOutcome::Dispatched(dispatched) => dispatched,
            _ => panic!("the exact WAL-durable member must dispatch"),
        };
        self.certification_complete_dispatched_mutation(basis, dispatched)
    }
}

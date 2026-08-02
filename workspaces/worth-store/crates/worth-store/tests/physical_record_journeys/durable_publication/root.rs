use worth_store::physical_runtime::{
    AdmittedRecordPlacementPolicy, CompletedPhysicalRootPublication,
    PhysicalManifestCapacityTransition, PhysicalMutationIdempotencyMaterial, RecordAppendBatch,
    ServingPhysicalRuntime,
};

pub(crate) fn publish_single(
    serving: &ServingPhysicalRuntime,
    placement: AdmittedRecordPlacementPolicy,
    material: PhysicalMutationIdempotencyMaterial,
    batch: RecordAppendBatch,
) -> CompletedPhysicalRootPublication {
    publish_single_with_manifest_capacity_transition(
        serving,
        placement,
        PhysicalManifestCapacityTransition::PreserveCurrent,
        material,
        batch,
    )
}

pub(crate) fn publish_single_with_manifest_capacity_transition(
    serving: &ServingPhysicalRuntime,
    placement: AdmittedRecordPlacementPolicy,
    manifest_capacity_transition: PhysicalManifestCapacityTransition,
    material: PhysicalMutationIdempotencyMaterial,
    batch: RecordAppendBatch,
) -> CompletedPhysicalRootPublication {
    serving.certification_publish_single_durable_mutation(
        placement,
        manifest_capacity_transition,
        material,
        batch,
    )
}

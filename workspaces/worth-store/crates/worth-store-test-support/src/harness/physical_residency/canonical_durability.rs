use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    PhysicalMutationAcknowledgment, PhysicalMutationDeadline, PhysicalMutationIdempotencyMaterial,
    PhysicalMutationOutcome, PhysicalMutationPreparationSuccess, PhysicalMutationRequest,
    RecordAppendBatch,
};

use super::PhysicalResidencyStoreWorld;

pub fn canonical_physical_mutation_acknowledgment(
    world: &PhysicalResidencyStoreWorld,
    idempotency_material: [u8; 32],
    record: &[u8],
) -> PhysicalMutationAcknowledgment {
    let submission = world.serving().record_submission();
    let key = submission
        .issue_idempotency_key(PhysicalMutationIdempotencyMaterial::new(
            idempotency_material,
        ))
        .expect("fixture idempotency key admission");
    let request = PhysicalMutationRequest::platform_durable(
        key,
        PhysicalMutationDeadline::after_milliseconds(1_000).expect("fixture deadline"),
    );
    let prepared = match submission
        .prepare_durable_append(
            RecordAppendBatch::try_from_iter([record]).expect("fixture append batch"),
            world.placement,
            request,
        )
        .into_raw()
    {
        TransitionOutcome::Success(PhysicalMutationPreparationSuccess::Prepared(prepared)) => {
            prepared
        }
        _ => panic!("canonical fixture mutation preparation must succeed"),
    };
    match prepared.execute() {
        PhysicalMutationOutcome::Completed(completed) => completed.into_acknowledgment(),
        PhysicalMutationOutcome::ProvenNoEffect(fate) => {
            panic!(
                "canonical fixture unexpectedly proved no effect: {:?}",
                fate.cause()
            )
        }
        PhysicalMutationOutcome::Indeterminate(fate) => {
            panic!(
                "canonical fixture became indeterminate at {:?}",
                fate.stage()
            )
        }
    }
}

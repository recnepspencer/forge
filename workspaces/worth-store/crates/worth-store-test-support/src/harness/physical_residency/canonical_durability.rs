#[cfg(feature = "recovery-runtime-fixtures")]
use worth_proof::NonEmpty;
use worth_proof::TransitionOutcome;
#[cfg(feature = "recovery-runtime-fixtures")]
use worth_store::physical_runtime::{
    PhysicalCurrentRootAdvanceOutcome, PhysicalDataDispatchOutcome, PhysicalDataSettlementOutcome,
    PhysicalRootNamespaceDurabilityOutcome, PhysicalRootPublicationPreparationOutcome,
    PhysicalRootReplacementOutcome, PhysicalWalGroupAppendOutcome, PhysicalWalGroupBarrierOutcome,
};
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
    canonical_physical_batch_acknowledgment(world, idempotency_material, [record])
}

pub fn canonical_physical_batch_acknowledgment<'a>(
    world: &PhysicalResidencyStoreWorld,
    idempotency_material: [u8; 32],
    records: impl IntoIterator<Item = &'a [u8]>,
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
            RecordAppendBatch::try_from_iter(records).expect("fixture append batch"),
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

#[cfg(feature = "recovery-runtime-fixtures")]
pub fn canonical_durable_wal_attempt_without_execution(
    world: &PhysicalResidencyStoreWorld,
    idempotency_material: [u8; 32],
    record: &[u8],
) {
    let submission = world.serving().certification_record_submission();
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
        _ => panic!("canonical WAL-only fixture preparation must succeed"),
    };
    let appended = match submission.append_prepared_wal_group(NonEmpty::new(prepared, Vec::new())) {
        PhysicalWalGroupAppendOutcome::Appended(appended) => appended,
        _ => panic!("canonical WAL-only fixture append must succeed"),
    };
    match submission.synchronize_appended_wal_group(appended) {
        PhysicalWalGroupBarrierOutcome::Durable(_) => {}
        _ => panic!("canonical WAL-only fixture barrier must succeed"),
    }
}

#[cfg(feature = "recovery-runtime-fixtures")]
pub fn canonical_rooted_mutation_without_acknowledgment(
    world: &PhysicalResidencyStoreWorld,
    idempotency_material: [u8; 32],
    record: &[u8],
) {
    let submission = world.serving().certification_record_submission();
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
        _ => panic!("canonical rooted fixture preparation must succeed"),
    };
    let appended = match submission.append_prepared_wal_group(NonEmpty::new(prepared, Vec::new())) {
        PhysicalWalGroupAppendOutcome::Appended(appended) => appended,
        _ => panic!("canonical rooted fixture WAL append must succeed"),
    };
    let basis = appended.basis();
    let durable = match submission.synchronize_appended_wal_group(appended) {
        PhysicalWalGroupBarrierOutcome::Durable(durable) => durable
            .into_members()
            .into_vec()
            .pop()
            .expect("the singleton group has one durable member"),
        _ => panic!("canonical rooted fixture WAL barrier must succeed"),
    };
    let dispatched = match submission.dispatch_wal_durable_data(durable) {
        PhysicalDataDispatchOutcome::Dispatched(dispatched) => dispatched,
        _ => panic!("canonical rooted fixture data dispatch must succeed"),
    };
    let settled = match dispatched.settle_exact_effects() {
        PhysicalDataSettlementOutcome::Settled(settled) => settled,
        _ => panic!("canonical rooted fixture data settlement must succeed"),
    };
    let joined = submission
        .join_data_settled_group(basis, NonEmpty::new(settled, Vec::new()))
        .unwrap_or_else(|rejected| {
            panic!("canonical rooted group rejected: {:?}", rejected.cause())
        });
    let prepared = match submission.prepare_root_publication(joined) {
        PhysicalRootPublicationPreparationOutcome::Prepared(prepared) => prepared,
        _ => panic!("canonical rooted fixture root preparation must succeed"),
    };
    let replaced = match submission.replace_prepared_root(prepared) {
        PhysicalRootReplacementOutcome::Replaced(replaced) => replaced,
        _ => panic!("canonical rooted fixture selector replacement must succeed"),
    };
    let durable = match submission.synchronize_replaced_root_namespace(replaced) {
        PhysicalRootNamespaceDurabilityOutcome::Durable(durable) => durable,
        _ => panic!("canonical rooted fixture namespace barrier must succeed"),
    };
    match submission.advance_namespace_durable_root(durable) {
        PhysicalCurrentRootAdvanceOutcome::Advanced(_) => {}
        _ => panic!("canonical rooted fixture current-root advance must succeed"),
    }
}

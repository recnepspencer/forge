use super::{
    checkpoint, identity_receipt, initialization::InitializedWriter,
    operation_program::C8Operation, CheckpointStageWithSeed,
};
use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    AdmittedRecordPlacementPolicy, PhysicalMutationDeadline, PhysicalMutationIdempotencyMaterial,
    PhysicalMutationOutcome, PhysicalMutationPreparationSuccess, PhysicalMutationRequest,
    RecordAppendBatch, ServingPhysicalRuntime,
};

pub(super) fn seed_initial_history(
    writer: &InitializedWriter,
    stage: &CheckpointStageWithSeed,
) -> Result<Vec<identity_receipt::IdentityReceipt>, String> {
    let scheduled_operations = writer
        .operation_program
        .scheduled_operations(stage.schedule_seed);
    let first_split = scheduled_operations.len() / 3;
    let second_split = scheduled_operations.len() * 2 / 3;
    let mut receipts = seed_operations(
        &writer.serving,
        writer.placement,
        &scheduled_operations[..first_split],
    )?;
    checkpoint::complete(&writer.serving, stage.perturbation_seed)?;
    receipts.extend(seed_operations(
        &writer.serving,
        writer.placement,
        &scheduled_operations[first_split..second_split],
    )?);
    checkpoint::complete(&writer.serving, stage.perturbation_seed ^ 0xC8_00_00_02)?;
    receipts.extend(seed_operations(
        &writer.serving,
        writer.placement,
        &scheduled_operations[second_split..],
    )?);
    Ok(receipts)
}

fn seed_operations(
    serving: &ServingPhysicalRuntime,
    placement: AdmittedRecordPlacementPolicy,
    operations: &[&C8Operation],
) -> Result<Vec<identity_receipt::IdentityReceipt>, String> {
    let mut receipts = Vec::with_capacity(operations.len());
    for operation in operations {
        let batch = RecordAppendBatch::try_from_iter([operation.payload().to_owned()])
            .map_err(|denial| format!("C8 writer seed batch denied: {denial:?}"))?;
        let submission = serving.record_submission();
        let key = submission
            .issue_idempotency_key(PhysicalMutationIdempotencyMaterial::new(
                operation.material(),
            ))
            .map_err(|denial| format!("C8 writer seed identity denied: {denial:?}"))?;
        let idempotency = key.identity().bytes();
        let request = PhysicalMutationRequest::platform_durable(
            key,
            PhysicalMutationDeadline::after_milliseconds(5_000)
                .expect("C8 mutation deadline is nonzero"),
        );
        let prepared = match submission
            .prepare_durable_append(batch, placement, request)
            .into_raw()
        {
            TransitionOutcome::Success(PhysicalMutationPreparationSuccess::Prepared(prepared)) => {
                prepared
            }
            TransitionOutcome::Success(PhysicalMutationPreparationSuccess::Completed(_)) => {
                return Err(
                    "ordinary C8 writer unexpectedly reused a completed mutation".to_owned(),
                )
            }
            TransitionOutcome::Success(PhysicalMutationPreparationSuccess::ProvenNoEffect(_))
            | TransitionOutcome::Success(PhysicalMutationPreparationSuccess::Indeterminate(_)) => {
                return Err(
                    "ordinary C8 writer seed mutation had no completed preparation".to_owned(),
                )
            }
            TransitionOutcome::Denied(_)
            | TransitionOutcome::Deferred(_)
            | TransitionOutcome::Stale(_)
            | TransitionOutcome::RebindRequired(_)
            | TransitionOutcome::Failed(_) => {
                return Err("ordinary C8 writer seed mutation was not admitted".to_owned())
            }
        };
        let PhysicalMutationOutcome::Completed(completed) = prepared.start().wait() else {
            return Err("ordinary C8 writer seed mutation did not complete".to_owned());
        };
        let [record] = completed.persisted_records() else {
            return Err(
                "ordinary C8 writer seed mutation completed with the wrong record count".to_owned(),
            );
        };
        receipts.push(identity_receipt::IdentityReceipt {
            material: operation.material(),
            idempotency,
            fate: 1,
            record: Some(*record),
        });
    }
    Ok(receipts)
}

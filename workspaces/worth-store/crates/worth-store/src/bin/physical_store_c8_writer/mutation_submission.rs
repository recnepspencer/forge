use super::mutation_material::{dirty_checkpoint_payload, mutation_payload};
use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    AdmittedRecordPlacementPolicy, PhysicalMutationDeadline, PhysicalMutationHandle,
    PhysicalMutationIdempotencyMaterial, PhysicalMutationPreparationSuccess,
    PhysicalMutationRequest, RecordAppendBatch, ServingPhysicalRuntime,
};

pub(super) fn start(
    serving: &ServingPhysicalRuntime,
    placement: AdmittedRecordPlacementPolicy,
    material: [u8; 32],
) -> Result<PhysicalMutationHandle, String> {
    start_with_payload(serving, placement, material, mutation_payload(material))
}

pub(super) fn start_dirty_checkpoint(
    serving: &ServingPhysicalRuntime,
    placement: AdmittedRecordPlacementPolicy,
    material: [u8; 32],
    payload_length: usize,
) -> Result<PhysicalMutationHandle, String> {
    start_with_payload(
        serving,
        placement,
        material,
        dirty_checkpoint_payload(material, payload_length),
    )
}

fn start_with_payload(
    serving: &ServingPhysicalRuntime,
    placement: AdmittedRecordPlacementPolicy,
    material: [u8; 32],
    payload: Vec<u8>,
) -> Result<PhysicalMutationHandle, String> {
    let submission = serving.record_submission();
    let key = submission
        .issue_idempotency_key(PhysicalMutationIdempotencyMaterial::new(material))
        .map_err(|denial| format!("C8 mutation identity denied: {denial:?}"))?;
    let batch = RecordAppendBatch::try_from_iter([payload])
        .map_err(|denial| format!("C8 mutation batch denied: {denial:?}"))?;
    let request = PhysicalMutationRequest::platform_durable(
        key,
        PhysicalMutationDeadline::after_milliseconds(30_000)
            .expect("C8 mutation deadline is nonzero"),
    );
    match submission
        .prepare_durable_append(batch, placement, request)
        .into_raw()
    {
        TransitionOutcome::Success(PhysicalMutationPreparationSuccess::Prepared(prepared)) => {
            Ok(prepared.start())
        }
        _ => Err("ordinary C8 mutation was not prepared".to_owned()),
    }
}

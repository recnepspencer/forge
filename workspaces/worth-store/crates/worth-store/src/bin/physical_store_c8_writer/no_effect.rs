use super::{identity_receipt, mutation_material::no_effect_material, mutation_submission::start};
use worth_store::physical_runtime::production::PhysicalMutationCheckpoint;
use worth_store::physical_runtime::{
    AdmittedRecordPlacementPolicy, PhysicalMutationCancellationOutcome, PhysicalMutationOutcome,
    ServingPhysicalRuntime,
};

pub(super) fn cancel_before_effect(
    serving: &ServingPhysicalRuntime,
    placement: AdmittedRecordPlacementPolicy,
    seed: u64,
    receipts: &mut Vec<identity_receipt::IdentityReceipt>,
) -> Result<(), String> {
    let gate = serving.pause_physical_mutation_at(PhysicalMutationCheckpoint::BeforeEffectCutover);
    let mutation = start(serving, placement, no_effect_material(seed))?;
    if !gate.await_arrival() {
        return Err(
            "ordinary C8 proven-no-effect mutation did not reach its pre-effect seam".to_owned(),
        );
    }
    if !matches!(
        mutation.request_cancellation(),
        PhysicalMutationCancellationOutcome::AcceptedBeforeEffect { .. }
    ) {
        return Err(
            "ordinary C8 proven-no-effect mutation cancellation was not accepted".to_owned(),
        );
    }
    gate.release();
    let idempotency = mutation.idempotency_identity().bytes();
    if !matches!(mutation.wait(), PhysicalMutationOutcome::ProvenNoEffect(_)) {
        return Err(
            "ordinary C8 cancellation did not produce a proven-no-effect terminal fact".to_owned(),
        );
    }
    receipts.push(identity_receipt::IdentityReceipt {
        material: no_effect_material(seed),
        idempotency,
        fate: 3,
        record: None,
    });
    Ok(())
}

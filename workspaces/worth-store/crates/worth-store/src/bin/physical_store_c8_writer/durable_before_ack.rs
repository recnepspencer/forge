use super::{
    identity_receipt, markers, mutation_material::dirty_material, mutation_submission::start,
    Invocation,
};
use worth_store::physical_runtime::production::PhysicalMutationCheckpoint;
use worth_store::physical_runtime::{AdmittedRecordPlacementPolicy, ServingPhysicalRuntime};

pub(super) fn hold_at_terminal_seam(
    serving: &ServingPhysicalRuntime,
    placement: AdmittedRecordPlacementPolicy,
    invocation: &Invocation,
    receipts: &mut Vec<identity_receipt::IdentityReceipt>,
) -> Result<(), String> {
    let gate =
        serving.pause_physical_mutation_at(PhysicalMutationCheckpoint::BeforeTerminalFinalization);
    let mutation = start(
        serving,
        placement,
        dirty_material(invocation.stage.perturbation_seed),
    )?;
    if !gate.await_arrival() {
        return Err("C8 durable-before-ack mutation did not reach terminal seam".to_owned());
    }
    receipts.push(identity_receipt::IdentityReceipt {
        material: dirty_material(invocation.stage.perturbation_seed),
        idempotency: mutation.idempotency_identity().bytes(),
        fate: 2,
        record: None,
    });
    identity_receipt::write(&invocation.identity_receipt, receipts)?;
    markers::write_ready(
        &invocation.start_marker,
        "write C8 durable-before-ack ready marker",
    )?;
    markers::wait_for_parent(&invocation.start_marker);
    markers::write_reached(
        &invocation.reached_marker,
        b"durable-before-ack",
        "write C8 durable-before-ack reached marker",
    )?;
    markers::park_forever();
}

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    PhysicalCheckpointDeadline, PhysicalCheckpointIdempotencyKey, PhysicalCheckpointOutcome,
    PhysicalCheckpointRequest, ServingPhysicalRuntime,
};

pub(super) fn complete_reliability_seed(serving: &ServingPhysicalRuntime) -> Result<(), String> {
    let deadline = PhysicalCheckpointDeadline::after_milliseconds(60_000)
        .ok_or_else(|| "bounded-residency checkpoint deadline was invalid".to_owned())?;
    let request = PhysicalCheckpointRequest::fuzzy(
        PhysicalCheckpointIdempotencyKey::new([0xc7; 32]),
        deadline,
    );
    let handle = match serving.checkpoints().start(request).into_raw() {
        TransitionOutcome::Success(handle) => handle,
        TransitionOutcome::Denied(cause) => {
            return Err(format!(
                "bounded-residency reliability seed checkpoint denied: {cause:?}"
            ))
        }
        TransitionOutcome::Deferred(cause) => {
            return Err(format!(
                "bounded-residency reliability seed checkpoint deferred: {cause:?}"
            ))
        }
        TransitionOutcome::Stale(cause) => {
            return Err(format!(
                "bounded-residency reliability seed checkpoint stale: {cause:?}"
            ))
        }
        TransitionOutcome::Failed(cause) => {
            return Err(format!(
                "bounded-residency reliability seed checkpoint failed to start: {cause:?}"
            ))
        }
    };
    match handle.wait() {
        PhysicalCheckpointOutcome::Completed(_) => Ok(()),
        outcome => Err(format!(
            "bounded-residency reliability seed checkpoint did not complete: {outcome:?}"
        )),
    }
}

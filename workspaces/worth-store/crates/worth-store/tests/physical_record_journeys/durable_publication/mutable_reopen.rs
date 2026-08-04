use sha2::{Digest, Sha256};
use worth_proof::TransitionOutcome;
use worth_signal::facade::TemporalDuration;
use worth_store::physical_runtime::{
    CompletedPhysicalCheckpoint, PhysicalCheckpointDeadline, PhysicalCheckpointIdempotencyKey,
    PhysicalCheckpointOutcome, PhysicalCheckpointRequest, ServingPhysicalRuntime,
};

pub(crate) fn checkpoint_for_mutable_reopen(
    serving: &ServingPhysicalRuntime,
    scenario: &'static str,
) -> CompletedPhysicalCheckpoint {
    let mut digest = Sha256::new();
    digest.update(b"worth-store.integration-mutable-reopen-checkpoint.v1");
    digest.update(scenario.as_bytes());
    let request = PhysicalCheckpointRequest::fuzzy(
        PhysicalCheckpointIdempotencyKey::new(digest.finalize().into()),
        PhysicalCheckpointDeadline::at(
            TemporalDuration::temporal_duration(10_000)
                .expect("the integration checkpoint deadline is nonzero"),
        ),
    );
    let handle = match serving.checkpoints().start(request).into_raw() {
        TransitionOutcome::Success(handle) => handle,
        TransitionOutcome::Denied(cause) => panic!("mutable-reopen checkpoint denied: {cause:?}"),
        TransitionOutcome::Deferred(cause) => {
            panic!("mutable-reopen checkpoint deferred: {cause:?}")
        }
        TransitionOutcome::Stale(cause) => panic!("mutable-reopen checkpoint stale: {cause:?}"),
        TransitionOutcome::Failed(cause) => {
            panic!("mutable-reopen checkpoint failed to start: {cause:?}")
        }
    };
    match handle.wait() {
        PhysicalCheckpointOutcome::Completed(completed) => completed,
        other => panic!("mutable-reopen checkpoint did not complete: {other:?}"),
    }
}

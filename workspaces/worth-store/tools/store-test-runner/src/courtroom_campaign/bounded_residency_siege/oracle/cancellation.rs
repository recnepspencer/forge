use super::super::protocol::{
    BoundedCancellationCaseObservation, BoundedCancellationDispatch, BoundedCancellationObligation,
    BoundedCancellationObservation, BoundedCancellationRecovery, BoundedCancellationSeam,
    BoundedCancellationSignal, BoundedCancellationTerminal, BoundedResidencyWorkEffectFate,
    BoundedResidencyWorkFamily, BoundedResidencyWorkReconciliationObservation,
    BoundedResidencyWorkRecovery, BoundedResidencyWorkTerminalFate,
};

pub(super) fn verify(
    cancellation: BoundedCancellationObservation,
    work: &BoundedResidencyWorkReconciliationObservation,
    store: [u8; 16],
    runtime: u64,
    generation: u64,
) -> Result<(), String> {
    verify_pre_dispatch(cancellation.pre_dispatch, store, runtime, generation)?;
    verify_post_dispatch(cancellation.post_dispatch, store, runtime, generation)?;
    if cancellation.pre_dispatch.operation == cancellation.post_dispatch.operation {
        return Err("A16 cancellation seams reused one physical work identity".to_owned());
    }
    let pre_records = matching_records(work, cancellation.pre_dispatch).count();
    if pre_records != 0 {
        return Err("A16 pre-dispatch cancellation produced a causal media record".to_owned());
    }
    let mut post_records = matching_records(work, cancellation.post_dispatch);
    let post = post_records
        .next()
        .ok_or_else(|| "A16 post-dispatch cancellation lost its causal work record".to_owned())?;
    if post_records.next().is_some() {
        return Err("A16 post-dispatch cancellation duplicated its causal work record".to_owned());
    }
    if post.family != BoundedResidencyWorkFamily::ArtifactRangeWrite
        || post.effect_fate != BoundedResidencyWorkEffectFate::WriteCompleted
        || post.recovery != BoundedResidencyWorkRecovery::ContinueSettlement
        || post.terminal != BoundedResidencyWorkTerminalFate::ContinuedAfterConsumerCancellation
        || Some(post.backend_operation) != cancellation.post_dispatch.backend_receipt
    {
        return Err(
            "A16 post-dispatch cancellation did not join its exact write settlement".to_owned(),
        );
    }
    if work.continued_terminal_fates != 1 {
        return Err(
            "A16 post-dispatch cancellation did not own the one continued terminal fate".to_owned(),
        );
    }
    Ok(())
}

fn verify_pre_dispatch(
    case: BoundedCancellationCaseObservation,
    store: [u8; 16],
    runtime: u64,
    generation: u64,
) -> Result<(), String> {
    if case.seam != BoundedCancellationSeam::PreDispatch {
        return Err("A16 pre-dispatch evidence named the wrong seam".to_owned());
    }
    verify_identity(case, store, runtime, generation, "pre-dispatch")?;
    if case.obligation != BoundedCancellationObligation::NotDispatched {
        return Err("A16 pre-dispatch cancellation lost NotDispatched".to_owned());
    }
    if case.signal != BoundedCancellationSignal::RequestCancelled {
        return Err("A16 pre-dispatch cancellation lacked Signal cancellation".to_owned());
    }
    if case.dispatch != BoundedCancellationDispatch::DeniedConsumerCancelled {
        return Err("A16 pre-dispatch work was not denied as ConsumerCancelled".to_owned());
    }
    if case.recovery != BoundedCancellationRecovery::NoSettlement {
        return Err("A16 pre-dispatch cancellation fabricated settlement recovery".to_owned());
    }
    if case.terminal != BoundedCancellationTerminal::CancelledBeforeDispatch {
        return Err("A16 pre-dispatch cancellation reached the wrong terminal fate".to_owned());
    }
    if case.media_before_cancellation != 0
        || case.cancellation_media_effects != 0
        || case.terminal_media_effects != 0
        || case.backend_receipt.is_some()
    {
        return Err("A16 pre-dispatch cancellation reached media".to_owned());
    }
    Ok(())
}

fn verify_post_dispatch(
    case: BoundedCancellationCaseObservation,
    store: [u8; 16],
    runtime: u64,
    generation: u64,
) -> Result<(), String> {
    if case.seam != BoundedCancellationSeam::PostDispatch {
        return Err("A16 post-dispatch evidence named the wrong seam".to_owned());
    }
    verify_identity(case, store, runtime, generation, "post-dispatch")?;
    if case.obligation != BoundedCancellationObligation::SettlementContinues {
        return Err("A16 post-dispatch cancellation lost SettlementContinues".to_owned());
    }
    if case.signal != BoundedCancellationSignal::ReconciledFromPhysicalTruth {
        return Err(
            "A16 post-dispatch settlement did not reconcile from physical truth".to_owned(),
        );
    }
    if case.dispatch != BoundedCancellationDispatch::WriteCompleted {
        return Err("A16 post-dispatch work did not complete its write".to_owned());
    }
    if case.recovery != BoundedCancellationRecovery::ContinueSettlement {
        return Err("A16 post-dispatch work lost ContinueSettlement".to_owned());
    }
    if case.terminal != BoundedCancellationTerminal::ContinuedAfterConsumerCancellation {
        return Err("A16 post-dispatch work reached the wrong terminal fate".to_owned());
    }
    if case.media_before_cancellation != 1
        || case.cancellation_media_effects != 0
        || case.terminal_media_effects != 1
        || case.backend_receipt.is_none()
    {
        return Err("A16 post-dispatch media and cancellation deltas did not reconcile".to_owned());
    }
    Ok(())
}

fn verify_identity(
    case: BoundedCancellationCaseObservation,
    store: [u8; 16],
    runtime: u64,
    generation: u64,
    seam: &str,
) -> Result<(), String> {
    if case.store != store
        || case.runtime != runtime
        || case.generation != generation
        || case.operation == 0
    {
        return Err(format!(
            "A16 {seam} cancellation carried a foreign physical work identity"
        ));
    }
    Ok(())
}

fn matching_records(
    evidence: &BoundedResidencyWorkReconciliationObservation,
    case: BoundedCancellationCaseObservation,
) -> impl Iterator<Item = &super::super::protocol::BoundedResidencyWorkRecordObservation> {
    evidence.records.iter().filter(move |record| {
        record.store == case.store
            && record.runtime == case.runtime
            && record.generation == case.generation
            && record.operation == case.operation
    })
}

#[cfg(test)]
#[path = "cancellation/tests.rs"]
mod tests;

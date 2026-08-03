use worth_store::physical_runtime::{
    PhysicalEffectObligation, PhysicalWorkPreEffectDenial, PhysicalWritebackFailureCause,
    ServingPhysicalRuntime,
};
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;

use super::{
    positioned_writes, same_bytes_dirty_frame, CancellationDispatchOutcome,
    CancellationRecoveryOutcome, CancellationSignalOutcome, PendingCancellationCase,
};

pub(super) fn exercise(
    serving: &ServingPhysicalRuntime,
    residency: &worth_store::physical_runtime::PhysicalResidencyCertification,
) -> Result<PendingCancellationCase, String> {
    let dirty_before = residency.counters().dirty_frames();
    let receipts_before = serving
        .residency_observation()
        .writebacks()
        .exact_receipts();
    let dirty = same_bytes_dirty_frame(residency)?;
    let scheduler_before = serving.physical_scheduler_capacity();
    let media_baseline = positioned_writes(serving);
    let ready = residency
        .request_writeback(
            residency
                .prepare_writeback(
                    dirty,
                    ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
                )
                .map_err(|failure| {
                    format!("pre-dispatch cancellation preparation failed: {failure:?}")
                })?,
        )
        .map_err(|failure| format!("pre-dispatch cancellation readiness failed: {failure:?}"))?;
    let identity = ready.identity();
    let consumer = ready.consumer_handle();
    let signal_request = consumer.signal_request();
    let admitted = residency.admit_writeback(ready).map_err(|failure| {
        format!("pre-dispatch cancellation scheduler admission failed: {failure:?}")
    })?;
    let before_cancellation = positioned_writes(serving);
    let cancellation = serving
        .cancel_physical_work(consumer)
        .map_err(|failure| format!("pre-dispatch cancellation failed: {failure:?}"))?;
    let after_cancellation = positioned_writes(serving);
    let signal_cancelled = cancellation
        .signal()
        .cancelled_request()
        .is_some_and(|cancelled| cancelled.handle() == signal_request);
    let obligation = cancellation.obligation();
    let failure = residency.execute_writeback(admitted).err().ok_or_else(|| {
        "pre-dispatch cancelled writeback reached execution successfully".to_owned()
    })?;
    let cause = failure.cause();
    failure
        .into_dirty()
        .discard()
        .map_err(|failure| format!("pre-dispatch dirty cleanup failed: {failure:?}"))?;
    let terminal_media = positioned_writes(serving);
    let receipts_after = serving
        .residency_observation()
        .writebacks()
        .exact_receipts();
    let scheduler_after = serving.physical_scheduler_capacity();
    let scheduler_reconciled = scheduler_after.configured() == scheduler_before.configured()
        && scheduler_after.available() == scheduler_before.available()
        && scheduler_after.active_reservations() == scheduler_before.active_reservations()
        && scheduler_after.admitted_reservations()
            == scheduler_before.admitted_reservations().saturating_add(1)
        && scheduler_after.released_reservations()
            == scheduler_before.released_reservations().saturating_add(1)
        && scheduler_after.denied_reservations() == scheduler_before.denied_reservations();
    if obligation != PhysicalEffectObligation::NotDispatched
        || !signal_cancelled
        || cause
            != PhysicalWritebackFailureCause::PreEffect(
                PhysicalWorkPreEffectDenial::ConsumerCancelled,
            )
        || before_cancellation != media_baseline
        || after_cancellation != before_cancellation
        || terminal_media != media_baseline
        || receipts_after != receipts_before
        || residency.counters().dirty_frames() != dirty_before
        || !scheduler_reconciled
    {
        return Err(format!(
            "BOUNDED_RESIDENCY_PREDICATE:predispatch-cancellation-dispatches: \
             obligation={obligation:?}, signal_cancelled={signal_cancelled}, cause={cause:?}, \
             media_baseline={media_baseline}, before_cancellation={before_cancellation}, \
             after_cancellation={after_cancellation}, terminal_media={terminal_media}, \
             receipts_before={receipts_before}, receipts_after={receipts_after}, \
             scheduler_before={scheduler_before:?}, scheduler_after={scheduler_after:?}"
        ));
    }
    Ok(PendingCancellationCase {
        identity,
        obligation,
        signal: CancellationSignalOutcome::RequestCancelled,
        dispatch: CancellationDispatchOutcome::DeniedConsumerCancelled,
        recovery: CancellationRecoveryOutcome::NoSettlement,
        media_before_cancellation: before_cancellation.saturating_sub(media_baseline),
        cancellation_media_effects: after_cancellation.saturating_sub(before_cancellation),
        terminal_media_effects: terminal_media.saturating_sub(media_baseline),
        backend_receipt: None,
    })
}

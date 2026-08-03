use worth_store::physical_runtime::{
    certification::CertificationPhysicalExecutionCheckpoint, PhysicalEffectObligation,
    PhysicalSignalSettlementOutcome, PhysicalWorkEffectFate, PhysicalWorkRecoveryDisposition,
    PhysicalWritebackExecution, ServingPhysicalRuntime,
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
    let media_baseline = positioned_writes(serving);
    let ready = residency
        .request_writeback(
            residency
                .prepare_writeback(
                    dirty,
                    ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
                )
                .map_err(|failure| {
                    format!("post-dispatch cancellation preparation failed: {failure:?}")
                })?,
        )
        .map_err(|failure| format!("post-dispatch cancellation readiness failed: {failure:?}"))?;
    let identity = ready.identity();
    let consumer = ready.consumer_handle();
    let signal_request = consumer.signal_request();
    let admitted = residency.admit_writeback(ready).map_err(|failure| {
        format!("post-dispatch cancellation scheduler admission failed: {failure:?}")
    })?;
    let gate = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::AfterResidencyWriteBeforeSchedulerSettlement,
    );
    let (cancellation, execution, before_cancellation, after_cancellation) =
        std::thread::scope(|scope| {
            let execution = scope.spawn(|| residency.execute_writeback(admitted));
            if !gate.await_arrival() {
                gate.release();
                let _ = execution.join();
                return Err(
                    "post-dispatch cancellation did not reach the backend-write checkpoint"
                        .to_owned(),
                );
            }
            let before_cancellation = positioned_writes(serving);
            let cancellation = serving.cancel_physical_work(consumer);
            let after_cancellation = positioned_writes(serving);
            gate.release();
            let execution = execution
                .join()
                .map_err(|_| "post-dispatch writeback panicked".to_owned())?;
            Ok((
                cancellation
                    .map_err(|failure| format!("post-dispatch cancellation failed: {failure:?}"))?,
                execution.map_err(|failure| {
                    format!("post-dispatch writeback execution failed: {failure:?}")
                })?,
                before_cancellation,
                after_cancellation,
            ))
        })?;
    let signal_cancelled = cancellation
        .signal()
        .cancelled_request()
        .is_some_and(|cancelled| cancelled.handle() == signal_request);
    let obligation = cancellation.obligation();
    let settlement = match execution {
        PhysicalWritebackExecution::Clean(settlement) => settlement,
        PhysicalWritebackExecution::Retryable(_) => {
            return Err("post-dispatch cancellation produced a retryable writeback".to_owned())
        }
        PhysicalWritebackExecution::InspectionRequired(_) => {
            return Err(
                "post-dispatch cancellation produced inspection-required writeback".to_owned(),
            )
        }
    };
    let effect = settlement
        .effect()
        .ok_or_else(|| "post-dispatch cancellation lost its backend receipt".to_owned())?;
    let terminal_media = positioned_writes(serving);
    let receipts_after = serving
        .residency_observation()
        .writebacks()
        .exact_receipts();
    if obligation != PhysicalEffectObligation::SettlementContinues
        || !signal_cancelled
        || settlement.identity() != identity
        || effect.work() != identity
        || settlement.effect_fate() != PhysicalWorkEffectFate::WriteCompleted
        || settlement.recovery() != PhysicalWorkRecoveryDisposition::ContinueSettlement
        || settlement.signal() != PhysicalSignalSettlementOutcome::ReconciledFromPhysicalTruth
        || before_cancellation != media_baseline.saturating_add(1)
        || after_cancellation != before_cancellation
        || terminal_media != before_cancellation
        || receipts_after != receipts_before.saturating_add(1)
        || residency.counters().dirty_frames() != dirty_before
    {
        return Err(format!(
            "BOUNDED_RESIDENCY_PREDICATE:postdispatch-cancellation-loses-terminal-fate: \
             obligation={obligation:?}, signal_cancelled={signal_cancelled}, \
             identity={identity:?}, settlement_identity={:?}, effect={effect:?}, \
             fate={:?}, recovery={:?}, signal={:?}, media_baseline={media_baseline}, \
             before_cancellation={before_cancellation}, \
             after_cancellation={after_cancellation}, terminal_media={terminal_media}, \
             receipts_before={receipts_before}, receipts_after={receipts_after}",
            settlement.identity(),
            settlement.effect_fate(),
            settlement.recovery(),
            settlement.signal(),
        ));
    }
    Ok(PendingCancellationCase {
        identity,
        obligation,
        signal: CancellationSignalOutcome::ReconciledFromPhysicalTruth,
        dispatch: CancellationDispatchOutcome::WriteCompleted,
        recovery: CancellationRecoveryOutcome::ContinueSettlement,
        media_before_cancellation: before_cancellation.saturating_sub(media_baseline),
        cancellation_media_effects: after_cancellation.saturating_sub(before_cancellation),
        terminal_media_effects: terminal_media.saturating_sub(media_baseline),
        backend_receipt: Some(effect.backend_operation().value()),
    })
}

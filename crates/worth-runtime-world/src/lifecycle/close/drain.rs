use crate::lifecycle::owner::{RuntimeWorldBootstrapState, RuntimeWorldOwnerState};

use super::report::{
    RuntimeWorldCloseReleaseCounts, RuntimeWorldCloseReport, RuntimeWorldRetainedRecordReport,
};
use super::RuntimeWorldCloseDenial;

/// Admit a close attempt. A declared critical section that is still in flight
/// cannot be drained, so it denies here rather than closing over live work.
pub(super) fn admit_close<D, I, E, Ctx, T>(
    state: &RuntimeWorldOwnerState<D, I, E, Ctx, T>,
) -> Result<(), RuntimeWorldCloseDenial>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    let bootstrap = state
        .bootstrap
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    if *bootstrap == RuntimeWorldBootstrapState::InProgress {
        return Err(RuntimeWorldCloseDenial::AlreadyClosing);
    }
    drop(bootstrap);
    let operation = state
        .operation
        .state
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    if operation.recovery_active != 0 {
        return Err(RuntimeWorldCloseDenial::InFlightCriticalSection);
    }
    if operation.active != 0 {
        return Err(RuntimeWorldCloseDenial::AlreadyClosing);
    }
    drop(operation);
    if state.recovery.reserved_slots() != 0 {
        return Err(RuntimeWorldCloseDenial::AlreadyClosing);
    }
    Ok(())
}

/// Settle or expose every retained owner obligation and release every pin the
/// owner still holds.
///
/// LANE D owns the settling and pin-release body. Until it lands, this drain
/// keeps the pre-Phase-4 behaviour: an installed retained record is refused as
/// an undrainable critical section instead of being enumerated into a report
/// row, and a close that proceeds has provably nothing to enumerate, so the
/// report it returns is an honest empty one rather than a fabricated summary.
pub(super) fn drain_for_close<D, I, E, Ctx, T>(
    state: &RuntimeWorldOwnerState<D, I, E, Ctx, T>,
) -> Result<RuntimeWorldCloseReport, RuntimeWorldCloseDenial>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    if state.recovery.installed_slots() != 0 {
        return Err(RuntimeWorldCloseDenial::InFlightCriticalSection);
    }
    let retained_records: Vec<RuntimeWorldRetainedRecordReport> = Vec::new();
    Ok(RuntimeWorldCloseReport::new(
        retained_records,
        RuntimeWorldCloseReleaseCounts::default(),
        Vec::new(),
    ))
}

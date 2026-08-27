use std::time::Instant;

use worth_ui::facade::app::{
    UiMountedInspectionOmission, UiMountedInspectionReceipt, UiMountedInspectionRequest,
    WorthUiNativeApplicationShell,
};

use super::{
    comparison, progression, PlatformPulseRetainedSnapshot, PlatformPulseVisualExecutionDenial,
    PlatformPulseVisualIdentityState, PlatformPulseVisualRefreshCapture,
    NATIVE_CAPTURE_POLL_INTERVAL, REPLACEMENT_FRAME_DEADLINE,
};

pub(super) fn advance_awaiting_capture(
    shell: &mut WorthUiNativeApplicationShell,
    tick: u64,
    now: Instant,
    deadline: Instant,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    if now >= deadline {
        return Err(PlatformPulseVisualExecutionDenial::SnapshotDeadline);
    }
    if mounted_frame_ready(shell)? {
        return progression::begin_capture_before(shell, tick, deadline)
            .map(PlatformPulseVisualIdentityState::Capturing);
    }
    Ok(PlatformPulseVisualIdentityState::AwaitingCapture { deadline })
}

pub(super) fn advance_awaiting_rebase(
    shell: &mut WorthUiNativeApplicationShell,
    tick: u64,
    now: Instant,
    deadline: Instant,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    if now >= deadline {
        return Err(PlatformPulseVisualExecutionDenial::SnapshotDeadline);
    }
    if mounted_frame_ready(shell)? {
        return progression::begin_capture_before(shell, tick, deadline)
            .map(PlatformPulseVisualIdentityState::Rebasing);
    }
    Ok(PlatformPulseVisualIdentityState::AwaitingRebase { deadline })
}

pub(super) fn advance_awaiting_refresh(
    shell: &mut WorthUiNativeApplicationShell,
    tick: u64,
    now: Instant,
    predecessor: PlatformPulseRetainedSnapshot,
    deadline: Instant,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    if now >= deadline {
        return Err(PlatformPulseVisualExecutionDenial::SnapshotDeadline);
    }
    if mounted_frame_ready(shell)? {
        let capture = progression::begin_capture_before(shell, tick, deadline)?;
        return Ok(PlatformPulseVisualIdentityState::Refreshing(
            PlatformPulseVisualRefreshCapture {
                capture,
                predecessor,
            },
        ));
    }
    Ok(PlatformPulseVisualIdentityState::AwaitingRefresh {
        predecessor,
        deadline,
    })
}

pub(super) fn advance_awaiting_comparison(
    shell: &mut WorthUiNativeApplicationShell,
    tick: u64,
    now: Instant,
    predecessor: PlatformPulseRetainedSnapshot,
    rebind: worth_ui::facade::rebind::UiRebindReceipt,
    deadline: Instant,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    if now >= deadline {
        return Err(PlatformPulseVisualExecutionDenial::SnapshotDeadline);
    }
    if mounted_frame_ready(shell)? {
        let capture = progression::begin_capture_before(shell, tick, deadline)?;
        return Ok(PlatformPulseVisualIdentityState::Comparing(
            comparison::begin(predecessor, rebind, capture),
        ));
    }
    Ok(PlatformPulseVisualIdentityState::AwaitingComparison {
        predecessor,
        rebind,
        deadline,
    })
}

pub(super) fn mounted_frame_ready(
    shell: &WorthUiNativeApplicationShell,
) -> Result<bool, PlatformPulseVisualExecutionDenial> {
    match shell.inspect_mounted_frame(UiMountedInspectionRequest::current()) {
        UiMountedInspectionReceipt::Available(_) => Ok(true),
        UiMountedInspectionReceipt::Omitted(
            UiMountedInspectionOmission::FrameTransitionInFlight,
        ) => Ok(false),
        UiMountedInspectionReceipt::Omitted(omission) => {
            Err(PlatformPulseVisualExecutionDenial::MountedFrameReadinessUnavailable(omission))
        }
    }
}

pub(super) fn replacement_frame_deadline(
    now: Instant,
) -> Result<Instant, PlatformPulseVisualExecutionDenial> {
    now.checked_add(REPLACEMENT_FRAME_DEADLINE)
        .ok_or(PlatformPulseVisualExecutionDenial::ClockOverflow)
}

pub(super) fn next_capture_poll(deadline: Instant) -> Instant {
    Instant::now()
        .checked_add(NATIVE_CAPTURE_POLL_INTERVAL)
        .map_or(deadline, |poll| poll.min(deadline))
}

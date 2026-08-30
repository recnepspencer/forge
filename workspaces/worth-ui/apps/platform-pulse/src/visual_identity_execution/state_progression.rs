use std::time::Instant;

use worth_ui::facade::app::{
    UiMountedInspectionOmission, UiMountedInspectionReceipt, UiMountedInspectionRequest,
    WorthUiNativeApplicationShell,
};

use super::{
    capture_restart::PlatformPulseAwaitingCaptureBudget, comparison, progression,
    PlatformPulseRetainedSnapshot, PlatformPulseVisualCapturePhase,
    PlatformPulseVisualExecutionDenial, PlatformPulseVisualIdentityState,
    PlatformPulseVisualRefreshCapture, NATIVE_CAPTURE_POLL_INTERVAL, REPLACEMENT_FRAME_DEADLINE,
};

pub(super) fn advance_awaiting_capture(
    shell: &mut WorthUiNativeApplicationShell,
    tick: u64,
    now: Instant,
    budget: PlatformPulseAwaitingCaptureBudget,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    let deadline = budget.readiness_deadline();
    if now >= deadline {
        return Err(PlatformPulseVisualExecutionDenial::SnapshotDeadline(
            PlatformPulseVisualCapturePhase::InitialReadiness,
        ));
    }
    if mounted_frame_ready(shell)? {
        return progression::begin_capture_before(shell, tick, budget.capture_deadline(now)?)
            .map(PlatformPulseVisualIdentityState::Capturing);
    }
    Ok(PlatformPulseVisualIdentityState::AwaitingCapture { budget })
}

pub(super) fn advance_awaiting_rebase(
    shell: &mut WorthUiNativeApplicationShell,
    tick: u64,
    now: Instant,
    budget: PlatformPulseAwaitingCaptureBudget,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    let deadline = budget.readiness_deadline();
    if now >= deadline {
        return Err(PlatformPulseVisualExecutionDenial::SnapshotDeadline(
            PlatformPulseVisualCapturePhase::RebaseReadiness,
        ));
    }
    if mounted_frame_ready(shell)? {
        return progression::begin_capture_before(shell, tick, budget.capture_deadline(now)?)
            .map(PlatformPulseVisualIdentityState::Rebasing);
    }
    Ok(PlatformPulseVisualIdentityState::AwaitingRebase { budget })
}

pub(super) fn advance_awaiting_refresh(
    shell: &mut WorthUiNativeApplicationShell,
    tick: u64,
    now: Instant,
    predecessor: PlatformPulseRetainedSnapshot,
    budget: PlatformPulseAwaitingCaptureBudget,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    let deadline = budget.readiness_deadline();
    if now >= deadline {
        shell.dispose_visual_snapshot(predecessor.snapshot);
        return Err(PlatformPulseVisualExecutionDenial::SnapshotDeadline(
            PlatformPulseVisualCapturePhase::RefreshReadiness,
        ));
    }
    if mounted_frame_ready(shell)? {
        let capture =
            progression::begin_capture_before(shell, tick, budget.capture_deadline(now)?)?;
        return Ok(PlatformPulseVisualIdentityState::Refreshing(
            PlatformPulseVisualRefreshCapture {
                capture,
                predecessor,
            },
        ));
    }
    Ok(PlatformPulseVisualIdentityState::AwaitingRefresh {
        predecessor,
        budget,
    })
}

pub(super) fn advance_awaiting_comparison(
    shell: &mut WorthUiNativeApplicationShell,
    tick: u64,
    now: Instant,
    predecessor: PlatformPulseRetainedSnapshot,
    rebind: worth_ui::facade::rebind::UiRebindReceipt,
    budget: PlatformPulseAwaitingCaptureBudget,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    let deadline = budget.readiness_deadline();
    if now >= deadline {
        shell.dispose_visual_snapshot(predecessor.snapshot);
        drop(rebind);
        return Err(PlatformPulseVisualExecutionDenial::SnapshotDeadline(
            PlatformPulseVisualCapturePhase::ComparisonReadiness,
        ));
    }
    if mounted_frame_ready(shell)? {
        let capture =
            progression::begin_capture_before(shell, tick, budget.capture_deadline(now)?)?;
        return Ok(PlatformPulseVisualIdentityState::Comparing(
            comparison::begin(predecessor, rebind, capture),
        ));
    }
    Ok(PlatformPulseVisualIdentityState::AwaitingComparison {
        predecessor,
        rebind,
        budget,
    })
}

pub(super) fn mounted_frame_ready(
    shell: &WorthUiNativeApplicationShell,
) -> Result<bool, PlatformPulseVisualExecutionDenial> {
    mounted_frame_readiness(shell.inspect_mounted_frame(UiMountedInspectionRequest::current()))
}

fn mounted_frame_readiness(
    receipt: UiMountedInspectionReceipt,
) -> Result<bool, PlatformPulseVisualExecutionDenial> {
    match receipt {
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

pub(super) fn initial_readiness_expired(now: Instant, deadline: Instant) -> bool {
    now >= deadline
}

pub(super) fn next_capture_wake(deadline: Instant, replacement_pending: bool) -> Instant {
    if replacement_pending {
        deadline
    } else {
        next_capture_poll(deadline)
    }
}

pub(super) fn next_frame_readiness_poll(deadline: Instant) -> Instant {
    next_capture_poll(deadline)
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};
    use worth_ui::facade::app::{UiMountedInspectionOmission, UiMountedInspectionReceipt};

    #[test]
    fn replacement_pending_capture_waits_for_physical_readiness_or_its_owned_deadline() {
        let deadline = Instant::now() + Duration::from_secs(5);
        assert_eq!(super::next_capture_wake(deadline, true), deadline);
    }

    #[test]
    fn initial_capture_cannot_admit_after_its_readiness_deadline() {
        let deadline = Instant::now();
        assert!(super::initial_readiness_expired(deadline, deadline));
        assert!(!super::initial_readiness_expired(
            deadline - Duration::from_nanos(1),
            deadline,
        ));
    }

    #[test]
    fn in_flight_mounted_frame_remains_pending_without_a_wall_clock_gate() {
        let readiness = super::mounted_frame_readiness(UiMountedInspectionReceipt::Omitted(
            UiMountedInspectionOmission::FrameTransitionInFlight,
        ))
        .expect("transition in flight is a retryable typed posture");

        assert!(!readiness);
    }
}

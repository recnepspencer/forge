use std::time::{Duration, Instant};

use worth_ui::facade::app::{
    UiMountedInspectionReceipt, UiMountedInspectionRequest, WorthUiNativeApplicationShell,
};
use worth_ui::facade::inspection::{
    UiPixelsRequired, UiVisualCaptureDeadline, UiVisualCapturePoll, UiVisualSnapshotOutcome,
    UiVisualSnapshotReceipt,
};
use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseVisualPointObservation, PlatformPulseVisualPointTraceInput,
};

use crate::lifecycle_observation_publication::PlatformPulseObservationPublisher;
use crate::visual_identity_adjudication::adjudicate_points;

use super::{
    comparison, PlatformPulseRetainedSnapshot, PlatformPulseVisibleOverlay,
    PlatformPulseVisualCapture, PlatformPulseVisualExecutionDenial,
    PlatformPulseVisualIdentityState, PlatformPulseVisualRefreshCapture,
};

#[path = "progression/retirement.rs"]
mod retirement;
pub(super) use retirement::retire_snapshot;
use retirement::{next_presentation_ticks, retain_refreshed_snapshot, retire_refresh_predecessor};

const CAPTURE_WALL_DEADLINE: Duration = Duration::from_secs(5);
const OVERLAY_VISIBLE_DWELL: Duration = Duration::from_secs(2);
const CAPTURE_TICK_ALLOWANCE: u64 = 1_000;

pub(super) enum PlatformPulseVisualCaptureResolution {
    Captured(UiVisualSnapshotReceipt<UiPixelsRequired>),
    RetryBefore { deadline: Instant },
}

pub(super) fn advance_state(
    state: PlatformPulseVisualIdentityState,
    shell: &mut WorthUiNativeApplicationShell,
    publisher: &PlatformPulseObservationPublisher,
    tick: &mut u64,
    now: Instant,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    match state {
        PlatformPulseVisualIdentityState::Settling { begin_at, deadline } if now >= begin_at => {
            match begin_capture_before(shell, *tick, deadline) {
                Ok(capture) => Ok(PlatformPulseVisualIdentityState::Capturing(capture)),
                Err(PlatformPulseVisualExecutionDenial::CaptureMountedFrameUnavailable(
                    worth_ui::facade::app::UiMountedInspectionOmission::FrameTransitionInFlight,
                )) if now < deadline => Ok(PlatformPulseVisualIdentityState::Settling {
                    begin_at: now
                        .checked_add(Duration::from_millis(1))
                        .map_or(deadline, |poll| poll.min(deadline)),
                    deadline,
                }),
                Err(denial) => Err(denial),
            }
        }
        PlatformPulseVisualIdentityState::Capturing(capture) => {
            poll_capture(capture, shell, publisher, tick, now)
        }
        PlatformPulseVisualIdentityState::Comparing(comparison) => {
            comparison::poll(comparison, shell, publisher, tick, now)
        }
        PlatformPulseVisualIdentityState::Rebasing(capture) => {
            poll_rebase(capture, shell, publisher, tick, now)
        }
        PlatformPulseVisualIdentityState::Refreshing(refresh) => {
            poll_refresh(refresh, shell, publisher, tick, now)
        }
        PlatformPulseVisualIdentityState::OverlayVisible(overlay) if now >= overlay.clear_at => {
            clear_overlay(overlay, shell, publisher, tick)
        }
        PlatformPulseVisualIdentityState::Transitioning => {
            Ok(PlatformPulseVisualIdentityState::Transitioning)
        }
        state => Ok(state),
    }
}

pub(super) fn begin_capture(
    shell: &mut WorthUiNativeApplicationShell,
    tick: u64,
    now: Instant,
) -> Result<PlatformPulseVisualCapture, PlatformPulseVisualExecutionDenial> {
    let deadline = now
        .checked_add(CAPTURE_WALL_DEADLINE)
        .ok_or(PlatformPulseVisualExecutionDenial::ClockOverflow)?;
    begin_capture_before(shell, tick, deadline)
}

pub(super) fn begin_capture_before(
    shell: &mut WorthUiNativeApplicationShell,
    tick: u64,
    deadline: Instant,
) -> Result<PlatformPulseVisualCapture, PlatformPulseVisualExecutionDenial> {
    let frame = match shell.inspect_mounted_frame(UiMountedInspectionRequest::current()) {
        UiMountedInspectionReceipt::Available(frame) => frame,
        UiMountedInspectionReceipt::Omitted(omission) => {
            return Err(
                PlatformPulseVisualExecutionDenial::CaptureMountedFrameUnavailable(omission),
            )
        }
    };
    let target = frame
        .current_visual_target()
        .map_err(|_| PlatformPulseVisualExecutionDenial::MountedVisualTarget)?;
    let grant = shell.visual_inspection_authority().issue_pixel_grant();
    let deadline_tick = tick
        .checked_add(CAPTURE_TICK_ALLOWANCE)
        .ok_or(PlatformPulseVisualExecutionDenial::TickExhausted)?;
    let request = worth_ui::facade::inspection::UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
        .artifacts(UiPixelsRequired::policy())
        .deadline(UiVisualCaptureDeadline::at_tick(deadline_tick));
    let pending = shell
        .begin_visual_pixel_snapshot(&grant, request)
        .map_err(PlatformPulseVisualExecutionDenial::SnapshotAdmission)?;
    Ok(PlatformPulseVisualCapture { pending, deadline })
}

fn poll_capture(
    capture: PlatformPulseVisualCapture,
    shell: &mut WorthUiNativeApplicationShell,
    publisher: &PlatformPulseObservationPublisher,
    tick: &mut u64,
    now: Instant,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    if now >= capture.deadline {
        shell.cancel_visual_snapshot(capture.pending);
        return Err(PlatformPulseVisualExecutionDenial::SnapshotDeadline);
    }
    match shell.poll_visual_snapshot(capture.pending, *tick) {
        UiVisualCapturePoll::Pending(pending) => Ok(PlatformPulseVisualIdentityState::Capturing(
            PlatformPulseVisualCapture {
                pending,
                deadline: capture.deadline,
            },
        )),
        UiVisualCapturePoll::Completed(outcome) => {
            match resolve_capture(outcome, capture.deadline)? {
                PlatformPulseVisualCaptureResolution::Captured(receipt) => {
                    if !snapshot_matches_current_mounted_frame(&receipt, shell)? {
                        shell.dispose_visual_snapshot(receipt);
                        return restart_capture_after_transition(shell, *tick, capture.deadline);
                    }
                    publish_overlay(receipt, shell, publisher, tick, now)
                }
                PlatformPulseVisualCaptureResolution::RetryBefore { deadline } => {
                    restart_capture_after_transition(shell, *tick, deadline)
                }
            }
        }
    }
}

pub(super) fn resolve_capture(
    outcome: UiVisualSnapshotOutcome<UiPixelsRequired>,
    deadline: Instant,
) -> Result<PlatformPulseVisualCaptureResolution, PlatformPulseVisualExecutionDenial> {
    match outcome {
        UiVisualSnapshotOutcome::Captured(receipt) => {
            Ok(PlatformPulseVisualCaptureResolution::Captured(receipt))
        }
        UiVisualSnapshotOutcome::Superseded(_) => {
            Ok(PlatformPulseVisualCaptureResolution::RetryBefore { deadline })
        }
        UiVisualSnapshotOutcome::Omitted(_) => {
            Err(PlatformPulseVisualExecutionDenial::SnapshotOmitted)
        }
        UiVisualSnapshotOutcome::Denied(denial) => {
            Err(PlatformPulseVisualExecutionDenial::SnapshotDenied(denial))
        }
        UiVisualSnapshotOutcome::Indeterminate(_) => {
            Err(PlatformPulseVisualExecutionDenial::SnapshotIndeterminate)
        }
    }
}

fn publish_overlay(
    receipt: UiVisualSnapshotReceipt<UiPixelsRequired>,
    shell: &mut WorthUiNativeApplicationShell,
    publisher: &PlatformPulseObservationPublisher,
    tick: &mut u64,
    now: Instant,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    publisher
        .visual_snapshot(&receipt)
        .map_err(PlatformPulseVisualExecutionDenial::Observation)?;
    let points = adjudicate_points(&receipt)?;
    publisher
        .visual_point_trace(PlatformPulseVisualPointTraceInput::new(
            &receipt,
            PlatformPulseVisualPointObservation::new(points.target_point, &points.target),
            PlatformPulseVisualPointObservation::new(points.background_point, &points.background),
        ))
        .map_err(PlatformPulseVisualExecutionDenial::Observation)?;
    let target = receipt
        .overlay_target(&points.selected_target)
        .map_err(PlatformPulseVisualExecutionDenial::OverlayTarget)?;
    let grant = shell.visual_inspection_authority().issue_overlay_grant();
    let pending = shell
        .show_identity_overlay(&grant, target)
        .map_err(PlatformPulseVisualExecutionDenial::OverlayAdmission)?;
    let (deadline, current) = next_presentation_ticks(tick)?;
    let published = shell
        .present_visual_overlay(pending, deadline, current)
        .map_err(|failure| {
            PlatformPulseVisualExecutionDenial::OverlayPublication(failure.denial())
        })?;
    publisher
        .visual_overlay_published(&published)
        .map_err(PlatformPulseVisualExecutionDenial::Observation)?;
    let clear_at = now
        .checked_add(OVERLAY_VISIBLE_DWELL)
        .ok_or(PlatformPulseVisualExecutionDenial::ClockOverflow)?;
    Ok(PlatformPulseVisualIdentityState::OverlayVisible(
        PlatformPulseVisibleOverlay {
            retained: PlatformPulseRetainedSnapshot {
                snapshot: receipt,
                overlay_clear: None,
            },
            published,
            clear_at,
        },
    ))
}

fn clear_overlay(
    overlay: PlatformPulseVisibleOverlay,
    shell: &mut WorthUiNativeApplicationShell,
    publisher: &PlatformPulseObservationPublisher,
    tick: &mut u64,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    let (deadline, current) = next_presentation_ticks(tick)?;
    let cleared = shell
        .clear_visual_overlay(overlay.published, deadline, current)
        .map_err(|failure| PlatformPulseVisualExecutionDenial::OverlayClear(failure.denial()))?;
    publisher
        .visual_overlay_cleared(cleared)
        .map_err(PlatformPulseVisualExecutionDenial::Observation)?;
    let mut retained = overlay.retained;
    retained.overlay_clear = Some(cleared);
    Ok(PlatformPulseVisualIdentityState::ComparisonReady(retained))
}

fn poll_refresh(
    mut refresh: PlatformPulseVisualRefreshCapture,
    shell: &mut WorthUiNativeApplicationShell,
    publisher: &PlatformPulseObservationPublisher,
    tick: &mut u64,
    now: Instant,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    let capture_deadline = refresh.capture.deadline;
    if now >= capture_deadline {
        shell.cancel_visual_snapshot(refresh.capture.pending);
        return Err(PlatformPulseVisualExecutionDenial::SnapshotDeadline);
    }
    match shell.poll_visual_snapshot(refresh.capture.pending, *tick) {
        UiVisualCapturePoll::Pending(pending) => {
            refresh.capture.pending = pending;
            Ok(PlatformPulseVisualIdentityState::Refreshing(refresh))
        }
        UiVisualCapturePoll::Completed(outcome) => {
            match resolve_capture(outcome, capture_deadline)? {
                PlatformPulseVisualCaptureResolution::Captured(successor) => {
                    if !snapshot_matches_current_mounted_frame(&successor, shell)? {
                        shell.dispose_visual_snapshot(successor);
                        return restart_refresh_after_transition(
                            shell,
                            *tick,
                            refresh.predecessor,
                            capture_deadline,
                        );
                    }
                    retire_refresh_predecessor(refresh.predecessor, &successor, shell, publisher)?;
                    retain_refreshed_snapshot(successor, publisher)
                }
                PlatformPulseVisualCaptureResolution::RetryBefore { deadline } => {
                    restart_refresh_after_transition(shell, *tick, refresh.predecessor, deadline)
                }
            }
        }
    }
}

fn poll_rebase(
    capture: PlatformPulseVisualCapture,
    shell: &mut WorthUiNativeApplicationShell,
    publisher: &PlatformPulseObservationPublisher,
    tick: &mut u64,
    now: Instant,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    if now >= capture.deadline {
        shell.cancel_visual_snapshot(capture.pending);
        return Err(PlatformPulseVisualExecutionDenial::SnapshotDeadline);
    }
    match shell.poll_visual_snapshot(capture.pending, *tick) {
        UiVisualCapturePoll::Pending(pending) => Ok(PlatformPulseVisualIdentityState::Rebasing(
            PlatformPulseVisualCapture {
                pending,
                deadline: capture.deadline,
            },
        )),
        UiVisualCapturePoll::Completed(outcome) => {
            match resolve_capture(outcome, capture.deadline)? {
                PlatformPulseVisualCaptureResolution::Captured(receipt) => {
                    if !snapshot_matches_current_mounted_frame(&receipt, shell)? {
                        shell.dispose_visual_snapshot(receipt);
                        return restart_rebase_after_transition(shell, *tick, capture.deadline);
                    }
                    retain_refreshed_snapshot(receipt, publisher)
                }
                PlatformPulseVisualCaptureResolution::RetryBefore { deadline } => {
                    restart_rebase_after_transition(shell, *tick, deadline)
                }
            }
        }
    }
}

fn snapshot_matches_current_mounted_frame(
    snapshot: &UiVisualSnapshotReceipt<UiPixelsRequired>,
    shell: &WorthUiNativeApplicationShell,
) -> Result<bool, PlatformPulseVisualExecutionDenial> {
    if snapshot
        .relation()
        .map_err(PlatformPulseVisualExecutionDenial::SnapshotRelation)?
        != worth_ui::facade::inspection::UiVisualSnapshotRelation::Current
    {
        return Ok(false);
    }
    let current = match shell.inspect_mounted_frame(UiMountedInspectionRequest::current()) {
        UiMountedInspectionReceipt::Available(frame) => frame.frame(),
        UiMountedInspectionReceipt::Omitted(
            worth_ui::facade::app::UiMountedInspectionOmission::FrameTransitionInFlight,
        ) => return Ok(false),
        UiMountedInspectionReceipt::Omitted(omission) => {
            return Err(
                PlatformPulseVisualExecutionDenial::ComparisonMountedFrameUnavailable(omission),
            )
        }
    };
    Ok(snapshot.affinity().frame() == current.diagnostic_value())
}

fn restart_capture_after_transition(
    shell: &mut WorthUiNativeApplicationShell,
    tick: u64,
    deadline: Instant,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    match begin_capture_before(shell, tick, deadline) {
        Ok(capture) => Ok(PlatformPulseVisualIdentityState::Capturing(capture)),
        Err(PlatformPulseVisualExecutionDenial::CaptureMountedFrameUnavailable(
            worth_ui::facade::app::UiMountedInspectionOmission::FrameTransitionInFlight,
        )) => Ok(PlatformPulseVisualIdentityState::AwaitingCapture { deadline }),
        Err(denial) => Err(denial),
    }
}

fn restart_rebase_after_transition(
    shell: &mut WorthUiNativeApplicationShell,
    tick: u64,
    deadline: Instant,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    match begin_capture_before(shell, tick, deadline) {
        Ok(capture) => Ok(PlatformPulseVisualIdentityState::Rebasing(capture)),
        Err(PlatformPulseVisualExecutionDenial::CaptureMountedFrameUnavailable(
            worth_ui::facade::app::UiMountedInspectionOmission::FrameTransitionInFlight,
        )) => Ok(PlatformPulseVisualIdentityState::AwaitingRebase { deadline }),
        Err(denial) => Err(denial),
    }
}

fn restart_refresh_after_transition(
    shell: &mut WorthUiNativeApplicationShell,
    tick: u64,
    predecessor: PlatformPulseRetainedSnapshot,
    deadline: Instant,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    match begin_capture_before(shell, tick, deadline) {
        Ok(capture) => Ok(PlatformPulseVisualIdentityState::Refreshing(
            PlatformPulseVisualRefreshCapture {
                capture,
                predecessor,
            },
        )),
        Err(PlatformPulseVisualExecutionDenial::CaptureMountedFrameUnavailable(
            worth_ui::facade::app::UiMountedInspectionOmission::FrameTransitionInFlight,
        )) => Ok(PlatformPulseVisualIdentityState::AwaitingRefresh {
            predecessor,
            deadline,
        }),
        Err(denial) => Err(denial),
    }
}

#[cfg(test)]
#[path = "progression/tests.rs"]
mod tests;

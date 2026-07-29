use std::time::{Duration, Instant};

use worth_ui::facade::app::{
    UiMountedInspectionReceipt, UiMountedInspectionRequest, WorthUiNativeApplicationShell,
};
use worth_ui::facade::inspection::{
    UiPixelsRequired, UiVisualCaptureDeadline, UiVisualCapturePoll, UiVisualOverlayDenial,
    UiVisualSnapshotOutcome, UiVisualSnapshotReceipt,
};
use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseVisualPointObservation, PlatformPulseVisualPointTraceInput,
};

use crate::lifecycle_observation_publication::PlatformPulseObservationPublisher;
use crate::visual_identity_adjudication::adjudicate_points;

use super::{
    comparison, PlatformPulseRetainedSnapshot, PlatformPulseVisibleOverlay,
    PlatformPulseVisualCapture, PlatformPulseVisualExecutionDenial,
    PlatformPulseVisualIdentityState,
};

const CAPTURE_WALL_DEADLINE: Duration = Duration::from_secs(5);
const OVERLAY_VISIBLE_DWELL: Duration = Duration::from_secs(2);
const CAPTURE_TICK_ALLOWANCE: u64 = 1_000;

pub(super) fn advance_state(
    state: PlatformPulseVisualIdentityState,
    shell: &mut WorthUiNativeApplicationShell,
    publisher: &PlatformPulseObservationPublisher,
    tick: &mut u64,
    now: Instant,
) -> Result<PlatformPulseVisualIdentityState, PlatformPulseVisualExecutionDenial> {
    match state {
        PlatformPulseVisualIdentityState::Settling { begin_at } if now >= begin_at => {
            begin_capture(shell, *tick, now).map(PlatformPulseVisualIdentityState::Capturing)
        }
        PlatformPulseVisualIdentityState::Capturing(capture) => {
            poll_capture(capture, shell, publisher, tick, now)
        }
        PlatformPulseVisualIdentityState::Comparing(comparison) => {
            comparison::poll(comparison, shell, publisher, tick, now)
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
    let frame = match shell.inspect_mounted_frame(UiMountedInspectionRequest::current()) {
        UiMountedInspectionReceipt::Available(frame) => frame,
        UiMountedInspectionReceipt::Omitted(_) => {
            return Err(PlatformPulseVisualExecutionDenial::MountedFrameUnavailable)
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
    let deadline = now
        .checked_add(CAPTURE_WALL_DEADLINE)
        .ok_or(PlatformPulseVisualExecutionDenial::ClockOverflow)?;
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
            let receipt = captured_receipt(outcome)?;
            publish_overlay(receipt, shell, publisher, tick, now)
        }
    }
}

pub(super) fn captured_receipt(
    outcome: UiVisualSnapshotOutcome<UiPixelsRequired>,
) -> Result<UiVisualSnapshotReceipt<UiPixelsRequired>, PlatformPulseVisualExecutionDenial> {
    match outcome {
        UiVisualSnapshotOutcome::Captured(receipt) => Ok(receipt),
        UiVisualSnapshotOutcome::Superseded(_) => {
            Err(PlatformPulseVisualExecutionDenial::SnapshotSuperseded)
        }
        UiVisualSnapshotOutcome::Omitted(_) => {
            Err(PlatformPulseVisualExecutionDenial::SnapshotOmitted)
        }
        UiVisualSnapshotOutcome::Denied(_) => {
            Err(PlatformPulseVisualExecutionDenial::SnapshotDenied)
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
                target: points.selected_target,
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
    Ok(PlatformPulseVisualIdentityState::OverlayCleared(retained))
}

pub(super) fn retire_snapshot(
    retained: PlatformPulseRetainedSnapshot,
    shell: &mut WorthUiNativeApplicationShell,
    publisher: &PlatformPulseObservationPublisher,
) -> Result<(), PlatformPulseVisualExecutionDenial> {
    let denial = match retained.snapshot.overlay_target(&retained.target) {
        Err(UiVisualOverlayDenial::Superseded) => UiVisualOverlayDenial::Superseded,
        Err(denial) => {
            return Err(PlatformPulseVisualExecutionDenial::SnapshotDidNotBecomeSuperseded(denial))
        }
        Ok(_) => {
            return Err(
                PlatformPulseVisualExecutionDenial::SnapshotDidNotBecomeSuperseded(
                    UiVisualOverlayDenial::Presentation,
                ),
            )
        }
    };
    let snapshot = retained.snapshot.identity();
    let disposal = shell.dispose_visual_snapshot(retained.snapshot);
    publisher
        .visual_snapshot_retired(snapshot, denial, disposal)
        .map_err(PlatformPulseVisualExecutionDenial::Observation)
}

fn next_presentation_ticks(
    tick: &mut u64,
) -> Result<(u64, u64), PlatformPulseVisualExecutionDenial> {
    let current = tick
        .checked_add(1)
        .ok_or(PlatformPulseVisualExecutionDenial::TickExhausted)?;
    let deadline = current
        .checked_add(1)
        .ok_or(PlatformPulseVisualExecutionDenial::TickExhausted)?;
    *tick = current;
    Ok((deadline, current))
}

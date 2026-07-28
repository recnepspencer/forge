use std::time::{Duration, Instant};

use worth_ui::facade::app::{
    UiMountedInspectionReceipt, UiMountedInspectionRequest, WorthUiNativeApplicationShell,
};
use worth_ui::facade::inspection::{
    UiCurrentPresentedSurfaceTarget, UiPendingVisualCapture, UiPixelsRequired,
    UiPublishedVisualOverlay, UiVisualCaptureDeadline, UiVisualCapturePoll, UiVisualHitTestTarget,
    UiVisualOverlayDenial, UiVisualSnapshotOutcome, UiVisualSnapshotReceipt,
};

use crate::lifecycle_observation_publication::{
    PlatformPulseObservationPublicationDenial, PlatformPulseObservationPublisher,
};
use crate::visual_identity_adjudication::adjudicate_points;
use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseVisualPointObservation, PlatformPulseVisualPointTraceInput,
};

const INITIAL_NATIVE_SETTLEMENT: Duration = Duration::from_secs(1);
const CAPTURE_WALL_DEADLINE: Duration = Duration::from_secs(5);
const OVERLAY_VISIBLE_DWELL: Duration = Duration::from_secs(2);
const CAPTURE_TICK_ALLOWANCE: u64 = 1_000;

pub(crate) struct PlatformPulseVisualIdentityExecution {
    state: Option<PlatformPulseVisualIdentityState>,
}

enum PlatformPulseVisualIdentityState {
    AwaitingFirstFrame,
    Settling { begin_at: Instant },
    Capturing(PlatformPulseVisualCapture),
    OverlayVisible(PlatformPulseVisibleOverlay),
    OverlayCleared(PlatformPulseRetainedSnapshot),
    Retired,
}

struct PlatformPulseVisualCapture {
    pending: UiPendingVisualCapture<UiCurrentPresentedSurfaceTarget, UiPixelsRequired>,
    deadline: Instant,
}

struct PlatformPulseRetainedSnapshot {
    snapshot: UiVisualSnapshotReceipt<UiPixelsRequired>,
    target: UiVisualHitTestTarget,
}

struct PlatformPulseVisibleOverlay {
    retained: PlatformPulseRetainedSnapshot,
    published: UiPublishedVisualOverlay,
    clear_at: Instant,
}

#[derive(Debug)]
pub(crate) enum PlatformPulseVisualExecutionDenial {
    ReentrantTransition,
    InitialFrameAlreadyArmed,
    ClockOverflow,
    TickExhausted,
    MountedFrameUnavailable,
    MountedVisualTarget,
    SnapshotAdmission,
    SnapshotDeadline,
    SnapshotSuperseded,
    SnapshotOmitted,
    SnapshotDenied,
    SnapshotIndeterminate,
    PointCoordinate,
    PointOmitted,
    PointUnsupported,
    PointIdentityMismatch,
    AuthoredNameMismatch,
    OverlayTarget(UiVisualOverlayDenial),
    OverlayAdmission(UiVisualOverlayDenial),
    OverlayPublication(UiVisualOverlayDenial),
    OverlayClear(UiVisualOverlayDenial),
    ReplacementBeforeOverlayClear,
    SnapshotDidNotBecomeSuperseded(UiVisualOverlayDenial),
    Observation(PlatformPulseObservationPublicationDenial),
}

impl std::fmt::Display for PlatformPulseVisualExecutionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OverlayTarget(denial) => write!(formatter, "overlay target: {denial:?}"),
            Self::OverlayAdmission(denial) => write!(formatter, "overlay admission: {denial:?}"),
            Self::OverlayPublication(denial) => {
                write!(formatter, "overlay publication: {denial:?}")
            }
            Self::OverlayClear(denial) => write!(formatter, "overlay clear: {denial:?}"),
            Self::SnapshotDidNotBecomeSuperseded(denial) => {
                write!(formatter, "snapshot retirement: {denial:?}")
            }
            Self::Observation(denial) => write!(formatter, "observation publication: {denial:?}"),
            denial => write!(formatter, "{denial:?}"),
        }
    }
}

impl PlatformPulseVisualIdentityExecution {
    pub(crate) fn new() -> Self {
        Self {
            state: Some(PlatformPulseVisualIdentityState::AwaitingFirstFrame),
        }
    }

    pub(crate) fn arm_after_first_frame(
        &mut self,
        now: Instant,
    ) -> Result<(), PlatformPulseVisualExecutionDenial> {
        let state = self
            .state
            .take()
            .ok_or(PlatformPulseVisualExecutionDenial::ReentrantTransition)?;
        if !matches!(state, PlatformPulseVisualIdentityState::AwaitingFirstFrame) {
            self.state = Some(state);
            return Err(PlatformPulseVisualExecutionDenial::InitialFrameAlreadyArmed);
        }
        let begin_at = now
            .checked_add(INITIAL_NATIVE_SETTLEMENT)
            .ok_or(PlatformPulseVisualExecutionDenial::ClockOverflow)?;
        self.state = Some(PlatformPulseVisualIdentityState::Settling { begin_at });
        Ok(())
    }

    pub(crate) fn advance(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        publisher: &PlatformPulseObservationPublisher,
        tick: &mut u64,
        now: Instant,
    ) -> Result<(), PlatformPulseVisualExecutionDenial> {
        let state = self
            .state
            .take()
            .ok_or(PlatformPulseVisualExecutionDenial::ReentrantTransition)?;
        let next = advance_state(state, shell, publisher, tick, now)?;
        self.state = Some(next);
        Ok(())
    }

    pub(crate) fn retire_after_replacement(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        publisher: &PlatformPulseObservationPublisher,
    ) -> Result<(), PlatformPulseVisualExecutionDenial> {
        let state = self
            .state
            .take()
            .ok_or(PlatformPulseVisualExecutionDenial::ReentrantTransition)?;
        match state {
            PlatformPulseVisualIdentityState::OverlayCleared(retained) => {
                retire_snapshot(retained, shell, publisher)?;
                self.state = Some(PlatformPulseVisualIdentityState::Retired);
                Ok(())
            }
            PlatformPulseVisualIdentityState::Retired => {
                self.state = Some(PlatformPulseVisualIdentityState::Retired);
                Ok(())
            }
            state => {
                self.state = Some(state);
                Err(PlatformPulseVisualExecutionDenial::ReplacementBeforeOverlayClear)
            }
        }
    }
}

fn advance_state(
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
        PlatformPulseVisualIdentityState::OverlayVisible(overlay) if now >= overlay.clear_at => {
            clear_overlay(overlay, shell, publisher, tick)
        }
        state => Ok(state),
    }
}

fn begin_capture(
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
        .map_err(|_| PlatformPulseVisualExecutionDenial::SnapshotAdmission)?;
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

fn captured_receipt(
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
    Ok(PlatformPulseVisualIdentityState::OverlayCleared(
        overlay.retained,
    ))
}

fn retire_snapshot(
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

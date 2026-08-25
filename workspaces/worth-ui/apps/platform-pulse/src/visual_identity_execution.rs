use std::time::{Duration, Instant};

use worth_ui::facade::app::WorthUiNativeApplicationShell;
use worth_ui::facade::inspection::{
    UiCurrentPresentedSurfaceTarget, UiPendingVisualCapture, UiPixelsRequired,
    UiPublishedVisualOverlay, UiVisualOverlayDenial, UiVisualSnapshotDenial,
    UiVisualSnapshotReceipt,
};

use crate::lifecycle_observation_publication::{
    PlatformPulseObservationPublicationDenial, PlatformPulseObservationPublisher,
};

mod comparison;
mod progression;
mod readiness;

const INITIAL_NATIVE_SETTLEMENT: Duration = Duration::from_secs(1);
const NATIVE_CAPTURE_POLL_INTERVAL: Duration = Duration::from_millis(1);

pub(crate) struct PlatformPulseVisualIdentityExecution {
    state: Option<PlatformPulseVisualIdentityState>,
    readiness: Option<readiness::PlatformPulseVisualReadiness>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PlatformPulseContentMutationReadiness {
    Ready,
    DeferredForVisualComparison,
    TransitionInProgress,
}

enum PlatformPulseVisualIdentityState {
    AwaitingFirstFrame,
    Settling { begin_at: Instant },
    Capturing(PlatformPulseVisualCapture),
    OverlayVisible(PlatformPulseVisibleOverlay),
    ComparisonReady(PlatformPulseRetainedSnapshot),
    Rebasing(PlatformPulseVisualCapture),
    Refreshing(PlatformPulseVisualRefreshCapture),
    Comparing(comparison::PlatformPulseVisualComparisonCapture),
    Transitioning,
    Retired,
}

struct PlatformPulseVisualCapture {
    pending: UiPendingVisualCapture<UiCurrentPresentedSurfaceTarget, UiPixelsRequired>,
    deadline: Instant,
}

struct PlatformPulseVisualRefreshCapture {
    capture: PlatformPulseVisualCapture,
    predecessor: PlatformPulseRetainedSnapshot,
}

struct PlatformPulseRetainedSnapshot {
    snapshot: UiVisualSnapshotReceipt<UiPixelsRequired>,
    overlay_clear: Option<worth_ui::facade::inspection::UiClearedVisualOverlayReceipt>,
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
    SnapshotAdmission(worth_ui::facade::inspection::UiVisualSnapshotDenial),
    SnapshotDeadline,
    SnapshotOmitted,
    SnapshotDenied(UiVisualSnapshotDenial),
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
    SnapshotRelation(worth_ui::facade::inspection::UiVisualSnapshotRelationDenial),
    SnapshotStillCurrent,
    ShutdownNotQuiescent,
    ComparisonOmitted(worth_ui::facade::inspection::UiVisualSnapshotComparisonOmission),
    ComparisonExpired(worth_ui::facade::inspection::UiVisualSnapshotComparisonExpiry),
    ComparisonIncompatible(worth_ui::facade::inspection::UiVisualSnapshotComparisonIncompatibility),
    ComparisonDenied(worth_ui::facade::inspection::UiVisualSnapshotComparisonDenial),
    Observation(PlatformPulseObservationPublicationDenial),
}

impl std::fmt::Display for PlatformPulseVisualExecutionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SnapshotAdmission(denial) => {
                write!(formatter, "snapshot admission: {denial:?}")
            }
            Self::SnapshotDenied(denial) => write!(formatter, "snapshot denied: {denial:?}"),
            Self::ComparisonOmitted(omission) => {
                write!(formatter, "comparison omitted: {omission:?}")
            }
            Self::ComparisonExpired(expiry) => {
                write!(formatter, "comparison expired: {expiry:?}")
            }
            Self::ComparisonIncompatible(incompatibility) => {
                write!(formatter, "comparison incompatible: {incompatibility:?}")
            }
            Self::ComparisonDenied(denial) => {
                write!(formatter, "comparison denied: {denial:?}")
            }
            Self::OverlayTarget(denial) => write!(formatter, "overlay target: {denial:?}"),
            Self::OverlayAdmission(denial) => write!(formatter, "overlay admission: {denial:?}"),
            Self::OverlayPublication(denial) => {
                write!(formatter, "overlay publication: {denial:?}")
            }
            Self::OverlayClear(denial) => write!(formatter, "overlay clear: {denial:?}"),
            Self::SnapshotRelation(denial) => write!(formatter, "snapshot relation: {denial:?}"),
            Self::Observation(denial) => write!(formatter, "observation publication: {denial:?}"),
            denial => write!(formatter, "{denial:?}"),
        }
    }
}

impl PlatformPulseVisualIdentityExecution {
    pub(crate) fn new() -> Self {
        Self {
            state: Some(PlatformPulseVisualIdentityState::AwaitingFirstFrame),
            readiness: None,
        }
    }

    pub(crate) fn install_readiness(
        &mut self,
        signal: worth_ui_platform_pulse::PlatformPulseApplicationReadinessSignal,
    ) {
        self.readiness = Some(readiness::PlatformPulseVisualReadiness::install(signal));
    }

    pub(crate) fn content_mutation_readiness(&self) -> PlatformPulseContentMutationReadiness {
        match self.state.as_ref() {
            Some(PlatformPulseVisualIdentityState::Comparing(_)) => {
                PlatformPulseContentMutationReadiness::DeferredForVisualComparison
            }
            Some(PlatformPulseVisualIdentityState::Transitioning) | None => {
                PlatformPulseContentMutationReadiness::TransitionInProgress
            }
            Some(_) => PlatformPulseContentMutationReadiness::Ready,
        }
    }

    pub(crate) fn shutdown_quiescent(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
    ) -> Result<(), PlatformPulseVisualExecutionDenial> {
        let state = self
            .state
            .take()
            .ok_or(PlatformPulseVisualExecutionDenial::ReentrantTransition)?;
        match state {
            PlatformPulseVisualIdentityState::ComparisonReady(retained) => {
                shell.dispose_visual_snapshot(retained.snapshot);
                self.state = Some(PlatformPulseVisualIdentityState::Retired);
                Ok(())
            }
            PlatformPulseVisualIdentityState::AwaitingFirstFrame
            | PlatformPulseVisualIdentityState::Settling { .. }
            | PlatformPulseVisualIdentityState::Retired => {
                self.state = Some(PlatformPulseVisualIdentityState::Retired);
                Ok(())
            }
            state => {
                self.state = Some(state);
                Err(PlatformPulseVisualExecutionDenial::ShutdownNotQuiescent)
            }
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
        self.schedule_wake(begin_at);
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
            .replace(PlatformPulseVisualIdentityState::Transitioning)
            .ok_or(PlatformPulseVisualExecutionDenial::ReentrantTransition)?;
        match progression::advance_state(state, shell, publisher, tick, now) {
            Ok(next) => {
                self.state = Some(next);
                self.schedule_current_wake();
                Ok(())
            }
            Err(denial) => Err(denial),
        }
    }

    pub(crate) fn compare_after_rebind(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        rebind: worth_ui::facade::rebind::UiRebindReceipt,
        tick: u64,
        now: Instant,
    ) -> Result<(), PlatformPulseVisualExecutionDenial> {
        match self.state.as_ref() {
            Some(PlatformPulseVisualIdentityState::ComparisonReady(_)) => {
                let capture = progression::begin_capture(shell, tick, now)?;
                let state = self
                    .state
                    .replace(PlatformPulseVisualIdentityState::Transitioning)
                    .ok_or(PlatformPulseVisualExecutionDenial::ReentrantTransition)?;
                let PlatformPulseVisualIdentityState::ComparisonReady(retained) = state else {
                    return Err(PlatformPulseVisualExecutionDenial::ReplacementBeforeOverlayClear);
                };
                let comparison = comparison::begin(retained, rebind, capture);
                self.state = Some(PlatformPulseVisualIdentityState::Comparing(comparison));
                self.schedule_current_wake();
                Ok(())
            }
            Some(PlatformPulseVisualIdentityState::Retired) => {
                drop(rebind);
                Ok(())
            }
            Some(_) => Err(PlatformPulseVisualExecutionDenial::ReplacementBeforeOverlayClear),
            None => Err(PlatformPulseVisualExecutionDenial::ReentrantTransition),
        }
    }

    pub(crate) fn refresh_after_content_rebind(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        tick: u64,
        now: Instant,
    ) -> Result<(), PlatformPulseVisualExecutionDenial> {
        if matches!(self.state, Some(PlatformPulseVisualIdentityState::Retired)) {
            let capture = progression::begin_capture(shell, tick, now)?;
            self.state = Some(PlatformPulseVisualIdentityState::Rebasing(capture));
            self.schedule_current_wake();
            return Ok(());
        }
        if !matches!(
            self.state,
            Some(PlatformPulseVisualIdentityState::ComparisonReady(_))
        ) {
            return Ok(());
        }
        let capture = progression::begin_capture(shell, tick, now)?;
        let state = self
            .state
            .replace(PlatformPulseVisualIdentityState::Transitioning)
            .ok_or(PlatformPulseVisualExecutionDenial::ReentrantTransition)?;
        let PlatformPulseVisualIdentityState::ComparisonReady(predecessor) = state else {
            return Err(PlatformPulseVisualExecutionDenial::ReentrantTransition);
        };
        self.state = Some(PlatformPulseVisualIdentityState::Refreshing(
            PlatformPulseVisualRefreshCapture {
                capture,
                predecessor,
            },
        ));
        self.schedule_current_wake();
        Ok(())
    }

    fn schedule_current_wake(&self) {
        let deadline = match self.state.as_ref() {
            Some(PlatformPulseVisualIdentityState::Settling { begin_at }) => Some(*begin_at),
            Some(PlatformPulseVisualIdentityState::Capturing(capture))
            | Some(PlatformPulseVisualIdentityState::Rebasing(capture)) => {
                Some(next_capture_poll(capture.deadline))
            }
            Some(PlatformPulseVisualIdentityState::Refreshing(refresh)) => {
                Some(next_capture_poll(refresh.capture.deadline))
            }
            Some(PlatformPulseVisualIdentityState::Comparing(comparison)) => {
                Some(next_capture_poll(comparison.deadline()))
            }
            Some(PlatformPulseVisualIdentityState::OverlayVisible(overlay)) => {
                Some(overlay.clear_at)
            }
            _ => None,
        };
        if let Some(deadline) = deadline {
            self.schedule_wake(deadline);
        }
    }

    fn schedule_wake(&self, deadline: Instant) {
        if let Some(readiness) = self.readiness.as_ref() {
            readiness.schedule(deadline);
        }
    }
}

fn next_capture_poll(deadline: Instant) -> Instant {
    Instant::now()
        .checked_add(NATIVE_CAPTURE_POLL_INTERVAL)
        .map_or(deadline, |poll| poll.min(deadline))
}

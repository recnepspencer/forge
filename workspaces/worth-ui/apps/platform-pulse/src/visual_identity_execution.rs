use std::time::{Duration, Instant};

use worth_ui::facade::app::WorthUiNativeApplicationShell;
use worth_ui::facade::inspection::{
    UiCurrentPresentedSurfaceTarget, UiPendingVisualCapture, UiPixelsRequired,
    UiPublishedVisualOverlay, UiVisualHitTestTarget, UiVisualOverlayDenial,
    UiVisualSnapshotReceipt,
};

use crate::lifecycle_observation_publication::{
    PlatformPulseObservationPublicationDenial, PlatformPulseObservationPublisher,
};

mod comparison;
mod progression;

const INITIAL_NATIVE_SETTLEMENT: Duration = Duration::from_secs(1);

pub(crate) struct PlatformPulseVisualIdentityExecution {
    state: Option<PlatformPulseVisualIdentityState>,
}

enum PlatformPulseVisualIdentityState {
    AwaitingFirstFrame,
    Settling { begin_at: Instant },
    Capturing(PlatformPulseVisualCapture),
    OverlayVisible(PlatformPulseVisibleOverlay),
    OverlayCleared(PlatformPulseRetainedSnapshot),
    Comparing(comparison::PlatformPulseVisualComparisonCapture),
    Transitioning,
    Retired,
}

struct PlatformPulseVisualCapture {
    pending: UiPendingVisualCapture<UiCurrentPresentedSurfaceTarget, UiPixelsRequired>,
    deadline: Instant,
}

struct PlatformPulseRetainedSnapshot {
    snapshot: UiVisualSnapshotReceipt<UiPixelsRequired>,
    target: UiVisualHitTestTarget,
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
            .replace(PlatformPulseVisualIdentityState::Transitioning)
            .ok_or(PlatformPulseVisualExecutionDenial::ReentrantTransition)?;
        match progression::advance_state(state, shell, publisher, tick, now) {
            Ok(next) => {
                self.state = Some(next);
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
            Some(PlatformPulseVisualIdentityState::OverlayCleared(_)) => {
                let capture = progression::begin_capture(shell, tick, now)?;
                let state = self
                    .state
                    .replace(PlatformPulseVisualIdentityState::Transitioning)
                    .ok_or(PlatformPulseVisualExecutionDenial::ReentrantTransition)?;
                let PlatformPulseVisualIdentityState::OverlayCleared(retained) = state else {
                    return Err(PlatformPulseVisualExecutionDenial::ReplacementBeforeOverlayClear);
                };
                let comparison = comparison::begin(retained, rebind, capture);
                self.state = Some(PlatformPulseVisualIdentityState::Comparing(comparison));
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
}

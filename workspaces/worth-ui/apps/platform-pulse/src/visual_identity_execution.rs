use std::time::{Duration, Instant};

use worth_ui::facade::app::WorthUiNativeApplicationShell;
use worth_ui::facade::inspection::{
    UiCurrentPresentedSurfaceTarget, UiPendingVisualCapture, UiPixelsRequired,
    UiPublishedVisualOverlay, UiVisualSnapshotReceipt,
};

use crate::lifecycle_observation_publication::PlatformPulseObservationPublisher;

mod comparison;
mod denial;
mod progression;
mod readiness;
mod state_progression;

pub(crate) use denial::PlatformPulseVisualExecutionDenial;
use state_progression::{
    advance_awaiting_capture, advance_awaiting_comparison, advance_awaiting_rebase,
    advance_awaiting_refresh, mounted_frame_ready, next_capture_poll, replacement_frame_deadline,
};

const INITIAL_NATIVE_SETTLEMENT: Duration = Duration::from_secs(1);
const NATIVE_CAPTURE_POLL_INTERVAL: Duration = Duration::from_millis(1);
const REPLACEMENT_FRAME_DEADLINE: Duration = Duration::from_secs(5);

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
    Settling {
        begin_at: Instant,
        deadline: Instant,
    },
    Capturing(PlatformPulseVisualCapture),
    AwaitingCapture {
        deadline: Instant,
    },
    OverlayVisible(PlatformPulseVisibleOverlay),
    ComparisonReady(PlatformPulseRetainedSnapshot),
    AwaitingRebase {
        deadline: Instant,
    },
    AwaitingRefresh {
        predecessor: PlatformPulseRetainedSnapshot,
        deadline: Instant,
    },
    Rebasing(PlatformPulseVisualCapture),
    Refreshing(PlatformPulseVisualRefreshCapture),
    AwaitingComparison {
        predecessor: PlatformPulseRetainedSnapshot,
        rebind: worth_ui::facade::rebind::UiRebindReceipt,
        deadline: Instant,
    },
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
            Some(PlatformPulseVisualIdentityState::AwaitingComparison { .. })
            | Some(PlatformPulseVisualIdentityState::Comparing(_)) => {
                PlatformPulseContentMutationReadiness::DeferredForVisualComparison
            }
            Some(PlatformPulseVisualIdentityState::Settling { .. })
            | Some(PlatformPulseVisualIdentityState::Capturing(_))
            | Some(PlatformPulseVisualIdentityState::AwaitingCapture { .. })
            | Some(PlatformPulseVisualIdentityState::OverlayVisible(_))
            | Some(PlatformPulseVisualIdentityState::AwaitingRebase { .. })
            | Some(PlatformPulseVisualIdentityState::Rebasing(_))
            | Some(PlatformPulseVisualIdentityState::AwaitingRefresh { .. })
            | Some(PlatformPulseVisualIdentityState::Refreshing(_))
            | Some(PlatformPulseVisualIdentityState::Transitioning)
            | None => PlatformPulseContentMutationReadiness::TransitionInProgress,
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
        let deadline = begin_at
            .checked_add(REPLACEMENT_FRAME_DEADLINE)
            .ok_or(PlatformPulseVisualExecutionDenial::ClockOverflow)?;
        self.state = Some(PlatformPulseVisualIdentityState::Settling { begin_at, deadline });
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
        let next = match state {
            PlatformPulseVisualIdentityState::AwaitingCapture { deadline } => {
                advance_awaiting_capture(shell, *tick, now, deadline)
            }
            PlatformPulseVisualIdentityState::AwaitingRebase { deadline } => {
                advance_awaiting_rebase(shell, *tick, now, deadline)
            }
            PlatformPulseVisualIdentityState::AwaitingRefresh {
                predecessor,
                deadline,
            } => advance_awaiting_refresh(shell, *tick, now, predecessor, deadline),
            PlatformPulseVisualIdentityState::AwaitingComparison {
                predecessor,
                rebind,
                deadline,
            } => advance_awaiting_comparison(shell, *tick, now, predecessor, rebind, deadline),
            state => progression::advance_state(state, shell, publisher, tick, now),
        };
        match next {
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
                let mounted_frame_ready = mounted_frame_ready(shell)?;
                let state = self
                    .state
                    .replace(PlatformPulseVisualIdentityState::Transitioning)
                    .ok_or(PlatformPulseVisualExecutionDenial::ReentrantTransition)?;
                let PlatformPulseVisualIdentityState::ComparisonReady(retained) = state else {
                    return Err(PlatformPulseVisualExecutionDenial::ReplacementBeforeOverlayClear);
                };
                self.state = Some(if mounted_frame_ready {
                    let capture = progression::begin_capture(shell, tick, now)?;
                    PlatformPulseVisualIdentityState::Comparing(comparison::begin(
                        retained, rebind, capture,
                    ))
                } else {
                    PlatformPulseVisualIdentityState::AwaitingComparison {
                        predecessor: retained,
                        rebind,
                        deadline: replacement_frame_deadline(now)?,
                    }
                });
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

    pub(crate) fn refresh_after_presentation_replacement(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        tick: u64,
        now: Instant,
    ) -> Result<(), PlatformPulseVisualExecutionDenial> {
        if matches!(
            self.state,
            Some(PlatformPulseVisualIdentityState::Retired)
                | Some(PlatformPulseVisualIdentityState::Rebasing(_))
                | Some(PlatformPulseVisualIdentityState::AwaitingRebase { .. })
        ) {
            let mounted_frame_ready = mounted_frame_ready(shell)?;
            let state = self
                .state
                .replace(PlatformPulseVisualIdentityState::Transitioning)
                .ok_or(PlatformPulseVisualExecutionDenial::ReentrantTransition)?;
            if let PlatformPulseVisualIdentityState::Rebasing(capture) = state {
                shell.cancel_visual_snapshot(capture.pending);
            }
            self.state = Some(if mounted_frame_ready {
                PlatformPulseVisualIdentityState::Rebasing(progression::begin_capture(
                    shell, tick, now,
                )?)
            } else {
                PlatformPulseVisualIdentityState::AwaitingRebase {
                    deadline: replacement_frame_deadline(now)?,
                }
            });
            self.schedule_current_wake();
            return Ok(());
        }
        if !matches!(
            self.state,
            Some(PlatformPulseVisualIdentityState::ComparisonReady(_))
        ) {
            return Ok(());
        }
        let mounted_frame_ready = mounted_frame_ready(shell)?;
        let state = self
            .state
            .replace(PlatformPulseVisualIdentityState::Transitioning)
            .ok_or(PlatformPulseVisualExecutionDenial::ReentrantTransition)?;
        let PlatformPulseVisualIdentityState::ComparisonReady(predecessor) = state else {
            return Err(PlatformPulseVisualExecutionDenial::ReentrantTransition);
        };
        self.state = Some(if mounted_frame_ready {
            PlatformPulseVisualIdentityState::Refreshing(PlatformPulseVisualRefreshCapture {
                capture: progression::begin_capture(shell, tick, now)?,
                predecessor,
            })
        } else {
            PlatformPulseVisualIdentityState::AwaitingRefresh {
                predecessor,
                deadline: replacement_frame_deadline(now)?,
            }
        });
        self.schedule_current_wake();
        Ok(())
    }

    fn schedule_current_wake(&self) {
        let deadline = match self.state.as_ref() {
            Some(PlatformPulseVisualIdentityState::Settling { begin_at, .. }) => Some(*begin_at),
            Some(PlatformPulseVisualIdentityState::Capturing(capture))
            | Some(PlatformPulseVisualIdentityState::Rebasing(capture)) => {
                Some(next_capture_poll(capture.deadline))
            }
            Some(PlatformPulseVisualIdentityState::AwaitingCapture { deadline }) => {
                Some(next_capture_poll(*deadline))
            }
            Some(PlatformPulseVisualIdentityState::Refreshing(refresh)) => {
                Some(next_capture_poll(refresh.capture.deadline))
            }
            Some(PlatformPulseVisualIdentityState::Comparing(comparison)) => {
                Some(next_capture_poll(comparison.deadline()))
            }
            Some(PlatformPulseVisualIdentityState::AwaitingComparison { deadline, .. }) => {
                Some(next_capture_poll(*deadline))
            }
            Some(PlatformPulseVisualIdentityState::AwaitingRebase { deadline })
            | Some(PlatformPulseVisualIdentityState::AwaitingRefresh { deadline, .. }) => {
                Some(next_capture_poll(*deadline))
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

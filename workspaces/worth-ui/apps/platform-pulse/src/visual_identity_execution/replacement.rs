use std::time::Instant;

use worth_ui::facade::app::WorthUiNativeApplicationShell;

use super::{
    replacement_frame_deadline, PlatformPulseVisualExecutionDenial,
    PlatformPulseVisualIdentityExecution, PlatformPulseVisualIdentityState,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) enum PlatformPulseReplacementPosture {
    #[default]
    Current,
    Pending,
}

impl PlatformPulseReplacementPosture {
    pub(super) fn note(&mut self) {
        *self = Self::Pending;
    }

    pub(super) const fn is_pending(self) -> bool {
        matches!(self, Self::Pending)
    }
}

impl PlatformPulseVisualIdentityExecution {
    pub(crate) fn refresh_after_presentation_replacement(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        _tick: u64,
        now: Instant,
    ) -> Result<(), PlatformPulseVisualExecutionDenial> {
        self.refresh_after_replacement(
            shell,
            now,
            PlatformPulseReplacementVisualPosture::DeferForActivePortal,
        )
    }

    pub(crate) fn refresh_after_viewport_replacement(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        _tick: u64,
        now: Instant,
    ) -> Result<(), PlatformPulseVisualExecutionDenial> {
        self.refresh_after_replacement(
            shell,
            now,
            PlatformPulseReplacementVisualPosture::RequireCurrentViewport,
        )
    }

    fn refresh_after_replacement(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        now: Instant,
        posture: PlatformPulseReplacementVisualPosture,
    ) -> Result<(), PlatformPulseVisualExecutionDenial> {
        let readiness = replacement_readiness(posture.portal_active(shell), now)?;
        let state = self
            .state
            .replace(PlatformPulseVisualIdentityState::Transitioning)
            .ok_or(PlatformPulseVisualExecutionDenial::ReentrantTransition)?;
        let next = match state {
            PlatformPulseVisualIdentityState::Settling { .. }
            | PlatformPulseVisualIdentityState::DeferredCapture => Ok(readiness.capture_state()),
            PlatformPulseVisualIdentityState::AwaitingCapture { budget } => {
                Ok(readiness.capture_state_with_budget(budget))
            }
            PlatformPulseVisualIdentityState::Retired
            | PlatformPulseVisualIdentityState::DeferredRebase => Ok(readiness.rebase_state()),
            PlatformPulseVisualIdentityState::AwaitingRebase { budget } => {
                Ok(readiness.rebase_state_with_budget(budget))
            }
            PlatformPulseVisualIdentityState::ComparisonReady(predecessor)
            | PlatformPulseVisualIdentityState::DeferredRefresh(predecessor) => {
                Ok(if readiness.deferred() {
                    PlatformPulseVisualIdentityState::DeferredRefresh(predecessor)
                } else {
                    PlatformPulseVisualIdentityState::AwaitingRefresh {
                        predecessor,
                        budget: super::capture_restart::PlatformPulseAwaitingCaptureBudget::fresh(
                            readiness.deadline(),
                        ),
                    }
                })
            }
            PlatformPulseVisualIdentityState::AwaitingRefresh {
                predecessor,
                budget,
            } => Ok(if readiness.deferred() {
                PlatformPulseVisualIdentityState::DeferredRefresh(predecessor)
            } else {
                PlatformPulseVisualIdentityState::AwaitingRefresh {
                    predecessor,
                    budget,
                }
            }),
            PlatformPulseVisualIdentityState::AwaitingComparison {
                predecessor,
                rebind,
                budget,
            } => {
                drop(rebind);
                Ok(if readiness.deferred() {
                    PlatformPulseVisualIdentityState::DeferredRefresh(predecessor)
                } else {
                    PlatformPulseVisualIdentityState::AwaitingRefresh {
                        predecessor,
                        budget,
                    }
                })
            }
            PlatformPulseVisualIdentityState::DeferredComparison {
                predecessor,
                rebind,
            } => {
                drop(rebind);
                Ok(if readiness.deferred() {
                    PlatformPulseVisualIdentityState::DeferredRefresh(predecessor)
                } else {
                    PlatformPulseVisualIdentityState::AwaitingRefresh {
                        predecessor,
                        budget: super::capture_restart::PlatformPulseAwaitingCaptureBudget::fresh(
                            readiness.deadline(),
                        ),
                    }
                })
            }
            PlatformPulseVisualIdentityState::OverlayVisible(mut overlay) => {
                overlay.replacement.note();
                Ok(PlatformPulseVisualIdentityState::OverlayVisible(overlay))
            }
            PlatformPulseVisualIdentityState::Comparing(mut comparison) => {
                comparison.note_presentation_replacement();
                Ok(PlatformPulseVisualIdentityState::Comparing(comparison))
            }
            PlatformPulseVisualIdentityState::Capturing(mut capture) => {
                capture.replacement.note();
                Ok(PlatformPulseVisualIdentityState::Capturing(capture))
            }
            PlatformPulseVisualIdentityState::Rebasing(mut capture) => {
                capture.replacement.note();
                Ok(PlatformPulseVisualIdentityState::Rebasing(capture))
            }
            PlatformPulseVisualIdentityState::Refreshing(mut refresh) => {
                refresh.capture.replacement.note();
                Ok(PlatformPulseVisualIdentityState::Refreshing(refresh))
            }
            state => Ok(state),
        };
        self.state = Some(match next {
            Ok(next) => next,
            Err(denial) => {
                self.state = Some(PlatformPulseVisualIdentityState::Failed);
                return Err(denial);
            }
        });
        self.schedule_current_wake();
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PlatformPulseReplacementVisualPosture {
    DeferForActivePortal,
    RequireCurrentViewport,
}

impl PlatformPulseReplacementVisualPosture {
    fn portal_active(self, shell: &WorthUiNativeApplicationShell) -> bool {
        matches!(self, Self::DeferForActivePortal) && portal_active(shell)
    }
}

pub(super) fn portal_active(shell: &WorthUiNativeApplicationShell) -> bool {
    shell.runtime_service_resource_census().active_portals() > 0
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PlatformPulseReplacementReadiness {
    DeferredForActivePortal,
    AwaitingFrame { deadline: Instant },
}

pub(super) fn replacement_readiness(
    portal_active: bool,
    now: Instant,
) -> Result<PlatformPulseReplacementReadiness, PlatformPulseVisualExecutionDenial> {
    if portal_active {
        Ok(PlatformPulseReplacementReadiness::DeferredForActivePortal)
    } else {
        Ok(PlatformPulseReplacementReadiness::AwaitingFrame {
            deadline: replacement_frame_deadline(now)?,
        })
    }
}

impl PlatformPulseReplacementReadiness {
    pub(super) const fn deferred(self) -> bool {
        matches!(self, Self::DeferredForActivePortal)
    }

    pub(super) fn deadline(self) -> Instant {
        match self {
            Self::AwaitingFrame { deadline } => deadline,
            Self::DeferredForActivePortal => {
                unreachable!("active Portal deferral does not own a readiness deadline")
            }
        }
    }

    fn capture_state(self) -> PlatformPulseVisualIdentityState {
        if self.deferred() {
            PlatformPulseVisualIdentityState::DeferredCapture
        } else {
            PlatformPulseVisualIdentityState::AwaitingCapture {
                budget: super::capture_restart::PlatformPulseAwaitingCaptureBudget::fresh(
                    self.deadline(),
                ),
            }
        }
    }

    fn capture_state_with_budget(
        self,
        budget: super::capture_restart::PlatformPulseAwaitingCaptureBudget,
    ) -> PlatformPulseVisualIdentityState {
        if self.deferred() {
            PlatformPulseVisualIdentityState::DeferredCapture
        } else {
            PlatformPulseVisualIdentityState::AwaitingCapture { budget }
        }
    }

    fn rebase_state(self) -> PlatformPulseVisualIdentityState {
        if self.deferred() {
            PlatformPulseVisualIdentityState::DeferredRebase
        } else {
            PlatformPulseVisualIdentityState::AwaitingRebase {
                budget: super::capture_restart::PlatformPulseAwaitingCaptureBudget::fresh(
                    self.deadline(),
                ),
            }
        }
    }

    fn rebase_state_with_budget(
        self,
        budget: super::capture_restart::PlatformPulseAwaitingCaptureBudget,
    ) -> PlatformPulseVisualIdentityState {
        if self.deferred() {
            PlatformPulseVisualIdentityState::DeferredRebase
        } else {
            PlatformPulseVisualIdentityState::AwaitingRebase { budget }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::{
        replacement_readiness, PlatformPulseReplacementPosture, PlatformPulseReplacementReadiness,
    };

    #[test]
    fn in_flight_replacement_is_retained_for_a_fresh_successor_capture() {
        let mut posture = PlatformPulseReplacementPosture::default();
        posture.note();
        posture.note();
        assert!(posture.is_pending());
    }

    #[test]
    fn overlay_and_comparison_replacements_share_the_pending_follow_up_posture() {
        let mut overlay = PlatformPulseReplacementPosture::default();
        let mut comparison = PlatformPulseReplacementPosture::default();
        overlay.note();
        comparison.note();
        assert_eq!(overlay, comparison);
        assert!(overlay.is_pending());
    }

    #[test]
    fn active_portal_defers_without_starting_a_readiness_clock() {
        let now = Instant::now();
        assert_eq!(
            replacement_readiness(true, now).expect("active Portal deferral"),
            PlatformPulseReplacementReadiness::DeferredForActivePortal,
        );
        assert_eq!(
            replacement_readiness(false, now).expect("dismissed Portal readiness"),
            PlatformPulseReplacementReadiness::AwaitingFrame {
                deadline: now + Duration::from_secs(5),
            },
        );
        assert!(matches!(
            replacement_readiness(true, now)
                .expect("active Portal state")
                .capture_state(),
            super::PlatformPulseVisualIdentityState::DeferredCapture,
        ));
        assert!(matches!(
            replacement_readiness(true, now)
                .expect("active Portal rebase state")
                .rebase_state(),
            super::PlatformPulseVisualIdentityState::DeferredRebase,
        ));
    }

    #[test]
    fn dismissed_portal_does_not_extend_an_awaiting_host_budget() {
        let now = Instant::now();
        let preserved = now + Duration::from_millis(275);
        let budget =
            super::super::capture_restart::PlatformPulseAwaitingCaptureBudget::preserved(preserved);
        let readiness = replacement_readiness(false, now).expect("dismissed Portal readiness");
        let super::PlatformPulseVisualIdentityState::AwaitingCapture { budget: resumed } =
            readiness.capture_state_with_budget(budget)
        else {
            panic!("dismissed Portal must retain awaiting capture posture");
        };
        assert_eq!(resumed.readiness_deadline(), preserved);
    }
}

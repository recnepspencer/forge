use std::time::{Duration, Instant};

use worth_ui_host_contract::{
    UiHostSurfacePresentationDenial, UiHostSurfacePresentationOutcome,
    UiMountedPresentationAttemptIdentity,
};

const TIMEOUT_RETRY_LIMIT: u8 = 3;
const TIMEOUT_RETRY_DELAY: Duration = Duration::from_millis(16);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativePresentationRetryWake {
    Timeout(Instant),
    Visibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativePresentationRetryFinalization {
    Unchanged,
    Wake(UiNativePresentationRetryWake),
    DeadlineExpired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum UiNativePresentationRetryRequirement {
    Timeout,
    TextAtlas,
    Visibility,
    Reconstruction,
    Terminal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum UiNativePresentationEffectPosture {
    InFlight,
    Presented,
}

struct UiNativePresentationRetryRound {
    attempt: UiMountedPresentationAttemptIdentity,
    requirement: Option<UiNativePresentationRetryRequirement>,
    effect_posture: Option<UiNativePresentationEffectPosture>,
}

pub(super) struct UiNativePresentationRetryPolicy {
    timeout_attempts: u8,
    wake: Option<UiNativePresentationRetryWake>,
    round: Option<UiNativePresentationRetryRound>,
}

impl UiNativePresentationRetryPolicy {
    pub(super) const fn new() -> Self {
        Self {
            timeout_attempts: 0,
            wake: None,
            round: None,
        }
    }

    pub(super) fn observe_outcome(
        &mut self,
        attempt: UiMountedPresentationAttemptIdentity,
        outcome: &UiHostSurfacePresentationOutcome,
    ) {
        self.begin_attempt(attempt);
        let round = self.round.as_mut().expect("attempt was installed");
        match outcome {
            UiHostSurfacePresentationOutcome::RejectedBeforeEffects(denial) => {
                let Some(requirement) = requirement_for(*denial) else {
                    return;
                };
                round.requirement = Some(
                    round
                        .requirement
                        .map_or(requirement, |current| current.max(requirement)),
                );
            }
            UiHostSurfacePresentationOutcome::Presented(_) => {
                round.effect_posture = Some(UiNativePresentationEffectPosture::Presented);
            }
            UiHostSurfacePresentationOutcome::InFlight(_) => {
                round.effect_posture = Some(
                    round
                        .effect_posture
                        .map_or(UiNativePresentationEffectPosture::InFlight, |current| {
                            current.max(UiNativePresentationEffectPosture::InFlight)
                        }),
                );
            }
            UiHostSurfacePresentationOutcome::PresentationIndeterminate => {
                round.requirement = Some(UiNativePresentationRetryRequirement::Reconstruction);
            }
        }
    }

    fn begin_attempt(&mut self, attempt: UiMountedPresentationAttemptIdentity) {
        if self
            .round
            .as_ref()
            .is_some_and(|round| round.attempt == attempt)
        {
            return;
        }
        if self.round.is_some() {
            self.timeout_attempts = 0;
            self.wake = None;
        }
        self.round = Some(UiNativePresentationRetryRound {
            attempt,
            requirement: None,
            effect_posture: None,
        });
    }

    pub(super) fn finalize_round(&mut self, now: Instant) -> UiNativePresentationRetryFinalization {
        let Some(round) = self.round.take() else {
            return UiNativePresentationRetryFinalization::Unchanged;
        };
        if round.effect_posture.is_some() {
            self.clear();
            return UiNativePresentationRetryFinalization::Unchanged;
        }
        match round.requirement {
            Some(UiNativePresentationRetryRequirement::Timeout) => {
                if self.timeout_attempts >= TIMEOUT_RETRY_LIMIT {
                    self.clear();
                    return UiNativePresentationRetryFinalization::DeadlineExpired;
                }
                self.timeout_attempts += 1;
                let Some(deadline) = now.checked_add(TIMEOUT_RETRY_DELAY) else {
                    self.clear();
                    return UiNativePresentationRetryFinalization::DeadlineExpired;
                };
                let wake = UiNativePresentationRetryWake::Timeout(deadline);
                self.wake = Some(wake);
                UiNativePresentationRetryFinalization::Wake(wake)
            }
            Some(UiNativePresentationRetryRequirement::TextAtlas) => {
                self.wake = None;
                UiNativePresentationRetryFinalization::Unchanged
            }
            Some(UiNativePresentationRetryRequirement::Visibility) => {
                let wake = UiNativePresentationRetryWake::Visibility;
                self.wake = Some(wake);
                UiNativePresentationRetryFinalization::Wake(wake)
            }
            Some(
                UiNativePresentationRetryRequirement::Reconstruction
                | UiNativePresentationRetryRequirement::Terminal,
            ) => {
                self.clear();
                UiNativePresentationRetryFinalization::Unchanged
            }
            None => UiNativePresentationRetryFinalization::Unchanged,
        }
    }

    pub(super) const fn wake(&self) -> Option<UiNativePresentationRetryWake> {
        self.wake
    }

    pub(super) fn consume_due_timeout(&mut self, now: Instant) -> bool {
        let Some(UiNativePresentationRetryWake::Timeout(deadline)) = self.wake else {
            return false;
        };
        if now < deadline {
            return false;
        }
        self.wake = None;
        true
    }

    pub(super) fn consume_visibility(&mut self) -> bool {
        if self.wake != Some(UiNativePresentationRetryWake::Visibility) {
            return false;
        }
        self.wake = None;
        true
    }

    pub(super) fn clear(&mut self) {
        self.timeout_attempts = 0;
        self.wake = None;
        self.round = None;
    }
}

fn requirement_for(
    denial: UiHostSurfacePresentationDenial,
) -> Option<UiNativePresentationRetryRequirement> {
    match denial {
        UiHostSurfacePresentationDenial::ExternalTimeout => {
            Some(UiNativePresentationRetryRequirement::Timeout)
        }
        UiHostSurfacePresentationDenial::SurfaceOccluded => {
            Some(UiNativePresentationRetryRequirement::Visibility)
        }
        UiHostSurfacePresentationDenial::ReconstructionRequired => {
            Some(UiNativePresentationRetryRequirement::Reconstruction)
        }
        UiHostSurfacePresentationDenial::TextAtlasPresentationDeferred => {
            Some(UiNativePresentationRetryRequirement::TextAtlas)
        }
        UiHostSurfacePresentationDenial::AdapterDeclined
        | UiHostSurfacePresentationDenial::ExternalValidationFailed
        | UiHostSurfacePresentationDenial::CancelledBeforeEffects
        | UiHostSurfacePresentationDenial::UnsupportedPresentationMode(_)
        | UiHostSurfacePresentationDenial::UnsupportedEffect(_)
        | UiHostSurfacePresentationDenial::Protocol(_)
        | UiHostSurfacePresentationDenial::ProtocolChanged
        | UiHostSurfacePresentationDenial::CapabilityGenerationChanged
        | UiHostSurfacePresentationDenial::CapabilityProfileChanged
        | UiHostSurfacePresentationDenial::SurfaceBindingChanged
        | UiHostSurfacePresentationDenial::StalePredecessor
        | UiHostSurfacePresentationDenial::MalformedProjection
        | UiHostSurfacePresentationDenial::DeadlineExpired
        | UiHostSurfacePresentationDenial::CapacityExceeded => {
            Some(UiNativePresentationRetryRequirement::Terminal)
        }
    }
}

#[cfg(test)]
#[path = "presentation_retry_tests.rs"]
mod tests;

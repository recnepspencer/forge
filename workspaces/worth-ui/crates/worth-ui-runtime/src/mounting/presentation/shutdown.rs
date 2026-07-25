use worth_ui_host_contract::{UiMountedPresentationAttemptIdentity, UiSurfaceBindingGeneration};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedPresentationShutdownDisposition {
    CancelledBeforeEffects,
    PresentationIndeterminate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedPresentationShutdownAttempt {
    attempt: UiMountedPresentationAttemptIdentity,
    disposition: UiMountedPresentationShutdownDisposition,
    affected_bindings: Box<[UiSurfaceBindingGeneration]>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UiMountedPresentationShutdownReport {
    attempts: Box<[UiMountedPresentationShutdownAttempt]>,
}

impl UiMountedPresentationShutdownAttempt {
    pub(super) fn new(
        attempt: UiMountedPresentationAttemptIdentity,
        disposition: UiMountedPresentationShutdownDisposition,
        affected_bindings: Vec<UiSurfaceBindingGeneration>,
    ) -> Self {
        Self {
            attempt,
            disposition,
            affected_bindings: affected_bindings.into_boxed_slice(),
        }
    }

    pub fn attempt(&self) -> UiMountedPresentationAttemptIdentity {
        self.attempt
    }

    pub fn disposition(&self) -> UiMountedPresentationShutdownDisposition {
        self.disposition
    }

    pub fn affected_bindings(&self) -> &[UiSurfaceBindingGeneration] {
        &self.affected_bindings
    }
}

impl UiMountedPresentationShutdownReport {
    pub(super) fn new(attempts: Vec<UiMountedPresentationShutdownAttempt>) -> Self {
        Self {
            attempts: attempts.into_boxed_slice(),
        }
    }

    pub fn attempts(&self) -> &[UiMountedPresentationShutdownAttempt] {
        &self.attempts
    }

    pub fn is_empty(&self) -> bool {
        self.attempts.is_empty()
    }
}

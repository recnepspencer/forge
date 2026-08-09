#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativePlatformStop {
    reason: UiNativePlatformStopReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativePlatformStopReason {
    NativeEffectsNotActivatedInPhaseOne,
}

#[must_use]
#[derive(Debug)]
pub enum UiNativePlatformOutcome {
    ApplicationPreparationDenied(super::UiNativeApplicationPreparationDenial),
    Stopped(UiNativePlatformStop),
}

impl UiNativePlatformStop {
    pub(crate) const fn phase_one_activation_boundary() -> Self {
        Self {
            reason: UiNativePlatformStopReason::NativeEffectsNotActivatedInPhaseOne,
        }
    }

    pub const fn reason(&self) -> UiNativePlatformStopReason {
        self.reason
    }
}

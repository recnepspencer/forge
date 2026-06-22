use crate::runtime::{WorthUiRuntimeActivationStatus, WorthUiRuntimeLifecycle};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeReplacementPosture {
    Supported,
    Deferred,
    Unsupported,
}

impl WorthUiRuntimeReplacementPosture {
    pub(crate) fn from_runtime_truth(
        lifecycle: WorthUiRuntimeLifecycle,
        status: WorthUiRuntimeActivationStatus,
    ) -> Self {
        match (lifecycle, status) {
            (WorthUiRuntimeLifecycle::Active, WorthUiRuntimeActivationStatus::Active) => {
                Self::Supported
            }
            (
                WorthUiRuntimeLifecycle::PausedForReplacement
                | WorthUiRuntimeLifecycle::PendingActivation,
                WorthUiRuntimeActivationStatus::Active,
            ) => Self::Deferred,
            _ => Self::Unsupported,
        }
    }

    pub fn is_supported(self) -> bool {
        matches!(self, Self::Supported)
    }
}

use super::routing::UiNativePhysicalSignalRequestToken;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativePhysicalSignalReadyAttempt {
    Current(UiNativePhysicalSignalRequestToken),
    Successor {
        predecessor: UiNativePhysicalSignalRequestToken,
        successor: UiNativePhysicalSignalRequestToken,
    },
}

impl UiNativePhysicalSignalReadyAttempt {
    pub(crate) const fn current(self) -> UiNativePhysicalSignalRequestToken {
        match self {
            Self::Current(current)
            | Self::Successor {
                successor: current, ..
            } => current,
        }
    }
}

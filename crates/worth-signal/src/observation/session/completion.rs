#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalObservationCompletion {
    Cancelled,
    Completed,
    NoExecution,
    Abandoned,
    /// The runtime crossed a snapshot/checkpoint boundary while this session
    /// was active, so its partial evidence was discarded before replacement.
    InterruptedByBoundary,
}

impl SignalObservationCompletion {
    pub(crate) const fn code(self) -> u8 {
        match self {
            Self::Cancelled => 1,
            Self::Completed => 2,
            Self::NoExecution => 3,
            Self::Abandoned => 4,
            Self::InterruptedByBoundary => 5,
        }
    }

    pub(crate) const fn from_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Cancelled),
            2 => Some(Self::Completed),
            3 => Some(Self::NoExecution),
            4 => Some(Self::Abandoned),
            5 => Some(Self::InterruptedByBoundary),
            _ => None,
        }
    }
}

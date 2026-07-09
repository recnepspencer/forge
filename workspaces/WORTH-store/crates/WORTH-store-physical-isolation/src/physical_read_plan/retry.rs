#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalReadPlanRetryPosture {
    Current,
    Retry,
    RebindRequired,
}

impl PhysicalReadPlanRetryPosture {
    pub const fn retry_decisions(self) -> u64 {
        match self {
            Self::Current => 0,
            Self::Retry | Self::RebindRequired => 1,
        }
    }
}

impl From<crate::EpochRetryDecision> for PhysicalReadPlanRetryPosture {
    fn from(decision: crate::EpochRetryDecision) -> Self {
        match decision {
            crate::EpochRetryDecision::Current => Self::Current,
            crate::EpochRetryDecision::Retry => Self::Retry,
            crate::EpochRetryDecision::RebindRequired => Self::RebindRequired,
        }
    }
}

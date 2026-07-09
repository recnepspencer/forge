use super::EpochComparisonScopeMismatch;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpochRetryDecision {
    Current,
    Retry,
    RebindRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalEpochDriftKind {
    ScopeMismatch,
    RootEpoch,
    ManifestEpoch,
    SegmentEpoch,
    ExtentEpoch,
    PageEpoch,
    ChunkEpoch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalEpochFreshness {
    decision: EpochRetryDecision,
    drift: Option<PhysicalEpochDriftKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StalePhysicalReadPlanDenial {
    decision: EpochRetryDecision,
    drift: PhysicalEpochDriftKind,
}

impl PhysicalEpochFreshness {
    pub const fn current() -> Self {
        Self {
            decision: EpochRetryDecision::Current,
            drift: None,
        }
    }

    pub const fn retry(drift: PhysicalEpochDriftKind) -> Self {
        Self {
            decision: EpochRetryDecision::Retry,
            drift: Some(drift),
        }
    }

    pub const fn rebind_required(drift: PhysicalEpochDriftKind) -> Self {
        Self {
            decision: EpochRetryDecision::RebindRequired,
            drift: Some(drift),
        }
    }

    pub const fn decision(self) -> EpochRetryDecision {
        self.decision
    }

    pub const fn drift(self) -> Option<PhysicalEpochDriftKind> {
        self.drift
    }

    pub const fn into_stale_read_plan_denial(self) -> Option<StalePhysicalReadPlanDenial> {
        match self.drift {
            Some(drift) => Some(StalePhysicalReadPlanDenial {
                decision: self.decision,
                drift,
            }),
            None => None,
        }
    }
}

impl StalePhysicalReadPlanDenial {
    pub const fn decision(self) -> EpochRetryDecision {
        self.decision
    }

    pub const fn drift(self) -> PhysicalEpochDriftKind {
        self.drift
    }
}

impl From<EpochComparisonScopeMismatch> for PhysicalEpochFreshness {
    fn from(_: EpochComparisonScopeMismatch) -> Self {
        Self::rebind_required(PhysicalEpochDriftKind::ScopeMismatch)
    }
}

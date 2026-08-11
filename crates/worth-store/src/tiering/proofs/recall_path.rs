use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ColdRecallTierPath {
    HotResident,
    WarmResident,
    ColdRecalled,
    RebuildAssistedDerived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RetainedReadPlacementPath {
    HotResident,
    WarmResident,
    ColdRecalled,
    RebuildAssistedDerived,
}

impl RetainedReadPlacementPath {
    pub fn tier_miss_outcome(self) -> TierMissOutcome {
        match self {
            Self::HotResident => TierMissOutcome::ResidentHit,
            Self::WarmResident => TierMissOutcome::WarmHit,
            Self::ColdRecalled => TierMissOutcome::ColdRecallHit,
            Self::RebuildAssistedDerived => TierMissOutcome::RebuildAssistedDerivedHit,
        }
    }
}

impl From<ColdRecallTierPath> for RetainedReadPlacementPath {
    fn from(value: ColdRecallTierPath) -> Self {
        match value {
            ColdRecallTierPath::HotResident => Self::HotResident,
            ColdRecallTierPath::WarmResident => Self::WarmResident,
            ColdRecallTierPath::ColdRecalled => Self::ColdRecalled,
            ColdRecallTierPath::RebuildAssistedDerived => Self::RebuildAssistedDerived,
        }
    }
}

impl From<RetainedReadPlacementPath> for ColdRecallTierPath {
    fn from(value: RetainedReadPlacementPath) -> Self {
        match value {
            RetainedReadPlacementPath::HotResident => Self::HotResident,
            RetainedReadPlacementPath::WarmResident => Self::WarmResident,
            RetainedReadPlacementPath::ColdRecalled => Self::ColdRecalled,
            RetainedReadPlacementPath::RebuildAssistedDerived => Self::RebuildAssistedDerived,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TierMissOutcome {
    ResidentHit,
    WarmHit,
    ColdRecallHit,
    RebuildAssistedDerivedHit,
}

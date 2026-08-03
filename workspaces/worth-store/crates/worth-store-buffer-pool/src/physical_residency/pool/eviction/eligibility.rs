use super::super::{FrameEntry, FrameState};

/// The selector-local reason a resident identity cannot become an eviction
/// victim. This type never leaves the pool: callers may observe pressure and
/// counters, but only the pool may classify or select a victim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EvictionEligibility {
    Legal,
    Pinned,
    Dirty,
    Loading,
    CandidatePublication,
    WritebackClaimed,
}

impl EvictionEligibility {
    pub(super) const fn violation_predicate(self) -> Option<&'static str> {
        match self {
            Self::Legal => None,
            Self::Pinned => Some("C5_PREDICATE:pinned-eviction"),
            Self::Dirty => Some("C5_PREDICATE:dirty-eviction"),
            Self::Loading => Some("C5_PREDICATE:loading-eviction"),
            Self::CandidatePublication => Some("C5_PREDICATE:candidate-eviction"),
            Self::WritebackClaimed => Some("C5_PREDICATE:writeback-claimed-eviction"),
        }
    }
}

impl FrameEntry {
    pub(in crate::physical_residency::pool::eviction) fn eviction_eligibility(
        &self,
    ) -> EvictionEligibility {
        if self.pins != 0 {
            EvictionEligibility::Pinned
        } else if self.writeback_claimed {
            EvictionEligibility::WritebackClaimed
        } else if self.dirty {
            EvictionEligibility::Dirty
        } else if self.loading_waiters != 0 {
            EvictionEligibility::Loading
        } else {
            match self.state {
                FrameState::Loading | FrameState::LoadFailed(_) => EvictionEligibility::Loading,
                FrameState::CandidateReserved => EvictionEligibility::CandidatePublication,
                FrameState::Resident(_) => EvictionEligibility::Legal,
            }
        }
    }

    pub(in crate::physical_residency::pool) fn is_evictable(&self) -> bool {
        self.eviction_eligibility() == EvictionEligibility::Legal
    }
}

use super::*;
use crate::runtime::allocation_frame_dispatch::UiAllocationFrameDispatcherCounters;

/// Move-only support result for one proof-bearing source retirement attempt.
#[derive(Debug, Eq, PartialEq)]
pub struct UiAllocationFrameSourceRetirementOutcome {
    counters: UiAllocationFrameDispatcherCounters,
    representation: UiAllocationFrameSourceRetirementRepresentation,
}

#[derive(Debug, Eq, PartialEq)]
enum UiAllocationFrameSourceRetirementRepresentation {
    Retired,
    Denied {
        denial: UiAllocationFrameSourceRetirementDenial,
        lease: crate::runtime::allocation_frame_dispatch::UiAllocationFrameSourceLease,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationFrameSourceRetirementDenial {
    NotTracked,
    LeaseExpired,
    GenerationMismatch {
        tracked_generation: UiAllocationFrameSourceGeneration,
    },
}

impl UiAllocationFrameSourceRetirementOutcome {
    pub(in crate::runtime::allocation_frame_dispatch) fn retired(
        counters: UiAllocationFrameDispatcherCounters,
    ) -> Self {
        Self {
            counters,
            representation: UiAllocationFrameSourceRetirementRepresentation::Retired,
        }
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn denied_from_registry(
        denial: UiAllocationFrameSourceRetirementDenial,
        counters: UiAllocationFrameDispatcherCounters,
        lease: crate::runtime::allocation_frame_dispatch::UiAllocationFrameSourceLease,
    ) -> Self {
        Self {
            counters,
            representation: UiAllocationFrameSourceRetirementRepresentation::Denied {
                denial,
                lease,
            },
        }
    }

    pub fn is_retired(&self) -> bool {
        matches!(
            self.representation,
            UiAllocationFrameSourceRetirementRepresentation::Retired
        )
    }

    pub fn denial(&self) -> Option<UiAllocationFrameSourceRetirementDenial> {
        match self.representation {
            UiAllocationFrameSourceRetirementRepresentation::Retired => None,
            UiAllocationFrameSourceRetirementRepresentation::Denied { denial, .. } => Some(denial),
        }
    }

    pub fn counters(&self) -> UiAllocationFrameDispatcherCounters {
        self.counters
    }

    pub fn denied_lease(
        &self,
    ) -> Option<&crate::runtime::allocation_frame_dispatch::UiAllocationFrameSourceLease> {
        match &self.representation {
            UiAllocationFrameSourceRetirementRepresentation::Retired => None,
            UiAllocationFrameSourceRetirementRepresentation::Denied { lease, .. } => Some(lease),
        }
    }

    pub fn into_denied_lease(
        self,
    ) -> Option<crate::runtime::allocation_frame_dispatch::UiAllocationFrameSourceLease> {
        match self.representation {
            UiAllocationFrameSourceRetirementRepresentation::Retired => None,
            UiAllocationFrameSourceRetirementRepresentation::Denied { lease, .. } => Some(lease),
        }
    }
}

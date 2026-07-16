use super::*;
use crate::runtime::allocation_frame_dispatch::UiAllocationFrameDispatcherCounters;

/// Move-only support result for one proof-bearing source retirement attempt.
#[derive(Debug, Eq, PartialEq)]
pub struct UiAllocationFrameSourceRetirementOutcome {
    retired: bool,
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
        _counters: UiAllocationFrameDispatcherCounters,
    ) -> Self {
        Self { retired: true }
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn denied_from_registry(
        _denial: UiAllocationFrameSourceRetirementDenial,
        _counters: UiAllocationFrameDispatcherCounters,
        _lease: crate::runtime::allocation_frame_dispatch::UiAllocationFrameSourceLease,
    ) -> Self {
        Self { retired: false }
    }

    pub fn is_retired(&self) -> bool {
        self.retired
    }
}

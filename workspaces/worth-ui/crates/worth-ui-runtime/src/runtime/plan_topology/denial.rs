use crate::runtime::WorthUiPlanTopologyCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanTopologyDenial {
    reason: WorthUiPlanTopologyDenialReason,
    counters: WorthUiPlanTopologyCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPlanTopologyDenialReason {
    AllocationPlanningDenied,
    HandleAllocationReceiptMismatch,
    MissingRuntimeHandle,
    RuntimeHandleOutOfBounds,
    RuntimeHandleFamilyMismatch,
    OrphanedChildRangeHandle,
    MissingChildOrLaneLink,
    LaneAdmissionMismatch,
    MissingEguiBoundaryDeclaration,
    MissingRegionStructure,
}

impl WorthUiPlanTopologyDenial {
    pub(crate) fn new(
        reason: WorthUiPlanTopologyDenialReason,
        mut counters: WorthUiPlanTopologyCounters,
    ) -> Self {
        counters.record_denial();
        Self { reason, counters }
    }

    pub fn reason(&self) -> WorthUiPlanTopologyDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiPlanTopologyCounters {
        self.counters
    }
}

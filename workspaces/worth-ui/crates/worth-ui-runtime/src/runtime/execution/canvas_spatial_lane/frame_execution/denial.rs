use crate::runtime::{WorthUiCanvasSpatialCounters, WorthUiHandleResolutionEvidence};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCanvasSpatialPlanDenialReason {
    LaneAdmissionMissingCanvasSpatialSupport,
    LaneAdmissionPlanMismatch,
    HandleAllocationPlanMismatch,
    NoCanvasSpatialRows,
    HostSupportMissing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCanvasSpatialFrameDenialReason {
    TargetNotInCanvasSpatialPlan,
    TargetArenaMismatch,
    TargetSlotGenerationMismatch,
    TargetFamilyMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialPlanDenial {
    reason: WorthUiCanvasSpatialPlanDenialReason,
    counters: Box<WorthUiCanvasSpatialCounters>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialFrameDenial {
    reason: WorthUiCanvasSpatialFrameDenialReason,
    plan_index: Option<u32>,
    counters: Box<WorthUiCanvasSpatialCounters>,
    resolution_evidence: Option<WorthUiHandleResolutionEvidence>,
}

impl WorthUiCanvasSpatialPlanDenial {
    pub(crate) fn new(
        reason: WorthUiCanvasSpatialPlanDenialReason,
        counters: WorthUiCanvasSpatialCounters,
    ) -> Self {
        Self {
            reason,
            counters: Box::new(counters),
        }
    }

    pub fn reason(&self) -> WorthUiCanvasSpatialPlanDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiCanvasSpatialCounters {
        *self.counters
    }
}

impl WorthUiCanvasSpatialFrameDenial {
    pub(crate) fn new(
        reason: WorthUiCanvasSpatialFrameDenialReason,
        plan_index: Option<u32>,
        counters: WorthUiCanvasSpatialCounters,
    ) -> Self {
        Self {
            reason,
            plan_index,
            counters: Box::new(counters),
            resolution_evidence: None,
        }
    }

    pub fn reason(&self) -> WorthUiCanvasSpatialFrameDenialReason {
        self.reason
    }

    pub fn plan_index(&self) -> Option<u32> {
        self.plan_index
    }

    pub fn counters(&self) -> WorthUiCanvasSpatialCounters {
        *self.counters
    }

    pub(crate) fn with_resolution_evidence(
        mut self,
        evidence: WorthUiHandleResolutionEvidence,
    ) -> Self {
        self.resolution_evidence = Some(evidence);
        self
    }

    pub fn resolution_evidence(&self) -> Option<WorthUiHandleResolutionEvidence> {
        self.resolution_evidence
    }
}

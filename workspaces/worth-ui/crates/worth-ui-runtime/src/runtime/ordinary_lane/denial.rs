use crate::runtime::{WorthUiHandleResolutionEvidence, WorthUiOrdinaryLaneCounters};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiOrdinaryLanePlanDenialReason {
    LaneAdmissionMissingOrdinarySupport,
    LaneAdmissionMissingCommandSurfaceSupport,
    LaneAdmissionMissingStyleTokenSupport,
    NoOrdinaryRows,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiOrdinaryLaneFrameDenialReason {
    ActivePlanNotOrdinaryExecutable,
    TargetNotInOrdinaryPlan,
    TargetArenaMismatch,
    TargetSlotGenerationMismatch,
    TargetFamilyMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiOrdinaryLanePlanDenial {
    reason: WorthUiOrdinaryLanePlanDenialReason,
    counters: Box<WorthUiOrdinaryLaneCounters>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiOrdinaryLaneFrameDenial {
    reason: WorthUiOrdinaryLaneFrameDenialReason,
    plan_index: Option<u32>,
    counters: Box<WorthUiOrdinaryLaneCounters>,
    resolution_evidence: Option<WorthUiHandleResolutionEvidence>,
}

impl WorthUiOrdinaryLanePlanDenial {
    pub(crate) fn new(
        reason: WorthUiOrdinaryLanePlanDenialReason,
        counters: WorthUiOrdinaryLaneCounters,
    ) -> Self {
        Self {
            reason,
            counters: Box::new(counters),
        }
    }

    pub fn reason(&self) -> WorthUiOrdinaryLanePlanDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiOrdinaryLaneCounters {
        *self.counters
    }
}

impl WorthUiOrdinaryLaneFrameDenial {
    pub(crate) fn new(
        reason: WorthUiOrdinaryLaneFrameDenialReason,
        plan_index: Option<u32>,
        counters: WorthUiOrdinaryLaneCounters,
    ) -> Self {
        Self {
            reason,
            plan_index,
            counters: Box::new(counters),
            resolution_evidence: None,
        }
    }

    pub fn reason(&self) -> WorthUiOrdinaryLaneFrameDenialReason {
        self.reason
    }

    pub fn plan_index(&self) -> Option<u32> {
        self.plan_index
    }

    pub fn counters(&self) -> WorthUiOrdinaryLaneCounters {
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

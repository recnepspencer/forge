use crate::runtime::{WorthUiHandleResolutionEvidence, WorthUiRealtimeLaneCounters};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiHudPlanDenialReason {
    LaneAdmissionMissingRealtimeSupport,
    NoHudRows,
    HostSupportMissing,
    FrameBudgetExhausted {
        budget_millis: u16,
        declared_cost_millis: u16,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiRealtimeFrameDenialReason {
    TargetNotInHudPlan,
    TargetArenaMismatch,
    TargetSlotGenerationMismatch,
    TargetFamilyMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHudPlanDenial {
    reason: WorthUiHudPlanDenialReason,
    counters: Box<WorthUiRealtimeLaneCounters>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRealtimeFrameDenial {
    reason: WorthUiRealtimeFrameDenialReason,
    plan_index: Option<u32>,
    counters: Box<WorthUiRealtimeLaneCounters>,
    resolution_evidence: Option<WorthUiHandleResolutionEvidence>,
}

impl WorthUiHudPlanDenial {
    pub(crate) fn new(
        reason: WorthUiHudPlanDenialReason,
        counters: WorthUiRealtimeLaneCounters,
    ) -> Self {
        Self {
            reason,
            counters: Box::new(counters),
        }
    }

    pub fn reason(&self) -> WorthUiHudPlanDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiRealtimeLaneCounters {
        *self.counters
    }
}

impl WorthUiRealtimeFrameDenial {
    pub(crate) fn new(
        reason: WorthUiRealtimeFrameDenialReason,
        plan_index: Option<u32>,
        counters: WorthUiRealtimeLaneCounters,
    ) -> Self {
        Self {
            reason,
            plan_index,
            counters: Box::new(counters),
            resolution_evidence: None,
        }
    }

    pub fn reason(&self) -> WorthUiRealtimeFrameDenialReason {
        self.reason
    }

    pub fn plan_index(&self) -> Option<u32> {
        self.plan_index
    }

    pub fn counters(&self) -> WorthUiRealtimeLaneCounters {
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

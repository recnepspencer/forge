use crate::runtime::WorthUiRealtimeLaneCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiHudPlanDenialReason {
    LaneAdmissionMissingRealtimeSupport,
    LaneAdmissionMissingRenderResourceSupport,
    LaneAdmissionPlanMismatch,
    HandleAllocationPlanMismatch,
    MissingRealtimeOverlayHook,
    UnsupportedRealtimeOverlayHook,
    NoHudRows,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiRealtimeFrameDenialReason {
    TargetNotInHudPlan,
    TargetGenerationMismatch,
    OrdinaryWidgetFallback,
    HiddenOrdinaryLayoutPass,
    ForbiddenWorkCounterSuppression,
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
}

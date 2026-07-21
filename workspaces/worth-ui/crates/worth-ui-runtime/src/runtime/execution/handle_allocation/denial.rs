use crate::runtime::WorthUiRuntimeHandleAllocationCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeHandleAllocationDenial {
    reason: WorthUiRuntimeHandleAllocationDenialReason,
    counters: WorthUiRuntimeHandleAllocationCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeHandleAllocationDenialReason {
    DuplicatePlanLocalHandleClaim,
    CollidingPlanLocalHandleClaim,
    StalePlanInputReceipt,
    UnsupportedHandleFamily,
    MissingQueryBindingEvidence,
    PlanIndexCapacityExhausted,
}

impl WorthUiRuntimeHandleAllocationDenial {
    pub(crate) fn new(
        reason: WorthUiRuntimeHandleAllocationDenialReason,
        counters: WorthUiRuntimeHandleAllocationCounters,
    ) -> Self {
        Self { reason, counters }
    }

    pub fn reason(&self) -> WorthUiRuntimeHandleAllocationDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiRuntimeHandleAllocationCounters {
        self.counters
    }
}

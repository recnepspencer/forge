use crate::runtime::WorthUiPlanInspectionCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPlanInspectionDenialReason {
    PlanInputReceiptMismatch,
    PlanInputNodeCountMismatch,
    PlanNodeFamilyMismatch,
    RuntimeHandlePlanIndexMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanInspectionDenial {
    reason: WorthUiPlanInspectionDenialReason,
    counters: WorthUiPlanInspectionCounters,
}

impl WorthUiPlanInspectionDenial {
    pub(crate) fn new(
        reason: WorthUiPlanInspectionDenialReason,
        mut counters: WorthUiPlanInspectionCounters,
    ) -> Self {
        counters.record_denial();
        Self { reason, counters }
    }

    pub fn reason(&self) -> WorthUiPlanInspectionDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiPlanInspectionCounters {
        self.counters
    }
}

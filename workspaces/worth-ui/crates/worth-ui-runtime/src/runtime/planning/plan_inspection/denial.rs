use crate::runtime::WorthUiPlanInspectionCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPlanInspectionDenialReason {
    ForeignLoweringAuthority,
    PlanInputReceiptMismatch,
    PlanInputNodeCountMismatch,
    PlanNodeFamilyMismatch,
    RuntimeHandlePlanIndexMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanInspectionDenial {
    reason: WorthUiPlanInspectionDenialReason,
    counters: Box<WorthUiPlanInspectionCounters>,
}

impl WorthUiPlanInspectionDenial {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn new(
        reason: WorthUiPlanInspectionDenialReason,
        mut counters: WorthUiPlanInspectionCounters,
    ) -> Self {
        counters.record_denial();
        Self {
            reason,
            counters: Box::new(counters),
        }
    }

    pub fn reason(&self) -> WorthUiPlanInspectionDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiPlanInspectionCounters {
        *self.counters
    }
}

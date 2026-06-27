use crate::runtime::WorthUiFileRustReplacementParityCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiFileRustReplacementParityDenial {
    reason: WorthUiFileRustReplacementParityDenialReason,
    counters: WorthUiFileRustReplacementParityCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiFileRustReplacementParityDenialReason {
    FileReportWasNotFileAuthored,
    RustReportWasNotRustAuthored,
    CandidateBasisMismatch,
    ArtifactComparisonMismatch,
    ExecutionPlanParityMismatch,
    LaneParityMismatch,
    ActivationReceiptMismatch,
    CandidateAdmissionDenied,
    ArtifactComparisonDenied,
    ImpactClassificationDenied,
    ImpactNarrowingDenied,
    IdentityMatchingDenied,
    NodeReplacementDenied,
    StateInventoryDenied,
    DurableStateReconciliationDenied,
    QueryBindingComparisonDenied,
    QueryLiveRebindDenied,
    ActivationStagingDenied,
    PlanLoweringDenied,
    HandleAllocationDenied,
    LaneAdmissionDenied,
    TopologyAssemblyDenied,
    ReadyActivationDenied,
    PlanSwapDenied,
}

impl WorthUiFileRustReplacementParityDenial {
    pub(crate) fn new(
        reason: WorthUiFileRustReplacementParityDenialReason,
        mut counters: WorthUiFileRustReplacementParityCounters,
    ) -> Self {
        counters.record_denial();
        Self { reason, counters }
    }

    pub fn reason(self) -> WorthUiFileRustReplacementParityDenialReason {
        self.reason
    }

    pub fn counters(self) -> WorthUiFileRustReplacementParityCounters {
        self.counters
    }
}

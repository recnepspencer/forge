use crate::runtime::{WorthUiHandleResolutionEvidence, WorthUiVirtualizedDataCounters};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiVisibleRangeDenialReason {
    EmptyRowRange,
    EmptyColumnRange,
    RangeOverflow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiVirtualizedDataPlanDenialReason {
    LaneAdmissionMissingVirtualizedDataSupport,
    LaneAdmissionMissingQuerySupport,
    LaneAdmissionPlanMismatch,
    MissingInstalledQueryReference,
    NoVirtualizedDataRows,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiVirtualizedDataFrameDenialReason {
    TargetNotInVirtualizedDataPlan,
    TargetArenaMismatch,
    TargetSlotGenerationMismatch,
    TargetFamilyMismatch,
    ActivePlanIsQueryFree,
    QueryNotInstalled,
    ForeignInstalledReference,
    ProjectionNotAdmitted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiVisibleRangeDenial {
    reason: WorthUiVisibleRangeDenialReason,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataPlanDenial {
    reason: WorthUiVirtualizedDataPlanDenialReason,
    counters: Box<WorthUiVirtualizedDataCounters>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataFrameDenial {
    reason: WorthUiVirtualizedDataFrameDenialReason,
    plan_index: Option<u32>,
    counters: Box<WorthUiVirtualizedDataCounters>,
    resolution_evidence: Option<Box<WorthUiHandleResolutionEvidence>>,
}

impl WorthUiVisibleRangeDenial {
    pub(crate) fn new(reason: WorthUiVisibleRangeDenialReason) -> Self {
        Self { reason }
    }

    pub fn reason(&self) -> WorthUiVisibleRangeDenialReason {
        self.reason
    }
}

impl WorthUiVirtualizedDataPlanDenial {
    pub(crate) fn new(
        reason: WorthUiVirtualizedDataPlanDenialReason,
        counters: WorthUiVirtualizedDataCounters,
    ) -> Self {
        Self {
            reason,
            counters: Box::new(counters),
        }
    }

    pub fn reason(&self) -> WorthUiVirtualizedDataPlanDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiVirtualizedDataCounters {
        *self.counters
    }
}

impl WorthUiVirtualizedDataFrameDenial {
    pub(crate) fn new(
        reason: WorthUiVirtualizedDataFrameDenialReason,
        plan_index: Option<u32>,
        counters: WorthUiVirtualizedDataCounters,
    ) -> Self {
        Self {
            reason,
            plan_index,
            counters: Box::new(counters),
            resolution_evidence: None,
        }
    }

    pub fn reason(&self) -> WorthUiVirtualizedDataFrameDenialReason {
        self.reason
    }

    pub fn plan_index(&self) -> Option<u32> {
        self.plan_index
    }

    pub fn counters(&self) -> WorthUiVirtualizedDataCounters {
        *self.counters
    }

    pub(crate) fn with_resolution_evidence(
        mut self,
        evidence: WorthUiHandleResolutionEvidence,
    ) -> Self {
        self.resolution_evidence = Some(Box::new(evidence));
        self
    }

    pub fn resolution_evidence(&self) -> Option<WorthUiHandleResolutionEvidence> {
        self.resolution_evidence.as_deref().copied()
    }
}

use crate::runtime::WorthUiVirtualizedDataCounters;

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
    MissingQueryPatchPosture,
    NoVirtualizedDataRows,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiVirtualizedDataFrameDenialReason {
    TargetNotInVirtualizedDataPlan,
    TargetGenerationMismatch,
    NonDataLaneClaim,
    OffsetPaginationSubstitute,
    FullCollectionScanCertificationFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiVisibleRangeDenial {
    reason: WorthUiVisibleRangeDenialReason,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataPlanDenial {
    reason: WorthUiVirtualizedDataPlanDenialReason,
    counters: WorthUiVirtualizedDataCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataFrameDenial {
    reason: WorthUiVirtualizedDataFrameDenialReason,
    plan_index: Option<u32>,
    counters: WorthUiVirtualizedDataCounters,
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
        Self { reason, counters }
    }

    pub fn reason(&self) -> WorthUiVirtualizedDataPlanDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiVirtualizedDataCounters {
        self.counters
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
            counters,
        }
    }

    pub fn reason(&self) -> WorthUiVirtualizedDataFrameDenialReason {
        self.reason
    }

    pub fn plan_index(&self) -> Option<u32> {
        self.plan_index
    }

    pub fn counters(&self) -> WorthUiVirtualizedDataCounters {
        self.counters
    }
}

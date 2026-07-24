#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCollectionResultPosture {
    Ready,
    Advisory,
    Pending,
    Partial,
    Violation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCollectionContinuationPosture {
    Complete,
    AdditionalSnapshotRows,
    AdditionalLiveRows,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCollectionWarningPosture {
    execution_warning_count: usize,
    projection_warning_present: bool,
    allocation_budget_clamped: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCollectionResetReason {
    ReexecutionRequired,
    CapabilityRebindRequired,
    ReplacementRequired,
    RetirementRequired,
    UnsupportedIncrementalMeaning,
    UnappliedPriorChange,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCollectionChangeInspection {
    result: Option<WorthUiCollectionResultPosture>,
    warnings: WorthUiCollectionWarningPosture,
    continuation: Option<WorthUiCollectionContinuationPosture>,
    foundational_scope_count: usize,
}

impl WorthUiCollectionWarningPosture {
    pub(crate) fn record_execution_warnings(&mut self, count: usize) {
        self.execution_warning_count = self.execution_warning_count.saturating_add(count);
    }

    pub(crate) fn record_projection_warning(&mut self) {
        self.projection_warning_present = true;
    }

    pub(crate) fn record_allocation_budget_clamp(&mut self) {
        self.allocation_budget_clamped = true;
    }

    pub fn execution_warning_count(self) -> usize {
        self.execution_warning_count
    }

    pub fn projection_warning_present(self) -> bool {
        self.projection_warning_present
    }

    pub fn allocation_budget_clamped(self) -> bool {
        self.allocation_budget_clamped
    }
}

impl WorthUiCollectionChangeInspection {
    pub(crate) fn new(foundational_scope_count: usize) -> Self {
        Self {
            foundational_scope_count,
            ..Self::default()
        }
    }

    pub(crate) fn set_result(&mut self, result: WorthUiCollectionResultPosture) {
        self.result = Some(result);
    }

    pub(crate) fn warnings_mut(&mut self) -> &mut WorthUiCollectionWarningPosture {
        &mut self.warnings
    }

    pub(crate) fn set_continuation(&mut self, continuation: WorthUiCollectionContinuationPosture) {
        self.continuation = Some(continuation);
    }

    pub fn result(self) -> Option<WorthUiCollectionResultPosture> {
        self.result
    }

    pub fn warnings(self) -> WorthUiCollectionWarningPosture {
        self.warnings
    }

    pub fn continuation(self) -> Option<WorthUiCollectionContinuationPosture> {
        self.continuation
    }

    pub fn foundational_scope_count(self) -> usize {
        self.foundational_scope_count
    }
}

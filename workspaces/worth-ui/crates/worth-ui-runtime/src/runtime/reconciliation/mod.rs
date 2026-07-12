mod families;
mod outcome;
mod plan;
mod planner;

/// Capability held only by durable reconciliation when it emits durable semantic truth.
pub(crate) struct UiAllocationDurableSemanticStateMintAuthority(());

impl UiAllocationDurableSemanticStateMintAuthority {
    const fn new() -> Self {
        Self(())
    }
}

pub use families::{
    WorthUiFocusChainReconciliation, WorthUiPanelVisibilityReconciliation,
    WorthUiScrollAnchorReconciliation, WorthUiSelectionRangeReconciliation,
    WorthUiSplitterPositionReconciliation, WorthUiTabStateReconciliation,
    WorthUiTextEditStateReconciliation,
};
pub use outcome::{
    WorthUiDurableStateCarryForward, WorthUiDurableStateReconciliationOutcome,
    WorthUiDurableStateReplacement,
};
pub(crate) use plan::WorthUiDurableResizeSourceAuthority;
pub use plan::{
    WorthUiAdmittedDurableResizeInput, WorthUiAdmittedDurableResizeSourceFact,
    WorthUiDurableResizeInputDisposition, WorthUiDurableResizeInputPosture,
    WorthUiDurableResizeSourceAdmissionDenial, WorthUiDurableStateReconciliationCounters,
    WorthUiDurableStateReconciliationDenial, WorthUiDurableStateReconciliationPlan,
    WorthUiDurableStateReconciliationReceipt,
};
pub(crate) use planner::WorthUiDurableStateReconciliationPlanner;

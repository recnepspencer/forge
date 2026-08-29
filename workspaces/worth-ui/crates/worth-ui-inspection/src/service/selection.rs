use super::{UiRuntimeServiceInspectionCost, UiRuntimeServiceInspectionSource};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiSelectionDropInspectionReason {
    Interaction,
    CatalogReconciliation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiSelectionDroppedInspectionSummary {
    source: UiRuntimeServiceInspectionSource,
    reason: UiSelectionDropInspectionReason,
    removed_count: u32,
    selected_count: usize,
    cost: UiRuntimeServiceInspectionCost,
}

impl UiSelectionDroppedInspectionSummary {
    pub const fn new(
        source: UiRuntimeServiceInspectionSource,
        reason: UiSelectionDropInspectionReason,
        removed_count: u32,
        selected_count: usize,
        cost: UiRuntimeServiceInspectionCost,
    ) -> Self {
        Self {
            source,
            reason,
            removed_count,
            selected_count,
            cost,
        }
    }

    pub const fn source(self) -> UiRuntimeServiceInspectionSource {
        self.source
    }
    pub const fn reason(self) -> UiSelectionDropInspectionReason {
        self.reason
    }
    pub const fn removed_count(self) -> u32 {
        self.removed_count
    }
    pub const fn selected_count(self) -> usize {
        self.selected_count
    }
    pub const fn cost(self) -> UiRuntimeServiceInspectionCost {
        self.cost
    }
}

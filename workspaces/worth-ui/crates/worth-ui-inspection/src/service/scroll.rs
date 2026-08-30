use super::{UiRuntimeServiceInspectionCost, UiRuntimeServiceInspectionSource};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiScrollOwnerInspectionSummary {
    source: UiRuntimeServiceInspectionSource,
    owners_visited: u16,
    owners_changed: u16,
    remainder_present: bool,
    cost: UiRuntimeServiceInspectionCost,
}

impl UiScrollOwnerInspectionSummary {
    pub const fn new(
        source: UiRuntimeServiceInspectionSource,
        owners_visited: u16,
        owners_changed: u16,
        remainder_present: bool,
        cost: UiRuntimeServiceInspectionCost,
    ) -> Self {
        Self {
            source,
            owners_visited,
            owners_changed,
            remainder_present,
            cost,
        }
    }

    pub const fn source(self) -> UiRuntimeServiceInspectionSource {
        self.source
    }
    pub const fn owners_visited(self) -> u16 {
        self.owners_visited
    }
    pub const fn owners_changed(self) -> u16 {
        self.owners_changed
    }
    pub const fn remainder_present(self) -> bool {
        self.remainder_present
    }
    pub const fn cost(self) -> UiRuntimeServiceInspectionCost {
        self.cost
    }
}

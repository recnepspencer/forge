use super::{UiRuntimeServiceInspectionCost, UiRuntimeServiceInspectionSource};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiPortalClosedInspectionReason {
    Escape,
    OutsidePress,
    AcceptedSelection,
    ExplicitOwnerRequest,
    AnchorLoss,
    ParentClosed,
    OwnerLoss,
    ApplicationShutdown,
    WindowFocusPolicy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiPortalClosedInspectionSummary {
    source: UiRuntimeServiceInspectionSource,
    reason: UiPortalClosedInspectionReason,
    closed_descendants: u16,
    cost: UiRuntimeServiceInspectionCost,
}

impl UiPortalClosedInspectionSummary {
    pub const fn new(
        source: UiRuntimeServiceInspectionSource,
        reason: UiPortalClosedInspectionReason,
        closed_descendants: u16,
        cost: UiRuntimeServiceInspectionCost,
    ) -> Self {
        Self {
            source,
            reason,
            closed_descendants,
            cost,
        }
    }

    pub const fn source(self) -> UiRuntimeServiceInspectionSource {
        self.source
    }
    pub const fn reason(self) -> UiPortalClosedInspectionReason {
        self.reason
    }
    pub const fn closed_descendants(self) -> u16 {
        self.closed_descendants
    }
    pub const fn cost(self) -> UiRuntimeServiceInspectionCost {
        self.cost
    }
}

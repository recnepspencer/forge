use super::selection_transition::{self, S8IssuedSelection, S8SelectionIssuedPayload};
use super::{S8PlanSelectionDenied, S8SelectedAccessPlan};

#[derive(Debug, PartialEq, Eq)]
pub struct S8AccessPlanSelectionOutcome {
    issued: S8IssuedSelection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8AccessPlanSelectionView<'a> {
    Selected(&'a S8SelectedAccessPlan),
    Denied(&'a S8PlanSelectionDenied),
}

impl S8AccessPlanSelectionOutcome {
    pub(super) const fn from_issued(issued: S8IssuedSelection) -> Self {
        Self { issued }
    }

    pub fn view(&self) -> S8AccessPlanSelectionView<'_> {
        match self.issued.payload() {
            S8SelectionIssuedPayload::Selected(plan) => S8AccessPlanSelectionView::Selected(plan),
            S8SelectionIssuedPayload::Denied(denial) => S8AccessPlanSelectionView::Denied(denial),
        }
    }

    pub const fn production_transition(
        &self,
    ) -> crate::production_transition::S8LayoutProductionTransition {
        self.issued.transition()
    }

    pub fn unwrap(self) -> S8SelectedAccessPlan {
        match self.issued.into_payload() {
            S8SelectionIssuedPayload::Selected(plan) => plan,
            S8SelectionIssuedPayload::Denied(denial) => panic!("selection denied: {denial:?}"),
        }
    }

    pub fn unwrap_err(self) -> S8PlanSelectionDenied {
        match self.issued.into_payload() {
            S8SelectionIssuedPayload::Denied(denial) => denial,
            S8SelectionIssuedPayload::Selected(_) => panic!("selection unexpectedly succeeded"),
        }
    }

    pub(crate) fn indexed_contract() -> crate::production_transition::S8OwnerTransitionContract {
        selection_transition::indexed_contract()
    }

    pub(crate) fn degraded_contract() -> crate::production_transition::S8OwnerTransitionContract {
        selection_transition::degraded_contract()
    }
}

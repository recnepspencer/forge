use super::selection_decision::S8PlanSelectionDecision;
use super::{S8PlanSelectionDenied, S8SelectedAccessPlan};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum S8SelectionIssuedPayload {
    Selected(S8SelectedAccessPlan),
    Denied(S8PlanSelectionDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct S8IssuedSelection {
    payload: S8SelectionIssuedPayload,
}

impl S8IssuedSelection {
    pub(super) const fn payload(&self) -> &S8SelectionIssuedPayload {
        &self.payload
    }

    pub(super) fn into_payload(self) -> S8SelectionIssuedPayload {
        self.payload
    }
}

pub(super) fn issue_selection_outcome(
    decision: S8PlanSelectionDecision,
) -> super::S8AccessPlanSelectionOutcome {
    let (_, result) = decision.into_parts();
    let payload = match result {
        Ok(plan) => S8SelectionIssuedPayload::Selected(plan),
        Err(denial) => S8SelectionIssuedPayload::Denied(denial),
    };
    super::S8AccessPlanSelectionOutcome::from_issued(S8IssuedSelection { payload })
}

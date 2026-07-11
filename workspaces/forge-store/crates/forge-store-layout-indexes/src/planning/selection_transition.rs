use super::selection_decision::{S8PlanSelectionDecision, S8SelectionRoute};
use super::{S8PlanSelectionDenied, S8SelectedAccessPlan};
use crate::production_transition::{
    owner_transition, S8LayoutMachineState as State, S8LayoutMachineTransition as Transition,
    S8LayoutProductionOperation as Operation, S8LayoutProductionTransition,
    S8LayoutStateMachine as Machine, S8OwnerIssuedCase, S8OwnerTransitionContract,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum S8SelectionIssuedPayload {
    Selected(S8SelectedAccessPlan),
    Denied(S8PlanSelectionDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct S8IssuedSelection {
    issued: S8OwnerIssuedCase<S8SelectionIssuedPayload>,
}

impl S8IssuedSelection {
    pub(super) const fn payload(&self) -> &S8SelectionIssuedPayload {
        self.issued.payload()
    }

    pub(super) fn into_payload(self) -> S8SelectionIssuedPayload {
        self.issued.into_payload()
    }

    pub(super) const fn transition(&self) -> S8LayoutProductionTransition {
        self.issued.transition()
    }
}

pub(super) fn issue_selection_outcome(
    decision: S8PlanSelectionDecision,
) -> super::S8AccessPlanSelectionOutcome {
    let (route, result) = decision.into_parts();
    let (payload, transition) = match result {
        Ok(plan) => (
            S8SelectionIssuedPayload::Selected(plan),
            transition_for(route, true),
        ),
        Err(denial) => (
            S8SelectionIssuedPayload::Denied(denial),
            transition_for(route, false),
        ),
    };
    super::S8AccessPlanSelectionOutcome::from_issued(S8IssuedSelection {
        issued: S8OwnerIssuedCase::issue(payload, transition),
    })
}

pub(super) fn indexed_contract() -> S8OwnerTransitionContract {
    static FACTS: [S8LayoutProductionTransition; 2] = [
        transition_for(S8SelectionRoute::Indexed, true),
        transition_for(S8SelectionRoute::Indexed, false),
    ];
    S8OwnerTransitionContract::from_owner_outcomes(
        Machine::AccessSelectionAndBudgetAdmission,
        Operation::SelectAccessPlanWithBudget,
        &FACTS,
    )
}

pub(super) fn degraded_contract() -> S8OwnerTransitionContract {
    static FACTS: [S8LayoutProductionTransition; 2] = [
        transition_for(S8SelectionRoute::Degraded, true),
        transition_for(S8SelectionRoute::Degraded, false),
    ];
    S8OwnerTransitionContract::from_owner_outcomes(
        Machine::DegradedExactScan,
        Operation::ExecuteBudgetedDegradedExactScan,
        &FACTS,
    )
}

const fn transition_for(route: S8SelectionRoute, selected: bool) -> S8LayoutProductionTransition {
    match (route, selected) {
        (S8SelectionRoute::Indexed, true) => owner_transition(
            Machine::AccessSelectionAndBudgetAdmission,
            Operation::SelectAccessPlanWithBudget,
            "Selected",
            State::Admitted,
            Transition::SelectAndAdmitBudget,
            State::Budgeted,
        ),
        (S8SelectionRoute::Indexed, false) => owner_transition(
            Machine::AccessSelectionAndBudgetAdmission,
            Operation::SelectAccessPlanWithBudget,
            "Denied",
            State::Admitted,
            Transition::Deny,
            State::Denied,
        ),
        (S8SelectionRoute::Degraded, true) => owner_transition(
            Machine::DegradedExactScan,
            Operation::ExecuteBudgetedDegradedExactScan,
            "Selected",
            State::SelectionRequested,
            Transition::Budget,
            State::Budgeted,
        ),
        (S8SelectionRoute::Degraded, false) => owner_transition(
            Machine::DegradedExactScan,
            Operation::ExecuteBudgetedDegradedExactScan,
            "Denied",
            State::SelectionRequested,
            Transition::Deny,
            State::Denied,
        ),
    }
}

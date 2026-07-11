use super::{
    S8AccessPlanCostEstimate, S8DeterministicSelectionRule, S8PlanFingerprint,
    S8SelectionCandidateAudit,
};
use crate::access::budget::S8PlannedCounterEnvelope;
use crate::access::shape::S8AccessShapeContract;
use crate::strategy::S8LayoutStrategyFamily;
use forge_store_budgets::S8PreExecutionBudgetAdmissionReceipt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8SelectedAccessPlan {
    selected_family: S8LayoutStrategyFamily,
    access_shape: S8AccessShapeContract,
    fingerprint: S8PlanFingerprint,
    cost_estimate: S8AccessPlanCostEstimate,
    planned_counter_envelope: S8PlannedCounterEnvelope,
    budget_receipt: S8PreExecutionBudgetAdmissionReceipt,
    selection_rule: S8DeterministicSelectionRule,
    primary_candidate: S8SelectionCandidateAudit,
    secondary_candidate: S8SelectionCandidateAudit,
}

impl S8SelectedAccessPlan {
    #[allow(clippy::too_many_arguments)]
    pub(super) const fn new(
        selected_family: S8LayoutStrategyFamily,
        access_shape: S8AccessShapeContract,
        fingerprint: S8PlanFingerprint,
        cost_estimate: S8AccessPlanCostEstimate,
        planned_counter_envelope: S8PlannedCounterEnvelope,
        budget_receipt: S8PreExecutionBudgetAdmissionReceipt,
        selection_rule: S8DeterministicSelectionRule,
        primary_candidate: S8SelectionCandidateAudit,
        secondary_candidate: S8SelectionCandidateAudit,
    ) -> Self {
        Self {
            selected_family,
            access_shape,
            fingerprint,
            cost_estimate,
            planned_counter_envelope,
            budget_receipt,
            selection_rule,
            primary_candidate,
            secondary_candidate,
        }
    }

    pub const fn selected_family(self) -> S8LayoutStrategyFamily {
        self.selected_family
    }
    pub const fn access_shape(self) -> S8AccessShapeContract {
        self.access_shape
    }
    pub const fn fingerprint(self) -> S8PlanFingerprint {
        self.fingerprint
    }
    pub const fn cost_estimate(self) -> S8AccessPlanCostEstimate {
        self.cost_estimate
    }
    pub const fn planned_counter_envelope(self) -> S8PlannedCounterEnvelope {
        self.planned_counter_envelope
    }
    pub const fn budget_receipt(self) -> S8PreExecutionBudgetAdmissionReceipt {
        self.budget_receipt
    }
    pub const fn selection_rule(self) -> S8DeterministicSelectionRule {
        self.selection_rule
    }
    pub const fn primary_candidate(self) -> S8SelectionCandidateAudit {
        self.primary_candidate
    }
    pub const fn secondary_candidate(self) -> S8SelectionCandidateAudit {
        self.secondary_candidate
    }
}

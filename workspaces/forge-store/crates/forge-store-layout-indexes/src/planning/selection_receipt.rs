use super::selection_decision::decide_access_plan;
use super::selection_issuance::issue_selection_outcome;
use super::selection_outcome::S8AccessPlanSelectionOutcome;
use crate::access::shape::S8AccessShapeContract;
use crate::catalog::ArtifactFamilyLifecycleAdmission;
use crate::keyspace::PhysicalKeyDomainWitness;
use forge_store_budgets::S8PreExecutionBudgetEnvelope;

/// The sole operation authorized to decide and issue an access-plan outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8AccessPlanSelection;

impl S8AccessPlanSelection {
    pub fn select_with_budget(
        &self,
        lifecycle: ArtifactFamilyLifecycleAdmission,
        key_domain: PhysicalKeyDomainWitness,
        access_shape: S8AccessShapeContract,
        admitted_budget: S8PreExecutionBudgetEnvelope,
    ) -> S8AccessPlanSelectionOutcome {
        issue_selection_outcome(decide_access_plan(
            lifecycle,
            key_domain,
            access_shape,
            admitted_budget,
        ))
    }
}

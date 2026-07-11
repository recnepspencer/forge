use super::alternative::{S8PlanningAlternative, S8PlanningAlternativeSet};
use super::selection_policy::S8SelectionPolicy;
use super::{
    S8AccessPlanCostEstimate, S8DeterministicSelectionRule, S8PlanFingerprint,
    S8PlanSelectionDenied, S8SelectedAccessPlan, S8SelectionCandidateAudit,
    S8SelectionCandidateEligibility, S8SelectionCandidateOutcome,
};
use crate::access::budget::S8PlannedCounterEnvelope;
use crate::access::execution::S8AccessPathCounterSnapshot;
use crate::access::shape::{S8AccessAuthorityPosture, S8AccessShapeContract};
use crate::catalog::{ArtifactFamilyLifecycleAdmission, AuthorityRole};
use crate::keyspace::PhysicalKeyDomainWitness;
use crate::strategy::S8LayoutStrategyFamily;
use forge_store_budgets::{
    pre_execution_budget_admission, S8PreExecutionBudgetAdmissionOutcome,
    S8PreExecutionBudgetEnvelope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum S8SelectionRoute {
    Indexed,
    Degraded,
}

pub(super) struct S8PlanSelectionDecision {
    route: S8SelectionRoute,
    result: Result<S8SelectedAccessPlan, S8PlanSelectionDenied>,
}

impl S8PlanSelectionDecision {
    pub(super) fn into_parts(
        self,
    ) -> (
        S8SelectionRoute,
        Result<S8SelectedAccessPlan, S8PlanSelectionDenied>,
    ) {
        (self.route, self.result)
    }
}

pub(super) fn decide_access_plan(
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
    access_shape: S8AccessShapeContract,
    admitted_budget: S8PreExecutionBudgetEnvelope,
) -> S8PlanSelectionDecision {
    let route = if is_degraded(access_shape) {
        S8SelectionRoute::Degraded
    } else {
        S8SelectionRoute::Indexed
    };
    let result = if is_degraded(access_shape) {
        decide_degraded_plan(key_domain, access_shape, admitted_budget)
    } else {
        decide_indexed_plan(lifecycle, key_domain, access_shape, admitted_budget)
    };
    S8PlanSelectionDecision { route, result }
}

fn decide_indexed_plan(
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
    access_shape: S8AccessShapeContract,
    admitted_budget: S8PreExecutionBudgetEnvelope,
) -> Result<S8SelectedAccessPlan, S8PlanSelectionDenied> {
    let alternatives = S8PlanningAlternativeSet::derive(lifecycle, key_domain, access_shape);
    let (selected, selection_rule) = select_alternative(alternatives, access_shape)?;
    let selected_family = selected.snapshot().admitted_strategy().family();
    let planned_counter_envelope =
        crate::strategy::planned_counter_envelope_for(selected_family, access_shape.detail())
            .expect("eligible alternatives declare planned envelopes");
    admit_budget_for_plan(
        selected_family,
        key_domain,
        access_shape,
        planned_counter_envelope,
        admitted_budget,
        selection_rule,
        alternatives.primary_audit(),
        alternatives.secondary_audit(),
    )
}

fn decide_degraded_plan(
    key_domain: PhysicalKeyDomainWitness,
    access_shape: S8AccessShapeContract,
    admitted_budget: S8PreExecutionBudgetEnvelope,
) -> Result<S8SelectedAccessPlan, S8PlanSelectionDenied> {
    let planned_counter_envelope = S8PlannedCounterEnvelope::new(
        S8AccessPathCounterSnapshot::exact(0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 4_096, 0, 0, 1, 0),
        zero_counters(),
        zero_counters(),
    );
    admit_budget_for_plan(
        S8LayoutStrategyFamily::ExactScan,
        key_domain,
        access_shape,
        planned_counter_envelope,
        admitted_budget,
        S8DeterministicSelectionRule::ExplicitDegradedExactScan,
        S8SelectionCandidateAudit::new(
            S8LayoutStrategyFamily::ExactScan,
            AuthorityRole::SemanticAuthorityConsumer,
            S8SelectionCandidateOutcome::Eligible(
                S8SelectionCandidateEligibility::ExplicitDegradedExactScan {
                    planned_counter_envelope,
                    budget_rows: access_shape.budget_rows().unwrap_or(0),
                },
            ),
        ),
        S8SelectionCandidateAudit::new(
            S8LayoutStrategyFamily::SparseIndex,
            AuthorityRole::SemanticAuthorityConsumer,
            S8SelectionCandidateOutcome::Rejected(
                super::S8SelectionCandidateRejection::StrategyUnsupported,
            ),
        ),
    )
}

fn select_alternative(
    alternatives: S8PlanningAlternativeSet,
    access_shape: S8AccessShapeContract,
) -> Result<(S8PlanningAlternative, S8DeterministicSelectionRule), S8PlanSelectionDenied> {
    let policy = S8SelectionPolicy;
    match (alternatives.primary(), alternatives.secondary()) {
        (Some(primary), None) | (None, Some(primary)) => {
            Ok((primary, S8DeterministicSelectionRule::SoleEligibleCandidate))
        }
        (None, None) => Err(S8PlanSelectionDenied::NoEligibleAlternative),
        (Some(primary), Some(secondary)) => {
            let rule = policy.rule(access_shape.shape());
            let first_rank = policy.rank(
                primary.snapshot().admitted_strategy().family(),
                access_shape.shape(),
            );
            let second_rank = policy.rank(
                secondary.snapshot().admitted_strategy().family(),
                access_shape.shape(),
            );
            if first_rank == second_rank {
                return Err(S8PlanSelectionDenied::AmbiguousAlternativeOrdering {
                    first_family: primary.snapshot().admitted_strategy().family(),
                    second_family: secondary.snapshot().admitted_strategy().family(),
                });
            }
            Ok((
                if first_rank < second_rank {
                    primary
                } else {
                    secondary
                },
                rule,
            ))
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn admit_budget_for_plan(
    selected_family: S8LayoutStrategyFamily,
    key_domain: PhysicalKeyDomainWitness,
    access_shape: S8AccessShapeContract,
    planned_counter_envelope: S8PlannedCounterEnvelope,
    admitted_budget: S8PreExecutionBudgetEnvelope,
    selection_rule: S8DeterministicSelectionRule,
    primary_candidate: S8SelectionCandidateAudit,
    secondary_candidate: S8SelectionCandidateAudit,
) -> Result<S8SelectedAccessPlan, S8PlanSelectionDenied> {
    let cost_estimate = S8AccessPlanCostEstimate::from_selected_plan(
        selected_family,
        access_shape,
        planned_counter_envelope,
    );
    let fingerprint = S8PlanFingerprint::new(
        selected_family,
        access_shape.detail(),
        access_shape.lane(),
        access_shape.authority_posture(),
        access_shape.stale_disposition(),
        key_domain,
        access_shape.expected_counters(),
        access_shape.mutation_shape(),
        access_shape.budget_rows(),
        planned_counter_envelope,
        selection_rule,
    );
    let budget_request = cost_estimate.to_budget_request(
        fingerprint.plan_binding(),
        S8AccessPlanCostEstimate::budget_scope_for(access_shape),
    );
    let budget_receipt =
        match pre_execution_budget_admission().admit(budget_request, admitted_budget) {
            S8PreExecutionBudgetAdmissionOutcome::Admitted(receipt) => receipt,
            S8PreExecutionBudgetAdmissionOutcome::Denied(denial) => {
                return Err(S8PlanSelectionDenied::BudgetDenied(denial));
            }
        };
    Ok(S8SelectedAccessPlan::new(
        selected_family,
        access_shape,
        fingerprint,
        cost_estimate,
        planned_counter_envelope,
        budget_receipt,
        selection_rule,
        primary_candidate,
        secondary_candidate,
    ))
}

const fn is_degraded(access_shape: S8AccessShapeContract) -> bool {
    matches!(
        access_shape.authority_posture(),
        S8AccessAuthorityPosture::ExplicitDegradedExactScan
    )
}

const fn zero_counters() -> S8AccessPathCounterSnapshot {
    S8AccessPathCounterSnapshot::exact(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
}

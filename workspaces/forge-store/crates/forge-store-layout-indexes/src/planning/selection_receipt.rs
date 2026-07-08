use super::alternative::{
    S8PlanningAlternative, S8PlanningAlternativeSet, S8SelectionCandidateAudit,
    S8SelectionCandidateOutcome,
};
use super::cost::S8AccessPlanCostEstimate;
use super::denial::S8PlanSelectionDenied;
use super::plan_fingerprint::S8PlanFingerprint;
use super::selection_basis::{S8DeterministicSelectionRule, S8SelectionCandidateEligibility};
use super::selection_policy::S8SelectionPolicy;
use crate::access_shape::{S8AccessAuthorityPosture, S8AccessShapeContract};
use crate::artifact_family::ArtifactFamilyLifecycleAdmission;
use crate::budget::S8PlannedCounterEnvelope;
use crate::execution::S8AccessPathCounterSnapshot;
use crate::key_domain::PhysicalKeyDomainWitness;
use crate::strategy::S8LayoutStrategyFamily;
use forge_store_budgets::{
    pre_execution_budget_admission, S8PreExecutionBudgetAdmissionOutcome,
    S8PreExecutionBudgetAdmissionReceipt, S8PreExecutionBudgetEnvelope,
};

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
    pub(crate) const fn new(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8AccessPlanSelection;

impl S8AccessPlanSelection {
    pub fn select_with_budget(
        &self,
        lifecycle: ArtifactFamilyLifecycleAdmission,
        key_domain: PhysicalKeyDomainWitness,
        access_shape: S8AccessShapeContract,
        admitted_budget: S8PreExecutionBudgetEnvelope,
    ) -> Result<S8SelectedAccessPlan, S8PlanSelectionDenied> {
        if access_shape.authority_posture() == S8AccessAuthorityPosture::ExplicitDegradedExactScan {
            return self.select_degraded_with_budget(key_domain, access_shape, admitted_budget);
        }

        let alternatives = S8PlanningAlternativeSet::derive(lifecycle, key_domain, access_shape);
        let (selected, selection_rule) = self.select_alternative(alternatives, access_shape)?;
        let planned_counter_envelope = crate::strategy::planned_counter_envelope_for(
            selected.snapshot().admitted_strategy().family(),
            access_shape.detail(),
        )
        .expect("eligible alternatives declare planned envelopes");
        self.admit_budget_for_plan(
            selected.snapshot().admitted_strategy().family(),
            key_domain,
            access_shape,
            planned_counter_envelope,
            admitted_budget,
            selection_rule,
            alternatives.primary_audit(),
            alternatives.secondary_audit(),
        )
    }

    fn select_degraded_with_budget(
        &self,
        key_domain: PhysicalKeyDomainWitness,
        access_shape: S8AccessShapeContract,
        admitted_budget: S8PreExecutionBudgetEnvelope,
    ) -> Result<S8SelectedAccessPlan, S8PlanSelectionDenied> {
        let planned_counter_envelope = S8PlannedCounterEnvelope::new(
            S8AccessPathCounterSnapshot::exact(0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 4_096, 0, 0, 1, 0),
            S8AccessPathCounterSnapshot::exact(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
            S8AccessPathCounterSnapshot::exact(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
        );
        self.admit_budget_for_plan(
            S8LayoutStrategyFamily::ExactScan,
            key_domain,
            access_shape,
            planned_counter_envelope,
            admitted_budget,
            S8DeterministicSelectionRule::ExplicitDegradedExactScan,
            S8SelectionCandidateAudit::new(
                S8LayoutStrategyFamily::ExactScan,
                crate::artifact_family::AuthorityRole::SemanticAuthorityConsumer,
                S8SelectionCandidateOutcome::Eligible(
                    S8SelectionCandidateEligibility::ExplicitDegradedExactScan {
                        planned_counter_envelope,
                        budget_rows: access_shape.budget_rows().unwrap_or(0),
                    },
                ),
            ),
            S8SelectionCandidateAudit::new(
                S8LayoutStrategyFamily::SparseIndex,
                crate::artifact_family::AuthorityRole::SemanticAuthorityConsumer,
                super::alternative::S8SelectionCandidateOutcome::Rejected(
                    super::denial::S8SelectionCandidateRejection::StrategyUnsupported,
                ),
            ),
        )
    }

    fn select_alternative(
        &self,
        alternatives: S8PlanningAlternativeSet,
        access_shape: S8AccessShapeContract,
    ) -> Result<(S8PlanningAlternative, S8DeterministicSelectionRule), S8PlanSelectionDenied> {
        let policy = S8SelectionPolicy;
        match (alternatives.primary(), alternatives.secondary()) {
            (Some(primary), None) => {
                Ok((primary, S8DeterministicSelectionRule::SoleEligibleCandidate))
            }
            (None, Some(secondary)) => Ok((
                secondary,
                S8DeterministicSelectionRule::SoleEligibleCandidate,
            )),
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
                if first_rank < second_rank {
                    Ok((primary, rule))
                } else {
                    Ok((secondary, rule))
                }
            }
        }
    }

    fn admit_budget_for_plan(
        &self,
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
}

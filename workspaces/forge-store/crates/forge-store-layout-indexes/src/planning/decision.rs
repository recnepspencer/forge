use super::candidates::{EligibleStrategyOperation, PlanningAlternative, PlanningAlternativeSet};
use super::selected_plan::{admit_selected_plan_budget, PlanSelectionBasis};
use super::{
    AccessPlanSelectionDenied, DeterministicSelectionRule, SelectedAccessPlanBasis,
    SelectionCandidateAudit, SelectionCandidateEligibility, SelectionCandidateOutcome,
};
use crate::access::budget::PlannedCounterEnvelope;
use crate::access::execution::AccessPathCounterSnapshot;
use crate::access::shape::AccessAuthorityPosture;
use crate::access::AdmittedAccessIntent;
use crate::artifact_family::AdmittedPhysicalArtifactFamily;
use crate::catalog::AuthorityRole;
use crate::keyspace::{AdmittedPhysicalAccessIdentity, AdmittedPhysicalKeyDomain};
use crate::materialization::AdmittedLayoutMaterialization;
use crate::strategy::LayoutStrategyFamily;
use forge_store_budgets::PreExecutionBudgetEnvelope;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum PlanSelectionDecision {
    BTreePointLookup(SelectedAccessPlanBasis, BTreeLookupSelectionGrant),
    BTreeRangeLookup(SelectedAccessPlanBasis, BTreeLookupSelectionGrant),
    BTreePrefixLookup(SelectedAccessPlanBasis, BTreeLookupSelectionGrant),
    BTreeReplayRecovery(SelectedAccessPlanBasis, BTreeReplaySelectionGrant),
    LsmLookup(SelectedAccessPlanBasis, LsmLookupSelectionGrant),
    LsmRunPublication(SelectedAccessPlanBasis, LsmPublicationSelectionGrant),
    LsmReplayRecovery(SelectedAccessPlanBasis, LsmReplaySelectionGrant),
    LsmCompaction(SelectedAccessPlanBasis, LsmCompactionSelectionGrant),
    Degraded(SelectedAccessPlanBasis, DegradedScanSelectionGrant),
    Denied(AccessPlanSelectionDenied),
}

macro_rules! define_selection_grant {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub(super) struct $name {
            _issued_by_decision_owner: (),
        }

        impl $name {
            const fn issue() -> Self {
                Self {
                    _issued_by_decision_owner: (),
                }
            }
        }
    };
}

define_selection_grant!(BTreeLookupSelectionGrant);
define_selection_grant!(BTreeReplaySelectionGrant);
define_selection_grant!(LsmLookupSelectionGrant);
define_selection_grant!(LsmPublicationSelectionGrant);
define_selection_grant!(LsmReplaySelectionGrant);
define_selection_grant!(LsmCompactionSelectionGrant);
define_selection_grant!(DegradedScanSelectionGrant);

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum AccessPlanSelectionCase {
    BTreePointLookup,
    BTreeRangeLookup,
    BTreePrefixLookup,
    BTreeReplayRecovery,
    LsmLookup,
    LsmRunPublication,
    LsmReplayRecovery,
    LsmCompaction,
    DegradedExactScan,
    Denied,
}

#[cfg(test)]
impl AccessPlanSelectionCase {
    pub const ALL: [Self; 10] = [
        Self::BTreePointLookup,
        Self::BTreeRangeLookup,
        Self::BTreePrefixLookup,
        Self::BTreeReplayRecovery,
        Self::LsmLookup,
        Self::LsmRunPublication,
        Self::LsmReplayRecovery,
        Self::LsmCompaction,
        Self::DegradedExactScan,
        Self::Denied,
    ];
}

pub(super) fn decide_access_plan(
    lifecycle: AdmittedPhysicalArtifactFamily,
    key_domain: AdmittedPhysicalKeyDomain,
    request_identity: AdmittedPhysicalAccessIdentity,
    materialization: Option<AdmittedLayoutMaterialization>,
    intent: AdmittedAccessIntent,
    admitted_budget: PreExecutionBudgetEnvelope,
) -> PlanSelectionDecision {
    if is_degraded(intent) {
        match decide_degraded_plan(
            lifecycle,
            key_domain,
            request_identity,
            materialization,
            intent,
            admitted_budget,
        ) {
            Ok(plan) => PlanSelectionDecision::Degraded(plan, DegradedScanSelectionGrant::issue()),
            Err(denial) => PlanSelectionDecision::Denied(denial),
        }
    } else {
        match decide_indexed_plan(
            lifecycle,
            key_domain,
            request_identity,
            materialization,
            intent,
            admitted_budget,
        ) {
            Ok(plan) => classify_indexed_operation(plan),
            Err(denial) => PlanSelectionDecision::Denied(denial),
        }
    }
}

fn classify_indexed_operation(plan: SelectedAccessPlanBasis) -> PlanSelectionDecision {
    match plan
        .selected_operation()
        .expect("indexed candidate retains its owner-classified operation")
    {
        EligibleStrategyOperation::BTreeLookup(super::BTreeLookupOperation::Point) => {
            PlanSelectionDecision::BTreePointLookup(plan, BTreeLookupSelectionGrant::issue())
        }
        EligibleStrategyOperation::BTreeLookup(super::BTreeLookupOperation::Range) => {
            PlanSelectionDecision::BTreeRangeLookup(plan, BTreeLookupSelectionGrant::issue())
        }
        EligibleStrategyOperation::BTreeLookup(super::BTreeLookupOperation::Prefix) => {
            PlanSelectionDecision::BTreePrefixLookup(plan, BTreeLookupSelectionGrant::issue())
        }
        EligibleStrategyOperation::BTreeReplayRecovery => {
            PlanSelectionDecision::BTreeReplayRecovery(plan, BTreeReplaySelectionGrant::issue())
        }
        EligibleStrategyOperation::LsmLookup => {
            PlanSelectionDecision::LsmLookup(plan, LsmLookupSelectionGrant::issue())
        }
        EligibleStrategyOperation::LsmRunPublication => {
            PlanSelectionDecision::LsmRunPublication(plan, LsmPublicationSelectionGrant::issue())
        }
        EligibleStrategyOperation::LsmReplayRecovery => {
            PlanSelectionDecision::LsmReplayRecovery(plan, LsmReplaySelectionGrant::issue())
        }
        EligibleStrategyOperation::LsmCompaction => {
            PlanSelectionDecision::LsmCompaction(plan, LsmCompactionSelectionGrant::issue())
        }
    }
}

fn decide_indexed_plan(
    lifecycle: AdmittedPhysicalArtifactFamily,
    key_domain: AdmittedPhysicalKeyDomain,
    request_identity: AdmittedPhysicalAccessIdentity,
    materialization: Option<AdmittedLayoutMaterialization>,
    intent: AdmittedAccessIntent,
    admitted_budget: PreExecutionBudgetEnvelope,
) -> Result<SelectedAccessPlanBasis, AccessPlanSelectionDenied> {
    let alternatives =
        PlanningAlternativeSet::derive(lifecycle, key_domain, materialization.clone(), intent);
    let (selected, selection_rule) = select_alternative(&alternatives)?;
    let strategy_admission = selected.snapshot().clone();
    let admitted_strategy = strategy_admission.admitted_strategy();
    let selected_family = admitted_strategy.family();
    let selected_operation = selected.operation();
    let planned_counter_envelope =
        crate::strategy::planned_counter_envelope_for(selected_family, intent.detail())
            .expect("eligible alternatives declare planned envelopes");
    admit_budget_for_plan(
        PlanSelectionBasis {
            family: lifecycle,
            key_domain,
            request_identity,
            materialization,
            strategy_admission: Some(strategy_admission),
            selected_family,
            selected_operation: Some(selected_operation),
            intent,
            planned_counter_envelope,
            selection_rule,
            primary_candidate: alternatives.primary_audit().clone(),
            secondary_candidate: alternatives.secondary_audit().clone(),
        },
        admitted_budget,
    )
}

fn decide_degraded_plan(
    lifecycle: AdmittedPhysicalArtifactFamily,
    key_domain: AdmittedPhysicalKeyDomain,
    request_identity: AdmittedPhysicalAccessIdentity,
    materialization: Option<AdmittedLayoutMaterialization>,
    intent: AdmittedAccessIntent,
    admitted_budget: PreExecutionBudgetEnvelope,
) -> Result<SelectedAccessPlanBasis, AccessPlanSelectionDenied> {
    let planned_counter_envelope = PlannedCounterEnvelope::new(
        AccessPathCounterSnapshot::exact(0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 4_096, 0, 0, 1, 0),
        zero_counters(),
        zero_counters(),
    );
    admit_budget_for_plan(
        PlanSelectionBasis {
            family: lifecycle,
            key_domain,
            request_identity,
            materialization,
            strategy_admission: None,
            selected_family: LayoutStrategyFamily::ExactScan,
            selected_operation: None,
            intent,
            planned_counter_envelope,
            selection_rule: DeterministicSelectionRule::ExplicitDegradedExactScan,
            primary_candidate: SelectionCandidateAudit::new(
                LayoutStrategyFamily::ExactScan,
                AuthorityRole::SemanticAuthorityConsumer,
                SelectionCandidateOutcome::Eligible(
                    SelectionCandidateEligibility::ExplicitDegradedExactScan {
                        planned_counter_envelope,
                        budget_rows: intent.budget_rows().unwrap_or(0),
                    },
                ),
            ),
            secondary_candidate: SelectionCandidateAudit::new(
                LayoutStrategyFamily::SparseIndex,
                AuthorityRole::SemanticAuthorityConsumer,
                SelectionCandidateOutcome::Rejected(
                    super::SelectionCandidateRejection::NotApplicableToExplicitDegradedScan,
                ),
            ),
        },
        admitted_budget,
    )
}

fn select_alternative(
    alternatives: &PlanningAlternativeSet,
) -> Result<(PlanningAlternative, DeterministicSelectionRule), AccessPlanSelectionDenied> {
    match (alternatives.primary(), alternatives.secondary()) {
        (Some(primary), None) | (None, Some(primary)) => Ok((
            primary.clone(),
            DeterministicSelectionRule::SoleEligibleCandidate,
        )),
        (None, None) => Err(AccessPlanSelectionDenied::NoEligibleAlternative),
        (Some(primary), Some(secondary)) => Err(
            AccessPlanSelectionDenied::OverlappingEligibleStrategyAuthority {
                first_family: primary.snapshot().admitted_strategy().family(),
                second_family: secondary.snapshot().admitted_strategy().family(),
            },
        ),
    }
}

fn admit_budget_for_plan(
    basis: PlanSelectionBasis,
    admitted_budget: PreExecutionBudgetEnvelope,
) -> Result<SelectedAccessPlanBasis, AccessPlanSelectionDenied> {
    admit_selected_plan_budget(basis, admitted_budget)
}

const fn is_degraded(intent: AdmittedAccessIntent) -> bool {
    matches!(
        intent.authority_posture(),
        AccessAuthorityPosture::ExplicitDegradedExactScan
    )
}

const fn zero_counters() -> AccessPathCounterSnapshot {
    AccessPathCounterSnapshot::exact(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
}

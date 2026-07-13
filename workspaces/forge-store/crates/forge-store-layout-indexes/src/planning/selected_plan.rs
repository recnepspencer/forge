use super::candidates::EligibleStrategyOperation;
use super::cost::derive_access_plan_cost;
use super::decision::{
    BTreeLookupSelectionGrant, BTreeReplaySelectionGrant, DegradedScanSelectionGrant,
    LsmCompactionSelectionGrant, LsmLookupSelectionGrant, LsmPublicationSelectionGrant,
    LsmReplaySelectionGrant,
};
use super::{
    AccessPlanCostEstimate, AccessPlanIdentity, DeterministicSelectionRule, SelectionCandidateAudit,
};
use crate::access::budget::PlannedCounterEnvelope;
use crate::access::AdmittedAccessIntent;
use crate::artifact_family::AdmittedPhysicalArtifactFamily;
use crate::catalog::ArtifactFamilyLifecycleAdmission;
use crate::keyspace::{
    AdmittedPhysicalAccessIdentity, AdmittedPhysicalKeyDomain, PhysicalKeyDomainWitness,
};
use crate::materialization::AdmittedLayoutMaterialization;
use crate::strategy::registry::LayoutStrategyRegistrySnapshot;
use crate::strategy::{AdmittedLayoutStrategy, LayoutStrategyFamily};
use forge_store_budgets::{
    pre_execution_budget_admission, PreExecutionBudgetAdmissionOutcome,
    PreExecutionBudgetAdmissionReceipt, PreExecutionBudgetEnvelope,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PlanSelectionBasis {
    pub(super) family: AdmittedPhysicalArtifactFamily,
    pub(super) key_domain: AdmittedPhysicalKeyDomain,
    pub(super) request_identity: AdmittedPhysicalAccessIdentity,
    pub(super) materialization: Option<AdmittedLayoutMaterialization>,
    pub(super) strategy_admission: Option<LayoutStrategyRegistrySnapshot>,
    pub(super) selected_family: LayoutStrategyFamily,
    pub(super) selected_operation: Option<EligibleStrategyOperation>,
    pub(super) intent: AdmittedAccessIntent,
    pub(super) planned_counter_envelope: PlannedCounterEnvelope,
    pub(super) selection_rule: DeterministicSelectionRule,
    pub(super) primary_candidate: SelectionCandidateAudit,
    pub(super) secondary_candidate: SelectionCandidateAudit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CostedAccessPlan {
    basis: PlanSelectionBasis,
    cost_estimate: AccessPlanCostEstimate,
}

impl CostedAccessPlan {
    fn derive(basis: PlanSelectionBasis) -> Result<Self, super::AccessPlanCostDenial> {
        let cost_estimate = derive_access_plan_cost(
            basis.selected_operation,
            basis.intent,
            basis.planned_counter_envelope,
            basis.materialization.clone(),
        )?;
        Ok(Self {
            basis,
            cost_estimate,
        })
    }

    const fn budget_request(&self) -> forge_store_budgets::PreExecutionBudgetRequest {
        self.cost_estimate
            .to_budget_request(AccessPlanCostEstimate::budget_scope_for(self.basis.intent))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedAccessPlanBasis {
    costed: CostedAccessPlan,
    budget_receipt: PreExecutionBudgetAdmissionReceipt,
    identity: AccessPlanIdentity,
}

impl SelectedAccessPlanBasis {
    fn from_budget_admission(
        costed: CostedAccessPlan,
        budget_receipt: PreExecutionBudgetAdmissionReceipt,
    ) -> Self {
        let basis = &costed.basis;
        let identity = AccessPlanIdentity::new(
            basis.family,
            basis.selected_family,
            basis.intent.detail(),
            basis.intent.lane(),
            basis.intent.authority_posture(),
            basis.intent.stale_disposition(),
            basis.key_domain,
            basis.request_identity,
            basis.materialization.clone(),
            basis.strategy_admission.clone(),
            basis.intent.expected_counters(),
            basis.intent.mutation_shape(),
            basis.intent.budget_rows(),
            basis.planned_counter_envelope,
            basis.selection_rule,
            costed.cost_estimate.clone(),
            budget_receipt.request(),
            budget_receipt.admitted_envelope(),
        );
        Self {
            costed,
            budget_receipt,
            identity,
        }
    }

    pub const fn lifecycle(&self) -> ArtifactFamilyLifecycleAdmission {
        self.costed.basis.family.lifecycle()
    }
    pub const fn admitted_family(&self) -> AdmittedPhysicalArtifactFamily {
        self.costed.basis.family
    }
    pub const fn key_domain(&self) -> PhysicalKeyDomainWitness {
        self.costed.basis.key_domain.witness()
    }
    pub const fn admitted_key_domain(&self) -> AdmittedPhysicalKeyDomain {
        self.costed.basis.key_domain
    }
    pub const fn request_identity(&self) -> AdmittedPhysicalAccessIdentity {
        self.costed.basis.request_identity
    }

    pub const fn materialization(&self) -> Option<&AdmittedLayoutMaterialization> {
        self.costed.basis.materialization.as_ref()
    }

    pub const fn admitted_strategy(&self) -> Option<AdmittedLayoutStrategy> {
        match &self.costed.basis.strategy_admission {
            Some(admission) => Some(admission.admitted_strategy()),
            None => None,
        }
    }

    pub const fn strategy_admission(&self) -> Option<&LayoutStrategyRegistrySnapshot> {
        self.costed.basis.strategy_admission.as_ref()
    }

    pub const fn selected_family(&self) -> LayoutStrategyFamily {
        self.costed.basis.selected_family
    }
    pub(super) const fn selected_operation(&self) -> Option<EligibleStrategyOperation> {
        self.costed.basis.selected_operation
    }
    pub const fn intent(&self) -> AdmittedAccessIntent {
        self.costed.basis.intent
    }
    pub const fn fingerprint(&self) -> &AccessPlanIdentity {
        &self.identity
    }
    pub const fn cost_estimate(&self) -> &AccessPlanCostEstimate {
        &self.costed.cost_estimate
    }
    pub const fn planned_counter_envelope(&self) -> PlannedCounterEnvelope {
        self.costed.basis.planned_counter_envelope
    }
    pub const fn budget_receipt(&self) -> PreExecutionBudgetAdmissionReceipt {
        self.budget_receipt
    }
    pub const fn selection_rule(&self) -> DeterministicSelectionRule {
        self.costed.basis.selection_rule
    }
    pub const fn primary_candidate(&self) -> &SelectionCandidateAudit {
        &self.costed.basis.primary_candidate
    }
    pub const fn secondary_candidate(&self) -> &SelectionCandidateAudit {
        &self.costed.basis.secondary_candidate
    }
}

pub(super) fn admit_selected_plan_budget(
    basis: PlanSelectionBasis,
    envelope: PreExecutionBudgetEnvelope,
) -> Result<SelectedAccessPlanBasis, super::AccessPlanSelectionDenied> {
    let costed =
        CostedAccessPlan::derive(basis).map_err(super::AccessPlanSelectionDenied::CostDenied)?;
    let request = costed.budget_request();
    let receipt = match pre_execution_budget_admission().admit(request, envelope) {
        PreExecutionBudgetAdmissionOutcome::Admitted(receipt) => receipt,
        PreExecutionBudgetAdmissionOutcome::Denied(denial) => {
            return Err(super::AccessPlanSelectionDenied::BudgetDenied(denial));
        }
    };
    debug_assert_eq!(receipt.request(), request);
    Ok(SelectedAccessPlanBasis::from_budget_admission(
        costed, receipt,
    ))
}

macro_rules! define_selected_operation {
    ($name:ident, $grant:ty) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            basis: SelectedAccessPlanBasis,
        }

        impl $name {
            pub(super) const fn from_decision(
                basis: SelectedAccessPlanBasis,
                _grant: $grant,
            ) -> Self {
                Self { basis }
            }

            pub const fn lifecycle(&self) -> ArtifactFamilyLifecycleAdmission {
                self.basis.lifecycle()
            }
            pub const fn admitted_family(&self) -> AdmittedPhysicalArtifactFamily {
                self.basis.admitted_family()
            }
            pub const fn key_domain(&self) -> PhysicalKeyDomainWitness {
                self.basis.key_domain()
            }
            pub const fn admitted_key_domain(&self) -> AdmittedPhysicalKeyDomain {
                self.basis.admitted_key_domain()
            }
            pub const fn request_identity(&self) -> AdmittedPhysicalAccessIdentity {
                self.basis.request_identity()
            }
            pub const fn materialization(&self) -> Option<&AdmittedLayoutMaterialization> {
                self.basis.materialization()
            }
            pub const fn admitted_strategy(&self) -> Option<AdmittedLayoutStrategy> {
                self.basis.admitted_strategy()
            }
            pub const fn strategy_admission(&self) -> Option<&LayoutStrategyRegistrySnapshot> {
                self.basis.strategy_admission()
            }
            pub const fn selected_family(&self) -> LayoutStrategyFamily {
                self.basis.selected_family()
            }
            pub const fn intent(&self) -> AdmittedAccessIntent {
                self.basis.intent()
            }
            pub const fn fingerprint(&self) -> &AccessPlanIdentity {
                self.basis.fingerprint()
            }
            pub const fn cost_estimate(&self) -> &AccessPlanCostEstimate {
                self.basis.cost_estimate()
            }
            pub const fn planned_counter_envelope(&self) -> PlannedCounterEnvelope {
                self.basis.planned_counter_envelope()
            }
            pub const fn budget_receipt(&self) -> PreExecutionBudgetAdmissionReceipt {
                self.basis.budget_receipt()
            }
            pub const fn selection_rule(&self) -> DeterministicSelectionRule {
                self.basis.selection_rule()
            }
            pub const fn primary_candidate(&self) -> &SelectionCandidateAudit {
                self.basis.primary_candidate()
            }
            pub const fn secondary_candidate(&self) -> &SelectionCandidateAudit {
                self.basis.secondary_candidate()
            }
        }
    };
}

define_selected_operation!(SelectedDegradedExactScan, DegradedScanSelectionGrant);
define_selected_operation!(SelectedBTreeLookup, BTreeLookupSelectionGrant);
define_selected_operation!(SelectedBTreeReplayRecovery, BTreeReplaySelectionGrant);
define_selected_operation!(SelectedLsmLookup, LsmLookupSelectionGrant);
define_selected_operation!(SelectedLsmRunPublication, LsmPublicationSelectionGrant);
define_selected_operation!(SelectedLsmReplayRecovery, LsmReplaySelectionGrant);
define_selected_operation!(SelectedLsmCompaction, LsmCompactionSelectionGrant);

impl SelectedBTreeLookup {
    pub fn operation(&self) -> super::BTreeLookupOperation {
        match self.basis.selected_operation() {
            Some(EligibleStrategyOperation::BTreeLookup(operation)) => operation,
            _ => unreachable!("B-tree lookup authority retains its classified operation"),
        }
    }
}

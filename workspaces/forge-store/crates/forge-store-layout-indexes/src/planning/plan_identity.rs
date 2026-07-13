use crate::access::budget::PlannedCounterEnvelope;
use crate::access::shape::{
    AccessAuthorityPosture, AccessLaneClassification, AccessShapeDetail, AccessStaleDisposition,
    ExpectedCounterClass,
};
use crate::artifact_family::AdmittedPhysicalArtifactFamily;
use crate::catalog::ArtifactFamilyLifecycleAdmission;
use crate::keyspace::{AdmittedPhysicalAccessIdentity, AdmittedPhysicalKeyDomain};
use crate::maintenance::PhysicalMutationShape;
use crate::materialization::AdmittedLayoutMaterialization;
use crate::strategy::registry::LayoutStrategyRegistrySnapshot;
use crate::strategy::{AdmittedLayoutStrategy, LayoutStrategyFamily};
use forge_store_budgets::{PreExecutionBudgetEnvelope, PreExecutionBudgetRequest};

use super::{AccessPlanCostEstimate, DeterministicSelectionRule};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessPlanIdentity {
    admitted_family: AdmittedPhysicalArtifactFamily,
    strategy_family: LayoutStrategyFamily,
    detail: AccessShapeDetail,
    lane: AccessLaneClassification,
    authority_posture: AccessAuthorityPosture,
    stale_disposition: AccessStaleDisposition,
    key_domain: AdmittedPhysicalKeyDomain,
    request_identity: AdmittedPhysicalAccessIdentity,
    materialization: Option<AdmittedLayoutMaterialization>,
    strategy_admission: Option<LayoutStrategyRegistrySnapshot>,
    expected_counters: ExpectedCounterClass,
    mutation_shape: Option<PhysicalMutationShape>,
    budget_rows: Option<u64>,
    planned_counter_envelope: PlannedCounterEnvelope,
    selection_rule: DeterministicSelectionRule,
    cost_estimate: AccessPlanCostEstimate,
    budget_request: PreExecutionBudgetRequest,
    budget_envelope: PreExecutionBudgetEnvelope,
}

impl AccessPlanIdentity {
    pub(crate) const fn new(
        admitted_family: AdmittedPhysicalArtifactFamily,
        strategy_family: LayoutStrategyFamily,
        detail: AccessShapeDetail,
        lane: AccessLaneClassification,
        authority_posture: AccessAuthorityPosture,
        stale_disposition: AccessStaleDisposition,
        key_domain: AdmittedPhysicalKeyDomain,
        request_identity: AdmittedPhysicalAccessIdentity,
        materialization: Option<AdmittedLayoutMaterialization>,
        strategy_admission: Option<LayoutStrategyRegistrySnapshot>,
        expected_counters: ExpectedCounterClass,
        mutation_shape: Option<PhysicalMutationShape>,
        budget_rows: Option<u64>,
        planned_counter_envelope: PlannedCounterEnvelope,
        selection_rule: DeterministicSelectionRule,
        cost_estimate: AccessPlanCostEstimate,
        budget_request: PreExecutionBudgetRequest,
        budget_envelope: PreExecutionBudgetEnvelope,
    ) -> Self {
        Self {
            admitted_family,
            strategy_family,
            detail,
            lane,
            authority_posture,
            stale_disposition,
            key_domain,
            request_identity,
            materialization,
            strategy_admission,
            expected_counters,
            mutation_shape,
            budget_rows,
            planned_counter_envelope,
            selection_rule,
            cost_estimate,
            budget_request,
            budget_envelope,
        }
    }

    pub const fn lifecycle(&self) -> ArtifactFamilyLifecycleAdmission {
        self.admitted_family.lifecycle()
    }

    pub const fn admitted_family(&self) -> AdmittedPhysicalArtifactFamily {
        self.admitted_family
    }

    pub const fn family(&self) -> LayoutStrategyFamily {
        self.strategy_family
    }

    pub const fn detail(&self) -> AccessShapeDetail {
        self.detail
    }

    pub const fn shape(&self) -> crate::access::shape::AccessShape {
        self.detail.shape()
    }

    pub const fn lane(&self) -> AccessLaneClassification {
        self.lane
    }

    pub const fn authority_posture(&self) -> AccessAuthorityPosture {
        self.authority_posture
    }

    pub const fn stale_disposition(&self) -> AccessStaleDisposition {
        self.stale_disposition
    }

    pub const fn key_domain(&self) -> crate::PhysicalKeyDomainWitness {
        self.key_domain.witness()
    }

    pub const fn admitted_key_domain(&self) -> AdmittedPhysicalKeyDomain {
        self.key_domain
    }

    pub const fn request_identity(&self) -> AdmittedPhysicalAccessIdentity {
        self.request_identity
    }

    pub const fn materialization(&self) -> Option<&AdmittedLayoutMaterialization> {
        self.materialization.as_ref()
    }

    pub const fn admitted_strategy(&self) -> Option<AdmittedLayoutStrategy> {
        match &self.strategy_admission {
            Some(admission) => Some(admission.admitted_strategy()),
            None => None,
        }
    }

    pub const fn strategy_admission(&self) -> Option<&LayoutStrategyRegistrySnapshot> {
        self.strategy_admission.as_ref()
    }

    pub const fn planned_counter_envelope(&self) -> PlannedCounterEnvelope {
        self.planned_counter_envelope
    }

    pub const fn selection_rule(&self) -> DeterministicSelectionRule {
        self.selection_rule
    }

    pub const fn cost_estimate(&self) -> &AccessPlanCostEstimate {
        &self.cost_estimate
    }

    pub const fn budget_request(&self) -> PreExecutionBudgetRequest {
        self.budget_request
    }

    pub const fn budget_envelope(&self) -> PreExecutionBudgetEnvelope {
        self.budget_envelope
    }
}

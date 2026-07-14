use crate::access::budget::PlannedCounterEnvelope;
use crate::access::shape::{AccessAuthorityPosture, AccessLaneClassification, AccessShapeDetail};
use crate::access::AdmittedAccessIntent;
use crate::artifact_family::AdmittedPhysicalArtifactFamily;
use crate::catalog::ArtifactFamilyLifecycleAdmission;
use crate::keyspace::{AdmittedPhysicalAccessIdentity, AdmittedPhysicalKeyDomain};
use crate::materialization::AdmittedLayoutMaterialization;
use crate::strategy::registry::LayoutStrategyRegistrySnapshot;
use crate::strategy::{AdmittedLayoutStrategy, LayoutStrategyFamily};
use forge_store_budgets::{PreExecutionBudgetEnvelope, PreExecutionBudgetRequest};
use std::sync::Arc;

use super::{AccessPlanCostEstimate, DeterministicSelectionRule, SelectionCandidateAudit};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessPlanIdentity {
    basis: Arc<AccessPlanIdentityBasis>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AccessPlanIdentityBasis {
    pub(super) admitted_family: AdmittedPhysicalArtifactFamily,
    pub(super) strategy_family: LayoutStrategyFamily,
    pub(super) intent: AdmittedAccessIntent,
    pub(super) key_domain: AdmittedPhysicalKeyDomain,
    pub(super) request_identity: AdmittedPhysicalAccessIdentity,
    pub(super) materialization: Option<AdmittedLayoutMaterialization>,
    pub(super) strategy_admission: Option<LayoutStrategyRegistrySnapshot>,
    pub(super) planned_counter_envelope: PlannedCounterEnvelope,
    pub(super) selection_rule: DeterministicSelectionRule,
    pub(super) primary_candidate: SelectionCandidateAudit,
    pub(super) secondary_candidate: SelectionCandidateAudit,
    pub(super) cost_estimate: AccessPlanCostEstimate,
    pub(super) budget_request: PreExecutionBudgetRequest,
    pub(super) budget_envelope: PreExecutionBudgetEnvelope,
}

impl AccessPlanIdentity {
    pub(super) fn new(basis: AccessPlanIdentityBasis) -> Self {
        Self {
            basis: Arc::new(basis),
        }
    }

    pub fn lifecycle(&self) -> ArtifactFamilyLifecycleAdmission {
        self.basis.admitted_family.lifecycle()
    }

    pub fn admitted_family(&self) -> AdmittedPhysicalArtifactFamily {
        self.basis.admitted_family
    }

    pub fn family(&self) -> LayoutStrategyFamily {
        self.basis.strategy_family
    }

    pub fn detail(&self) -> AccessShapeDetail {
        self.basis.intent.detail()
    }

    pub fn shape(&self) -> crate::access::shape::AccessShape {
        self.basis.intent.detail().shape()
    }

    pub fn lane(&self) -> AccessLaneClassification {
        self.basis.intent.lane()
    }

    pub fn authority_posture(&self) -> AccessAuthorityPosture {
        self.basis.intent.authority_posture()
    }

    pub fn intent(&self) -> AdmittedAccessIntent {
        self.basis.intent
    }

    pub fn key_domain(&self) -> crate::PhysicalKeyDomainWitness {
        self.basis.key_domain.witness()
    }

    pub fn admitted_key_domain(&self) -> AdmittedPhysicalKeyDomain {
        self.basis.key_domain
    }

    pub fn request_identity(&self) -> AdmittedPhysicalAccessIdentity {
        self.basis.request_identity
    }

    pub fn materialization(&self) -> Option<&AdmittedLayoutMaterialization> {
        self.basis.materialization.as_ref()
    }

    pub fn admitted_strategy(&self) -> Option<&AdmittedLayoutStrategy> {
        self.basis
            .strategy_admission
            .as_ref()
            .map(LayoutStrategyRegistrySnapshot::admitted_strategy)
    }

    pub fn strategy_admission(&self) -> Option<&LayoutStrategyRegistrySnapshot> {
        self.basis.strategy_admission.as_ref()
    }

    pub fn planned_counter_envelope(&self) -> PlannedCounterEnvelope {
        self.basis.planned_counter_envelope
    }

    pub fn selection_rule(&self) -> DeterministicSelectionRule {
        self.basis.selection_rule
    }

    pub fn primary_candidate(&self) -> &SelectionCandidateAudit {
        &self.basis.primary_candidate
    }

    pub fn secondary_candidate(&self) -> &SelectionCandidateAudit {
        &self.basis.secondary_candidate
    }

    pub fn cost_estimate(&self) -> &AccessPlanCostEstimate {
        &self.basis.cost_estimate
    }

    pub fn budget_request(&self) -> PreExecutionBudgetRequest {
        self.basis.budget_request
    }

    pub fn budget_envelope(&self) -> PreExecutionBudgetEnvelope {
        self.basis.budget_envelope
    }
}

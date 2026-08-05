use std::sync::Arc;

use worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority;

use super::declared_closure::WorthQueryProviderPlanDeclarations;
use crate::domain_computation::operation_binding::WorthQueryExecutionBoundOperationAuthority;

pub(crate) struct WorthQueryProviderPlanExecutionBinding<'a> {
    pub(crate) managed_run_identity: &'a str,
    pub(crate) execution_basis_identity: &'a str,
    pub(crate) admitted_session_identity: &'a str,
    pub(crate) resource_attempt_identity: &'a str,
    pub(crate) graph: &'a WorthQueryInstalledGraphParticipationAuthority,
    pub(crate) snapshot_identity: &'a str,
    pub(crate) resource_envelope_identity: &'a str,
    pub(crate) provider_identity: &'a str,
    pub(crate) provider_generation: u64,
}

pub(crate) struct WorthQueryProviderPlanContractMaterial<'a> {
    pub(crate) declarations: &'a WorthQueryProviderPlanDeclarations,
    pub(crate) artifact_closure: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryProviderOperationScope {
    Direct,
    WorkflowStage(Arc<str>),
}

impl WorthQueryProviderOperationScope {
    pub fn workflow_stage(identity: impl Into<Arc<str>>) -> Self {
        Self::WorkflowStage(identity.into())
    }

    pub fn stage_identity(&self) -> Option<&str> {
        match self {
            Self::Direct => None,
            Self::WorkflowStage(identity) => Some(identity),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProviderExecutionPlanContract {
    identity: Arc<str>,
    operation_identity: Arc<str>,
    binding_identity: Arc<str>,
    scope: WorthQueryProviderOperationScope,
    provider_role: Arc<str>,
    provider_identity: Arc<str>,
    provider_generation: u64,
    graph_authority_identity: Arc<str>,
    managed_run_identity: Arc<str>,
    execution_basis_identity: Arc<str>,
    admitted_session_identity: Arc<str>,
    resource_attempt_identity: Arc<str>,
    basis_identity: Arc<str>,
    snapshot_identity: Arc<str>,
    resource_envelope_identity: Arc<str>,
    read_closure: Arc<[String]>,
    touch_closure: Arc<[String]>,
    effect_closure: Arc<[String]>,
    invariant_closure: Arc<[String]>,
    artifact_closure: Arc<[String]>,
    decision_fact_families: Arc<[worth_query_installation::facade::WorthQueryDecisionFactFamily]>,
    invariant_requirements:
        Arc<[worth_query_installation::facade::WorthQueryInstalledInvariantExecutionRequirement]>,
    transaction_posture: Arc<str>,
    reconciliation_posture: Arc<str>,
    graph_work_session: Option<u64>,
    graph_work_managed_run: Option<u64>,
}

impl WorthQueryProviderExecutionPlanContract {
    pub(crate) fn bind(
        operation: &WorthQueryExecutionBoundOperationAuthority,
        stage_identity: Option<&str>,
        execution: &WorthQueryProviderPlanExecutionBinding<'_>,
        material: WorthQueryProviderPlanContractMaterial<'_>,
    ) -> Option<Self> {
        let declared_closure = material
            .declarations
            .closure(stage_identity, execution.graph.role())?;
        let invariant_requirements = material
            .declarations
            .invariant_requirements_for(stage_identity, execution.graph.role());
        let mut closure = declared_closure.clone();
        closure.invariant = invariant_requirements
            .iter()
            .map(|requirement| requirement.slot().to_owned())
            .collect();
        let scope = stage_identity.map_or(WorthQueryProviderOperationScope::Direct, |identity| {
            WorthQueryProviderOperationScope::workflow_stage(identity.to_owned())
        });
        let transaction_posture = operation.commit_posture().as_str();
        let reconciliation_posture = material.declarations.reconciliation_posture();
        let graph_work = operation.graph_work_affinity();
        Some(Self {
            identity: execution.resource_attempt_identity.into(),
            operation_identity: operation.operation_identity().into(),
            binding_identity: operation.binding_identity().into(),
            scope,
            provider_role: execution.graph.role().into(),
            provider_identity: execution.provider_identity.into(),
            provider_generation: execution.provider_generation,
            graph_authority_identity: execution.graph.authority_identity().into(),
            managed_run_identity: execution.managed_run_identity.into(),
            execution_basis_identity: execution.execution_basis_identity.into(),
            admitted_session_identity: execution.admitted_session_identity.into(),
            resource_attempt_identity: execution.resource_attempt_identity.into(),
            basis_identity: operation.basis_identity().into(),
            snapshot_identity: execution.snapshot_identity.into(),
            resource_envelope_identity: execution.resource_envelope_identity.into(),
            read_closure: closure.read.into(),
            touch_closure: closure.touch.into(),
            effect_closure: closure.effect.into(),
            invariant_closure: closure.invariant.into(),
            artifact_closure: material.artifact_closure.into(),
            decision_fact_families: material.declarations.decision_fact_families().into(),
            invariant_requirements: invariant_requirements.into(),
            transaction_posture: transaction_posture.into(),
            reconciliation_posture: reconciliation_posture.into(),
            graph_work_session: graph_work.map(|affinity| affinity.session.as_u64()),
            graph_work_managed_run: graph_work.map(|affinity| affinity.managed_run.as_u64()),
        })
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn operation_identity(&self) -> &str {
        &self.operation_identity
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn scope(&self) -> &WorthQueryProviderOperationScope {
        &self.scope
    }

    pub fn provider_role(&self) -> &str {
        &self.provider_role
    }

    pub fn provider_identity(&self) -> &str {
        &self.provider_identity
    }

    pub fn provider_generation(&self) -> u64 {
        self.provider_generation
    }

    pub fn graph_authority_identity(&self) -> &str {
        &self.graph_authority_identity
    }

    pub fn managed_run_identity(&self) -> &str {
        &self.managed_run_identity
    }

    pub fn execution_basis_identity(&self) -> &str {
        &self.execution_basis_identity
    }

    pub fn admitted_session_identity(&self) -> &str {
        &self.admitted_session_identity
    }

    pub fn resource_attempt_identity(&self) -> &str {
        &self.resource_attempt_identity
    }

    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn snapshot_identity(&self) -> &str {
        &self.snapshot_identity
    }

    pub fn resource_envelope_identity(&self) -> &str {
        &self.resource_envelope_identity
    }

    pub fn read_closure(&self) -> &[String] {
        &self.read_closure
    }

    pub fn touch_closure(&self) -> &[String] {
        &self.touch_closure
    }

    pub fn effect_closure(&self) -> &[String] {
        &self.effect_closure
    }

    pub fn invariant_closure(&self) -> &[String] {
        &self.invariant_closure
    }

    pub fn artifact_closure(&self) -> &[String] {
        &self.artifact_closure
    }

    pub fn decision_fact_families(
        &self,
    ) -> &[worth_query_installation::facade::WorthQueryDecisionFactFamily] {
        &self.decision_fact_families
    }

    pub fn invariant_requirements(
        &self,
    ) -> &[worth_query_installation::facade::WorthQueryInstalledInvariantExecutionRequirement] {
        &self.invariant_requirements
    }

    pub fn transaction_posture(&self) -> &str {
        &self.transaction_posture
    }

    pub fn reconciliation_posture(&self) -> &str {
        &self.reconciliation_posture
    }

    pub const fn graph_work_session_identity(&self) -> Option<u64> {
        self.graph_work_session
    }

    pub const fn graph_work_managed_run_identity(&self) -> Option<u64> {
        self.graph_work_managed_run
    }

    pub(super) fn closure_width(&self) -> usize {
        self.read_closure.len()
            + self.touch_closure.len()
            + self.effect_closure.len()
            + self.invariant_closure.len()
            + self.artifact_closure.len()
            + self.decision_fact_families.len()
            + self.invariant_requirements.len()
    }
}

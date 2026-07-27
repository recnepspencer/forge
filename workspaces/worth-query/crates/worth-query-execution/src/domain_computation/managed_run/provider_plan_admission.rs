use std::sync::Arc;

use worth_runtime_bridge::facade::BridgeBoundExecutionBasis;

use super::{WorthQueryRunningDirectRun, WorthQueryRunningWorkflowRun};
use crate::domain_computation::{
    WorthQueryAdmittedProviderExecutionPlan, WorthQueryExecutionBoundOperationAuthority,
    WorthQueryExecutionProviderSession, WorthQueryExecutionResourceAttemptEvidence,
    WorthQueryProviderSessionFailure,
};

impl WorthQueryRunningDirectRun {
    pub fn admit_provider_execution_plan(
        &mut self,
        graph_authority: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    ) -> Result<WorthQueryAdmittedProviderExecutionPlan<'_>, WorthQueryProviderSessionFailure> {
        WorthQueryAdmittedProviderExecutionPlan::direct(self, graph_authority)
    }

    pub(crate) fn provider_plan_operation(&self) -> &WorthQueryExecutionBoundOperationAuthority {
        self.resource_attempt.binding_authority()
    }

    pub(crate) fn provider_plan_session(&self) -> &WorthQueryExecutionProviderSession {
        self.resource_attempt.provider_session()
    }

    pub(crate) fn provider_plan_resources(
        &self,
    ) -> (
        &worth_query_admission::facade::resource_admission::WorthQueryAdmittedExecutionResourcePlan,
        &WorthQueryExecutionResourceAttemptEvidence,
    ) {
        (
            self.resource_attempt.resources(),
            self.resource_attempt.evidence(),
        )
    }

    pub(crate) fn provider_plan_bridge_basis(&self) -> &BridgeBoundExecutionBasis {
        &self.bridge_basis
    }
}

impl WorthQueryRunningWorkflowRun {
    pub fn admit_stage_provider_execution_plan(
        &mut self,
        stage_identity: &str,
        graph_authority: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    ) -> Result<WorthQueryAdmittedProviderExecutionPlan<'_>, WorthQueryProviderSessionFailure> {
        WorthQueryAdmittedProviderExecutionPlan::workflow_stage(
            self,
            stage_identity,
            graph_authority,
        )
    }

    pub(crate) fn provider_plan_operation(&self) -> &WorthQueryExecutionBoundOperationAuthority {
        self.resource_attempt.binding_authority()
    }

    pub(crate) fn provider_plan_session(&self) -> &WorthQueryExecutionProviderSession {
        self.resource_attempt.provider_session()
    }

    pub(crate) fn provider_plan_stage_resources(
        &self,
        stage_identity: &str,
    ) -> Option<(
        Arc<
            worth_query_admission::facade::resource_admission::WorthQueryAdmittedExecutionResourcePlan,
        >,
        WorthQueryExecutionResourceAttemptEvidence,
    )>{
        self.resource_attempt
            .stage_resources_and_evidence(stage_identity)
    }

    pub(crate) fn provider_plan_bridge_basis(&self) -> &BridgeBoundExecutionBasis {
        &self.bridge_basis
    }
}

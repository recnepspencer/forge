use std::sync::Arc;

use worth_query_admission::facade::resource_admission::{
    WorthQueryAdmittedExecutionResourcePlan, WorthQueryAdmittedWorkflowResourcePlan,
};

use super::{WorthQueryExecutionProviderSession, WorthQueryExecutionResourceAttemptEvidence};

pub struct WorthQueryWorkflowExecutionResourceAttempt {
    resources: WorthQueryAdmittedWorkflowResourcePlan,
    provider_session: WorthQueryExecutionProviderSession,
    evidence: WorthQueryExecutionResourceAttemptEvidence,
}

impl WorthQueryWorkflowExecutionResourceAttempt {
    pub fn start(mut resources: WorthQueryAdmittedWorkflowResourcePlan) -> Self {
        let provider_session = WorthQueryExecutionProviderSession::mint(resources.identity());
        resources.record_provider_session_mint();
        let evidence = WorthQueryExecutionResourceAttemptEvidence::capture(
            resources.operation(),
            &provider_session,
        );
        Self {
            resources,
            provider_session,
            evidence,
        }
    }

    pub fn resources(&self) -> &WorthQueryAdmittedWorkflowResourcePlan {
        &self.resources
    }

    pub fn operation_resources(&self) -> &WorthQueryAdmittedExecutionResourcePlan {
        self.resources.operation()
    }

    pub fn provider_session(&self) -> &WorthQueryExecutionProviderSession {
        &self.provider_session
    }

    pub fn evidence(&self) -> &WorthQueryExecutionResourceAttemptEvidence {
        &self.evidence
    }

    pub fn stage_resources_and_evidence(
        &self,
        stage_identity: &str,
    ) -> Option<(
        Arc<WorthQueryAdmittedExecutionResourcePlan>,
        WorthQueryExecutionResourceAttemptEvidence,
    )> {
        let resources = self.resources.shared_stage(stage_identity)?;
        let evidence =
            WorthQueryExecutionResourceAttemptEvidence::capture(&resources, &self.provider_session);
        Some((resources, evidence))
    }
}

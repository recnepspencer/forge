use worth_query_admission::facade::resource_admission::WorthQueryAdmittedExecutionResourcePlan;

use super::{WorthQueryExecutionProviderSession, WorthQueryExecutionResourceAttemptEvidence};

pub struct WorthQueryDirectExecutionResourceAttempt {
    resources: WorthQueryAdmittedExecutionResourcePlan,
    provider_session: WorthQueryExecutionProviderSession,
    evidence: WorthQueryExecutionResourceAttemptEvidence,
}

impl WorthQueryDirectExecutionResourceAttempt {
    pub fn start(mut resources: WorthQueryAdmittedExecutionResourcePlan) -> Self {
        let provider_session = WorthQueryExecutionProviderSession::mint(resources.identity());
        resources.record_provider_session_mint();
        let evidence =
            WorthQueryExecutionResourceAttemptEvidence::capture(&resources, &provider_session);
        Self {
            resources,
            provider_session,
            evidence,
        }
    }

    pub fn resources(&self) -> &WorthQueryAdmittedExecutionResourcePlan {
        &self.resources
    }

    pub fn provider_session(&self) -> &WorthQueryExecutionProviderSession {
        &self.provider_session
    }

    pub fn evidence(&self) -> &WorthQueryExecutionResourceAttemptEvidence {
        &self.evidence
    }
}

use worth_query_admission::facade::resource_admission::WorthQueryAdmittedExecutionResourcePlan;
use worth_query_admission::integration::WorthQueryCapacityReservedExecutionResourcePlan;

use super::{WorthQueryExecutionProviderSession, WorthQueryExecutionResourceAttemptEvidence};
pub struct WorthQueryDirectExecutionResourceAttempt {
    reserved: WorthQueryCapacityReservedExecutionResourcePlan,
    provider_session: WorthQueryExecutionProviderSession,
    evidence: WorthQueryExecutionResourceAttemptEvidence,
}

impl WorthQueryDirectExecutionResourceAttempt {
    pub(crate) fn start(
        mut reserved: WorthQueryCapacityReservedExecutionResourcePlan,
        binding_authority: &crate::domain_computation::operation_binding::WorthQueryExecutionBoundOperationAuthority,
    ) -> Self {
        let provider_session = WorthQueryExecutionProviderSession::mint(
            reserved.resources().identity(),
            binding_authority,
        );
        reserved.resources_mut().record_provider_session_mint();
        let evidence = WorthQueryExecutionResourceAttemptEvidence::capture(
            reserved.resources(),
            &provider_session,
        );
        Self {
            reserved,
            provider_session,
            evidence,
        }
    }

    pub fn resources(&self) -> &WorthQueryAdmittedExecutionResourcePlan {
        self.reserved.resources()
    }

    pub fn provider_session(&self) -> &WorthQueryExecutionProviderSession {
        &self.provider_session
    }

    pub fn evidence(&self) -> &WorthQueryExecutionResourceAttemptEvidence {
        &self.evidence
    }
}

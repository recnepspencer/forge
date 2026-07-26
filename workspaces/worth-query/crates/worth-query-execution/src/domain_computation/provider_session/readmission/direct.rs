use super::super::{
    WorthQueryDirectExecutionResourceAttempt, WorthQueryExecutionAttemptIdentity,
    WorthQueryExecutionProviderSession, WorthQueryExecutionResourceAttemptEvidence,
};
use crate::domain_computation::provider_session::graph_provider::{
    WorthQueryGraphProviderCall, WorthQueryGraphProviderCallReadmissionPlan,
};

pub(crate) struct WorthQueryDirectResourceReadmissionPending {
    yielded_attempt: WorthQueryDirectExecutionResourceAttempt,
    fresh_attempt_identity: WorthQueryExecutionAttemptIdentity,
    fresh_provider_session: WorthQueryExecutionProviderSession,
    fresh_evidence: WorthQueryExecutionResourceAttemptEvidence,
}

impl WorthQueryDirectResourceReadmissionPending {
    pub(crate) fn begin(
        yielded_attempt: WorthQueryDirectExecutionResourceAttempt,
        call: WorthQueryGraphProviderCallReadmissionPlan,
    ) -> (Self, WorthQueryGraphProviderCall) {
        let fresh_attempt_identity = WorthQueryExecutionAttemptIdentity::readmission(
            "direct",
            yielded_attempt.resources().identity(),
            yielded_attempt.attempt_identity().as_str(),
        );
        let fresh_provider_session = WorthQueryExecutionProviderSession::mint(
            &fresh_attempt_identity,
            yielded_attempt.binding_authority(),
        );
        let fresh_evidence = WorthQueryExecutionResourceAttemptEvidence::capture(
            yielded_attempt.resources(),
            &fresh_provider_session,
        );
        let fresh_call = call.mint(&fresh_provider_session, &fresh_evidence);
        (
            Self {
                yielded_attempt,
                fresh_attempt_identity,
                fresh_provider_session,
                fresh_evidence,
            },
            fresh_call,
        )
    }

    pub(crate) fn attempt_identity(&self) -> &WorthQueryExecutionAttemptIdentity {
        &self.fresh_attempt_identity
    }

    pub(crate) fn abort(self) -> WorthQueryDirectExecutionResourceAttempt {
        self.yielded_attempt
    }

    pub(crate) fn commit(self) -> WorthQueryDirectExecutionResourceAttempt {
        let Self {
            yielded_attempt,
            fresh_attempt_identity,
            fresh_provider_session,
            fresh_evidence,
        } = self;
        let WorthQueryDirectExecutionResourceAttempt {
            mut reserved,
            attempt_identity: _,
            provider_session,
            evidence: _,
        } = yielded_attempt;
        drop(provider_session);
        reserved.resources_mut().record_provider_session_mint();
        WorthQueryDirectExecutionResourceAttempt {
            reserved,
            attempt_identity: fresh_attempt_identity,
            provider_session: fresh_provider_session,
            evidence: fresh_evidence,
        }
    }
}

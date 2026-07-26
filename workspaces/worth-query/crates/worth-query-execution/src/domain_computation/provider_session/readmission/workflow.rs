use super::super::{
    WorthQueryExecutionAttemptIdentity, WorthQueryExecutionProviderSession,
    WorthQueryExecutionResourceAttemptEvidence, WorthQueryWorkflowExecutionResourceAttempt,
};
use std::sync::Arc;
use worth_query_admission::facade::resource_admission::WorthQueryAdmittedExecutionResourcePlan;

use crate::domain_computation::provider_session::graph_provider::{
    WorthQueryGraphProviderCall, WorthQueryGraphProviderCallReadmissionPlan,
};

pub(crate) struct WorthQueryWorkflowResourceReadmissionPending {
    yielded_attempt: WorthQueryWorkflowExecutionResourceAttempt,
    fresh_attempt_identity: WorthQueryExecutionAttemptIdentity,
    fresh_provider_session: WorthQueryExecutionProviderSession,
    fresh_evidence: WorthQueryExecutionResourceAttemptEvidence,
}

impl WorthQueryWorkflowResourceReadmissionPending {
    pub(crate) fn begin(
        yielded_attempt: WorthQueryWorkflowExecutionResourceAttempt,
        stage_resources: Arc<WorthQueryAdmittedExecutionResourcePlan>,
        call: WorthQueryGraphProviderCallReadmissionPlan,
    ) -> (Self, WorthQueryGraphProviderCall) {
        let fresh_attempt_identity = WorthQueryExecutionAttemptIdentity::readmission(
            "workflow",
            yielded_attempt.resources().identity(),
            yielded_attempt.attempt_identity().as_str(),
        );
        let fresh_provider_session = WorthQueryExecutionProviderSession::mint(
            &fresh_attempt_identity,
            yielded_attempt.binding_authority(),
        );
        let fresh_evidence = WorthQueryExecutionResourceAttemptEvidence::capture(
            yielded_attempt.operation_resources(),
            &fresh_provider_session,
        );
        let stage_evidence = WorthQueryExecutionResourceAttemptEvidence::capture(
            &stage_resources,
            &fresh_provider_session,
        );
        let fresh_call = call.mint(&fresh_provider_session, &stage_evidence);
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

    pub(crate) fn abort(self) -> WorthQueryWorkflowExecutionResourceAttempt {
        self.yielded_attempt
    }

    pub(crate) fn commit(self) -> WorthQueryWorkflowExecutionResourceAttempt {
        let Self {
            yielded_attempt,
            fresh_attempt_identity,
            fresh_provider_session,
            fresh_evidence,
        } = self;
        let WorthQueryWorkflowExecutionResourceAttempt {
            mut reserved,
            attempt_identity: _,
            provider_session,
            evidence: _,
            artifact_run,
        } = yielded_attempt;
        drop(provider_session);
        reserved.resources_mut().record_provider_session_mint();
        WorthQueryWorkflowExecutionResourceAttempt {
            reserved,
            attempt_identity: fresh_attempt_identity,
            provider_session: fresh_provider_session,
            evidence: fresh_evidence,
            artifact_run,
        }
    }
}

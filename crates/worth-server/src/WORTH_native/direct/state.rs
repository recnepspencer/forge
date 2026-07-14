use worth_query::facade::runtime::{
    WorthQueryAuthorityLane, WorthQueryRuntimeAsyncResultState, WorthQueryRuntimeStateSnapshot,
};

use crate::{
    WorthServerDirectContextArtifact, WorthServerQuerySupportPosture, WorthServerResponseEnvelope,
};

#[derive(Debug)]
pub struct WorthServerDirectState {
    plan_proof: crate::WorthServerOperationPlanProof,
    support_posture: WorthServerQuerySupportPosture,
    workspace_name: String,
    handoff_digest: String,
    direct_context: WorthServerDirectContextArtifact,
    runtime_state: WorthQueryRuntimeStateSnapshot,
    response_envelope: WorthServerResponseEnvelope,
    canonical_digest: String,
}

impl WorthServerDirectState {
    pub(crate) fn new(
        plan_proof: crate::WorthServerOperationPlanProof,
        support_posture: WorthServerQuerySupportPosture,
        workspace_name: String,
        handoff_digest: String,
        direct_context: WorthServerDirectContextArtifact,
        runtime_state: WorthQueryRuntimeStateSnapshot,
        response_envelope: WorthServerResponseEnvelope,
    ) -> Self {
        let canonical_digest = format!(
            "worth-server-direct-state-v1:{}:{}",
            handoff_digest,
            runtime_state
                .state_digest()
                .terminal_projection_for_reporting(),
        );
        Self {
            plan_proof,
            support_posture,
            workspace_name,
            handoff_digest,
            direct_context,
            runtime_state,
            response_envelope,
            canonical_digest,
        }
    }

    pub fn plan_proof(&self) -> &crate::WorthServerOperationPlanProof {
        &self.plan_proof
    }

    pub fn support_posture(&self) -> &WorthServerQuerySupportPosture {
        &self.support_posture
    }

    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }

    pub fn direct_context(&self) -> &WorthServerDirectContextArtifact {
        &self.direct_context
    }

    pub fn runtime_state(&self) -> &WorthQueryRuntimeStateSnapshot {
        &self.runtime_state
    }

    pub fn async_result_state(&self) -> Option<WorthServerDirectAsyncResultState> {
        self.runtime_state
            .async_result_state()
            .cloned()
            .map(WorthServerDirectAsyncResultState::new)
    }

    pub fn temporal_state(&self) -> Option<WorthServerDirectTemporalState> {
        (self.runtime_state.authority_lane() == WorthQueryAuthorityLane::TemporalExecutionState)
            .then(|| WorthServerDirectTemporalState::new(self.runtime_state.clone()))
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn response_envelope(&self) -> &WorthServerResponseEnvelope {
        &self.response_envelope
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDirectAsyncResultState {
    inner: WorthQueryRuntimeAsyncResultState,
}

impl WorthServerDirectAsyncResultState {
    fn new(inner: WorthQueryRuntimeAsyncResultState) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &WorthQueryRuntimeAsyncResultState {
        &self.inner
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDirectTemporalState {
    inner: WorthQueryRuntimeStateSnapshot,
}

impl WorthServerDirectTemporalState {
    fn new(inner: WorthQueryRuntimeStateSnapshot) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &WorthQueryRuntimeStateSnapshot {
        &self.inner
    }
}

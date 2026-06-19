use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryRuntimeAsyncResultState, ForgeQueryRuntimeStateSnapshot,
};

use crate::{
    ForgeServerDirectContextArtifact, ForgeServerQuerySupportPosture, ForgeServerResponseEnvelope,
};

#[derive(Debug)]
pub struct ForgeServerDirectState {
    plan_proof: crate::ForgeServerOperationPlanProof,
    support_posture: ForgeServerQuerySupportPosture,
    workspace_name: String,
    handoff_digest: String,
    direct_context: ForgeServerDirectContextArtifact,
    runtime_state: ForgeQueryRuntimeStateSnapshot,
    response_envelope: ForgeServerResponseEnvelope,
    canonical_digest: String,
}

impl ForgeServerDirectState {
    pub(crate) fn new(
        plan_proof: crate::ForgeServerOperationPlanProof,
        support_posture: ForgeServerQuerySupportPosture,
        workspace_name: String,
        handoff_digest: String,
        direct_context: ForgeServerDirectContextArtifact,
        runtime_state: ForgeQueryRuntimeStateSnapshot,
        response_envelope: ForgeServerResponseEnvelope,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-direct-state-v1:{}:{}",
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

    pub fn plan_proof(&self) -> &crate::ForgeServerOperationPlanProof {
        &self.plan_proof
    }

    pub fn support_posture(&self) -> &ForgeServerQuerySupportPosture {
        &self.support_posture
    }

    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }

    pub fn direct_context(&self) -> &ForgeServerDirectContextArtifact {
        &self.direct_context
    }

    pub fn runtime_state(&self) -> &ForgeQueryRuntimeStateSnapshot {
        &self.runtime_state
    }

    pub fn async_result_state(&self) -> Option<ForgeServerDirectAsyncResultState> {
        self.runtime_state
            .async_result_state()
            .cloned()
            .map(ForgeServerDirectAsyncResultState::new)
    }

    pub fn temporal_state(&self) -> Option<ForgeServerDirectTemporalState> {
        (self.runtime_state.authority_lane() == ForgeQueryAuthorityLane::TemporalExecutionState)
            .then(|| ForgeServerDirectTemporalState::new(self.runtime_state.clone()))
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn response_envelope(&self) -> &ForgeServerResponseEnvelope {
        &self.response_envelope
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerDirectAsyncResultState {
    inner: ForgeQueryRuntimeAsyncResultState,
}

impl ForgeServerDirectAsyncResultState {
    fn new(inner: ForgeQueryRuntimeAsyncResultState) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &ForgeQueryRuntimeAsyncResultState {
        &self.inner
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerDirectTemporalState {
    inner: ForgeQueryRuntimeStateSnapshot,
}

impl ForgeServerDirectTemporalState {
    fn new(inner: ForgeQueryRuntimeStateSnapshot) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &ForgeQueryRuntimeStateSnapshot {
        &self.inner
    }
}

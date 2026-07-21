use crate::{
    WorthServerDirectAsyncResultState, WorthServerDirectContextArtifact,
    WorthServerDirectDeclarationSnapshot, WorthServerDirectProvenance,
    WorthServerDirectRemaskPosture, WorthServerDirectState, WorthServerDirectTemporalState,
    WorthServerQuerySupportPosture,
};

#[derive(Debug)]
pub struct WorthServerDirectRetainedPosture {
    declaration_snapshot: WorthServerDirectDeclarationSnapshot,
    support_posture: WorthServerQuerySupportPosture,
    direct_context: WorthServerDirectContextArtifact,
    runtime_state: worth_query::facade::runtime::WorthQueryRuntimeStateSnapshot,
    async_result_state: Option<WorthServerDirectAsyncResultState>,
    temporal_state: Option<WorthServerDirectTemporalState>,
    canonical_digest: String,
}

impl WorthServerDirectRetainedPosture {
    pub(crate) fn new(
        declaration_snapshot: WorthServerDirectDeclarationSnapshot,
        state: WorthServerDirectState,
    ) -> Self {
        let canonical_digest = format!(
            "worth-server-direct-retained-posture-v1|declaration:{}|state:{}|context:{}",
            declaration_snapshot.declaration_digest(),
            state
                .runtime_state()
                .state_digest()
                .terminal_projection_for_reporting(),
            state.direct_context().canonical_digest(),
        );
        let async_result_state = state.async_result_state();
        let temporal_state = state.temporal_state();
        Self {
            declaration_snapshot,
            support_posture: state.support_posture().clone(),
            direct_context: state.direct_context().clone(),
            runtime_state: state.runtime_state().clone(),
            async_result_state,
            temporal_state,
            canonical_digest,
        }
    }

    pub fn declaration_snapshot(&self) -> &WorthServerDirectDeclarationSnapshot {
        &self.declaration_snapshot
    }

    pub fn support_posture(&self) -> &WorthServerQuerySupportPosture {
        &self.support_posture
    }

    pub fn direct_context(&self) -> &WorthServerDirectContextArtifact {
        &self.direct_context
    }

    pub fn runtime_state(&self) -> &worth_query::facade::runtime::WorthQueryRuntimeStateSnapshot {
        &self.runtime_state
    }

    pub fn basis_digest(&self) -> Option<&str> {
        self.direct_context.basis_digest()
    }

    pub fn remask_posture(&self) -> &WorthServerDirectRemaskPosture {
        self.direct_context.remask_posture()
    }

    pub fn provenance(&self) -> &WorthServerDirectProvenance {
        self.direct_context.provenance()
    }

    pub fn async_result_state(&self) -> Option<&WorthServerDirectAsyncResultState> {
        self.async_result_state.as_ref()
    }

    pub fn temporal_state(&self) -> Option<&WorthServerDirectTemporalState> {
        self.temporal_state.as_ref()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

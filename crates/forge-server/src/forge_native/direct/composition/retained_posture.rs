use crate::{
    ForgeServerDirectAsyncResultState, ForgeServerDirectContextArtifact,
    ForgeServerDirectDeclarationSnapshot, ForgeServerDirectProvenance,
    ForgeServerDirectRemaskPosture, ForgeServerDirectState, ForgeServerDirectTemporalState,
    ForgeServerQuerySupportPosture,
};

#[derive(Debug)]
pub struct ForgeServerDirectRetainedPosture {
    declaration_snapshot: ForgeServerDirectDeclarationSnapshot,
    support_posture: ForgeServerQuerySupportPosture,
    direct_context: ForgeServerDirectContextArtifact,
    runtime_state: forge_query::facade::ForgeQueryRuntimeStateSnapshot,
    async_result_state: Option<ForgeServerDirectAsyncResultState>,
    temporal_state: Option<ForgeServerDirectTemporalState>,
    canonical_digest: String,
}

impl ForgeServerDirectRetainedPosture {
    pub(crate) fn new(
        declaration_snapshot: ForgeServerDirectDeclarationSnapshot,
        state: ForgeServerDirectState,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-direct-retained-posture-v1|declaration:{}|state:{}|context:{}",
            declaration_snapshot.declaration_digest(),
            state.runtime_state().state_digest().terminal_projection_for_reporting(),
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

    pub fn declaration_snapshot(&self) -> &ForgeServerDirectDeclarationSnapshot {
        &self.declaration_snapshot
    }

    pub fn support_posture(&self) -> &ForgeServerQuerySupportPosture {
        &self.support_posture
    }

    pub fn direct_context(&self) -> &ForgeServerDirectContextArtifact {
        &self.direct_context
    }

    pub fn runtime_state(&self) -> &forge_query::facade::ForgeQueryRuntimeStateSnapshot {
        &self.runtime_state
    }

    pub fn basis_digest(&self) -> Option<&str> {
        self.direct_context.basis_digest()
    }

    pub fn remask_posture(&self) -> &ForgeServerDirectRemaskPosture {
        self.direct_context.remask_posture()
    }

    pub fn provenance(&self) -> &ForgeServerDirectProvenance {
        self.direct_context.provenance()
    }

    pub fn async_result_state(&self) -> Option<&ForgeServerDirectAsyncResultState> {
        self.async_result_state.as_ref()
    }

    pub fn temporal_state(&self) -> Option<&ForgeServerDirectTemporalState> {
        self.temporal_state.as_ref()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

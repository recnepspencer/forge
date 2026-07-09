use super::*;

impl RuntimeBridge {
    /// Admits one async completion into the authoritative writeback family
    /// boundary without yet staging or executing relational mutation.
    pub fn admit_async_writeback(
        &self,
        request: BridgeAsyncWritebackAdmissionRequest,
    ) -> Result<AdmittedBridgeAsyncWriteback, BridgeAsyncWritebackRejection> {
        AdmittedBridgeAsyncWriteback::admit(
            request,
            self.policy.diagnostics_tier(),
            self.policy.allow_replay_artifacts(),
            self.policy.record_route_artifacts(),
        )
    }

    /// Produces the staged writeback effect proof over the generic writeback
    /// engine for one already-admitted async completion writeback request.
    pub fn stage_async_writeback_effect(
        &self,
        writeback: &AdmittedBridgeAsyncWriteback,
    ) -> Result<StagedBridgeAsyncWritebackEffect, BridgeAsyncWritebackRejection> {
        StagedBridgeAsyncWritebackEffect::stage(self, writeback)
    }

    /// Commits one staged async writeback effect and returns explicit commit,
    /// noop, or rejection artifacts.
    pub fn commit_async_writeback(
        &self,
        staged: &StagedBridgeAsyncWritebackEffect,
    ) -> BridgeAsyncWritebackCommitReport {
        BridgeAsyncWritebackCommitReport::commit(self.signal_runtime_key, self, staged)
    }
}

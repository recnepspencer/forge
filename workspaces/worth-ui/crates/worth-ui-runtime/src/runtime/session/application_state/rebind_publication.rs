use super::WorthUiApplicationSessionState;

impl WorthUiApplicationSessionState {
    pub(crate) fn commit_evidence_only_rebind(
        &mut self,
        successor:
            crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
    ) -> (
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    ) {
        self.runtime.active_application_lowering_authority = successor.lowering_authority();
        self.app.commit_evidence_only_prepared_authority(successor)
    }
}

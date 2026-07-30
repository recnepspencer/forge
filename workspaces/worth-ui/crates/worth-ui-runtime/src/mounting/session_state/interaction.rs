use super::WorthUiMountedSessionState;

impl WorthUiMountedSessionState {
    pub(crate) fn interaction_hit_test_basis(
        &self,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    ) -> Result<
        crate::mounting::UiPresentedHitTestBasis,
        crate::mounting::UiPresentedFrameBasisDenial,
    > {
        self.retention.interaction_hit_test_basis(presentation)
    }
}

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

    pub(crate) fn admit_current_hit_target(
        &self,
        row: worth_ui_host_contract::UiMountedHitTestMechanic,
    ) -> Result<
        crate::mounting::UiCurrentHitTarget,
        crate::mounting::UiCurrentHitTargetAffinityDenial,
    > {
        self.identity.admit_current_hit_target(row)
    }

    pub(crate) fn admit_current_interaction_affinity(
        &self,
        input: crate::mounting::UiMountedInteractionAffinityInput,
    ) -> Result<(), crate::mounting::UiCurrentHitTargetAffinityDenial> {
        self.identity.admit_current_interaction_affinity(input)
    }
}

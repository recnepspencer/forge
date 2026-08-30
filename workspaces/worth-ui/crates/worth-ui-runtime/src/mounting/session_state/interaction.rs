use super::WorthUiMountedSessionState;

impl WorthUiMountedSessionState {
    pub(crate) fn interaction_hit_test_basis(
        &self,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    ) -> Result<
        crate::mounting::UiPresentedHitTestBasis,
        crate::mounting::UiPresentedFrameBasisDenial,
    > {
        if self
            .presentation
            .binding_requires_reconstruction(presentation.binding())
        {
            return Err(crate::mounting::UiPresentedFrameBasisDenial::PresentationTruthUnavailable);
        }
        let mut basis = self.retention.interaction_hit_test_basis(presentation)?;
        basis.apply_motion_samples(&self.motion_sampling);
        Ok(basis)
    }

    pub(crate) fn semantic_focus_placement_basis(
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
    ) -> Result<
        crate::mounting::UiCurrentInteractionAffinity,
        crate::mounting::UiCurrentHitTargetAffinityDenial,
    > {
        self.identity.admit_current_interaction_affinity(input)
    }

    pub(crate) fn admit_current_mounted_incarnation_affinity(
        &self,
        input: crate::mounting::UiMountedIncarnationAffinityInput,
    ) -> Result<
        crate::mounting::UiCurrentInteractionAffinity,
        crate::mounting::UiCurrentHitTargetAffinityDenial,
    > {
        self.identity
            .admit_current_mounted_incarnation_affinity(input)
    }

    pub(crate) fn input_text_profile(
        &self,
        target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    ) -> Option<worth_ui_host_contract::UiTextProfileGeneration> {
        crate::runtime::interaction::targeting::require_current_target(self, target).ok()?;
        Some(self.identity.current_projection()?.input_text_profile())
    }
}

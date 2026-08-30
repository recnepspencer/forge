use super::UiMountedPresentationCoordinator;

impl UiMountedPresentationCoordinator {
    pub fn reconcile(
        &mut self,
        reconciliation: super::super::UiHostPresentationReconciliation,
        current_frame: Option<worth_ui_host_contract::UiMountedFrameIdentity>,
    ) -> bool {
        self.host_truth
            .reconcile_presentation(reconciliation, current_frame)
    }

    pub(crate) fn binding_requires_reconciliation(
        &self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> bool {
        self.host_truth.binding_requires_reconciliation(binding)
    }

    pub(crate) fn binding_requires_reconstruction(
        &self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> bool {
        self.reconstruction_bindings.contains(&binding)
    }

    pub(crate) fn motion_presentation_truth_unavailable(&self) -> bool {
        !self.reconstruction_bindings.is_empty()
    }

    pub(crate) fn commit_current_frame_reconciliation(
        &mut self,
        replacements: &[super::super::UiMountedSurfaceReconciliationBinding],
    ) {
        self.host_truth
            .commit_current_frame_reconciliation(replacements);
    }

    pub(crate) fn reconcile_candidate_only_deregistration(
        &mut self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) {
        self.host_truth
            .reconcile_candidate_only_deregistration(binding);
    }

    pub(crate) fn has_active_attempt(&self) -> bool {
        !self.active.borrow().is_empty()
    }

    pub(crate) fn host_truth_mut(&mut self) -> &mut crate::mounting::UiMountedHostTruthCoordinator {
        &mut self.host_truth
    }
}

use worth_ui_host_contract::{UiMountedFrameIdentity, UiSurfaceBindingGeneration};

use super::binding_truth::UiMountedHostTruthCoordinator;
use crate::mounting::{
    UiHostPresentationReconciliation, UiMountedSurfaceReconciliationBinding, UiPreparedMountedFrame,
};

impl UiMountedHostTruthCoordinator {
    pub(crate) fn reconcile_presentation(
        &mut self,
        reconciliation: UiHostPresentationReconciliation,
        current_frame: Option<UiMountedFrameIdentity>,
    ) -> bool {
        let binding = reconciliation.affected_binding();
        let Some(requirement) = self.blocked_presentation_requirement(binding) else {
            return false;
        };
        if !reconciliation.proves(requirement, current_frame) {
            return false;
        }
        let surface = requirement.semantic_surface();
        self.clear_presentations_for_surface(surface);
        true
    }

    pub(crate) fn reconcile_candidate_only_deregistration(
        &mut self,
        binding: UiSurfaceBindingGeneration,
    ) {
        let Some(requirement) = self.blocked_presentation_requirement(binding) else {
            return;
        };
        let surface = requirement.semantic_surface();
        self.clear_presentations_for_surface(surface);
    }

    pub(crate) fn reconciliation_covers(
        &self,
        frame: &UiPreparedMountedFrame,
        replacements: &[UiMountedSurfaceReconciliationBinding],
    ) -> bool {
        !replacements.is_empty()
            && replacements
                .iter()
                .all(|replacement| self.replacement_is_valid(frame, *replacement))
            && frame.surfaces().iter().all(|surface| {
                !replacements
                    .iter()
                    .any(|replacement| surface.requirement().binding() == replacement.affected())
            })
            && self
                .blocked
                .values()
                .all(|blocked| blocked.native_lifecycle_obligation().is_none())
            && self.blocked.values().all(|blocked| {
                blocked.presentation_requirement().is_none()
                    || replacements.iter().any(|replacement| {
                        self.blocked_presentation_requirement(replacement.affected())
                            .is_some_and(|covered| {
                                covered.semantic_surface() == blocked.semantic_surface()
                            })
                    })
            })
    }

    pub(crate) fn commit_current_frame_reconciliation(
        &mut self,
        replacements: &[UiMountedSurfaceReconciliationBinding],
    ) {
        for replacement in replacements {
            let Some(surface) = self
                .blocked_presentation_requirement(replacement.affected())
                .map(|blocked| blocked.semantic_surface())
            else {
                continue;
            };
            self.clear_presentations_for_surface(surface);
        }
    }

    fn replacement_is_valid(
        &self,
        frame: &UiPreparedMountedFrame,
        replacement: UiMountedSurfaceReconciliationBinding,
    ) -> bool {
        let Some(blocked) = self.blocked_presentation_requirement(replacement.affected()) else {
            return false;
        };
        frame.surfaces().iter().any(|surface| {
            let requirement = surface.requirement();
            requirement.binding() == replacement.replacement()
                && requirement.semantic_surface() == blocked.semantic_surface()
        })
    }
}

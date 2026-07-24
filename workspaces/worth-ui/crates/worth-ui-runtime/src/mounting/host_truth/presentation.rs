use worth_ui_host_contract::{UiMountedSurfaceBindingRequirement, UiSurfaceBindingGeneration};

use super::binding_truth::UiMountedHostTruthCoordinator;
use super::registration_attempt::UiHostTruthIndeterminateObligations;

impl UiMountedHostTruthCoordinator {
    pub(crate) fn block_presentation(&mut self, requirement: UiMountedSurfaceBindingRequirement) {
        self.blocked
            .entry(requirement.binding())
            .and_modify(|blocked| blocked.record_presentation(requirement))
            .or_insert_with(|| UiHostTruthIndeterminateObligations::presentation(requirement));
    }

    pub(crate) fn blocked_presentation_requirement(
        &self,
        binding: UiSurfaceBindingGeneration,
    ) -> Option<UiMountedSurfaceBindingRequirement> {
        self.blocked
            .get(&binding)
            .and_then(|record| record.presentation_requirement())
    }
}

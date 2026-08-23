use std::collections::BTreeMap;

use worth_ui_host_contract::{
    UiHostSurfaceBaselineIdentity, UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
};

use super::registration_attempt::{
    UiHostTruthIndeterminateObligations, UiHostTruthNativeLifecycleKind,
};

#[derive(Default)]
pub(crate) struct UiMountedHostTruthCoordinator {
    pub(super) known_empty: BTreeMap<
        UiSurfaceBindingGeneration,
        super::surface_lifecycle::UiMountedSurfaceBaselineReceipt,
    >,
    pub(super) blocked: BTreeMap<UiSurfaceBindingGeneration, UiHostTruthIndeterminateObligations>,
}

impl UiMountedHostTruthCoordinator {
    pub(crate) fn binding_requires_reconciliation(
        &self,
        binding: UiSurfaceBindingGeneration,
    ) -> bool {
        self.blocked.contains_key(&binding)
    }

    pub(crate) fn surface_requires_reconciliation(
        &self,
        surface: UiSemanticSurfaceIdentity,
    ) -> bool {
        self.blocked
            .values()
            .any(|record| record.semantic_surface() == surface)
    }

    pub(crate) fn has_live_baseline(
        &self,
        binding: UiSurfaceBindingGeneration,
        baseline: UiHostSurfaceBaselineIdentity,
    ) -> bool {
        self.known_empty
            .get(&binding)
            .is_some_and(|receipt| receipt.identity() == baseline)
    }

    pub(super) fn surface_has_indeterminate_native_lifecycle(
        &self,
        surface: UiSemanticSurfaceIdentity,
    ) -> bool {
        self.blocked.values().any(|record| {
            record.semantic_surface() == surface && record.native_lifecycle_obligation().is_some()
        })
    }

    pub(super) fn block_native_lifecycle(
        &mut self,
        kind: UiHostTruthNativeLifecycleKind,
        request: worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
    ) {
        self.blocked
            .entry(request.binding_generation())
            .and_modify(|blocked| blocked.record_native_lifecycle(kind, request))
            .or_insert_with(|| {
                UiHostTruthIndeterminateObligations::native_lifecycle(kind, request)
            });
    }

    pub(super) fn clear_native_lifecycle(&mut self, binding: UiSurfaceBindingGeneration) {
        if let Some(blocked) = self.blocked.get_mut(&binding) {
            blocked.clear_native_lifecycle();
            if blocked.is_empty() {
                self.blocked.remove(&binding);
            }
        }
    }

    pub(super) fn clear_presentations_for_surface(&mut self, surface: UiSemanticSurfaceIdentity) {
        for blocked in self.blocked.values_mut() {
            if blocked.semantic_surface() == surface {
                blocked.clear_presentation();
            }
        }
        self.blocked.retain(|_, blocked| !blocked.is_empty());
    }
}

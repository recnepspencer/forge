use worth_ui_host_contract::{
    UiMountedDiagnosticProjection, UiMountedInstanceIdentity, UiSurfaceBindingGeneration,
};

use super::{UiMountedProjectionFrame, UiMountedSemanticProjection};
use crate::runtime::persistent_index::UiPersistentOrdMap;

pub(in crate::mounting) type UiMountedDiagnosticSourceRow = (
    UiSurfaceBindingGeneration,
    UiMountedInstanceIdentity,
    UiMountedDiagnosticProjection,
);

#[derive(Clone, Default)]
pub(in crate::mounting) struct UiMountedDiagnosticSource {
    by_instance: UiPersistentOrdMap<
        UiMountedInstanceIdentity,
        (UiSurfaceBindingGeneration, UiMountedDiagnosticProjection),
    >,
}

impl UiMountedDiagnosticSource {
    pub(super) fn apply(
        &mut self,
        semantic: &UiMountedSemanticProjection,
        changed: &[UiMountedInstanceIdentity],
        overlay: Option<crate::mounting::UiMountedVisualOverlayProjectionInput>,
        frame: worth_ui_host_contract::UiMountedFrameIdentity,
    ) {
        for instance in changed {
            self.by_instance.remove(instance);
            let Some(node) = semantic.nodes.get(instance) else {
                continue;
            };
            let Some(surface) = semantic.surface_for(node.receipt.semantic_surface()) else {
                continue;
            };
            let diagnostic = overlay
                .filter(|_| surface.audience.diagnostics_disclosed())
                .filter(|overlay| overlay.target_instance() == *instance)
                .and_then(|overlay| overlay.mechanic_for(frame, surface.surface, surface.binding))
                .map_or_else(
                    || node.receipt.diagnostic(),
                    UiMountedDiagnosticProjection::IdentityOverlay,
                );
            self.by_instance
                .insert(*instance, (surface.binding, diagnostic));
        }
    }

    pub(in crate::mounting) fn len(&self) -> usize {
        self.by_instance.len()
    }

    pub(in crate::mounting) fn rows(
        &self,
    ) -> impl Iterator<Item = UiMountedDiagnosticSourceRow> + '_ {
        self.by_instance
            .iter()
            .map(|(instance, (binding, diagnostic))| (*binding, *instance, *diagnostic))
    }
}

impl UiMountedProjectionFrame {
    pub(in crate::mounting) fn diagnostic_source(&self) -> UiMountedDiagnosticSource {
        self.diagnostics.clone()
    }
}

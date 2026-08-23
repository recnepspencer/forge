use worth_ui_host_contract::UiSurfaceBindingGeneration;

use super::UiMountedProjectionFrame;
use crate::mounting::projection::UiMountedProjectionDenial;

impl UiMountedProjectionFrame {
    pub(in crate::mounting) fn rebind_retained_mechanics(
        &mut self,
        replacements: &[(
            UiSurfaceBindingGeneration,
            crate::mounting::UiSurfaceBindingIdentityView,
        )],
    ) -> Result<(), UiMountedProjectionDenial> {
        self.mechanics.rebind(replacements)
    }

    pub(crate) fn rebound(
        &self,
        successor: worth_ui_host_contract::UiMountedFrameIdentity,
        replacements: &[(
            UiSurfaceBindingGeneration,
            crate::mounting::UiSurfaceBindingIdentityView,
        )],
    ) -> Result<Self, UiMountedProjectionDenial> {
        let mut rebound = self.clone();
        rebound.frame = successor;
        for (affected, replacement) in replacements {
            let mut surface = rebound
                .semantic
                .surfaces
                .get(affected)
                .copied()
                .ok_or(UiMountedProjectionDenial::MissingSurfaceBinding)?;
            if surface.surface != replacement.semantic_surface_identity() {
                return Err(UiMountedProjectionDenial::MissingSurfaceBinding);
            }
            rebound.semantic.surfaces.remove(affected);
            surface.binding = replacement.binding_generation();
            rebound.semantic.surfaces.insert(surface.binding, surface);
        }
        rebound.mechanics.rebind(replacements)?;
        Ok(rebound)
    }
}

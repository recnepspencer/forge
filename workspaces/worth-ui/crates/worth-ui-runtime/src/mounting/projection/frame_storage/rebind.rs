use worth_ui_host_contract::UiSurfaceBindingGeneration;

use super::UiMountedProjectionFrame;
use crate::mounting::projection::UiMountedProjectionDenial;

impl UiMountedProjectionFrame {
    pub(crate) fn rebound(
        &self,
        replacements: &[(
            UiSurfaceBindingGeneration,
            crate::mounting::UiSurfaceBindingIdentityView,
        )],
    ) -> Result<Self, UiMountedProjectionDenial> {
        let mut rebound = self.clone();
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
        super::super::static_paint::rebind_filled_rects(&mut rebound.filled_rects, replacements)?;
        super::super::semantic_text::rebind_semantic_text(
            &mut rebound.semantic_text,
            replacements,
        )?;
        super::super::hit_test::rebind_hit_tests(&mut rebound.hit_tests, replacements)?;
        Ok(rebound)
    }
}

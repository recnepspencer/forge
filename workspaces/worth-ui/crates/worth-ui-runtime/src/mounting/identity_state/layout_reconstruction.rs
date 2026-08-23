use super::UiMountedIdentityState;

impl UiMountedIdentityState {
    pub(crate) const fn peak_qualified_layouts(&self) -> usize {
        self.peak_qualified_layouts
    }

    pub(crate) fn require_current_layout_reconstruction(
        &mut self,
    ) -> Result<usize, crate::mounting::UiMountedProjectionDenial> {
        let projection = self.current_projection.as_mut().ok_or(
            crate::mounting::UiMountedProjectionDenial::MissingSemanticTextReconstructionSource,
        )?;
        let mut successor = projection.as_ref().clone();
        let lost = successor.require_qualified_layout_reconstruction()?;
        *projection = std::sync::Arc::new(successor);
        Ok(lost)
    }

    pub(crate) fn reconstruct_current_layouts(
        &mut self,
    ) -> Result<usize, crate::mounting::UiMountedProjectionDenial> {
        if !self
            .current_projection
            .as_ref()
            .is_some_and(|projection| projection.qualified_layout_reconstruction_required())
        {
            return Ok(0);
        }
        let projection = self.current_projection.as_mut().ok_or(
            crate::mounting::UiMountedProjectionDenial::MissingSemanticTextReconstructionSource,
        )?;
        let mut successor = projection.as_ref().clone();
        let reconstructed = successor.reconstruct_qualified_layouts()?;
        *projection = std::sync::Arc::new(successor);
        Ok(reconstructed)
    }
}

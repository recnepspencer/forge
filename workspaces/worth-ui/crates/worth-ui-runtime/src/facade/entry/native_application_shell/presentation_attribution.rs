use super::WorthUiNativeApplicationShell;

impl WorthUiNativeApplicationShell {
    pub(crate) fn current_presentation_attribution(
        &self,
    ) -> Option<worth_ui_host_native::UiNativeClientPresentationAttribution> {
        let publication = self.session.mounted.current_publication()?;
        let frame = publication.frame();
        let binding = *publication.bindings().first()?;
        let attribution = self
            .session
            .mounted
            .native_paint_attribution(frame, binding)?;
        Some(
            worth_ui_host_native::UiNativeClientPresentationAttribution::reported(
                [
                    frame.diagnostic_value(),
                    attribution.surface.diagnostic_value(),
                    binding.diagnostic_value(),
                    attribution.mounted_instance.diagnostic_value(),
                    attribution.node_receipt.diagnostic_value(),
                    publication.attempt().diagnostic_value(),
                ],
                [
                    attribution.authored_provenance_digest,
                    attribution.authored_semantic_identity_digest,
                ],
            ),
        )
    }
}

impl<Posture: worth_ui_inspection::UiVisualArtifactPolicy> super::UiVisualSnapshotReceipt<Posture> {
    pub(crate) const fn session_identity(
        &self,
    ) -> crate::lifecycle::WorthUiActiveApplicationSessionIdentity {
        self.session
    }

    pub(crate) const fn disclosure(&self) -> worth_ui_inspection::UiVisualInspectionDisclosure {
        self.evidence.disclosure()
    }

    pub(crate) fn retained_pixel_artifact(
        &self,
    ) -> Option<&worth_ui_inspection::UiVisualPixelArtifact> {
        self.pixel_artifact.as_ref()
    }
}

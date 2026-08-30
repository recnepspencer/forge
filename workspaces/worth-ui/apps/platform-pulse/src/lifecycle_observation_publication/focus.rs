use super::{PlatformPulseObservationPublicationDenial, PlatformPulseObservationPublisher};

impl PlatformPulseObservationPublisher {
    pub(crate) fn semantic_focus_published(
        &self,
        receipt: worth_ui::facade::app::UiSemanticFocusPublicationReceipt,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.project_observation(|stream| stream.project_semantic_focus_published(receipt))
    }
}

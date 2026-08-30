use super::{
    PlatformPulseLifecycleObservation, PlatformPulseLifecycleObservationEnvelope,
    PlatformPulseLifecycleObservationProjectionDenial, PlatformPulseLifecycleObservationStream,
    PlatformPulseSemanticFocusPublished,
};

impl PlatformPulseLifecycleObservationStream {
    pub fn project_semantic_focus_published(
        &mut self,
        receipt: worth_ui::facade::app::UiSemanticFocusPublicationReceipt,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        let (_, _, published_frame) = self.published_predecessor()?;
        let observation = PlatformPulseSemanticFocusPublished::from_runtime(receipt)?;
        if observation.frame() != published_frame.diagnostic_value() {
            return Err(
                PlatformPulseLifecycleObservationProjectionDenial::SemanticFocusPublicationMismatch,
            );
        }
        self.next_envelope(PlatformPulseLifecycleObservation::SemanticFocusPublished(
            observation,
        ))
    }
}

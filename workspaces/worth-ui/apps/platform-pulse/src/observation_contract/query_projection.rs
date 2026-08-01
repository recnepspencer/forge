use worth_ui::facade::app::UiMountedFramePublicationReceipt;

use super::{
    PlatformPulseLifecycleObservation, PlatformPulseLifecycleObservationEnvelope,
    PlatformPulseLifecycleObservationProjectionDenial, PlatformPulseLifecycleObservationStream,
    PlatformPulseQueryProjectionEvidence, PlatformPulseQueryProjectionPosture,
    PlatformPulseQueryProjectionPublished,
};

impl PlatformPulseLifecycleObservationStream {
    pub fn project_query_projection_issued(
        &mut self,
        projection: &PlatformPulseQueryProjectionEvidence,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        if matches!(
            self.state,
            super::projection::PlatformPulseObservationState::Terminal
        ) {
            return Err(PlatformPulseLifecycleObservationProjectionDenial::StreamTerminated);
        }
        self.next_envelope(PlatformPulseLifecycleObservation::QueryProjectionIssued(
            projection.clone(),
        ))
    }

    pub fn project_query_projection_published(
        &mut self,
        projection: &PlatformPulseQueryProjectionEvidence,
        publication: &UiMountedFramePublicationReceipt,
    ) -> Result<
        PlatformPulseLifecycleObservationEnvelope,
        PlatformPulseLifecycleObservationProjectionDenial,
    > {
        let validated = self.validate_content_publication(publication)?;
        let frame = validated.frame();
        let next_visual_state = match (projection.posture(), projection.owner_order()) {
            (PlatformPulseQueryProjectionPosture::Current, 2) => {
                super::projection::PlatformPulseVisualObservationState::AwaitingSnapshot {
                    frame: frame.diagnostic_value(),
                }
            }
            (PlatformPulseQueryProjectionPosture::Current, _) => self
                .visual_state
                .after_content_publication(frame.diagnostic_value())?,
            _ => self.visual_state,
        };
        let envelope =
            self.next_envelope(PlatformPulseLifecycleObservation::QueryProjectionPublished(
                PlatformPulseQueryProjectionPublished::new(projection.clone(), frame),
            ))?;
        self.commit_content_publication(validated);
        self.visual_state = next_visual_state;
        Ok(envelope)
    }
}

use worth_ui::facade::app::UiMountedFramePublicationReceipt;

use super::{
    PlatformPulseLifecycleObservation, PlatformPulseLifecycleObservationEnvelope,
    PlatformPulseLifecycleObservationProjectionDenial, PlatformPulseLifecycleObservationStream,
    PlatformPulseMountedFrameObservation, PlatformPulseQueryProjectionEvidence,
    PlatformPulseQueryProjectionPosture, PlatformPulseQueryProjectionPublished,
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
        if matches!(
            self.state,
            super::projection::PlatformPulseObservationState::Terminal
        ) {
            return Err(PlatformPulseLifecycleObservationProjectionDenial::StreamTerminated);
        }
        let frame = PlatformPulseMountedFrameObservation {
            diagnostic_value: publication.frame().diagnostic_value(),
        };
        match &mut self.state {
            super::projection::PlatformPulseObservationState::Published {
                generation,
                frame: active_frame,
                ..
            } if publication.generation() == generation => {
                *active_frame = frame;
            }
            super::projection::PlatformPulseObservationState::Published { .. } => {
                return Err(
                    PlatformPulseLifecycleObservationProjectionDenial::ActiveGenerationMismatch,
                )
            }
            super::projection::PlatformPulseObservationState::Started => return Err(
                PlatformPulseLifecycleObservationProjectionDenial::PublishedPredecessorUnavailable,
            ),
            super::projection::PlatformPulseObservationState::Terminal => unreachable!(),
        }
        if projection.posture() == PlatformPulseQueryProjectionPosture::Current
            && projection.owner_order() == 2
        {
            self.visual_state =
                super::projection::PlatformPulseVisualObservationState::AwaitingSnapshot {
                    frame: frame.diagnostic_value,
                };
        } else if projection.posture() == PlatformPulseQueryProjectionPosture::Current {
            if let super::projection::PlatformPulseVisualObservationState::OverlayCleared {
                snapshot,
                snapshot_frame,
                ..
            } = self.visual_state
            {
                self.visual_state = super::projection::PlatformPulseVisualObservationState::
                    AwaitingRefreshRetirement {
                        snapshot,
                        snapshot_frame,
                        refresh_frame: frame.diagnostic_value,
                    };
            }
        }
        self.next_envelope(PlatformPulseLifecycleObservation::QueryProjectionPublished(
            PlatformPulseQueryProjectionPublished::new(projection.clone(), frame),
        ))
    }
}

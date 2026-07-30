use crate::facade::observation_report::{
    UiHostObservationBatch, UiHostObservationReportDenial, UiHostObservationReportOutcome,
};
use crate::runtime::interaction::{
    UiHostInteractionIngressOutcome, UiInteractionObservationDenial, UiInteractionStateSnapshot,
    UiPointerGestureStopReason,
};

use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    /// Validates raw host evidence and immediately moves admitted evidence into
    /// the interaction owner. Callers cannot supply pre-validated proxy input.
    pub fn admit_host_interaction_batch(
        &mut self,
        batch: UiHostObservationBatch,
    ) -> UiHostInteractionIngressOutcome {
        let binding = batch.canonical_core().binding();
        match self.validate_host_observation_batch(batch) {
            UiHostObservationReportOutcome::Validated(batch) => {
                UiHostInteractionIngressOutcome::Applied(
                    self.interaction.ingest(batch, &self.mounted),
                )
            }
            UiHostObservationReportOutcome::Duplicate(duplicate) => {
                UiHostInteractionIngressOutcome::Duplicate(duplicate)
            }
            UiHostObservationReportOutcome::Quarantined(quarantined) => {
                UiHostInteractionIngressOutcome::Quarantined(quarantined)
            }
            UiHostObservationReportOutcome::Denied(denial) => {
                let settled_gestures = if denial_invalidates_local_gesture(denial) {
                    self.interaction
                        .cancel_binding(binding, UiPointerGestureStopReason::InvalidObservation)
                } else {
                    0
                };
                UiHostInteractionIngressOutcome::Denied(UiInteractionObservationDenial::new(
                    denial,
                    settled_gestures,
                ))
            }
        }
    }

    pub fn interaction_state(&self) -> UiInteractionStateSnapshot {
        self.interaction.snapshot()
    }
}

fn denial_invalidates_local_gesture(denial: UiHostObservationReportDenial) -> bool {
    matches!(
        denial,
        UiHostObservationReportDenial::SequenceGap
            | UiHostObservationReportDenial::SequenceReordered
            | UiHostObservationReportDenial::SequenceOverlap
            | UiHostObservationReportDenial::SequenceExhausted
            | UiHostObservationReportDenial::LosslessOverflow(_)
            | UiHostObservationReportDenial::UnknownFrame
            | UiHostObservationReportDenial::ExpiredFrame
            | UiHostObservationReportDenial::RejectedFrame
            | UiHostObservationReportDenial::NeverPresentedFrame
            | UiHostObservationReportDenial::BindingNotPresented
            | UiHostObservationReportDenial::PresentationEpochMismatch
            | UiHostObservationReportDenial::MountedInstanceNotPresented
            | UiHostObservationReportDenial::NodeReceiptMismatch
            | UiHostObservationReportDenial::FrameTransitionInFlight
    )
}

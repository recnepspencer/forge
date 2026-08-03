use crate::facade::observation_report::{
    UiHostObservationBatch, UiHostObservationCanonicalCore, UiHostObservationFamily,
    UiHostObservationLoss, UiHostObservationReportDenial, UiHostObservationReportOutcome,
};
use crate::runtime::interaction::{
    UiHostInteractionIngressOutcome, UiInteractionLifecycleStopReason,
    UiInteractionObservationDenial, UiInteractionStateSnapshot, UiQuarantinedHostInteractionBatch,
};

use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    /// Validates raw host evidence and immediately moves admitted evidence into
    /// the interaction owner. Callers cannot supply pre-validated proxy input.
    pub fn admit_host_interaction_batch(
        &mut self,
        batch: UiHostObservationBatch,
    ) -> UiHostInteractionIngressOutcome {
        let core = batch.canonical_core();
        let binding = core.binding();
        match self.validate_host_observation_batch(batch) {
            UiHostObservationReportOutcome::Validated(batch) => {
                UiHostInteractionIngressOutcome::Applied(self.interaction.ingest(
                    batch,
                    &self.mounted,
                    self.application.generation_identity(),
                ))
            }
            UiHostObservationReportOutcome::Duplicate(duplicate) => {
                UiHostInteractionIngressOutcome::Duplicate(duplicate)
            }
            UiHostObservationReportOutcome::Quarantined(quarantined) => {
                let settlement = self.interaction.cancel_binding(
                    binding,
                    UiInteractionLifecycleStopReason::ObservationQuarantined,
                );
                UiHostInteractionIngressOutcome::Quarantined(
                    UiQuarantinedHostInteractionBatch::new(quarantined, settlement),
                )
            }
            UiHostObservationReportOutcome::Denied(denial) => {
                let settlement = if denial_invalidates_local_gesture(denial) {
                    self.interaction
                        .cancel_binding(binding, denial_stop_reason(denial, core))
                } else {
                    self.interaction.unchanged_settlement()
                };
                UiHostInteractionIngressOutcome::Denied(UiInteractionObservationDenial::new(
                    denial, settlement,
                ))
            }
        }
    }

    pub fn interaction_state(&self) -> UiInteractionStateSnapshot {
        self.interaction.snapshot()
    }
}

fn denial_stop_reason(
    denial: UiHostObservationReportDenial,
    core: UiHostObservationCanonicalCore,
) -> UiInteractionLifecycleStopReason {
    match (denial, core.loss()) {
        (
            UiHostObservationReportDenial::LosslessOverflow(UiHostObservationFamily::PointerButton),
            UiHostObservationLoss::Overflow {
                family: UiHostObservationFamily::PointerButton,
                affected,
            },
        ) => UiInteractionLifecycleStopReason::ObservationLoss {
            family: UiHostObservationFamily::PointerButton,
            affected: Some(affected),
        },
        (UiHostObservationReportDenial::LosslessOverflow(family), _) => {
            UiInteractionLifecycleStopReason::ObservationLoss {
                family,
                affected: None,
            }
        }
        _ => UiInteractionLifecycleStopReason::ObservationInvalid,
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

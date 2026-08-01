use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseIntentAttemptObservationReference, PlatformPulseIntentPostureObservation,
    PlatformPulseLifecycleObservation,
};

use crate::product_process::{NativeBoundExecutableWorld, WatchedPulseTransition};

use super::super::{observation, PlatformPulseIntentJourneyFailure};

pub(super) fn await_rebind_cancellation(
    world: &mut NativeBoundExecutableWorld,
    expected_attempt: PlatformPulseIntentAttemptObservationReference,
) -> Result<u64, PlatformPulseIntentJourneyFailure> {
    let rebind = observation::next(world, WatchedPulseTransition::IntentCancellationRebind)?;
    let PlatformPulseLifecycleObservation::RebindPublished(replacement) = rebind.outcome() else {
        return Err(PlatformPulseIntentJourneyFailure::Cancellation(
            "route-removal source edit did not publish an application replacement",
        ));
    };
    if replacement.predecessor_generation() == replacement.active_generation() {
        return Err(PlatformPulseIntentJourneyFailure::Cancellation(
            "route removal did not change application generation",
        ));
    }

    let mut observed = CancellationObservationProgress::default();
    let mut sequence = rebind.sequence().value();
    while !observed.complete() {
        let envelope = observation::next(world, WatchedPulseTransition::IntentCancellationRebind)?;
        sequence = envelope.sequence().value();
        observed.advance(envelope.outcome(), expected_attempt)?;
    }
    Ok(sequence)
}

#[derive(Default)]
struct CancellationObservationProgress {
    cancelled: bool,
    captured: bool,
    compared: bool,
    retired: bool,
}

impl CancellationObservationProgress {
    fn advance(
        &mut self,
        outcome: &PlatformPulseLifecycleObservation,
        expected_attempt: PlatformPulseIntentAttemptObservationReference,
    ) -> Result<(), PlatformPulseIntentJourneyFailure> {
        match outcome {
            PlatformPulseLifecycleObservation::IntentPosturePublished(posture)
                if !self.cancelled
                    && matches!(
                        posture.posture(),
                        PlatformPulseIntentPostureObservation::Cancelled { reference }
                            if *reference == expected_attempt
                    ) =>
            {
                self.cancelled = true;
            }
            PlatformPulseLifecycleObservation::VisualSnapshotCaptured(_) if !self.captured => {
                self.captured = true;
            }
            PlatformPulseLifecycleObservation::VisualComparison(_)
                if self.captured && !self.compared =>
            {
                self.compared = true;
            }
            PlatformPulseLifecycleObservation::VisualSnapshotRetired(_)
                if self.compared && !self.retired =>
            {
                self.retired = true;
            }
            _ => {
                return Err(PlatformPulseIntentJourneyFailure::Cancellation(
                    "replacement cancellation emitted an unexpected or out-of-order lifecycle event",
                ));
            }
        }
        Ok(())
    }

    const fn complete(&self) -> bool {
        self.cancelled && self.captured && self.compared && self.retired
    }
}

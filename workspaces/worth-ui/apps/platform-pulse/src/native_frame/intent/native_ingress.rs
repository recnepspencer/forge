use worth_ui::facade::app::{
    WorthUiNativeApplicationShell, WorthUiNativeIntentPosture, WorthUiNativeIntentPostureKind,
    WorthUiNativeIntentPosturePublicationOutcome, WorthUiNativeIntentStop,
    WorthUiNativeIntentTransition,
};
use worth_ui_platform_pulse::observation_contract::PlatformPulseIntentPostureObservation;

use super::super::PlatformPulseNativeFrame;

impl PlatformPulseNativeFrame {
    pub(in crate::native_frame) fn admit_native_intent_input(&mut self) {
        let Some(host) = self.host.as_ref() else {
            return;
        };
        let deadline = match self.intent_clock.new_attempt_deadline() {
            Ok(deadline) => deadline,
            Err(denial) => {
                self.fail_intent_clock(denial);
                return;
            }
        };
        let Some(mut shell) = self.shell.take() else {
            return;
        };
        let drain = host.drain_native_observations(shell.host_session_identity().as_u64());
        let ingress = shell.admit_native_intent_observations(
            worth_ui_platform_pulse::intent::platform_pulse_action_definition(),
            drain,
            deadline,
        );
        if ingress.duplicate_batches() > 0 || !ingress.interaction_stops().is_empty() {
            self.fail_intent_settlement("native interaction ingress stopped or duplicated");
            self.shell = Some(shell);
            return;
        }
        for transition in ingress.into_transitions() {
            let (posture, observation) = match transition {
                WorthUiNativeIntentTransition::AttemptPrepared(prepared) => {
                    if let Err(denial) = self.intent_evidence_index.retain(prepared.dispatch()) {
                        self.fail_intent_settlement(format!(
                            "prepared intent evidence could not be retained: {denial:?}"
                        ));
                        break;
                    }
                    let observation =
                        PlatformPulseIntentPostureObservation::admitted(prepared.dispatch());
                    (prepared.into_posture(), observation)
                }
                WorthUiNativeIntentTransition::ConfirmationRequired(pending) => {
                    let observation = PlatformPulseIntentPostureObservation::confirmation_required(
                        pending.pending(),
                    );
                    let (_, posture) = pending.into_parts();
                    (posture, observation)
                }
                WorthUiNativeIntentTransition::Stopped(stopped) => {
                    let (stop, posture) = stopped.into_parts();
                    if posture.is_none() && is_unrouted_interaction(&stop) {
                        continue;
                    }
                    let Some(posture) = posture else {
                        self.fail_intent_settlement(
                            "native intent stopped without a publishable posture",
                        );
                        break;
                    };
                    let Some(observation) = stopped_posture_observation(posture.kind()) else {
                        self.fail_intent_settlement(
                            "native intent stopped with an invalid terminal posture",
                        );
                        break;
                    };
                    (posture, observation)
                }
            };
            if !self.publish_native_intent_posture(&mut shell, posture, observation) {
                break;
            }
        }
        self.shell = Some(shell);
    }

    pub(in crate::native_frame::intent) fn publish_native_intent_posture(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        posture: WorthUiNativeIntentPosture,
        observation: PlatformPulseIntentPostureObservation,
    ) -> bool {
        self.presentation_tick = self.presentation_tick.saturating_add(1);
        let outcome = shell.publish_native_intent_posture(posture, self.presentation_tick);
        let receipt =
            match finish_posture_publication(outcome, self.presentation_tick.saturating_add(1)) {
                Ok(receipt) => receipt,
                Err(denial) => {
                    let observation = self.publisher.intent_preparation_failure();
                    self.fail(
                        super::super::PlatformPulseTerminalError::IntentPosturePublication(denial),
                        observation,
                    );
                    return false;
                }
            };
        let Some(mounted) = receipt.mounted_publication() else {
            self.fail_intent_settlement("intent posture receipt omitted mounted publication");
            return false;
        };
        if let Err(error) = self
            .publisher
            .intent_posture_published(observation, mounted)
        {
            self.fail(
                super::super::PlatformPulseTerminalError::ObservationPublication,
                Err(error),
            );
            return false;
        }
        if let Err(denial) = self.visual_identity.refresh_after_content_rebind(
            shell,
            self.presentation_tick,
            std::time::Instant::now(),
        ) {
            self.fail_visual_identity(denial);
            return false;
        }
        true
    }
}

fn is_unrouted_interaction(stop: &WorthUiNativeIntentStop) -> bool {
    matches!(
        stop,
        WorthUiNativeIntentStop::Route(
            worth_ui::facade::intent::UiIntentRouteResolutionStop::Unrouted { .. }
        )
    )
}

fn stopped_posture_observation(
    kind: WorthUiNativeIntentPostureKind,
) -> Option<PlatformPulseIntentPostureObservation> {
    match kind {
        WorthUiNativeIntentPostureKind::Denied => {
            Some(PlatformPulseIntentPostureObservation::Denied)
        }
        WorthUiNativeIntentPostureKind::StaleConfirmation => {
            Some(PlatformPulseIntentPostureObservation::StaleConfirmation)
        }
        WorthUiNativeIntentPostureKind::Admitted
        | WorthUiNativeIntentPostureKind::ConfirmationRequired
        | WorthUiNativeIntentPostureKind::Completed
        | WorthUiNativeIntentPostureKind::Cancelled => None,
    }
}

fn finish_posture_publication(
    outcome: WorthUiNativeIntentPosturePublicationOutcome<'_>,
    completion_tick: u64,
) -> Result<
    worth_ui::facade::rebind::UiRebindReceipt,
    super::PlatformPulseIntentPosturePublicationDenial,
> {
    let outcome = match outcome {
        WorthUiNativeIntentPosturePublicationOutcome::InFlight(completion) => {
            completion.complete(completion_tick)
        }
        outcome => outcome,
    };
    match outcome {
        WorthUiNativeIntentPosturePublicationOutcome::Published(receipt) => Ok(receipt),
        WorthUiNativeIntentPosturePublicationOutcome::Indeterminate(recovery) => {
            let _ = recovery.into_session_for_shutdown();
            Err(super::PlatformPulseIntentPosturePublicationDenial::Indeterminate)
        }
        WorthUiNativeIntentPosturePublicationOutcome::InFlight(completion) => {
            drop(completion.dispose());
            Err(super::PlatformPulseIntentPosturePublicationDenial::RemainedInFlight)
        }
        WorthUiNativeIntentPosturePublicationOutcome::Stopped(stop) => Err(
            super::PlatformPulseIntentPosturePublicationDenial::Stopped(stop),
        ),
        WorthUiNativeIntentPosturePublicationOutcome::InternalDefect(defect) => {
            Err(super::PlatformPulseIntentPosturePublicationDenial::InternalDefect(defect))
        }
    }
}

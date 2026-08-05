use worth_ui::facade::app::{
    WorthUiNativeApplicationShell, WorthUiNativeIntentTerminalPostureOutcome,
};
use worth_ui::facade::intent::{
    UiIntentConsequencePublicationOutcome, UiIntentExecutionAdvanceOutcome,
    UiIntentExecutionTransition, UiIntentExecutionTransitionPosture,
};
use worth_ui_platform_pulse::observation_contract::PlatformPulseIntentPostureObservation;

use super::super::{
    PlatformPulseNativeFrame, PlatformPulsePendingQueryAction, PlatformPulseTerminalError,
};

enum PlatformPulseIntentTransitionContinuation {
    ContinueToConsequence,
    Finished(bool),
}

impl PlatformPulseNativeFrame {
    pub(in crate::native_frame) fn advance_intent_execution(&mut self) {
        let reading = match self.intent_clock.read() {
            Ok(reading) => reading,
            Err(denial) => {
                self.fail_intent_clock(denial);
                return;
            }
        };
        let Some(mut shell) = self.shell.take() else {
            return;
        };
        match shell.advance_native_intent_executions(reading) {
            UiIntentExecutionAdvanceOutcome::Stopped(stop) => {
                self.fail(
                    PlatformPulseTerminalError::IntentExecution(format!(
                        "execution clock stopped: {stop:?}"
                    )),
                    self.publisher.intent_preparation_failure(),
                );
            }
            UiIntentExecutionAdvanceOutcome::Advanced(report) => {
                if !self.publish_intent_executor_starts(&report) {
                    self.shell = Some(shell);
                    return;
                }
                for transition in report.into_transitions() {
                    if !self.finish_intent_transition(&mut shell, transition) {
                        break;
                    }
                }
            }
        }
        self.shell = Some(shell);
    }

    fn publish_intent_executor_starts(
        &mut self,
        report: &worth_ui::facade::intent::UiIntentExecutionAdvanceReport,
    ) -> bool {
        for observation in report.transitions().iter().filter_map(|transition| {
            worth_ui_platform_pulse::observation_contract::
                PlatformPulseIntentExecutorStartedObservation::from_transition(report, transition)
        }) {
            if let Err(error) = self.publisher.intent_executor_started(observation) {
                self.fail(
                    PlatformPulseTerminalError::ObservationPublication,
                    Err(error),
                );
                return false;
            }
        }
        true
    }

    fn finish_intent_transition(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        transition: UiIntentExecutionTransition,
    ) -> bool {
        let attempt = transition.attempt();
        let idempotency = transition.idempotency();
        match self.finish_terminal_intent_posture(shell, &transition, attempt, idempotency) {
            PlatformPulseIntentTransitionContinuation::Finished(finished) => return finished,
            PlatformPulseIntentTransitionContinuation::ContinueToConsequence => {}
        }
        if !matches!(
            transition.posture(),
            UiIntentExecutionTransitionPosture::Completed { .. }
        ) {
            return true;
        }
        let Some(consequence) = transition.into_consequence() else {
            self.fail_intent_settlement("completed transition omitted its consequence handle");
            return false;
        };
        self.presentation_tick = self.presentation_tick.saturating_add(1);
        let outcome = shell.publish_native_intent_consequences(consequence, self.presentation_tick);
        let receipt =
            match finish_consequence_publication(outcome, self.presentation_tick.saturating_add(1))
            {
                Ok(receipt) => receipt,
                Err(detail) => {
                    self.fail_intent_settlement(detail);
                    return false;
                }
            };
        self.finish_action_query_publication(shell, attempt, idempotency, receipt)
    }

    fn finish_terminal_intent_posture(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        transition: &UiIntentExecutionTransition,
        attempt: worth_ui::facade::intent::UiIntentExecutionAttemptIdentity,
        idempotency: worth_ui::facade::intent::UiIntentExecutionIdempotencyIdentity,
    ) -> PlatformPulseIntentTransitionContinuation {
        match shell.prepare_native_intent_terminal_posture(transition) {
            WorthUiNativeIntentTerminalPostureOutcome::Prepared(posture) => {
                let observation = match transition.posture() {
                    UiIntentExecutionTransitionPosture::CancelledBeforeEffect { .. } => {
                        PlatformPulseIntentPostureObservation::cancelled(attempt, idempotency)
                    }
                    UiIntentExecutionTransitionPosture::RejectedBeforeEffect { .. }
                    | UiIntentExecutionTransitionPosture::FailedBeforeEffect { .. }
                    | UiIntentExecutionTransitionPosture::TimedOutBeforeEffect { .. } => {
                        PlatformPulseIntentPostureObservation::Denied
                    }
                    _ => {
                        self.fail_intent_settlement(
                            "terminal execution posture disagreed with its transition",
                        );
                        return PlatformPulseIntentTransitionContinuation::Finished(false);
                    }
                };
                if !self.publish_native_intent_posture(shell, posture, observation) {
                    return PlatformPulseIntentTransitionContinuation::Finished(false);
                }
                if self
                    .intent_evidence_index
                    .retire_execution(attempt, idempotency)
                    .is_none()
                {
                    self.fail_intent_settlement(
                        "terminal attempt omitted its retained intent evidence reference",
                    );
                    return PlatformPulseIntentTransitionContinuation::Finished(false);
                }
                PlatformPulseIntentTransitionContinuation::Finished(true)
            }
            WorthUiNativeIntentTerminalPostureOutcome::MissingExecutionBasis => {
                self.fail_intent_settlement("terminal execution omitted its mounted target basis");
                PlatformPulseIntentTransitionContinuation::Finished(false)
            }
            WorthUiNativeIntentTerminalPostureOutcome::PostureIdentityExhausted => {
                self.fail_intent_settlement("terminal execution posture identity exhausted");
                PlatformPulseIntentTransitionContinuation::Finished(false)
            }
            WorthUiNativeIntentTerminalPostureOutcome::RecoveryRetained
            | WorthUiNativeIntentTerminalPostureOutcome::NotTerminal => {
                PlatformPulseIntentTransitionContinuation::ContinueToConsequence
            }
        }
    }

    fn finish_action_query_publication(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        attempt: worth_ui::facade::intent::UiIntentExecutionAttemptIdentity,
        idempotency: worth_ui::facade::intent::UiIntentExecutionIdempotencyIdentity,
        receipt: worth_ui::facade::rebind::UiRebindReceipt,
    ) -> bool {
        let Some(pending) = self.take_completed_query_action(attempt, idempotency) else {
            return false;
        };
        let Some(trace) = self.resolve_completed_trace(shell, &pending, attempt, idempotency)
        else {
            return false;
        };
        let Some(mounted) = receipt.mounted_publication() else {
            self.fail_intent_settlement("intent consequence receipt omitted mounted publication");
            return false;
        };
        if !self.publish_query_projection_evidence(&pending.projection, mounted) {
            return false;
        }
        let posture = PlatformPulseIntentPostureObservation::completed(attempt, idempotency);
        if let Err(error) = self.publisher.intent_posture_published(posture, mounted) {
            self.fail(
                PlatformPulseTerminalError::ObservationPublication,
                Err(error),
            );
            return false;
        }
        if !self.publish_completed_trace(trace, &pending.projection, mounted) {
            return false;
        }
        if let Err(denial) = self.refresh_query_visual_identity(shell, &pending.projection) {
            self.fail_visual_identity(denial);
            return false;
        }
        self.admit_query_predecessor(receipt)
    }

    fn take_completed_query_action(
        &mut self,
        attempt: worth_ui::facade::intent::UiIntentExecutionAttemptIdentity,
        idempotency: worth_ui::facade::intent::UiIntentExecutionIdempotencyIdentity,
    ) -> Option<PlatformPulsePendingQueryAction> {
        let index = self
            .pending_query_actions
            .iter()
            .position(|pending| pending.reference.matches_execution(attempt, idempotency));
        let Some(index) = index else {
            self.fail_intent_settlement("completed attempt has no product-issued Query evidence");
            return None;
        };
        Some(self.pending_query_actions.remove(index))
    }

    fn resolve_completed_trace(
        &mut self,
        shell: &WorthUiNativeApplicationShell,
        pending: &PlatformPulsePendingQueryAction,
        attempt: worth_ui::facade::intent::UiIntentExecutionAttemptIdentity,
        idempotency: worth_ui::facade::intent::UiIntentExecutionIdempotencyIdentity,
    ) -> Option<worth_ui::facade::inspection::UiIntentCausalTraceEvidence> {
        let trace = match shell.lookup_intent_causal_trace(pending.evidence_reference) {
            worth_ui::facade::inspection::UiIntentEvidenceLookup::Found(trace)
                if trace.is_complete_through_product_outcome() =>
            {
                trace
            }
            worth_ui::facade::inspection::UiIntentEvidenceLookup::Found(_) => {
                self.fail_intent_settlement(
                    "completed product action resolved an incomplete intent causal trace",
                );
                return None;
            }
            posture => {
                self.fail_intent_settlement(format!(
                    "completed product action lost its intent causal trace: {posture:?}"
                ));
                return None;
            }
        };
        if self
            .intent_evidence_index
            .retire_execution(attempt, idempotency)
            != Some(trace.reference())
        {
            self.fail_intent_settlement(
                "completed product action substituted its retained intent evidence reference",
            );
            return None;
        }
        Some(trace)
    }

    fn publish_completed_trace(
        &mut self,
        trace: worth_ui::facade::inspection::UiIntentCausalTraceEvidence,
        projection: &worth_ui_platform_pulse::observation_contract::PlatformPulseQueryProjectionEvidence,
        mounted: &worth_ui::facade::app::UiMountedFramePublicationReceipt,
    ) -> bool {
        let causal_trace = match worth_ui_platform_pulse::observation_contract::
            PlatformPulseIntentCausalTraceObservation::from_completed_publication(
                trace, projection, mounted,
            ) {
            Ok(trace) => trace,
            Err(denial) => {
                self.fail_intent_settlement(format!(
                    "completed intent causal trace projection stopped: {denial:?}"
                ));
                return false;
            }
        };
        if let Err(error) = self.publisher.intent_causal_trace(causal_trace) {
            self.fail(
                PlatformPulseTerminalError::ObservationPublication,
                Err(error),
            );
            return false;
        }
        true
    }

    fn admit_query_predecessor(
        &mut self,
        receipt: worth_ui::facade::rebind::UiRebindReceipt,
    ) -> bool {
        let observation = match receipt.release_scalar_projection_observation() {
            Ok(observation) => observation,
            Err(_) => {
                self.fail_intent_settlement(
                    "intent consequence receipt omitted scalar Query predecessor",
                );
                return false;
            }
        };
        let admission = self
            .query_lifecycle
            .as_mut()
            .expect("prepared Pulse retains its Query lifecycle")
            .admit_publication(observation);
        if let Err(denial) = admission {
            self.fail(
                PlatformPulseTerminalError::QueryLifecycle(denial),
                self.publisher.intent_preparation_failure(),
            );
            return false;
        }
        true
    }
}

fn finish_consequence_publication(
    outcome: UiIntentConsequencePublicationOutcome<'_>,
    completion_tick: u64,
) -> Result<worth_ui::facade::rebind::UiRebindReceipt, String> {
    let outcome = match outcome {
        UiIntentConsequencePublicationOutcome::InFlight(completion) => {
            completion.complete(completion_tick)
        }
        outcome => outcome,
    };
    match outcome {
        UiIntentConsequencePublicationOutcome::Published(receipt) => Ok(receipt),
        UiIntentConsequencePublicationOutcome::Indeterminate(recovery) => {
            let _ = recovery.into_session_for_shutdown();
            Err("intent consequence publication became indeterminate".to_owned())
        }
        UiIntentConsequencePublicationOutcome::InFlight(completion) => {
            drop(completion.dispose());
            Err("intent consequence publication remained in flight".to_owned())
        }
        UiIntentConsequencePublicationOutcome::NoConsequences(_) => {
            Err("completed Pulse action produced no declared consequences".to_owned())
        }
        UiIntentConsequencePublicationOutcome::Stopped(stop) => Err(format!(
            "intent consequence publication stopped: {:?}",
            stop.reason()
        )),
        UiIntentConsequencePublicationOutcome::InternalDefect(_) => {
            Err("intent consequence publication reached an internal defect".to_owned())
        }
    }
}

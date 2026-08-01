use crate::adjudication::{NativeControlPixelRegion, PlatformPulseActionControlPoint};
use crate::external_observation::{NativeClientPixelCapture, NativeClientPixelPoint};
use crate::source_delta::{
    ConfirmationHeldIntentDelta, ConfirmationReleasedIntentDelta, DeniedIntentDelta,
    DisabledIntentDelta, FinalHeldIntentDelta, IntentRouteRemovalSourceDelta,
    ReadyReleasedIntentDelta,
};
use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseIntentExecutorGateObservation as Gate,
    PlatformPulseIntentOperabilityObservation as Operability,
};

use super::evidence::PlatformPulseIntentJourneyEvidenceBuilder;
use super::observation::{self, ExpectedTerminalPosture};
use super::states::{
    Cancelled, ConfirmationPending, ConfirmationStale, DisabledStopped, FinalHeld, FirstCompleted,
    FirstHeld, FreshConfirmationPending, PolicyDeniedStopped, Ready, SecondCompleted,
};
use super::PlatformPulseIntentJourneyFailure;
use crate::product_process::NativeBoundExecutableWorld;

mod cancellation;

use cancellation::await_rebind_cancellation;

pub(super) fn run(
    world: &mut NativeBoundExecutableWorld,
    baseline: &NativeClientPixelCapture,
    action: PlatformPulseActionControlPoint,
    evidence: &mut PlatformPulseIntentJourneyEvidenceBuilder,
    route_removal: IntentRouteRemovalSourceDelta,
) -> Result<(), PlatformPulseIntentJourneyFailure> {
    let ready = IntentJourney {
        world,
        baseline,
        action,
        evidence,
        state: Ready,
    };
    let first_held = ready.activate_first()?;
    let first_completed = first_held.release_first(ReadyReleasedIntentDelta)?;
    let confirmation = first_completed.require_confirmation(ConfirmationHeldIntentDelta)?;
    let stale = confirmation.stale_confirmation(ConfirmationReleasedIntentDelta)?;
    let fresh = stale.obtain_fresh_confirmation()?;
    let second_completed = fresh.confirm_and_complete()?;
    let disabled = second_completed.stop_disabled(DisabledIntentDelta)?;
    let denied = disabled.stop_policy_denied(DeniedIntentDelta)?;
    let final_held = denied.activate_final_held(FinalHeldIntentDelta)?;
    let cancelled = final_held.cancel_through_rebind(route_removal)?;
    let IntentJourney {
        state: Cancelled, ..
    } = cancelled;
    Ok(())
}

pub(super) fn run_causal_pulse(
    world: &mut NativeBoundExecutableWorld,
    baseline: &NativeClientPixelCapture,
    action: PlatformPulseActionControlPoint,
    evidence: &mut PlatformPulseIntentJourneyEvidenceBuilder,
) -> Result<(), PlatformPulseIntentJourneyFailure> {
    let ready = IntentJourney {
        world,
        baseline,
        action,
        evidence,
        state: Ready,
    };
    let first_held = ready.activate_first()?;
    let IntentJourney {
        state: FirstCompleted,
        ..
    } = first_held.release_first(ReadyReleasedIntentDelta)?;
    Ok(())
}

struct IntentJourney<'a, State> {
    world: &'a mut NativeBoundExecutableWorld,
    baseline: &'a NativeClientPixelCapture,
    action: PlatformPulseActionControlPoint,
    evidence: &'a mut PlatformPulseIntentJourneyEvidenceBuilder,
    state: State,
}

impl<State> IntentJourney<'_, State> {
    fn activate(
        &mut self,
        point: NativeClientPixelPoint,
    ) -> Result<(), PlatformPulseIntentJourneyFailure> {
        observation::activate_native_control(self.world, point)?;
        self.evidence.record_native_activation();
        Ok(())
    }

    fn visible(
        &mut self,
        region: NativeControlPixelRegion,
    ) -> Result<(), PlatformPulseIntentJourneyFailure> {
        let change = observation::capture_visible_change(self.world, self.baseline, region)?;
        self.evidence.record_visible_change(change);
        Ok(())
    }

    fn visible_completed(
        &mut self,
        region: NativeControlPixelRegion,
    ) -> Result<(), PlatformPulseIntentJourneyFailure> {
        let change = observation::capture_visible_change(self.world, self.baseline, region)?;
        if !self.evidence.record_causal_visible_change(change) {
            return Err(PlatformPulseIntentJourneyFailure::EvidenceOrder(
                "completed visible change had no exact pending causal trace",
            ));
        }
        Ok(())
    }

    fn record_refresh(&mut self) -> Result<(), PlatformPulseIntentJourneyFailure> {
        let sequence = observation::await_visual_refresh(self.world)?;
        self.evidence.record_sequence(sequence);
        Ok(())
    }

    fn record_rebase(&mut self) -> Result<(), PlatformPulseIntentJourneyFailure> {
        let sequence = observation::await_visual_rebase(self.world)?;
        self.evidence.record_sequence(sequence);
        Ok(())
    }

    fn record_query_completion(
        &mut self,
        attempt: worth_ui_platform_pulse::observation_contract::PlatformPulseIntentAttemptObservationReference,
        action_input_revision: u64,
        query_source_revision: u64,
        status: &str,
    ) -> Result<(), PlatformPulseIntentJourneyFailure> {
        let trace = observation::await_query_completion(
            self.world,
            attempt,
            action_input_revision,
            query_source_revision,
            status,
        )?;
        if !self.evidence.record_causal_trace(trace.value) {
            return Err(PlatformPulseIntentJourneyFailure::EvidenceOrder(
                "completed intent overlapped an unobserved causal pixel edge",
            ));
        }
        self.evidence.record_query_action();
        self.evidence.record_completion();
        self.evidence.record_sequence(trace.sequence);
        Ok(())
    }
}

impl<'a> IntentJourney<'a, Ready> {
    fn activate_first(
        mut self,
    ) -> Result<IntentJourney<'a, FirstHeld>, PlatformPulseIntentJourneyFailure> {
        self.activate(self.action.point())?;
        let admitted = observation::await_admitted(self.world)?;
        self.evidence.record_sequence(admitted.sequence);
        let started = observation::await_executor_started(self.world, admitted.value)?;
        self.evidence.record_provider_start();
        self.evidence.record_sequence(started);
        self.record_rebase()?;
        self.visible(self.action.region())?;
        self.evidence.record_first_attempt(admitted.value);
        Ok(self.with_state(FirstHeld {
            attempt: admitted.value,
        }))
    }
}

impl<'a> IntentJourney<'a, FirstHeld> {
    fn release_first(
        mut self,
        delta: ReadyReleasedIntentDelta,
    ) -> Result<IntentJourney<'a, FirstCompleted>, PlatformPulseIntentJourneyFailure> {
        delta
            .apply(&self.world.installation)
            .map_err(PlatformPulseIntentJourneyFailure::SourceAction)?;
        self.evidence.record_source_action();
        let sequence =
            observation::await_intent_input(self.world, 2, Operability::Ready, Gate::Released)?;
        self.evidence.record_sequence(sequence);
        self.record_query_completion(self.state.attempt, 1, 2, "ACTION 1")?;
        self.record_refresh()?;
        self.visible_completed(self.action.region())?;
        Ok(self.with_state(FirstCompleted))
    }
}

impl<'a> IntentJourney<'a, FirstCompleted> {
    fn require_confirmation(
        mut self,
        delta: ConfirmationHeldIntentDelta,
    ) -> Result<IntentJourney<'a, ConfirmationPending>, PlatformPulseIntentJourneyFailure> {
        delta
            .apply(&self.world.installation)
            .map_err(PlatformPulseIntentJourneyFailure::SourceAction)?;
        self.evidence.record_source_action();
        let sequence = observation::await_intent_input(
            self.world,
            3,
            Operability::ConfirmationRequired,
            Gate::Held,
        )?;
        self.evidence.record_sequence(sequence);
        self.activate(self.action.point())?;
        let challenge = observation::await_confirmation_required(self.world)?;
        self.evidence.record_sequence(challenge.sequence);
        self.record_refresh()?;
        let (control, change) =
            observation::capture_visible_confirmation(self.world, self.baseline, self.action)?;
        self.evidence.record_visible_change(change);
        Ok(self.with_state(ConfirmationPending {
            challenge: challenge.value,
            control,
        }))
    }
}

impl<'a> IntentJourney<'a, ConfirmationPending> {
    fn stale_confirmation(
        mut self,
        delta: ConfirmationReleasedIntentDelta,
    ) -> Result<IntentJourney<'a, ConfirmationStale>, PlatformPulseIntentJourneyFailure> {
        delta
            .apply(&self.world.installation)
            .map_err(PlatformPulseIntentJourneyFailure::SourceAction)?;
        self.evidence.record_source_action();
        let sequence = observation::await_intent_input(
            self.world,
            4,
            Operability::ConfirmationRequired,
            Gate::Released,
        )?;
        self.evidence.record_sequence(sequence);
        self.activate(self.state.control.point())?;
        let sequence = observation::await_terminal_posture(
            self.world,
            ExpectedTerminalPosture::StaleConfirmation,
        )?;
        self.evidence.record_sequence(sequence);
        self.record_refresh()?;
        self.visible(self.state.control.region())?;
        let predecessor = self.state.challenge;
        let control = self.state.control;
        Ok(self.with_state(ConfirmationStale {
            predecessor,
            control,
        }))
    }
}

impl<'a> IntentJourney<'a, ConfirmationStale> {
    fn obtain_fresh_confirmation(
        mut self,
    ) -> Result<IntentJourney<'a, FreshConfirmationPending>, PlatformPulseIntentJourneyFailure>
    {
        self.activate(self.action.point())?;
        let challenge = observation::await_confirmation_required(self.world)?;
        if challenge.value == self.state.predecessor {
            return Err(PlatformPulseIntentJourneyFailure::Cancellation(
                "fresh activation replayed the stale confirmation challenge",
            ));
        }
        self.evidence.record_sequence(challenge.sequence);
        self.record_refresh()?;
        self.visible(self.state.control.region())?;
        let control = self.state.control;
        Ok(self.with_state(FreshConfirmationPending {
            challenge: challenge.value,
            control,
        }))
    }
}

impl<'a> IntentJourney<'a, FreshConfirmationPending> {
    fn confirm_and_complete(
        mut self,
    ) -> Result<IntentJourney<'a, SecondCompleted>, PlatformPulseIntentJourneyFailure> {
        if self.state.challenge.expires_at_tick == 0 {
            return Err(PlatformPulseIntentJourneyFailure::Cancellation(
                "fresh challenge carried no expiry boundary",
            ));
        }
        self.activate(self.state.control.point())?;
        let admitted = observation::await_admitted(self.world)?;
        self.evidence.record_sequence(admitted.sequence);
        let sequence = observation::await_executor_started(self.world, admitted.value)?;
        self.evidence.record_provider_start();
        self.evidence.record_sequence(sequence);
        self.record_query_completion(admitted.value, 4, 2, "ACTION 4")?;
        self.evidence.record_second_attempt(admitted.value);
        self.record_refresh()?;
        self.visible_completed(self.action.region())?;
        Ok(self.with_state(SecondCompleted))
    }
}

impl<'a> IntentJourney<'a, SecondCompleted> {
    fn stop_disabled(
        mut self,
        delta: DisabledIntentDelta,
    ) -> Result<IntentJourney<'a, DisabledStopped>, PlatformPulseIntentJourneyFailure> {
        delta
            .apply(&self.world.installation)
            .map_err(PlatformPulseIntentJourneyFailure::SourceAction)?;
        self.evidence.record_source_action();
        let sequence =
            observation::await_intent_input(self.world, 5, Operability::Disabled, Gate::Released)?;
        self.evidence.record_sequence(sequence);
        self.activate(self.action.point())?;
        let sequence =
            observation::await_terminal_posture(self.world, ExpectedTerminalPosture::Denied)?;
        self.evidence.record_sequence(sequence);
        self.record_refresh()?;
        self.visible(self.action.region())?;
        Ok(self.with_state(DisabledStopped))
    }
}

impl<'a> IntentJourney<'a, DisabledStopped> {
    fn stop_policy_denied(
        mut self,
        delta: DeniedIntentDelta,
    ) -> Result<IntentJourney<'a, PolicyDeniedStopped>, PlatformPulseIntentJourneyFailure> {
        delta
            .apply(&self.world.installation)
            .map_err(PlatformPulseIntentJourneyFailure::SourceAction)?;
        self.evidence.record_source_action();
        let sequence =
            observation::await_intent_input(self.world, 6, Operability::Denied, Gate::Released)?;
        self.evidence.record_sequence(sequence);
        self.activate(self.action.point())?;
        let sequence =
            observation::await_terminal_posture(self.world, ExpectedTerminalPosture::Denied)?;
        self.evidence.record_sequence(sequence);
        self.record_refresh()?;
        self.visible(self.action.region())?;
        Ok(self.with_state(PolicyDeniedStopped))
    }
}

impl<'a> IntentJourney<'a, PolicyDeniedStopped> {
    fn activate_final_held(
        mut self,
        delta: FinalHeldIntentDelta,
    ) -> Result<IntentJourney<'a, FinalHeld>, PlatformPulseIntentJourneyFailure> {
        delta
            .apply(&self.world.installation)
            .map_err(PlatformPulseIntentJourneyFailure::SourceAction)?;
        self.evidence.record_source_action();
        let sequence =
            observation::await_intent_input(self.world, 7, Operability::Ready, Gate::Held)?;
        self.evidence.record_sequence(sequence);
        self.activate(self.action.point())?;
        let admitted = observation::await_admitted(self.world)?;
        self.evidence.record_sequence(admitted.sequence);
        let sequence = observation::await_executor_started(self.world, admitted.value)?;
        self.evidence.record_provider_start();
        self.evidence.record_sequence(sequence);
        self.record_refresh()?;
        self.visible(self.action.region())?;
        Ok(self.with_state(FinalHeld {
            attempt: admitted.value,
        }))
    }
}

impl<'a> IntentJourney<'a, FinalHeld> {
    fn cancel_through_rebind(
        mut self,
        delta: IntentRouteRemovalSourceDelta,
    ) -> Result<IntentJourney<'a, Cancelled>, PlatformPulseIntentJourneyFailure> {
        delta
            .apply(&self.world.installation)
            .map_err(PlatformPulseIntentJourneyFailure::SourceAction)?;
        self.evidence.record_source_action();
        let sequence = await_rebind_cancellation(self.world, self.state.attempt)?;
        self.evidence.record_sequence(sequence);
        self.evidence.record_cancelled_attempt(self.state.attempt);
        self.record_rebase()?;
        self.visible(self.action.region())?;
        Ok(self.with_state(Cancelled))
    }
}

impl<'a, State> IntentJourney<'a, State> {
    fn with_state<Next>(self, state: Next) -> IntentJourney<'a, Next> {
        IntentJourney {
            world: self.world,
            baseline: self.baseline,
            action: self.action,
            evidence: self.evidence,
            state,
        }
    }
}

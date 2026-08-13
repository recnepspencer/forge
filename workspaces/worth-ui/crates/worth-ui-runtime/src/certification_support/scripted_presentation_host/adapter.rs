use std::collections::VecDeque;

use crate::facade::host::{
    UiHostAdapterSessionAuthority, UiHostSessionReleaseOutcome, UiHostSessionReleaseReceipt,
    WorthUiOperationalHostAdapter,
};
use worth_ui_host_contract::WorthUiHostContract;

use super::*;

impl WorthUiOperationalHostAdapter for ScriptedPresentationHost {
    fn operational_host_contract(&self) -> WorthUiHostContract {
        self.state.lock().unwrap().contract
    }

    fn operational_capability_report(&self) -> WorthUiHostCapabilityReport {
        self.state.lock().unwrap().capabilities.clone()
    }

    fn operational_protocol_contract(&self) -> worth_ui_host_contract::UiHostProtocolContract {
        self.state.lock().unwrap().protocol
    }

    fn drain_host_observations(
        &self,
        authority: &UiHostAdapterSessionAuthority,
    ) -> Result<
        worth_ui_host_contract::UiHostObservationDrain,
        worth_ui_host_contract::UiHostObservationDrainDenial,
    > {
        Ok(self
            .observation_retention
            .drain(authority.host_session_identity()))
    }

    fn measurement_environment_report(
        &self,
    ) -> worth_ui_host_contract::UiHostMeasurementEnvironmentReport {
        let state = self.state.lock().unwrap();
        let supports = |capability| state.capabilities.supports(capability);
        worth_ui_host_contract::UiHostMeasurementEnvironmentReport::new(
            supports(worth_ui_host_contract::WorthUiHostCapability::ViewportObservation)
                .then_some(state.viewport_environment_generation),
            supports(worth_ui_host_contract::WorthUiHostCapability::DpiObservation)
                .then_some(state.viewport_environment_generation),
            (supports(worth_ui_host_contract::WorthUiHostCapability::TextIntrinsicMeasurement)
                || supports(
                    worth_ui_host_contract::WorthUiHostCapability::TextBaselineMeasurement,
                )
                || supports(worth_ui_host_contract::WorthUiHostCapability::FontMetrics))
            .then_some(state.font_environment_generation),
            (supports(
                worth_ui_host_contract::WorthUiHostCapability::NativeControlIntrinsicMeasurement,
            ) || supports(worth_ui_host_contract::WorthUiHostCapability::PortalAnchorObservation)
                || supports(
                    worth_ui_host_contract::WorthUiHostCapability::ScrollContainerObservation,
                ))
            .then_some(state.adapter_environment_generation),
        )
    }

    fn visual_capture_capability(&self) -> worth_ui_host_contract::UiHostCaptureCapability {
        self.state.lock().unwrap().visual_capture_capability
    }

    fn capture_visual_presentation(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
        if !authority.admits_visual_capture(request) {
            return worth_ui_host_contract::UiHostCaptureObservationOutcome::Unsupported;
        }
        let mut state = self.state.lock().unwrap();
        state.visual_capture_calls.push(request);
        let script = state
            .visual_captures
            .pop_front()
            .expect("script names every visual capture outcome");
        visual_capture_script::observe(request, script)
    }

    fn cancel_visual_capture(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> worth_ui_host_contract::UiHostCaptureCancellationOutcome {
        if !authority.admits_visual_capture(request) {
            return worth_ui_host_contract::UiHostCaptureCancellationOutcome::CleanupIndeterminate;
        }
        let mut state = self.state.lock().unwrap();
        state.visual_cancellation_calls.push(request);
        state.visual_cancellation_outcome
    }

    fn register_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceRegistrationOutcome {
        if request.host_session_identity() != authority.host_session_identity() {
            return worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        let mut state = self.state.lock().unwrap();
        if state
            .registrations
            .contains_key(&request.host_surface_identity())
        {
            return worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        state
            .registrations
            .insert(request.host_surface_identity(), request);
        if std::mem::take(&mut state.indeterminate_next_registration) {
            return worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RegistrationIndeterminate(
                worth_ui_host_contract::UiHostSurfaceRegistrationIndeterminate::after_effects_may_have_begun(request),
            );
        }
        worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RegisteredKnownEmpty
    }

    fn deregister_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome {
        if request.host_session_identity() != authority.host_session_identity() {
            return worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        let mut state = self.state.lock().unwrap();
        if state.registrations.remove(&request.host_surface_identity()) != Some(request) {
            return worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        if std::mem::take(&mut state.wrong_next_deregistration_receipt) {
            return worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::Deregistered(
                worth_ui_host_contract::UiHostSurfaceDeregistrationReceipt::from_runtime(
                    request.host_session_identity().wrapping_add(1),
                    request.host_surface_identity(),
                ),
            );
        }
        worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::Deregistered(
            worth_ui_host_contract::UiHostSurfaceDeregistrationReceipt::from_runtime(
                request.host_session_identity(),
                request.host_surface_identity(),
            ),
        )
    }

    fn present_mounted_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    ) -> UiHostSurfacePresentationOutcome {
        if !authority.admits_mounted_presentation(request) {
            return UiHostSurfacePresentationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfacePresentationDenial::SurfaceBindingChanged,
            );
        }
        let (outcome, queued_observation, queued_measurement) = {
            let mut state = self.state.lock().unwrap();
            state.presentation_calls += 1;
            state.observation_events.push("presentation-enter");
            let start = state
                .presentations
                .pop_front()
                .expect("script names every surface outcome");
            let outcome = match start {
                ScriptedPresentationStart::Outcome(outcome) => outcome,
                ScriptedPresentationStart::InFlight {
                    completions,
                    cancellation,
                } => {
                    let token = request.issue_completion_token();
                    let identity = token.diagnostic_value();
                    state.completions.insert(identity, completions);
                    state.cancellations.insert(identity, cancellation);
                    state
                        .token_sessions
                        .insert(identity, authority.host_session_identity());
                    UiHostSurfacePresentationOutcome::InFlight(token)
                }
            };
            (
                outcome,
                state.queued_observation.take(),
                state.queued_measurement.take(),
            )
        };
        dispatch_queued_ingress(self, queued_observation, queued_measurement);
        self.state
            .lock()
            .unwrap()
            .observation_events
            .push("presentation-exit");
        outcome
    }

    fn complete_mounted_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        token: worth_ui_host_contract::UiHostPresentationCompletionToken,
    ) -> UiHostSurfaceInFlightCompletion {
        if !authority.admits_mounted_completion_token(&token) {
            return UiHostSurfaceInFlightCompletion::PresentationIndeterminate;
        }
        let identity = token.diagnostic_value();
        let mut state = self.state.lock().unwrap();
        let completion = state
            .completions
            .get_mut(&identity)
            .and_then(VecDeque::pop_front);
        match completion {
            Some(ScriptedSurfaceCompletion::Pending) => {
                UiHostSurfaceInFlightCompletion::Pending(token)
            }
            Some(ScriptedSurfaceCompletion::RejectedBeforeEffects(denial)) => {
                clear_token_state(&mut state, identity);
                UiHostSurfaceInFlightCompletion::RejectedBeforeEffects(denial)
            }
            Some(ScriptedSurfaceCompletion::Presented(completion)) => {
                clear_token_state(&mut state, identity);
                UiHostSurfaceInFlightCompletion::Presented(completion)
            }
            Some(ScriptedSurfaceCompletion::PresentationIndeterminate) | None => {
                clear_token_state(&mut state, identity);
                UiHostSurfaceInFlightCompletion::PresentationIndeterminate
            }
        }
    }

    fn cancel_mounted_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        token: worth_ui_host_contract::UiHostPresentationCompletionToken,
    ) -> UiHostSurfaceCancellationOutcome {
        if !authority.admits_mounted_completion_token(&token) {
            return UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun;
        }
        let identity = token.diagnostic_value();
        let mut state = self.state.lock().unwrap();
        state.cancellation_calls.push(identity);
        state.completions.remove(&identity);
        state.token_sessions.remove(&identity);
        state
            .cancellations
            .remove(&identity)
            .unwrap_or(UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun)
    }

    fn release_host_session(
        &self,
        authority: &UiHostAdapterSessionAuthority,
    ) -> UiHostSessionReleaseOutcome {
        let mut state = self.state.lock().unwrap();
        let before = state.registrations.len();
        state.registrations.retain(|_, request| {
            request.host_session_identity() != authority.host_session_identity()
        });
        let session_tokens = state
            .token_sessions
            .iter()
            .filter_map(|(token, session)| {
                (*session == authority.host_session_identity()).then_some(*token)
            })
            .collect::<Vec<_>>();
        for token in session_tokens {
            clear_token_state(&mut state, token);
        }
        self.observation_retention
            .release_session(authority.host_session_identity());
        UiHostSessionReleaseOutcome::Released(UiHostSessionReleaseReceipt::released(
            authority.host_session_identity(),
            before - state.registrations.len(),
        ))
    }
}

fn clear_token_state(state: &mut ScriptedPresentationState, identity: u64) {
    state.completions.remove(&identity);
    state.cancellations.remove(&identity);
    state.token_sessions.remove(&identity);
}

fn dispatch_queued_ingress(
    host: &ScriptedPresentationHost,
    observation: Option<crate::facade::observation_report::UiHostObservationBatch>,
    measurement: Option<(
        crate::facade::measurement_exchange::WorthUiHostMeasurementIngress,
        crate::facade::measurement_exchange::UiHostMeasurementCompletion,
    )>,
) {
    if let Some(batch) = observation {
        host.observation_retention
            .retain(batch)
            .expect("scripted in-call raw report fits adapter retention");
        host.state
            .lock()
            .unwrap()
            .observation_events
            .push("observation-enqueued");
    }
    if let Some((ingress, completion)) = measurement {
        ingress
            .enqueue(completion)
            .expect("scripted in-call measurement completion fits ingress");
        host.state
            .lock()
            .unwrap()
            .observation_events
            .push("measurement-enqueued");
    }
}

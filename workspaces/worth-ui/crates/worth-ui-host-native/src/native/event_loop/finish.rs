use std::rc::Rc;

use super::{
    terminal_cleanup_complete, UiNativeEventLoopApplication, UiNativeEventLoopCleanup,
    UiNativeEventLoopClient, UiNativeEventLoopRunDenial, UiNativeEventLoopRunReport,
    UiNativeEventLoopStopReport,
};
use crate::native::UiNativeGraphicsObservation;

struct UiNativeEventLoopCompletionEvidence {
    presentation: crate::native::UiNativePresentationObservation,
    graphics: UiNativeGraphicsObservation,
    client_attribution: super::UiNativeClientPresentationAttribution,
    peak_census: crate::native::UiNativeResourceCensus,
    terminal_census: crate::native::UiNativeResourceCensus,
    retained_frames: Vec<crate::native::UiNativeRetainedFrameObservation>,
    peak_text_pins: Box<[crate::native::text_atlas::UiNativeTextPinObservation]>,
    text_pin_frame_counts: Box<[u32]>,
    text_pin_frame_observations:
        Box<[Box<[crate::native::text_atlas::UiNativeTextPinObservation]>]>,
    text_atlas_model_frame_digests: Box<[[u8; 32]]>,
    text_atlas_plan_observations:
        Box<[crate::native::text_atlas::UiNativeTextAtlasPlanObservation]>,
    physical_signal_transition_observations:
        Box<[crate::native::physical_work_signal::UiNativePhysicalSignalTransitionObservation]>,
    physical_signal_transition_trace_complete: bool,
    physical_signal_lifecycle: crate::native::UiNativePhysicalSignalLifecycleObservation,
    observation_history_complete: bool,
    text_atlas_transactions: u64,
    derived_state_reconstruction: Option<crate::UiNativeDerivedStateReconstructionObservation>,
    client_shutdown: Option<super::UiNativeClientShutdownObservation>,
}

impl<Client: UiNativeEventLoopClient> UiNativeEventLoopApplication<Client> {
    pub(super) fn finish(
        mut self,
    ) -> Result<UiNativeEventLoopRunReport, UiNativeEventLoopStopReport> {
        let presentation = self.shared.borrow().last_presentation.clone();
        let retained_frames = self.shared.borrow().retained_frame_observations.clone();
        let client_attribution = self
            .client
            .as_ref()
            .and_then(UiNativeEventLoopClient::presentation_attribution);
        let host_peak_census = self.shared.borrow().compiler_total_peak();
        let peak_text_pins = self.shared.borrow().peak_text_pins.clone();
        let text_pin_frame_counts = self.shared.borrow().text_pin_frame_counts.clone();
        let text_pin_frame_observations = self.shared.borrow().text_pin_frame_observations.clone();
        let text_atlas_model_frame_digests =
            self.shared.borrow().text_atlas_model_frame_digests.clone();
        let text_atlas_plan_observations =
            self.shared.borrow().text_atlas_plan_observations.clone();
        let physical_signal_transition_observations = self
            .shared
            .borrow()
            .physical_signal
            .transition_observations()
            .to_vec();
        let physical_signal_transition_trace_complete = self
            .shared
            .borrow()
            .physical_signal
            .transition_observation_trace_complete();
        let physical_signal_lifecycle =
            self.shared.borrow().physical_signal.lifecycle_observation();
        let observation_history_complete = !self.shared.borrow().observation_history_overflowed;
        let text_atlas_transactions = self.shared.borrow().text_atlas.committed_transactions();
        let derived_state_reconstruction = self
            .shared
            .borrow()
            .certified_derived_state_reconstruction();
        let effect_posture = self.shared.borrow().effect_posture;
        let graphics = self
            .shared
            .borrow()
            .graphics
            .as_ref()
            .map(|graphics| UiNativeGraphicsObservation::from_graphics(graphics));
        let (client_cleanup, client_shutdown) = self
            .client
            .take()
            .map(|client| client.close().into_parts())
            .unwrap_or((None, None));
        let client_closed = client_cleanup.is_none();
        let peak_census = client_shutdown.as_ref().map_or(host_peak_census, |client| {
            host_peak_census.with_client_peak(client.resources())
        });
        let readiness_owner_count = self.readiness.close();
        let mut shared = self.shared.borrow_mut();
        shared
            .resources
            .release_all(self.loop_resources.drain(..))
            .expect("event-loop owners must remain exact");
        let host_census = shared.close();
        let terminal_census = client_shutdown.as_ref().map_or(host_census, |client| {
            host_census.with_client_terminal(client.resources())
        });
        drop(shared);
        let cleanup_complete =
            terminal_cleanup_complete(client_closed, readiness_owner_count == 2, &terminal_census);
        let failure = self.failure_cause(
            cleanup_complete,
            presentation.as_ref(),
            graphics.as_ref(),
            client_attribution,
        );
        if let Some(cause) = failure {
            let cleanup = UiNativeEventLoopCleanup::retain(
                Rc::clone(&self.shared),
                terminal_census,
                client_cleanup,
                self.physical_clock,
            );
            return Err(UiNativeEventLoopStopReport {
                cause,
                effect_posture,
                peak_census,
                terminal_census,
                client_cleanup_complete: client_closed,
                cleanup,
                peak_text_pins,
            });
        }
        Ok(self.completed_report(UiNativeEventLoopCompletionEvidence {
            presentation: presentation.expect("validated presentation"),
            graphics: graphics.expect("validated graphics"),
            client_attribution: client_attribution.expect("validated client attribution"),
            peak_census,
            terminal_census,
            retained_frames,
            peak_text_pins,
            text_pin_frame_counts: text_pin_frame_counts.into_boxed_slice(),
            text_pin_frame_observations: text_pin_frame_observations.into_boxed_slice(),
            text_atlas_model_frame_digests: text_atlas_model_frame_digests.into_boxed_slice(),
            text_atlas_plan_observations: text_atlas_plan_observations.into_boxed_slice(),
            physical_signal_transition_observations: physical_signal_transition_observations
                .into_boxed_slice(),
            physical_signal_transition_trace_complete,
            physical_signal_lifecycle,
            observation_history_complete,
            text_atlas_transactions,
            derived_state_reconstruction,
            client_shutdown,
        }))
    }

    fn failure_cause(
        &self,
        cleanup_complete: bool,
        presentation: Option<&crate::native::UiNativePresentationObservation>,
        graphics: Option<&UiNativeGraphicsObservation>,
        attribution: Option<super::UiNativeClientPresentationAttribution>,
    ) -> Option<UiNativeEventLoopRunDenial> {
        if !cleanup_complete {
            return Some(UiNativeEventLoopRunDenial::IncompleteCleanup);
        }
        self.failure
            .or_else(|| {
                presentation
                    .is_none()
                    .then_some(UiNativeEventLoopRunDenial::ApplicationDriver)
            })
            .or_else(|| {
                graphics
                    .is_none()
                    .then_some(UiNativeEventLoopRunDenial::GraphicsPreparation)
            })
            .or_else(|| {
                self.thread_observation
                    .is_none()
                    .then_some(UiNativeEventLoopRunDenial::EventLoopRun)
            })
            .or_else(|| {
                self.thread_observation
                    .is_some_and(|value| !value.matches_launch)
                    .then_some(UiNativeEventLoopRunDenial::EventLoopRun)
            })
            .or_else(|| {
                attribution
                    .zip(presentation)
                    .is_none_or(|(value, observed)| !value.matches(observed))
                    .then_some(UiNativeEventLoopRunDenial::ApplicationDriver)
            })
    }

    fn completed_report(
        self,
        evidence: UiNativeEventLoopCompletionEvidence,
    ) -> UiNativeEventLoopRunReport {
        let thread = self
            .thread_observation
            .expect("validated event-loop thread");
        UiNativeEventLoopRunReport {
            port_crossings: self
                .port_crossings
                .saturating_add(evidence.presentation.port_crossings()),
            presentation: evidence.presentation,
            graphics: evidence.graphics,
            event_loop_thread: format!("{:?}", thread.thread).into_boxed_str(),
            event_loop_thread_matches_launch: thread.matches_launch,
            client_attribution: evidence.client_attribution,
            readiness_signals: self.readiness_signals,
            redraw_turns: self.redraw_turns,
            idle_wait_turns: self.idle_wait_turns,
            coalesced_wakes: self.coalesced_wakes,
            peak_census: evidence.peak_census,
            terminal_census: evidence.terminal_census,
            retained_frames: evidence.retained_frames.into_boxed_slice(),
            peak_text_pins: evidence.peak_text_pins,
            text_pin_frame_counts: evidence.text_pin_frame_counts,
            text_pin_frame_observations: evidence.text_pin_frame_observations,
            text_atlas_model_frame_digests: evidence.text_atlas_model_frame_digests,
            text_atlas_plan_observations: evidence.text_atlas_plan_observations,
            physical_signal_transition_observations: evidence
                .physical_signal_transition_observations,
            physical_signal_transition_trace_complete: evidence
                .physical_signal_transition_trace_complete,
            physical_signal_lifecycle: evidence.physical_signal_lifecycle,
            observation_history_complete: evidence.observation_history_complete,
            text_atlas_transactions: evidence.text_atlas_transactions,
            derived_state_reconstruction: evidence.derived_state_reconstruction,
            client_shutdown: evidence.client_shutdown,
        }
    }
}

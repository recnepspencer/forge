use std::rc::Rc;

use super::completion_report::UiNativeEventLoopCompletionEvidence;
use super::{
    finish_capture, finish_cleanup, UiNativeEventLoopApplication, UiNativeEventLoopCleanup,
    UiNativeEventLoopClient, UiNativeEventLoopRunDenial, UiNativeEventLoopRunReport,
    UiNativeEventLoopStopReport,
};

impl<Client: UiNativeEventLoopClient> UiNativeEventLoopApplication<Client> {
    pub(super) fn finish(
        mut self,
    ) -> Result<UiNativeEventLoopRunReport, UiNativeEventLoopStopReport> {
        let captured = finish_capture::capture(&self);
        let terminal = finish_cleanup::close(&mut self, captured.host_peak_census);
        let failure = self.failure_cause(
            terminal.cleanup_complete,
            captured.presentation.as_ref(),
            captured.graphics.as_ref(),
            captured.client_attribution,
        );
        if let Some(cause) = failure {
            let cleanup = UiNativeEventLoopCleanup::retain(
                Rc::clone(&self.shared),
                terminal.terminal_census,
                terminal.client_cleanup,
                self.physical_clock,
            );
            return Err(UiNativeEventLoopStopReport {
                cause,
                effect_posture: captured.effect_posture,
                peak_census: terminal.peak_census,
                terminal_census: terminal.terminal_census,
                client_cleanup_complete: terminal.client_closed
                    && terminal.client_resources_complete,
                cleanup,
                peak_text_pins: captured.peak_text_pins,
                input_observations: captured.input_observations,
                shutdown_overlap: terminal.shutdown_overlap,
            });
        }
        Ok(self.completed_report(UiNativeEventLoopCompletionEvidence {
            presentation: captured.presentation.expect("validated presentation"),
            graphics: captured.graphics.expect("validated graphics"),
            client_attribution: captured
                .client_attribution
                .expect("validated client attribution"),
            peak_census: terminal.peak_census,
            terminal_census: terminal.terminal_census,
            retained_frames: captured.retained_frames,
            peak_text_pins: captured.peak_text_pins,
            text_pin_frame_counts: captured.text_pin_frame_counts,
            text_pin_frame_observations: captured.text_pin_frame_observations,
            text_atlas_model_frame_digests: captured.text_atlas_model_frame_digests,
            text_atlas_plan_observations: captured.text_atlas_plan_observations,
            physical_signal_transition_observations: captured
                .physical_signal_transition_observations,
            physical_signal_transition_trace_complete: captured
                .physical_signal_transition_trace_complete,
            physical_signal_lifecycle: captured.physical_signal_lifecycle,
            input_observations: captured.input_observations,
            observation_history_complete: captured.observation_history_complete,
            text_atlas_transactions: captured.text_atlas_transactions,
            derived_state_reconstruction: captured.derived_state_reconstruction,
            client_shutdown: terminal.client_shutdown,
            shutdown_overlap: terminal.shutdown_overlap,
        }))
    }

    fn failure_cause(
        &self,
        cleanup_complete: bool,
        presentation: Option<&crate::native::UiNativePresentationObservation>,
        graphics: Option<&crate::native::UiNativeGraphicsObservation>,
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
            .or_else(|| {
                self.shared
                    .borrow()
                    .lifecycle
                    .input_report()
                    .terminal_stop()
                    .is_some()
                    .then_some(UiNativeEventLoopRunDenial::ApplicationDriver)
            })
    }
}

use std::rc::Rc;

use super::{
    terminal_cleanup_complete, UiNativeEventLoopApplication, UiNativeEventLoopCleanup,
    UiNativeEventLoopClient, UiNativeEventLoopRunDenial, UiNativeEventLoopRunReport,
    UiNativeEventLoopStopReport,
};
use crate::native::UiNativeGraphicsObservation;

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
        let peak_census = self.shared.borrow().compiler_total_peak();
        let peak_text_pins = self.shared.borrow().peak_text_pins.clone();
        let effect_posture = self.shared.borrow().effect_posture;
        let graphics = self
            .shared
            .borrow()
            .graphics
            .as_ref()
            .map(|graphics| UiNativeGraphicsObservation::from_graphics(graphics));
        let client_cleanup = self
            .client
            .take()
            .and_then(|client| client.close().into_cleanup());
        let client_closed = client_cleanup.is_none();
        let readiness_owner_count = self.readiness.close();
        let mut shared = self.shared.borrow_mut();
        shared
            .resources
            .release_all(self.loop_resources.drain(..))
            .expect("event-loop owners must remain exact");
        let host_census = shared.close();
        drop(shared);
        let cleanup_complete =
            terminal_cleanup_complete(client_closed, readiness_owner_count == 2, &host_census);
        let failure = self.failure_cause(
            cleanup_complete,
            presentation.as_ref(),
            graphics.as_ref(),
            client_attribution,
        );
        if let Some(cause) = failure {
            let cleanup = UiNativeEventLoopCleanup::retain(
                Rc::clone(&self.shared),
                host_census,
                client_cleanup,
                self.physical_clock,
            );
            return Err(UiNativeEventLoopStopReport {
                cause,
                effect_posture,
                peak_census,
                terminal_census: host_census,
                client_cleanup_complete: client_closed,
                cleanup,
                peak_text_pins,
            });
        }
        Ok(self.completed_report(
            presentation.expect("validated presentation"),
            graphics.expect("validated graphics"),
            client_attribution.expect("validated client attribution"),
            peak_census,
            host_census,
            retained_frames,
            peak_text_pins,
        ))
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
        presentation: crate::native::UiNativePresentationObservation,
        graphics: UiNativeGraphicsObservation,
        client_attribution: super::UiNativeClientPresentationAttribution,
        peak_census: crate::native::UiNativeResourceCensus,
        terminal_census: crate::native::UiNativeResourceCensus,
        retained_frames: Vec<crate::native::UiNativeRetainedFrameObservation>,
        peak_text_pins: Box<[crate::native::text_atlas::UiNativeTextPinObservation]>,
    ) -> UiNativeEventLoopRunReport {
        let thread = self
            .thread_observation
            .expect("validated event-loop thread");
        UiNativeEventLoopRunReport {
            port_crossings: self
                .port_crossings
                .saturating_add(presentation.port_crossings()),
            presentation,
            graphics,
            event_loop_thread: format!("{:?}", thread.thread).into_boxed_str(),
            event_loop_thread_matches_launch: thread.matches_launch,
            client_attribution,
            readiness_signals: self.readiness_signals,
            redraw_turns: self.redraw_turns,
            idle_wait_turns: self.idle_wait_turns,
            coalesced_wakes: self.coalesced_wakes,
            peak_census,
            terminal_census,
            retained_frames: retained_frames.into_boxed_slice(),
            peak_text_pins,
        }
    }
}

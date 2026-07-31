use eframe::egui;
use worth_ui_host_egui::{
    UiEguiRawInputIngressOutcome, UiEguiRawInputIngressStopReason, UiEguiRawInputReachability,
    WorthUiHostEgui,
};
use worth_ui_platform_pulse::observation_contract::PlatformPulseNativeInputIngressPosture;

use crate::lifecycle_observation_publication::{
    PlatformPulseObservationPublicationDenial, PlatformPulseObservationPublisher,
};

#[derive(Default)]
pub(super) struct PlatformPulseNativeInputIngress {
    armed: bool,
    pointer_published: bool,
    keyboard_published: bool,
}

pub(super) enum PlatformPulseNativeInputIngressDenial {
    Adapter {
        reason: UiEguiRawInputIngressStopReason,
        publication: Result<(), PlatformPulseObservationPublicationDenial>,
    },
    Publication(PlatformPulseObservationPublicationDenial),
}

impl PlatformPulseNativeInputIngress {
    pub(super) fn arm_after_first_frame(&mut self) {
        self.armed = true;
    }

    pub(super) fn observe(
        &mut self,
        host: Option<&WorthUiHostEgui>,
        raw_input: &egui::RawInput,
        publisher: &PlatformPulseObservationPublisher,
    ) -> Result<(), PlatformPulseNativeInputIngressDenial> {
        if !self.armed {
            return Ok(());
        }
        let Some(outcome) = observe_egui_input(host, raw_input) else {
            return Ok(());
        };
        match outcome {
            UiEguiRawInputIngressOutcome::Retained(retained) => self
                .publish_discovered(
                    retained.reachability(),
                    PlatformPulseNativeInputIngressPosture::Retained,
                    publisher,
                )
                .map_err(PlatformPulseNativeInputIngressDenial::Publication),
            UiEguiRawInputIngressOutcome::NoMechanicalObservations(_) => Ok(()),
            UiEguiRawInputIngressOutcome::Stopped(stop) => {
                let publication = self.publish_discovered(
                    stop.reachability(),
                    PlatformPulseNativeInputIngressPosture::Stopped,
                    publisher,
                );
                Err(PlatformPulseNativeInputIngressDenial::Adapter {
                    reason: stop.reason(),
                    publication,
                })
            }
        }
    }

    fn publish_discovered(
        &mut self,
        reached: UiEguiRawInputReachability,
        posture: PlatformPulseNativeInputIngressPosture,
        publisher: &PlatformPulseObservationPublisher,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        let pointer_discovered = reached.pointer_button_events() > 0 && !self.pointer_published;
        let keyboard_discovered = reached.keyboard_events() > 0 && !self.keyboard_published;
        if pointer_discovered
            || keyboard_discovered
            || posture == PlatformPulseNativeInputIngressPosture::Stopped
        {
            publisher.native_input_reached(reached, posture)?;
            self.pointer_published |= pointer_discovered;
            self.keyboard_published |= keyboard_discovered;
        }
        Ok(())
    }
}

pub(super) fn observe_egui_input(
    host: Option<&WorthUiHostEgui>,
    raw_input: &egui::RawInput,
) -> Option<UiEguiRawInputIngressOutcome> {
    host.map(|host| host.observe_native_input(raw_input))
}

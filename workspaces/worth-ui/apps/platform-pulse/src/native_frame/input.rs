use eframe::egui;
use worth_ui_host_egui::{
    UiEguiRawInputIngressOutcome, UiEguiRawInputReachability, WorthUiHostEgui,
};

use crate::lifecycle_observation_publication::{
    PlatformPulseObservationPublicationDenial, PlatformPulseObservationPublisher,
};

#[derive(Default)]
pub(super) struct PlatformPulseNativeInputIngress {
    armed: bool,
    pointer_published: bool,
    keyboard_published: bool,
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
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        if !self.armed {
            return Ok(());
        }
        let Some(reached) = observe_egui_input(host, raw_input) else {
            return Ok(());
        };
        let pointer_discovered = reached.pointer_button_events() > 0 && !self.pointer_published;
        let keyboard_discovered = reached.keyboard_events() > 0 && !self.keyboard_published;
        if pointer_discovered || keyboard_discovered {
            publisher.native_input_reached(reached)?;
            self.pointer_published |= pointer_discovered;
            self.keyboard_published |= keyboard_discovered;
        }
        Ok(())
    }
}

pub(super) fn observe_egui_input(
    host: Option<&WorthUiHostEgui>,
    raw_input: &egui::RawInput,
) -> Option<UiEguiRawInputReachability> {
    host.map(|host| match host.observe_native_input(raw_input) {
        UiEguiRawInputIngressOutcome::Unsupported(reachability) => reachability,
    })
}

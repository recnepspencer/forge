use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use worth_ui_host_contract::UiSurfaceBindingGeneration;

mod input_observation;
mod measurement;
mod mechanics;

#[derive(Clone, Default)]
pub struct WorthUiHostEgui {
    context: egui::Context,
    registrations: Arc<
        Mutex<
            BTreeMap<
                UiSurfaceBindingGeneration,
                worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
            >,
        >,
    >,
    measurement_environment: Arc<Mutex<EguiMeasurementEnvironment>>,
    observation_retention: Arc<worth_ui_host_contract::UiHostObservationRetention>,
    input_translators: super::input_observation::UiEguiInstalledInputTranslators,
    input_observation: Arc<Mutex<super::input_observation::UiEguiInputObservationState>>,
    retained_presentations: Arc<
        Mutex<
            BTreeMap<
                UiSurfaceBindingGeneration,
                super::mounted_presentation::UiEguiPreparedMountedPresentation,
            >,
        >,
    >,
    visual_captures: Arc<Mutex<super::visual_snapshot::UiEguiVisualCaptureState>>,
}

#[derive(Default)]
struct EguiMeasurementEnvironment {
    viewport: Option<(u32, u32)>,
    dpi: Option<u32>,
    viewport_generation: u64,
    dpi_generation: u64,
}

impl WorthUiHostEgui {
    pub fn new(context: egui::Context) -> Self {
        Self {
            context,
            registrations: Arc::default(),
            measurement_environment: Arc::default(),
            observation_retention: Arc::default(),
            input_translators: Default::default(),
            input_observation: Arc::default(),
            retained_presentations: Arc::default(),
            visual_captures: Arc::default(),
        }
    }

    #[cfg(test)]
    pub(super) fn record_completed_input_basis_for_test(
        &self,
        view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
        epoch: worth_ui_host_contract::UiHostPresentationEpoch,
    ) {
        super::input_observation::record_completed_presentation(
            &self.input_observation,
            view,
            epoch,
        );
    }

    pub fn registered_surface_count(&self) -> usize {
        self.registrations.lock().unwrap().len()
    }

    /// Drain adapter-owned native observations for the mounted host session.
    ///
    /// The returned observations remain governed by the host-contract batch;
    /// this adapter method exposes transport, not semantic admission authority.
    pub fn drain_native_observations(
        &self,
        host_session_identity: u64,
    ) -> worth_ui_host_contract::UiHostObservationDrain {
        self.observation_retention.drain(host_session_identity)
    }

    /// Replay the currently admitted mounted mechanics for one egui frame.
    ///
    /// Egui paint commands are frame-ephemeral. This method replays only
    /// adapter-owned mechanics retained from a successful mounted presentation;
    /// it does not execute Worth UI or construct new product meaning.
    pub fn repaint_retained_surfaces(&self) {
        for presentation in self.retained_presentations.lock().unwrap().values() {
            presentation.paint(&self.context);
        }
    }
}

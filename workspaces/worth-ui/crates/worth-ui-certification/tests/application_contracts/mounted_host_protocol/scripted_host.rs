use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Mutex};

use worth_ui_host_contract::WorthUiHostCapabilityReport;
use worth_ui_runtime::facade::mounted::{
    UiHostSurfaceCancellationOutcome, UiHostSurfaceInFlightCompletion,
    UiHostSurfacePresentationMode, UiHostSurfacePresentationOutcome, UiMountedCompletedEffects,
    UiMountedEffectFamily,
};

mod adapter;

#[derive(Clone, Default)]
pub(crate) struct ScriptedPresentationHost {
    state: Arc<Mutex<ScriptedPresentationState>>,
}

enum ScriptedPresentationStart {
    Outcome(UiHostSurfacePresentationOutcome),
    InFlight {
        completions: VecDeque<ScriptedSurfaceCompletion>,
        cancellation: UiHostSurfaceCancellationOutcome,
    },
}

pub(crate) enum ScriptedSurfaceCompletion {
    Pending,
    RejectedBeforeEffects(worth_ui_host_contract::UiHostSurfacePresentationDenial),
    Presented(worth_ui_host_contract::UiMountedSurfacePresentationCompletion),
    PresentationIndeterminate,
}

struct ScriptedPresentationState {
    protocol: worth_ui_host_contract::UiHostProtocolContract,
    capabilities: WorthUiHostCapabilityReport,
    presentations: VecDeque<ScriptedPresentationStart>,
    completions: BTreeMap<u64, VecDeque<ScriptedSurfaceCompletion>>,
    cancellations: BTreeMap<u64, UiHostSurfaceCancellationOutcome>,
    token_sessions: BTreeMap<u64, u64>,
    registrations: BTreeMap<
        worth_ui_host_contract::UiHostSurfaceIdentity,
        worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
    >,
    wrong_next_registration_receipt: bool,
    wrong_next_deregistration_receipt: bool,
    cancellation_calls: Vec<u64>,
    presentation_calls: usize,
    viewport_environment_generation: u64,
    font_environment_generation: u64,
    adapter_environment_generation: u64,
    queued_observation: Option<(
        worth_ui::facade::observation_report::WorthUiHostObservationIngress,
        worth_ui::facade::observation_report::UiHostObservationBatch,
    )>,
    queued_measurement: Option<(
        worth_ui::facade::measurement_exchange::WorthUiHostMeasurementIngress,
        worth_ui::facade::measurement_exchange::UiHostMeasurementCompletion,
    )>,
    observation_events: Vec<&'static str>,
}

impl Default for ScriptedPresentationState {
    fn default() -> Self {
        Self {
            protocol: worth_ui_host_contract::UiHostProtocolContract::current(),
            capabilities: recording_capabilities(),
            presentations: VecDeque::new(),
            completions: BTreeMap::new(),
            cancellations: BTreeMap::new(),
            token_sessions: BTreeMap::new(),
            registrations: BTreeMap::new(),
            wrong_next_registration_receipt: false,
            wrong_next_deregistration_receipt: false,
            cancellation_calls: Vec::new(),
            presentation_calls: 0,
            viewport_environment_generation: 1,
            font_environment_generation: 1,
            adapter_environment_generation: 1,
            queued_observation: None,
            queued_measurement: None,
            observation_events: Vec::new(),
        }
    }
}

pub(crate) fn recorded_effects() -> UiMountedCompletedEffects {
    UiMountedCompletedEffects::new(vec![UiMountedEffectFamily::RecordedProjection])
}

fn scripted_presentation_cost() -> worth_ui_host_contract::UiHostPresentationCostReport {
    worth_ui_host_contract::UiHostPresentationCostReport::from_adapter(
        worth_ui_host_contract::UiHostPresentationCostInput {
            presented_surfaces: 1,
            translated_rows: 0,
            translated_bytes: 0,
            native_resource_cache_hits: 0,
            native_resource_cache_misses: 0,
            asynchronous_handoffs: 0,
        },
    )
}

pub(crate) fn presented_completion() -> ScriptedSurfaceCompletion {
    ScriptedSurfaceCompletion::Presented(
        worth_ui_host_contract::UiMountedSurfacePresentationCompletion::new(
            UiHostSurfacePresentationMode::RecordOnly,
            recorded_effects(),
            scripted_presentation_cost(),
        ),
    )
}

impl ScriptedPresentationHost {
    pub(crate) fn set_protocol(&self, protocol: worth_ui_host_contract::UiHostProtocolContract) {
        self.state.lock().unwrap().protocol = protocol;
    }

    pub(crate) fn set_capabilities(&self, capabilities: WorthUiHostCapabilityReport) {
        self.state.lock().unwrap().capabilities = capabilities;
    }

    pub(crate) fn push_presented(&self) {
        self.push_presentation(UiHostSurfacePresentationOutcome::Presented(
            worth_ui_host_contract::UiMountedSurfacePresentationCompletion::new(
                UiHostSurfacePresentationMode::RecordOnly,
                recorded_effects(),
                scripted_presentation_cost(),
            ),
        ));
    }

    pub(crate) fn push_rejected(&self) {
        self.push_presentation(UiHostSurfacePresentationOutcome::RejectedBeforeEffects(
            worth_ui_host_contract::UiHostSurfacePresentationDenial::AdapterDeclined,
        ));
    }

    pub(crate) fn push_presentation(&self, outcome: UiHostSurfacePresentationOutcome) {
        self.state
            .lock()
            .unwrap()
            .presentations
            .push_back(ScriptedPresentationStart::Outcome(outcome));
    }

    pub(crate) fn push_in_flight(
        &self,
        completions: Vec<ScriptedSurfaceCompletion>,
        cancellation: UiHostSurfaceCancellationOutcome,
    ) {
        self.state
            .lock()
            .unwrap()
            .presentations
            .push_back(ScriptedPresentationStart::InFlight {
                completions: completions.into_iter().collect(),
                cancellation,
            });
    }

    pub(crate) fn presentation_calls(&self) -> usize {
        self.state.lock().unwrap().presentation_calls
    }

    pub(crate) fn return_wrong_next_registration_receipt(&self) {
        self.state.lock().unwrap().wrong_next_registration_receipt = true;
    }

    pub(crate) fn return_wrong_next_deregistration_receipt(&self) {
        self.state.lock().unwrap().wrong_next_deregistration_receipt = true;
    }

    pub(crate) fn native_registration_count(&self) -> usize {
        self.state.lock().unwrap().registrations.len()
    }

    pub(crate) fn native_in_flight_count(&self) -> usize {
        self.state.lock().unwrap().token_sessions.len()
    }

    pub(crate) fn advance_viewport_environment(&self) {
        let mut state = self.state.lock().unwrap();
        state.viewport_environment_generation = state
            .viewport_environment_generation
            .checked_add(1)
            .expect("scripted viewport generation capacity");
    }

    pub(crate) fn advance_font_environment(&self) {
        let mut state = self.state.lock().unwrap();
        state.font_environment_generation = state
            .font_environment_generation
            .checked_add(1)
            .expect("scripted font generation capacity");
    }

    pub(crate) fn cancellation_calls(&self) -> Vec<u64> {
        self.state.lock().unwrap().cancellation_calls.clone()
    }

    pub(crate) fn enqueue_observation_during_next_presentation(
        &self,
        ingress: worth_ui::facade::observation_report::WorthUiHostObservationIngress,
        batch: worth_ui::facade::observation_report::UiHostObservationBatch,
    ) {
        self.state.lock().unwrap().queued_observation = Some((ingress, batch));
    }

    pub(crate) fn enqueue_measurement_during_next_presentation(
        &self,
        ingress: worth_ui::facade::measurement_exchange::WorthUiHostMeasurementIngress,
        completion: worth_ui::facade::measurement_exchange::UiHostMeasurementCompletion,
    ) {
        self.state.lock().unwrap().queued_measurement = Some((ingress, completion));
    }

    pub(crate) fn observation_events(&self) -> Vec<&'static str> {
        self.state.lock().unwrap().observation_events.clone()
    }
}

fn recording_capabilities() -> WorthUiHostCapabilityReport {
    WorthUiHostCapabilityReport::available(vec![
        worth_ui_host_contract::WorthUiHostCapability::MountedFrameRecording,
    ])
}

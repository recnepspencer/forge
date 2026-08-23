mod keyboard;
mod outcome;
mod pointer;
mod presentation_basis;
mod reachability;
mod state;
#[cfg(test)]
mod tests;
mod text_ime;
mod translation;

pub use outcome::{
    UiEguiCoordinateConversionDenial, UiEguiInputTranslatorFamily, UiEguiRawInputIngressOutcome,
    UiEguiRawInputIngressStop, UiEguiRawInputIngressStopReason, UiEguiRetainedRawInput,
    UiEguiUnsupportedEventFamily,
};
pub use reachability::UiEguiRawInputReachability;

pub(crate) use state::UiEguiInputObservationState;
pub(crate) use translation::UiEguiInstalledInputTranslators;

pub(super) fn observe_raw_input(
    translators: UiEguiInstalledInputTranslators,
    state: &std::sync::Mutex<UiEguiInputObservationState>,
    retention: &worth_ui_host_contract::UiHostObservationRetention,
    raw_input: &egui::RawInput,
) -> UiEguiRawInputIngressOutcome {
    let mut state = state.lock().expect("egui input observation state poisoned");
    let basis = match state.select_presented_basis() {
        state::UiEguiPresentedInputSelection::Missing => {
            return stopped_without_basis(
                raw_input,
                UiEguiRawInputIngressStopReason::NoPresentedSurface,
            );
        }
        state::UiEguiPresentedInputSelection::Ambiguous(count) => {
            return stopped_without_basis(
                raw_input,
                UiEguiRawInputIngressStopReason::AmbiguousPresentedSurfaces { count },
            );
        }
        state::UiEguiPresentedInputSelection::Unique(basis) => basis,
    };
    let transaction = state.transaction_state(basis);
    let recipient = state.input_recipient(basis);
    let translated = match translators.translate(raw_input, basis, recipient, transaction) {
        Ok(translated) => translated,
        Err(stop) => return UiEguiRawInputIngressOutcome::Stopped(stop),
    };
    let Some(batch) = translated.batch else {
        state.commit(basis, translated.state);
        return UiEguiRawInputIngressOutcome::NoMechanicalObservations(translated.reachability);
    };
    let core = batch.canonical_core();
    if let Err(denial) = retention.retain(batch) {
        return UiEguiRawInputIngressOutcome::Stopped(UiEguiRawInputIngressStop::new(
            translated.reachability,
            UiEguiRawInputIngressStopReason::Retention(denial),
        ));
    }
    state.commit(basis, translated.state);
    UiEguiRawInputIngressOutcome::Retained(UiEguiRetainedRawInput::new(
        translated.reachability,
        core.presentation(),
        core.sequences(),
        core.report_count(),
    ))
}

pub(super) fn install_input_recipient(
    state: &std::sync::Mutex<UiEguiInputObservationState>,
    binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
) -> bool {
    state
        .lock()
        .expect("egui input observation state poisoned")
        .install_input_recipient(binding)
}

pub(super) fn clear_input_recipient(
    state: &std::sync::Mutex<UiEguiInputObservationState>,
    binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
) -> bool {
    state
        .lock()
        .expect("egui input observation state poisoned")
        .clear_input_recipient(binding)
}

pub(super) fn record_completed_presentation(
    state: &std::sync::Mutex<UiEguiInputObservationState>,
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    epoch: worth_ui_host_contract::UiHostPresentationEpoch,
) {
    state
        .lock()
        .expect("egui input observation state poisoned")
        .record_presentation(presentation_basis::UiEguiPresentedInputBasis::completed(
            view, epoch,
        ));
}

pub(super) fn remove_binding(
    state: &std::sync::Mutex<UiEguiInputObservationState>,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
) {
    state
        .lock()
        .expect("egui input observation state poisoned")
        .remove_binding(binding);
}

pub(super) fn release_session(
    state: &std::sync::Mutex<UiEguiInputObservationState>,
    host_session: u64,
) {
    state
        .lock()
        .expect("egui input observation state poisoned")
        .release_session(host_session);
}

fn stopped_without_basis(
    raw_input: &egui::RawInput,
    reason: UiEguiRawInputIngressStopReason,
) -> UiEguiRawInputIngressOutcome {
    UiEguiRawInputIngressOutcome::Stopped(UiEguiRawInputIngressStop::new(
        UiEguiRawInputReachability::inspect(raw_input),
        reason,
    ))
}

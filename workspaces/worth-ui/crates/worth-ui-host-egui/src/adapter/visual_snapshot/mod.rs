use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

mod observation;
mod state;
#[cfg(test)]
mod state_tests;

pub(super) use state::UiEguiVisualCaptureState;

pub(super) const fn capture_capability() -> worth_ui_host_contract::UiHostCaptureCapability {
    worth_ui_host_contract::UiHostCaptureCapability::Pixels {
        maximum_bytes: 64 * 1024 * 1024,
        exact_presentation_epoch: true,
    }
}

pub(super) fn record_presentation(
    state: &Arc<Mutex<UiEguiVisualCaptureState>>,
    view: &worth_ui_host_contract::UiMountedFrameConsumptionView<'_>,
    epoch: worth_ui_host_contract::UiHostPresentationEpoch,
    regions: Vec<worth_ui_host_contract::UiHostRealizedRegion>,
) {
    state.lock().unwrap().record_presentation(
        view.requirement().binding(),
        state::UiEguiPresentedSurface::from_view(view, epoch, regions),
    );
}

pub(super) fn remove_binding(
    state: &Arc<Mutex<UiEguiVisualCaptureState>>,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
) {
    state.lock().unwrap().remove_binding(binding);
}

pub(super) fn capture(
    context: &egui::Context,
    registrations: &Arc<
        Mutex<
            BTreeMap<
                worth_ui_host_contract::UiSurfaceBindingGeneration,
                worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
            >,
        >,
    >,
    state: &Arc<Mutex<UiEguiVisualCaptureState>>,
    request: worth_ui_host_contract::UiHostVisualCaptureRequest,
) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
    if !registration_matches(registrations, request) {
        return worth_ui_host_contract::UiHostCaptureObservationOutcome::Unsupported;
    }
    let presented = match state.lock().unwrap().presentation_affinity(request) {
        state::UiEguiPresentationAffinity::Exact(presented) => presented,
        state::UiEguiPresentationAffinity::Superseded => {
            return worth_ui_host_contract::UiHostCaptureObservationOutcome::SupersededBeforeReadback;
        }
    };
    if !request.pixels_requested() {
        return observation::geometry_observation(context, request, presented);
    }
    poll_or_request_screenshot(context, state, request, presented)
}

pub(super) fn cancel(
    state: &Arc<Mutex<UiEguiVisualCaptureState>>,
    request: worth_ui_host_contract::UiHostVisualCaptureRequest,
) -> worth_ui_host_contract::UiHostCaptureCancellationOutcome {
    if state.lock().unwrap().cancel(request) {
        worth_ui_host_contract::UiHostCaptureCancellationOutcome::ReadbackMayHaveBegun
    } else {
        worth_ui_host_contract::UiHostCaptureCancellationOutcome::CleanupIndeterminate
    }
}

fn poll_or_request_screenshot(
    context: &egui::Context,
    state: &Arc<Mutex<UiEguiVisualCaptureState>>,
    request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    presented: state::UiEguiPresentedSurface,
) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
    if let Some(event) = observation::matching_screenshot(context, request.identity()) {
        let still_exact = state.lock().unwrap().finish_if_exact(request);
        return if still_exact {
            observation::pixel_observation(context, request, presented, event)
        } else {
            worth_ui_host_contract::UiHostCaptureObservationOutcome::SupersededBeforeReadback
        };
    }
    let mut captures = state.lock().unwrap();
    match captures.admit_pending(request) {
        state::UiEguiPendingAdmission::AlreadyPending => {
            worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
        }
        state::UiEguiPendingAdmission::CapacityExceeded => {
            worth_ui_host_contract::UiHostCaptureObservationOutcome::CapacityExceeded
        }
        state::UiEguiPendingAdmission::Admitted(correlation) => {
            context.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::new(
                correlation,
            )));
            worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
        }
    }
}

fn registration_matches(
    registrations: &Arc<
        Mutex<
            BTreeMap<
                worth_ui_host_contract::UiSurfaceBindingGeneration,
                worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
            >,
        >,
    >,
    request: worth_ui_host_contract::UiHostVisualCaptureRequest,
) -> bool {
    registrations
        .lock()
        .unwrap()
        .get(&request.binding())
        .is_some_and(|registered| {
            registered.host_session_identity() == request.host_session_identity()
                && registered.host_surface_identity() == request.host_surface()
                && registered.binding_generation() == request.binding()
        })
}

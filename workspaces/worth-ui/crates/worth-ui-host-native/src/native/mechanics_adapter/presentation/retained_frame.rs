use worth_ui_host_contract::{
    UiHostPresentationCostReport, UiHostSurfacePresentationOutcome, UiMountedFrameConsumptionView,
    UiMountedPresentationUnchanged,
};

use crate::native::{
    presentation::observation_for_retained, UiNativeHostState, UiNativePresentationWorkKind,
    UiNativeRetainedFrameObservation,
};

pub(super) fn perform_unchanged(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
    unchanged: &UiMountedPresentationUnchanged,
) -> UiHostSurfacePresentationOutcome {
    let key = view.binding().diagnostic_value();
    if state.reconstruction_required.contains(&key) {
        return super::require_owner_reconstruction(state, key);
    }
    retain_unchanged(state, view, unchanged, key)
}

fn retain_unchanged(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
    unchanged: &UiMountedPresentationUnchanged,
    key: u64,
) -> UiHostSurfacePresentationOutcome {
    let Some(retained) = state.retained_draw_lists.get_mut(&key) else {
        return super::malformed();
    };
    if retained.apply_unchanged(unchanged).is_err() {
        return super::malformed();
    }
    let pixels = latest_pixels(state);
    record_retained_frame(
        state,
        view,
        key,
        UiNativePresentationWorkKind::Unchanged,
        pixels,
        Default::default(),
        0,
    );
    super::completed(state, key, view, Default::default(), false)
}

pub(super) fn record_retained_frame(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
    key: u64,
    kind: UiNativePresentationWorkKind,
    pixels: [[u8; 4]; 2],
    cost: UiHostPresentationCostReport,
    port_crossings: u8,
) {
    let observation = state
        .graphics
        .as_ref()
        .zip(state.retained_draw_lists.get(&key))
        .and_then(|(graphics, retained)| {
            observation_for_retained(
                view,
                graphics,
                &state.text_atlas,
                retained,
                pixels,
                cost,
                port_crossings,
            )
        });
    state.record_retained_frame_observation(UiNativeRetainedFrameObservation::observed(
        view.frame().diagnostic_value(),
        kind,
        pixels,
        cost,
        observation.clone(),
    ));
    state.last_presentation = observation;
}

pub(super) fn latest_pixels(state: &UiNativeHostState) -> [[u8; 4]; 2] {
    state
        .retained_frame_observations
        .last()
        .map(|observation| {
            [
                observation.retained_baseline_rgba8(),
                observation.retained_center_rgba8(),
            ]
        })
        .unwrap_or([[0, 0, 0, 0]; 2])
}

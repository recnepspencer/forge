use worth_ui_host_contract::{
    UiHostSurfacePresentationMode, UiHostSurfacePresentationOutcome, UiMountedCompletedEffects,
    UiMountedEffectFamily, UiMountedFrameConsumptionView, UiMountedPresentationWorkView,
    UiMountedSurfacePresentationCompletion,
};

use crate::native::{
    presentation::{
        present_cold_reconstruction, present_delta, present_initial, UiNativePresentationFailure,
        UiWgpuNativePresentationPort,
    },
    UiNativeEffectPosture, UiNativeHostState, UiNativePresentationWorkKind,
    UiNativeRetainedFrameObservation,
};

use super::presentation_text_atlas as text_atlas;

#[path = "presentation/pending_completion.rs"]
mod pending_completion;
pub(super) use pending_completion::{complete_pending, owns_completion, stop_pending};

#[path = "presentation/retained_frame.rs"]
mod retained_frame;

#[path = "presentation/glyph_run_admission.rs"]
pub(super) mod glyph_run_admission;

#[cfg(test)]
#[path = "presentation/text_atlas_tests.rs"]
pub(crate) mod text_atlas_tests;

pub(super) fn perform_native_presentation(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
) -> UiHostSurfacePresentationOutcome {
    if view.text_raster_work().is_some() {
        return match text_atlas::begin(state, view) {
            text_atlas::UiMountedTextWorkOutcome::Ready => {
                state.record_text_pin_frame_observation();
                perform_surface_work(state, view)
            }
            text_atlas::UiMountedTextWorkOutcome::Pending(pending) => {
                text_atlas::settle_deferred(state, view, Some(pending))
            }
            text_atlas::UiMountedTextWorkOutcome::Terminal(outcome) => outcome,
        };
    }
    perform_surface_work(state, view)
}

fn perform_surface_work(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
) -> UiHostSurfacePresentationOutcome {
    match view.presentation_work() {
        UiMountedPresentationWorkView::Initial(_) => perform_initial(state, view),
        UiMountedPresentationWorkView::Delta(_) => perform_delta(state, view),
        UiMountedPresentationWorkView::Reconstruction(_) => perform_reconstruction(state, view),
        UiMountedPresentationWorkView::Unchanged(unchanged) => {
            retained_frame::perform_unchanged(state, view, unchanged)
        }
    }
}

fn perform_reconstruction(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
) -> UiHostSurfacePresentationOutcome {
    let key = view.binding().diagnostic_value();
    let defer_initial_observation = defer_presentation_initial_observation(state);
    let Some(graphics) = state.graphics.as_mut() else {
        return adapter_declined();
    };
    let result = present_cold_reconstruction::<UiWgpuNativePresentationPort>(
        graphics,
        &mut state.resources,
        &mut state.physical_signal,
        &state.text_atlas,
        state.text_atlas_gpu.as_ref(),
        view,
        defer_initial_observation,
    );
    let (cost, retained, pixels, port_crossings) = match result {
        Ok(reconstruction) => reconstruction.into_parts(),
        Err(failure) => return settle_presentation_failure(state, view, failure),
    };
    state.retained_draw_lists.insert(key, retained);
    state.reconstruction_required.remove(&key);
    state.effect_posture = UiNativeEffectPosture::Presented;
    retained_frame::record_retained_frame(
        state,
        view,
        key,
        UiNativePresentationWorkKind::Reconstruction,
        pixels,
        cost,
        port_crossings,
    );
    let outcome = completed(state, key, view, cost, true);
    #[cfg(feature = "certification-support")]
    if matches!(&outcome, UiHostSurfacePresentationOutcome::Presented(_)) {
        state.record_qualified_derived_state_reconstruction(key);
    }
    outcome
}

fn perform_initial(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
) -> UiHostSurfacePresentationOutcome {
    let key = view.binding().diagnostic_value();
    if state.retained_draw_lists.contains_key(&key) {
        return malformed();
    }
    let defer_initial_observation = defer_presentation_initial_observation(state);
    let Some(graphics) = state.graphics.as_mut() else {
        return adapter_declined();
    };
    let result = present_initial::<UiWgpuNativePresentationPort>(
        graphics,
        &mut state.resources,
        &mut state.physical_signal,
        &state.text_atlas,
        state.text_atlas_gpu.as_ref(),
        view,
        defer_initial_observation,
    );
    let (observation, cost, retained) = match result {
        Ok(presented) => presented.into_parts(),
        Err(failure) => return settle_presentation_failure(state, view, failure),
    };
    state.effect_posture = UiNativeEffectPosture::Presented;
    state.record_retained_frame_observation(UiNativeRetainedFrameObservation::observed(
        view.frame().diagnostic_value(),
        UiNativePresentationWorkKind::Initial,
        [
            observation.retained_baseline_rgba8(),
            observation.retained_center_rgba8(),
        ],
        cost,
        Some(observation.clone()),
    ));
    state.last_presentation = Some(observation);
    state.retained_draw_lists.insert(key, retained);
    completed(state, key, view, cost, true)
}

fn perform_delta(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
) -> UiHostSurfacePresentationOutcome {
    let key = view.binding().diagnostic_value();
    if state.reconstruction_required.contains(&key) {
        return require_owner_reconstruction(state, key);
    }
    let result = present_delta_work(state, view, key);
    let (cost, painted, observed_pixels, port_crossings) = match result {
        Ok(presented) => presented.into_parts(),
        Err(failure) => return settle_presentation_failure(state, view, failure),
    };
    state.reconstruction_required.remove(&key);
    state.effect_posture = UiNativeEffectPosture::Presented;
    let pixels = observed_pixels.unwrap_or_else(|| retained_frame::latest_pixels(state));
    retained_frame::record_retained_frame(
        state,
        view,
        key,
        UiNativePresentationWorkKind::Delta,
        pixels,
        cost,
        port_crossings,
    );
    completed(state, key, view, cost, painted)
}

fn present_delta_work(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
    key: u64,
) -> Result<crate::native::presentation::UiNativeDeltaPresentation, UiNativePresentationFailure> {
    let defer_initial_observation = defer_presentation_initial_observation(state);
    let Some(graphics) = state.graphics.as_mut() else {
        return Err(before_effects_declined());
    };
    let Some(retained) = state.retained_draw_lists.get_mut(&key) else {
        return Err(before_effects_malformed());
    };
    present_delta::<UiWgpuNativePresentationPort>(
        graphics,
        &mut state.resources,
        &mut state.physical_signal,
        &state.text_atlas,
        state.text_atlas_gpu.as_ref(),
        view,
        retained,
        defer_initial_observation,
    )
}

fn defer_presentation_initial_observation(state: &mut UiNativeHostState) -> bool {
    #[cfg(feature = "certification-support")]
    {
        state
            .qualification
            .defer_next_presentation_initial_observation()
    }
    #[cfg(not(feature = "certification-support"))]
    {
        let _ = state;
        false
    }
}

fn require_owner_reconstruction(
    state: &mut UiNativeHostState,
    key: u64,
) -> UiHostSurfacePresentationOutcome {
    state.retained_draw_lists.remove(&key);
    UiHostSurfacePresentationOutcome::RejectedBeforeEffects(
        worth_ui_host_contract::UiHostSurfacePresentationDenial::ReconstructionRequired,
    )
}

fn completed(
    state: &mut UiNativeHostState,
    key: u64,
    view: &UiMountedFrameConsumptionView<'_>,
    cost: worth_ui_host_contract::UiHostPresentationCostReport,
    painted: bool,
) -> UiHostSurfacePresentationOutcome {
    let Some(epoch) = presentation_epoch(state, key, view.attempt().diagnostic_value(), painted)
    else {
        return malformed();
    };
    let effects = painted
        .then_some(UiMountedEffectFamily::NativePaint)
        .into_iter()
        .collect();
    let outcome =
        UiHostSurfacePresentationOutcome::Presented(UiMountedSurfacePresentationCompletion::new(
            UiHostSurfacePresentationMode::NativeDisplay,
            epoch,
            UiMountedCompletedEffects::new(effects),
            cost,
        ));
    #[cfg(feature = "certification-support")]
    state.apply_completed_qualified_derived_state_loss(key);
    outcome
}

fn presentation_epoch(
    state: &mut UiNativeHostState,
    key: u64,
    attempt: u64,
    painted: bool,
) -> Option<worth_ui_host_contract::UiHostPresentationEpoch> {
    if painted {
        let epoch = worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(attempt);
        state.presentation_epochs.insert(key, epoch);
        return Some(epoch);
    }
    state.presentation_epochs.get(&key).copied()
}

fn settle_presentation_failure(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
    failure: UiNativePresentationFailure,
) -> UiHostSurfacePresentationOutcome {
    match failure {
        UiNativePresentationFailure::BeforeEffects(denial) => {
            UiHostSurfacePresentationOutcome::RejectedBeforeEffects(denial)
        }
        UiNativePresentationFailure::Pending(mut pending) => {
            let token = view.issue_completion_token();
            if !pending.bind_completion_identity(token.diagnostic_value()) {
                return mark_presentation_indeterminate(state);
            }
            #[cfg(feature = "certification-support")]
            {
                let qualification = state
                    .qualification
                    .presentation_external_qualification(pending.physical_work());
                pending.qualify_external_observation(
                    qualification.effects_indeterminate(),
                    qualification.duplicate_completed(),
                );
            }
            state.pending_presentations.push(pending);
            UiHostSurfacePresentationOutcome::InFlight(token)
        }
    }
}

pub(super) fn mark_presentation_indeterminate(
    state: &mut UiNativeHostState,
) -> UiHostSurfacePresentationOutcome {
    state.effect_posture = UiNativeEffectPosture::PresentationIndeterminate;
    UiHostSurfacePresentationOutcome::PresentationIndeterminate
}

fn malformed() -> UiHostSurfacePresentationOutcome {
    UiHostSurfacePresentationOutcome::RejectedBeforeEffects(
        worth_ui_host_contract::UiHostSurfacePresentationDenial::MalformedProjection,
    )
}

pub(super) fn adapter_declined() -> UiHostSurfacePresentationOutcome {
    UiHostSurfacePresentationOutcome::RejectedBeforeEffects(
        worth_ui_host_contract::UiHostSurfacePresentationDenial::AdapterDeclined,
    )
}

fn before_effects_malformed() -> UiNativePresentationFailure {
    UiNativePresentationFailure::BeforeEffects(
        worth_ui_host_contract::UiHostSurfacePresentationDenial::MalformedProjection,
    )
}

fn before_effects_declined() -> UiNativePresentationFailure {
    UiNativePresentationFailure::BeforeEffects(
        worth_ui_host_contract::UiHostSurfacePresentationDenial::AdapterDeclined,
    )
}

#[cfg(test)]
#[path = "presentation_tests.rs"]
mod tests;

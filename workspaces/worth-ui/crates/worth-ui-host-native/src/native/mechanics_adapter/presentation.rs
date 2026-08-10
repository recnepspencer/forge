use worth_ui_host_contract::{
    UiHostSurfacePresentationMode, UiHostSurfacePresentationOutcome, UiMountedCompletedEffects,
    UiMountedEffectFamily, UiMountedFrameConsumptionView, UiMountedPresentationWorkView,
    UiMountedSurfacePresentationCompletion,
};

use crate::native::{
    presentation::{
        present_delta, present_initial, present_reconstruction, UiNativePresentationFailure,
        UiWgpuNativePresentationPort,
    },
    UiNativeEffectPosture, UiNativeHostState,
};

pub(super) fn perform_native_presentation(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
) -> UiHostSurfacePresentationOutcome {
    if !state.pending_presentations.is_empty() {
        return mark_presentation_indeterminate(state);
    }
    match view.presentation_work() {
        UiMountedPresentationWorkView::Initial(_) => perform_initial(state, view),
        UiMountedPresentationWorkView::Delta(_) => perform_delta(state, view),
        UiMountedPresentationWorkView::Unchanged(unchanged) => {
            perform_unchanged(state, view, unchanged)
        }
    }
}

fn perform_initial(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
) -> UiHostSurfacePresentationOutcome {
    let key = view.binding().diagnostic_value();
    if state.retained_draw_lists.contains_key(&key) {
        return malformed();
    }
    let Some(graphics) = state.graphics.as_mut() else {
        return adapter_declined();
    };
    let result =
        present_initial::<UiWgpuNativePresentationPort>(graphics, &mut state.resources, view);
    let (observation, cost, retained) = match result {
        Ok(presented) => presented.into_parts(),
        Err(failure) => return settle_presentation_failure(state, failure),
    };
    state.effect_posture = UiNativeEffectPosture::Presented;
    state.last_presentation = Some(observation);
    state.retained_draw_lists.insert(key, retained);
    completed(view, cost, true)
}

fn perform_delta(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
) -> UiHostSurfacePresentationOutcome {
    let key = view.binding().diagnostic_value();
    let reconstruct = state.reconstruction_required.contains(&key);
    let result = present_delta_work(state, view, key, reconstruct);
    let (cost, painted) = match result {
        Ok(presented) => presented.into_parts(),
        Err(failure) => {
            if matches!(failure, UiNativePresentationFailure::Indeterminate(_)) {
                state.reconstruction_required.insert(key);
            }
            return settle_presentation_failure(state, failure);
        }
    };
    state.reconstruction_required.remove(&key);
    state.effect_posture = UiNativeEffectPosture::Presented;
    state.last_presentation = None;
    completed(view, cost, painted)
}

fn present_delta_work(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
    key: u64,
    reconstruct: bool,
) -> Result<crate::native::presentation::UiNativeDeltaPresentation, UiNativePresentationFailure> {
    let Some(graphics) = state.graphics.as_mut() else {
        return Err(before_effects_declined());
    };
    let Some(retained) = state.retained_draw_lists.get_mut(&key) else {
        return Err(before_effects_malformed());
    };
    present_delta::<UiWgpuNativePresentationPort>(
        graphics,
        &mut state.resources,
        view,
        retained,
        reconstruct,
    )
}

fn perform_unchanged(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
    unchanged: &worth_ui_host_contract::UiMountedPresentationUnchanged,
) -> UiHostSurfacePresentationOutcome {
    let key = view.binding().diagnostic_value();
    if !state.reconstruction_required.contains(&key) {
        return retain_unchanged(state, view, unchanged, key);
    }
    let result = reconstruct_unchanged(state, unchanged, key);
    let cost = match result {
        Ok(cost) => cost,
        Err(failure) => return settle_presentation_failure(state, failure),
    };
    state.reconstruction_required.remove(&key);
    state.effect_posture = UiNativeEffectPosture::Presented;
    state.last_presentation = None;
    completed(view, cost, true)
}

fn retain_unchanged(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
    unchanged: &worth_ui_host_contract::UiMountedPresentationUnchanged,
    key: u64,
) -> UiHostSurfacePresentationOutcome {
    let Some(retained) = state.retained_draw_lists.get_mut(&key) else {
        return malformed();
    };
    if retained.apply_unchanged(unchanged).is_err() {
        return malformed();
    }
    state.last_presentation = None;
    completed(view, Default::default(), false)
}

fn reconstruct_unchanged(
    state: &mut UiNativeHostState,
    unchanged: &worth_ui_host_contract::UiMountedPresentationUnchanged,
    key: u64,
) -> Result<worth_ui_host_contract::UiHostPresentationCostReport, UiNativePresentationFailure> {
    let Some(graphics) = state.graphics.as_mut() else {
        return Err(before_effects_declined());
    };
    let Some(retained) = state.retained_draw_lists.get_mut(&key) else {
        return Err(before_effects_malformed());
    };
    let predecessor = retained
        .stage_unchanged(unchanged)
        .map_err(|_| before_effects_malformed())?;
    let result = present_reconstruction::<UiWgpuNativePresentationPort>(
        graphics,
        &mut state.resources,
        retained,
    );
    if result.is_err() {
        retained.rollback_unchanged(predecessor);
    }
    result
}

fn completed(
    view: &UiMountedFrameConsumptionView<'_>,
    cost: worth_ui_host_contract::UiHostPresentationCostReport,
    painted: bool,
) -> UiHostSurfacePresentationOutcome {
    let effects = painted
        .then_some(UiMountedEffectFamily::NativePaint)
        .into_iter()
        .collect();
    UiHostSurfacePresentationOutcome::Presented(UiMountedSurfacePresentationCompletion::new(
        UiHostSurfacePresentationMode::NativeDisplay,
        worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(
            view.attempt().diagnostic_value(),
        ),
        UiMountedCompletedEffects::new(effects),
        cost,
    ))
}

fn settle_presentation_failure(
    state: &mut UiNativeHostState,
    failure: UiNativePresentationFailure,
) -> UiHostSurfacePresentationOutcome {
    match failure {
        UiNativePresentationFailure::BeforeEffects(denial) => {
            UiHostSurfacePresentationOutcome::RejectedBeforeEffects(denial)
        }
        UiNativePresentationFailure::Indeterminate(pending) => {
            state.pending_presentations.push(pending);
            mark_presentation_indeterminate(state)
        }
    }
}

fn mark_presentation_indeterminate(
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

fn adapter_declined() -> UiHostSurfacePresentationOutcome {
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

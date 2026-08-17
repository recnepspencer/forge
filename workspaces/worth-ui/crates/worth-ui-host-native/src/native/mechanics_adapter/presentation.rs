use worth_ui_host_contract::{
    UiHostSurfacePresentationMode, UiHostSurfacePresentationOutcome, UiMountedCompletedEffects,
    UiMountedEffectFamily, UiMountedFrameConsumptionView, UiMountedPresentationWorkView,
    UiMountedSurfacePresentationCompletion,
};

use crate::native::{
    presentation::{
        observation_for_retained, present_cold_reconstruction, present_delta, present_initial,
        UiNativePresentationFailure, UiWgpuNativePresentationPort,
    },
    UiNativeEffectPosture, UiNativeHostState, UiNativePresentationWorkKind,
    UiNativeRetainedFrameObservation,
};

pub(super) fn perform_native_presentation(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
) -> UiHostSurfacePresentationOutcome {
    if let Some(outcome) = perform_mounted_text_work(state, view) {
        return outcome;
    }
    match view.presentation_work() {
        UiMountedPresentationWorkView::Initial(_) => perform_initial(state, view),
        UiMountedPresentationWorkView::Delta(_) => perform_delta(state, view),
        UiMountedPresentationWorkView::Reconstruction(_) => perform_reconstruction(state, view),
        UiMountedPresentationWorkView::Unchanged(unchanged) => {
            perform_unchanged(state, view, unchanged)
        }
    }
}

fn perform_mounted_text_work(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
) -> Option<UiHostSurfacePresentationOutcome> {
    let work = view.text_raster_work()?;
    struct CallbackRasterizer<'work>(&'work worth_ui_host_contract::UiMountedTextRasterWork<'work>);
    impl worth_ui_host_contract::UiGlyphRasterMissRasterizer for CallbackRasterizer<'_> {
        fn rasterize(
            &mut self,
            misses: worth_ui_host_contract::UiGlyphRasterMissSelectionView<'_>,
            sink: &mut dyn worth_ui_host_contract::UiGlyphRasterBatchSink,
        ) -> Result<(), worth_ui_host_contract::UiGlyphRasterCallbackDenial> {
            self.0.rasterize(misses, sink)
        }
    }
    let mut rasterizer = CallbackRasterizer(work);
    let outcome = super::text_atlas::perform(
        state,
        crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::from_view(view),
        work.demands(),
        work.pins(),
        &mut rasterizer,
    );
    if matches!(
        outcome,
        worth_ui_host_contract::UiGlyphRasterTransactionOutcome::Committed(_)
            | worth_ui_host_contract::UiGlyphRasterTransactionOutcome::Pending(_)
    ) {
        state.text_pins_by_binding.insert(
            view.binding().diagnostic_value(),
            work.binding_pins().to_vec().into_boxed_slice(),
        );
    }
    match outcome {
        worth_ui_host_contract::UiGlyphRasterTransactionOutcome::Committed(_) => None,
        worth_ui_host_contract::UiGlyphRasterTransactionOutcome::Pending(_) => Some(
            UiHostSurfacePresentationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfacePresentationDenial::TextAtlasPresentationDeferred,
            ),
        ),
        worth_ui_host_contract::UiGlyphRasterTransactionOutcome::RejectedBeforeEffects(_)
        | worth_ui_host_contract::UiGlyphRasterTransactionOutcome::RejectedAfterRasterization(_) => {
            Some(adapter_declined())
        }
        worth_ui_host_contract::UiGlyphRasterTransactionOutcome::EffectsIndeterminate(_) => {
            Some(mark_presentation_indeterminate(state))
        }
    }
}

fn perform_reconstruction(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
) -> UiHostSurfacePresentationOutcome {
    let key = view.binding().diagnostic_value();
    let Some(graphics) = state.graphics.as_mut() else {
        return adapter_declined();
    };
    let result = present_cold_reconstruction::<UiWgpuNativePresentationPort>(
        graphics,
        &mut state.resources,
        &mut state.physical_signal,
        view,
    );
    let (cost, retained, pixels, port_crossings) = match result {
        Ok(reconstruction) => reconstruction.into_parts(),
        Err(failure) => return settle_presentation_failure(state, failure),
    };
    state.retained_draw_lists.insert(key, retained);
    state.reconstruction_required.remove(&key);
    state.effect_posture = UiNativeEffectPosture::Presented;
    record_retained_frame(
        state,
        view,
        key,
        UiNativePresentationWorkKind::Reconstruction,
        pixels,
        cost,
        port_crossings,
    );
    completed(state, key, view, cost, true)
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
    let result = present_initial::<UiWgpuNativePresentationPort>(
        graphics,
        &mut state.resources,
        &mut state.physical_signal,
        view,
    );
    let (observation, cost, retained) = match result {
        Ok(presented) => presented.into_parts(),
        Err(failure) => return settle_presentation_failure(state, failure),
    };
    state.effect_posture = UiNativeEffectPosture::Presented;
    state
        .retained_frame_observations
        .push(UiNativeRetainedFrameObservation::observed(
            view.frame().diagnostic_value(),
            UiNativePresentationWorkKind::Initial,
            [
                observation.retained_baseline_rgba8(),
                observation.retained_center_rgba8(),
            ],
            cost,
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
        Err(failure) => {
            if matches!(failure, UiNativePresentationFailure::Indeterminate(_)) {
                state.reconstruction_required.insert(key);
            }
            return settle_presentation_failure(state, failure);
        }
    };
    state.reconstruction_required.remove(&key);
    state.effect_posture = UiNativeEffectPosture::Presented;
    let pixels = observed_pixels.unwrap_or_else(|| latest_pixels(state));
    record_retained_frame(
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
        view,
        retained,
    )
}

fn perform_unchanged(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
    unchanged: &worth_ui_host_contract::UiMountedPresentationUnchanged,
) -> UiHostSurfacePresentationOutcome {
    let key = view.binding().diagnostic_value();
    if state.reconstruction_required.contains(&key) {
        return require_owner_reconstruction(state, key);
    }
    retain_unchanged(state, view, unchanged, key)
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
    completed(state, key, view, Default::default(), false)
}

fn record_retained_frame(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
    key: u64,
    kind: UiNativePresentationWorkKind,
    pixels: [[u8; 4]; 2],
    cost: worth_ui_host_contract::UiHostPresentationCostReport,
    port_crossings: u8,
) {
    state
        .retained_frame_observations
        .push(UiNativeRetainedFrameObservation::observed(
            view.frame().diagnostic_value(),
            kind,
            pixels,
            cost,
        ));
    state.last_presentation = state
        .graphics
        .as_ref()
        .zip(state.retained_draw_lists.get(&key))
        .and_then(|(graphics, retained)| {
            observation_for_retained(view, graphics, retained, pixels, cost, port_crossings)
        });
}

fn latest_pixels(state: &UiNativeHostState) -> [[u8; 4]; 2] {
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
    UiHostSurfacePresentationOutcome::Presented(UiMountedSurfacePresentationCompletion::new(
        UiHostSurfacePresentationMode::NativeDisplay,
        epoch,
        UiMountedCompletedEffects::new(effects),
        cost,
    ))
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

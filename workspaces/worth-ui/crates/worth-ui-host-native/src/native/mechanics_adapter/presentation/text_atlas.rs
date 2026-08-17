use worth_ui_host_contract::{
    UiHostPresentationCompletionToken, UiHostSurfaceCancellationOutcome,
    UiHostSurfaceInFlightCompletion, UiHostSurfacePresentationOutcome,
    UiMountedFrameConsumptionView,
};

use crate::native::{
    host_state::{UiNativePendingTextContinuation, UiNativePendingTextPresentation},
    UiNativeEffectPosture, UiNativeHostState,
};

pub(super) enum UiMountedTextWorkOutcome {
    Ready,
    Pending(
        (
            worth_ui_host_contract::UiGlyphRasterTransactionPending,
            Box<[worth_ui_host_contract::UiGlyphRasterPinRequest]>,
        ),
    ),
    Terminal(UiHostSurfacePresentationOutcome),
}

pub(super) fn begin(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
) -> UiMountedTextWorkOutcome {
    let Some(work) = view.text_raster_work() else {
        return UiMountedTextWorkOutcome::Ready;
    };
    let mut rasterizer = CallbackRasterizer(work);
    let outcome = super::text_atlas::perform(
        state,
        crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::from_view(view),
        work.demands(),
        work.pins(),
        &mut rasterizer,
    );
    match outcome {
        worth_ui_host_contract::UiGlyphRasterTransactionOutcome::Committed(_) => {
            state.text_pins_by_binding.insert(
                view.binding().diagnostic_value(),
                work.binding_pins().to_vec().into_boxed_slice(),
            );
            UiMountedTextWorkOutcome::Ready
        }
        worth_ui_host_contract::UiGlyphRasterTransactionOutcome::Pending(pending) => {
            UiMountedTextWorkOutcome::Pending((
                pending,
                work.binding_pins().to_vec().into_boxed_slice(),
            ))
        }
        worth_ui_host_contract::UiGlyphRasterTransactionOutcome::RejectedBeforeEffects(_) => {
            UiMountedTextWorkOutcome::Terminal(super::presentation::adapter_declined())
        }
        worth_ui_host_contract::UiGlyphRasterTransactionOutcome::RejectedAfterRasterization(_)
        | worth_ui_host_contract::UiGlyphRasterTransactionOutcome::EffectsIndeterminate(_) => {
            UiMountedTextWorkOutcome::Terminal(
                super::presentation::mark_presentation_indeterminate(state),
            )
        }
    }
}

pub(super) fn retain_pending(
    state: &mut UiNativeHostState,
    view: &UiMountedFrameConsumptionView<'_>,
    token: &UiHostPresentationCompletionToken,
    (atlas, binding_pins): (
        worth_ui_host_contract::UiGlyphRasterTransactionPending,
        Box<[worth_ui_host_contract::UiGlyphRasterPinRequest]>,
    ),
    continuation: UiNativePendingTextContinuation,
) {
    state.pending_text_presentations.insert(
        token.diagnostic_value(),
        UiNativePendingTextPresentation {
            atlas,
            continuation,
            binding: view.binding().diagnostic_value(),
            binding_pins,
        },
    );
}

pub(super) fn complete(
    state: &mut UiNativeHostState,
    token: UiHostPresentationCompletionToken,
) -> UiHostSurfaceInFlightCompletion {
    let key = token.diagnostic_value();
    let Some(mut pending) = state.pending_text_presentations.remove(&key) else {
        return UiHostSurfaceInFlightCompletion::RejectedBeforeEffects(
            worth_ui_host_contract::UiHostSurfacePresentationDenial::AdapterDeclined,
        );
    };
    match state.complete_pending_text_atlas(pending.atlas) {
        worth_ui_host_contract::UiGlyphRasterTransactionOutcome::Pending(next) => {
            pending.atlas = next;
            state.pending_text_presentations.insert(key, pending);
            UiHostSurfaceInFlightCompletion::Pending(token)
        }
        worth_ui_host_contract::UiGlyphRasterTransactionOutcome::Committed(_) => {
            state
                .text_pins_by_binding
                .insert(pending.binding, pending.binding_pins);
            match pending.continuation {
                UiNativePendingTextContinuation::Presented(completion) => {
                    UiHostSurfaceInFlightCompletion::Presented(completion)
                }
                UiNativePendingTextContinuation::AtlasReady => {
                    UiHostSurfaceInFlightCompletion::RejectedBeforeEffects(
                        worth_ui_host_contract::UiHostSurfacePresentationDenial::TextAtlasPresentationDeferred,
                    )
                }
            }
        }
        worth_ui_host_contract::UiGlyphRasterTransactionOutcome::RejectedBeforeEffects(_)
        | worth_ui_host_contract::UiGlyphRasterTransactionOutcome::RejectedAfterRasterization(_)
        | worth_ui_host_contract::UiGlyphRasterTransactionOutcome::EffectsIndeterminate(_) => {
            state.effect_posture = UiNativeEffectPosture::PresentationIndeterminate;
            UiHostSurfaceInFlightCompletion::PresentationIndeterminate
        }
    }
}

pub(super) fn cancel(
    state: &mut UiNativeHostState,
    token: UiHostPresentationCompletionToken,
) -> UiHostSurfaceCancellationOutcome {
    let Some(pending) = state
        .pending_text_presentations
        .remove(&token.diagnostic_value())
    else {
        return UiHostSurfaceCancellationOutcome::CancelledBeforeEffects;
    };
    let _ = state.cancel_pending_text_atlas(pending.atlas);
    UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun
}

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

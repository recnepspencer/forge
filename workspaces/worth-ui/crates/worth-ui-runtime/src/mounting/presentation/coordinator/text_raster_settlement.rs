use worth_ui_host_contract::{
    UiGlyphRasterTransactionDenial, UiGlyphRasterTransactionOutcome,
    UiHostSurfacePresentationDenial, UiSurfaceBindingGeneration,
};

use super::super::state::UiPendingMountedTextRaster;
use super::super::UiMountedSurfacePresentationRejection;
use super::UiMountedPresentationProgress;
use crate::facade::UiHostEffectPort;

pub(super) fn observe_pending(
    host: UiHostEffectPort<'_>,
    pending: Vec<UiPendingMountedTextRaster>,
    progress: &mut UiMountedPresentationProgress,
    text: &mut crate::native_platform::text_presentation::UiNativeMountedTextCoordinator,
) -> Option<UiSurfaceBindingGeneration> {
    let mut iter = pending.into_iter();
    while let Some(pending) = iter.next() {
        let UiPendingMountedTextRaster { binding, pending } = pending;
        match text.complete(host, pending) {
            crate::native_platform::text_presentation::UiNativeMountedTextOutcome::Pending(
                pending,
            ) => {
                progress.pending_text.push(UiPendingMountedTextRaster {
                    binding,
                    pending,
                });
            }
            crate::native_platform::text_presentation::UiNativeMountedTextOutcome::Committed {
                ..
            } => {
                progress
                    .rejected
                    .push(UiMountedSurfacePresentationRejection::new(
                        binding,
                        UiHostSurfacePresentationDenial::TextAtlasPresentationDeferred,
                    ));
            }
            crate::native_platform::text_presentation::UiNativeMountedTextOutcome::RejectedBeforeEffects { denial, .. }
            | crate::native_platform::text_presentation::UiNativeMountedTextOutcome::RejectedAfterRasterization { denial, .. } => {
                progress
                    .rejected
                    .push(UiMountedSurfacePresentationRejection::new(
                        binding,
                        map_denial(denial),
                    ));
            }
            crate::native_platform::text_presentation::UiNativeMountedTextOutcome::EffectsIndeterminate { .. } => {
                progress.pending_text.extend(iter);
                return Some(binding);
            }
        }
    }
    None
}

pub(super) fn map_denial(
    denial: UiGlyphRasterTransactionDenial,
) -> UiHostSurfacePresentationDenial {
    match denial {
        UiGlyphRasterTransactionDenial::CapacityExceeded
        | UiGlyphRasterTransactionDenial::PinnedCapacityExceeded => {
            UiHostSurfacePresentationDenial::CapacityExceeded
        }
        UiGlyphRasterTransactionDenial::ReconstructionRequired => {
            UiHostSurfacePresentationDenial::ReconstructionRequired
        }
        _ => UiHostSurfacePresentationDenial::AdapterDeclined,
    }
}

pub(super) fn cancel_all_text(
    pending: Vec<UiPendingMountedTextRaster>,
    host: UiHostEffectPort<'_>,
) -> bool {
    pending.into_iter().any(|pending| {
        !matches!(
            crate::native_platform::text_presentation::UiNativeMountedTextCoordinator::cancel(
                host,
                pending.pending,
            ),
            UiGlyphRasterTransactionOutcome::RejectedBeforeEffects(_)
                | UiGlyphRasterTransactionOutcome::RejectedAfterRasterization(_)
        )
    })
}

pub(super) fn extend_bindings(
    affected: &mut Vec<UiSurfaceBindingGeneration>,
    pending: &[UiPendingMountedTextRaster],
) {
    for binding in pending.iter().map(|pending| pending.binding) {
        if !affected.contains(&binding) {
            affected.push(binding);
        }
    }
}

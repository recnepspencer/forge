//! Typed logical atlas settlement after physical Signal reconciliation.

use worth_ui_host_contract::{
    UiGlyphRasterTransactionDenial, UiGlyphRasterTransactionOutcome,
    UiGlyphRasterTransactionReceipt,
};

use crate::native::text_atlas::{
    UiNativeTextAtlasCommitOutcome, UiNativeTextAtlasExternalOutcome,
    UiNativeTextAtlasTransactionPlan, UiNativeTextAtlasUpload,
};
use crate::native::UiNativeHostState;

pub(super) fn reject_plan(
    state: &mut UiNativeHostState,
    plan: UiNativeTextAtlasTransactionPlan,
    denial: UiGlyphRasterTransactionDenial,
) -> UiGlyphRasterTransactionOutcome {
    let _ = state
        .text_atlas
        .settle(plan, &[], UiNativeTextAtlasExternalOutcome::Rejected);
    UiGlyphRasterTransactionOutcome::RejectedBeforeEffects(denial)
}

pub(super) fn reject_after_rasterization(
    state: &mut UiNativeHostState,
    plan: UiNativeTextAtlasTransactionPlan,
    denial: UiGlyphRasterTransactionDenial,
) -> UiGlyphRasterTransactionOutcome {
    let _ = state
        .text_atlas
        .settle(plan, &[], UiNativeTextAtlasExternalOutcome::Rejected);
    UiGlyphRasterTransactionOutcome::RejectedAfterRasterization(denial)
}

pub(super) fn settle_plan(
    state: &mut UiNativeHostState,
    plan: UiNativeTextAtlasTransactionPlan,
    uploads: Vec<UiNativeTextAtlasUpload>,
    external: UiNativeTextAtlasExternalOutcome,
) -> UiGlyphRasterTransactionOutcome {
    match state.text_atlas.settle(plan, &uploads, external) {
        UiNativeTextAtlasCommitOutcome::Committed(receipt) => {
            UiGlyphRasterTransactionOutcome::Committed(
                UiGlyphRasterTransactionReceipt::from_text_mechanics(
                    receipt.generation.get(),
                    receipt.misses,
                    receipt.hits,
                    receipt.evictions,
                    receipt.committed_pins,
                    receipt.staged_bytes,
                    receipt.physical_staged_bytes,
                    receipt.peak_entries,
                    receipt.peak_texel_bytes,
                ),
            )
        }
        UiNativeTextAtlasCommitOutcome::Denied(denial) => {
            UiGlyphRasterTransactionOutcome::RejectedBeforeEffects(map_denial(denial))
        }
        UiNativeTextAtlasCommitOutcome::EffectsIndeterminate(recovery) => {
            let demand = recovery.demand_identity();
            let generation = recovery.generation().get();
            state.text_atlas_recovery = Some(recovery);
            UiGlyphRasterTransactionOutcome::EffectsIndeterminate(
                worth_ui_host_contract::UiGlyphRasterEffectsIndeterminate::from_text_mechanics(
                    demand, generation,
                ),
            )
        }
    }
}

pub(super) fn map_denial(
    denial: crate::native::text_atlas::UiNativeTextAtlasDenial,
) -> UiGlyphRasterTransactionDenial {
    use crate::native::text_atlas::UiNativeTextAtlasDenial as Native;
    match denial {
        Native::ReservationConflict => UiGlyphRasterTransactionDenial::ReservationConflict,
        Native::GenerationExhausted => UiGlyphRasterTransactionDenial::GenerationExhausted,
        Native::StalePlan => UiGlyphRasterTransactionDenial::StalePlan,
        Native::StalePin => UiGlyphRasterTransactionDenial::StalePin,
        Native::ReconstructionRequired => UiGlyphRasterTransactionDenial::ReconstructionRequired,
        Native::PinnedCapacityExceeded => UiGlyphRasterTransactionDenial::PinnedCapacityExceeded,
        Native::RasterGeometryMismatch => UiGlyphRasterTransactionDenial::RasterGeometryMismatch,
        Native::RasterBatchMismatch | Native::UploadRejected => {
            UiGlyphRasterTransactionDenial::RasterBatchMismatch
        }
        Native::PageCapacityExceeded
        | Native::EntryCapacityExceeded
        | Native::TexelCapacityExceeded
        | Native::StagingCapacityExceeded
        | Native::GlyphExtentExceeded => UiGlyphRasterTransactionDenial::CapacityExceeded,
        Native::MalformedDemand | Native::StaleDemand => {
            UiGlyphRasterTransactionDenial::MalformedDemand
        }
        Native::PinConflict => UiGlyphRasterTransactionDenial::StalePlan,
    }
}

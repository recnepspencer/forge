use worth_ui_host_contract::{UiGlyphRasterTransactionOutcome, UiGlyphRasterTransactionReceipt};

use super::text_atlas_lifecycle::{map_atlas_denial, stale_plan};
use super::UiNativeHostState;
use crate::native::text_atlas::{
    UiNativeTextAtlasCommitOutcome, UiNativeTextAtlasExternalOutcome, UiNativeTextAtlasInFlight,
};

impl UiNativeHostState {
    pub(crate) fn commit_text_atlas_in_flight(&mut self) -> UiGlyphRasterTransactionOutcome {
        let Some(in_flight) = self.text_atlas_in_flight.take() else {
            return stale_plan();
        };
        let pending = in_flight.pending();
        let signal_token = in_flight.signal_token();
        let Some((plan, uploads)) = in_flight.into_commit_parts() else {
            self.text_atlas_in_flight =
                Some(UiNativeTextAtlasInFlight::recovery(pending, signal_token));
            return stale_plan();
        };
        match self
            .text_atlas
            .settle(plan, &uploads, UiNativeTextAtlasExternalOutcome::Submitted)
        {
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
                UiGlyphRasterTransactionOutcome::RejectedBeforeEffects(map_atlas_denial(denial))
            }
            UiNativeTextAtlasCommitOutcome::EffectsIndeterminate(recovery) => {
                let generation = recovery.generation().get();
                self.text_atlas_recovery = Some(recovery);
                UiGlyphRasterTransactionOutcome::EffectsIndeterminate(
                    worth_ui_host_contract::UiGlyphRasterEffectsIndeterminate::from_text_mechanics(
                        pending.demand_identity(),
                        generation,
                    ),
                )
            }
        }
    }
}

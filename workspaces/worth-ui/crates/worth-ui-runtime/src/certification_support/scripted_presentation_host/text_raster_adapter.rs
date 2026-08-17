use worth_ui_host_contract::{
    UiAlphaRasterBatchView, UiColorRasterBatchView, UiGlyphRasterBatchSink,
    UiGlyphRasterBatchSubmissionDenial, UiGlyphRasterMissRasterizer,
    UiGlyphRasterMissSelectionView, UiGlyphRasterPinRequest, UiGlyphRasterPinTransitionView,
    UiGlyphRasterTransactionDenial, UiGlyphRasterTransactionOutcome,
    UiGlyphRasterTransactionReceipt, UiMountedFrameConsumptionView,
};

use crate::facade::host::UiHostAdapterSessionAuthority;

use super::ScriptedPresentationHost;

pub(super) fn prepare(
    host: &ScriptedPresentationHost,
    authority: &UiHostAdapterSessionAuthority,
    view: &UiMountedFrameConsumptionView<'_>,
    demands: &[worth_ui_host_contract::UiGlyphRasterDemandBatchView<'_>],
    pins: UiGlyphRasterPinTransitionView<'_>,
    rasterizer: &mut dyn UiGlyphRasterMissRasterizer,
) -> UiGlyphRasterTransactionOutcome {
    if !authority.admits_mounted_presentation(view) {
        return UiGlyphRasterTransactionOutcome::RejectedBeforeEffects(
            UiGlyphRasterTransactionDenial::StaleDemand,
        );
    }
    let mut sink = ScriptedRasterSink::default();
    for demand in demands {
        let selection = UiGlyphRasterMissSelectionView::from_text_mechanics(
            demand.identity(),
            demand.layout_identity(),
            demand.lane(),
            demand.records(),
        );
        if rasterizer.rasterize(selection, &mut sink).is_err() {
            return UiGlyphRasterTransactionOutcome::RejectedAfterRasterization(
                UiGlyphRasterTransactionDenial::CallbackRejected,
            );
        }
    }
    let mut state = host.state.lock().unwrap();
    state.text_raster_calls = state.text_raster_calls.saturating_add(1);
    state.text_rasterized_records = state
        .text_rasterized_records
        .saturating_add(sink.alpha_records + sink.color_records);
    apply_pins(&mut state.live_text_pins, pins);
    UiGlyphRasterTransactionOutcome::Committed(
        UiGlyphRasterTransactionReceipt::from_text_mechanics(
            state.text_raster_calls as u64,
            u32::try_from(sink.alpha_records + sink.color_records).unwrap_or(u32::MAX),
            0,
            0,
            u32::try_from(state.live_text_pins.len()).unwrap_or(u32::MAX),
            sink.staged_bytes,
            sink.staged_bytes,
            u32::try_from(state.live_text_pins.len()).unwrap_or(u32::MAX),
            sink.staged_bytes,
        ),
    )
}

pub(super) fn release(
    host: &ScriptedPresentationHost,
    authority: &UiHostAdapterSessionAuthority,
    request: worth_ui_host_contract::UiMountedTextPinReleaseRequest,
    pins: UiGlyphRasterPinTransitionView<'_>,
) -> UiGlyphRasterTransactionOutcome {
    if request.surface().host_session_identity() != authority.host_session_identity()
        || !pins.additions().is_empty()
    {
        return UiGlyphRasterTransactionOutcome::RejectedBeforeEffects(
            UiGlyphRasterTransactionDenial::StalePin,
        );
    }
    let mut state = host.state.lock().unwrap();
    apply_pins(&mut state.live_text_pins, pins);
    UiGlyphRasterTransactionOutcome::Committed(
        UiGlyphRasterTransactionReceipt::from_text_mechanics(
            state.text_raster_calls as u64,
            0,
            u32::try_from(state.live_text_pins.len()).unwrap_or(u32::MAX),
            0,
            0,
            0,
            0,
            u32::try_from(state.live_text_pins.len()).unwrap_or(u32::MAX),
            0,
        ),
    )
}

fn apply_pins(live: &mut Vec<UiGlyphRasterPinRequest>, pins: UiGlyphRasterPinTransitionView<'_>) {
    for release in pins.releases() {
        if let Some(index) = live.iter().position(|current| current == release) {
            live.remove(index);
        }
    }
    for addition in pins.additions() {
        if !live.contains(addition) {
            live.push(*addition);
        }
    }
}

#[derive(Default)]
struct ScriptedRasterSink {
    alpha_records: usize,
    color_records: usize,
    staged_bytes: u64,
}

impl UiGlyphRasterBatchSink for ScriptedRasterSink {
    fn submit_alpha(
        &mut self,
        batch: UiAlphaRasterBatchView<'_, '_>,
    ) -> Result<(), UiGlyphRasterBatchSubmissionDenial> {
        self.alpha_records = self.alpha_records.saturating_add(batch.records().len());
        self.staged_bytes = self.staged_bytes.saturating_add(
            batch
                .records()
                .iter()
                .map(|record| record.pixels().len() as u64)
                .sum::<u64>(),
        );
        Ok(())
    }

    fn submit_color(
        &mut self,
        batch: UiColorRasterBatchView<'_, '_>,
    ) -> Result<(), UiGlyphRasterBatchSubmissionDenial> {
        self.color_records = self.color_records.saturating_add(batch.records().len());
        self.staged_bytes = self.staged_bytes.saturating_add(
            batch
                .records()
                .iter()
                .map(|record| record.pixels().len() as u64)
                .sum::<u64>(),
        );
        Ok(())
    }
}

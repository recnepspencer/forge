use super::*;
use worth_ui_host_contract::{
    UiColorRasterBatchView, UiColorRasterRecordView, UiGlyphRasterTransactionReceipt,
};

struct MixedSubmittingRasterizer;

impl UiGlyphRasterMissRasterizer for MixedSubmittingRasterizer {
    fn rasterize(
        &mut self,
        misses: UiGlyphRasterMissSelectionView<'_>,
        sink: &mut dyn UiGlyphRasterBatchSink,
    ) -> Result<(), UiGlyphRasterCallbackDenial> {
        let demanded = misses.records();
        if demanded.iter().all(|record| {
            matches!(
                record.key().source(),
                UiGlyphRasterSource::AlphaOutline | UiGlyphRasterSource::LastResort
            )
        }) {
            return signal_failure_tests::SubmittingRasterizer.rasterize(misses, sink);
        }
        let pixels = demanded
            .iter()
            .map(|record| {
                vec![
                    0_u8;
                    usize::try_from(record.extent().width() * record.extent().height() * 4).unwrap()
                ]
            })
            .collect::<Vec<_>>();
        let records = demanded
            .iter()
            .zip(&pixels)
            .map(|(record, pixels)| {
                UiColorRasterRecordView::from_text_mechanics(UiGlyphRasterRecordViewInput {
                    key: record.key(),
                    attribution: record.attribution(),
                    bearing: UiGlyphRasterBearing::from_sixty_fourths(0, 0),
                    extent: record.extent(),
                    stride: record.extent().width() * 4,
                    pixels,
                    digest: UiGlyphRasterContentDigest::from_text_mechanics(
                        Sha256::digest(pixels).into(),
                    ),
                })
                .map_err(|_| UiGlyphRasterCallbackDenial::RasterizationDenied)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let (miss, batch) = super::text_atlas_upload_sink::expected_color_batch_identity(
            misses.demand_identity(),
            misses.layout_identity(),
            1_000,
            1,
            misses.lane(),
            &records,
        );
        sink.submit_color(UiColorRasterBatchView::from_text_mechanics(
            misses.demand_identity(),
            miss,
            batch,
            misses.layout_identity(),
            misses.lane(),
            &records,
        ))
        .map_err(UiGlyphRasterCallbackDenial::BatchRejected)
    }
}

#[test]
#[ignore = "requires the serialized interactive Windows 11 DX12 desktop"]
fn real_dx12_signal_transaction_matches_the_independent_atlas_model_and_closes_exactly() {
    let mut state = crate::native::UiNativeHostState::new();
    let mut rasterizer = MixedSubmittingRasterizer;
    let mut port = super::text_atlas_dx12_upload_port::QualifiedDx12UploadPort::new();
    exercise_real_dx12_commit(&mut state, &mut rasterizer, &mut port);
    port.defer_settlement();
    exercise_temporal_recovery(&mut state, &mut rasterizer, &mut port);
    assert!(state.text_atlas.census().is_zero());
    let signal = state.physical_signal.observation();
    assert!(signal.runtime_owned);
    assert!(signal.signal_performed_transitions > 0);
    assert!(state.close().is_zero());
}

fn exercise_real_dx12_commit(
    state: &mut crate::native::UiNativeHostState,
    rasterizer: &mut MixedSubmittingRasterizer,
    port: &mut super::text_atlas_dx12_upload_port::QualifiedDx12UploadPort,
) {
    let alpha = demand_for(key_for(51), 51);
    let color_key = key_for_source(52, UiGlyphRasterSource::ColorOutline);
    let color = demand_for(color_key, 52);
    let demands = [alpha, color];
    let additions = [
        UiGlyphRasterPinRequest::from_text_mechanics(alpha.layout_identity(), key_for(51)),
        UiGlyphRasterPinRequest::from_text_mechanics(color.layout_identity(), color_key),
    ];
    let pins = UiGlyphRasterPinTransitionView::from_text_mechanics(&additions, &[]);
    let outcome = perform_with_upload_port(
        state,
        presentation_basis(),
        &demands,
        pins,
        rasterizer,
        port,
    );
    let receipt = match outcome {
        UiGlyphRasterTransactionOutcome::Committed(receipt) => receipt,
        UiGlyphRasterTransactionOutcome::Pending(pending) => {
            settle_qualified_pending(state, pending)
        }
        other => panic!("the qualified DX12 Signal transaction must commit: {other:?}"),
    };
    crate::native::text_atlas::assert_independent_committed_transaction(
        &demands,
        pins,
        receipt,
        state.text_atlas.snapshot(),
    );
    let signal = state.physical_signal.observation();
    assert_eq!(signal.counters.admissions, 1);
    assert_eq!(signal.counters.completed_observations, 1);
    assert_eq!(signal.active_requests, 0);
    assert!(signal.signal_performed_transitions > 0);
    assert!(signal.signal_performed_nodes > 0);
    let census = state.text_atlas.census();
    assert_eq!(census.alpha_entries, 1);
    assert_eq!(census.color_entries, 1);
    assert_eq!(census.pins, 2);
}

fn exercise_temporal_recovery(
    state: &mut crate::native::UiNativeHostState,
    rasterizer: &mut MixedSubmittingRasterizer,
    pending_port: &mut super::text_atlas_dx12_upload_port::QualifiedDx12UploadPort,
) {
    let retry_demand = demand_for(key_for(53), 53);
    let pending = perform_with_upload_port(
        state,
        presentation_basis(),
        &[retry_demand],
        UiGlyphRasterPinTransitionView::from_text_mechanics(&[], &[]),
        rasterizer,
        pending_port,
    );
    let UiGlyphRasterTransactionOutcome::Pending(retry_pending) = pending else {
        panic!("the injected pending port must retain exact physical work");
    };
    let before_retry = state.physical_signal.observation().counters;
    for _ in 0..4 {
        let due = state
            .physical_signal
            .next_due_tick()
            .expect("pending native work retains its temporal Signal frontier");
        state.physical_signal.advance_clock_to(due).unwrap();
        if state.physical_signal.observation().counters.retry_schedules
            > before_retry.retry_schedules
        {
            break;
        }
    }
    let after_retry = state.physical_signal.observation();
    assert!(after_retry.counters.timeout_observations > before_retry.timeout_observations);
    assert!(after_retry.counters.retry_schedules > before_retry.retry_schedules);
    let retry_due = state
        .physical_signal
        .next_due_tick()
        .expect("the scheduled native retry retains its exact temporal wake");
    state.physical_signal.advance_clock_to(retry_due).unwrap();
    let cancellation = state.cancel_pending_text_atlas(retry_pending);
    assert!(
        matches!(
            cancellation,
            UiGlyphRasterTransactionOutcome::EffectsIndeterminate(_)
        ),
        "the retained retry must transition to recovery: {cancellation:?}"
    );
    pending_port.complete();
    assert_eq!(
        state.progress_text_atlas_physical(retry_pending),
        crate::native::host_state::UiNativeTextAtlasPhysicalProgress::Terminal
    );
    let terminal = state.physical_signal.observation();
    assert_eq!(terminal.active_requests, 0);
    assert!(terminal.counters.cancellations > 0);
    assert!(terminal.counters.recovery_schedules > 0);
}

fn settle_qualified_pending(
    state: &mut crate::native::UiNativeHostState,
    pending: UiGlyphRasterTransactionPending,
) -> UiGlyphRasterTransactionReceipt {
    for _ in 0..4 {
        let due = state
            .physical_signal
            .next_due_tick()
            .expect("an unresolved DX12 submission retains a Signal wake");
        state.physical_signal.advance_clock_to(due).unwrap();
        assert_ne!(
            state.progress_text_atlas_physical(pending),
            crate::native::host_state::UiNativeTextAtlasPhysicalProgress::NoProgress
        );
        match state.complete_pending_text_atlas(pending) {
            UiGlyphRasterTransactionOutcome::Committed(receipt) => return receipt,
            UiGlyphRasterTransactionOutcome::Pending(_) => {}
            other => panic!("the qualified DX12 submission must settle: {other:?}"),
        }
    }
    panic!("the qualified DX12 submission exceeded its Signal retry policy")
}

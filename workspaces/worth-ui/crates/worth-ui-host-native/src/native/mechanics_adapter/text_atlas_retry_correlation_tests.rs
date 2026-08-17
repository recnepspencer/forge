use super::*;

#[test]
pub(super) fn production_retry_rebinds_the_retained_gpu_correlation() {
    let demand = hostile_upload_demand();
    let mut state = crate::native::UiNativeHostState::new();
    let mut rasterizer = signal_failure_tests::SubmittingRasterizer;
    let mut pending_port = signal_failure_tests::PendingUploadPort;
    let outcome = perform_with_upload_port(
        &mut state,
        presentation_basis(),
        &[demand],
        UiGlyphRasterPinTransitionView::from_text_mechanics(&[], &[]),
        &mut rasterizer,
        &mut pending_port,
    );
    let UiGlyphRasterTransactionOutcome::Pending(pending) = outcome else {
        panic!("the retry proof requires one retained atlas transaction: {outcome:?}");
    };
    let first_basis = state
        .text_atlas_in_flight
        .as_ref()
        .expect("the pending transaction owns its Signal attempt")
        .signal_token()
        .external_basis();

    let (device, queue, _) = crate::native::text_atlas::qualified_test_device();
    let mut gpu = crate::native::text_atlas::UiNativeTextAtlasGpuPages::new();
    gpu.ensure_page(
        &device,
        &mut state.resources,
        crate::native::text_atlas::UiNativeGpuAtlasKind::Alpha,
    )
    .unwrap();
    let key = demand.records()[0].key();
    let upload = crate::native::text_atlas::UiNativeTextAtlasUpload::from_text_mechanics(
        key,
        4,
        4,
        4,
        vec![0; 16],
        [0; 32],
    );
    gpu.upload_for_transaction(
        crate::native::text_atlas::UiNativeTextAtlasGpuUploadRequest {
            device: &device,
            queue: &queue,
            resources: &mut state.resources,
            kind: crate::native::text_atlas::UiNativeGpuAtlasKind::Alpha,
            page: 0,
            origin: [0, 0],
            upload: &upload,
        },
        pending.transaction(),
    )
    .unwrap();
    gpu.bind_transaction_correlation(pending.transaction(), first_basis)
        .unwrap();
    state.text_atlas_gpu = Some(gpu);

    let before_retry = state.physical_signal.observation().counters.retry_schedules;
    for _ in 0..4 {
        let due = state
            .physical_signal
            .next_due_tick()
            .expect("pending physical work retains a temporal wake");
        state.physical_signal.advance_clock_to(due).unwrap();
        if state.physical_signal.observation().counters.retry_schedules > before_retry {
            break;
        }
    }
    let retry_due = state
        .physical_signal
        .next_due_tick()
        .expect("the scheduled retry owns its successor wake");
    state.physical_signal.advance_clock_to(retry_due).unwrap();
    assert_eq!(
        state
            .text_atlas_gpu
            .as_ref()
            .expect("the retry retains its physical submission")
            .pending_count(),
        1
    );
    assert_eq!(state.resources.current().atlas_staging_buffers, 1);
    device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(crate::native::presentation::GPU_WAIT_DEADLINE),
        })
        .expect("the qualified retry submission must physically complete");
    assert!(state.progress_text_atlas_physical(pending));
    assert!(matches!(
        state.complete_pending_text_atlas(pending),
        UiGlyphRasterTransactionOutcome::Committed(_)
    ));
    assert_eq!(state.resources.current().atlas_staging_buffers, 0);

    let gpu = state.text_atlas_gpu.take().unwrap();
    assert_eq!(gpu.pending_count(), 0);
    gpu.try_close(&mut state.resources)
        .unwrap_or_else(|_| panic!("the settled correlation must release exactly"));
    assert!(state.close().is_zero());
}

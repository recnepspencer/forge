use super::*;

pub(super) struct SubmittingRasterizer;

impl UiGlyphRasterMissRasterizer for SubmittingRasterizer {
    fn rasterize(
        &mut self,
        misses: UiGlyphRasterMissSelectionView<'_>,
        sink: &mut dyn UiGlyphRasterBatchSink,
    ) -> Result<(), UiGlyphRasterCallbackDenial> {
        let demanded = misses.records();
        let pixels = demanded
            .iter()
            .map(|record| {
                vec![
                    0_u8;
                    usize::try_from(record.extent().width() * record.extent().height()).unwrap()
                ]
            })
            .collect::<Vec<_>>();
        let records = demanded
            .iter()
            .zip(&pixels)
            .map(|(record, pixels)| {
                let digest: [u8; 32] = Sha256::digest(pixels).into();
                UiAlphaRasterRecordView::from_text_mechanics(UiGlyphRasterRecordViewInput {
                    key: record.key(),
                    attribution: record.attribution(),
                    bearing: UiGlyphRasterBearing::from_sixty_fourths(0, 0),
                    extent: record.extent(),
                    stride: record.extent().width(),
                    pixels,
                    digest: UiGlyphRasterContentDigest::from_text_mechanics(digest),
                })
                .map_err(|_| UiGlyphRasterCallbackDenial::RasterizationDenied)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let (miss, batch) = super::super::text_atlas_upload_sink::expected_alpha_batch_identity(
            misses.demand_identity(),
            misses.layout_identity(),
            1_000,
            1,
            misses.lane(),
            &records,
        );
        sink.submit_alpha(UiAlphaRasterBatchView::from_text_mechanics(
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

#[derive(Clone, Copy)]
enum UploadDisposition {
    FailBeforeFirst,
    FailAfterFirst,
}

struct HostileUploadPort {
    calls: Cell<u32>,
    disposition: UploadDisposition,
}

pub(super) struct PendingUploadPort;

impl UiNativeTextAtlasUploadPort for PendingUploadPort {
    fn upload(
        &mut self,
        _state: &mut crate::native::UiNativeHostState,
        _plan: &crate::native::text_atlas::UiNativeTextAtlasTransactionPlan,
        _uploads: &[crate::native::text_atlas::UiNativeTextAtlasUpload],
        basis: crate::native::physical_work_signal::UiNativePhysicalSignalExternalBasis,
    ) -> super::super::text_atlas_upload::CorrelatedGpuUploadObservation {
        super::super::text_atlas_upload::CorrelatedGpuUploadObservation {
            external: Ok(crate::native::text_atlas::UiNativeTextAtlasExternalOutcome::Submitted),
            signal: basis.observe(
                crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Pending,
            ),
        }
    }
}

#[derive(Default)]
struct CompletingUploadPort {
    calls: u32,
    previous:
        Option<crate::native::physical_work_signal::UiNativePhysicalSignalExternalObservation>,
    replay_previous: bool,
}

impl UiNativeTextAtlasUploadPort for CompletingUploadPort {
    fn upload(
        &mut self,
        _state: &mut crate::native::UiNativeHostState,
        _plan: &crate::native::text_atlas::UiNativeTextAtlasTransactionPlan,
        _uploads: &[crate::native::text_atlas::UiNativeTextAtlasUpload],
        basis: crate::native::physical_work_signal::UiNativePhysicalSignalExternalBasis,
    ) -> super::super::text_atlas_upload::CorrelatedGpuUploadObservation {
        self.calls = self.calls.saturating_add(1);
        let current = basis
            .observe(crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Completed);
        let signal = if self.replay_previous {
            self.previous
                .expect("the hostile replay needs a prior completion")
        } else {
            self.previous = Some(current);
            current
        };
        super::super::text_atlas_upload::CorrelatedGpuUploadObservation {
            external: Ok(crate::native::text_atlas::UiNativeTextAtlasExternalOutcome::Submitted),
            signal,
        }
    }
}

impl UiNativeTextAtlasUploadPort for HostileUploadPort {
    fn upload(
        &mut self,
        _state: &mut crate::native::UiNativeHostState,
        _plan: &crate::native::text_atlas::UiNativeTextAtlasTransactionPlan,
        uploads: &[crate::native::text_atlas::UiNativeTextAtlasUpload],
        basis: crate::native::physical_work_signal::UiNativePhysicalSignalExternalBasis,
    ) -> super::super::text_atlas_upload::CorrelatedGpuUploadObservation {
        self.calls.set(self.calls.get() + 1);
        let submitted = !uploads.is_empty();
        match self.disposition {
            UploadDisposition::FailBeforeFirst => {
                super::super::text_atlas_upload::correlated_failure(
                    basis,
                    super::super::text_atlas_upload::upload_failure(
                        false,
                        crate::native::text_atlas::UiNativeTextAtlasDenial::UploadRejected,
                    ),
                )
            }
            UploadDisposition::FailAfterFirst => {
                super::super::text_atlas_upload::correlated_failure(
                    basis,
                    super::super::text_atlas_upload::upload_failure(
                        submitted,
                        crate::native::text_atlas::UiNativeTextAtlasDenial::UploadRejected,
                    ),
                )
            }
        }
    }
}

#[test]
pub(super) fn named_upload_port_faults_are_causal_and_preserve_atlas_rollback() {
    let demand = hostile_upload_demand();
    let pins = UiGlyphRasterPinTransitionView::from_text_mechanics(&[], &[]);
    let mut state = crate::native::UiNativeHostState::new();
    let mut rasterizer = SubmittingRasterizer;
    let mut before = HostileUploadPort {
        calls: Cell::new(0),
        disposition: UploadDisposition::FailBeforeFirst,
    };
    let rejected = super::super::perform_with_upload_port(
        &mut state,
        crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::test(),
        &[demand],
        pins,
        &mut rasterizer,
        &mut before,
    );
    assert_eq!(before.calls.get(), 1);
    assert_eq!(
        rejected,
        UiGlyphRasterTransactionOutcome::RejectedAfterRasterization(
            UiGlyphRasterTransactionDenial::RasterBatchMismatch
        )
    );
    assert!(state.text_atlas.census().is_zero());

    let mut indeterminate = HostileUploadPort {
        calls: Cell::new(0),
        disposition: UploadDisposition::FailAfterFirst,
    };
    let outcome = super::super::perform_with_upload_port(
        &mut state,
        crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::test(),
        &[demand],
        pins,
        &mut rasterizer,
        &mut indeterminate,
    );
    assert_eq!(indeterminate.calls.get(), 1);
    assert!(matches!(
        outcome,
        UiGlyphRasterTransactionOutcome::EffectsIndeterminate(_)
    ));
    assert!(matches!(
        state.text_atlas.plan_many(
            &[demand],
            &super::super::text_atlas_admission::native_pin_transition(pins),
        ),
        Err(crate::native::text_atlas::UiNativeTextAtlasDenial::ReconstructionRequired)
    ));
    let signal = state.physical_signal.observation();
    assert_eq!(signal.active_requests, 1);
    assert_eq!(signal.counters.indeterminate_observations, 1);
    assert_eq!(signal.counters.recovery_schedules, 1);
    assert!(state
        .text_atlas_in_flight
        .as_ref()
        .is_some_and(crate::native::text_atlas::UiNativeTextAtlasInFlight::awaits_recovery));

    let pending = state.text_atlas_in_flight.as_ref().unwrap().pending();
    state.progress_text_atlas_physical(pending);
    assert_eq!(state.physical_signal.observation().active_requests, 0);
    assert!(state.text_atlas_in_flight.is_none());
    assert!(state.text_atlas_recovery.is_none());
    assert!(state.text_atlas.census().is_zero());
}

#[test]
pub(super) fn cancellation_and_supersession_atomically_quarantine_retained_native_uploads() {
    for supersede in [false, true] {
        let demand = hostile_upload_demand();
        let pins = UiGlyphRasterPinTransitionView::from_text_mechanics(&[], &[]);
        let mut state = crate::native::UiNativeHostState::new();
        let mut rasterizer = SubmittingRasterizer;
        let mut upload = PendingUploadPort;
        let pending = super::super::perform_with_upload_port(
            &mut state,
            crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::test(),
            &[demand],
            pins,
            &mut rasterizer,
            &mut upload,
        );
        let UiGlyphRasterTransactionOutcome::Pending(pending) = pending else {
            panic!("pending external work must retain the exact native obligation")
        };
        assert!(state
            .text_atlas_in_flight
            .as_ref()
            .is_some_and(|work| !work.awaits_recovery()));

        let outcome = if supersede {
            state.supersede_pending_text_atlas(pending)
        } else {
            state.cancel_pending_text_atlas(pending)
        };
        assert!(matches!(
            outcome,
            UiGlyphRasterTransactionOutcome::EffectsIndeterminate(_)
        ));
        assert!(state
            .text_atlas_in_flight
            .as_ref()
            .is_some_and(crate::native::text_atlas::UiNativeTextAtlasInFlight::awaits_recovery));
        assert!(state.text_atlas_recovery.is_some());
        assert_eq!(state.physical_signal.observation().active_requests, 1);
        let counters = state.physical_signal.observation().counters;
        assert_eq!(counters.supersessions, u64::from(supersede));
        assert_eq!(counters.cancellations, 1);

        assert!(state.progress_text_atlas_physical(pending));
        assert!(state.text_atlas_in_flight.is_none());
        assert!(state.text_atlas_recovery.is_none());
        assert_eq!(state.physical_signal.observation().active_requests, 0);
        assert!(state.close().is_zero());
    }
}

#[test]
fn lawful_signal_transaction_commits_and_releases_one_live_layout_pin() {
    let demand = hostile_upload_demand();
    let pin = worth_ui_host_contract::UiGlyphRasterPinRequest::from_text_mechanics(
        demand.layout_identity(),
        demand.records()[0].key(),
    );
    let additions = [pin];
    let releases = [pin];
    let mut state = crate::native::UiNativeHostState::new();
    let mut rasterizer = SubmittingRasterizer;
    let mut upload = CompletingUploadPort::default();

    let committed = super::super::perform_with_upload_port(
        &mut state,
        crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::test(),
        &[demand],
        UiGlyphRasterPinTransitionView::from_text_mechanics(&additions, &[]),
        &mut rasterizer,
        &mut upload,
    );
    assert!(matches!(
        committed,
        UiGlyphRasterTransactionOutcome::Committed(_)
    ));
    let committed_census = state.text_atlas.census();
    assert_eq!(committed_census.alpha_entries, 1);
    assert_eq!(committed_census.pins, 1);
    assert_eq!(upload.calls, 1);
    let signal = state.physical_signal.observation();
    assert_eq!(signal.active_requests, 0);
    assert_eq!(signal.counters.admissions, 1);
    assert_eq!(signal.counters.completed_observations, 1);

    let released = super::super::perform_with_upload_port(
        &mut state,
        crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::test(),
        &[],
        UiGlyphRasterPinTransitionView::from_text_mechanics(&[], &releases),
        &mut rasterizer,
        &mut upload,
    );
    assert!(matches!(
        released,
        UiGlyphRasterTransactionOutcome::Committed(_)
    ));
    assert_eq!(state.text_atlas.census().pins, 0);
    assert_eq!(upload.calls, 1, "pin-only work performs no upload");
    assert!(state.close().is_zero());
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P5-ATLAS-01\":1}}");
}

#[test]
pub(super) fn replayed_external_completion_cannot_settle_a_new_atlas_request() {
    let first = demand_for(key_for(5), 9);
    let second = demand_for(key_for(6), 10);
    let pins = UiGlyphRasterPinTransitionView::from_text_mechanics(&[], &[]);
    let mut state = crate::native::UiNativeHostState::new();
    let mut rasterizer = SubmittingRasterizer;
    let mut upload = CompletingUploadPort::default();

    let committed = super::super::perform_with_upload_port(
        &mut state,
        crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::test(),
        &[first],
        pins,
        &mut rasterizer,
        &mut upload,
    );
    assert!(matches!(
        committed,
        UiGlyphRasterTransactionOutcome::Committed(_)
    ));
    assert_eq!(state.text_atlas.census().alpha_entries, 1);

    upload.replay_previous = true;
    let stale = super::super::perform_with_upload_port(
        &mut state,
        crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::test(),
        &[second],
        pins,
        &mut rasterizer,
        &mut upload,
    );
    assert!(matches!(
        stale,
        UiGlyphRasterTransactionOutcome::EffectsIndeterminate(_)
    ));
    let signal = state.physical_signal.observation();
    assert_eq!(signal.counters.stale_observations, 1);
    assert_eq!(signal.counters.recovery_schedules, 1);
    assert_eq!(signal.active_requests, 1);
    assert!(state
        .text_atlas_in_flight
        .as_ref()
        .is_some_and(crate::native::text_atlas::UiNativeTextAtlasInFlight::awaits_recovery));

    let pending = state.text_atlas_in_flight.as_ref().unwrap().pending();
    state.progress_text_atlas_physical(pending);
    assert_eq!(state.physical_signal.observation().active_requests, 0);
    assert!(state.text_atlas.census().is_zero());
}

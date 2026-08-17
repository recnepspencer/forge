use std::collections::{BTreeMap, BTreeSet};

use worth_ui_host_contract::UiHostSurfaceRegistrationRequest;

use super::physical_work_signal::UiNativePhysicalSignalOwner;
use super::text_atlas::{UiNativeTextAtlas, UiNativeTextAtlasGpuPages, UiNativeTextAtlasInFlight};
use super::{
    event_loop::UiNativeOwnedWindow, UiNativeOwnedGraphics, UiNativePendingPresentation,
    UiNativePresentationObservation, UiNativeResourceCensus, UiNativeResourceOwner,
    UiNativeResourceRegistry, UiNativeRetainedDrawList, UiNativeRetainedFrameObservation,
};

mod presentation_lifecycle;
#[cfg(test)]
#[path = "host_state/temporal_retry_tests.rs"]
mod temporal_retry_tests;
mod text_atlas_lifecycle;

pub(crate) struct UiNativeHostState {
    pub(crate) registrations: BTreeMap<u64, UiHostSurfaceRegistrationRequest>,
    pub(crate) registration_resources: BTreeMap<u64, UiNativeResourceOwner>,
    pub(crate) window: Option<UiNativeOwnedWindow>,
    pub(crate) graphics: Option<UiNativeOwnedGraphics>,
    pub(crate) last_presentation: Option<UiNativePresentationObservation>,
    pub(crate) retained_frame_observations: Vec<UiNativeRetainedFrameObservation>,
    pub(crate) resources: UiNativeResourceRegistry,
    pub(crate) effect_posture: UiNativeEffectPosture,
    pub(crate) pending_presentations: Vec<UiNativePendingPresentation>,
    pub(crate) retained_draw_lists: BTreeMap<u64, UiNativeRetainedDrawList>,
    pub(crate) presentation_epochs: BTreeMap<u64, worth_ui_host_contract::UiHostPresentationEpoch>,
    pub(crate) reconstruction_required: BTreeSet<u64>,
    pub(crate) text_atlas: UiNativeTextAtlas,
    pub(crate) text_atlas_gpu: Option<UiNativeTextAtlasGpuPages>,
    pub(crate) text_atlas_in_flight: Option<UiNativeTextAtlasInFlight>,
    pub(crate) text_atlas_recovery: Option<super::text_atlas::UiNativeTextAtlasRecovery>,
    pub(crate) text_atlas_completion: Option<(
        worth_ui_host_contract::UiGlyphRasterTransactionPending,
        worth_ui_host_contract::UiGlyphRasterTransactionOutcome,
    )>,
    pub(crate) text_pins_by_binding:
        BTreeMap<u64, Box<[worth_ui_host_contract::UiGlyphRasterPinRequest]>>,
    pub(crate) pending_text_presentations: BTreeMap<u64, UiNativePendingTextPresentation>,
    pub(crate) physical_signal: UiNativePhysicalSignalOwner,
    pub(crate) peak_census: UiNativeResourceCensus,
    pub(crate) peak_text_pins: Box<[super::text_atlas::UiNativeTextPinObservation]>,
}

pub(crate) struct UiNativePendingTextPresentation {
    pub(crate) atlas: worth_ui_host_contract::UiGlyphRasterTransactionPending,
    pub(crate) completion: worth_ui_host_contract::UiMountedSurfacePresentationCompletion,
    pub(crate) binding: u64,
    pub(crate) binding_pins: Box<[worth_ui_host_contract::UiGlyphRasterPinRequest]>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum UiNativeEffectPosture {
    #[default]
    BeforeEffects,
    PresentationIndeterminate,
    Presented,
}

impl UiNativeHostState {
    pub(crate) fn new() -> Self {
        let mut state = Self {
            registrations: BTreeMap::new(),
            registration_resources: BTreeMap::new(),
            window: None,
            graphics: None,
            last_presentation: None,
            retained_frame_observations: Vec::new(),
            resources: UiNativeResourceRegistry::new(),
            effect_posture: UiNativeEffectPosture::BeforeEffects,
            pending_presentations: Vec::new(),
            retained_draw_lists: BTreeMap::new(),
            presentation_epochs: BTreeMap::new(),
            reconstruction_required: BTreeSet::new(),
            text_atlas: UiNativeTextAtlas::new(),
            text_atlas_gpu: None,
            text_atlas_in_flight: None,
            text_atlas_recovery: None,
            text_atlas_completion: None,
            text_pins_by_binding: BTreeMap::new(),
            pending_text_presentations: BTreeMap::new(),
            physical_signal: UiNativePhysicalSignalOwner::new(),
            peak_census: UiNativeResourceCensus::default(),
            peak_text_pins: Box::new([]),
        };
        state.record_compiler_total_peak();
        state
    }

    pub(crate) fn record_compiler_total_peak(&mut self) {
        let current = self
            .resources
            .current()
            .with_text_atlas(self.text_atlas_census())
            .with_physical_signal(self.physical_signal.observation());
        self.peak_census = self.peak_census.max(self.resources.peak()).max(current);
        let pins = self.text_atlas.pin_observations();
        if pins.len() > self.peak_text_pins.len() {
            self.peak_text_pins = pins;
        }
    }

    pub(crate) fn compiler_total_peak(&self) -> UiNativeResourceCensus {
        self.peak_census.max(self.resources.peak())
    }

    pub(crate) fn close(&mut self) -> UiNativeResourceCensus {
        self.last_presentation = None;
        let _ = self.physical_signal.shutdown();
        if self.pending_presentations.is_empty() {
            if self
                .text_atlas_gpu
                .as_ref()
                .is_some_and(|gpu| gpu.pending_count() != 0)
                || self.text_atlas_in_flight.is_some()
                || self.text_atlas_recovery.is_some()
            {
                return self
                    .resources
                    .current()
                    .with_text_atlas(self.text_atlas_census())
                    .with_physical_signal(self.physical_signal.observation());
            }
            self.retained_draw_lists.clear();
            self.presentation_epochs.clear();
            self.reconstruction_required.clear();
            let cleared = self.text_atlas.clear();
            self.text_atlas_recovery = None;
            self.text_atlas_completion = None;
            self.text_pins_by_binding.clear();
            self.pending_text_presentations.clear();
            if let Some(atlas_gpu) = self.text_atlas_gpu.take() {
                atlas_gpu
                    .try_close(&mut self.resources)
                    .unwrap_or_else(|_| panic!("settled atlas pages must close"));
            }
            if let Some(graphics) = self.graphics.take() {
                graphics.close(&mut self.resources);
            }
            if let Some(window) = self.window.take() {
                window.close(&mut self.resources);
            }
            let _ = self.physical_signal.shutdown();
            debug_assert!(cleared && self.text_atlas_census().is_zero());
        }
        self.resources
            .current()
            .with_text_atlas(self.text_atlas_census())
            .with_physical_signal(self.physical_signal.observation())
    }

    pub(crate) fn progress_one_physical_signal_ready(&mut self) -> bool {
        match self.physical_signal.next_ready_work() {
            Some(super::physical_work_signal::UiNativePhysicalSignalWork::AtlasUpload(
                identity,
            )) => self.progress_text_atlas_physical(identity.pending()),
            Some(super::physical_work_signal::UiNativePhysicalSignalWork::Presentation(
                identity,
            )) => self.progress_pending_presentation(identity),
            Some(super::physical_work_signal::UiNativePhysicalSignalWork::AtlasPlanning(_))
            | None => false,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::UiNativeHostState;
    use crate::native::presentation::{
        reserve_presentation_owners, settle_port_result, UiNativePendingExternalObligation,
        UiNativePresentationFailure, UiNativePresentationPortFailure,
    };
    use std::cell::Cell;
    use std::rc::Rc;
    use worth_ui_host_contract::{
        UiFontCollectionGeneration, UiFontCollectionLineageIdentity, UiGlyphRasterFractionalOrigin,
        UiGlyphRasterKey, UiGlyphRasterKeyInput, UiGlyphRasterPalette, UiGlyphRasterSize,
        UiGlyphRasterSource, UiGlyphVariationCoordinates, UiQualifiedFontFaceIdentity,
        UiTextProfileGeneration,
    };

    struct PendingProbe {
        dropped: Rc<Cell<bool>>,
        settles: Rc<Cell<bool>>,
    }

    impl UiNativePendingExternalObligation for PendingProbe {
        fn poll_observation(
            &mut self,
            basis: crate::native::physical_work_signal::UiNativePhysicalSignalExternalBasis,
            _device: Option<&wgpu::Device>,
        ) -> crate::native::physical_work_signal::UiNativePhysicalSignalExternalObservation
        {
            basis.observe(if self.settles.get() {
                crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Completed
            } else {
                crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Pending
            })
        }
    }

    impl Drop for PendingProbe {
        fn drop(&mut self) {
            self.dropped.set(true);
        }
    }

    #[test]
    pub(crate) fn pending_external_work_retains_cleanup_authority_until_it_settles() {
        let mut state = UiNativeHostState::new();
        let dropped = Rc::new(Cell::new(false));
        let settles = Rc::new(Cell::new(false));
        let owners = reserve_presentation_owners(
            &mut state.resources,
            &mut state.physical_signal,
            crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::test(),
        )
        .unwrap_or_else(|_| panic!("empty registry must reserve presentation owners"));
        let pending = settle_port_result(
            &mut state.resources,
            &mut state.physical_signal,
            owners,
            Err(UiNativePresentationPortFailure::ReadbackUnsettled(
                Box::new(PendingProbe {
                    dropped: Rc::clone(&dropped),
                    settles: Rc::clone(&settles),
                }),
            )),
        );
        let Err(UiNativePresentationFailure::Indeterminate(pending)) = pending else {
            panic!("unsettled external work must enter pending host state");
        };
        state.pending_presentations.push(pending);

        let pending = state.close();
        assert_eq!(pending.readback_buffers, 1);
        assert_eq!(pending.pending_submissions, 1);
        assert!(!dropped.get());
        let physical = state.physical_signal.observation();
        assert_eq!(physical.counters.admissions, 1);
        assert_eq!(physical.counters.pending_observations, 1);
        assert_eq!(physical.active_requests, 1);

        settles.set(true);
        let due = state
            .physical_signal
            .next_due_tick()
            .expect("pending presentation must retain one Signal-owned poll wake");
        state
            .physical_signal
            .advance_clock_to(due)
            .expect("the exact pending poll wake must become ready");
        assert!(state.progress_one_physical_signal_ready());
        assert!(state.close().is_zero());
        assert!(dropped.get());
        let physical = state.physical_signal.observation();
        assert_eq!(physical.counters.completed_observations, 1);
        assert_eq!(physical.active_requests, 0);
        assert!(!physical.accepting_admissions);
        assert!(!physical.runtime_owned);
    }

    #[test]
    fn host_state_exposes_one_terminal_text_atlas_census() {
        let mut state = UiNativeHostState::new();
        assert!(state.text_atlas_census().is_zero());
        assert!(state.close().is_zero());
    }

    #[test]
    fn host_close_retains_pending_atlas_upload_until_external_settlement() {
        let (device, queue, _) = crate::native::text_atlas::qualified_test_device();
        let mut state = UiNativeHostState::new();
        let mut gpu = crate::native::text_atlas::UiNativeTextAtlasGpuPages::new();
        gpu.ensure_page(
            &device,
            &mut state.resources,
            crate::native::text_atlas::UiNativeGpuAtlasKind::Alpha,
        )
        .unwrap();
        let key = UiGlyphRasterKey::from_text_mechanics(UiGlyphRasterKeyInput {
            font_collection: UiFontCollectionGeneration::new(1).unwrap(),
            font_collection_lineage: UiFontCollectionLineageIdentity::from_text_mechanics([3; 32]),
            profile: UiTextProfileGeneration::new(1).unwrap(),
            face: UiQualifiedFontFaceIdentity::from_text_mechanics([4; 32], 0),
            glyph_id: 1,
            variations: UiGlyphVariationCoordinates::empty(),
            palette: UiGlyphRasterPalette::new(0),
            size: UiGlyphRasterSize::from_millipoints(12_000).unwrap(),
            source: UiGlyphRasterSource::AlphaOutline,
            dpi_milli: 1_000,
            origin: UiGlyphRasterFractionalOrigin::from_sixty_fourths(0, 0),
        })
        .unwrap();
        let upload = crate::native::text_atlas::UiNativeTextAtlasUpload::from_text_mechanics(
            key,
            2,
            2,
            2,
            vec![0; 4],
            [0; 32],
        );
        gpu.upload(
            crate::native::text_atlas::UiNativeTextAtlasGpuUploadRequest {
                device: &device,
                queue: &queue,
                resources: &mut state.resources,
                kind: crate::native::text_atlas::UiNativeGpuAtlasKind::Alpha,
                page: 0,
                origin: [0, 0],
                upload: &upload,
            },
        )
        .unwrap();
        state.text_atlas_gpu = Some(gpu);

        let incomplete = state.close();
        assert_eq!(incomplete.atlas_staging_buffers, 1);
        assert_eq!(incomplete.text_atlas_upload_submissions, 1);
        assert!(state.text_atlas_gpu.is_some());

        state
            .text_atlas_gpu
            .as_mut()
            .unwrap()
            .settle_pending(&device, &mut state.resources);
        assert!(state.close().is_zero());
        assert!(state.text_atlas_gpu.is_none());
    }
}

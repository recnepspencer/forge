use super::source::{UiNativeCaptureSource, UiNativeCaptureSourceInput};
use super::UiNativeCaptureState;
use std::cell::Cell;
use std::rc::Rc;

struct CaptureFixture {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    host_surface: worth_ui_host_contract::UiHostSurfaceIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    epoch: worth_ui_host_contract::UiHostPresentationEpoch,
    transform: worth_ui_host_contract::UiHostCoordinateTransform,
}

#[test]
fn admitted_capture_cancels_before_readback_and_releases_its_slot() {
    let fixture = fixture();
    let mut captures = capture_state(&fixture);
    let mut resources = crate::native::UiNativeResourceRegistry::new();
    let admitted = request(&fixture, 1, true);
    assert!(matches!(
        captures.observe(None, &mut resources, admitted),
        worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
    ));
    assert_eq!(
        captures.cancel(None, &mut resources, admitted),
        worth_ui_host_contract::UiHostCaptureCancellationOutcome::CancelledBeforeReadback
    );
    assert!(resources.current().is_zero());
    assert!(matches!(
        captures.observe(None, &mut resources, request(&fixture, 2, true)),
        worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
    ));
}

#[test]
fn capture_slots_are_bounded_without_starting_gpu_effects() {
    let fixture = fixture();
    let mut captures = capture_state(&fixture);
    let mut resources = crate::native::UiNativeResourceRegistry::new();
    let requests = (1..=4)
        .map(|identity| request(&fixture, identity, true))
        .collect::<Vec<_>>();
    for request in &requests {
        assert!(matches!(
            captures.observe(None, &mut resources, *request),
            worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
        ));
    }
    assert!(matches!(
        captures.observe(None, &mut resources, request(&fixture, 5, true)),
        worth_ui_host_contract::UiHostCaptureObservationOutcome::CapacityExceeded
    ));
    for request in requests {
        assert_eq!(
            captures.cancel(None, &mut resources, request),
            worth_ui_host_contract::UiHostCaptureCancellationOutcome::CancelledBeforeReadback
        );
    }
}

#[test]
fn submitted_cancellations_remain_charged_until_external_settlement() {
    let fixture = fixture();
    let settlement = Rc::new(Cell::new(false));
    let mut captures = capture_state_with_port(
        &fixture,
        Box::new(GatedCapturePort {
            settlement: Rc::clone(&settlement),
        }),
    );
    let mut resources = crate::native::UiNativeResourceRegistry::new();
    for identity in 1..=4 {
        let request = request(&fixture, identity, true);
        assert!(matches!(
            captures.observe(None, &mut resources, request),
            worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
        ));
        assert!(matches!(
            captures.observe(None, &mut resources, request),
            worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
        ));
        assert_eq!(
            captures.cancel(None, &mut resources, request),
            worth_ui_host_contract::UiHostCaptureCancellationOutcome::ReadbackMayHaveBegun
        );
    }
    assert_eq!(resources.current().readback_buffers, 4);
    assert_eq!(resources.current().pending_submissions, 4);
    assert_eq!(captures.occupied_slots(), 4);
    assert_eq!(captures.reserved_bytes, 4 * 256);
    assert!(matches!(
        captures.observe(None, &mut resources, request(&fixture, 5, true)),
        worth_ui_host_contract::UiHostCaptureObservationOutcome::CapacityExceeded
    ));

    settlement.set(true);
    assert!(matches!(
        captures.observe(None, &mut resources, request(&fixture, 5, true)),
        worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
    ));
    assert_eq!(captures.occupied_slots(), 1);
    assert_eq!(captures.reserved_bytes, 256);
    assert!(resources.current().is_zero());
}

#[test]
fn native_close_retries_retain_physically_unsettled_capture_owners() {
    let fixture = fixture();
    let settlement = Rc::new(Cell::new(false));
    let mut state = crate::native::UiNativeHostState::new();
    state.captures = capture_state_with_port(
        &fixture,
        Box::new(GatedCapturePort {
            settlement: Rc::clone(&settlement),
        }),
    );
    let request = request(&fixture, 1, true);
    assert!(matches!(
        state.captures.observe(None, &mut state.resources, request),
        worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
    ));
    assert!(matches!(
        state.captures.observe(None, &mut state.resources, request),
        worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
    ));

    let census = state.close();
    assert_eq!(census.readback_buffers, 1);
    assert_eq!(census.pending_submissions, 1);

    settlement.set(true);
    assert!(state.close().is_zero());
}

#[test]
fn aggregate_padded_bytes_bound_admission_before_the_slot_limit() {
    let fixture = fixture_with_dimensions([2_048, 768]);
    let mut captures = capture_state(&fixture);
    let mut resources = crate::native::UiNativeResourceRegistry::new();
    let budget = 2_048_u64 * 768 * 4;
    let first = request_with_budget(&fixture, 1, budget);
    let second = request_with_budget(&fixture, 2, budget);
    let third = request_with_budget(&fixture, 3, budget);
    for request in [first, second] {
        assert!(matches!(
            captures.observe(None, &mut resources, request),
            worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
        ));
    }
    assert_eq!(captures.occupied_slots(), 2);
    assert!(matches!(
        captures.observe(None, &mut resources, third),
        worth_ui_host_contract::UiHostCaptureObservationOutcome::CapacityExceeded
    ));
    assert_eq!(
        captures.cancel(None, &mut resources, first),
        worth_ui_host_contract::UiHostCaptureCancellationOutcome::CancelledBeforeReadback
    );
    assert!(matches!(
        captures.observe(None, &mut resources, third),
        worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
    ));
}

#[test]
fn source_invalidation_after_admission_prevents_a_stale_readback() {
    let fixture = fixture();
    let mut captures = capture_state(&fixture);
    let mut resources = crate::native::UiNativeResourceRegistry::new();
    let request = request(&fixture, 1, true);
    assert!(matches!(
        captures.observe(None, &mut resources, request),
        worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
    ));

    captures.invalidate_source(fixture.binding.diagnostic_value());

    assert!(matches!(
        captures.observe(None, &mut resources, request),
        worth_ui_host_contract::UiHostCaptureObservationOutcome::SupersededBeforeReadback
    ));
    assert!(resources.current().is_zero());
}

#[test]
fn source_invalidation_preserves_a_submitted_predecessor_readback() {
    let fixture = fixture();
    let settlement = Rc::new(Cell::new(true));
    let mut captures = capture_state_with_port(&fixture, Box::new(GatedCapturePort { settlement }));
    let mut resources = crate::native::UiNativeResourceRegistry::new();
    let request = request(&fixture, 1, true);
    assert!(matches!(
        captures.observe(None, &mut resources, request),
        worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
    ));
    assert!(matches!(
        captures.observe(None, &mut resources, request),
        worth_ui_host_contract::UiHostCaptureObservationOutcome::Pending
    ));

    captures.invalidate_source(fixture.binding.diagnostic_value());

    let worth_ui_host_contract::UiHostCaptureObservationOutcome::Captured(observation) =
        captures.observe(None, &mut resources, request)
    else {
        panic!("the submitted predecessor must settle against its retained source")
    };
    assert_eq!(observation.affinity().request(), request.identity());
    assert_eq!(observation.affinity().copy_epoch(), fixture.epoch);
    assert_eq!(observation.pixels().unwrap().bytes().len(), 8);
    assert!(resources.current().is_zero());
}

#[test]
fn geometry_capture_returns_exact_affinity_without_gpu_work() {
    let fixture = fixture();
    let mut captures = capture_state(&fixture);
    let mut resources = crate::native::UiNativeResourceRegistry::new();
    let request = request(&fixture, 1, false);
    let worth_ui_host_contract::UiHostCaptureObservationOutcome::Captured(observation) =
        captures.observe(None, &mut resources, request)
    else {
        panic!("geometry capture must complete before GPU effects");
    };
    assert_eq!(observation.affinity().request(), request.identity());
    assert_eq!(observation.affinity().copy_epoch(), fixture.epoch);
    assert_eq!(observation.transform(), fixture.transform);
    assert!(observation.pixels().is_none());
    assert!(resources.current().is_zero());
}

fn capture_state(fixture: &CaptureFixture) -> UiNativeCaptureState {
    let mut captures = UiNativeCaptureState::default();
    record_fixture_source(&mut captures, fixture);
    captures
}

fn capture_state_with_port(
    fixture: &CaptureFixture,
    port: Box<dyn super::port::UiNativeCaptureReadbackPort>,
) -> UiNativeCaptureState {
    let mut captures = UiNativeCaptureState::with_port(port);
    record_fixture_source(&mut captures, fixture);
    captures
}

fn record_fixture_source(captures: &mut UiNativeCaptureState, fixture: &CaptureFixture) {
    captures.record_source(
        fixture.binding,
        UiNativeCaptureSource::completed(UiNativeCaptureSourceInput {
            host_session: 7,
            frame: fixture.frame,
            attempt: fixture.attempt,
            host_surface: fixture.host_surface,
            binding: fixture.binding,
            epoch: fixture.epoch,
            transform: Some(fixture.transform),
            regions: Vec::new(),
        }),
    );
}

fn fixture() -> CaptureFixture {
    fixture_with_dimensions([2, 1])
}

fn fixture_with_dimensions(dimensions: [u32; 2]) -> CaptureFixture {
    let frame = worth_ui_host_contract::UiMountedFrameIdentity::mint_unbound().unwrap();
    let attempt =
        worth_ui_host_contract::UiMountedPresentationAttemptIdentity::mint_unbound().unwrap();
    let host_surface = worth_ui_host_contract::UiHostSurfaceIdentity::mint_unbound().unwrap();
    let binding = worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap();
    let epoch =
        worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(attempt.diagnostic_value());
    let transform = worth_ui_host_contract::UiHostCoordinateTransform::observed_by_host(
        worth_ui_host_contract::UiHostClientAreaObservation::observed_by_host([11, 13], dimensions),
        worth_ui_host_contract::UiHostViewportTransformObservation::observed_by_host(
            [2.0, 1.0],
            [1.0, 1.0],
            [0.0, 0.0],
        ),
        worth_ui_host_contract::UiHostCoordinatePosture::observed_by_host(
            worth_ui_host_contract::UiHostCoordinateOrientation::TopLeftOrigin,
            worth_ui_host_contract::UiHostCoordinateRounding::PixelCenterNearest,
        ),
    );
    CaptureFixture {
        frame,
        attempt,
        host_surface,
        binding,
        epoch,
        transform,
    }
}

fn request(
    fixture: &CaptureFixture,
    identity: u64,
    pixels_requested: bool,
) -> worth_ui_host_contract::UiHostVisualCaptureRequest {
    request_with_artifact(fixture, identity, pixels_requested, 8)
}

fn request_with_budget(
    fixture: &CaptureFixture,
    identity: u64,
    maximum_pixel_bytes: u64,
) -> worth_ui_host_contract::UiHostVisualCaptureRequest {
    request_with_artifact(fixture, identity, true, maximum_pixel_bytes)
}

fn request_with_artifact(
    fixture: &CaptureFixture,
    identity: u64,
    pixels_requested: bool,
    maximum_pixel_bytes: u64,
) -> worth_ui_host_contract::UiHostVisualCaptureRequest {
    worth_ui_host_contract::UiHostVisualCaptureRequest::admitted_by_runtime(
        worth_ui_host_contract::UiHostCaptureRequestIdentity::issued_by_runtime(identity),
        worth_ui_host_contract::UiHostCaptureFrameAffinity::observed_by_runtime(
            fixture.frame,
            fixture.attempt,
        ),
        worth_ui_host_contract::UiHostCaptureSurfaceAffinity::observed_by_runtime(
            7,
            fixture.host_surface,
            fixture.binding,
            fixture.epoch,
        ),
        worth_ui_host_contract::UiHostCaptureArtifactBudget::admitted_by_runtime(
            pixels_requested,
            maximum_pixel_bytes,
        ),
    )
}

struct GatedCapturePort {
    settlement: Rc<Cell<bool>>,
}

struct GatedPendingCapture {
    settlement: Rc<Cell<bool>>,
    canonical_byte_len: usize,
}

impl super::port::UiNativeCaptureReadbackPort for GatedCapturePort {
    fn begin(
        &mut self,
        _graphics: Option<&crate::native::UiNativePresentationAccess>,
        layout: super::readback::UiNativeReadbackLayout,
    ) -> Result<Box<dyn super::port::UiNativePendingCaptureReadback>, ()> {
        Ok(Box::new(GatedPendingCapture {
            settlement: Rc::clone(&self.settlement),
            canonical_byte_len: layout.canonical_byte_len(),
        }))
    }
}

impl super::port::UiNativePendingCaptureReadback for GatedPendingCapture {
    fn poll(
        self: Box<Self>,
        _graphics: Option<&crate::native::UiNativePresentationAccess>,
    ) -> super::port::UiNativeCaptureReadbackPoll {
        self.poll_recovery(None)
    }

    fn poll_recovery(
        self: Box<Self>,
        _graphics: Option<&crate::native::UiNativePresentationAccess>,
    ) -> super::port::UiNativeCaptureReadbackPoll {
        if self.settlement.get() {
            super::port::UiNativeCaptureReadbackPoll::Captured(
                vec![17; self.canonical_byte_len].into_boxed_slice(),
            )
        } else {
            super::port::UiNativeCaptureReadbackPoll::PhysicalCompletionIndeterminate(self)
        }
    }
}

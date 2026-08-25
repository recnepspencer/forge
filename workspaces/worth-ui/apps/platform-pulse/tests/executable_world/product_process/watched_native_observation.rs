use crate::external_observation::{
    observe_stable_process_liveness, NativeClientPixelCapture,
    ProcessBoundNativeClientAreaObservation, StableProcessLivenessObservation,
};
use crate::failure_teardown::PulseExecutableWorldFailure;
use crate::native_platform::NativePlatformContract;

use super::NativeBoundExecutableWorld;

pub(super) struct WatchedNativeObservation {
    pub(super) client: ProcessBoundNativeClientAreaObservation,
    pub(super) liveness: StableProcessLivenessObservation,
    pub(super) pixels: NativeClientPixelCapture,
}

pub(super) fn observe_watched_native(
    world: &mut NativeBoundExecutableWorld,
) -> Result<WatchedNativeObservation, PulseExecutableWorldFailure> {
    let client = world
        .platform
        .observe_bound_client_area(&world.native_client)
        .map_err(PulseExecutableWorldFailure::Native)?;
    let liveness = observe_stable_process_liveness(&mut world.process)
        .map_err(PulseExecutableWorldFailure::Liveness)?;
    let pixels = world
        .platform
        .capture_client_area(&world.native_client)
        .map_err(PulseExecutableWorldFailure::Native)?;
    Ok(WatchedNativeObservation {
        client,
        liveness,
        pixels,
    })
}

pub(super) fn observe_watched_native_until_pixels(
    world: &mut NativeBoundExecutableWorld,
    expected: &NativeClientPixelCapture,
    deadline: Instant,
    posture: &'static str,
) -> Result<WatchedNativeObservation, PulseExecutableWorldFailure> {
    let client = world
        .platform
        .observe_bound_client_area(&world.native_client)
        .map_err(PulseExecutableWorldFailure::Native)?;
    let liveness = observe_stable_process_liveness(&mut world.process)
        .map_err(PulseExecutableWorldFailure::Liveness)?;
    let mut capture_count = 0_u32;
    loop {
        let pixels = world
            .platform
            .capture_client_area(&world.native_client)
            .map_err(PulseExecutableWorldFailure::Native)?;
        capture_count = capture_count.saturating_add(pixels.capture_count());
        if pixels.width() == expected.width()
            && pixels.height() == expected.height()
            && pixels.rgba() == expected.rgba()
        {
            return Ok(WatchedNativeObservation {
                client,
                liveness,
                pixels: pixels.with_capture_count(capture_count),
            });
        }
        if Instant::now() >= deadline {
            return Err(PulseExecutableWorldFailure::Native(
                crate::native_platform::NativePlatformFailure::ClientPixelDeadline(posture),
            ));
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

pub(super) fn observe_watched_native_stable(
    world: &mut NativeBoundExecutableWorld,
    deadline: Instant,
    posture: &'static str,
) -> Result<WatchedNativeObservation, PulseExecutableWorldFailure> {
    let client = world
        .platform
        .observe_bound_client_area(&world.native_client)
        .map_err(PulseExecutableWorldFailure::Native)?;
    let liveness = observe_stable_process_liveness(&mut world.process)
        .map_err(PulseExecutableWorldFailure::Liveness)?;
    let mut capture_count = 0_u32;
    let mut predecessor: Option<NativeClientPixelCapture> = None;
    loop {
        let pixels = world
            .platform
            .capture_client_area(&world.native_client)
            .map_err(PulseExecutableWorldFailure::Native)?;
        capture_count = capture_count.saturating_add(pixels.capture_count());
        if predecessor.as_ref().is_some_and(|prior| {
            pixels.width() == prior.width()
                && pixels.height() == prior.height()
                && pixels.rgba() == prior.rgba()
        }) {
            return Ok(WatchedNativeObservation {
                client,
                liveness,
                pixels: pixels.with_capture_count(capture_count),
            });
        }
        predecessor = Some(pixels);
        if Instant::now() >= deadline {
            return Err(PulseExecutableWorldFailure::Native(
                crate::native_platform::NativePlatformFailure::ClientPixelDeadline(posture),
            ));
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
use std::time::{Duration, Instant};

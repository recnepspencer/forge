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

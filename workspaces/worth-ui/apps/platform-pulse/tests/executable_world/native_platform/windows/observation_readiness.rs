use std::thread;
use std::time::{Duration, Instant};

use super::{
    NativePlatformContract, NativePlatformFailure, ProcessBoundNativeClientAreaObservation,
    WindowsNativePlatform, WindowsProcessBoundNativeClientArea,
};

pub(super) fn await_ready(
    platform: &WindowsNativePlatform,
    bound: &WindowsProcessBoundNativeClientArea,
    deadline: Instant,
) -> Result<ProcessBoundNativeClientAreaObservation, NativePlatformFailure> {
    loop {
        let observation = platform.observe_bound_client_area(bound)?;
        let title = bound
            .window
            .GetWindowText()
            .map_err(|error| NativePlatformFailure::WindowEnumeration(error.to_string()))?;
        if title == "WORTH UI External Observation Ready" {
            return Ok(observation);
        }
        if Instant::now() >= deadline {
            return Err(NativePlatformFailure::ExternalObservationDeadline);
        }
        thread::sleep(Duration::from_millis(20));
    }
}

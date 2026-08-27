/// Map actual runtime-issued shutdown attempts through the same production
/// projection used by the native application driver.
pub fn native_shutdown_attempt_observations_for_certification(
    attempts: &[crate::mounting::UiMountedPresentationShutdownAttempt],
) -> Box<[worth_ui_host_native::UiNativeClientShutdownAttemptObservation]> {
    crate::native_platform::map_shutdown_attempts(attempts)
}

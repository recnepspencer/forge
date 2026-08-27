use crate::native_platform::{current_platform_posture, NativePlatformPosture};
use crate::source_delta::PulseCausalActionManifest;

use super::platform_pulse_journey::{self, PlatformPulseJourneyDeltas};

#[test]
fn pulse_native_cutover_runs_the_complete_causal_journey() {
    assert_eq!(
        current_platform_posture(),
        NativePlatformPosture::CertifiedExecutable
    );
    let manifest = PulseCausalActionManifest::checked_in()
        .unwrap_or_else(|failure| panic!("admit checked-in causal manifest: {failure}"));
    let installation_path = crate::installation::PulseInstallationPath::fresh();
    let native = platform_pulse_journey::complete_native(
        PlatformPulseJourneyDeltas::exact()
            .unwrap_or_else(|failure| panic!("derive exact native source deltas: {failure}")),
        &manifest,
        &installation_path,
    );
    assert!(native.cost().full_journey() <= manifest.host_journey_deadline());
    let verdict = native
        .evidence()
        .validate()
        .unwrap_or_else(|failure| panic!("post-retirement native journey: {failure}"));
    assert!(verdict.event_count() > 0);
    assert_ne!(verdict.process_id(), 0);
    assert_ne!(verdict.exit_poll_count(), 0);
    assert!(native.closed().evidence().installation_removed());
    native.cost().report();
}

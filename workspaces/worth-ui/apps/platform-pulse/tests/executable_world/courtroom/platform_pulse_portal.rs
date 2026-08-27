use crate::adjudication::ExecutableLifecycleCleanupEvidence;
use crate::product_process::{
    PlatformPulseNativeSampleFrameEvidence, PlatformPulsePortalJourneyEvidence,
};
use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseSemanticFocusCause, PlatformPulseSemanticFocusPhysicalOutcome,
};

use super::platform_pulse_cleanup::close_recovered_at_sequence;
use super::platform_pulse_journey::{complete_open, PlatformPulseJourneyDeltas};

#[test]
fn native_portal_opens_focuses_and_closes_through_intent_and_escape() {
    let recovered = complete_open(
        PlatformPulseJourneyDeltas::exact().expect("derive the exact inherited source deltas"),
    )
    .into_recovered();
    let completed = recovered
        .complete_portal_journey()
        .unwrap_or_else(|failure| {
            panic!("native Portal open/focus/close public behavior must remain causal: {failure}")
        });
    assert_portal_pixels(completed.evidence());
    assert_portal_focus(completed.evidence());
    let shutdown_sequence = completed.evidence().expected_shutdown_sequence();
    let closed = close_recovered_at_sequence(completed.into_recovered(), shutdown_sequence);
    assert!(closed.evidence().successful_exit().status().success());
    assert_native_motion_samples(closed.evidence());
}

fn assert_native_motion_samples(evidence: &ExecutableLifecycleCleanupEvidence) {
    let samples: &[PlatformPulseNativeSampleFrameEvidence] =
        evidence.native_close_evidence().sample_frames();
    assert!(
        samples.len() >= 2,
        "the real native journey must retain multiple presentation-only Motion samples"
    );
    assert!(samples.iter().all(|sample| {
        sample.presentation_epoch().is_some()
            && sample.presentation_epoch() == sample.presentation_attempt()
    }));
    assert!(samples.windows(2).any(|pair| {
        pair[0].frame() == pair[1].frame()
            && pair[0].presentation_epoch() != pair[1].presentation_epoch()
    }));
    assert!(samples.iter().any(|sample| {
        sample.logical_damage_regions() > 0
            && sample.rendered_pixels() > 0
            && sample.queue_submissions() > 0
            && sample.presents() > 0
    }));
}

fn assert_portal_focus(evidence: &PlatformPulsePortalJourneyEvidence) {
    let [first_open, intent_close, second_open, escape_close] = evidence.focus_publications();
    for opened in [first_open, second_open] {
        assert_eq!(
            opened.cause(),
            PlatformPulseSemanticFocusCause::PortalInitial
        );
        assert_eq!(
            opened.physical_outcome(),
            PlatformPulseSemanticFocusPhysicalOutcome::Applied
        );
        assert!(opened.current().is_some());
        assert!(opened.host_request().is_some());
    }
    for (opened, restored) in [(first_open, intent_close), (second_open, escape_close)] {
        assert_eq!(
            restored.cause(),
            PlatformPulseSemanticFocusCause::PortalRestoration
        );
        assert_eq!(restored.previous(), opened.current());
        assert_eq!(restored.current(), opened.previous());
    }
    assert_eq!(escape_close.frame(), evidence.escape_dismissed_frame());
}

fn assert_portal_pixels(evidence: &PlatformPulsePortalJourneyEvidence) {
    let resized = evidence.resized_open_pixels();
    assert!(resized.overlay_matching_pixels() * 4 >= resized.sampled_pixels() * 3);
    assert!(resized.authored_surface_matching_pixels() > 0);
    assert!(resized.semantic_ink_pixels() >= 60);
    for pixels in [
        evidence.intent_close_pixels(),
        evidence.escape_close_pixels(),
    ] {
        assert!(pixels.changed_pixels() * 2 >= pixels.sampled_pixels());
        assert!(pixels.overlay_matching_pixels() * 4 >= pixels.sampled_pixels() * 3);
        assert!(pixels.authored_surface_matching_pixels() > 0);
        assert!(pixels.semantic_ink_pixels() >= 60);
    }
}

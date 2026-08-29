use crate::adjudication::ExecutableLifecycleCleanupEvidence;
use crate::product_process::{
    PlatformPulseNativeSampleFrameEvidence, PlatformPulsePortalJourneyEvidence,
};
use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseSemanticFocusCause, PlatformPulseSemanticFocusPhysicalOutcome,
};

use super::platform_pulse_cleanup::close_recovered_at_sequence;
use super::platform_pulse_journey::complete_portal_open;

#[test]
fn native_portal_opens_focuses_and_closes_through_intent_and_escape() {
    let ready = complete_portal_open();
    let journey_started = ready.native_journey_started();
    let completed = ready.complete_portal_journey().unwrap_or_else(|failure| {
        panic!("native Portal open/focus/close public behavior must remain causal: {failure}")
    });
    assert_portal_pixels(completed.evidence());
    assert_portal_focus(completed.evidence());
    assert_runtime_service_story(completed.evidence());
    let shutdown_sequence = completed.evidence().expected_shutdown_sequence();
    let closed = close_recovered_at_sequence(completed.into_ready(), shutdown_sequence);
    assert!(closed.evidence().successful_exit().status().success());
    assert_native_motion_samples(closed.evidence());
    assert!(
        journey_started.elapsed() <= std::time::Duration::from_secs(45),
        "the complete RS-01 native product journey must finish within 45 seconds"
    );
}

fn assert_runtime_service_story(evidence: &PlatformPulsePortalJourneyEvidence) {
    let [input, application_action, application_terminal, portal_action, portal_terminal] =
        evidence.runtime_service_sequences();
    assert!(input < application_action);
    assert!(application_action < application_terminal);
    assert!(application_terminal < portal_action);
    assert!(portal_action < portal_terminal);
    let [active, submitted] = evidence.runtime_service_query_revisions();
    assert_eq!(submitted, active + 1);
    let [command_pixels, query_denial_pixels] = evidence.runtime_service_changed_pixels();
    assert!(command_pixels >= 24);
    assert!(query_denial_pixels >= 24);
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
    let [opened, escape_close] = evidence.focus_publications();
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
    assert_eq!(
        escape_close.cause(),
        PlatformPulseSemanticFocusCause::PortalRestoration
    );
    assert_eq!(escape_close.current(), opened.previous());
    assert_eq!(escape_close.frame(), evidence.escape_dismissed_frame());
}

fn assert_portal_pixels(evidence: &PlatformPulsePortalJourneyEvidence) {
    let resized = evidence.resized_open_pixels();
    assert!(resized.overlay_matching_pixels() * 4 >= resized.sampled_pixels() * 3);
    assert!(resized.authored_surface_matching_pixels() > 0);
    assert!(resized.semantic_ink_pixels() >= 60);
    let pixels = evidence.initial_open_pixels();
    assert!(pixels.changed_pixels() * 2 >= pixels.sampled_pixels());
    assert!(pixels.overlay_matching_pixels() * 4 >= pixels.sampled_pixels() * 3);
    assert!(pixels.authored_surface_matching_pixels() > 0);
    assert!(pixels.semantic_ink_pixels() >= 60);
    assert!(evidence.portal_rebind_sequence() > 0);
    let [removed_primary, fallback_action] = evidence.portal_rebind_pixels();
    assert!(removed_primary >= 12);
    assert!(fallback_action > 0);
}

use std::time::{Duration, Instant};

use crate::adjudication::ExpectedNativeColor;
use crate::installation::CanonicalPlatformPulse;
use crate::product_process::{
    AwaitingFirstFrame, AwaitingPreservation, AwaitingRecovery, AwaitingReplacement,
    CargoBuiltPlatformPulse, Closed, GreenSuccessor, InitialBlue, Installed, PreservedPredecessor,
    Published, PulseExecutableWorld, RecoveredBlue,
};
use crate::source_delta::{
    CanonicalBlueRecoverySourceDelta, GreenPulseSourceDelta, MalformedPulseSourceDelta,
    PulseSourceDeltaDefinitionFailure, PulseSourceDeltaIdentity,
};

use super::journey_cost::{JourneyCostInputs, PlatformPulseJourneyCost};

const TRANSITION_DEADLINE: Duration = Duration::from_secs(5);

pub(super) struct PlatformPulseJourneyDeltas {
    canonical: CanonicalPlatformPulse,
    green: GreenPulseSourceDelta,
    malformed: MalformedPulseSourceDelta,
    recovery: CanonicalBlueRecoverySourceDelta,
}

pub(super) struct CompletedPlatformPulseJourney {
    closed: PulseExecutableWorld<Closed>,
    cost: PlatformPulseJourneyCost,
}

impl PlatformPulseJourneyDeltas {
    pub(super) fn exact() -> Result<Self, PulseSourceDeltaDefinitionFailure> {
        let canonical = CanonicalPlatformPulse::checked_in();
        Ok(Self {
            canonical,
            green: GreenPulseSourceDelta::from_checked_in(canonical)?,
            malformed: MalformedPulseSourceDelta::stable(),
            recovery: CanonicalBlueRecoverySourceDelta::exact(canonical),
        })
    }
}

pub(super) fn complete(deltas: PlatformPulseJourneyDeltas) -> CompletedPlatformPulseJourney {
    let journey_started = Instant::now();
    let initial = launch_initial(deltas.canonical);
    let first_publication = initial.launch_to_first_publication();
    let mut native_captures = initial.evidence().capture_count();
    let window_lookups = initial.evidence().client_area().window_lookup_count();
    let green = publish_green(initial, deltas.green);
    native_captures += green.evidence().capture_count();
    let preserved = preserve_green(green, deltas.malformed);
    native_captures += preserved.evidence().capture_count();
    let recovered = recover_blue(preserved, deltas.recovery);
    native_captures += recovered.evidence().capture_count();
    let source_actions = recovered.source_action_count();
    let closed = close_recovered(recovered);
    let cost = PlatformPulseJourneyCost::from_completed(
        JourneyCostInputs {
            first_publication,
            full_journey: journey_started.elapsed(),
            source_actions,
            native_captures,
            window_lookups,
        },
        closed.evidence(),
    );
    CompletedPlatformPulseJourney { closed, cost }
}

impl CompletedPlatformPulseJourney {
    pub(super) fn closed(&self) -> &PulseExecutableWorld<Closed> {
        &self.closed
    }

    pub(super) fn cost(&self) -> PlatformPulseJourneyCost {
        self.cost
    }
}

fn launch_initial(
    canonical: CanonicalPlatformPulse,
) -> PulseExecutableWorld<Published<InitialBlue>> {
    let installed: PulseExecutableWorld<Installed> = PulseExecutableWorld::install(canonical)
        .unwrap_or_else(|failure| panic!("install exact canonical source: {failure}"));
    let binary = CargoBuiltPlatformPulse::exact()
        .unwrap_or_else(|failure| panic!("resolve exact Cargo executable: {failure}"));
    let awaiting: PulseExecutableWorld<AwaitingFirstFrame> = installed
        .launch(binary)
        .unwrap_or_else(|failure| panic!("launch exact product process: {failure}"));
    let published = awaiting
        .await_first_frame(Instant::now() + TRANSITION_DEADLINE)
        .unwrap_or_else(|failure| {
            panic!("causal first publication plus independent native pixels: {failure}")
        });
    let evidence = published.evidence();
    assert_eq!(evidence.sequence_pair(), (1, 2));
    assert!(evidence.first_frame().actual_native_effect_count() > 0);
    assert!(evidence.client_area().window_lookup_count() > 0);
    assert!(evidence.liveness().liveness_checks() >= 2);
    assert_eq!(evidence.capture_count(), 1);
    assert!(evidence.matching_blue_samples() * 4 >= evidence.sampled_pixels() * 3);
    published
}

fn publish_green(
    initial: PulseExecutableWorld<Published<InitialBlue>>,
    delta: GreenPulseSourceDelta,
) -> PulseExecutableWorld<Published<GreenSuccessor>> {
    let first_process = initial.evidence().process_id();
    let first_window = initial.evidence().client_area().window();
    let awaiting: PulseExecutableWorld<AwaitingReplacement> = initial
        .apply_green(delta)
        .unwrap_or_else(|failure| panic!("atomically apply green source: {failure}"));
    let green = awaiting
        .await_green_successor(Instant::now() + TRANSITION_DEADLINE)
        .unwrap_or_else(|failure| {
            panic!("green successor publication plus independent pixels: {failure}")
        });
    let evidence = green.evidence();
    assert_action(evidence.action(), PulseSourceDeltaIdentity::Green);
    assert_eq!(evidence.sequence(), 3);
    assert!(evidence.replacement().actual_native_effect_count() > 0);
    assert_eq!(evidence.identity().process_id(), first_process);
    assert_eq!(evidence.identity().window(), first_window);
    assert_eq!(evidence.expected_color(), ExpectedNativeColor::Green);
    assert!(evidence.liveness().liveness_checks() >= 2);
    assert_eq!(evidence.capture_count(), 1);
    assert!(evidence.matching_color_samples() * 4 >= evidence.sampled_pixels() * 3);
    green
}

fn preserve_green(
    green: PulseExecutableWorld<Published<GreenSuccessor>>,
    delta: MalformedPulseSourceDelta,
) -> PulseExecutableWorld<PreservedPredecessor> {
    let predecessor = green.evidence();
    let prior_generation = predecessor.replacement().active_generation();
    let prior_frame = predecessor.replacement().successor_frame();
    let prior_window = predecessor.identity().window();
    let awaiting: PulseExecutableWorld<AwaitingPreservation> = green
        .apply_malformed(delta)
        .unwrap_or_else(|failure| panic!("atomically apply malformed source: {failure}"));
    let preserved = awaiting
        .await_preserved_predecessor(Instant::now() + TRANSITION_DEADLINE)
        .unwrap_or_else(|failure| {
            panic!("typed denial plus exact green predecessor preservation: {failure}")
        });
    let evidence = preserved.evidence();
    assert_action(evidence.action(), PulseSourceDeltaIdentity::Malformed);
    assert_eq!(evidence.sequence(), 4);
    assert_eq!(evidence.preserved().active_generation(), prior_generation);
    assert_eq!(evidence.preserved().active_frame(), prior_frame);
    assert_eq!(evidence.identity().window(), prior_window);
    assert_eq!(evidence.expected_color(), ExpectedNativeColor::Green);
    assert!(evidence.liveness().liveness_checks() >= 2);
    assert_eq!(evidence.capture_count(), 1);
    assert!(evidence.matching_green_samples() * 4 >= evidence.sampled_pixels() * 3);
    preserved
}

fn recover_blue(
    preserved: PulseExecutableWorld<PreservedPredecessor>,
    delta: CanonicalBlueRecoverySourceDelta,
) -> PulseExecutableWorld<Published<RecoveredBlue>> {
    let predecessor = preserved.evidence();
    let prior_generation = predecessor.preserved().active_generation();
    let process = predecessor.identity().process_id();
    let window = predecessor.identity().window();
    let awaiting: PulseExecutableWorld<AwaitingRecovery> = preserved
        .restore_canonical(delta)
        .unwrap_or_else(|failure| panic!("atomically restore canonical source: {failure}"));
    let recovered = awaiting
        .await_recovered_blue(Instant::now() + TRANSITION_DEADLINE)
        .unwrap_or_else(|failure| {
            panic!("fresh canonical blue successor plus independent pixels: {failure}")
        });
    let evidence = recovered.evidence();
    assert_action(
        evidence.action(),
        PulseSourceDeltaIdentity::CanonicalBlueRecovery,
    );
    assert_eq!(evidence.sequence(), 5);
    assert_eq!(recovered.preservation_evidence().sequence(), 4);
    assert_eq!(evidence.identity().process_id(), process);
    assert_eq!(evidence.identity().window(), window);
    assert_ne!(evidence.replacement().active_generation(), prior_generation);
    assert_eq!(evidence.expected_color(), ExpectedNativeColor::Blue);
    assert!(evidence.liveness().liveness_checks() >= 2);
    assert_eq!(evidence.capture_count(), 1);
    assert!(evidence.matching_color_samples() * 4 >= evidence.sampled_pixels() * 3);
    recovered
}

fn close_recovered(
    recovered: PulseExecutableWorld<Published<RecoveredBlue>>,
) -> PulseExecutableWorld<Closed> {
    let closed = recovered
        .close_native_window(Instant::now() + TRANSITION_DEADLINE)
        .unwrap_or_else(|failure| {
            panic!("normal close, typed shutdown, successful exit, and cleanup: {failure}")
        });
    let cleanup = closed.evidence();
    assert_eq!(cleanup.close_request_count(), 1);
    assert_eq!(cleanup.shutdown_sequence(), 6);
    assert!(cleanup.shutdown().host_session_released());
    assert!(cleanup.successful_exit().status().success());
    assert!(cleanup.successful_exit().poll_count() > 0);
    assert!(cleanup.installation_removed());
    closed
}

fn assert_action<Kind>(
    action: &crate::source_delta::AppliedPulseSourceDelta<Kind>,
    expected: PulseSourceDeltaIdentity,
) {
    assert_eq!(action.identity(), expected);
    assert_eq!(action.action_count(), 1);
    assert!(action.written_bytes() > 0);
    assert_ne!(action.content_fingerprint(), 0);
    assert_eq!(
        action
            .entry_source()
            .file_name()
            .and_then(|name| name.to_str()),
        Some("main.wui")
    );
}

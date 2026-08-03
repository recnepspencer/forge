use std::time::{Duration, Instant};

use crate::adjudication::ExpectedNativeColor;
use crate::installation::CanonicalPlatformPulse;
use crate::product_process::{
    AwaitingFirstFrame, AwaitingPreservation, AwaitingRecovery, AwaitingReplacement,
    AwaitingSchemaStop, AwaitingStatusRecovery, CargoBuiltPlatformPulse, Closed, FinalRecovered,
    FirstCurrent, GreenSuccessor, IdentityTraced, InitialBlue, Installed, NativeInputReached,
    OverlayCleared, OverlayPublished, PreservedPredecessor, Published, PulseExecutableWorld,
    RecoveredBlue, SchemaStopped, SecondCurrent, SnapshotCaptured,
};
use crate::source_delta::{
    CanonicalBlueRecoverySourceDelta, GreenPulseSourceDelta, MalformedPulseSourceDelta,
    PulseSourceDeltaDefinitionFailure, PulseSourceDeltaIdentity, QueryStatusV1, QueryStatusV2,
    RevisionSchemaSourceDelta, StatusSchemaRecoverySourceDelta,
};

use super::journey_cost::{JourneyCostInputs, PlatformPulseJourneyCost};
use super::platform_pulse_cleanup::close_recovered;
use super::platform_pulse_input::reach_native_input;

const TRANSITION_DEADLINE: Duration = Duration::from_secs(5);

pub(super) struct PlatformPulseJourneyDeltas {
    canonical: CanonicalPlatformPulse,
    green: GreenPulseSourceDelta,
    malformed: MalformedPulseSourceDelta,
    recovery: CanonicalBlueRecoverySourceDelta,
    revision_schema: RevisionSchemaSourceDelta,
    status_schema_recovery: StatusSchemaRecoverySourceDelta,
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
            revision_schema: RevisionSchemaSourceDelta::from_checked_in(canonical)?,
            status_schema_recovery: StatusSchemaRecoverySourceDelta::exact(canonical),
        })
    }
}

pub(super) fn complete(deltas: PlatformPulseJourneyDeltas) -> CompletedPlatformPulseJourney {
    let journey_started = Instant::now();
    let initial = launch_initial(deltas.canonical);
    let first_publication = initial.launch_to_first_publication();
    let mut native_captures = initial.evidence().capture_count();
    let window_lookups = initial.evidence().client_area().window_lookup_count();
    let input_reached = reach_native_input(initial, Instant::now() + TRANSITION_DEADLINE);
    native_captures += input_reached.evidence().capture_count();
    let first_current = publish_first_current(input_reached);
    native_captures += first_current.query_evidence().pixels().capture_count();
    let (visualized, visual_captures) = publish_visual_identity(first_current);
    native_captures += visual_captures;
    let second_current = publish_second_current(visualized);
    native_captures += second_current.query_evidence().pixels().capture_count();
    let green = publish_green(second_current, deltas.green);
    native_captures += green.evidence().capture_count();
    let preserved = preserve_green(green, deltas.malformed);
    native_captures += preserved.evidence().capture_count();
    let recovered = recover_blue(preserved, deltas.recovery);
    native_captures += recovered.evidence().capture_count();
    let stopped = stop_on_revision_schema(recovered, deltas.revision_schema);
    native_captures += stopped.evidence().replacement().capture_count();
    let recovered = recover_status_schema(stopped, deltas.status_schema_recovery);
    native_captures += recovered.evidence().replacement().capture_count();
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

fn publish_visual_identity(
    initial: PulseExecutableWorld<Published<FirstCurrent>>,
) -> (
    PulseExecutableWorld<Published<OverlayCleared<FirstCurrent>>>,
    u32,
) {
    let snapshot: PulseExecutableWorld<Published<SnapshotCaptured<FirstCurrent>>> = initial
        .await_visual_snapshot(Instant::now() + TRANSITION_DEADLINE)
        .unwrap_or_else(|failure| panic!("capture exact first-frame snapshot: {failure}"));
    assert_ne!(snapshot.evidence().snapshot().affinity().frame(), 0);
    assert!(
        snapshot.evidence().snapshot().pixels().byte_count()
            <= worth_ui_platform_pulse::visual_identity_pulse::PLATFORM_PULSE_MAXIMUM_PIXEL_BYTES
    );

    let trace: PulseExecutableWorld<Published<IdentityTraced<FirstCurrent>>> = snapshot
        .await_identity_trace(Instant::now() + TRANSITION_DEADLINE)
        .unwrap_or_else(|failure| panic!("trace target and background identity: {failure}"));
    assert_eq!(
        trace.evidence().trace().target().hit().authored_semantic_name(),
        worth_ui_platform_pulse::visual_identity_pulse::PLATFORM_PULSE_IDENTITY_TARGET_AUTHORED_NAME
    );

    let overlay: PulseExecutableWorld<Published<OverlayPublished<FirstCurrent>>> = trace
        .await_overlay_published(Instant::now() + TRANSITION_DEADLINE)
        .unwrap_or_else(|failure| panic!("publish visible mounted identity overlay: {failure}"));
    let (matching, sampled) = overlay.evidence().border_ratio();
    assert_eq!(overlay.evidence().sequence(), 11);
    assert!(matching * 4 >= sampled * 3);
    let overlay_captures = overlay.evidence().capture_count();

    let cleared = overlay
        .await_overlay_cleared(Instant::now() + TRANSITION_DEADLINE)
        .unwrap_or_else(|failure| panic!("clear overlay and restore native pixels: {failure}"));
    assert_ne!(
        cleared.evidence().clear().cleared_frame(),
        cleared.evidence().clear().published_frame()
    );
    assert_eq!(cleared.evidence().sequence(), 12);
    let captures = overlay_captures + cleared.evidence().capture_count();
    (cleared, captures)
}

fn publish_first_current(
    initial: PulseExecutableWorld<Published<NativeInputReached<InitialBlue>>>,
) -> PulseExecutableWorld<Published<FirstCurrent>> {
    let current = initial
        .publish_first_query_value(QueryStatusV1)
        .unwrap_or_else(|failure| panic!("publish first Query world input: {failure}"))
        .await_first_query_value(Instant::now() + TRANSITION_DEADLINE)
        .unwrap_or_else(|failure| panic!("first Query value reaches native pixels: {failure}"));
    assert_eq!(current.query_evidence().issued_sequence(), 7);
    assert_eq!(current.query_evidence().published_sequence(), 8);
    assert!(
        current.query_evidence().matching_blue_samples() * 4
            >= current.query_evidence().sampled_pixels() * 3
    );
    current
}

fn publish_second_current(
    visualized: PulseExecutableWorld<Published<OverlayCleared<FirstCurrent>>>,
) -> PulseExecutableWorld<Published<SecondCurrent>> {
    let current = visualized
        .publish_second_query_value(QueryStatusV2)
        .unwrap_or_else(|failure| panic!("publish second Query world input: {failure}"))
        .await_second_query_value(Instant::now() + TRANSITION_DEADLINE)
        .unwrap_or_else(|failure| panic!("second Query value reaches native pixels: {failure}"));
    assert_eq!(current.query_evidence().issued_sequence(), 13);
    assert_eq!(current.query_evidence().published_sequence(), 14);
    assert_eq!(current.refresh_retirement_evidence().sequence(), 15);
    assert_eq!(current.refresh_snapshot_evidence().sequence(), 16);
    assert!(
        current.query_evidence().matching_blue_samples() * 4
            >= current.query_evidence().sampled_pixels() * 3
    );
    current
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
    assert_eq!(evidence.sequence_quad(), (1, 1, 3, 1));
    assert_eq!(
        evidence.pending_projection().projection_identity(),
        "platform.pulse.status"
    );
    assert!(evidence.first_frame().actual_native_effect_count() > 0);
    assert!(evidence.client_area().window_lookup_count() > 0);
    assert!(evidence.liveness().liveness_checks() >= 2);
    assert_eq!(evidence.capture_count(), 1);
    assert!(evidence.matching_blue_samples() * 4 >= evidence.sampled_pixels() * 3);
    published
}

fn publish_green(
    initial: PulseExecutableWorld<Published<SecondCurrent>>,
    delta: GreenPulseSourceDelta,
) -> PulseExecutableWorld<Published<GreenSuccessor>> {
    let first_process = initial.query_evidence().client().process_id();
    let first_window = initial.query_evidence().client().window();
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
    assert_eq!(evidence.sequence(), 17);
    assert_eq!(
        green.retirement_evidence().retirement().successor_frame(),
        evidence.replacement().successor_frame().diagnostic_value()
    );
    let comparison = green.comparison_evidence().comparison();
    assert!(
        !comparison.identity_rebound(),
        "the exact color-only edit must preserve stable authored identity"
    );
    assert_eq!(comparison.retained_pixels_differ(), Some(true));
    assert!(comparison.retained_pixel_bytes_examined() > 0);
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
    assert_eq!(evidence.sequence(), 21);
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
    assert_eq!(evidence.sequence(), 22);
    assert_eq!(recovered.preservation_evidence().sequence(), 21);
    assert_eq!(evidence.identity().process_id(), process);
    assert_eq!(evidence.identity().window(), window);
    assert_ne!(evidence.replacement().active_generation(), prior_generation);
    assert_eq!(evidence.expected_color(), ExpectedNativeColor::Blue);
    assert!(evidence.liveness().liveness_checks() >= 2);
    assert_eq!(evidence.capture_count(), 1);
    assert!(evidence.matching_color_samples() * 4 >= evidence.sampled_pixels() * 3);
    recovered
}

fn stop_on_revision_schema(
    recovered: PulseExecutableWorld<Published<RecoveredBlue>>,
    delta: RevisionSchemaSourceDelta,
) -> PulseExecutableWorld<Published<SchemaStopped>> {
    let prior_query = recovered.query_basis().clone();
    let awaiting: PulseExecutableWorld<AwaitingSchemaStop> = recovered
        .apply_revision_schema(delta)
        .unwrap_or_else(|failure| panic!("atomically apply revision schema: {failure}"));
    let stopped = awaiting
        .await_schema_stopped(Instant::now() + TRANSITION_DEADLINE)
        .unwrap_or_else(|failure| {
            panic!("schema mismatch preserves value and becomes visible: {failure}")
        });
    let evidence = stopped.evidence();
    assert_action(
        evidence.replacement().action(),
        PulseSourceDeltaIdentity::RevisionSchema,
    );
    assert_eq!(evidence.replacement().sequence(), 23);
    assert_eq!(evidence.query_basis(), &prior_query);
    assert_eq!(
        evidence.transition().kind(),
        worth_ui_platform_pulse::observation_contract::
            PlatformPulseProjectionSchemaTransitionKind::Stopped
    );
    assert!(evidence.retained_upper_pixel_bytes() > 0);
    assert!(evidence.changed_lower_pixel_bytes() > 0);
    assert!(!evidence.canonical_current_restored());
    stopped
}

fn recover_status_schema(
    stopped: PulseExecutableWorld<Published<SchemaStopped>>,
    delta: StatusSchemaRecoverySourceDelta,
) -> PulseExecutableWorld<Published<FinalRecovered>> {
    let stopped_query = stopped.evidence().query_basis().clone();
    let awaiting: PulseExecutableWorld<AwaitingStatusRecovery> = stopped
        .restore_status_schema(delta)
        .unwrap_or_else(|failure| panic!("atomically restore status schema: {failure}"));
    let recovered = awaiting
        .await_status_recovered(Instant::now() + TRANSITION_DEADLINE)
        .unwrap_or_else(|failure| {
            panic!("status schema recovery restores current native pixels: {failure}")
        });
    let evidence = recovered.evidence();
    assert_action(
        evidence.replacement().action(),
        PulseSourceDeltaIdentity::StatusSchemaRecovery,
    );
    assert_eq!(evidence.replacement().sequence(), 24);
    assert_eq!(evidence.query_basis(), &stopped_query);
    assert_eq!(
        evidence.transition().kind(),
        worth_ui_platform_pulse::observation_contract::
            PlatformPulseProjectionSchemaTransitionKind::Recovered
    );
    assert!(evidence.retained_upper_pixel_bytes() > 0);
    assert!(evidence.changed_lower_pixel_bytes() > 0);
    assert!(evidence.canonical_current_restored());
    assert_eq!(
        recovered.stopped_evidence().transition().kind(),
        worth_ui_platform_pulse::observation_contract::
            PlatformPulseProjectionSchemaTransitionKind::Stopped
    );
    recovered
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

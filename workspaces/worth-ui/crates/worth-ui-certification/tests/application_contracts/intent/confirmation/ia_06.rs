use worth_ui::facade::intent::{
    UiIntentAdmissionDecision, UiIntentAdmissionSettlementPosture,
    UiIntentConfirmationContinuation, UiIntentConfirmationStopReason, UiIntentDefinition,
};
use worth_ui::facade::observation_report::UiHostObservationTimeBasis;
use worth_ui_test_support::{
    WorthUiIntentOccupancyCertificationExt, WorthUiMountedIdentityCertificationExt,
};

use super::types::ConfirmationIntent;
use super::world::ConfirmationWorld;

#[test]
fn exact_challenge_reenters_admission_once_without_execution() {
    let (mut world, provider) = ConfirmationWorld::launch_with_provider_observation();
    let issued = world.issue();
    assert_eq!(issued.pending.slot_identity().slot(), 0);
    assert_eq!(issued.pending.slot_identity().generation(), 1);
    assert_eq!(issued.pending.lineage().diagnostic_value(), 1);
    assert_eq!(
        world
            .interaction
            .session
            .intent_confirmation_metrics()
            .pending_challenges(),
        1
    );

    world.publish_successor();
    let route = world.confirmation_route(
        monotonic(issued.pending.expires_at_tick() - 1),
        monotonic(issued.pending.expires_at_tick()),
    );
    let ready = match world
        .interaction
        .session
        .continue_intent_confirmation(route)
    {
        UiIntentConfirmationContinuation::AdmissionReady(ready) => ready,
        UiIntentConfirmationContinuation::Stopped(stop) => {
            panic!("exact challenge must continue: {:?}", stop.reason())
        }
    };
    assert_eq!(ready.definition_id(), issued.pending.definition_id());
    assert_eq!(
        ready.declaration_identity(),
        issued.pending.declaration_identity()
    );
    assert_eq!(ready.lineage(), issued.pending.lineage());
    assert_eq!(ready.retained_payload_count(), 1);
    let admitted = match world.interaction.session.admit_confirmed_intent(
        UiIntentDefinition::<ConfirmationIntent>::application_effect(),
        ready,
    ) {
        UiIntentAdmissionDecision::Admitted(admitted) => admitted,
        UiIntentAdmissionDecision::ConfirmationRequired(_) => {
            panic!("an exact confirmed candidate cannot require confirmation twice")
        }
        UiIntentAdmissionDecision::Stopped(stop) => {
            panic!(
                "an exact confirmed candidate must admit: {:?}",
                stop.reason()
            )
        }
    };
    let active = world.interaction.session.intent_admission_metrics();
    assert_eq!(active.active_attempts(), 1);
    assert_eq!(active.retained_candidates(), 1);
    assert_eq!(active.retained_payloads(), 1);
    assert_eq!(active.admitted(), 1);
    assert_eq!(provider.begin_calls(), 0);
    assert_eq!(
        world
            .interaction
            .session
            .cancel_admitted_intent(admitted)
            .posture(),
        UiIntentAdmissionSettlementPosture::Released
    );
    let released = world.interaction.session.intent_admission_metrics();
    assert_eq!(released.active_attempts(), 0);
    assert_eq!(released.retained_candidates(), 0);
    assert_eq!(released.retained_payloads(), 0);
    assert_eq!(released.released(), 1);
    assert_eq!(provider.begin_calls(), 0);
    let settled = world.interaction.session.intent_confirmation_metrics();
    assert_eq!(settled.pending_challenges(), 0);
    assert_eq!(settled.retained_candidates(), 0);
    assert_eq!(settled.retained_payloads(), 0);

    let duplicate = world.confirmation_route(monotonic(20), monotonic(21));
    assert_stop(
        world
            .interaction
            .session
            .continue_intent_confirmation(duplicate),
        UiIntentConfirmationStopReason::AlreadyContinued,
    );
    assert_eq!(
        world
            .interaction
            .session
            .intent_confirmation_metrics()
            .continued(),
        1
    );
    assert_eq!(
        world
            .interaction
            .session
            .intent_confirmation_metrics()
            .replays(),
        1
    );
}

#[test]
fn same_frame_ambiguous_and_expired_continuations_stop_exactly() {
    let mut same_frame = ConfirmationWorld::launch();
    let issued = same_frame.issue();
    let route = same_frame.confirmation_route(
        monotonic(issued.pending.expires_at_tick() - 1),
        monotonic(issued.pending.expires_at_tick()),
    );
    assert_stop(
        same_frame
            .interaction
            .session
            .continue_intent_confirmation(route),
        UiIntentConfirmationStopReason::ConfirmationNotPresented,
    );

    let mut ambiguous = ConfirmationWorld::launch();
    let first = ambiguous.issue();
    let second = ambiguous.issue();
    assert_ne!(first.pending.lineage(), second.pending.lineage());
    ambiguous.publish_successor();
    let route = ambiguous.confirmation_route(monotonic(20), monotonic(21));
    let stop = expect_stop(
        ambiguous
            .interaction
            .session
            .continue_intent_confirmation(route),
    );
    assert_eq!(
        stop.reason(),
        &UiIntentConfirmationStopReason::AmbiguousPendingChallenges {
            declaration: first.pending.declaration_identity().into(),
            observed: 2,
        }
    );
    assert_eq!(stop.cost().slots_inspected(), 16);
    let metrics = ambiguous.interaction.session.intent_confirmation_metrics();
    assert_eq!(metrics.pending_challenges(), 0);
    assert_eq!(metrics.cancelled(), 2);

    let mut expired = ConfirmationWorld::launch();
    let issued = expired.issue();
    expired.publish_successor();
    let observed = issued.pending.expires_at_tick() + 1;
    let route = expired.confirmation_route(monotonic(observed - 1), monotonic(observed));
    assert_stop(
        expired
            .interaction
            .session
            .continue_intent_confirmation(route),
        UiIntentConfirmationStopReason::Expired {
            expires_at: issued.pending.expires_at_tick(),
            observed,
        },
    );
    assert_eq!(
        expired
            .interaction
            .session
            .intent_confirmation_metrics()
            .expired(),
        1
    );
}

#[test]
fn payload_operability_policy_and_confirmation_drift_are_distinct() {
    for (mut world, mutate, expected) in [
        (
            ConfirmationWorld::launch(),
            ConfirmationWorld::set_revision as fn(&mut ConfirmationWorld, u64),
            UiIntentConfirmationStopReason::PayloadInputChanged,
        ),
        (
            ConfirmationWorld::launch(),
            |world: &mut ConfirmationWorld, value| world.set_mutability(value != 0),
            UiIntentConfirmationStopReason::OperabilityDependencyChanged,
        ),
        (
            ConfirmationWorld::launch(),
            |world: &mut ConfirmationWorld, value| world.set_policy(value != 0),
            UiIntentConfirmationStopReason::PolicyChanged,
        ),
        (
            ConfirmationWorld::launch(),
            |world: &mut ConfirmationWorld, value| world.set_confirmation(value != 0),
            UiIntentConfirmationStopReason::ConfirmationPolicyChanged,
        ),
    ] {
        let issued = world.issue();
        mutate(&mut world, 0);
        world.publish_successor();
        let route = world.confirmation_route(monotonic(20), monotonic(21));
        assert_stop(
            world
                .interaction
                .session
                .continue_intent_confirmation(route),
            expected,
        );
        assert_eq!(issued.pending.lineage().diagnostic_value(), 1);
    }
}

#[test]
fn occupancy_world_time_and_lifecycle_substitution_cannot_continue() {
    let mut occupied = ConfirmationWorld::launch();
    let proof = occupied.operable_proof();
    let issued = occupied.issue();
    let reservation = occupied
        .interaction
        .session
        .reserve_intent_occupancy_for_certification(proof)
        .expect("predecessor idle proof reserves the exact target scope");
    occupied.publish_successor();
    let route = occupied.confirmation_route(monotonic(20), monotonic(21));
    assert_stop(
        occupied
            .interaction
            .session
            .continue_intent_confirmation(route),
        UiIntentConfirmationStopReason::OccupancyChanged,
    );
    let release = occupied
        .interaction
        .session
        .release_intent_occupancy_for_certification(reservation);
    assert_eq!(
        release,
        worth_ui_test_support::UiIntentOccupancyReleasePosture::Released
    );
    assert_eq!(issued.pending.lineage().diagnostic_value(), 1);

    let mut left = ConfirmationWorld::launch();
    let mut right = ConfirmationWorld::launch();
    let _ = left.issue();
    let _ = right.issue();
    left.publish_successor();
    right.publish_successor();
    let foreign = left.confirmation_route(monotonic(20), monotonic(21));
    assert_stop(
        right
            .interaction
            .session
            .continue_intent_confirmation(foreign),
        UiIntentConfirmationStopReason::ApplicationWorldChanged,
    );

    let mut cancelled = ConfirmationWorld::launch();
    let issued = cancelled.issue();
    cancelled
        .interaction
        .session
        .unmount_instance(issued.product_instance)
        .expect("product target unmounts through the production lifecycle");
    let route = cancelled.confirmation_route(monotonic(20), monotonic(21));
    assert_stop(
        cancelled
            .interaction
            .session
            .continue_intent_confirmation(route),
        UiIntentConfirmationStopReason::LifecycleCancelled(
            worth_ui::facade::intent::UiIntentConfirmationCancellationReason::
                MountedInstanceRemoved,
        ),
    );
}

#[test]
fn removed_confirmation_control_cannot_continue_a_live_product_challenge() {
    let mut world = ConfirmationWorld::launch();
    let _ = world.issue();
    world.publish_successor();
    let route = world.confirmation_route(monotonic(20), monotonic(21));
    let confirmation_instance = route.source().target().mounted_instance();
    world
        .interaction
        .session
        .unmount_instance(confirmation_instance)
        .expect("confirmation control unmounts through the production lifecycle");

    assert_stop(
        world
            .interaction
            .session
            .continue_intent_confirmation(route),
        UiIntentConfirmationStopReason::ConfirmationTargetChanged(
            worth_ui::facade::interaction::UiInteractionTargetingDenial::
                MountedInstanceNoLongerCurrent,
        ),
    );
}

#[test]
fn application_rebind_cancels_challenge_before_old_route_can_continue() {
    let mut world = ConfirmationWorld::launch();
    let _ = world.issue();
    world.publish_successor();
    let route = world.confirmation_route(monotonic(20), monotonic(21));
    world.rebind_application();
    let cancelled = world.interaction.session.intent_confirmation_metrics();
    assert_eq!(cancelled.pending_challenges(), 0);
    assert_eq!(cancelled.retained_candidates(), 0);
    assert_eq!(cancelled.retained_payloads(), 0);
    assert_eq!(cancelled.cancelled(), 1);
    world.publish_successor();

    assert!(matches!(
        world.route_at_confirmation_control(),
        worth_ui::facade::intent::UiIntentRouteResolution::Product(_)
    ));

    assert_stop(
        world
            .interaction
            .session
            .continue_intent_confirmation(route),
        UiIntentConfirmationStopReason::LifecycleCancelled(
            worth_ui::facade::intent::UiIntentConfirmationCancellationReason::ApplicationRebound,
        ),
    );
}

#[test]
fn non_monotonic_confirmation_and_shutdown_retire_authority() {
    let mut non_monotonic = ConfirmationWorld::launch();
    let _ = non_monotonic.issue();
    non_monotonic.publish_successor();
    let route = non_monotonic.confirmation_route(
        UiHostObservationTimeBasis::HostWallClockMicros(1_000),
        UiHostObservationTimeBasis::HostWallClockMicros(1_001),
    );
    assert_stop(
        non_monotonic
            .interaction
            .session
            .continue_intent_confirmation(route),
        UiIntentConfirmationStopReason::MonotonicTimeRequired {
            observed: worth_ui::facade::intent::UiIntentConfirmationTimeBasisKind::HostWallClock,
        },
    );

    let mut shutdown = ConfirmationWorld::launch();
    let _ = shutdown.issue();
    let receipt = shutdown.interaction.session.shutdown();
    assert_eq!(receipt.intent_confirmation().settled_challenges(), 1);
    assert_eq!(receipt.intent_confirmation().pending_after(), 0);
}

fn monotonic(tick: u64) -> UiHostObservationTimeBasis {
    UiHostObservationTimeBasis::HostMonotonicTick(tick)
}

fn assert_stop(
    outcome: UiIntentConfirmationContinuation,
    expected: UiIntentConfirmationStopReason,
) {
    assert_eq!(expect_stop(outcome).reason(), &expected);
}

fn expect_stop(
    outcome: UiIntentConfirmationContinuation,
) -> worth_ui::facade::intent::UiIntentConfirmationStop {
    match outcome {
        UiIntentConfirmationContinuation::Stopped(stop) => stop,
        UiIntentConfirmationContinuation::AdmissionReady(_) => {
            panic!("hostile continuation must not re-enter admission")
        }
    }
}

use worth_ui::facade::intent::{
    UiIntent, UiIntentAdmissionCancellationReason, UiIntentAdmissionDecision,
    UiIntentAdmissionStopReason, UiIntentDeclaration, UiIntentDefinition,
    UiIntentExecutionCurrentnessStop, UiIntentExecutionDispatchOutcome,
    UiIntentExecutionDispatchStopReason, UiIntentExecutionReservationDenial,
    UiIntentOperabilityOutcome, UiIntentPayloadSource, UiIntentProviderVersion,
    UiIntentRouteResolution, UiIntentRouteSource, UiIntentText,
    UI_INTENT_MAXIMUM_APPLICATION_ATTEMPTS, UI_INTENT_MAXIMUM_DESTINATION_ATTEMPTS,
    UI_INTENT_MAXIMUM_INTENT_ATTEMPTS, UI_INTENT_MAXIMUM_PROVIDER_ATTEMPTS,
    UI_INTENT_MAXIMUM_RETAINED_PAYLOAD_BYTES,
};
use worth_ui::facade::interaction::{
    UiHostInteractionIngressOutcome, UiInteractionTransition, UiSemanticInteraction,
};
use worth_ui::facade::observation_report::UiHostPointerButtonTransition;
use worth_ui_dsl::WorthUiIntentInteractionFamily;
use worth_ui_runtime::certification_support::{
    UiIntentExecutionCapacityCertificationProfile,
    WorthUiIntentExecutionReservationCertificationExt,
};

use crate::intent::admission::phase3::world::AdmissionWorld;
use crate::intent::confirmation::ConfirmationWorld;
use crate::intent::operability::PrimaryIntent;
use crate::intent::payload::{
    launch_payload_world, routed_payload_input, BudgetTextIntent, PayloadApplicationFacts,
    PayloadProjectionRegistration, BUDGET_TEXT_FIELD, DECLARATION,
};

#[test]
fn production_capacity_reserves_sixteen_and_stops_the_seventeenth_at_provider_scope() {
    let mut world = AdmissionWorld::launch(17);
    let mut admitted = (0..UI_INTENT_MAXIMUM_APPLICATION_ATTEMPTS)
        .map(|target| world.admit_exact(target))
        .collect::<Vec<_>>();
    assert_metrics(&world, 16, 0, 0);
    let UiIntentAdmissionDecision::Stopped(stop) = world.admit(16) else {
        panic!("the seventeenth provider-scoped reservation must stop")
    };
    assert_eq!(
        stop.reason(),
        &UiIntentAdmissionStopReason::ExecutionReservation(
            UiIntentExecutionReservationDenial::ProviderCapacityExceeded {
                intent: PrimaryIntent::ID,
                provider_version: UiIntentProviderVersion::stable(1),
                maximum: UI_INTENT_MAXIMUM_PROVIDER_ATTEMPTS,
            }
        )
    );
    assert_metrics(&world, 16, 0, 0);
    for handle in admitted.drain(..) {
        let _ = world.session.cancel_admitted_intent(handle);
    }
    assert_metrics(&world, 0, 0, 0);
    let retry = world.admit_exact(16);
    assert_metrics(&world, 1, 0, 0);
    let _ = world.session.cancel_admitted_intent(retry);
    assert_metrics(&world, 0, 0, 0);
    let _ = world.session.shutdown();
}

#[test]
fn each_execution_capacity_scope_denies_independently_and_releases_to_zero() {
    let full = 16;
    assert_scope_denial(
        profile(1, full, full, full, 4_096),
        UiIntentExecutionReservationDenial::ApplicationCapacityExceeded { maximum: 1 },
    );
    assert_scope_denial(
        profile(full, 1, full, full, 4_096),
        UiIntentExecutionReservationDenial::DestinationCapacityExceeded {
            destination: worth_ui::facade::intent::UiIntentExecutionDestination::ApplicationEffect,
            maximum: 1,
        },
    );
    assert_scope_denial(
        profile(full, full, 1, full, 4_096),
        UiIntentExecutionReservationDenial::ProviderCapacityExceeded {
            intent: PrimaryIntent::ID,
            provider_version: UiIntentProviderVersion::stable(1),
            maximum: 1,
        },
    );
    assert_scope_denial(
        profile(full, full, full, 1, 4_096),
        UiIntentExecutionReservationDenial::IntentCapacityExceeded {
            intent: PrimaryIntent::ID,
            maximum: 1,
        },
    );
}

#[test]
fn retained_payload_bytes_stop_before_admitted_and_exact_limit_retries_cleanly() {
    let mut world = budget_text_world();
    assert!(world
        .interaction
        .session
        .install_intent_execution_capacity_for_certification(profile(16, 16, 16, 16, 3)));
    let UiIntentAdmissionDecision::Stopped(stop) = admit_budget_text(&mut world) else {
        panic!("four admitted text bytes cannot enter a three-byte reservation budget")
    };
    assert_eq!(
        stop.reason(),
        &UiIntentAdmissionStopReason::ExecutionReservation(
            UiIntentExecutionReservationDenial::RetainedPayloadBytesExceeded {
                active: 0,
                requested: 4,
                maximum: 3,
            }
        )
    );
    assert_payload_metrics(&world.interaction.session, 0, 0, 0);
    assert!(world
        .interaction
        .session
        .install_intent_execution_capacity_for_certification(profile(16, 16, 16, 16, 4)));
    let UiIntentAdmissionDecision::Admitted(admitted) = admit_budget_text(&mut world) else {
        panic!("four admitted text bytes must fit the exact four-byte budget")
    };
    assert_payload_metrics(&world.interaction.session, 1, 0, 4);
    let _ = world.interaction.session.cancel_admitted_intent(admitted);
    assert_payload_metrics(&world.interaction.session, 0, 0, 0);
    let UiIntentAdmissionDecision::Admitted(retry) = admit_budget_text(&mut world) else {
        panic!("released retained bytes must be independently reacquirable")
    };
    let _ = world.interaction.session.cancel_admitted_intent(retry);
    assert_payload_metrics(&world.interaction.session, 0, 0, 0);
    let _ = world.interaction.session.shutdown();
}

#[test]
fn exhausted_reservation_identities_stop_without_partial_scope_commit() {
    let mut world = AdmissionWorld::launch(1);
    assert_eq!(
        world
            .session
            .exhaust_intent_execution_reservation_identities_for_certification(),
        UI_INTENT_MAXIMUM_APPLICATION_ATTEMPTS
    );
    let UiIntentAdmissionDecision::Stopped(stop) = world.admit(0) else {
        panic!("an execution ledger with no issuable generation must stop")
    };
    assert_eq!(
        stop.reason(),
        &UiIntentAdmissionStopReason::ReservationIdentityExhausted
    );
    assert_metrics(&world, 0, 0, 0);
    let _ = world.session.shutdown();
}

#[test]
fn move_only_admission_prepares_one_revalidated_framework_attempt_in_place() {
    let (mut world, provider) = AdmissionWorld::launch_with_provider_observation(1);
    let admitted = world.admit_exact(0);
    let admission_slot = admitted.slot_identity();
    let lineage = admitted.lineage();
    assert_metrics(&world, 1, 0, 0);
    let UiIntentExecutionDispatchOutcome::AttemptPrepared(receipt) = world
        .session
        .dispatch_admitted_intent(admitted, super::execution_deadline(2_000))
    else {
        panic!("one current admission must prepare one framework attempt")
    };
    assert_eq!(receipt.attempt().slot(), admission_slot.slot());
    assert_eq!(receipt.attempt().generation(), admission_slot.generation());
    assert_eq!(
        receipt.idempotency().session(),
        world.session.session_identity().as_u64()
    );
    assert_eq!(receipt.idempotency().lineage(), lineage.diagnostic_value());
    assert_eq!(receipt.deadline().tick(), 2_000);
    assert_eq!(receipt.currentness_checks(), 8);
    assert_eq!(provider.begin_calls(), 0);
    assert_metrics(&world, 1, 1, 0);
    let shutdown = world.session.shutdown();
    assert_eq!(shutdown.intent_admission().settled_attempts(), 1);
    assert_eq!(shutdown.intent_admission().active_after(), 0);
}

#[test]
fn dispatch_revalidates_policy_and_payload_inputs_before_provider_invocation() {
    let (mut policy_world, provider) = AdmissionWorld::launch_with_provider_observation(1);
    let admitted = policy_world.admit_exact(0);
    policy_world.set_policy(false);
    let UiIntentExecutionDispatchOutcome::Stopped(stop) = policy_world
        .session
        .dispatch_admitted_intent(admitted, super::execution_deadline(2_000))
    else {
        panic!("changed policy must stop before provider invocation")
    };
    assert_eq!(
        stop.reason(),
        &UiIntentExecutionDispatchStopReason::Currentness(
            UiIntentExecutionCurrentnessStop::PolicyChanged
        )
    );
    assert_eq!(stop.active_after(), 0);
    assert_eq!(provider.begin_calls(), 0);
    assert_metrics(&policy_world, 0, 0, 0);
    let _ = policy_world.session.shutdown();

    let (mut input_world, input_provider) = ConfirmationWorld::launch_with_provider_observation();
    let admitted = input_world.admit_operable();
    input_world.set_revision(9);
    let UiIntentExecutionDispatchOutcome::Stopped(stop) = input_world
        .interaction
        .session
        .dispatch_admitted_intent(admitted, super::execution_deadline(2_000))
    else {
        panic!("changed payload input must stop before provider invocation")
    };
    assert_eq!(
        stop.reason(),
        &UiIntentExecutionDispatchStopReason::Currentness(
            UiIntentExecutionCurrentnessStop::PayloadInputChanged
        )
    );
    assert_eq!(input_provider.begin_calls(), 0);
    assert_payload_metrics(&input_world.interaction.session, 0, 0, 0);
    let _ = input_world.interaction.session.shutdown();
}

#[test]
fn lifecycle_cancellation_wins_before_dispatch_and_releases_every_scope() {
    let mut world = AdmissionWorld::launch(1);
    let admitted = world.admit_exact(0);
    world.unmount(0).expect("the admitted target unmounts");
    let UiIntentExecutionDispatchOutcome::Stopped(stop) = world
        .session
        .dispatch_admitted_intent(admitted, super::execution_deadline(2_000))
    else {
        panic!("a lifecycle-cancelled admission cannot dispatch")
    };
    assert_eq!(
        stop.reason(),
        &UiIntentExecutionDispatchStopReason::AdmissionSettled(
            worth_ui::facade::intent::UiIntentAdmissionSettlementPosture::LifecycleCancelled(
                UiIntentAdmissionCancellationReason::MountedInstanceRemoved,
            )
        )
    );
    assert_metrics(&world, 0, 0, 0);
    let _ = world.session.shutdown();
}

fn assert_scope_denial(
    profile: UiIntentExecutionCapacityCertificationProfile,
    expected: UiIntentExecutionReservationDenial,
) {
    let mut world = AdmissionWorld::launch(2);
    assert!(world
        .session
        .install_intent_execution_capacity_for_certification(profile));
    let admitted = world.admit_exact(0);
    let UiIntentAdmissionDecision::Stopped(stop) = world.admit(1) else {
        panic!("the independently exhausted execution scope must stop")
    };
    assert_eq!(
        stop.reason(),
        &UiIntentAdmissionStopReason::ExecutionReservation(expected)
    );
    assert_metrics(&world, 1, 0, 0);
    let _ = world.session.cancel_admitted_intent(admitted);
    assert_metrics(&world, 0, 0, 0);
    let retry = world.admit_exact(1);
    let _ = world.session.cancel_admitted_intent(retry);
    assert_metrics(&world, 0, 0, 0);
    let _ = world.session.shutdown();
}

fn profile(
    application: usize,
    destination: usize,
    provider: usize,
    intent: usize,
    bytes: usize,
) -> UiIntentExecutionCapacityCertificationProfile {
    UiIntentExecutionCapacityCertificationProfile::bounded(
        application,
        destination,
        provider,
        intent,
        bytes,
    )
    .expect("test profile only tightens production limits")
}

fn budget_text_world() -> crate::intent::payload::PayloadWorld {
    let declaration = UiIntentDeclaration::<BudgetTextIntent>::activate(DECLARATION)
        .unwrap()
        .bind_payload(
            BUDGET_TEXT_FIELD,
            UiIntentPayloadSource::<UiIntentText>::constant("four"),
        );
    launch_payload_world::<BudgetTextIntent>(
        routed_payload_input(declaration, WorthUiIntentInteractionFamily::Activate),
        PayloadProjectionRegistration::None,
        PayloadApplicationFacts::default(),
    )
}

fn admit_budget_text(
    world: &mut crate::intent::payload::PayloadWorld,
) -> UiIntentAdmissionDecision<BudgetTextIntent> {
    let interaction = activation(&mut world.interaction);
    let route = match world
        .interaction
        .session
        .resolve_intent_route(UiIntentRouteSource::mounted_interaction(interaction))
        .unwrap()
    {
        UiIntentRouteResolution::Product(route) => route,
        UiIntentRouteResolution::Confirmation(_) => unreachable!(),
    };
    let payload = world
        .interaction
        .session
        .prepare_intent_payload(route)
        .expect("four-byte payload prepares");
    let outcome = world
        .interaction
        .session
        .evaluate_intent_operability(payload);
    let UiIntentOperabilityOutcome::Operable(_) = &outcome else {
        panic!("four-byte payload remains operable")
    };
    world.interaction.session.admit_intent(
        UiIntentDefinition::<BudgetTextIntent>::application_effect(),
        outcome,
    )
}

fn activation(
    world: &mut crate::intent::interaction_world::InteractionWorld,
) -> UiSemanticInteraction {
    let _ = world.button(90, 1, UiHostPointerButtonTransition::Pressed, [10, 20]);
    let released = world.button(90, 1, UiHostPointerButtonTransition::Released, [10, 20]);
    let UiHostInteractionIngressOutcome::Applied(receipt) = released else {
        panic!("budget payload activation reaches the interaction owner")
    };
    receipt
        .into_transitions()
        .into_vec()
        .into_iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(interaction) => Some(interaction),
            _ => None,
        })
        .expect("budget payload press/release mints one activation")
}

fn assert_metrics(world: &AdmissionWorld, active: usize, prepared: usize, bytes: usize) {
    assert_payload_metrics(&world.session, active, prepared, bytes);
}

fn assert_payload_metrics(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    active: usize,
    prepared: usize,
    bytes: usize,
) {
    let metrics = session.intent_execution_reservation_metrics_for_certification();
    assert_eq!(metrics.active_attempts(), active);
    assert_eq!(metrics.prepared_attempts(), prepared);
    assert_eq!(metrics.active_occupancy(), active);
    assert_eq!(metrics.retained_payload_bytes(), bytes);
}

#[test]
fn certification_profiles_cannot_raise_production_capacity() {
    assert!(UiIntentExecutionCapacityCertificationProfile::bounded(
        UI_INTENT_MAXIMUM_APPLICATION_ATTEMPTS + 1,
        UI_INTENT_MAXIMUM_DESTINATION_ATTEMPTS,
        UI_INTENT_MAXIMUM_PROVIDER_ATTEMPTS,
        UI_INTENT_MAXIMUM_INTENT_ATTEMPTS,
        UI_INTENT_MAXIMUM_RETAINED_PAYLOAD_BYTES,
    )
    .is_none());
}

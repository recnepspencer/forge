use worth_ui::facade::intent::{
    UiIntent, UiIntentAdmissionCancellationReason, UiIntentAdmissionCost,
    UiIntentAdmissionDecision, UiIntentAdmissionMetrics, UiIntentAdmissionSettlementPosture,
    UiIntentAdmissionStopReason, UiIntentDeclaration, UiIntentDefinition,
    UiIntentExecutionReservationDenial, UiIntentOperabilityOutcome, UiIntentPayloadSource,
    UiIntentProviderVersion, UiIntentRouteResolution, UiIntentRouteSource, UiIntentUnsigned64,
};
use worth_ui::facade::interaction::{
    UiHostInteractionIngressOutcome, UiInteractionTransition, UiSemanticInteraction,
};
use worth_ui::facade::observation_report::UiHostPointerButtonTransition;
use worth_ui_dsl::WorthUiIntentInteractionFamily;

use super::world::AdmissionWorld;
use crate::intent::confirmation::ConfirmationWorld;
use crate::intent::payload::{
    launch_payload_world, routed_payload_input, PayloadApplicationFacts,
    PayloadProjectionRegistration, WideIntent, DECLARATION, WIDE_FIELDS,
};

#[test]
fn motion_only_work_never_enters_intent_admission() {
    let mut world = AdmissionWorld::launch(1);
    assert_eq!(
        world.session.intent_admission_metrics(),
        UiIntentAdmissionMetrics::default()
    );
    world.motion_storm(0, 16);
    assert_eq!(
        world.session.intent_admission_metrics(),
        UiIntentAdmissionMetrics::default()
    );
    let _ = world.session.shutdown();
}

#[test]
fn one_real_semantic_activation_admits_and_releases_one_effect_free_attempt() {
    let mut world = AdmissionWorld::launch(1);
    let UiIntentAdmissionDecision::Admitted(admitted) = world.admit(0) else {
        panic!("one current, operable activation must admit")
    };
    assert_empty_admission_cost(admitted.cost(), 1, 16);

    let metrics = world.session.intent_admission_metrics();
    assert_eq!(metrics.active_attempts(), 1);
    assert_eq!(metrics.active_occupancy(), 1);
    assert_eq!(metrics.retained_candidates(), 1);
    assert_eq!(metrics.retained_payloads(), 1);
    let receipt = world.session.cancel_admitted_intent(admitted);
    assert_eq!(
        receipt.posture(),
        UiIntentAdmissionSettlementPosture::Released
    );
    assert_eq!(receipt.active_after(), 0);
    assert_zero_current_census(world.session.intent_admission_metrics());
    let _ = world.session.shutdown();
}

#[test]
fn bounded_attempts_cover_capacity_unmount_retry_repeated_cleanup_and_shutdown() {
    let mut world = AdmissionWorld::launch(17);
    assert_zero_current_census(world.session.intent_admission_metrics());
    let mut admitted = Vec::new();
    for target in 0..15 {
        let handle = world.admit_exact(target);
        assert_empty_admission_cost(handle.cost(), target + 1, 16);
        admitted.push(Some(handle));
    }
    assert_active_census(world.session.intent_admission_metrics(), 15, 5);
    let sixteenth = world.admit_exact(15);
    assert_empty_admission_cost(sixteenth.cost(), 16, 16);
    admitted.push(Some(sixteenth));
    assert_active_census(world.session.intent_admission_metrics(), 16, 5);

    let UiIntentAdmissionDecision::Stopped(full) = world.admit(16) else {
        panic!("the seventeenth attempt must stop at the admission authority")
    };
    assert_eq!(
        full.reason(),
        &UiIntentAdmissionStopReason::ExecutionReservation(
            UiIntentExecutionReservationDenial::ProviderCapacityExceeded {
                intent: crate::intent::operability::PrimaryIntent::ID,
                provider_version: UiIntentProviderVersion::stable(1),
                maximum: 16,
            }
        )
    );
    assert_empty_admission_cost(full.cost(), 16, 0);

    world.unmount(0).expect("the first exact target unmounts");
    let metrics_after_unmount = world.session.intent_admission_metrics();
    assert_active_census(metrics_after_unmount, 15, 5);
    assert!(world.unmount(0).is_err());
    assert_eq!(
        world.session.intent_admission_metrics(),
        metrics_after_unmount
    );
    let retired = world
        .session
        .cancel_admitted_intent(admitted[0].take().unwrap());
    assert_eq!(
        retired.posture(),
        UiIntentAdmissionSettlementPosture::LifecycleCancelled(
            UiIntentAdmissionCancellationReason::MountedInstanceRemoved
        )
    );
    assert_eq!(retired.active_after(), 15);

    for handle in admitted.into_iter().skip(1).flatten() {
        assert_eq!(
            world.session.cancel_admitted_intent(handle).posture(),
            UiIntentAdmissionSettlementPosture::Released
        );
    }
    assert_zero_current_census(world.session.intent_admission_metrics());
    let retry = world.admit_exact(16);
    assert_active_census(world.session.intent_admission_metrics(), 1, 5);
    let shutdown = world.session.shutdown();
    assert_eq!(shutdown.intent_admission().settled_attempts(), 1);
    assert_eq!(shutdown.intent_admission().active_after(), 0);
    assert_eq!(shutdown.intent_admission().retained_candidates_after(), 0);
    assert_eq!(shutdown.intent_admission().retained_payloads_after(), 0);
    drop(retry);
}

#[test]
fn denial_releases_every_candidate_and_retry_starts_from_zero() {
    let mut world = AdmissionWorld::launch(1);
    world.set_policy(false);
    let UiIntentAdmissionDecision::Stopped(denied) = world.admit(0) else {
        panic!("policy denial must stop before admission")
    };
    assert!(matches!(
        denied.reason(),
        UiIntentAdmissionStopReason::Inoperable(_)
    ));
    assert_zero_current_census(world.session.intent_admission_metrics());
    world.set_policy(true);
    let retry = world.admit_exact(0);
    assert_active_census(world.session.intent_admission_metrics(), 1, 5);
    let receipt = world.session.cancel_admitted_intent(retry);
    assert_eq!(
        receipt.posture(),
        UiIntentAdmissionSettlementPosture::Released
    );
    assert_zero_current_census(world.session.intent_admission_metrics());
    let _ = world.session.shutdown();
}

#[test]
fn admitted_cost_carries_one_declared_payload_field() {
    let mut world = ConfirmationWorld::launch();
    let admitted = world.admit_operable();
    assert_active_census(world.interaction.session.intent_admission_metrics(), 1, 6);
    assert_eq!(admitted.cost().payload_projection().declared_fields(), 1);
    assert_eq!(
        world
            .interaction
            .session
            .cancel_admitted_intent(admitted)
            .posture(),
        UiIntentAdmissionSettlementPosture::Released
    );
    assert_zero_current_census(world.interaction.session.intent_admission_metrics());
    let _ = world.interaction.session.shutdown();
}

#[test]
fn application_replacement_cancels_predecessor_and_fresh_retry_admits() {
    let mut world = ConfirmationWorld::launch();
    let predecessor = world.admit_operable();
    assert_active_census(world.interaction.session.intent_admission_metrics(), 1, 6);
    world.rebind_application();
    assert_zero_current_census(world.interaction.session.intent_admission_metrics());
    let retired = world
        .interaction
        .session
        .cancel_admitted_intent(predecessor);
    assert_eq!(
        retired.posture(),
        UiIntentAdmissionSettlementPosture::LifecycleCancelled(
            UiIntentAdmissionCancellationReason::ApplicationRebound
        )
    );
    world.publish_successor();
    let successor = world.admit_operable();
    assert_eq!(
        world
            .interaction
            .session
            .cancel_admitted_intent(successor)
            .posture(),
        UiIntentAdmissionSettlementPosture::Released
    );
    assert_zero_current_census(world.interaction.session.intent_admission_metrics());
    let _ = world.interaction.session.shutdown();
}

#[test]
fn admitted_cost_carries_the_exact_sixty_four_field_projection() {
    let mut declaration = UiIntentDeclaration::<WideIntent>::activate(DECLARATION).unwrap();
    for (index, field) in WIDE_FIELDS.into_iter().enumerate() {
        declaration = declaration.bind_payload(
            field,
            UiIntentPayloadSource::<UiIntentUnsigned64>::constant(index as u64),
        );
    }
    let mut world = launch_payload_world::<WideIntent>(
        routed_payload_input(declaration, WorthUiIntentInteractionFamily::Activate),
        PayloadProjectionRegistration::None,
        PayloadApplicationFacts::default(),
    );
    let interaction = payload_activation(&mut world.interaction);
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
        .expect("the contractual maximum payload projects");
    let outcome = world
        .interaction
        .session
        .evaluate_intent_operability(payload);
    let UiIntentOperabilityOutcome::Operable(_) = &outcome else {
        panic!("the constant wide payload remains operable")
    };
    let UiIntentAdmissionDecision::Admitted(admitted) = world.interaction.session.admit_intent(
        UiIntentDefinition::<WideIntent>::application_effect(),
        outcome,
    ) else {
        panic!("the maximum-width candidate must admit")
    };
    let cost = admitted.cost().payload_projection();
    assert_eq!(cost.declared_fields(), 64);
    assert_eq!(cost.query_inputs_read(), 0);
    assert_eq!(cost.application_inputs_read(), 0);
    assert_eq!(cost.admitted_utf8_bytes(), 0);
    assert_eq!(
        world
            .interaction
            .session
            .cancel_admitted_intent(admitted)
            .posture(),
        UiIntentAdmissionSettlementPosture::Released
    );
    assert_zero_current_census(world.interaction.session.intent_admission_metrics());
    let _ = world.interaction.session.shutdown();
}

fn payload_activation(
    world: &mut crate::intent::interaction_world::InteractionWorld,
) -> UiSemanticInteraction {
    let _ = world.button(1, 1, UiHostPointerButtonTransition::Pressed, [10, 20]);
    let released = world.button(1, 1, UiHostPointerButtonTransition::Released, [10, 20]);
    let UiHostInteractionIngressOutcome::Applied(receipt) = released else {
        panic!("wide payload activation reaches the production interaction owner")
    };
    receipt
        .into_transitions()
        .into_vec()
        .into_iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(interaction) => Some(interaction),
            _ => None,
        })
        .expect("wide payload press/release mints one semantic activation")
}

fn assert_active_census(
    metrics: UiIntentAdmissionMetrics,
    expected: usize,
    owner_references_per_attempt: usize,
) {
    assert_eq!(metrics.active_attempts(), expected);
    assert_eq!(metrics.active_occupancy(), expected);
    assert_eq!(metrics.retained_candidates(), expected);
    assert_eq!(metrics.retained_payloads(), expected);
    assert_eq!(
        metrics.retained_owner_references(),
        expected * owner_references_per_attempt
    );
}

fn assert_empty_admission_cost(
    cost: UiIntentAdmissionCost,
    expected_attempt_slots: usize,
    expected_occupancy_slots: usize,
) {
    assert_eq!(cost.payload_projection().declared_fields(), 0);
    assert_eq!(cost.payload_projection().query_inputs_read(), 0);
    assert_eq!(cost.payload_projection().application_inputs_read(), 0);
    assert_eq!(cost.payload_projection().admitted_utf8_bytes(), 0);
    assert_eq!(cost.operability_dependencies_visited(), 7);
    assert_eq!(cost.currentness_checks(), 9);
    assert_eq!(cost.occupancy_slots_inspected(), expected_occupancy_slots);
    assert_eq!(cost.slots_inspected(), expected_attempt_slots);
}

fn assert_zero_current_census(metrics: UiIntentAdmissionMetrics) {
    assert_eq!(metrics.active_attempts(), 0);
    assert_eq!(metrics.active_occupancy(), 0);
    assert_eq!(metrics.retained_candidates(), 0);
    assert_eq!(metrics.retained_payloads(), 0);
    assert_eq!(metrics.retained_owner_references(), 0);
}

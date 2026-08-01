use worth_ui::facade::intent::{
    UiAdmittedIntent, UiIntentAdmissionDecision, UiIntentDeclaration, UiIntentDefinition,
    UiIntentOperabilityOutcome, UiIntentPayloadProjectionCost, UiIntentPayloadSource,
    UiIntentRouteResolution, UiIntentRouteSource, UiIntentUnsigned64,
};
use worth_ui::facade::interaction::{
    UiHostInteractionIngressOutcome, UiInteractionTransition, UiSemanticInteraction,
};
use worth_ui::facade::observation_report::UiHostPointerButtonTransition;
use worth_ui_dsl::WorthUiIntentInteractionFamily;

use crate::intent::admission::phase3::world::AdmissionWorld;
use crate::intent::confirmation::ConfirmationWorld;
use crate::intent::payload::{
    launch_payload_world, routed_payload_input, PayloadApplicationFacts,
    PayloadProjectionRegistration, WideIntent, DECLARATION, WIDE_FIELDS,
};

#[test]
fn payload_projection_cost_follows_zero_one_and_sixty_four_declared_fields() {
    assert_zero_width();
    assert_one_width();
    assert_sixty_four_width();
}

fn assert_zero_width() {
    let mut world = AdmissionWorld::launch(1);
    let admitted = world.admit_exact(0);
    assert_projection(admitted.cost().payload_projection(), 0, 0);
    let _ = world.session.cancel_admitted_intent(admitted);
    let _ = world.session.shutdown();
}

fn assert_one_width() {
    let mut world = ConfirmationWorld::launch();
    let admitted = world.admit_operable();
    assert_projection(admitted.cost().payload_projection(), 1, 1);
    let _ = world.interaction.session.cancel_admitted_intent(admitted);
    let _ = world.interaction.session.shutdown();
}

fn assert_sixty_four_width() {
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
    let admitted = admit_wide(&mut world.interaction);
    assert_projection(admitted.cost().payload_projection(), 64, 0);
    let _ = world.interaction.session.cancel_admitted_intent(admitted);
    let _ = world.interaction.session.shutdown();
}

fn admit_wide(
    world: &mut crate::intent::interaction_world::InteractionWorld,
) -> UiAdmittedIntent<WideIntent> {
    let interaction = activation(world);
    let route = match world
        .session
        .resolve_intent_route(UiIntentRouteSource::mounted_interaction(interaction))
        .expect("wide-payload route resolves")
    {
        UiIntentRouteResolution::Product(route) => route,
        UiIntentRouteResolution::Confirmation(_) => unreachable!(),
    };
    let payload = world
        .session
        .prepare_intent_payload(route)
        .expect("sixty-four-field payload projects");
    let outcome = world.session.evaluate_intent_operability(payload);
    let UiIntentOperabilityOutcome::Operable(_) = &outcome else {
        panic!("constant wide payload remains operable")
    };
    match world.session.admit_intent(
        UiIntentDefinition::<WideIntent>::application_effect(),
        outcome,
    ) {
        UiIntentAdmissionDecision::Admitted(admitted) => admitted,
        _ => panic!("sixty-four-field payload admits"),
    }
}

fn activation(
    world: &mut crate::intent::interaction_world::InteractionWorld,
) -> UiSemanticInteraction {
    let _ = world.button(1, 1, UiHostPointerButtonTransition::Pressed, [10, 20]);
    let released = world.button(1, 1, UiHostPointerButtonTransition::Released, [10, 20]);
    let UiHostInteractionIngressOutcome::Applied(receipt) = released else {
        panic!("wide-payload release reaches the interaction owner")
    };
    receipt
        .into_transitions()
        .into_vec()
        .into_iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(interaction) => Some(interaction),
            _ => None,
        })
        .expect("wide-payload press/release mints one activation")
}

fn assert_projection(
    cost: UiIntentPayloadProjectionCost,
    declared_fields: usize,
    application_inputs: usize,
) {
    assert_eq!(cost.declared_fields(), declared_fields);
    assert_eq!(cost.query_inputs_read(), 0);
    assert_eq!(cost.application_inputs_read(), application_inputs);
    assert_eq!(cost.admitted_utf8_bytes(), 0);
}

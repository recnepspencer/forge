use worth_ui::facade::intent::{
    UiIntentAdmissionDecision, UiIntentApplicationFact, UiIntentDeclaration, UiIntentDefinition,
    UiIntentPayloadSource, UiIntentRouteResolution, UiIntentRouteSource, UiIntentText,
};
use worth_ui::facade::interaction::{
    UiHostInteractionIngressOutcome, UiInteractionTransition, UiSemanticInteraction,
};
use worth_ui::facade::observation_report::UiHostPointerButtonTransition;
use worth_ui_dsl::WorthUiIntentInteractionFamily;

use super::{assert_evidence_count, assert_only_evidence, assert_retirement, census};
use crate::intent::payload::{
    launch_payload_world, routed_payload_input, BudgetTextIntent, PayloadApplicationFacts,
    PayloadProjectionRegistration, BUDGET_TEXT_FIELD, DECLARATION,
};

#[test]
fn nonempty_admission_payload_bytes_retire_exactly() {
    let fact = UiIntentApplicationFact::<UiIntentText>::text("phase314.lifecycle.payload", 4)
        .expect("lifecycle payload fact has an exact byte budget");
    let declaration = UiIntentDeclaration::<BudgetTextIntent>::activate(DECLARATION)
        .expect("lifecycle payload declaration is valid")
        .bind_payload(
            BUDGET_TEXT_FIELD,
            UiIntentPayloadSource::<UiIntentText>::application_fact(&fact),
        );
    let mut world = launch_payload_world::<BudgetTextIntent>(
        routed_payload_input(declaration, WorthUiIntentInteractionFamily::Activate),
        PayloadProjectionRegistration::None,
        PayloadApplicationFacts::text(fact, "rust"),
    );
    let interaction = activation(&mut world.interaction);
    let route = resolve_route(&world.interaction, interaction);
    let payload = world
        .interaction
        .session
        .prepare_intent_payload(route)
        .expect("the exact four-byte payload prepares");
    assert_eq!(payload.input_basis().cost().admitted_utf8_bytes(), 4);
    let operability = world
        .interaction
        .session
        .evaluate_intent_operability(payload);
    let UiIntentAdmissionDecision::Admitted(admitted) = world.interaction.session.admit_intent(
        UiIntentDefinition::<BudgetTextIntent>::application_effect(),
        operability,
    ) else {
        panic!("the current typed payload must admit")
    };
    let active = census(&world.interaction.session);
    assert_eq!(active.retained_payloads(), 1);
    assert_eq!(active.retained_payload_bytes(), 4);
    assert_evidence_count(active, 1);

    let settlement = world.interaction.session.cancel_admitted_intent(admitted);
    assert_eq!(settlement.active_after(), 0);
    assert_only_evidence(census(&world.interaction.session), 1);
    let shutdown = world.interaction.session.shutdown();
    assert_retirement(
        shutdown.intent_evidence(),
        worth_ui_inspection::UiIntentEvidenceRetirementCause::ApplicationShutdown,
        1,
    );
}

fn activation(
    world: &mut crate::intent::interaction_world::InteractionWorld,
) -> UiSemanticInteraction {
    let _ = world.button(1, 1, UiHostPointerButtonTransition::Pressed, [10, 20]);
    let released = world.button(1, 1, UiHostPointerButtonTransition::Released, [10, 20]);
    let UiHostInteractionIngressOutcome::Applied(receipt) = released else {
        panic!("release reaches the production interaction owner")
    };
    receipt
        .into_transitions()
        .into_vec()
        .into_iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(interaction) => Some(interaction),
            _ => None,
        })
        .expect("one press and release mint a semantic activation")
}

fn resolve_route(
    world: &crate::intent::interaction_world::InteractionWorld,
    interaction: UiSemanticInteraction,
) -> worth_ui::facade::intent::UiResolvedProductIntentRoute {
    match world
        .session
        .resolve_intent_route(UiIntentRouteSource::mounted_interaction(interaction))
        .expect("the mounted interaction resolves its typed route")
    {
        UiIntentRouteResolution::Product(route) => route,
        UiIntentRouteResolution::Confirmation(_) => {
            panic!("the payload route cannot cross into confirmation")
        }
    }
}

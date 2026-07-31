use worth_ui::facade::intent::{
    UiIntentRouteResolution, UiIntentRouteResolutionStop, UiIntentRouteSource,
};
use worth_ui::facade::interaction::{
    UiHostInteractionIngressOutcome, UiInteractionTransition, UiSemanticInteraction,
};
use worth_ui::facade::observation_report::UiHostPointerButtonTransition;
use worth_ui_test_support::WorthUiMountedInteractionLifecycleCertificationExt;

use super::support::{
    confirmation_file_world, confirmation_rust_world, file_world, rust_world, DECLARATION,
    DEFINITION,
};

#[test]
fn real_mounted_activation_resolves_the_same_product_route_for_both_authorships() {
    for mut world in [file_world(), rust_world()] {
        let interaction = activation(&mut world, [20, 20]);
        let resolved = world
            .session
            .resolve_intent_route(UiIntentRouteSource::mounted_interaction(interaction))
            .expect("current mounted activation should resolve");
        match resolved {
            UiIntentRouteResolution::Product(route) => {
                assert_eq!(route.declaration_identity(), DECLARATION);
                assert_eq!(route.definition_id().as_str(), DEFINITION);
                assert_eq!(
                    route.interaction(),
                    worth_ui::facade::intent::UiSemanticInteractionFamily::Activate
                );
            }
            UiIntentRouteResolution::Confirmation(_) => {
                panic!("product control cannot resolve as confirmation")
            }
        }
        let _ = world.session.shutdown();
    }
}

#[test]
fn product_and_confirmation_controls_resolve_to_distinct_typed_routes() {
    for mut world in [confirmation_file_world(), confirmation_rust_world()] {
        let interaction = activation(&mut world, [10, 20]);
        assert_eq!(
            interaction.target().hit_test_order(),
            1,
            "the independent geometry oracle selects the hit-only confirmation control"
        );
        let resolved = world
            .session
            .resolve_intent_route(UiIntentRouteSource::mounted_interaction(interaction))
            .expect("current mounted confirmation activation should resolve");
        match resolved {
            UiIntentRouteResolution::Product(_) => {
                panic!("confirmation control cannot resolve as a product route")
            }
            UiIntentRouteResolution::Confirmation(route) => {
                assert_eq!(route.declaration_identity(), DECLARATION);
                assert_eq!(route.definition_id().as_str(), DEFINITION);
            }
        }
        let _ = world.session.shutdown();
    }
}

#[test]
fn unmounted_interaction_cannot_retarget_through_the_catalog() {
    let mut world = file_world();
    let interaction = activation(&mut world, [20, 20]);
    let mounted_instance = interaction.target().mounted_instance();
    world
        .session
        .unmount_instance_with_interaction_receipt(mounted_instance)
        .expect("the exact mounted target should unmount");

    let stop = match world
        .session
        .resolve_intent_route(UiIntentRouteSource::mounted_interaction(interaction))
    {
        Ok(_) => panic!("a retired mounted incarnation cannot resolve"),
        Err(stop) => stop,
    };
    assert_eq!(
        stop,
        UiIntentRouteResolutionStop::Targeting(
            worth_ui::facade::interaction::UiInteractionTargetingDenial::
                MountedInstanceNoLongerCurrent,
        )
    );
    let _ = world.session.shutdown();
}

fn activation(
    world: &mut super::super::interaction_world::InteractionWorld,
    point: [i64; 2],
) -> UiSemanticInteraction {
    let _ = world.button(1, 1, UiHostPointerButtonTransition::Pressed, point);
    let released = world.button(1, 1, UiHostPointerButtonTransition::Released, point);
    let UiHostInteractionIngressOutcome::Applied(receipt) = released else {
        panic!("release should enter the production interaction runtime");
    };
    receipt
        .into_transitions()
        .into_vec()
        .into_iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(interaction) => Some(interaction),
            _ => None,
        })
        .expect("press/release should issue one semantic interaction")
}

use worth_query::facade::runtime::WorthQueryWorkspace;
use worth_ui::facade::intent::{
    UiIntentDeclaration, UiIntentInputOwnerRevision, UiIntentPayloadSource, UiIntentSelection,
};
use worth_ui::facade::interaction::{UiSelectionCommitStopReason, UiSemanticInteraction};
use worth_ui_dsl::WorthUiIntentInteractionFamily;
use worth_ui_query_binding::{
    UiCollectionProjectionBindingAdmission, UiCollectionProjectionBudget,
    UiCollectionProjectionFactReceipt, UiCollectionProjectionOpenOutcome,
    UiCollectionProjectionRegistration, UiLiveCollectionProjection,
    UiLiveCollectionProjectionCloseOutcome, UiPresentProjection, UiProjectionAvailability,
    UiProjectionFieldRequirement, UiProjectionInputFactReference, UiProjectionObservation,
    WorthUiQueryWorkspaceExt,
};

use super::super::payload_types::{SelectionIntent, SELECTION_FIELD};
use super::super::world::{
    launch, routed_input, PayloadApplicationFacts, PayloadProjectionRegistration, PayloadWorld,
    DECLARATION,
};

#[test]
fn ia_05_selection_uses_exact_query_identity_and_rejects_the_stale_reorder_revision() {
    let (mut query, entities) =
        worth_ui_query_binding::certification::seeded_collection_projection_workspace(
            vec![
                ("pulse.alpha".to_owned(), "Alpha".to_owned()),
                ("pulse.bravo".to_owned(), "Bravo".to_owned()),
            ],
            worth_ui_query_binding::certification::WorthUiCollectionProjectionSeedPosture::Complete,
        );
    let registration = collection_registration(&query);
    let (mut live, snapshot) = open_collection(&registration, &mut query);
    let UiProjectionAvailability::Present(UiPresentProjection::Current(snapshot_value)) =
        snapshot.availability()
    else {
        panic!("the canonical Query snapshot is current before WUI launch")
    };
    assert_eq!(snapshot_value.rows().len(), 2);
    let projection = registration.view().identity().clone();
    let declaration = UiIntentDeclaration::<SelectionIntent>::selection_commit(DECLARATION)
        .unwrap()
        .bind_payload(
            SELECTION_FIELD,
            UiIntentPayloadSource::<UiIntentSelection>::projection(&projection),
        );
    let mut world = launch::<SelectionIntent>(
        routed_input(declaration, WorthUiIntentInteractionFamily::SelectionCommit),
        PayloadProjectionRegistration::Collection(registration.clone()),
        PayloadApplicationFacts::default(),
    );
    let slot = world
        .projection_slot
        .expect("the frozen plan assigns one compact collection slot");
    let bravo_entity_identity = entities[1].evidence_identity();
    let bravo_identity = bravo_entity_identity.terminal_projection_for_reporting();
    let original_option = option_for_identity(&snapshot, slot, bravo_identity);
    assert_eq!(original_option.identity_for_reporting(), bravo_identity);
    publish_collection(&mut world, snapshot, 314_051);
    assert_selection_payload(&mut world, original_option.clone());

    worth_ui_query_binding::certification::update_projection_identity(
        &mut query,
        entities[1].clone(),
        "pulse.00-bravo",
    );
    let reordered = refresh_collection(&mut live, &mut query);
    let current_option = option_for_identity(&reordered, slot, bravo_identity);
    assert_eq!(
        current_option.identity_for_reporting(),
        original_option.identity_for_reporting()
    );
    assert_ne!(
        current_option.owner_revision(),
        original_option.owner_revision()
    );
    publish_collection(&mut world, reordered, 314_052);

    let UiSemanticInteraction::Activate(stale_activation) = super::activation(&mut world, [10, 20])
    else {
        panic!("current target produces activation")
    };
    let stale = world
        .interaction
        .session
        .commit_selection_interaction(stale_activation, original_option)
        .expect_err("the pre-reorder owner revision must stop");
    assert_eq!(
        stale.reason(),
        UiSelectionCommitStopReason::ProjectionRevisionChanged
    );
    assert_selection_payload(&mut world, current_option);

    match live.close(&mut query) {
        UiLiveCollectionProjectionCloseOutcome::Closed(closed) => {
            assert!(closed.owner_terminal())
        }
        UiLiveCollectionProjectionCloseOutcome::Stopped(stop) => {
            panic!("the exact Query owner closes: {:?}", stop.query_error())
        }
    }
    let _ = world.interaction.session.shutdown();
}

fn assert_selection_payload(
    world: &mut PayloadWorld,
    option: worth_ui_query_binding::UiProjectionOptionReference,
) {
    let expected_option = option.clone();
    let UiSemanticInteraction::Activate(activation) = super::activation(world, [10, 20]) else {
        panic!("current target produces activation")
    };
    let selection = world
        .interaction
        .session
        .commit_selection_interaction(activation, option)
        .expect("the exact current option becomes a selection interaction");
    assert_eq!(selection.option(), &expected_option);
    let route = super::product_route(
        &world.interaction,
        UiSemanticInteraction::SelectionCommit(selection),
    );
    let prepared = world
        .interaction
        .session
        .prepare_intent_payload(route)
        .expect("current option identity reaches the typed payload");
    assert_eq!(prepared.input_basis().cost().declared_fields(), 1);
    assert_eq!(prepared.input_basis().cost().query_inputs_read(), 1);
    let [UiIntentInputOwnerRevision::Query(revision)] = prepared.input_basis().owner_revisions()
    else {
        panic!("the selected option retains one exact Query owner revision")
    };
    assert_eq!(revision.field(), SELECTION_FIELD.descriptor());
    assert_eq!(revision.revision(), expected_option.owner_revision());
}

fn collection_registration(query: &WorthQueryWorkspace) -> UiCollectionProjectionRegistration {
    let domain = query.worth_ui().expect("Worth UI Query domain installed");
    UiCollectionProjectionRegistration::text(
        domain
            .projection_view("platform.pulse.status")
            .expect("collection fixture view is installed"),
        UiProjectionFieldRequirement::declared("identity.id").unwrap(),
        [UiProjectionFieldRequirement::declared("status").unwrap()],
        false,
        false,
    )
    .expect("identity-preserving collection registration")
}

fn open_collection(
    registration: &UiCollectionProjectionRegistration,
    query: &mut WorthQueryWorkspace,
) -> (
    UiLiveCollectionProjection,
    UiCollectionProjectionFactReceipt,
) {
    let UiCollectionProjectionBindingAdmission::Ready(binding) = registration.clone().admit(query)
    else {
        panic!("real collection binding admits")
    };
    let budget = UiCollectionProjectionBudget::new(2, 2, 0, 1024).unwrap();
    let UiCollectionProjectionOpenOutcome::Opened(opened) = binding.open(budget, query) else {
        panic!("real collection projection opens")
    };
    opened.into_parts()
}

fn refresh_collection(
    live: &mut UiLiveCollectionProjection,
    query: &mut WorthQueryWorkspace,
) -> UiCollectionProjectionFactReceipt {
    match live.refresh(query).expect("real Query refresh completes") {
        worth_ui_query_binding::UiCollectionProjectionRefreshOutcome::Applied(receipt) => {
            receipt.into_fact()
        }
        worth_ui_query_binding::UiCollectionProjectionRefreshOutcome::NoSemanticDelivery => {
            panic!("reorder changes the collection revision")
        }
    }
}

fn option_for_identity(
    fact: &UiCollectionProjectionFactReceipt,
    slot: worth_ui_query_binding::UiProjectionInputSlot,
    identity: &str,
) -> worth_ui_query_binding::UiProjectionOptionReference {
    let UiProjectionInputFactReference::Collection(input) = fact.intent_input_reference(slot)
    else {
        unreachable!("a collection fact produces collection input")
    };
    let observed = input
        .rows()
        .iter()
        .find(|row| row.option().identity_for_reporting() == identity)
        .map(|row| row.option().clone());
    observed.unwrap_or_else(|| {
        panic!(
            "Query-issued entity identity missing: expected={identity:?}, availability={:?}, rows={:?}",
            fact.availability(),
            input.rows(),
        )
    })
}

fn publish_collection(
    world: &mut PayloadWorld,
    fact: UiCollectionProjectionFactReceipt,
    request: u64,
) {
    super::publish_projection(
        world,
        UiProjectionObservation::Collection(fact.into_observation()),
        request,
    );
    world.interaction.publish_successor();
}

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
    let (mut scenario, original_option) = SelectionScenario::launch();
    let original_revision = original_option.owner_revision().clone();
    let registration = scenario.registration.clone();
    let projection = scenario.projection.clone();
    assert_foreign_selection_stops(
        &mut scenario.world,
        &registration,
        &projection,
        original_option.clone(),
    );
    let original_prepared = assert_selection_payload(&mut scenario.world, original_option.clone());
    let original_generation = original_prepared.input_basis().generation().clone();
    let original_frame = original_prepared.input_basis().publication_frame();
    let current_option = scenario.reorder(&original_option);
    assert_stale_selection_stops(&mut scenario.world, original_option);
    let current_revision = current_option.owner_revision().clone();
    let current_prepared = assert_selection_payload(&mut scenario.world, current_option);
    assert_ne!(
        current_prepared.input_basis().publication_frame(),
        original_frame
    );
    assert_eq!(
        current_prepared.input_basis().generation(),
        &original_generation
    );
    assert_eq!(
        original_prepared.input_basis().generation(),
        &original_generation
    );
    assert_eq!(
        original_prepared.input_basis().publication_frame(),
        original_frame
    );
    assert_ne!(original_revision, current_revision);
    assert_query_revision(&original_prepared, &original_revision);
    assert_query_revision(&current_prepared, &current_revision);
    assert_query_revision(&original_prepared, &original_revision);
    scenario.close();
}

struct SelectionScenario {
    query: WorthQueryWorkspace,
    entity: worth_query::facade::foundation::WorthQueryEntityIdentity,
    registration: UiCollectionProjectionRegistration,
    live: UiLiveCollectionProjection,
    world: PayloadWorld,
    projection: worth_ui_query_binding::WorthUiQueryViewIdentity,
    row: worth_ui_query_binding::UiCollectionProjectionRowReference,
    snapshot_input: UiProjectionInputFactReference,
}

impl SelectionScenario {
    fn launch() -> (Self, worth_ui_query_binding::UiProjectionOptionReference) {
        let (mut query, entities) =
            worth_ui_query_binding::certification::seeded_collection_projection_workspace(
                vec![
                    ("pulse.alpha".to_owned(), "Alpha".to_owned()),
                    ("pulse.bravo".to_owned(), "Bravo".to_owned()),
                ],
                worth_ui_query_binding::certification::WorthUiCollectionProjectionSeedPosture::
                    Complete,
            );
        let registration = collection_registration(&query);
        let (live, snapshot) = open_collection(&registration, &mut query);
        let UiProjectionAvailability::Present(UiPresentProjection::Current(snapshot_value)) =
            snapshot.availability()
        else {
            panic!("the canonical Query snapshot is current before WUI launch")
        };
        assert_eq!(snapshot_value.rows().len(), 2);
        let entity = entities[1].clone();
        let entity_evidence = entity.evidence_identity();
        let expected = entity_evidence.operational_key().correlation_digest();
        let row = snapshot_value
            .rows()
            .iter()
            .find(|row| row.row().identity().host_correlation_digest() == expected)
            .expect("the Query snapshot contains the independently seeded entity")
            .row()
            .clone();
        let projection = registration.view().identity().clone();
        let mut world = launch::<SelectionIntent>(
            selection_route_input(&projection),
            PayloadProjectionRegistration::Collection(registration.clone()),
            PayloadApplicationFacts::default(),
        );
        let slot = world
            .projection_slot
            .expect("the frozen plan assigns one compact collection slot");
        let snapshot_input = snapshot.intent_input_transition(slot).apply(None);
        publish_collection(&mut world, snapshot, 314_051);
        let original = world
            .interaction
            .session
            .current_projection_option(&projection, &row)
            .expect("the mounted snapshot exposes the exact current Query option");
        assert_eq!(original.identity().host_correlation_digest(), expected);
        (
            Self {
                query,
                entity,
                registration,
                live,
                world,
                projection,
                row,
                snapshot_input,
            },
            original,
        )
    }

    fn reorder(
        &mut self,
        original: &worth_ui_query_binding::UiProjectionOptionReference,
    ) -> worth_ui_query_binding::UiProjectionOptionReference {
        worth_ui_query_binding::certification::update_projection_identity(
            &mut self.query,
            self.entity.clone(),
            "pulse.00-bravo",
        );
        let reordered = refresh_collection(&mut self.live, &mut self.query);
        assert_move_only_patch(&reordered, &self.snapshot_input);
        publish_collection(&mut self.world, reordered, 314_052);
        let current = self
            .world
            .interaction
            .session
            .current_projection_option(&self.projection, &self.row)
            .expect("the mounted move patch retains the exact Query row");
        assert_eq!(current.identity(), original.identity());
        assert_ne!(current.owner_revision(), original.owner_revision());
        current
    }

    fn close(mut self) {
        match self.live.close(&mut self.query) {
            UiLiveCollectionProjectionCloseOutcome::Closed(closed) => {
                assert!(closed.owner_terminal())
            }
            UiLiveCollectionProjectionCloseOutcome::Stopped(stop) => {
                panic!("the exact Query owner closes: {:?}", stop.query_error())
            }
        }
        let _ = self.world.interaction.session.shutdown();
    }
}

fn assert_move_only_patch(
    reordered: &UiCollectionProjectionFactReceipt,
    predecessor: &UiProjectionInputFactReference,
) {
    assert_eq!(reordered.changes().len(), 2);
    assert!(reordered.changes().iter().all(|change| matches!(
        change,
        worth_ui_query_binding::UiCollectionProjectionChange::Move { .. }
    )));
    let UiProjectionAvailability::Present(UiPresentProjection::Current(value)) =
        reordered.availability()
    else {
        panic!("the Query reorder remains current")
    };
    assert!(value.rows().is_empty());
    let input = reordered
        .intent_input_transition(predecessor.revision().slot())
        .apply(Some(predecessor));
    let UiProjectionInputFactReference::Collection(collection) = input else {
        unreachable!("a collection patch preserves collection input shape")
    };
    assert_eq!(collection.row_count(), 2);
    assert_eq!(collection.transition_work().replaced_rows(), 0);
    assert_eq!(collection.transition_work().change_operations(), 2);
    assert_eq!(collection.transition_work().key_probes(), 3);
    assert_eq!(collection.transition_work().node_copies(), 0);
}

fn assert_stale_selection_stops(
    world: &mut PayloadWorld,
    option: worth_ui_query_binding::UiProjectionOptionReference,
) {
    let UiSemanticInteraction::Activate(activation) = super::activation(world, [10, 20]) else {
        panic!("current target produces activation")
    };
    let stale = world
        .interaction
        .session
        .commit_selection_interaction(activation, option)
        .expect_err("the pre-reorder owner revision must stop");
    assert_eq!(
        stale.reason(),
        UiSelectionCommitStopReason::ProjectionRevisionChanged
    );
}

fn assert_foreign_selection_stops(
    source: &mut PayloadWorld,
    registration: &UiCollectionProjectionRegistration,
    projection: &worth_ui_query_binding::WorthUiQueryViewIdentity,
    option: worth_ui_query_binding::UiProjectionOptionReference,
) {
    let UiSemanticInteraction::Activate(activation) = super::activation(source, [10, 20]) else {
        panic!("the source world produces one exact activation")
    };
    let mut foreign = launch::<SelectionIntent>(
        selection_route_input(projection),
        PayloadProjectionRegistration::Collection(registration.clone()),
        PayloadApplicationFacts::default(),
    );
    let stopped = foreign
        .interaction
        .session
        .commit_selection_interaction(activation, option)
        .expect_err("an equivalent foreign launch cannot consume source interaction authority");
    assert_eq!(
        stopped.reason(),
        UiSelectionCommitStopReason::ApplicationGenerationChanged
    );
    let _ = foreign.interaction.session.shutdown();
}

fn selection_route_input(
    projection: &worth_ui_query_binding::WorthUiQueryViewIdentity,
) -> worth_ui_dsl::WorthUiRustAuthoredArtifactInput {
    let declaration = UiIntentDeclaration::<SelectionIntent>::selection_commit(DECLARATION)
        .unwrap()
        .bind_payload(
            SELECTION_FIELD,
            UiIntentPayloadSource::<UiIntentSelection>::projection(projection),
        );
    routed_input(declaration, WorthUiIntentInteractionFamily::SelectionCommit)
}

fn assert_selection_payload(
    world: &mut PayloadWorld,
    option: worth_ui_query_binding::UiProjectionOptionReference,
) -> worth_ui::facade::intent::UiPreparedIntentPayload {
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
    prepared
}

fn assert_query_revision(
    prepared: &worth_ui::facade::intent::UiPreparedIntentPayload,
    expected: &worth_ui_query_binding::UiProjectionInputRevision,
) {
    let [UiIntentInputOwnerRevision::Query(revision)] = prepared.input_basis().owner_revisions()
    else {
        panic!("the selection payload retains one exact Query owner revision")
    };
    assert_eq!(revision.revision(), expected);
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

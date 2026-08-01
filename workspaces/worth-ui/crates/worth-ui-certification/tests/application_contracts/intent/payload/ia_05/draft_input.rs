use worth_ui::facade::intent::{
    UiIntentDeclaration, UiIntentInputOwnerRevision, UiIntentPayloadSource,
};
use worth_ui::facade::interaction::{
    UiHostInteractionIngressOutcome, UiInteractionStop, UiInteractionTransition,
    UiLocalInputRecipientContract, UiLocalInputStopReason, UiSemanticInteraction,
};
use worth_ui_dsl::WorthUiIntentInteractionFamily;
use worth_ui_host_egui::UiEguiRawInputIngressOutcome;

use super::super::payload_types::{DraftIntent, DRAFT_FIELD};
use super::super::world::{
    launch_native, routed_input, PayloadApplicationFacts, PayloadProjectionRegistration,
    DECLARATION,
};

#[test]
fn ia_05_preedit_cancel_and_unicode_commit_seal_only_committed_draft() {
    let mut world = draft_world();
    bind_native_draft(&mut world);
    let committed = commit_native_draft(&mut world);
    assert_committed_payload(&mut world, committed);
    let _ = world.interaction.session.shutdown();
}

fn draft_world() -> super::super::world::PayloadWorld {
    let declaration = UiIntentDeclaration::<DraftIntent>::edit_commit(DECLARATION)
        .unwrap()
        .bind_payload(DRAFT_FIELD, UiIntentPayloadSource::committed_draft());
    launch_native::<DraftIntent>(
        routed_input(declaration, WorthUiIntentInteractionFamily::EditCommit),
        PayloadProjectionRegistration::None,
        PayloadApplicationFacts::default(),
    )
}

fn bind_native_draft(world: &mut super::super::world::PayloadWorld) {
    let activation = native_activation(world);
    world
        .interaction
        .session
        .bind_local_input_recipient(
            activation,
            UiLocalInputRecipientContract::draft(DRAFT_FIELD).unwrap(),
        )
        .expect("the exact payload field binds the runtime draft");
}

fn commit_native_draft(world: &mut super::super::world::PayloadWorld) -> UiSemanticInteraction {
    let native = world.interaction.native_input(vec![
        egui::Event::Text("é".to_owned()),
        egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "discard🦀".to_owned(),
            active_range_chars: Some(7..8),
        }),
        enter(true),
        enter(false),
        egui::Event::Ime(egui::ImeEvent::Preedit {
            text: String::new(),
            active_range_chars: None,
        }),
        egui::Event::Ime(egui::ImeEvent::Commit("🦀done".to_owned())),
        enter(true),
        enter(false),
    ]);
    assert!(matches!(
        native.adapter(),
        UiEguiRawInputIngressOutcome::Retained(_)
    ));
    let mut runtime = native.into_runtime().into_vec();
    assert_eq!(runtime.len(), 1);
    let receipt = applied(runtime.remove(0));
    assert_composition_active(&receipt);
    receipt
        .into_transitions()
        .into_vec()
        .into_iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(UiSemanticInteraction::EditCommit(commit)) => {
                Some(UiSemanticInteraction::EditCommit(commit))
            }
            _ => None,
        })
        .expect("declared Enter commit seals one edit interaction")
}

fn assert_committed_payload(
    world: &mut super::super::world::PayloadWorld,
    committed: UiSemanticInteraction,
) {
    let route = super::product_route(&world.interaction, committed);
    let prepared = world
        .interaction
        .session
        .prepare_intent_payload(route)
        .expect("only committed Unicode text reaches the typed payload");
    let cost = prepared.input_basis().cost();
    assert_eq!(cost.declared_fields(), 1);
    assert_eq!(cost.admitted_utf8_bytes(), "é🦀done".len());
    assert_eq!(prepared.retained_owner_reference_count(), 1);
    let [UiIntentInputOwnerRevision::Draft(revision)] = prepared.input_basis().owner_revisions()
    else {
        panic!("the exact draft revision is retained")
    };
    assert_eq!(revision.field(), DRAFT_FIELD.descriptor());
    assert_eq!(revision.input_revision(), Some(4));
    assert_eq!(revision.draft_revision(), 4);
    drop(prepared);
}

fn native_activation(
    world: &mut super::super::world::PayloadWorld,
) -> worth_ui::facade::interaction::UiActivateInteraction {
    let ingress = world.interaction.native_input(vec![
        pointer_button([10.0, 20.0], true),
        pointer_button([10.0, 20.0], false),
    ]);
    let UiEguiRawInputIngressOutcome::Retained(retained) = ingress.adapter() else {
        panic!("installed native pointer translators retain the activation pair")
    };
    assert_eq!(retained.report_count(), 2);
    let mut runtime = ingress.into_runtime().into_vec();
    assert_eq!(runtime.len(), 1);
    applied(runtime.remove(0))
        .into_transitions()
        .into_vec()
        .into_iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(UiSemanticInteraction::Activate(activation)) => {
                Some(activation)
            }
            _ => None,
        })
        .expect("one native pointer pair mints one production activation")
}

fn pointer_button(position: [f32; 2], pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos: egui::pos2(position[0], position[1]),
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

fn enter(pressed: bool) -> egui::Event {
    egui::Event::Key {
        key: egui::Key::Enter,
        physical_key: Some(egui::Key::Enter),
        pressed,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }
}

fn applied(
    outcome: UiHostInteractionIngressOutcome,
) -> worth_ui::facade::interaction::UiInteractionBatchReceipt {
    let UiHostInteractionIngressOutcome::Applied(receipt) = outcome else {
        panic!("native host observation reaches interaction admission, got {outcome:?}")
    };
    receipt
}

fn assert_composition_active(receipt: &worth_ui::facade::interaction::UiInteractionBatchReceipt) {
    let (stop_index, stop) = receipt
        .transitions()
        .iter()
        .enumerate()
        .find_map(|transition| match transition {
            (index, UiInteractionTransition::Stopped(UiInteractionStop::LocalInput(stop))) => {
                Some((index, stop))
            }
            _ => None,
        })
        .expect("preedit blocks a commit gesture with an exact local-input stop");
    assert_eq!(stop.reason(), UiLocalInputStopReason::CompositionActive);
    assert!(!stop.settled_session());
    let commit_index = receipt
        .transitions()
        .iter()
        .position(|transition| matches!(transition, UiInteractionTransition::Semantic(_)))
        .expect("post-cancel Enter eventually emits one semantic commit");
    assert!(stop_index < commit_index);
}

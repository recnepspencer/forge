use worth_ui::facade::interaction::{
    UiInteractionBatchReceipt, UiInteractionStop, UiInteractionTransition,
    UiLocalInputRecipientContract, UiLocalInputRecipientFamily, UiLocalInputStopReason,
    UiSemanticInteraction,
};
use worth_ui::facade::observation_report::UiHostPointerButtonTransition;
use worth_ui_host_contract::WorthUiHostMechanicsAdapter;
use worth_ui_host_egui::UiEguiRawInputIngressOutcome;
use worth_ui_test_support::draft_recipient_contract_for_certification;

use super::super::super::interaction_world::InteractionWorld;
use super::super::assertions::{applied, take_pointer_activation};

mod recipient_affinity;

#[test]
fn installed_native_translators_feed_bound_draft_and_ignore_repeat_metadata() {
    let mut world = InteractionWorld::native();
    bind_native_draft(&mut world);
    let receipt = commit_native_draft(&mut world);
    assert_committed_draft(&receipt);
    assert_native_teardown(world);
}

fn bind_native_draft(world: &mut InteractionWorld) {
    let activation_ingress = world.native_input(vec![
        pointer_button([20.0, 20.0], true),
        pointer_button([20.0, 20.0], false),
    ]);
    let UiEguiRawInputIngressOutcome::Retained(retained) = activation_ingress.adapter() else {
        panic!("installed pointer translator must retain the exact pair")
    };
    assert_eq!(retained.report_count(), 2);
    let mut activation_outcomes = activation_ingress.into_runtime().into_vec();
    assert_eq!(activation_outcomes.len(), 1);
    let activation = take_pointer_activation(activation_outcomes.remove(0));
    let contract = draft_recipient_contract_for_certification();
    let admission = world
        .session
        .bind_local_input_recipient(activation, contract)
        .expect("the sealed current activation binds the declared draft");
    assert_eq!(
        admission.binding().family(),
        UiLocalInputRecipientFamily::Draft
    );
    assert!(admission.displaced_recipient().is_none());
}

fn commit_native_draft(world: &mut InteractionWorld) -> UiInteractionBatchReceipt {
    let draft_ingress = world.native_input(vec![
        egui::Event::Text("é".to_owned()),
        egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "a🦀".to_owned(),
            active_range_chars: Some(1..2),
        }),
        egui::Event::Ime(egui::ImeEvent::Commit("done".to_owned())),
        key(egui::Key::Enter, true),
        key(egui::Key::Enter, false),
    ]);
    assert!(matches!(
        draft_ingress.adapter(),
        UiEguiRawInputIngressOutcome::Retained(_)
    ));
    let runtime = draft_ingress.into_runtime();
    assert_eq!(runtime.len(), 1);
    applied(runtime.into_vec().remove(0))
}

fn assert_committed_draft(receipt: &UiInteractionBatchReceipt) {
    let edits = receipt
        .transitions()
        .iter()
        .filter_map(|transition| match transition {
            UiInteractionTransition::Semantic(UiSemanticInteraction::EditCommit(edit)) => {
                Some(edit)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(edits.len(), 1);
    assert_eq!(edits[0].committed_text(), "édone");
    assert_eq!(receipt.state().active_draft_sessions(), 0);
    assert_eq!(receipt.state().active_recipients(), 0);
    let counters = receipt.state().counters();
    assert_eq!(counters.recipients_bound(), 1);
    assert_eq!(counters.draft_sessions_started(), 1);
    assert_eq!(counters.draft_sessions_settled(), 1);
    assert_eq!(counters.draft_mutations(), 3);
    assert_eq!(counters.semantic_interactions(), 2);
}

fn assert_native_teardown(world: InteractionWorld) {
    let host = world.native_host().clone();
    let host_session = world.session.host_session_identity().as_u64();
    let shutdown = world.session.shutdown();
    let final_state = shutdown.interaction().final_state().unwrap();
    assert_eq!(final_state.active_gestures(), 0);
    assert_eq!(final_state.active_draft_sessions(), 0);
    assert_eq!(final_state.active_recipients(), 0);
    assert_eq!(host.registered_surface_count(), 0);
    assert!(host
        .drain_mechanical_host_observations(host_session)
        .expect("released adapter drain remains readable")
        .into_batches()
        .is_empty());
}

#[test]
fn mounted_recipient_replacement_and_shutdown_settle_every_owner_once() {
    let mut world = InteractionWorld::canonical();
    let first = completed_activation(&mut world, 1);
    let draft = draft_recipient_contract_for_certification();
    world
        .session
        .bind_local_input_recipient(first, draft)
        .expect("the first activation binds a draft");

    let second = completed_activation(&mut world, 2);
    let admission = world
        .session
        .bind_local_input_recipient(second, UiLocalInputRecipientContract::activation())
        .expect("the successor activation replaces the local recipient");
    let displaced = admission
        .displaced_recipient()
        .expect("recipient replacement must be explicit");
    assert_eq!(
        displaced.reason(),
        UiLocalInputStopReason::RecipientReplaced
    );
    assert!(displaced.settled_recipient());
    assert!(!displaced.settled_session());
    assert_eq!(world.session.interaction_state().active_recipients(), 1);
    assert_eq!(world.session.interaction_state().active_draft_sessions(), 1);

    let shutdown = world.session.shutdown();
    let settlement = shutdown.interaction().settlement().unwrap();
    assert_eq!(settlement.settled_local_recipients(), 2);
    assert_eq!(settlement.settled_draft_sessions(), 1);
    assert_eq!(settlement.final_state().active_recipients(), 0);
    assert_eq!(settlement.final_state().active_draft_sessions(), 0);
}

#[test]
fn text_at_activation_recipient_stops_without_panicking_or_retargeting() {
    let mut world = InteractionWorld::native();
    let ingress = world.native_input(vec![
        pointer_button([20.0, 20.0], true),
        pointer_button([20.0, 20.0], false),
    ]);
    let activation = take_pointer_activation(ingress.into_runtime().into_vec().remove(0));
    world
        .session
        .bind_local_input_recipient(activation, UiLocalInputRecipientContract::activation())
        .expect("the current target binds an activation recipient");

    let text = world.native_input(vec![egui::Event::Text("wrong-family".to_owned())]);
    let receipt = applied(text.into_runtime().into_vec().remove(0));
    let stop = receipt
        .transitions()
        .iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Stopped(UiInteractionStop::LocalInput(stop)) => Some(stop),
            _ => None,
        })
        .expect("wrong-family text emits an exact local stop");
    assert_eq!(
        stop.reason(),
        UiLocalInputStopReason::RecipientFamilyMismatch {
            required: UiLocalInputRecipientFamily::Draft,
            active: UiLocalInputRecipientFamily::Activation,
        }
    );
    assert!(!stop.settled_recipient());
    assert_eq!(receipt.state().active_recipients(), 1);
    let shutdown = world.session.shutdown();
    assert_eq!(
        shutdown
            .interaction()
            .settlement()
            .unwrap()
            .settled_local_recipients(),
        1
    );
}

fn completed_activation(
    world: &mut InteractionWorld,
    pointer: u64,
) -> worth_ui::facade::interaction::UiActivateInteraction {
    let _ = applied(world.button(pointer, 1, UiHostPointerButtonTransition::Pressed, [20, 20]));
    take_pointer_activation(world.button(
        pointer,
        1,
        UiHostPointerButtonTransition::Released,
        [20, 20],
    ))
}

fn native_activation(
    world: &mut InteractionWorld,
) -> worth_ui::facade::interaction::UiActivateInteraction {
    let ingress = world.native_input(vec![
        pointer_button([20.0, 20.0], true),
        pointer_button([20.0, 20.0], false),
    ]);
    take_pointer_activation(ingress.into_runtime().into_vec().remove(0))
}

fn pointer_button(position: [f32; 2], pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos: egui::pos2(position[0], position[1]),
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

fn key(key: egui::Key, repeat: bool) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat,
        modifiers: egui::Modifiers::NONE,
    }
}

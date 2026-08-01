use worth_ui::facade::intent::{UiIntentAdmissionDecision, UiIntentResourceCensus};
use worth_ui::facade::interaction::{UiHostInteractionIngressOutcome, UiInteractionTransition};
use worth_ui_host_egui::UiEguiRawInputIngressOutcome;
use worth_ui_test_support::draft_recipient_contract_for_certification;

use super::{
    assert_evidence_count, assert_only_evidence, assert_retirement, census, latest_evidence,
};
use crate::intent::admission::phase3::world::AdmissionWorld;
use crate::intent::interaction_world::InteractionWorld;

#[test]
fn draft_replacement_denial_and_shutdown_leave_no_retained_resource() {
    draft_shutdown();
    policy_denial();
}

fn draft_shutdown() {
    let mut world = InteractionWorld::native();
    let activation = activation(&mut world);
    world
        .session
        .bind_local_input_recipient(activation, draft_recipient_contract_for_certification())
        .expect("the current activation binds one draft owner");
    let draft_text = "lifecycle-draft";
    let ingress = world.native_input(vec![egui::Event::Text(draft_text.to_owned())]);
    assert!(matches!(
        ingress.adapter(),
        UiEguiRawInputIngressOutcome::Retained(_)
    ));
    let _ = ingress.into_runtime();
    let active = census(&world.session);
    assert_eq!(active.active_input_recipients(), 1);
    assert_eq!(active.active_draft_sessions(), 1);
    assert_eq!(active.retained_draft_utf8_bytes(), draft_text.len());
    assert_evidence_count(active, 1);

    let shutdown = world.session.shutdown();
    let settlement = shutdown
        .interaction()
        .settlement()
        .expect("draft shutdown owns one settlement");
    assert_eq!(settlement.settled_local_recipients(), 1);
    assert_eq!(settlement.settled_draft_sessions(), 1);
    assert_eq!(
        shutdown.intent_resource_census(),
        UiIntentResourceCensus::EMPTY
    );
    assert_retirement(
        shutdown.intent_evidence(),
        worth_ui_inspection::UiIntentEvidenceRetirementCause::ApplicationShutdown,
        1,
    );
}

fn policy_denial() {
    let mut world = AdmissionWorld::launch(1);
    world.set_policy(false);
    let UiIntentAdmissionDecision::Stopped(stop) = world.admit(0) else {
        panic!("policy denial must stop before admission")
    };
    assert!(matches!(
        stop.reason(),
        worth_ui::facade::intent::UiIntentAdmissionStopReason::Inoperable(_)
    ));
    assert_only_evidence(census(&world.session), 1);
    assert_eq!(
        latest_evidence(&world.session).input().family(),
        worth_ui_inspection::UiIntentInteractionEvidenceFamily::Activate
    );
    let shutdown = world.session.shutdown();
    assert_eq!(
        shutdown.intent_resource_census(),
        UiIntentResourceCensus::EMPTY
    );
    assert_retirement(
        shutdown.intent_evidence(),
        worth_ui_inspection::UiIntentEvidenceRetirementCause::ApplicationShutdown,
        1,
    );
}

fn activation(
    world: &mut InteractionWorld,
) -> worth_ui::facade::interaction::UiActivateInteraction {
    let ingress = world.native_input(vec![pointer_button(true), pointer_button(false)]);
    assert!(matches!(
        ingress.adapter(),
        UiEguiRawInputIngressOutcome::Retained(_)
    ));
    let mut outcomes = ingress.into_runtime().into_vec();
    assert_eq!(outcomes.len(), 1);
    let UiHostInteractionIngressOutcome::Applied(receipt) = outcomes.remove(0) else {
        panic!("native release reaches the production interaction owner")
    };
    receipt
        .into_transitions()
        .into_vec()
        .into_iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(
                worth_ui::facade::interaction::UiSemanticInteraction::Activate(activation),
            ) => Some(activation),
            _ => None,
        })
        .expect("press and release mint one activation")
}

fn pointer_button(pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos: egui::pos2(20.0, 20.0),
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

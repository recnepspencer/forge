use worth_runtime_bridge::facade::BridgeMixedCauseOrderingInput;
use worth_signal::facade::NodeId;
use worth_ui::facade::intent::{
    UiIntentInoperableCause, UiIntentMutabilityPosture, UiIntentOperabilityOutcome,
    UiIntentReadinessPosture,
};
use worth_ui::facade::interaction::{
    UiHostInteractionIngressOutcome, UiInteractionTransition, UiLocalInputRecipientContract,
    UiSemanticInteraction,
};
use worth_ui::facade::observation::UiChangeClassificationOutcome;
use worth_ui::facade::rebind::{
    UiRebindExecutionPolicy, UiRebindExecutionRequest, UiRebindOutcome,
};
use worth_ui_query_binding::{
    UiProjectionFieldRequirement, UiProjectionObservation, UiScalarProjectionRegistration,
    WorthUiQueryWorkspaceExt,
};

use super::super::intent_types::EDIT_FIELD;
use super::super::world::{OperabilityWorld, PRIMARY_POINT};
use crate::projection_lifecycle::support::ScalarLifecycleWorld;

#[test]
fn ia_04_projection_role_is_readonly_while_exact_currentness_controls_readiness() {
    let (mut query, completion) =
        ScalarLifecycleWorld::standard(NodeId::new(314_040, 0), "query-current");
    let registration = scalar_registration(&query);
    let mut world = OperabilityWorld::projection(registration);

    let pending = decision(world.evaluate(PRIMARY_POINT));
    assert_eq!(pending.mutability(), UiIntentMutabilityPosture::Readonly);
    assert_eq!(pending.readiness(), UiIntentReadinessPosture::Pending);
    assert_eq!(
        pending.causes().collect::<Vec<_>>(),
        [
            UiIntentInoperableCause::Readonly,
            UiIntentInoperableCause::Pending
        ]
    );

    let predecessor = query.initial().into_fact_and_predecessor().0;
    let current = query
        .advance(
            BridgeMixedCauseOrderingInput::AsyncCompletion(completion),
            Some(predecessor),
        )
        .into_fact_and_predecessor()
        .0;
    publish_projection(
        &mut world,
        UiProjectionObservation::Scalar(current.into_observation()),
    );
    world.interaction.publish_successor();

    let current = decision(world.evaluate(PRIMARY_POINT));
    assert_eq!(current.mutability(), UiIntentMutabilityPosture::Readonly);
    assert_eq!(current.readiness(), UiIntentReadinessPosture::Ready);
    assert_eq!(
        current.causes().collect::<Vec<_>>(),
        [UiIntentInoperableCause::Readonly]
    );
    let _ = world.interaction.session.shutdown();
}

#[test]
fn ia_04_committed_draft_role_is_writable_and_ready_only_after_real_commit() {
    let mut world = OperabilityWorld::committed_draft();
    let activation = native_activation(&mut world);
    world
        .interaction
        .session
        .bind_local_input_recipient(
            activation,
            UiLocalInputRecipientContract::draft(EDIT_FIELD).unwrap(),
        )
        .expect("the exact edit field binds the runtime draft owner");
    let committed = native_commit(&mut world);
    let candidate = world.prepare_interaction(committed);
    assert_eq!(
        candidate.retained_operability_dependency_reference_count(),
        2
    );
    let outcome = world
        .interaction
        .session
        .evaluate_intent_operability(candidate);
    let UiIntentOperabilityOutcome::Operable(proof) = outcome else {
        panic!("a committed current draft must mint the exact operability proof")
    };
    assert_eq!(
        proof.decision().mutability(),
        UiIntentMutabilityPosture::Writable
    );
    assert_eq!(
        proof.decision().readiness(),
        UiIntentReadinessPosture::Ready
    );
    drop(proof);
    let _ = world.interaction.session.shutdown();
}

fn scalar_registration(world: &ScalarLifecycleWorld) -> UiScalarProjectionRegistration {
    let domain = world
        .workspace
        .worth_ui()
        .expect("Worth UI domain installed");
    UiScalarProjectionRegistration::text(
        domain
            .projection_view("platform.pulse.status")
            .expect("fixture projection is installed"),
        UiProjectionFieldRequirement::declared("status").unwrap(),
    )
}

fn publish_projection(world: &mut OperabilityWorld, observation: UiProjectionObservation) {
    let mut turn = world.interaction.session.begin_observation_turn().unwrap();
    turn.admit_projection_query(observation).unwrap();
    let admitted = turn.seal().unwrap();
    let changed = match world
        .interaction
        .session
        .classify_observations(admitted)
        .unwrap()
    {
        UiChangeClassificationOutcome::Changed(changed) => changed,
        UiChangeClassificationOutcome::ObservedNoChange(_) => {
            panic!("a new projection fact cannot classify as no change")
        }
        UiChangeClassificationOutcome::EvidenceOnly(_) => {
            panic!("an operability projection must retain semantic change")
        }
    };
    let lifecycle = world
        .interaction
        .session
        .resolve_affected_scope(changed)
        .unwrap()
        .resolve_identity_lifecycle()
        .unwrap();
    let plan = world
        .interaction
        .session
        .compile_rebind_plan(lifecycle, UiRebindExecutionPolicy::ordinary())
        .unwrap();
    let prepared = world
        .interaction
        .session
        .prepare_rebind(plan, UiRebindExecutionRequest::new(314_040))
        .expect("projection observation prepares an exact successor");
    assert!(matches!(
        prepared.execute(314_040),
        UiRebindOutcome::Published(_)
    ));
}

fn decision(
    outcome: UiIntentOperabilityOutcome,
) -> worth_ui::facade::intent::UiIntentOperabilityDecision {
    match outcome {
        UiIntentOperabilityOutcome::Inoperable(candidate) => candidate.decision().clone(),
        UiIntentOperabilityOutcome::Operable(_) => {
            panic!("readonly projection must never mint operability proof")
        }
    }
}

fn native_activation(
    world: &mut OperabilityWorld,
) -> worth_ui::facade::interaction::UiActivateInteraction {
    let ingress = world.interaction.native_input(vec![
        pointer_button([10.0, 20.0], true),
        pointer_button([10.0, 20.0], false),
    ]);
    applied(ingress.into_runtime().into_vec().remove(0))
        .into_transitions()
        .into_vec()
        .into_iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(UiSemanticInteraction::Activate(activation)) => {
                Some(activation)
            }
            _ => None,
        })
        .expect("native pointer pair mints one activation")
}

fn native_commit(world: &mut OperabilityWorld) -> UiSemanticInteraction {
    let ingress = world.interaction.native_input(vec![
        egui::Event::Text("committed".to_owned()),
        enter(true),
        enter(false),
    ]);
    applied(ingress.into_runtime().into_vec().remove(0))
        .into_transitions()
        .into_vec()
        .into_iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(UiSemanticInteraction::EditCommit(commit)) => {
                Some(UiSemanticInteraction::EditCommit(commit))
            }
            _ => None,
        })
        .expect("native Enter seals one committed draft interaction")
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
        panic!("native observation must reach interaction admission")
    };
    receipt
}

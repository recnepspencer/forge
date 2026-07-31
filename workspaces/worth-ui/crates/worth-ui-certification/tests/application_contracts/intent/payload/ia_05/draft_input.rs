use worth_ui::facade::intent::{
    UiIntentDeclaration, UiIntentInputOwnerRevision, UiIntentPayloadSource,
};
use worth_ui::facade::interaction::{
    UiHostInteractionIngressOutcome, UiInteractionStop, UiInteractionTransition,
    UiLocalInputRecipientContract, UiLocalInputStopReason, UiSemanticInteraction,
};
use worth_ui::facade::observation_report::{
    UiHostImeCompositionPhase, UiHostImePreedit, UiHostKey, UiHostKeyTransition,
    UiHostKeyboardModifiers, UiHostObservationPayload,
};
use worth_ui_dsl::WorthUiIntentInteractionFamily;

use super::super::payload_types::{DraftIntent, DRAFT_FIELD};
use super::super::world::{
    launch, routed_input, PayloadApplicationFacts, PayloadProjectionRegistration, DECLARATION,
};

#[test]
fn ia_05_preedit_cancel_and_unicode_commit_seal_only_committed_draft() {
    let declaration = UiIntentDeclaration::<DraftIntent>::edit_commit(DECLARATION)
        .unwrap()
        .bind_payload(DRAFT_FIELD, UiIntentPayloadSource::committed_draft());
    let mut world = launch::<DraftIntent>(
        routed_input(declaration, WorthUiIntentInteractionFamily::EditCommit),
        PayloadProjectionRegistration::None,
        PayloadApplicationFacts::default(),
    );
    let UiSemanticInteraction::Activate(activation) = super::activation(&mut world, [10, 20])
    else {
        panic!("pointer pair produces an activation")
    };
    world
        .interaction
        .session
        .bind_local_input_recipient(
            activation,
            UiLocalInputRecipientContract::draft(DRAFT_FIELD).unwrap(),
        )
        .expect("the exact payload field binds the runtime draft");

    applied(world.interaction.payload_at(
        3,
        3,
        UiHostObservationPayload::TextInput {
            revision: 1,
            text: "é".into(),
        },
    ));
    applied(world.interaction.payload_at(
        4,
        4,
        UiHostObservationPayload::ImeComposition {
            revision: 2,
            phase: UiHostImeCompositionPhase::Preedit(
                UiHostImePreedit::from_unicode_scalar_range("discard🦀", Some(7..8)).unwrap(),
            ),
        },
    ));
    assert_composition_active(applied(world.interaction.payload_at(5, 5, enter())));
    applied(world.interaction.payload_at(
        6,
        6,
        UiHostObservationPayload::ImeComposition {
            revision: 3,
            phase: UiHostImeCompositionPhase::Cancel,
        },
    ));
    applied(world.interaction.payload_at(
        7,
        7,
        UiHostObservationPayload::ImeComposition {
            revision: 4,
            phase: UiHostImeCompositionPhase::Commit("🦀done".into()),
        },
    ));
    let committed = applied(world.interaction.payload_at(8, 8, enter()))
        .into_transitions()
        .into_vec()
        .into_iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(UiSemanticInteraction::EditCommit(commit)) => {
                Some(UiSemanticInteraction::EditCommit(commit))
            }
            _ => None,
        })
        .expect("declared Enter commit seals one edit interaction");
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
    let _ = world.interaction.session.shutdown();
}

fn enter() -> UiHostObservationPayload {
    UiHostObservationPayload::Keyboard {
        logical_key: UiHostKey::Enter,
        physical_key: Some(UiHostKey::Enter),
        modifiers: UiHostKeyboardModifiers::default(),
        transition: UiHostKeyTransition::Pressed { repeat: false },
    }
}

fn applied(
    outcome: UiHostInteractionIngressOutcome,
) -> worth_ui::facade::interaction::UiInteractionBatchReceipt {
    let UiHostInteractionIngressOutcome::Applied(receipt) = outcome else {
        panic!("canonical host observation reaches interaction admission")
    };
    receipt
}

fn assert_composition_active(receipt: worth_ui::facade::interaction::UiInteractionBatchReceipt) {
    let stop = receipt
        .transitions()
        .iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Stopped(UiInteractionStop::LocalInput(stop)) => Some(stop),
            _ => None,
        })
        .expect("preedit blocks a commit gesture with an exact local-input stop");
    assert_eq!(stop.reason(), UiLocalInputStopReason::CompositionActive);
    assert!(!stop.settled_session());
    assert!(receipt
        .transitions()
        .iter()
        .all(|transition| !matches!(transition, UiInteractionTransition::Semantic(_))));
}

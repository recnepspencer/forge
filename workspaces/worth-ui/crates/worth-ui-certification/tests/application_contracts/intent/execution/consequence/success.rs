use worth_ui::facade::{
    intent::{UiIntentConsequenceContract, UiIntentConsequencePublicationOutcome},
    rebind::{UiRebindExecutionPolicy, UiRebindExecutionRequest},
};

use super::world::ConsequenceWorld;

#[test]
fn completed_effect_publishes_declared_query_and_mounted_posture_as_one_batch() {
    let mut world = ConsequenceWorld::launch(
        UiIntentConsequenceContract::mounted_posture_and_query(world_query_identity()),
    );
    let before = world.query_change_state();
    assert_eq!(before.staged_change_count(), 0);
    assert_eq!(before.admitted_change_count(), 0);

    let handle = world.complete_with_query();
    let staged = world.query_change_state();
    assert_eq!(staged.staged_change_count(), 1);
    assert_eq!(staged.admitted_change_count(), 0);
    assert_eq!(world.provider_calls(), [1, 1]);

    let receipt = match world.interaction.session.publish_intent_consequences(
        handle,
        UiRebindExecutionPolicy::ordinary(),
        UiRebindExecutionRequest::new(40),
    ) {
        UiIntentConsequencePublicationOutcome::Published(receipt) => receipt,
        UiIntentConsequencePublicationOutcome::InFlight(_) => {
            panic!("the headless consequence frame unexpectedly remained in flight")
        }
        UiIntentConsequencePublicationOutcome::Indeterminate(_) => {
            panic!("the headless consequence frame became indeterminate")
        }
        UiIntentConsequencePublicationOutcome::InternalDefect(defect) => {
            panic!("consequence publication defect: {:?}", defect.kind())
        }
        UiIntentConsequencePublicationOutcome::Stopped(stop) => {
            panic!("consequence handoff stopped: {:?}", stop.reason())
        }
        UiIntentConsequencePublicationOutcome::NoConsequences(_) => {
            panic!("declared mounted and Query consequences cannot be empty")
        }
    };
    let scope = receipt
        .rebind()
        .plan()
        .scope()
        .expect("the consequence publication resolves one changed scope");
    assert_eq!(scope.facts().len(), 2);
    assert_eq!(
        scope
            .facts()
            .iter()
            .filter(|fact| fact.intent_posture().is_some())
            .count(),
        1
    );
    assert_eq!(
        scope
            .facts()
            .iter()
            .filter(|fact| fact.query().is_some())
            .count(),
        1
    );
    let selected_entries = scope
        .lookups()
        .iter()
        .map(|lookup| lookup.predecessor().entries().len() + lookup.candidate().entries().len())
        .sum::<usize>();
    let cost = scope.cost();
    assert_eq!(cost.observations(), 2);
    assert_eq!(cost.changed_facts(), 2);
    assert_eq!(cost.lookup_receipts(), 4);
    assert_eq!(cost.index_probes(), 4);
    assert_eq!(cost.contract_checks(), selected_entries);
    assert_eq!(cost.graph_and_mounted_entries(), selected_entries);
    assert_eq!(cost.indexed_consumers(), scope.consumers().len());
    assert_eq!(
        receipt.rebind().decision_record().consumer_count(),
        cost.indexed_consumers()
    );
    let published = world.query_change_state();
    assert_eq!(published.staged_change_count(), 0);
    assert_eq!(published.admitted_change_count(), 1);
    assert_eq!(published.next_change_order(), staged.next_change_order());
    assert_eq!(world.provider_calls(), [1, 1]);

    let transcripts = world.transcripts();
    let latest = transcripts
        .last()
        .expect("consequence publication records a frame");
    assert!(latest
        .semantic_text()
        .iter()
        .any(|text| text.text() == "COMPLETED"));
    drop(receipt);
    world.shutdown();
}

fn world_query_identity() -> worth_ui_query_binding::WorthUiQueryViewIdentity {
    worth_ui_query_binding::WorthUiQueryViewIdentity::new("certification.live.measurements")
        .expect("static consequence Query identity")
}

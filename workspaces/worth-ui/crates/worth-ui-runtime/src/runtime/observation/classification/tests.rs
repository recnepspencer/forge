use super::UiChangeClassificationOutcome;
use crate::fact_contract::{UiAuthoredFactKind, UiProducedFact};
use crate::runtime::tests::active_application_session_test_support::{
    component_candidate_submission, source_backed_component_session,
};

#[test]
fn exact_repeated_authored_observation_is_terminal_no_change() {
    let mut session = source_backed_component_session();
    let candidate = component_candidate_submission(
        &session,
        "active-session-current",
        "workspace.component.active_session_current",
    );

    let mut turn = session
        .begin_observation_turn()
        .expect("production observation turn begins");
    turn.admit_source(candidate)
        .expect("production source observation admits");
    let observations = turn.seal().expect("non-empty turn seals");
    let outcome = session
        .classify_observations(observations)
        .expect("admitted source observation classifies");
    let receipt = match outcome {
        UiChangeClassificationOutcome::ObservedNoChange(receipt) => receipt,
        _ => panic!("the exact current source observation must be a no-change receipt"),
    };

    assert_eq!(receipt.observation_count(), 1);
    let _ = session.shutdown();
}

#[test]
fn equivalent_semantics_with_new_source_evidence_is_not_no_change() {
    let mut session = source_backed_component_session();
    let candidate = component_candidate_submission(
        &session,
        "same-semantics-new-evidence",
        "workspace.component.active_session_current",
    );

    let mut turn = session
        .begin_observation_turn()
        .expect("production observation turn begins");
    turn.admit_source(candidate)
        .expect("production source observation admits");
    let observations = turn.seal().expect("non-empty turn seals");
    let evidence = match session
        .classify_observations(observations)
        .expect("admitted source observation classifies")
    {
        UiChangeClassificationOutcome::EvidenceOnly(evidence) => evidence,
        _ => panic!("new exact source evidence with equal semantics must be evidence-only"),
    };

    assert_eq!(
        evidence.active_artifact_digest(),
        evidence.candidate_artifact_digest()
    );
    let _ = session.shutdown();
}

#[test]
fn semantic_source_change_enumerates_owner_issued_facts() {
    let mut session = source_backed_component_session();
    let candidate = component_candidate_submission(
        &session,
        "changed-component-evidence",
        "workspace.component.active_session_candidate",
    );

    let mut turn = session
        .begin_observation_turn()
        .expect("production observation turn begins");
    turn.admit_source(candidate)
        .expect("production source observation admits");
    let observations = turn.seal().expect("non-empty turn seals");
    let changed = match session
        .classify_observations(observations)
        .expect("admitted source observation classifies")
    {
        UiChangeClassificationOutcome::Changed(changed) => changed,
        _ => panic!("a distinct authored component must classify as changed"),
    };

    let authored = changed
        .facts()
        .iter()
        .filter_map(|fact| match fact {
            UiProducedFact::AuthoredSource(authored) => Some(authored.kind()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        authored,
        [UiAuthoredFactKind::Created, UiAuthoredFactKind::Retired],
        "identity-first ordering must enumerate creation and retirement"
    );
    let _ = session.shutdown();
}

#[test]
fn query_owner_preserves_an_explicit_reset_consequence() {
    let mut fixture =
        worth_ui_query_binding::certification::WorthUiOperationLiveTestFixture::
            without_collection_entity_lookup("phase-312-query-reset");
    let mut binding = fixture.binding_plan().prepare_downstream_state();
    binding
        .admit_operation_live(fixture.open_resource())
        .expect("the exact Query resource admits");
    fixture.update_measurement();
    let consequence = match binding
        .refresh_operation_live(fixture.refresh_request())
        .expect("unsupported incremental meaning produces a UI consequence")
    {
        worth_ui_query_binding::WorthUiOperationLiveRefreshOutcome::Applied(consequence) => {
            consequence
        }
        worth_ui_query_binding::WorthUiOperationLiveRefreshOutcome::NoSemanticDelivery => {
            panic!("the changed Query value must not disappear")
        }
    };
    let observation = binding
        .validate_operation_live_change_observation(consequence)
        .expect("the owner validates its exact consequence");
    let UiProducedFact::Query(fact) = super::owner::query::classify(observation) else {
        panic!("the Query owner can produce only a Query fact")
    };
    assert!(matches!(
        fact.kind(),
        crate::fact_contract::UiQueryChangedFactKind::Reset(_)
    ));
    let retirement = binding.into_operation_live_retirement();
    assert!(matches!(
        fixture.close_retirement(retirement),
        worth_ui_query_binding::WorthUiOperationLiveRetirementCloseOutcome::Closed(_)
    ));
}

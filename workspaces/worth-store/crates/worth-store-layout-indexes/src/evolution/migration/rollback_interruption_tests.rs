use std::collections::BTreeSet;

use super::test_support::{
    current_authority, declaration, rollback_execution_request_for_publication,
};
use super::{
    layout_rollback_execution, layout_rollback_interruption_cases, LayoutEvolutionDenial,
    LayoutRollbackInterruptionPosture,
};

#[test]
fn rollback_replay_classifies_each_execution_boundary() {
    let authority = current_authority("store.rollback.interruption", "current");
    let source = rollback_execution_request_for_publication(
        declaration(),
        &authority,
        "rollback-replay",
        2_301,
    );
    let source_state = source.interruption_state();
    let replay = rollback_execution_request_for_publication(
        declaration(),
        &authority,
        "rollback-replay",
        2_301,
    );
    assert_eq!(
        replay.classify_interruption(source_state).into_result(),
        Ok(LayoutRollbackInterruptionPosture::ResumeFromSource)
    );

    let mut publication =
        worth_store_physical_isolation::PhysicalRootPublicationRuntime::from_current_root(
            source.publication_source_root(),
        );
    let published_state = layout_rollback_execution(&mut publication)
        .execute(source)
        .into_published()
        .expect("rollback must publish")
        .interruption_state();
    assert_eq!(
        replay.classify_interruption(published_state).into_result(),
        Ok(LayoutRollbackInterruptionPosture::TargetAlreadyPublished)
    );
}

#[test]
fn rollback_replay_rejects_a_different_physical_execution() {
    let authority = current_authority("store.rollback.execution.binding", "current");
    let first = rollback_execution_request_for_publication(
        declaration(),
        &authority,
        "rollback-publication-first",
        2_311,
    );
    let mut publication =
        worth_store_physical_isolation::PhysicalRootPublicationRuntime::from_current_root(
            first.publication_source_root(),
        );
    let state = layout_rollback_execution(&mut publication)
        .execute(first)
        .into_published()
        .expect("first rollback must publish")
        .interruption_state();
    let different = rollback_execution_request_for_publication(
        declaration(),
        &authority,
        "rollback-publication-different",
        2_312,
    );

    assert!(matches!(
        different.classify_interruption(state).into_result(),
        Err(LayoutEvolutionDenial::RollbackInterruptStateDoesNotMatchExecution { .. })
    ));
}

#[test]
fn rollback_interruption_owner_declares_exactly_ordinary_cases() {
    let authority = current_authority("store.rollback.interruption.cases", "current");
    let request = rollback_execution_request_for_publication(
        declaration(),
        &authority,
        "rollback-case-inventory",
        2_321,
    );
    let source_state = request.interruption_state();
    let matching = rollback_execution_request_for_publication(
        declaration(),
        &authority,
        "rollback-case-inventory",
        2_321,
    );
    let mut publication =
        worth_store_physical_isolation::PhysicalRootPublicationRuntime::from_current_root(
            request.publication_source_root(),
        );
    let target_state = layout_rollback_execution(&mut publication)
        .execute(request)
        .into_published()
        .unwrap()
        .interruption_state();
    let different = rollback_execution_request_for_publication(
        declaration(),
        &authority,
        "rollback-case-inventory-other",
        2_322,
    );

    let observed = [
        matching.classify_interruption(source_state).case_id(),
        matching
            .classify_interruption(target_state.clone())
            .case_id(),
        different.classify_interruption(target_state).case_id(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    let declared = layout_rollback_interruption_cases().collect::<BTreeSet<_>>();
    assert_eq!(observed, declared);
}

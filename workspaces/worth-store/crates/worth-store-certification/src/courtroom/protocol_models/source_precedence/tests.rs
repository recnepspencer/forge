use worth_store_formal_models::{
    map_recovery_source_decision_trace, require_selectable_source, SourceAuthorityPosture,
    SourcePrecedenceAction, SourcePrecedenceActionKind, SourcePrecedenceDenial,
};
use worth_store_test_support::harness::recovery::redo_replay::checkpoint_plus_tail_source;

use super::scenario::execute_ordinary_source_precedence;

#[test]
fn ordinary_owner_execution_covers_every_source_precedence_action_kind() {
    let mut observed = execute_ordinary_source_precedence()
        .into_iter()
        .map(SourcePrecedenceAction::kind)
        .collect::<Vec<_>>();
    observed.sort_unstable();
    observed.dedup();

    assert_eq!(observed, SourcePrecedenceActionKind::all());
}

#[test]
fn ordinary_precedence_retains_every_candidate_and_selects_once() {
    let admitted = checkpoint_plus_tail_source(20, 21);
    let trace = admitted.trace();
    let actions = map_recovery_source_decision_trace(trace);
    let discoveries = actions
        .iter()
        .filter(|action| matches!(action, SourcePrecedenceAction::CandidateDiscovered { .. }))
        .count();

    assert_eq!(discoveries, trace.candidate_count());
    assert_eq!(
        actions.last(),
        Some(&SourcePrecedenceAction::SourceSelected)
    );
    assert!(actions.contains(&SourcePrecedenceAction::ContradictionPreserved));
}

#[test]
fn repeated_reopen_basis_is_deterministic() {
    let first = checkpoint_plus_tail_source(20, 21);
    let second = checkpoint_plus_tail_source(20, 21);

    assert_eq!(
        first.trace().canonical_replay_digest(),
        second.trace().canonical_replay_digest()
    );
    assert_eq!(
        map_recovery_source_decision_trace(first.trace()),
        map_recovery_source_decision_trace(second.trace())
    );
}

#[test]
fn derived_and_quarantined_sources_cannot_be_promoted() {
    assert_eq!(
        require_selectable_source(SourceAuthorityPosture::DerivedLocator),
        Err(SourcePrecedenceDenial::DerivedSourceCannotBeAuthority)
    );
    assert_eq!(
        require_selectable_source(SourceAuthorityPosture::ReplayHelper),
        Err(SourcePrecedenceDenial::DerivedSourceCannotBeAuthority)
    );
    assert_eq!(
        require_selectable_source(SourceAuthorityPosture::Quarantined),
        Err(SourcePrecedenceDenial::QuarantinedSourceCannotBeSelected)
    );
}

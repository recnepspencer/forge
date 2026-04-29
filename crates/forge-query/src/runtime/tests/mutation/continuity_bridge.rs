use super::super::support::*;
use forge_runtime_bridge::facade::{BridgeContinuityMutationBundle, BridgeContinuityOutcomeClass};

#[test]
fn bridge_split_successor_continuity_preserves_successor_set() {
    let evidence = ForgeQueryContinuityMutationEvidence::from_bridge(
        &BridgeContinuityMutationBundle::split_existing_target(
            BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors,
            "authority:task-1",
            ["authority:task-1:a", "authority:task-1:b"],
            Some("binding:sha256:task-1"),
            Some("entity:task-1"),
            Some("Task"),
            "lineage:sha256:task-1",
            "continuity:sha256:task-1",
        )
        .expect("split continuity bundle should build"),
    );

    assert_eq!(
        evidence.family(),
        ForgeQueryContinuityMutationFamily::SplitExistingTarget
    );
    assert_eq!(
        evidence.outcome_class(),
        ForgeQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors
    );
    assert_eq!(
        evidence.successor_authoritative_identities(),
        &[
            "authority:task-1:a".to_string(),
            "authority:task-1:b".to_string()
        ]
    );
    assert_eq!(evidence.successor_authoritative_identity(), None);
    assert_eq!(
        evidence.basis_binding_digest(),
        Some("binding:sha256:task-1")
    );
}

#[test]
fn split_successor_intent_requires_at_least_two_successors() {
    let error = ForgeQueryContinuityMutationIntent::split_existing_target(
        "authority:task-1",
        ["authority:task-1:a"],
    )
    .expect_err("split-successor continuity should reject a singleton successor set");

    assert_eq!(
        error.to_string(),
        "split-successor continuity requires at least two successor authoritative identities"
    );
}

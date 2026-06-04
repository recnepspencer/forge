use super::super::support::*;
use forge_runtime_bridge::facade::{
    BridgeContinuityAuthoritativeIdentity, BridgeContinuityMutationBundle,
    BridgeContinuityOutcomeClass, BridgeContinuityResolvedTargetIdentity,
    BridgeContinuityTargetCollection,
};

#[test]
fn bridge_split_successor_continuity_preserves_successor_set() {
    let evidence = ForgeQueryContinuityMutationEvidence::from_bridge(
        &BridgeContinuityMutationBundle::split_existing_target(
            BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors,
            continuity_identity("authority:task-1"),
            [
                continuity_identity("authority:task-1:a"),
                continuity_identity("authority:task-1:b"),
            ],
            Some(resolved_target("entity:task-1")),
            Some(target_collection("Task")),
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
    assert!(evidence.basis_binding_digest().is_some());
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

fn continuity_identity(value: &str) -> BridgeContinuityAuthoritativeIdentity {
    BridgeContinuityAuthoritativeIdentity::new(value)
        .expect("test continuity identity should be native")
}

fn resolved_target(value: &str) -> BridgeContinuityResolvedTargetIdentity {
    BridgeContinuityResolvedTargetIdentity::new(value)
        .expect("test resolved target should be native")
}

fn target_collection(value: &str) -> BridgeContinuityTargetCollection {
    BridgeContinuityTargetCollection::new(value).expect("test target collection should be native")
}

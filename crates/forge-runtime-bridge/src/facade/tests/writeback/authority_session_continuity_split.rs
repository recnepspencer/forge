use super::support::*;
use std::sync::Arc;

fn authoritative_identity(value: impl Into<Arc<str>>) -> BridgeContinuityAuthoritativeIdentity {
    BridgeContinuityAuthoritativeIdentity::new(value)
        .expect("test continuity authoritative identity should be valid")
}

fn resolved_target_identity(value: impl Into<Arc<str>>) -> BridgeContinuityResolvedTargetIdentity {
    BridgeContinuityResolvedTargetIdentity::new(value)
        .expect("test continuity resolved target should be valid")
}

#[test]
fn split_successor_bundle_preserves_successor_set() {
    let bundle = crate::facade::BridgeContinuityMutationBundle::split_existing_target(
        crate::continuity::BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors,
        authoritative_identity("authority:task-1"),
        [
            authoritative_identity("authority:task-1:a"),
            authoritative_identity("authority:task-1:b"),
        ],
        Some(resolved_target_identity("entity:task-1")),
        Some(
            BridgeContinuityTargetCollection::new("Task")
                .expect("test continuity target collection should be native"),
        ),
    )
    .expect("split continuity bundle should build");

    assert_eq!(
        bundle.successor_authoritative_identities(),
        [
            authoritative_identity("authority:task-1:a"),
            authoritative_identity("authority:task-1:b"),
        ]
        .as_slice()
    );
    assert_eq!(bundle.successor_authoritative_identity(), None);
    assert!(bundle
        .basis_binding_digest()
        .expect("split target binding digest should derive from semantic target evidence")
        .starts_with("bridge-continuity-mutation-binding-basis:sha256:"));
    assert!(bundle
        .lineage_digest()
        .starts_with("bridge-continuity-mutation-lineage:sha256:"));
    assert!(bundle
        .continuity_resolution_digest()
        .starts_with("bridge-continuity-mutation-resolution:sha256:"));
}

#[test]
fn split_successor_bundle_rejects_singleton_successor_set() {
    let error = crate::facade::BridgeContinuityMutationBundle::split_existing_target(
        crate::continuity::BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors,
        authoritative_identity("authority:task-1"),
        [authoritative_identity("authority:task-1:a")],
        Some(resolved_target_identity("entity:task-1")),
        Some(
            BridgeContinuityTargetCollection::new("Task")
                .expect("test continuity target collection should be native"),
        ),
    )
    .expect_err("split continuity bundle should reject singleton successor sets");

    assert_eq!(
        error.to_string(),
        "split-successor continuity requires at least two successor authoritative identities"
    );
}

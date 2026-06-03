use super::support::*;

#[test]
fn split_successor_bundle_preserves_successor_set() {
    let bundle = crate::facade::BridgeContinuityMutationBundle::split_existing_target(
        crate::continuity::BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors,
        BridgeContinuityAuthoritativeIdentity::new("authority:task-1")
            .expect("test continuity authoritative identity should be native"),
        [
            BridgeContinuityAuthoritativeIdentity::new("authority:task-1:a")
                .expect("test continuity authoritative identity should be native"),
            BridgeContinuityAuthoritativeIdentity::new("authority:task-1:b")
                .expect("test continuity authoritative identity should be native"),
        ],
        Some(
            BridgeContinuityResolvedTargetIdentity::new("entity:task-1")
                .expect("test continuity resolved target should be native"),
        ),
        Some(
            BridgeContinuityTargetCollection::new("Task")
                .expect("test continuity target collection should be native"),
        ),
    )
    .expect("split continuity bundle should build");

    assert_eq!(
        bundle.successor_authoritative_identities(),
        [
            BridgeContinuityAuthoritativeIdentity::new("authority:task-1:a")
                .expect("test continuity authoritative identity should be native"),
            BridgeContinuityAuthoritativeIdentity::new("authority:task-1:b")
                .expect("test continuity authoritative identity should be native"),
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
        BridgeContinuityAuthoritativeIdentity::new("authority:task-1")
            .expect("test continuity authoritative identity should be native"),
        [
            BridgeContinuityAuthoritativeIdentity::new("authority:task-1:a")
                .expect("test continuity authoritative identity should be native"),
        ],
        Some(
            BridgeContinuityResolvedTargetIdentity::new("entity:task-1")
                .expect("test continuity resolved target should be native"),
        ),
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

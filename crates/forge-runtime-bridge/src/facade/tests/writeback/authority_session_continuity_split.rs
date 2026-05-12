#[test]
fn split_successor_bundle_preserves_successor_set() {
    let bundle = crate::facade::BridgeContinuityMutationBundle::split_existing_target(
        crate::continuity::BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors,
        "authority:task-1",
        ["authority:task-1:a", "authority:task-1:b"],
        Some("binding:sha256:task-1"),
        Some("entity:task-1"),
        Some("Task"),
        "lineage:sha256:task-1",
        "continuity:sha256:task-1",
    )
    .expect("split continuity bundle should build");

    assert_eq!(
        bundle.successor_authoritative_identities(),
        &[
            std::sync::Arc::<str>::from("authority:task-1:a"),
            std::sync::Arc::<str>::from("authority:task-1:b")
        ]
    );
    assert_eq!(bundle.successor_authoritative_identity(), None);
    assert_eq!(bundle.basis_binding_digest(), Some("binding:sha256:task-1"));
}

#[test]
fn split_successor_bundle_rejects_singleton_successor_set() {
    let error = crate::facade::BridgeContinuityMutationBundle::split_existing_target(
        crate::continuity::BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors,
        "authority:task-1",
        ["authority:task-1:a"],
        Some("binding:sha256:task-1"),
        Some("entity:task-1"),
        Some("Task"),
        "lineage:sha256:task-1",
        "continuity:sha256:task-1",
    )
    .expect_err("split continuity bundle should reject singleton successor sets");

    assert_eq!(
        error.to_string(),
        "split-successor continuity requires at least two successor authoritative identities"
    );
}

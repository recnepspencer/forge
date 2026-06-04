use super::*;
use crate::facade::TruthSnapshotIdentity;

#[test]
fn bridge_prepared_delivery_is_equivalent_to_one_shot_delivery() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    left_source.insert_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    right_source.insert_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let one_shot = left_runtime
        .deliver_invalidation(
            left_runtime
                .plan_committed_patch(BridgeRouteRequest::for_commit(
                    crate::facade::TruthCommitIdentity::new("commit-a"),
                ))
                .expect("one-shot route should plan"),
        )
        .expect("one-shot delivery should succeed");
    let prepared = right_runtime.prepare_delivery(
        right_runtime
            .plan_committed_patch(BridgeRouteRequest::for_commit(
                crate::facade::TruthCommitIdentity::new("commit-a"),
            ))
            .expect("prepared route should plan"),
    );
    let staged = right_runtime
        .deliver_prepared(prepared)
        .expect("prepared delivery should succeed");

    assert_eq!(
        one_shot.result_summary().route_identity(),
        staged.result_summary().route_identity()
    );
    assert_eq!(
        one_shot.result_summary().invalidation_identity(),
        staged.result_summary().invalidation_identity()
    );
    assert_eq!(
        one_shot.result_summary().subscription_slice_identity(),
        staged.result_summary().subscription_slice_identity()
    );
    assert_eq!(one_shot.counters(), staged.counters());
}

#[test]
fn bridge_empty_mapping_context_is_equivalent_to_default_planning_path() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    left_source.insert_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    right_source.insert_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let default_route = left_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit(
            crate::facade::TruthCommitIdentity::new("commit-a"),
        ))
        .expect("default planning should succeed");
    let explicit_route = right_runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit(crate::facade::TruthCommitIdentity::new("commit-a")),
            BridgeMappingContext::empty(),
        )
        .expect("explicit empty mapping context planning should succeed");

    assert_eq!(
        default_route.route_identity(),
        explicit_route.route_identity()
    );
    assert_eq!(
        default_route.source_digest(),
        explicit_route.source_digest()
    );
    assert_eq!(
        default_route.planning_provenance().digest(),
        explicit_route.planning_provenance().digest()
    );
    assert_eq!(
        default_route.lowering_provenance().digest(),
        explicit_route.lowering_provenance().digest()
    );
    assert_eq!(default_route.read_packet(), explicit_route.read_packet());
    assert_eq!(default_route.counters(), explicit_route.counters());
}

#[test]
fn bridge_route_identity_is_stable_across_equivalent_native_field_constructors() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch_items(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        vec![crate::facade::BridgeCommittedPatchItem::with_target(
            "user",
            crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                forge_foundational::facade::AspectLocator::new(
                    forge_foundational::facade::LocatorAuthority::Authoritative,
                    forge_foundational::facade::AspectKey::new("profile")
                        .expect("valid bridge patch aspect key"),
                ),
                forge_foundational::facade::CanonicalFieldPath::single(
                    forge_foundational::facade::FieldKey::new("name".to_owned())
                        .expect("valid foundational field key"),
                ),
            ),
        )],
    ));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let left_route = left_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit(
            crate::facade::TruthCommitIdentity::new("commit-a"),
        ))
        .expect("unprefixed field route should plan");
    let right_route = right_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit(
            crate::facade::TruthCommitIdentity::new("commit-a"),
        ))
        .expect("explicit native field route should plan");

    assert_eq!(left_route.route_identity(), right_route.route_identity());
    assert_eq!(left_route.read_packet(), right_route.read_packet());
    assert_eq!(
        left_route.lowering_summary().subscription_slice_identity(),
        right_route.lowering_summary().subscription_slice_identity()
    );
}

#[test]
fn bridge_route_identity_is_stable_when_patch_items_arrive_out_of_order_with_duplicates() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-canonical-patch-order",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch_items(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                vec![
                    crate::facade::BridgeCommittedPatchItem::with_target(
                        "user",
                        crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                            forge_foundational::facade::AspectLocator::new(
                                forge_foundational::facade::LocatorAuthority::Authoritative,
                                forge_foundational::facade::AspectKey::new("profile")
                                    .expect("valid bridge patch aspect key"),
                            ),
                            forge_foundational::facade::CanonicalFieldPath::single(
                                forge_foundational::facade::FieldKey::new("name".to_owned())
                                    .expect("valid foundational field key"),
                            ),
                        ),
                    ),
                    crate::facade::BridgeCommittedPatchItem::with_target(
                        "user",
                        crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                            forge_foundational::facade::AspectLocator::new(
                                forge_foundational::facade::LocatorAuthority::Authoritative,
                                forge_foundational::facade::AspectKey::new("profile")
                                    .expect("valid bridge patch aspect key"),
                            ),
                            forge_foundational::facade::CanonicalFieldPath::single(
                                forge_foundational::facade::FieldKey::new("name".to_owned())
                                    .expect("valid foundational field key"),
                            ),
                        ),
                    ),
                    crate::facade::BridgeCommittedPatchItem::with_target(
                        "user",
                        crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                            forge_foundational::facade::AspectLocator::new(
                                forge_foundational::facade::LocatorAuthority::Authoritative,
                                forge_foundational::facade::AspectKey::new("profile")
                                    .expect("valid bridge patch aspect key"),
                            ),
                            forge_foundational::facade::CanonicalFieldPath::single(
                                forge_foundational::facade::FieldKey::new("name".to_owned())
                                    .expect("valid foundational field key"),
                            ),
                        ),
                    ),
                ],
            ))
            .with_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target(
        "deliver-commit-a",
        BridgeHarnessTargetId::committed_route(crate::facade::TruthCommitIdentity::new("commit-a")),
    );
    let profile = ExecutionProfile::development("development");

    let mut left = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut left, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut left, &fixture)
        .expect("bridge harness load fixture");
    adapter
        .execute(&mut left, &fixture, &request, &profile)
        .expect("bridge harness execute");
    let left_route_identity = left
        .runtime
        .as_ref()
        .expect("left bridge runtime")
        .diagnostics()
        .last_route_record()
        .expect("left route record")
        .route_identity()
        .clone();

    let reordered_fixture = ScenarioPlan::new(
        "bridge-canonical-patch-order-reordered",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch_items(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                vec![
                    crate::facade::BridgeCommittedPatchItem::with_target(
                        "user",
                        crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                            forge_foundational::facade::AspectLocator::new(
                                forge_foundational::facade::LocatorAuthority::Authoritative,
                                forge_foundational::facade::AspectKey::new("profile")
                                    .expect("valid bridge patch aspect key"),
                            ),
                            forge_foundational::facade::CanonicalFieldPath::single(
                                forge_foundational::facade::FieldKey::new("name".to_owned())
                                    .expect("valid foundational field key"),
                            ),
                        ),
                    ),
                    crate::facade::BridgeCommittedPatchItem::with_target(
                        "user",
                        crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                            forge_foundational::facade::AspectLocator::new(
                                forge_foundational::facade::LocatorAuthority::Authoritative,
                                forge_foundational::facade::AspectKey::new("profile")
                                    .expect("valid bridge patch aspect key"),
                            ),
                            forge_foundational::facade::CanonicalFieldPath::single(
                                forge_foundational::facade::FieldKey::new("name".to_owned())
                                    .expect("valid foundational field key"),
                            ),
                        ),
                    ),
                ],
            ))
            .with_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let mut right = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut right, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut right, &reordered_fixture)
        .expect("bridge harness load fixture");
    adapter
        .execute(&mut right, &reordered_fixture, &request, &profile)
        .expect("bridge harness execute");
    let right_route_identity = right
        .runtime
        .as_ref()
        .expect("right bridge runtime")
        .diagnostics()
        .last_route_record()
        .expect("right route record")
        .route_identity()
        .clone();

    assert_eq!(left_route_identity, right_route_identity);
}

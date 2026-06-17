use super::*;

#[test]
fn bridge_replay_capture_exposes_last_route_record() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-replay",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                "alice",
            )),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target(
        "deliver-commit-a",
        BridgeHarnessTargetId::committed_route(
            crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        ),
    );
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    let run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("bridge harness execute");
    let canonical_record = session
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .diagnostics()
        .last_canonical_route_record()
        .expect("bridge should retain typed route record before replay capture");
    let replay = adapter
        .capture_replay(
            &session,
            &fixture,
            &ReplayRequest {
                name: "replay".to_string(),
                source_run: run,
                request: request.clone(),
                profile: profile.clone(),
            },
        )
        .expect("bridge replay capture should succeed");
    let typed_replay = session
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .replay_canonical_record(&canonical_record)
        .expect("typed route replay should succeed from retained route record");

    assert_eq!(
        typed_replay.source_commit().as_str(),
        crate::truth_identity_fixtures::truth_commit_fixture("commit-a").as_str()
    );
    assert_eq!(
        typed_replay.source_snapshot().as_str(),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a").as_str()
    );
    assert_eq!(replay.requested_targets, request.targets);
}

#[test]
fn bridge_replay_accepts_versioned_canonical_route_record() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch(
        crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(snapshot(
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        "alice",
    ));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit(
            crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        ))
        .expect("bridge should plan route before canonical replay capture");
    runtime
        .deliver_invalidation(route)
        .expect("bridge should deliver route before canonical replay capture");
    let canonical_record = runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("bridge should expose a versioned canonical route record");

    let replay = runtime
        .replay_canonical_record(&canonical_record)
        .expect("bridge should replay a supported canonical route record");

    assert_eq!(
        replay.source_commit().as_str(),
        crate::truth_identity_fixtures::truth_commit_fixture("commit-a").as_str()
    );
    assert_eq!(
        canonical_record.schema_version(),
        crate::facade::BRIDGE_CANONICAL_ROUTE_RECORD_SCHEMA_V3
    );
}

#[test]
fn bridge_replay_preserves_canonical_route_outcome_for_delivered_patch() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch(
        crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(field_slice_snapshot(
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        "alice",
    ));
    let runtime = build_runtime_with_aspects(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let result = runtime
        .deliver_invalidation(
            runtime
                .plan_committed_patch(BridgeRouteRequest::for_commit(
                    crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                ))
                .expect("route should plan before replay parity certification"),
        )
        .expect("route should deliver before replay parity certification");
    let canonical_record = runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("bridge should retain a canonical route record for replay parity certification");

    let replay = runtime
        .replay_canonical_record(&canonical_record)
        .expect("bridge replay should preserve the canonical route outcome");

    assert_eq!(
        replay.route_identity(),
        result.result_summary().route_identity()
    );
    assert_eq!(
        replay.invalidation_identity(),
        result.result_summary().invalidation_identity()
    );
    assert_eq!(
        replay.subscription_slice_identity(),
        result.result_summary().subscription_slice_identity()
    );
    assert_eq!(
        replay.source_commit(),
        result.result_summary().source_commit()
    );
    assert_eq!(
        replay.source_patch(),
        result.result_summary().source_patch()
    );
    assert_eq!(
        replay.source_snapshot(),
        result.result_summary().snapshot_identity()
    );
}

#[test]
fn bridge_replay_rejects_subscription_slice_drift() {
    let original_source = InMemoryRelationalBridgeSource::default();
    original_source.insert_committed_patch(committed_patch(
        crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    original_source.insert_snapshot(field_slice_snapshot(
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        "alice",
    ));
    let original_runtime = build_runtime_with_aspects(
        original_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let route = original_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit(
            crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        ))
        .expect("original route should plan before replay certification");
    original_runtime
        .deliver_invalidation(route)
        .expect("original route should deliver before replay certification");
    let canonical_record = original_runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("original runtime should expose a canonical route record");

    let restarted_source = InMemoryRelationalBridgeSource::default();
    restarted_source.insert_committed_patch(committed_patch(
        crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    restarted_source.insert_snapshot(field_slice_snapshot(
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        "alice",
    ));
    let restarted_runtime = build_runtime_with_aspects(
        restarted_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration_with_kind(
            "profile-name-region",
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalRegion,
        )],
    );

    let error = restarted_runtime
        .replay_canonical_record(&canonical_record)
        .expect_err("replay should reject subscription slice identity drift");
    let canonical_route_record = canonical_record
        .decode()
        .expect("test canonical route record should decode");

    assert_eq!(
        error.kind(),
        crate::facade::BridgeReplayErrorKind::SubscriptionSliceMismatch
    );
    assert_eq!(
        error.context().route_identity(),
        Some(canonical_route_record.route_identity())
    );
    assert_eq!(
        error.context().snapshot_identity(),
        Some(canonical_route_record.source_snapshot())
    );
    assert_eq!(
        error.context().subscription_slice_identity(),
        Some(canonical_route_record.subscription_slice_identity())
    );
}

#[test]
fn bridge_replay_detects_route_drift_after_restart_shaped_truth_change() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-replay-restart-drift",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                "alice",
            )),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target(
        "deliver-commit-a",
        BridgeHarnessTargetId::committed_route(
            crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        ),
    );
    let profile = ExecutionProfile::development("development");

    let mut original = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut original, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut original, &fixture)
        .expect("bridge harness load fixture");
    adapter
        .execute(&mut original, &fixture, &request, &profile)
        .expect("bridge harness execute");
    let original_record = original
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .diagnostics()
        .last_canonical_route_record()
        .expect("original canonical bridge route record");

    let drifted_fixture = ScenarioPlan::new(
        "bridge-replay-restart-drift-rehydrated",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch_items(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
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
                                forge_foundational::facade::FieldKey::new("avatar".to_owned())
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
            .with_snapshot(snapshot(
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                "alice",
            )),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let mut restarted = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut restarted, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut restarted, &drifted_fixture)
        .expect("bridge harness load fixture");
    let error = restarted
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .replay_canonical_record(&original_record)
        .expect_err("bridge replay should reject route drift after restart");
    let original_route_record = original_record
        .decode()
        .expect("original canonical route record should decode");

    assert_eq!(
        error.kind(),
        crate::facade::BridgeReplayErrorKind::RouteMismatch
    );
    assert_eq!(
        error.context().route_identity(),
        Some(original_route_record.route_identity())
    );
    assert_eq!(
        error.context().snapshot_identity(),
        Some(original_route_record.source_snapshot())
    );
    let failure_record = restarted
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .diagnostics()
        .last_failure_record()
        .expect("bridge replay failure record");
    assert_eq!(failure_record.counters().route_replay_mismatch_count(), 1);
}

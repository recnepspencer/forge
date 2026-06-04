use super::*;

#[test]
fn bridge_snapshot_delivery_remains_stable_after_newer_truth_arrives() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-snapshot-stability",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let mutation = MutationBatch::new("publish-newer-truth")
        .push(BridgeHarnessMutation::PublishCommittedPatch(
            committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-b"),
                crate::facade::TruthPatchIdentity::new("patch-b"),
                TruthSnapshotIdentity::new("snapshot-b"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ),
        ))
        .push(BridgeHarnessMutation::PublishSnapshot(snapshot(
            TruthSnapshotIdentity::new("snapshot-b"),
            "bob",
        )));
    let request = ExecutionRequest::target(
        "deliver-commit-a",
        BridgeHarnessTargetId::committed_route(crate::facade::TruthCommitIdentity::new("commit-a")),
    );
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    adapter
        .apply_mutation_batch(&mut session, &mutation)
        .expect("bridge harness mutation should publish newer truth");
    adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("bridge snapshot-stability execution should succeed");
    let route_record = session
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .diagnostics()
        .last_route_record()
        .expect("bridge should retain typed route record");

    assert_eq!(route_record.source_snapshot().as_str(), "snapshot-a");
}

#[test]
fn bridge_delivery_keeps_preplanned_snapshot_after_newer_truth_arrives_during_delivery() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_runtime(source.clone(), sink.clone(), vec![registration()]);

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit(
            crate::facade::TruthCommitIdentity::new("commit-a"),
        ))
        .expect("bridge should plan from the original committed artifact");

    source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-b"),
        crate::facade::TruthPatchIdentity::new("patch-b"),
        TruthSnapshotIdentity::new("snapshot-b"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-b"), "bob"));

    let result = runtime
        .deliver_invalidation(route)
        .expect("bridge should deliver the preplanned route against its original snapshot");

    assert_eq!(
        result.result_summary().snapshot_identity().as_str(),
        "snapshot-a"
    );
    assert_eq!(result.receipt().snapshot_identity().as_str(), "snapshot-a");
    let delivered = sink
        .last_delivery()
        .expect("bridge sink should record the delivered artifact");
    assert_eq!(delivered.delivery.source_snapshot().as_str(), "snapshot-a");
}

#[test]
fn bridge_prepares_signal_evaluation_with_snapshot_context_without_sink_delivery() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_runtime(source, sink.clone(), vec![registration()]);

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit(
            crate::facade::TruthCommitIdentity::new("commit-a"),
        ))
        .expect("bridge should plan the route");
    let evaluation = runtime
        .prepare_signal_evaluation(route)
        .expect("bridge should prepare signal evaluation");

    assert_eq!(
        evaluation.snapshot().snapshot_identity().as_str(),
        "snapshot-a"
    );
    assert!(sink.last_delivery().is_none());
}

#[test]
fn bridge_prepared_signal_evaluation_keeps_preplanned_snapshot_after_newer_truth_arrives() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_runtime(source.clone(), sink, vec![registration()]);

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit(
            crate::facade::TruthCommitIdentity::new("commit-a"),
        ))
        .expect("bridge should plan the route");

    source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-b"),
        crate::facade::TruthPatchIdentity::new("patch-b"),
        TruthSnapshotIdentity::new("snapshot-b"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-b"), "bob"));

    let evaluation = runtime
        .prepare_signal_evaluation(route)
        .expect("bridge should prepare signal evaluation");

    assert_eq!(
        evaluation.snapshot().snapshot_identity().as_str(),
        "snapshot-a"
    );
}

#[test]
fn bridge_snapshot_identity_mismatch_fails_explicitly() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-snapshot-mismatch",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(
                snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice")
                    .with_read_result_identity(TruthSnapshotIdentity::new("snapshot-bad")),
            ),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target(
        "deliver-commit-a",
        BridgeHarnessTargetId::committed_route(crate::facade::TruthCommitIdentity::new("commit-a")),
    );
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    let _error = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect_err("bridge execution should fail on snapshot identity mismatch");

    let failure_record = session
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .diagnostics()
        .last_failure_record()
        .expect("bridge failure record");
    assert_eq!(
        failure_record.failure_class(),
        &BridgeFailureClass::Delivery(BridgeDeliveryErrorKind::SnapshotIdentityMismatch)
    );
    assert_eq!(
        failure_record
            .context()
            .snapshot_identity()
            .map(|id| id.as_str()),
        Some("snapshot-a")
    );
    assert_eq!(
        failure_record.counters().snapshot_identity_mismatch_count(),
        1
    );
}

#[test]
fn bridge_snapshot_contract_rejects_missing_required_reads() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(SnapshotFixture::new(
        TruthSnapshotIdentity::new("snapshot-a"),
        vec![],
    ));
    let runtime = build_runtime_with_aspects(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit(
            crate::facade::TruthCommitIdentity::new("commit-a"),
        ))
        .expect("bridge should plan before validating snapshot reads");
    let expected_target_identity = route.read_packet().reads()[0].target_identity().clone();

    let error = runtime
        .deliver_invalidation(route)
        .expect_err("bridge should reject incomplete snapshot read results");

    assert_eq!(
        error.kind(),
        BridgeDeliveryErrorKind::SnapshotReadContractViolation
    );
    assert_eq!(
        error.context().snapshot_identity().map(|id| id.as_str()),
        Some("snapshot-a")
    );
    let snapshot_read = error
        .context()
        .snapshot_read()
        .expect("snapshot contract violation should retain read coordinate");
    assert_eq!(snapshot_read.entity_identity(), "user");
    assert_eq!(snapshot_read.aspect_key().as_str(), "profile");
    assert_eq!(
        snapshot_read
            .target_identity()
            .expect("subscription-slice read coordinate should retain target identity"),
        &expected_target_identity
    );
    assert!(snapshot_read
        .target_identity()
        .expect("subscription-slice read coordinate should retain target identity")
        .as_str()
        .starts_with("snapshot-read-target:sha256:"));
}

#[test]
fn bridge_delivery_fails_when_newer_truth_arrives_without_required_snapshot() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    let runtime = build_runtime(
        source.clone(),
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit(
            crate::facade::TruthCommitIdentity::new("commit-a"),
        ))
        .expect("bridge should plan the route");

    source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-b"),
        crate::facade::TruthPatchIdentity::new("patch-b"),
        TruthSnapshotIdentity::new("snapshot-b"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-b"), "bob"));

    let error = runtime
        .deliver_invalidation(route)
        .expect_err("delivery should still require the original planned snapshot");

    assert_eq!(
        error.kind(),
        BridgeDeliveryErrorKind::SnapshotAcquisitionFailure
    );
    assert_eq!(
        error.context().snapshot_identity().map(|id| id.as_str()),
        Some("snapshot-a")
    );
}

#[test]
fn bridge_snapshot_reader_pool_is_used_when_configured() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice"));
    let pool = CountingSnapshotReaderPool::new(source.clone());
    let runtime = RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_snapshot_reader_pool(pool.clone())
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .register_mapping(registration())
        .build()
        .expect("bridge runtime should build with a snapshot reader pool");

    runtime
        .deliver_invalidation(
            runtime
                .plan_committed_patch(BridgeRouteRequest::for_commit(
                    crate::facade::TruthCommitIdentity::new("commit-a"),
                ))
                .expect("bridge should plan the route"),
        )
        .expect("bridge delivery should succeed");

    assert_eq!(pool.acquire_count(), 1);
    assert_eq!(pool.release_count(), 1);
}

use crate::tests::support::*;

#[test]
fn durable_log_compaction_respects_checkpoint_policy() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .durable_log_policy(DurableLogPolicy {
            retention_mode: DurableLogRetentionMode::CompactAfterCheckpoint,
            max_in_memory_envelopes: 1,
            compact_after_checkpoint: true,
        })
        .build();

    create_entity(&mut runtime, "first");
    runtime.checkpoint().unwrap();
    create_entity(&mut runtime, "second");
    create_entity(&mut runtime, "third");

    assert!(runtime.durable_log().len() <= 1);
}

#[test]
fn relation_payload_history_is_removed_after_reclaim() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .mvcc(crate::facade::MvccConfig {
            track_visibility_metadata: true,
            snapshot_release_policy: crate::facade::SnapshotReleasePolicy::ExplicitRelease,
            auto_reclaim_deleted_records: true,
            reclaim_batch_size: 32,
            retention_backend: crate::facade::RetentionBackend::PinTrackedRetention,
        })
        .build();
    let source_outcome = create_entity_outcome(&mut runtime, "source");
    let target_outcome = create_entity_outcome(&mut runtime, "target");
    let source = changed_entities(&source_outcome)[0];
    let target = changed_entities(&target_outcome)[0];
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(WorkerIntentBatch::new("payload-bearing-relation").push(
        TransactionIntent::CreateRelation(crate::transactions::data::RelationSpec {
            partition_id: PartitionId::main(),
            kind_id: KindId(2),
            client_key: InternedString::Raw("r1".to_string()),
            source,
            target,
            payload: Some(RecordPayload::StructuredJson(
                json!({"weight": 1, "name": "r1"}),
            )),
        }),
    ));
    let created = txn.commit().unwrap();
    let relation = changed_relations(&created)[0];
    assert_eq!(runtime.relation_history_len_for_test(relation), 1);

    let mut delete_txn = runtime.begin_transaction(TransactionOptions::default());
    delete_txn.push_batch(WorkerIntentBatch::new("delete-relation").push(
        TransactionIntent::DeleteRelation {
            relation_id: relation,
        },
    ));
    let deleted = delete_txn.commit().unwrap();
    runtime.release_snapshot(&source_outcome.snapshot);
    runtime.release_snapshot(&target_outcome.snapshot);
    runtime.release_snapshot(&created.snapshot);
    runtime.release_snapshot(&deleted.snapshot);
    runtime.run_retention_pass();

    assert_eq!(runtime.relation_history_len_for_test(relation), 0);
}

#[test]
fn visibility_cache_zero_window_does_not_accumulate_unprotected_history() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .visibility_cache_policy(VisibilityCachePolicy {
            enabled: true,
            protect_branch_heads: false,
            protect_replay_retained: false,
            protect_active_snapshots: true,
            recent_version_window: 0,
        })
        .build();

    let first = create_entity_outcome(&mut runtime, "first");
    let second = create_entity_outcome(&mut runtime, "second");

    runtime.reset_complexity_counters();
    let _ = runtime.read_version(first.version_id);
    let _ = runtime.read_version(first.version_id);
    let stats = runtime.storage_stats();
    let counters = runtime.complexity_counters();

    assert_eq!(stats.recent_visibility_cache_count, 0);
    assert_eq!(stats.cached_visibility_version_count, 0);
    assert_eq!(stats.protected_visibility_version_count, 0);
    assert_eq!(counters.visibility_cache_hits, 0);
    assert!(counters.visibility_cache_miss_reconstructions >= 2);
    assert_eq!(second.version_id.0, 2);
}

#[test]
fn explicit_snapshots_can_skip_cache_protection_and_still_read_until_release() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .visibility_cache_policy(VisibilityCachePolicy {
            enabled: true,
            protect_branch_heads: false,
            protect_replay_retained: false,
            protect_active_snapshots: false,
            recent_version_window: 0,
        })
        .build();

    let first = create_entity_outcome(&mut runtime, "first");
    let snapshot = runtime.snapshot();
    let entity = changed_entities(&first)[0];
    let _updated = update_entity(&mut runtime, entity, "first-updated");

    let read = runtime.read_snapshot(&snapshot).unwrap();
    let inspection = runtime.inspect_snapshot(&snapshot).unwrap();
    let stats = runtime.storage_stats();

    assert_eq!(
        read.get_entity(entity).unwrap().payload,
        RecordPayload::StructuredJson(json!({"name":"first"}))
    );
    assert_eq!(inspection.pinned_entity_count, 1);
    assert_eq!(stats.snapshot_count, 1);
    assert_eq!(stats.cached_visibility_version_count, 0);
    assert_eq!(stats.protected_visibility_version_count, 0);

    assert!(runtime.release_snapshot(&snapshot));
    assert!(runtime.read_snapshot(&snapshot).is_none());
}

#[test]
fn unprotected_active_snapshots_can_use_recent_cache_when_enabled() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .visibility_cache_policy(VisibilityCachePolicy {
            enabled: true,
            protect_branch_heads: false,
            protect_replay_retained: false,
            protect_active_snapshots: false,
            recent_version_window: 1,
        })
        .build();

    let first = create_entity_outcome(&mut runtime, "first");
    let snapshot = runtime.snapshot();
    let entity = changed_entities(&first)[0];
    let _updated = update_entity(&mut runtime, entity, "first-updated");

    runtime.reset_complexity_counters();
    let _ = runtime.read_snapshot(&snapshot).unwrap();
    let _ = runtime.read_snapshot(&snapshot).unwrap();

    let stats = runtime.storage_stats();
    let counters = runtime.complexity_counters();

    assert_eq!(stats.snapshot_count, 1);
    assert_eq!(stats.protected_visibility_version_count, 0);
    assert_eq!(stats.recent_visibility_cache_count, 1);
    assert_eq!(stats.cached_visibility_version_count, 1);
    assert!(counters.visibility_cache_miss_reconstructions >= 1);
    assert!(counters.visibility_cache_hits >= 1);
}

#[test]
fn visibility_cache_recent_window_is_bounded_and_reports_hits() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .visibility_cache_policy(VisibilityCachePolicy {
            enabled: true,
            protect_branch_heads: true,
            protect_replay_retained: true,
            protect_active_snapshots: true,
            recent_version_window: 1,
        })
        .build();

    let first = create_entity_outcome(&mut runtime, "first");
    let second = create_entity_outcome(&mut runtime, "second");
    let third = create_entity_outcome(&mut runtime, "third");

    runtime.reset_complexity_counters();
    let _ = runtime.read_version(first.version_id);
    let _ = runtime.read_version(second.version_id);
    let _ = runtime.read_version(second.version_id);
    let stats = runtime.storage_stats();
    let counters = runtime.complexity_counters();

    assert_eq!(stats.recent_visibility_cache_count, 1);
    assert_eq!(stats.protected_visibility_version_count, 1);
    assert!(stats.cached_visibility_version_count <= 2);
    assert!(counters.visibility_cache_miss_reconstructions >= 2);
    assert!(counters.visibility_cache_hits >= 1);
    assert!(counters.visibility_cache_recent_evictions >= 1);
    assert_eq!(
        third.version_id,
        runtime.latest_commit().unwrap().version_id
    );
}

#[test]
fn heavy_profiles_keep_recent_visibility_cache_small_under_sustained_history_reads() {
    let mut runtime = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::ChipSimulation)
        .schema_registry(test_schema_registry())
        .build();
    let mut versions = Vec::new();
    for index in 0..6 {
        versions.push(create_entity_outcome(&mut runtime, &format!("e{index}")).version_id);
    }

    runtime.reset_complexity_counters();
    for version_id in &versions[..versions.len() - 1] {
        let _ = runtime.read_version(*version_id);
    }
    let stats = runtime.storage_stats();

    assert_eq!(
        runtime
            .config()
            .visibility_cache_policy
            .recent_version_window,
        2
    );
    assert_eq!(stats.recent_visibility_cache_count, 2);
    assert!(stats.cached_visibility_version_count <= 3);
}

#[test]
fn branch_head_visibility_updates_incrementally_under_branch_churn() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .visibility_cache_policy(VisibilityCachePolicy {
            enabled: true,
            protect_branch_heads: true,
            protect_replay_retained: false,
            protect_active_snapshots: false,
            recent_version_window: 0,
        })
        .build();

    let base = create_entity_outcome(&mut runtime, "base");
    let entity = changed_entities(&base)[0];
    runtime
        .create_branch(
            BranchId("analysis".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();

    runtime.reset_complexity_counters();
    for revision in 0..3 {
        let _ = update_entity(&mut runtime, entity, &format!("base-r{revision}"));
    }

    let stats = runtime.storage_stats();
    let counters = runtime.complexity_counters();

    assert_eq!(stats.protected_visibility_version_count, 2);
    assert_eq!(stats.cached_visibility_version_count, 2);
    assert_eq!(stats.recent_visibility_cache_count, 0);
    assert_eq!(counters.visibility_cache_branch_head_promotions, 3);
}

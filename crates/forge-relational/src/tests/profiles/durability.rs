use crate::diagnostics::data::RelationalDiagnosticValue;
use crate::facade::config::{MvccConfig, RetentionBackend, SnapshotReleasePolicy};
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
    runtime.durability_authority().checkpoint().unwrap();
    create_entity(&mut runtime, "second");
    create_entity(&mut runtime, "third");

    assert!(runtime.durability().durable_log().len() <= 1);
}

#[test]
fn relation_aspect_history_remains_available_for_historical_reads_after_reclaim() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .mvcc(MvccConfig {
            track_visibility_metadata: true,
            snapshot_release_policy: SnapshotReleasePolicy::ExplicitRelease,
            auto_reclaim_deleted_records: true,
            reclaim_batch_size: 32,
            retention_backend: RetentionBackend::PinTrackedRetention,
        })
        .build();
    let source_outcome = create_entity_outcome(&mut runtime, "source");
    let target_outcome = create_entity_outcome(&mut runtime, "target");
    let source = changed_entities(&source_outcome)[0];
    let target = changed_entities(&target_outcome)[0];
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("aspect-bearing-relation").push(MutationIntent::Create(
            CreateIntent::Relation(crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: crate::symbols::data::ClientKey::raw("r1"),
                source: crate::transactions::data::EntityReference::Existing(source),
                target: crate::transactions::data::EntityReference::Existing(target),
                fields: relation_label_field_patch("r1"),
            }),
        )),
    );
    let created = txn.commit().unwrap();
    let relation = changed_relations(&created)[0];
    assert_eq!(runtime.relation_history_len_for_test(relation), 1);

    let mut delete_txn = runtime.begin_transaction(TransactionOptions::default());
    delete_txn.push_batch(WorkerIntentBatch::new("delete-relation").push(
        MutationIntent::Relation(RelationMutationIntent::Delete(DeleteRelationIntent {
            relation_id: relation,
        })),
    ));
    let deleted = delete_txn.commit().unwrap();
    runtime
        .visibility_authority()
        .release_snapshot(&source_outcome.snapshot);
    runtime
        .visibility_authority()
        .release_snapshot(&target_outcome.snapshot);
    runtime
        .visibility_authority()
        .release_snapshot(&created.snapshot);
    runtime
        .visibility_authority()
        .release_snapshot(&deleted.snapshot);
    let _ = runtime.retention().run_pass();

    assert_eq!(runtime.relation_history_len_for_test(relation), 1);
    let historical = runtime.read_truth().read_version(created.version_id);
    let relation_record = historical
        .relations
        .iter()
        .find(|record| record.relation_id == relation)
        .expect("retained relation aspect record");
    assert_eq!(
        read_relation_field(relation_record, field_key("label")),
        Some("r1".into())
    );
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

    runtime.performance_access().reset_counters();
    let _ = runtime.read_truth().read_version(first.version_id);
    let _ = runtime.read_truth().read_version(first.version_id);
    let stats = runtime.storage_access().storage_stats();
    let counters = runtime.performance_access().counters();

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
    let snapshot = runtime.visibility_authority().snapshot();
    let entity = changed_entities(&first)[0];
    let _updated = update_entity(&mut runtime, entity, "first-updated");

    let read_path = runtime
        .read_truth()
        .inspect_snapshot_read_path(&snapshot)
        .unwrap();
    assert!(read_path
        .entries
        .iter()
        .any(|entry| entry.code == DiagnosticCode::VisibilityCacheTransientRead));
    assert!(read_path.entries.iter().any(|entry| {
        entry.code == DiagnosticCode::SnapshotReadPathInspected
            && diagnostic_field(entry, "recent_candidate")
                == &RelationalDiagnosticValue::Bool(false)
    }));

    let read = runtime.read_truth().read_snapshot(&snapshot).unwrap();
    let inspection = runtime.read_truth().inspect_snapshot(&snapshot).unwrap();
    let stats = runtime.storage_access().storage_stats();

    assert_eq!(
        read_entity_field(read.get_entity(entity).unwrap(), field_key("name")),
        Some("first".into())
    );
    assert_eq!(inspection.pinned_entity_count, 1);
    assert_eq!(stats.snapshot_count, 1);
    assert_eq!(stats.cached_visibility_version_count, 0);
    assert_eq!(stats.protected_visibility_version_count, 0);

    assert!(runtime.visibility_authority().release_snapshot(&snapshot));
    assert!(runtime.read_truth().read_snapshot(&snapshot).is_none());
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
    let snapshot = runtime.visibility_authority().snapshot();
    let entity = changed_entities(&first)[0];
    let _updated = update_entity(&mut runtime, entity, "first-updated");

    runtime.performance_access().reset_counters();
    let _ = runtime.read_truth().read_snapshot(&snapshot).unwrap();
    let _ = runtime.read_truth().read_snapshot(&snapshot).unwrap();

    let stats = runtime.storage_access().storage_stats();
    let counters = runtime.performance_access().counters();

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

    let first_read_path = runtime
        .read_truth()
        .inspect_version_read_path(first.version_id)
        .unwrap();
    assert!(first_read_path
        .entries
        .iter()
        .any(|entry| entry.code == DiagnosticCode::VisibilityCacheMissReconstructed));
    assert!(first_read_path.entries.iter().any(|entry| {
        entry.code == DiagnosticCode::VisibilityCacheRecentAdmissionCandidate
            && diagnostic_field(entry, "recent_admission_candidate")
                == &RelationalDiagnosticValue::Bool(true)
    }));

    runtime.performance_access().reset_counters();
    let _ = runtime.read_truth().read_version(first.version_id);
    let _ = runtime.read_truth().read_version(second.version_id);
    let _ = runtime.read_truth().read_version(second.version_id);
    let stats = runtime.storage_access().storage_stats();
    let counters = runtime.performance_access().counters();

    assert_eq!(stats.recent_visibility_cache_count, 1);
    assert_eq!(stats.protected_visibility_version_count, 1);
    assert!(stats.cached_visibility_version_count <= 2);
    assert!(counters.visibility_cache_miss_reconstructions >= 2);
    assert!(counters.visibility_cache_hits >= 1);
    assert!(counters.visibility_cache_recent_evictions >= 1);
    let second_read_path = runtime
        .read_truth()
        .inspect_version_read_path(second.version_id)
        .unwrap();
    assert!(second_read_path
        .entries
        .iter()
        .any(|entry| entry.code == DiagnosticCode::VisibilityCacheHit));
    let evicted_first_read_path = runtime
        .read_truth()
        .inspect_version_read_path(first.version_id)
        .unwrap();
    assert!(evicted_first_read_path.entries.iter().any(|entry| {
        entry.code == DiagnosticCode::SnapshotReadPathInspected
            && diagnostic_field(entry, "cached_visibility_state")
                == &RelationalDiagnosticValue::Bool(false)
            && diagnostic_field(entry, "recent_resident") == &RelationalDiagnosticValue::Bool(false)
    }));
    assert_eq!(
        third.version_id,
        runtime.history().latest_commit().unwrap().version_id
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

    runtime.performance_access().reset_counters();
    for version_id in &versions[..versions.len() - 1] {
        let _ = runtime.read_truth().read_version(*version_id);
    }
    let stats = runtime.storage_access().storage_stats();

    assert_eq!(
        runtime
            .config()
            .visibility
            .cache_policy
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
        .history_authority()
        .create_branch(
            BranchId("analysis".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();

    runtime.performance_access().reset_counters();
    for revision in 0..3 {
        let _ = update_entity(&mut runtime, entity, &format!("base-r{revision}"));
    }

    let stats = runtime.storage_access().storage_stats();
    let counters = runtime.performance_access().counters();

    assert_eq!(stats.protected_visibility_version_count, 2);
    assert_eq!(stats.cached_visibility_version_count, 0);
    assert_eq!(stats.recent_visibility_cache_count, 0);
    assert_eq!(counters.visibility_cache_branch_head_promotions, 3);
}

#[test]
fn branch_head_protection_can_be_lazy_without_populating_visibility_cache() {
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
        .history_authority()
        .create_branch(
            BranchId("analysis".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _ = update_entity(&mut runtime, entity, "base-updated");

    let stats = runtime.storage_access().storage_stats();

    assert_eq!(stats.protected_visibility_version_count, 2);
    assert_eq!(stats.cached_visibility_version_count, 0);
    assert_eq!(stats.recent_visibility_cache_count, 0);

    let _ = runtime.read_truth().read_version(base.version_id);
    let warmed_stats = runtime.storage_access().storage_stats();
    assert_eq!(warmed_stats.cached_visibility_version_count, 1);
    assert_eq!(warmed_stats.protected_visibility_version_count, 2);
}

#[test]
fn geometry_profile_does_not_force_retention_pass_on_each_commit() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);

    runtime.performance_access().reset_counters();
    let created = create_entity_outcome(&mut runtime, "geometry-hot-retention");
    let entity = changed_entities(&created)[0];
    let counters_after_create = runtime.performance_access().counters();

    assert_eq!(counters_after_create.retention_entity_slots_scanned, 0);
    assert_eq!(counters_after_create.retention_relation_slots_scanned, 0);

    runtime.performance_access().reset_counters();
    let deleted = delete_entity(&mut runtime, entity);
    let counters_after_delete = runtime.performance_access().counters();

    assert_eq!(counters_after_delete.retention_entity_slots_scanned, 0);
    assert_eq!(counters_after_delete.retention_relation_slots_scanned, 0);

    assert!(runtime
        .visibility_authority()
        .release_snapshot(&created.snapshot));
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&deleted.snapshot));

    let plan = runtime.retention().inspect_plan();
    assert!(plan.reclaimable_entities >= 1);
}
